#!/usr/bin/env bats

load "../helpers"

setup_file() {
  stop_connector
  stop_coordinator
  start_network
  start_connector
  retry 5 1 curl_connector GetStatus
}

teardown_file() {
  teardown_network
  stop_connector
}

setup() {
  start_coordinator
  retry 5 1 curl_coordinator GetStatus 
}

teardown() {
  stop_coordinator
}

@test "Coordinator connects to connector" {
  n_connectors=$(curl_coordinator GetStatus | jq -r '.connectors | length')
  [ "${n_connectors}" -eq 1 ]
}

@test "Can list channel snapshots" {
  open_channel lnd1 lnd2
  chan_id=$(lnd_cmd lnd1 listchannels | jq -r '.channels[0].chan_id')

  n_snapshots=$(curl_coordinator ListMonitoredChannelSnapshots "$(jq -n -c --arg chan_id ${chan_id} '{ short_channel_id: $chan_id }')" | jq -r '.snapshots | length')
  [ "${n_snapshots}" -eq 1 ]
}
