use std::io::Cursor;

use crate::{
    graphql::WebContext,
    photos::{get_font_width, FONT, WATERMARK},
    schema::{DiveMetric, DiveQuery, DiveSiteQuery},
};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    web, Error as ActixError, HttpResponse,
};
use anyhow::Error;
use askama::Template;
use image::{imageops::overlay, ImageOutputFormat, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use once_cell::sync::Lazy;
use resvg::{
    tiny_skia::{self, Color},
    usvg::{
        fontdb::{self, Database},
        TreeParsing, TreeTextToPath,
    },
};
use rusttype::Scale;
use std::fmt::Write;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ChartRequest {
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
}

static FONT_DB: Lazy<Database> = Lazy::new(|| {
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();

    fontdb
});

pub async fn png_chart(
    context: web::Data<WebContext>,
    dive_id: web::Path<Uuid>,
) -> Result<HttpResponse, ActixError> {
    let dive_metrics = context
        .handle
        .dive_metrics(*dive_id)
        .await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    let dive = context
        .handle
        .dives(
            None,
            &DiveQuery {
                id: Some(*dive_id),
                ..Default::default()
            },
        )
        .await
        .map_err(ErrorInternalServerError)?
        .pop()
        .ok_or_else(|| ErrorNotFound("No dive found"))?;

    let user = context
        .handle
        .user_details(dive.user_id)
        .await
        .map_err(ErrorNotFound)?;

    let dive_site = match dive.dive_site_id {
        Some(site_id) => context
            .handle
            .dive_sites(
                None,
                &DiveSiteQuery {
                    id: Some(site_id),
                    ..Default::default()
                },
            )
            .await
            .map_err(ErrorInternalServerError)?
            .pop(),
        None => None,
    };

    let width = Some(800.0);
    let height = Some(560.0);

    let svg = render_dive(dive_metrics, width, height)
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    let mut tree = resvg::usvg::Tree::from_data(svg.as_bytes(), &Default::default()).unwrap();
    tree.convert_text(&FONT_DB);

    let rtree = resvg::Tree::from_usvg(&tree);

    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    pixmap.fill(Color::from_rgba8(48, 55, 66, 255));
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let img = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
        .ok_or_else(|| ErrorInternalServerError("Not enough pixels"))?;

    static EXTRA_HEIGHT: u32 = 40;

    let mut out_img = RgbaImage::from_pixel(
        img.width(),
        img.height() + EXTRA_HEIGHT,
        Rgba([48, 55, 66, 255]),
    );

    overlay(&mut out_img, &img, 0, EXTRA_HEIGHT as i64);

    let mut output_text = format!(
        "{} - #{}",
        user.display_name.unwrap_or(user.username),
        dive.dive_number
    );

    if let Some(site) = dive_site {
        write!(&mut output_text, " - {}", site.name).ok();
    }

    let font_height = 30.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let font_width = get_font_width(&output_text, scale);

    let logo_x = img.width() / 2 - (WATERMARK.width() / 2);

    overlay(&mut out_img, &*WATERMARK, logo_x as i64, 10);

    let font_x = img.width() / 2 - (font_width / 2);

    draw_text_mut(
        &mut out_img,
        Rgba([255, 255, 255, 255]),
        font_x as i32,
        15 + WATERMARK.height() as i32,
        scale,
        &FONT,
        &output_text,
    );

    let mut output_body = Cursor::new(Vec::<u8>::new());

    out_img
        .write_to(&mut output_body, ImageOutputFormat::Png)
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(output_body.into_inner()))
}

pub async fn svg_chart(
    context: web::Data<WebContext>,
    query: web::Query<ChartRequest>,
    dive_id: web::Path<Uuid>,
) -> Result<HttpResponse, ActixError> {
    let dive_metrics = context
        .handle
        .dive_metrics(*dive_id)
        .await
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    let ChartRequest { width, height } = query.into_inner();

    let svg = render_dive(dive_metrics, width, height)
        .map_err(|err| ErrorInternalServerError(err.to_string()))?;

    Ok(HttpResponse::Ok().content_type("image/svg+xml").body(svg))
}

