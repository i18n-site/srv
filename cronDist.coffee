#!/usr/bin/env coffee

> zx/globals:
  ./sh/MOD_LI.js:@ > ROOT
  path > join basename
  fs > existsSync

$.verbose = true
await $"#{ROOT}/sh/cronEnv.sh"

dist = (dir)=>
  cron = join dir, 'cron'
  if not existsSync cron
    return
  cd cron
  name = basename(dir).replace(/_$/,'')
  await $"#{ROOT}/sh/cronDist.sh #{name}"
  return

for dir from MOD_LI
  await dist dir

process.exit()

