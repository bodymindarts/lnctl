mod proto {
    tonic::include_proto!("connector");
}

use crate::config::ServerConfig;

struct ConnectorServer {}

pub(crate) async fn run_server(config: ServerConfig) {}
