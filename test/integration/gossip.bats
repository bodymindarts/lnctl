#!/usr/bin/env bats

load "helpers"

setup() {
  start_network
}

teardown() {
  teardown_network
}

@test "Node can start" {
  start_lnctl
  peers=$(${lnctl} list-peers)
  node_id=$(${lnctl} node-status)
  stop_lnctl
}
