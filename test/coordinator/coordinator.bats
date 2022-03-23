#!/usr/bin/env bats

load "../helpers"

setup_file() {
  stop_connector
  start_network
  start_connector
}

teardown_file() {
  teardown_network
  stop_connector
}

@test "Coordinator connects to connector" {
  retry 5 1 curl_connector GetStatus
  start_coordinator

  retry 5 1 curl_coordinator ListConnectors 
  n_connectors=$(curl_coordinator ListConnectors | jq -r '.connectors | length')
  [ "${n_connectors}" -eq 1 ]

  stop_coordinator
}
