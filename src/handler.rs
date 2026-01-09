use crate::metrics::Metrics;
use prometheus::{Encoder, TextEncoder};
use warp::Filter;

pub fn metrics_route(
    metrics: Metrics,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("metrics")
        .map(move || {
            let encoder = TextEncoder::new();
            let metric_families = metrics.registry.gather();

            let mut buffer = Vec::new();
            encoder.encode(&metric_families, &mut buffer).unwrap();

            warp::reply::with_header(buffer, "Content-Type", encoder.format_type())
        })
        .with(warp::trace::named("metrics"))
}
