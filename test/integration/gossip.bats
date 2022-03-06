#!/usr/bin/env bats

load "helpers"

setup() {
  start_network
  start_lnctl
  sleep 2
}

teardown() {
  stop_lnctl
  teardown_network
}

@test "Node can be added to the network" {
  node_id=$(${lnctl} node-status | jq -r '.id')
  lnd_cmd lnd1 connect "${node_id}@host.docker.internal:9735"
  n_peers=$(${lnctl} list-peers | jq -r '. | length')
  [ "${n_peers}" -eq 1 ]
}

@test "Node will receive node announcement" {
  open_channel lnd1 lnd2 100000
  # docker compose up additional-deps
  # node_id=$(${lnctl} node-status | jq -r '.id')
  # lnd1_pubkey=$(lnd_cmd lnd1 getinfo | jq -r '.identity_pubkey')
  # lnd_cmd lnd3 connect "${lnd1_pubkey}@lnd1:9735"
  # n_peers=$(${lnctl} list-peers | jq -r '. | length')
  # [ "${n_peers}" -eq 2 ]
}
