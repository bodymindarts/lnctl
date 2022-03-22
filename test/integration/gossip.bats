#!/usr/bin/env bats

load "../helpers"

setup_file() {
  start_network
  start_lnctl
  sleep 2
}

teardown_file() {
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
  open_channel lnd1 lnctl 100000
  open_channel lnd1 lnd2 100000
  restart_lnctl
  sleep 10
  n_nodes=$(${lnctl} graph | jq -r '.nodes | length')
  [ "${n_nodes}" -eq 1 ]
}
