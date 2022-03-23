export CONNECTOR_CONFIG=./test/connector/connector.yml
export COORDINATOR_CONFIG=./test/coordinator/coordinator.yml

start_connector() {
  background cargo run --bin connector
}

stop_connector() {
  [ -f .lnctl/connector/pid ] && kill $(cat .lnctl/connector/pid) > /dev/null || true
}

start_coordinator() {
  background cargo run --bin coordinator
}

stop_coordinator() {
  [ -f .lnctl/coordinator/pid ] && kill $(cat .lnctl/coordinator/pid) > /dev/null || true
}

curl_connector() {
  grpcurl -plaintext -import-path ./proto/connector -proto connector.proto localhost:5626 connector.LnctlConnector/$1 
}

curl_coordinator() {
  grpcurl -plaintext -import-path ./proto/coordinator -proto coordinator.proto localhost:5625 coordinator.LnctlCoordinator/$1 
}

start_network() {
  rm -rf .lnctl || true
  rm dev/lnd/*.macaroon || true
  docker compose up -d bitcoind

  bitcoin_cmd createwallet default
  genblocks 250
  docker compose up integration-deps
  fetch_macaroon lnd1
  lnd1_pubkey=$(lnd_cmd lnd1 getinfo | jq -r '.identity_pubkey')
  lnd2_pubkey=$(lnd_cmd lnd2 getinfo | jq -r '.identity_pubkey')
  lnd_cmd lnd1 connect "${lnd2_pubkey}@lnd2:9735"
}

teardown_network() {
  docker compose down -v
}

genblocks() {
  addr=$(bitcoin_cmd getnewaddress)
  bitcoin_cmd generatetoaddress ${1} ${addr}
}

lnd_cmd() {
  container="${1}"
  shift
  docker compose exec -T ${container} lncli -n regtest ${@}
}

bitcoin_cmd() {
  docker compose exec -T bitcoind bitcoin-cli -regtest ${@}
}

fetch_macaroon() {
  local container_id=$(docker ps -q -f status=running -f name="${PWD##*/}-$1-")
  if [ ! -z "${container_id}" ]; then
    # On Arch Linux `docker compose up` appears to complete before the lnd containers have initialized the macaroons.
    # Here we retry for 10 seconds until we can copy the macroon successfully
    for i in 1 2 3 4 5 6 7 8 9 10; do
      docker cp $container_id:/root/.lnd/data/chain/bitcoin/regtest/admin.macaroon dev/lnd/$1.macaroon 2> /dev/null
      sleep 1
    done
  fi
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
