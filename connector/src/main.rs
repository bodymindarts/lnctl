use anyhow::Context;
use connector::ConnectorConfig;

const CONFIG_PATH_KEY: &'static str = "CONNECTOR_CONFIG";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::var(CONFIG_PATH_KEY)?;
    let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
    let config: ConnectorConfig =
        serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
    connector::run(config).await?;
    Ok(())
}
