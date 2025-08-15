use std::collections::HashMap;

use anyhow::Error;
use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{
    Field, IndexRecordOption, OwnedValue, Schema, TextFieldIndexing, TextOptions,
    Value as DocValue, STORED, STRING, TEXT,
};
use tantivy::{doc, Document, Index, IndexReader, ReloadPolicy};
use tantivy::{tokenizer::*, TantivyDocument};
use uuid::Uuid;

use crate::db::DbHandle;
use crate::escape::truncate;

pub struct Searcher {
    idx: Index,
    reader: IndexReader,
    schema: Schema,
    id: Field,
    kind: Field,
    slug: Field,
    photo_id: Field,
    name: Field,
    category: Field,
    autosuggest: Field,
    parser: QueryParser,
    scientific_name: Field,
    summary: Field,
    description: Field,
}

impl Searcher {
    pub fn new() -> Self {
        let mut schema_builder = Schema::builder();

        let id = schema_builder.add_text_field("id", STRING | STORED);
        let kind = schema_builder.add_text_field("kind", STRING | STORED);
        let slug = schema_builder.add_text_field("slug", STORED);
        let photo_id = schema_builder.add_text_field("photo_id", STORED);
        let name = schema_builder.add_text_field("name", TEXT | STORED);
        let scientific_name = schema_builder.add_text_field("scientific_name", TEXT | STORED);

        let summary = schema_builder.add_text_field("summary", STORED);
        let category = schema_builder.add_text_field("category", STRING | STORED);
        let description = schema_builder.add_text_field("description", TEXT);

        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("autosuggest")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);

        let text_options = TextOptions::default().set_indexing_options(text_field_indexing);

        let autosuggest = schema_builder.add_text_field("autosuggest", text_options);

        let schema = schema_builder.build();

        let idx = Index::create_in_ram(schema.clone());

        let ngrams = LowerCaser.transform(NgramTokenizer::new(1, 4, false).expect("always works"));

        idx.tokenizers().register("autosuggest", ngrams);

        let reader = idx
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()
            .expect("Could not build reader!");

        let mut parser = QueryParser::for_index(
            &idx,
            vec![name, description, scientific_name, autosuggest, category],
        );

        parser.set_field_boost(name, 3.0);
        parser.set_field_boost(scientific_name, 2.0);
        parser.set_field_boost(category, 1.5);
        parser.set_field_boost(autosuggest, 0.5);
        parser.set_conjunction_by_default();

        Searcher {
            idx,
            reader,
            schema,
            id,
            kind,
            slug,
            photo_id,
            name,
            scientific_name,
            summary,
            description,
            autosuggest,
            category,
            parser,
        }
    }

    pub async fn build_index(&self, handle: &DbHandle) -> Result<(), Error> {
        let mut index_writer = self.idx.writer(50_000_000)?;

        index_writer.delete_all_documents()?;

        let category_values: HashMap<Uuid, String> = handle
            .category_values(None)
            .await?
            .into_iter()
            .map(|cat| (cat.id, cat.value))
            .collect();

        for sealife in handle.sealife(&Default::default()).await? {
            let mut doc = doc!(
                    self.id => sealife.id.to_string(),
                    self.kind => "sealife",
                    self.name => &sealife.name as &str,
                    self.autosuggest => &sealife.name as &str,
                    self.summary => truncate(&sealife.description, 155),
                    self.description => sealife.description
            );

            if let Some(slug) = sealife.slug {
                doc.add_text(self.slug, slug);
            }

            if let Some(ref scientific_name) = sealife.scientific_name {
                doc.add_text(self.scientific_name, scientific_name);
                doc.add_text(self.autosuggest, scientific_name);
            }

            if let Some(photo_id) = sealife.photo_id {
                doc.add_text(self.photo_id, photo_id.to_string());
            }

            for cat_val in handle.category_map(sealife.id).await?.values().flatten() {
                if let Some(value) = category_values.get(cat_val) {
                    doc.add_text(self.category, cat_val.to_string());
                    doc.add_text(self.autosuggest, value);
                }
            }

            index_writer.add_document(doc)?;
        }

        for dive_site in handle.dive_sites(None, &Default::default()).await? {
            let mut doc = doc!(
                  self.id => dive_site.id.to_string(),
                  self.kind => "dive_site",
                  self.name => &dive_site.name as &str,
                  self.autosuggest => &dive_site.name as &str,
                  self.summary => truncate(&dive_site.description, 155),
                  self.description => dive_site.description
            );

            if let Some(slug) = dive_site.slug {
                doc.add_text(self.slug, slug);
            }

            if let Some(photo_id) = dive_site.photo_id {
                doc.add_text(self.photo_id, photo_id.to_string());
            }

            index_writer.add_document(doc)?;
        }

        tokio::task::spawn_blocking(move || index_writer.commit()).await??;

        Ok(())
    }

    pub fn search(&self, query: &str, offset: Option<usize>) -> Result<Vec<SearchResult>, Error> {
        let searcher = self.reader.searcher();

        let query = self.parser.parse_query(query)?;

        let collector = match offset {
            Some(val) => TopDocs::with_limit(10).and_offset(val),
            None => TopDocs::with_limit(10),
        };

        let top_docs = searcher.search(&query, &collector)?;

        let mut output = Vec::with_capacity(top_docs.len());

        for (_score, addr) in top_docs {
            let doc = searcher.doc::<TantivyDocument>(addr)?;

            let search_result: SearchResult =
                serde_json::from_value(to_json_value(&doc, &self.schema)?)?;

            output.push(search_result);
        }

        Ok(output)
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, SimpleObject)]
pub struct SearchResult {
    pub id: String,
    pub kind: SearchResultKind,
    pub photo_id: Option<String>,
    pub slug: String,
    pub name: String,
    pub scientific_name: Option<String>,
    pub summary: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Enum)]
#[serde(rename_all = "snake_case")]
pub enum SearchResultKind {
    Sealife,
    DiveSite,
}

// This is a little hack to use serde infrastructure to rehydrate a doc into something that can be deserialized.
// Not sure if there is a way to do this without relying on serde_json or another deserializer
pub fn to_json_value<D: Document>(doc: &D, schema: &Schema) -> Result<Value, Error> {
    let mut json_map = Map::new();

    for (field, mut field_values) in doc.get_sorted_field_values() {
        let field_name = schema.get_field_name(field);

        let value: Option<OwnedValue> = field_values
            .pop()
            .map(|val| OwnedValue::from(val.as_value()));

        json_map.insert(field_name.to_string(), serde_json::to_value(&value)?);
    }

    Ok(Value::Object(json_map))
}
