#!/usr/bin/env bats

lnctl=./target/debug/lnctl

start_network() {
  docker compose up -d
}

teardown_network() {
  docker compose down -v
}

test_tmp_dir() {
  mkdir -p ${BATS_TMPDIR}/${BATS_TEST_NAME}
  echo ${BATS_TMPDIR}/${BATS_TEST_NAME}
}

start_lnctl() {
  background "${lnctl}" server > $(test_tmp_dir)/lnctl_pid
}

stop_lnctl() {
  kill $(cat $(test_tmp_dir)/lnctl_pid) > /dev/null
}

# Run the given command in the background. Useful for starting a
# node and then moving on with commands that exercise it for the
# test.
#
# Ensures that BATS' handling of file handles is taken into account;
# see
# https://github.com/bats-core/bats-core#printing-to-the-terminal
# https://github.com/sstephenson/bats/issues/80#issuecomment-174101686
# for details.
background() {
  "$@" 3>- &
  echo $!
}
