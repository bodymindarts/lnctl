use tonic_lnd::Client as InnerClient;

use super::config::LndConnectorConfig;

pub struct LndClient {
    inner: InnerClient,
}

impl LndClient {
    pub async fn new(config: LndConnectorConfig) -> anyhow::Result<Self> {
        let inner = tonic_lnd::connect(
            format!("https://{}", config.admin_endpoint),
            config.cert_path,
            config.macaroon_path,
        )
        .await?;
        Ok(Self { inner })
    }
}
