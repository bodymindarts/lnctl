#!/usr/bin/env bats

load "../helpers"

setup_file() {
  stop_connector
  start_network
}

teardown_file() {
  teardown_network
}

@test "Connector can run" {
  run_connector

  sleep 2

  connector_id=$(grpcurl -plaintext -import-path ./proto/connector -proto connector.proto localhost:5626 connector.LnctlConnector/GetStatus | jq -r '.connectorId')
  [[ "${connector_id}" = "$(cat .lnctl/connector/connector-id)" ]]

  stop_connector
}
