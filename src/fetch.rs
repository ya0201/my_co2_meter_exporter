use crate::metrics::Metrics;
use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub async fn fetch_external_api(metrics: Metrics) {
    // let client = Client::new();

    loop {
        // match client.get("https://example.com/api").send().await {
        //     Ok(resp) => {
        //         if let Ok(value) = resp.json::<f64>().await {
        //             metrics.api_value.set(value);
        //         }
        //     }
        //     Err(err) => {
        //         eprintln!("api error: {}", err);
        //     }
        // }

        metrics
            .temperature
            .with_label_values(&["hogeId", "fugaType", "mogeHubId"])
            .set(1.3);
        metrics
            .humidity
            .with_label_values(&["hogeId", "fugaType", "mogeHubId"])
            .set(40);
        metrics
            .co2
            .with_label_values(&["hogeId", "fugaType", "mogeHubId"])
            .set(1000);

        sleep(Duration::from_secs(300)).await;
    }
}
