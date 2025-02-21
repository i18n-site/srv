#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
. sh/pid.sh

set -ex

name=$(basename $DIR)

onexit() {
  docker compose -p $name stop
}

trap onexit EXIT

./docker/up.sh -d
exec mise exec -- bun x concurrently --names "kv,srv" \
  "docker-compose -p $name logs -f" \
  "./srv/dev.sh"