use serde::{Deserialize, Serialize};
use tracing::*;

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[allow(dead_code)]
    fn as_svg(&self) -> String {
        format!("L {} {}", self.x, self.y)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Curve {
    c1: Point,
    c2: Point,
    end: Point,
}

impl Curve {
    #[allow(dead_code)]
    fn as_svg(&self) -> String {
        format!(
            "C {:.4} {:.4}, {:.4} {:.4}, {:.4} {:.4}",
            self.c1.x, self.c1.y, self.c2.x, self.c2.y, self.end.x, self.end.y
        )
    }
}

#[allow(dead_code)]
fn catmull_bezier(points: Vec<Point>) -> Vec<Curve> {
    let mut res = Vec::new();

    let last = points.len() - 1;

    for i in 0..last {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };

        let p1 = points[i];

        let p2 = points[i + 1];

        let p3 = if i + 2 > last {
            points[i + 1]
        } else {
            points[i + 2]
        };

        let c1 = Point {
            x: ((-p0.x + 6.0 * p1.x + p2.x) / 6.0),
            y: ((-p0.y + 6.0 * p1.y + p2.y) / 6.0),
        };

        let c2 = Point {
            x: ((p1.x + 6.0 * p2.x - p3.x) / 6.0),
            y: ((p1.y + 6.0 * p2.y - p3.y) / 6.0),
        };

        let end = p2;

        res.push(Curve { c1, c2, end });
    }

    res
}

fn render_dive(
    metrics: Vec<DiveMetric>,
    width: Option<f64>,
    height: Option<f64>,
) -> Result<String, Error> {
    let mut points = metrics
        .iter()
        .map(|val| Point {
            x: val.time as f64,
            y: val.depth as f64,
        })
        .collect::<Vec<Point>>();

    let offset = 50.0;
    let width = width.map(|val| val - offset * 2.).unwrap_or(877.0);
    let height = height.map(|val| val - offset * 2.).unwrap_or(300.0);

    let max_x = points.iter().map(|val| val.x).fold(f64::NAN, f64::max);
    let max_y = points.iter().map(|val| val.y).fold(f64::NAN, f64::max);

    #[allow(clippy::float_cmp)]
    let max_point = points.iter().find(|val| val.y == max_y).unwrap();
    let max_depth = max_point.y;

    let divider_depth = 5.0;

    let max_y = ((max_y + divider_depth) / divider_depth).round() * divider_depth;

    let max_point = Point {
        x: (max_point.x / max_x * width) + offset,
        y: (max_point.y / max_y * height) + offset,
    };

    debug!(
        "max_x:{}, max_y:{}, max_point:{:?}",
        max_x, max_y, max_point
    );

    points = points
        .into_iter()
        .map(|val| Point {
            x: (val.x / max_x * width) + offset,
            y: (val.y / max_y * height) + offset,
        })
        .collect();

    let path = points
        .iter()
        .map(|val| val.as_svg())
        .collect::<Vec<String>>()
        .join("");

    let chart = ChartTemplate {
        width,
        height,
        path,
        offset,
        lines: 5,
        max_x,
        max_y,
        max_point,
        max_depth,
    };
    //let output = catmull_bezier(uddf.to_points()).iter().map(|val| val.to_svg()).collect::<Vec<String>>().join("");

    //debug!("curve:{}", output);

    Ok(chart.render()?)
}

mod filters {
    pub fn minutes(val: &f64) -> ::askama::Result<String> {
        Ok(super::minutes(val))
    }
}

pub fn minutes(val: &f64) -> String {
    let h = (val / 3600.0).floor() as usize;
    let m: usize = (val % 3600.0 / 60.0).floor() as usize;
    format!("{h}:{m:0>2}")
}

#[derive(Template)]
#[template(path = "chart.svg", escape = "xml")]
struct ChartTemplate {
    width: f64,
    height: f64,
    path: String,
    offset: f64,
    lines: usize,
    max_x: f64,
    max_y: f64,
    max_point: Point,
    max_depth: f64,
}

impl ChartTemplate {
    fn lidx(&self, idx: &usize) -> f64 {
        *idx as f64 / self.lines as f64
    }
}
