use anyhow::Context;
use lnctl_gateway::GatewayConfig;

const CONFIG_PATH_KEY: &'static str = "GATEWAY_CONFIG";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::var(CONFIG_PATH_KEY)?;
    let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
    let config: GatewayConfig =
        serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
    lnctl_gateway::run(config).await?;
    Ok(())
}
