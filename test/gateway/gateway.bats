#!/usr/bin/env bats

load "../helpers"

setup_file() {
  stop_connector
  stop_gateway
  start_network
  start_connector
  retry 5 1 curl_connector GetStatus
}

teardown_file() {
  teardown_network
  stop_connector
}

setup() {
  start_gateway
  retry 5 1 curl_gateway GetStatus
}

teardown() {
  stop_gateway
}

@test "Gateway connects to connector" {
  n_connectors=$(curl_gateway GetStatus | jq -r '.connectors | length')
  [ "${n_connectors}" -eq 1 ]
}

@test "Can list channel snapshots" {
  open_channel lnd1 lnd2
  chan_id=$(lnd_cmd lnd1 listchannels | jq -r '.channels[0].chan_id')

  sleep 2

  n_snapshots=$(curl_gateway ListMonitoredChannelSnapshots "$(jq -n -c --arg chan_id ${chan_id} '{ short_channel_id: $chan_id }')" | jq -r '.snapshots | length')
  [ "${n_snapshots}" -eq 1 ]
}
