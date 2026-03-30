use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use std::time::Duration;
use tracing::{debug, warn};

use crate::controllers::door::{http_client, OCT_OPEN_URL};

pub struct OctKeepAliveInitializer;

#[async_trait]
impl Initializer for OctKeepAliveInitializer {
    fn name(&self) -> String {
        "oct-keepalive".to_string()
    }

    async fn before_run(&self, _app_context: &AppContext) -> Result<()> {
        // Eagerly initialize the HTTP client and warm the TCP+TLS connection
        let client = http_client();
        match client.head(OCT_OPEN_URL).send().await {
            Ok(_) => debug!("OCT server connection warmed up"),
            Err(e) => warn!("failed to warm OCT connection (will retry in background): {e}"),
        }

        // Spawn a background task to keep the connection alive
        tokio::spawn(async {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;
                match http_client().head(OCT_OPEN_URL).send().await {
                    Ok(_) => debug!("OCT keep-alive ping OK"),
                    Err(e) => warn!("OCT keep-alive ping failed: {e}"),
                }
            }
        });

        Ok(())
    }
}
