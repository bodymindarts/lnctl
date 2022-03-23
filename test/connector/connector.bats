#!/usr/bin/env bats

load "../helpers"

setup_file() {
  stop_connector
  start_network
}

teardown_file() {
  teardown_network
  stop_connector
}

@test "Connector can run" {
  start_connector

  retry 5 1 curl_connector GetStatus

  connector_id=$(curl_connector GetStatus | jq -r '.connectorId')
  [[ "${connector_id}" = "$(cat .lnctl/connector/connector-id)" ]]

  stop_connector
}
