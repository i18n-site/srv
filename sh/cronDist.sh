#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -ex

./sh/cronEnv.sh
. conf/cron/projectPrefix.sh

cd $1
deployctl deploy \
  --save-config=false \
  --force --prod \
  --project=${PROJECT_PREFIX}$2 \
  --env-file=/tmp/srv_cron.env
