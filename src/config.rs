#[derive(Clone)]
pub struct Config {
    pub api_token: String,
    pub api_secret: String,
    pub co2_meter_device_id: String,
}

impl Config {
    pub fn new() -> Self {
        let api_token =
            std::env::var("SWITCHBOT_API_TOKEN").expect("SWITCHBOT_API_TOKEN must be set.");
        let api_secret =
            std::env::var("SWITCHBOT_API_SECRET").expect("SWITCHBOT_API_SECRET must be set.");
        let co2_meter_device_id = std::env::var("SWITCHBOT_CO2_METER_DEVICE_ID")
            .expect("SWITCHBOT_CO2_METER_DEVICE_ID must be set.");

        Self {
            api_token,
            api_secret,
            co2_meter_device_id,
        }
    }
}
