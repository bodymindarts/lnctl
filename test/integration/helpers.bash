#!/usr/bin/env bats

lnctl=./target/debug/lnctl

start_network() {
  stop_lnctl
  rm -rf .lnctl
  docker compose up -d bitcoind
  bitcoin_cmd createwallet default
  genblocks 250
  docker compose up integration-deps
  sleep 2
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
  background "${lnctl}" server
}

stop_lnctl() {
  [ -f .lnctl/pid ] && kill $(cat .lnctl/pid) > /dev/null || true
}

restart_lnctl() {
  stop_lnctl
  start_lnctl
}

lnd_cmd() {
  container="${1}"
  shift
  docker compose exec -T ${container} lncli -n regtest ${@}
}

bitcoin_cmd() {
  docker compose exec -T bitcoind bitcoin-cli -regtest ${@}
}

open_channel() {
  origin=${1}
  dest=${2}
  if [[ "${dest}" == "lnctl" ]]; then
    dest_pubkey=$(lnctl node-status | jq -r '.id')
  else
    dest_pubkey=$(lnd_cmd ${dest} getinfo | jq -r '.identity_pubkey')
  fi
  addr=$(lnd_cmd ${origin} newaddress p2wkh | jq -r '.address')
  bitcoin_cmd sendtoaddress "${addr}" 50
  genblocks 10
  lnd_cmd "${origin}" openchannel "${dest_pubkey}" 5000000 0
  genblocks 10
}

genblocks() {
  addr=$(bitcoin_cmd getnewaddress)
  bitcoin_cmd generatetoaddress ${1} ${addr}
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
