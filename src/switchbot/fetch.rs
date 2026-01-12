use crate::{config::Config, metrics::Metrics, switchbot::types::*};
use base64::{engine::general_purpose, Engine as _};
use hmac::Mac;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

const INTERVAL: u64 = 300;

fn generate_headers(token: &str, secret: &str) -> (String, String, String) {
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let nonce: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let message = format!("{}{}{}", token, t, nonce);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(message.as_bytes());

    let sign = general_purpose::STANDARD.encode(mac.finalize().into_bytes());

    (t, nonce, sign)
}

async fn fetch_and_update(
    client: &reqwest::Client,
    config: &Config,
    metrics: &Metrics,
) -> anyhow::Result<()> {
    let (t, nonce, sign) = generate_headers(&config.api_token, &config.api_secret);

    let resp = client
        .get(format!(
            "https://api.switch-bot.com/v1.1/devices/{}/status",
            config.co2_meter_device_id
        ))
        .header("Authorization", &config.api_token)
        .header("t", t)
        .header("nonce", nonce)
        .header("sign", sign)
        .send()
        .await?
        .error_for_status()?; // ← ここで 4xx/5xx は Err

    let value = resp.json::<CO2MeterStatusResponse>().await?;

    let body = value.body;

    metrics
        .temperature
        .with_label_values(&[&body.device_id, &body.device_type, &body.hub_device_id])
        .set(body.temperature);
    metrics
        .humidity
        .with_label_values(&[&body.device_id, &body.device_type, &body.hub_device_id])
        .set(body.humidity);
    metrics
        .co2
        .with_label_values(&[&body.device_id, &body.device_type, &body.hub_device_id])
        .set(body.co2);

    Ok(())
}

pub async fn run_switchbot_fetcher(metrics: Metrics, config: Config) {
    let client = Client::new();

    loop {
        tracing::info!("fetch start");

        if let Err(err) = fetch_and_update(&client, &config, &metrics).await {
            tracing::error!("fetch failed: {:?}", err);
        } else {
            tracing::info!("fetch end successfully");
        }

        sleep(Duration::from_secs(INTERVAL)).await;
    }
}
