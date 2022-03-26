use anyhow::Context;
use lnctl_coordinator::CoordinatorConfig;

const CONFIG_PATH_KEY: &'static str = "COORDINATOR_CONFIG";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = std::env::var(CONFIG_PATH_KEY)?;
    let config_file = std::fs::read_to_string(path).context("Couldn't read config file")?;
    let config: CoordinatorConfig =
        serde_yaml::from_str(&config_file).context("Couldn't parse config file")?;
    lnctl_coordinator::run(config).await?;
    Ok(())
}
