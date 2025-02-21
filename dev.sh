#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
. sh/pid.sh

set -ex

name=$(basename $DIR)

onexit() {
  docker compose -p $name down
}

trap onexit EXIT
set -a
. env.sh
set +a

./docker/up.sh -d
./sh/wait-for-it.sh $R_NODE -t 999

exec mise exec -- bun x concurrently --names "kv,srv" \
  "docker-compose -p $name logs -f" \
  "./srv/dev.sh"
