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
  stop_lnctl
}
