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
