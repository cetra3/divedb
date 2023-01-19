use crate::{graphql::WebContext, schema::DiveMetric};
use actix_web::{error::ErrorInternalServerError, web, Error as ActixError, HttpResponse};
use anyhow::Error;
use askama::Template;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ChartRequest {
    #[serde(default)]
    width: Option<f64>,
    #[serde(default)]
    height: Option<f64>,
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

use log::*;
use serde::{Deserialize, Serialize};

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
    fn to_svg(&self) -> String {
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

    let max_y = ((max_y + 5.0) / 10.0).round() * 10.0;

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
        let h = (val / 3600.0).floor() as usize;
        let m = (val % 3600.0 / 60.0).floor() as usize;

        Ok(format!("{}:{:0>2}", h, m))
    }
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
