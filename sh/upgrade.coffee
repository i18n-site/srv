#!/usr/bin/env coffee

> zx/globals:
  path > join dirname
  @3-/read
  @iarna/toml > parse

ROOT = dirname import.meta.dirname
DIR_MOD = join ROOT,'mod'

mod_li = []

for {path} from Object.values parse(read join(DIR_MOD,'Cargo.toml')).dependencies
  if path
    console.log join DIR_MOD, path
