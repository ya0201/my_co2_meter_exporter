use std::str::FromStr;

const DEFAULT_PORT: u16 = 9100;

#[derive(Clone)]
pub struct Config {
    pub api_token: String,
    pub api_secret: String,
    pub co2_meter_device_id: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        let api_token =
            std::env::var("SWITCHBOT_API_TOKEN").expect("SWITCHBOT_API_TOKEN must be set.");
        let api_secret =
            std::env::var("SWITCHBOT_API_SECRET").expect("SWITCHBOT_API_SECRET must be set.");
        let co2_meter_device_id = std::env::var("SWITCHBOT_CO2_METER_DEVICE_ID")
            .expect("SWITCHBOT_CO2_METER_DEVICE_ID must be set.");
        let port_str = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());

        // Parse the String into a u16 (unsigned 16-bit integer)
        let port: u16 = match u16::from_str(&port_str) {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!(
                    "Error parsing port number: {}. Using default port {}.",
                    e,
                    DEFAULT_PORT
                );
                DEFAULT_PORT // Fallback to a default port if parsing fails
            }
        };

        Self {
            api_token,
            api_secret,
            co2_meter_device_id,
            port,
        }
    }
}
