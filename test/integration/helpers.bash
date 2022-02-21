#!/usr/bin/env bats

lnctl=./target/debug/lnctl

FAUCET_PRIVATE_KEY="92Qba5hnyWSn5Ffcka56yMQauaWY6ZLd91Vzxbi4a9CCetaHtYj"
FAUCET="mgWUuj1J1N882jmqFxtDepEC73Rr22E9GU"

start_network() {
  docker compose up -d bitcoind
  genblocks
  docker compose up integration-deps
  lnd1_pubkey=$(lnd_cmd lnd1 getinfo | jq -r '.identity_pubkey')
  lnd2_pubkey=$(lnd_cmd lnd2 getinfo | jq -r '.identity_pubkey')

  cache_value "lnd1_pubkey" "$lnd1_pubkey"
  cache_value "lnd2_pubkey" "$lnd2_pubkey"

  lnd_cmd lnd1 connect "${lnd2_pubkey}@lnd2:9735"
}

cache_value() {
  echo $2 > ${BATS_TMPDIR}/$1
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

lnd_cmd() {
  container="${1}"
  shift
  docker compose exec -T ${container} lncli -n regtest ${@}
}

genblocks() {
  docker compose exec -T bitcoind bitcoin-cli -regtest generatetoaddress 250 ${FAUCET}
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

# Stolen from
# https://github.com/docker/swarm/blob/master/test/integration/helpers.bash
retry() {
  local attempts=$1
  shift
  local delay=$1
  shift
  local i

  for ((i=0; i < attempts; i++)); do
    run "$@"
    # shellcheck disable=2154
    if [[ "$status" -eq 0 ]] ; then
      return 0
    fi
    sleep "$delay"
  done

  # shellcheck disable=2154
  echo "Command \"$*\" failed $attempts times. Output: $output"
  false
}
