use actix_web::error::{ErrorNotFound, ErrorUnauthorized};
use anyhow::anyhow;
use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{error::ErrorBadRequest, web, Error, HttpResponse};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use exif::{In, Tag};
use futures::TryStreamExt;
use image::{
    codecs::jpeg::JpegEncoder,
    imageops::{overlay, FilterType},
    DynamicImage, Rgba,
};
use imageproc::drawing::draw_text_mut;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::*;
use twoway::find_bytes;
use uuid::Uuid;

use rusttype::{point, Font, Scale};

use chrono::prelude::*;

use crate::{
    graphql::WebContext,
    log_error,
    schema::{CreatePhoto, Photo, PhotoQuery, User, UserLevel},
    token::Token,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OpenRequest {
    #[serde(default)]
    force_rerender: bool,
}

#[derive(Deserialize, PartialEq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum PhotoKind {
    #[default]
    Jpeg,
    JpegLarge,
    Webp,
    WebpLarge,
    Full,
}

pub async fn open_photo(
    id: web::Path<(PhotoKind, Uuid)>,
    query: web::Query<OpenRequest>,
    context: web::Data<WebContext>,
    token: Token,
) -> Result<NamedFile, Error> {
    let (kind, id) = id.into_inner();

    let photo = context
        .handle
        .photos(None, &PhotoQuery::id(id))
        .await
        .map_err(log_error)?
        .pop()
        .ok_or_else(|| ErrorNotFound("No photo found"))?;

    let thumb_location = match kind {
        PhotoKind::Jpeg => photo.jpg_thumb_location(),
        PhotoKind::Webp => photo.webp_thumb_location(),
        PhotoKind::JpegLarge => photo.jpg_large_location(),
        PhotoKind::WebpLarge => photo.webp_large_location(),
        PhotoKind::Full => {
            if photo.internal {
                return Ok(NamedFile::open(photo.orig_location())?);
            }

            if let Some(user_id) = token.user_id {
                let user = context
                    .handle
                    .user_details(user_id)
                    .await
                    .map_err(|_| ErrorUnauthorized("No valid user found"))?;

                if user.is_admin() || photo.user_id == user.id {
                    return Ok(NamedFile::open(photo.orig_location())?);
                }
            }

            return Err(ErrorUnauthorized("Unauthorized"));
        }
    };

    let user = context
        .handle
        .user_details(photo.user_id)
        .await
        .map_err(log_error)?;

    tokio::task::spawn_blocking(move || resize_image(&photo, &user, query.force_rerender))
        .await
        .map_err(log_error)?
        .map_err(log_error)?;

    Ok(NamedFile::open(thumb_location)?)
}

use once_cell::sync::Lazy;

pub static WATERMARK: Lazy<DynamicImage> =
    Lazy::new(|| image::load_from_memory(include_bytes!("../watermark.png")).unwrap());

pub static FONT: Lazy<Font> =
    Lazy::new(|| Font::try_from_bytes(include_bytes!("./static/Asap-Bold.otf")).unwrap());

pub fn image_dims<P: AsRef<Path>>(path: P) -> Result<(i32, i32), anyhow::Error> {
    let im = image::open(path)?;

    Ok((im.width() as i32, im.height() as i32))
}

const THUMB_WIDTH: u32 = 1000;
const LARGE_WIDTH: u32 = 2000;

fn add_overlay(im: &mut DynamicImage, date: Option<DateTime<Local>>, user: &User) {
    use crate::schema::OverlayLocation::*;

    let (xl, yl) = match user.watermark_location {
        TopLeft => (10, 10),
        TopRight => (im.width() - 160, 10),
        BottomLeft => (10, im.height() - 40),
        BottomRight => (im.width() - 160, im.height() - 40),
    };

    overlay(im, &*WATERMARK, xl as i64, yl as i64);

    let height = 30.0;
    let scale = Scale {
        x: height,
        y: height,
    };

    if let Some(location) = user.copyright_location {
        let mut copyright_notice = String::from("©");

        if let Some(ref username) = user.display_name {
            copyright_notice.push(' ');
            copyright_notice.push_str(username);
        }

        if let Some(date) = date {
            copyright_notice.push(' ');
            copyright_notice.push_str(&date.format("%Y").to_string())
        }

        let width = get_font_width(&copyright_notice, scale);

        let (xl, yl) = match location {
            TopLeft => (10, 10),
            TopRight => (im.width() - width - 10, 10),
            BottomLeft => (10, im.height() - 40),
            BottomRight => (im.width() - width - 10, im.height() - 40),
        };

        draw_shadowed_text(im, xl as i32, yl as i32, scale, &copyright_notice);
    }
}

