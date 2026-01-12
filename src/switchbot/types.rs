use hmac::Hmac;
use serde::Deserialize;
use sha2::Sha256;

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Deserialize)]
pub struct CO2MeterStatusResponse {
    pub body: CO2MeterStatusBody,
}

#[derive(Debug, Deserialize)]
pub struct CO2MeterStatusBody {
    #[serde(alias = "deviceId")]
    pub device_id: String,

    #[serde(alias = "deviceType")]
    pub device_type: String,

    #[serde(alias = "hubDeviceId")]
    pub hub_device_id: String,

    // pub battery: i64,
    // pub version: String,
    pub temperature: f64,
    pub humidity: i64,

    #[serde(alias = "CO2")]
    pub co2: i64,
}
