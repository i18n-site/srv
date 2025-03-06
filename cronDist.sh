#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./sh/cronEnv.sh
. conf/cron/projectPrefix.sh

deploy() {
  cd ../mod/auth_/cron
  deployctl deploy \
    --save-config=false \
    --force --prod \
    --project=${PROJECT_PREFIX}$1 \
    --env-file=/tmp/srv_cron.env
}

deploy auth
