use actix_web::{web, App, HttpResponse, HttpServer, Result};
use prometheus_client::encoding::{text::encode, EncodeMetric, MetricEncoder};
use prometheus_client::metrics::MetricType;
use prometheus_client::registry::Registry;
use std::env;
use std::str::FromStr;
use std::sync::Mutex;

#[derive(Debug)]
struct Metric01 {}

impl EncodeMetric for Metric01 {
    fn metric_type(&self) -> prometheus_client::metrics::MetricType {
        MetricType::Counter
    }

    fn encode(&self, mut encoder: MetricEncoder) -> Result<(), std::fmt::Error> {
        encoder.encode_counter::<(), _, u64>(&1, None)
    }
}

#[derive(Debug)]
struct Metric02 {}

impl EncodeMetric for Metric02 {
    fn metric_type(&self) -> prometheus_client::metrics::MetricType {
        MetricType::Gauge
    }

    fn encode(&self, mut encoder: MetricEncoder) -> Result<(), std::fmt::Error> {
        let i: i64 = 1;
        encoder.encode_gauge(&i)
    }
}

#[derive(Debug)]
struct Metric03 {}

impl EncodeMetric for Metric03 {
    fn metric_type(&self) -> prometheus_client::metrics::MetricType {
        MetricType::Histogram
    }

    fn encode(&self, mut encoder: MetricEncoder) -> Result<(), std::fmt::Error> {
        encoder.encode_histogram::<()>(1.0, 2, &[(1.0, 2)], None)
    }
}

#[derive(Debug)]
struct Metric04 {}

impl EncodeMetric for Metric04 {
    fn metric_type(&self) -> prometheus_client::metrics::MetricType {
        MetricType::Info
    }

    fn encode(&self, mut encoder: MetricEncoder) -> Result<(), std::fmt::Error> {
        encoder.encode_info(&[("label-01", "value-01")])
    }
}

pub struct Exporter {
    pub registry: Registry,
}

pub async fn metrics_handler(exporter: web::Data<Mutex<Exporter>>) -> Result<HttpResponse> {
    let exporter = exporter.lock().unwrap();
    let mut body = String::new();
    encode(&mut body, &exporter.registry).unwrap();

    Ok(HttpResponse::Ok().body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut exporter = Exporter {
        registry: Registry::default(),
    };

    exporter
        .registry
        .register("matric_01", "matric_01 help", Metric01 {});
    exporter
        .registry
        .register("matric_02", "matric_02 help", Metric02 {});
    exporter
        .registry
        .register("matric_03", "matric_03 help", Metric03 {});
    exporter
        .registry
        .register("matric_04", "matric_04 help", Metric04 {});

    let exporter = web::Data::new(Mutex::new(exporter));

    let port = u16::from_str(&String::from(&env::args().collect::<Vec<_>>()[1])).unwrap_or(0);
    let uri = String::from(&env::args().collect::<Vec<_>>()[2]);

    HttpServer::new(move || {
        App::new()
            .app_data(exporter.clone())
            .service(web::resource(uri.clone()).route(web::get().to(metrics_handler)))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
