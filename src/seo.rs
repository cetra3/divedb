use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse};

use crate::{
    graphql::WebContext,
    schema::{DiveSiteQuery, SealifeQuery},
    SITE_URL,
};

#[get("/robots.txt")]
pub async fn robots() -> HttpResponse {
    HttpResponse::Ok().content_type("text/plain").body(format!(
        "User-agent: *
Disallow: /login
Disallow: /register
Disallow: /forgot-password
Disallow: /reset-password
Disallow: /divesites/edit
Disallow: /dives

Sitemap: {}/sitemap.xml
",
        &*SITE_URL
    ))
}

#[get("/sitemap.xml")]
pub async fn sitemap_handler(
    web_context: web::Data<WebContext>,
) -> Result<HttpResponse, actix_web::Error> {
    let sites = web_context
        .handle
        .dive_sites(None, &DiveSiteQuery::default())
        .await
        .map_err(ErrorInternalServerError)?;

    let sealife_list = web_context
        .handle
        .sealife(&SealifeQuery::default())
        .await
        .map_err(ErrorInternalServerError)?;

    let mut vec: Vec<u8> = Vec::new();

    use sitemap::structs::UrlEntry;
    use sitemap::writer::SiteMapWriter;

    let sitemap_writer = SiteMapWriter::new(&mut vec);

    let mut urlwriter = sitemap_writer
        .start_urlset()
        .expect("Unable to write urlset");

    urlwriter
        .url(UrlEntry::builder().loc(&*SITE_URL))
        .expect("Could not write url");

    for site in sites {
        if let Some(slug) = site.slug {
            urlwriter
                .url(
                    UrlEntry::builder()
                        .lastmod(site.date.into())
                        .loc(format!("{}/sites/{}", &*SITE_URL, slug)),
                )
                .expect("Could not write url");
        }
    }

    for entry in sealife_list {
        if let Some(slug) = entry.slug {
            urlwriter
                .url(
                    UrlEntry::builder()
                        .lastmod(entry.date.into())
                        .loc(format!("{}/sealife/{}", &*SITE_URL, slug)),
                )
                .expect("Could not write url");
        }
    }

    urlwriter.end().expect("Could not close");

    Ok(HttpResponse::Ok()
        .content_type("text/xml; charset=utf-8")
        .body(vec))
}
