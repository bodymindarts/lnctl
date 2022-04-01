#!/usr/bin/env bats

load "../helpers"

setup_file() {
  start_network
}

teardown_file() {
  teardown_network
}

teardown() {
  stop_connector
  stop_coordinator
}

@test "Can list channel history" {
  background ${lnctl} connector
  retry 5 1 curl_connector GetStatus
  background ${lnctl} server

  open_channel lnd1 lnd2
  chan_id=$(lnd_cmd lnd1 listchannels | jq -r '.channels[0].chan_id')
  sleep 2

  n_snapshots=$(${lnctl} channel-history ${chan_id} | jq -r '.snapshots | length')
  [ "${n_snapshots}" -eq 1 ]
}