pub fn draw_shadowed_text(im: &mut DynamicImage, x: i32, y: i32, scale: Scale, text: &str) {
    draw_text_mut(im, Rgba([59, 67, 81, 0]), x, y + 1, scale, &FONT, text);
    draw_text_mut(im, Rgba([255, 255, 255, 255]), x, y, scale, &FONT, text);
}

pub fn get_font_width(text: &str, scale: Scale) -> u32 {
    let v_metrics = &FONT.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    FONT.layout(text, scale, offset)
        .filter_map(|val| val.pixel_bounding_box())
        .map(|val| val.max.x)
        .max()
        .unwrap_or_default() as u32
}

pub fn resize_image(photo: &Photo, user: &User, force_rerender: bool) -> Result<(), anyhow::Error> {
    let jpg_location = PathBuf::from(photo.jpg_thumb_location());

    // Return if the image is already at the new location
    if std::fs::metadata(&jpg_location).is_ok() && !force_rerender {
        return Ok(());
    }

    let im = image::open(photo.orig_location())?;

    if photo.internal {
        let width = 512;

        let im = im.resize(width, width, FilterType::Lanczos3);
        std::fs::create_dir_all(jpg_location.parent().unwrap())?;
        let mut file = BufWriter::new(std::fs::File::create(jpg_location)?);
        let mut encoder = JpegEncoder::new_with_quality(&mut file, 80);
        encoder.encode_image(&im)?;

        return Ok(());
    }

    for (width, jpg_location, webp_location) in [
        (
            THUMB_WIDTH,
            PathBuf::from(photo.jpg_thumb_location()),
            PathBuf::from(photo.webp_thumb_location()),
        ),
        (
            LARGE_WIDTH,
            PathBuf::from(photo.jpg_large_location()),
            PathBuf::from(photo.webp_large_location()),
        ),
    ] {
        let mut im = im.resize(width, width, FilterType::Lanczos3);

        add_overlay(&mut im, photo.date, user);

        let mut error_count = 0;

        while std::fs::create_dir_all(jpg_location.parent().unwrap()).is_err() {
            std::thread::sleep(Duration::from_secs(10));
            if error_count > 3 {
                return Err(anyhow!("Could not create directory"));
            } else {
                debug!("Retrying directory creation");
                error_count += error_count;
            }
        }

        let mut file = BufWriter::new(std::fs::File::create(jpg_location)?);
        let mut encoder = JpegEncoder::new_with_quality(&mut file, 50);
        encoder.encode_image(&im)?;

        let mut webp_file = BufWriter::new(std::fs::File::create(webp_location)?);

        let webp_encoder =
            webp::Encoder::from_image(&im).map_err(|err| anyhow!("Error webp encoding:{err}"))?;

        let webp = webp_encoder.encode(50.0);

        webp_file.write_all(&webp)?;
    }

    /*

    let (width, height) = im.dimensions();

    let rgba8_im = im.into_rgba8();
    // TODO: conversion re-using the target buffer?
    let image = rgba8_im.as_pixels();
    let img = Img::new(image, width as usize, height as usize);

    let threads = thread::available_parallelism()?.get();
    let quality = 50.0;

    let (out_data, _, _) = encode_rgba(
        img,
        &ravif::Config {
            quality,
            alpha_quality: quality,
            speed: 2,
            premultiplied_alpha: false,
            color_space: ravif::ColorSpace::YCbCr,
            threads,
        },
    )
    .map_err(|err| anyhow!("Could not create avif file:{err}"))?;

    debug!("File size:{}", out_data.len());

    avif_file.write_all(&out_data)?;
    */

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct UploadQuery {
    #[serde(default)]
    dive_site_id: Option<Uuid>,
    #[serde(default)]
    sealife_id: Option<Uuid>,
    #[serde(default)]
    internal: Option<bool>,
}

pub async fn save_files(
    mut payload: Multipart,
    context: web::Data<WebContext>,
    query: web::Query<UploadQuery>,
    token: Token,
) -> Result<HttpResponse, Error> {
    let user = match token.user_id {
        Some(id) => context
            .handle
            .user_details(id)
            .await
            .map_err(|_| ErrorBadRequest("Unauthorized"))?,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized")),
    };

    debug!("User ID:{}", user.id);
    let quota = context
        .handle
        .photo_quota_usage(user.id)
        .await
        .map_err(log_error)? as usize;

    let max_quota = match user.level {
        UserLevel::User => 536870912,
        UserLevel::Editor => 2097152000,
        UserLevel::Admin => usize::MAX,
    };

    debug!("Current Quota:{}, Max Quota: {}", quota, max_quota);
    let mut photos = Vec::new();

    let query = query.into_inner();

    let query_site_id = query.dive_site_id;

    debug!("Site id:{:?}", query_site_id);

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        let filename = content_type
            .get_filename()
            .ok_or_else(|| ErrorBadRequest("No Filename found"))?
            .to_string();

        let id = Uuid::new_v4();

        let uuid_string = id.to_string();

        let write_folder = format!(
            "store/{}/{}/{}",
            &uuid_string[0..2],
            &uuid_string[2..4],
            &uuid_string[4..]
        );
        tokio::fs::create_dir_all(&write_folder).await?;

        let mut bytes_written = 0;

        let write_path = format!("{}/{}", write_folder, filename);

        debug!("Writing to:{}", write_path);

        let mut file = File::create(&write_path).await?;

        let mut jpeg_reader = JpegReader::new();

        // Field in turn is stream of *Bytes* object
        while let Ok(Some(chunk)) = field.try_next().await {
            file.write_all(&chunk).await?;
            if !jpeg_reader.has_enough() {
                jpeg_reader.push(&chunk);
            }

            bytes_written += chunk.len();

            if bytes_written + quota >= max_quota {
                tokio::fs::remove_dir_all(&write_folder).await?;
                return Ok(HttpResponse::InsufficientStorage().body("Upload Quota Exceeded"));
            }
        }

        let date = jpeg_reader.get_date();

        let mut dive = None;

        let mut dive_site_id = query_site_id;

        let internal = query.internal.unwrap_or_default();

        if !internal {
            if let Some(val) = date {
                dive = context
                    .handle
                    .nearest_dive_by_time(user.id, val)
                    .await
                    .map_err(log_error)?;

                if dive_site_id.is_none() {
                    dive_site_id = dive.as_ref().and_then(|val| val.dive_site_id);
                }
            }
        }

        let photo = CreatePhoto {
            id: Some(id),
            user_id: user.id,
            date: date.unwrap_or_else(Local::now),
            filename,
            dive_id: dive.as_ref().map(|val| val.id),
            dive_site_id,
            size: bytes_written as i32,
            sealife_id: query.sealife_id,
            internal: query.internal,
        };

        let user = context
            .handle
            .user_details(photo.user_id)
            .await
            .map_err(log_error)?;

        let mut new_photo = context.handle.add_photo(&photo).await.map_err(log_error)?;

        let (width, height) = tokio::task::spawn_blocking(move || image_dims(write_path))
            .await
            .map_err(log_error)?
            .map_err(log_error)?;

        context
            .handle
            .update_dims(new_photo.id, width, height)
            .await
            .map_err(log_error)?;

        new_photo.width = width;
        new_photo.height = height;
        photos.push(new_photo.clone());

        tokio::task::spawn_blocking(move || resize_image(&new_photo, &user, false))
            .await
            .map_err(log_error)?
            .map_err(log_error)?;
    }
    Ok(HttpResponse::Ok().json(&photos))
}

