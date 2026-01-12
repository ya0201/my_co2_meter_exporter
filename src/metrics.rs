use prometheus::{GaugeVec, IntGaugeVec, Registry};

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,
    pub temperature: GaugeVec,
    pub humidity: IntGaugeVec,
    pub co2: IntGaugeVec,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let temperature = GaugeVec::new(
            prometheus::Opts::new("switchbot_co2_meter_temperature", "temperature in celsius"),
            &["deviceId", "devicdType", "hubDeviceId"], // ← ラベル名
        )
        .unwrap();
        let humidity = IntGaugeVec::new(
            prometheus::Opts::new("switchbot_co2_meter_humidity", "humidity percentage"),
            &["deviceId", "devicdType", "hubDeviceId"], // ← ラベル名
        )
        .unwrap();
        let co2 = IntGaugeVec::new(
            prometheus::Opts::new("switchbot_co2_meter_co2", "CO2 ppm value, 0-9999"),
            &["deviceId", "devicdType", "hubDeviceId"], // ← ラベル名
        )
        .unwrap();

        registry.register(Box::new(temperature.clone())).unwrap();
        registry.register(Box::new(humidity.clone())).unwrap();
        registry.register(Box::new(co2.clone())).unwrap();

        Self {
            registry,
            temperature,
            humidity,
            co2,
        }
    }
}
