use anyhow::{anyhow, Error};

use futures::stream::{self, StreamExt, TryStreamExt};
use git2::{Cred, RemoteCallbacks};
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::{
    db::DbHandle,
    schema::{
        CreateDive, CreateDiveSite, Difficulty, Dive, DiveMetric, DiveQuery, DiveSite,
        DiveSiteQuery,
    },
};
use std::{
    fs::{read_dir, File},
    mem,
};
use tracing::*;
use uuid::Uuid;

use tempdir::TempDir;

use chrono::prelude::*;

pub struct SubsurfaceRepo {
    pub sites: Vec<DiveSite>,
}

//Randomly generated Uuid for a unique name space to ensure sites are consistent
const SITE_NAMESPACE: Uuid = Uuid::from_u128(68368825254275185855298742576399799974);

//Same for dives
const DIVE_NAMESPACE: Uuid = Uuid::from_u128(6214112353904840326363746027736122008);

pub struct Repository {
    sites: Vec<DiveSite>,
    dives: Vec<Dive>,
    dive_metrics: HashMap<Uuid, Vec<DiveMetric>>,
}

pub fn sync_repository(user_id: Uuid, email: &str, password: &str) -> Result<Repository, Error> {
    let site_ns = Uuid::new_v5(&SITE_NAMESPACE, email.as_bytes());
    let dive_ns = Uuid::new_v5(&DIVE_NAMESPACE, email.as_bytes());

    let (_, d2, d3, d4) = site_ns.as_fields();

    let temp_dir = TempDir::new("subsurface")?;

    if let Err(err) = clone_git(temp_dir.path(), email, password) {
        if err.class() == git2::ErrorClass::Http {
            return Err(anyhow!("Failed to login to Subsurface cloud with the provided credentials.  Please check your email and password."));
        }

        return Err(err.into());
    }

    let dir = temp_dir.path().to_path_buf();

    let mut dives_sites = dir.clone();
    dives_sites.push("01-Divesites");

    let mut sites = HashMap::new();

    for entry in read_dir(dives_sites)? {
        let file = entry?.path();

        let file_name = file
            .file_name()
            .ok_or_else(|| anyhow!("No Filename for Directory entry"))?
            .to_string_lossy();

        if let Some(suffix) = file_name.strip_prefix("Site-") {
            let mut lines = read_lines(&file)?;

            let id = dive_id_from_hex(suffix)?;

            let mut name = None;
            let mut description = None;
            let (mut lat, mut lon) = (0.0, 0.0);

            while let Some(Ok(line)) = lines.next() {
                // Name lines look like:
                // name "Port Hughes"
                if line.starts_with("name") {
                    name = Some(line[("name".len() + 2)..(line.len() - 1)].to_string());
                }

                if line.starts_with("description") {
                    description =
                        Some(line[("description".len() + 2)..(line.len() - 1)].to_string());
                }

                // GPS lines look like:
                // gps -34.075304 137.544702
                if line.starts_with("gps") {
                    let mut gps = line[("gps".len() + 1)..].split_whitespace();

                    if let Some(num) = gps.next() {
                        lat = num.parse()?;
                    }

                    if let Some(num) = gps.next() {
                        lon = num.parse()?;
                    }
                }
            }

            if let Some(name) = name {
                let site_id = Uuid::from_fields(id, d2, d3, d4);

                let dive_site = DiveSite {
                    id: site_id,
                    user_id: Some(user_id),
                    name,
                    description: description.unwrap_or_default(),
                    access: String::default(),
                    difficulty: Difficulty::OW,
                    depth: 0.0,
                    lat,
                    lon,
                    published: false,
                    photo_id: None,
                    date: Local::now(),
                    slug: None,
                };

                sites.insert(id, dive_site);
            }
        }
    }

    let mut dives = Vec::new();
    let mut dive_metrics = HashMap::new();

    let (_, d2, d3, d4) = dive_ns.as_fields();

    for entry in read_dir(dir)? {
        let dir_entry = entry?;

        if dir_entry.file_type()?.is_dir() {
            let folder = dir_entry.path();

            let file_name = folder
                .file_name()
                .ok_or_else(|| anyhow!("No Filename for Directory entry"))?
                .to_string_lossy();

            if file_name.len() == 4 {
                if let Ok(year) = file_name.parse::<i32>() {
                    debug!("Year folder:{}", year);

                    for entry in read_dir(folder)? {
                        let dir_entry = entry?;

                        if dir_entry.file_type()?.is_dir() {
                            let folder = dir_entry.path();

                            let file_name = folder
                                .file_name()
                                .ok_or_else(|| anyhow!("No Filename for Directory entry"))?
                                .to_string_lossy();

                            if let Ok(month) = file_name.parse::<u32>() {
                                debug!("Month folder:{}", month);
                                for entry in read_dir(folder)? {
                                    let dir_entry = entry?;

                                    if dir_entry.file_type()?.is_dir() {
                                        let folder = dir_entry.path();

                                        let mut subsurface_dive = parse_dive(year, month, folder)?;

                                        let dive_id = Uuid::from_fields(
                                            subsurface_dive.dive_number,
                                            d2,
                                            d3,
                                            d4,
                                        );

                                        let dive = Dive {
                                            user_id,
                                            date: subsurface_dive.date,
                                            depth: subsurface_dive.depth,
                                            duration: subsurface_dive.duration,
                                            description: subsurface_dive.description,
                                            dive_number: 0,
                                            published: false,
                                            dive_site_id: subsurface_dive
                                                .dive_site_id
                                                .and_then(|id| sites.get(&id))
                                                .map(|site| site.id),
                                            id: dive_id,
                                            deco_model: subsurface_dive
                                                .extra_data
                                                .remove("Deco model"),
                                        };

                                        dive_metrics.insert(dive_id, subsurface_dive.metrics);

                                        dives.push(dive);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(Repository {
        sites: sites.into_values().collect(),
        dives,
        dive_metrics,
    })
}

fn dive_id_from_hex(input: &str) -> Result<u32, Error> {
    if input.len() != 8 {
        return Err(anyhow!("Need 8 hex characters to parse Dive ID"));
    }

    let octet1 = u8::from_str_radix(&input[0..2], 16)?;
    let octet2 = u8::from_str_radix(&input[2..4], 16)?;
    let octet3 = u8::from_str_radix(&input[4..6], 16)?;
    let octet4 = u8::from_str_radix(&input[6..8], 16)?;

    let id = u32::from_be_bytes([octet1, octet2, octet3, octet4]);

    Ok(id)
}

#[derive(Debug)]
pub struct SubsurfaceDive {
    pub dive_site_id: Option<u32>,
    pub dive_number: u32,
    pub date: DateTime<Local>,
    pub duration: i32,
    pub description: String,
    pub depth: f32,
    pub metrics: Vec<DiveMetric>,
    pub extra_data: HashMap<String, String>,
}

pub fn parse_dive(year: i32, month: u32, folder: PathBuf) -> Result<SubsurfaceDive, Error> {
    let file_name = folder
        .file_name()
        .ok_or_else(|| anyhow!("No Filename for Directory entry"))?
        .to_string_lossy();

    // Dives look like `09-Sun-14=58=08`
    let day: u32 = file_name[0..2].parse()?;
    let hour: u32 = file_name[7..9].parse()?;
    let minute: u32 = file_name[10..12].parse()?;
    let second: u32 = file_name[13..15].parse()?;

    #[allow(deprecated)]
    let dt: NaiveDateTime = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second);

    let date = Local
        .from_local_datetime(&dt)
        .single()
        .ok_or_else(|| anyhow!("Could not get local date from naive date"))?;

    let mut dive_site_id = None;

    let mut depth = 0.0;

    let mut metrics = vec![];
    let mut dive_number = 0;
    let mut description = String::new();
    let mut extra_data = HashMap::new();
    let mut gas_changes = HashMap::new();

    for entry in read_dir(folder)? {
        let file = entry?.path();

        let file_name = file
            .file_name()
            .ok_or_else(|| anyhow!("No Filename for Directory entry"))?
            .to_string_lossy();

        if let Some(suffix) = file_name.strip_prefix("Dive-") {
            dive_number = suffix.parse::<u32>()?;

            debug!("Dive number:{}", dive_number);

            let mut in_notes = false;

            let mut lines = read_lines(&file)?;

            while let Some(Ok(line)) = lines.next() {
                if line.starts_with("divesiteid") {
                    dive_site_id = Some(dive_id_from_hex(&line[("divesiteid".len() + 1)..])?);
                }
                if in_notes {
                    in_notes = !parse_notes(line.trim_start_matches('\t'), &mut description);
                    if in_notes {
                        description.push('\n');
                    }
                }
                if let Some(val) = line.strip_prefix("notes \"") {
                    in_notes = !parse_notes(val, &mut description);
                    if in_notes {
                        description.push('\n');
                    }
                }
            }
        } else if file_name == "Divecomputer" {
            let mut lines = read_lines(&file)?;

            let mut first_metric_line = None;

            //We skip ahead to the first time entry in the file
            while let Some(Ok(line)) = lines.next() {
                if line.starts_with("keyvalue") {
                    if let Some((key, value)) = parse_keyvalue_simple(&line) {
                        extra_data.insert(key.to_string(), value.to_string());
                    }
                }

                if line.starts_with("event") {
                    if let Some((time_seconds, o2, he)) = parse_gas_change(&line) {
                        gas_changes.insert(time_seconds, (o2, he));
                    }
                }

                if line.starts_with("  ") {
                    first_metric_line = Some(line);
                    break;
                }
            }

            if let Some(line) = first_metric_line {
                let first_metric = parse_metric(line, &mut gas_changes)?;
                depth = first_metric.depth;

                metrics.push(first_metric);

                while let Some(Ok(line)) = lines.next() {
                    let metric = parse_metric(line, &mut gas_changes)?;

                    if metric.depth > depth {
                        depth = metric.depth;
                    }

                    metrics.push(metric);
                }
            }
        }
    }

    let duration = if !metrics.is_empty() {
        metrics[metrics.len() - 1].time
    } else {
        0
    };

    if !gas_changes.is_empty() {
        warn!("Missing metrics to associate to gas changes: {gas_changes:?}");
    }

    let subsurface_dive = SubsurfaceDive {
        dive_site_id,
        dive_number,
        date,
        duration,
        description,
        depth,
        metrics,
        extra_data,
    };

    Ok(subsurface_dive)
}

fn parse_keyvalue_simple(line: &str) -> Option<(&str, &str)> {
    let content = line.strip_prefix("keyvalue ")?;

    let start1 = content.find('"')? + 1;
    let end1 = content[start1..].find('"')? + start1;
    let key = &content[start1..end1];

    let remaining = &content[end1 + 1..];
    let start2 = remaining.find('"')? + 1;
    let end2 = remaining[start2..].find('"')? + start2;
    let value = &remaining[start2..end2];

    Some((key, value))
}

// Add this helper function
fn parse_gas_change(line: &str) -> Option<(i32, Option<f32>, Option<f32>)> {
    let event_start = line.find("event ")? + 6;
    let time_end = line[event_start..].find(' ')? + event_start;
    let time_str = &line[event_start..time_end];

    let time_seconds = parse_time(time_str)?;

    let o2 = parse_percentage(line, "o2=");
    let he = parse_percentage(line, "he=");

    Some((time_seconds, o2, he))
}

fn parse_time(time_str: &str) -> Option<i32> {
    let mut time_parts = time_str.split(':');
    let minutes: u32 = time_parts.next()?.parse().ok()?;
    let seconds: u32 = time_parts.next()?.parse().ok()?;
    Some((minutes * 60 + seconds) as i32)
}

fn parse_percentage(line: &str, prefix: &str) -> Option<f32> {
    let start = line.find(prefix)? + prefix.len();
    let end = line[start..].find('%')? + start;
    let value_str = &line[start..end];
    value_str.parse().ok()
}

/// Will return `true` if it's at the end of the notes
fn parse_notes(line: &str, result: &mut String) -> bool {
    let chars = line.chars();
    let mut escaped = false;
    let mut within_quotes = true;

    for ch in chars {
        if escaped {
            result.push(ch);
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            if within_quotes {
                return true;
            }
            within_quotes = true;
        } else if within_quotes {
            result.push(ch);
        }
    }

    false
}

fn parse_metric(
    line: String,
    gas_changes: &mut HashMap<i32, (Option<f32>, Option<f32>)>,
) -> Result<DiveMetric, Error> {
    //Parses a metric string like ` 2:20 2.48m 30.2°C 199.75bar tts=0:16`
    //Time & depth are always first, then there are various other metrics

    let mut metrics = line.split_whitespace();

    let time_str = metrics.next().ok_or_else(|| anyhow!("No time value!"))?;

    let time = parse_time(time_str)
        .ok_or_else(|| anyhow!("Failed to parse time from string: {time_str}"))?;

    let depth_val = metrics.next().ok_or_else(|| anyhow!("No depth value!"))?;

    let depth: f32 = depth_val[0..depth_val.len() - 1].parse()?;

    let mut pressure = None;
    let mut temperature = None;

    for val in metrics {
        if val.ends_with("bar") {
            pressure = Some(val[0..val.len() - 3].parse()?);
        } else if val.ends_with("°C") {
            temperature = Some(val[0..val.len() - 3].parse()?);
        }
    }

    let (o2, he) = gas_changes.remove(&time).unwrap_or((None, None));

    let metric = DiveMetric {
        time,
        depth,
        pressure,
        temperature,
        o2,
        he,
    };

    Ok(metric)
}

pub async fn resolve_existing_sites(
    user_id: Uuid,
    repo: &mut Repository,
    db: DbHandle,
) -> Result<(), Error> {
    for site in repo.sites.iter_mut() {
        debug!("Searching for site:{}", site.name);

        let existing_sites = db
            .dive_sites(
                Some(user_id),
                &DiveSiteQuery {
                    name: Some(site.name.clone()),
                    ..Default::default()
                },
            )
            .await?;

        if let Some(mut existing) = existing_sites.into_iter().next() {
            info!("Found site:{}", existing.name);

            //Adjust the dives to ensure that the uuid is updated too
            for dive in repo.dives.iter_mut() {
                if let Some(site_id) = dive.dive_site_id {
                    if site_id == site.id {
                        dive.dive_site_id = Some(existing.id);
                    }
                }
            }

            mem::swap(site, &mut existing);
        }
    }

    Ok(())
}

pub async fn import_repository(user_id: Uuid, repo: Repository, db: DbHandle) -> Result<(), Error> {
    let dives = &repo.dives;
    let db = &db;

    stream::iter(repo.sites)
        .map(Ok)
        .try_for_each_concurrent(0, |site| async move {
            if let Some(existing_site) = db
                .dive_sites(
                    Some(user_id),
                    &DiveSiteQuery {
                        id: Some(site.id),
                        ..Default::default()
                    },
                )
                .await?
                .first()
            {
                trace!("Site exists:{}, skipping", existing_site.id);
            } else {
                //Resolve the depth & difficulty based upon site data.
                let mut depth = 0.0;

                for dive in dives
                    .iter()
                    .filter(|dive| dive.dive_site_id.as_ref() == Some(&site.id))
                {
                    if dive.depth > depth {
                        depth = dive.depth;
                    }
                }

                let difficulty = if depth < 20.0 {
                    Difficulty::OW
                } else if depth < 40.0 {
                    Difficulty::AOW
                } else {
                    Difficulty::Tech
                };

                //Round to nearest 5 meters
                depth = (depth / 5.0).round() * 5.0;

                let request = CreateDiveSite {
                    id: Some(site.id),
                    name: site.name,
                    access: site.access,
                    difficulty,
                    description: site.description,
                    depth: depth as f64,
                    lat: site.lat,
                    lon: site.lon,
                    photo_id: None,
                    published: false,
                };

                db.create_dive_site(user_id, &request).await?;
            }

            Ok(()) as Result<(), Error>
        })
        .await?;

    let mut metrics = repo.dive_metrics;

    let handle = &db;

    stream::iter(repo.dives)
        .map(|dive| {
            let id = dive.id;
            Ok((dive, metrics.remove(&id)))
        })
        .try_for_each_concurrent(1, |(dive, metrics)| async move {
            let mut should_update = true;

            let mut is_published = false;

            if let Some(existing_dive) = handle
                .dives(
                    Some(dive.user_id),
                    &DiveQuery {
                        id: Some(dive.id),
                        ..Default::default()
                    },
                )
                .await?
                .first()
            {
                is_published = existing_dive.published;
                if existing_dive.date != dive.date
                    || existing_dive.duration != dive.duration
                    || existing_dive.depth != dive.depth
                    || existing_dive.dive_site_id != dive.dive_site_id
                    || existing_dive.description != dive.description
                    || existing_dive.deco_model != dive.deco_model
                {
                    debug!("Changes to dive: before: {existing_dive:?}, after: {dive:?}");
                } else {
                    if !handle.has_metrics(dive.id).await?
                        && metrics.as_ref().is_some_and(|m| !m.is_empty())
                    {
                        debug!("Dive has no metrics: {}, updating", dive.id);
                    } else {
                        debug!("Dive exists:{}, skipping", existing_dive.id);
                        should_update = false;
                    }
                }
            }

            if should_update {
                debug!("Syncing {:?}", dive);
                let request = CreateDive {
                    id: Some(dive.id),
                    date: dive.date.into(),
                    duration: dive.duration,
                    depth: dive.depth as f64,
                    dive_site_id: dive.dive_site_id,
                    description: dive.description,
                    published: is_published,
                    deco_model: dive.deco_model,
                };

                handle.create_dive(dive.user_id, &request, metrics).await?;


            }

            Ok(()) as Result<(), Error>
        })
        .await?;

    handle.refresh_dives(user_id).await?;

    debug!("Finished subsurface sync for {user_id}");

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn clone_git(path: &Path, email: &str, password: &str) -> Result<(), git2::Error> {
    // This has started failing with:
    //    `an unknown git error occurred; code=NotFound (-3)`
    // So we're using the git client for now...

    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(|_, _, _| Cred::userpass_plaintext(email, password));

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    let repo = builder.clone(
        &format!("https://ssrf-cloud-eu.subsurface-divelog.org/git/{email}"),
        path,
    )?;

    repo.set_head(&format!("refs/remotes/origin/{email}"))?;
    repo.checkout_head(None)?;

    Ok(())
}