struct JpegReader {
    buffer: BytesMut,
    exif_len: u16,
}

impl JpegReader {
    fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
            exif_len: 65535,
        }
    }

    fn push(&mut self, chunk: &Bytes) {
        self.buffer.put(chunk.clone());

        // Check to see if the exif header length has been found
        if self.exif_len == 65535 {
            if let Some(idx) = find_bytes(&self.buffer, &[0xFF, 0xE1]) {
                if self.buffer.len() > idx + 2 {
                    let length_value: [u8; 2] = [self.buffer[idx + 2], self.buffer[idx + 3]];

                    self.exif_len = u16::from_be_bytes(length_value);
                }
            }
        }
    }

    fn has_enough(&self) -> bool {
        self.buffer.len() >= self.exif_len as usize
    }

    fn get_date(&self) -> Option<DateTime<Local>> {
        if self.has_enough() {
            let exifreader = exif::Reader::new();

            let mut reader = self.buffer.clone().reader();

            let attr = match exif::get_exif_attr_from_jpeg(&mut reader) {
                Ok(attr) => attr,
                Err(err) => {
                    error!("Could not get exif attr: {}", err);
                    return None;
                }
            };

            let exif = match exifreader.read_raw(attr) {
                Ok(exif) => exif,
                Err(err) => {
                    error!("Could not read exif data: {}", err);
                    return None;
                }
            };

            if let Some(date_time) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                let date_string = date_time.display_value().to_string();
                debug!(
                    "Field Value:{:?}, Date String:{}",
                    date_time.value, date_string
                );

                return NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S")
                    .ok()
                    .and_then(|date| Local.from_local_datetime(&date).single());
            }
        }

        None
    }
}
