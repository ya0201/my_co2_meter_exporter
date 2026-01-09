use prometheus::{Gauge, Registry};

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    pub api_value: Gauge,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let api_value = Gauge::new(
            "external_api_value",
            "value fetched from external api",
        ).unwrap();

        registry.register(Box::new(api_value.clone())).unwrap();

        Self { registry, api_value }
    }
}
