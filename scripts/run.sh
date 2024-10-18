#!/usr/bin/env bash

echo "Execute 'init_db.sh'"
. /Users/dima/Development/rust/zero2prod/scripts/init_db.sh

echo "Execute 'init_redis.sh'"
. /Users/dima/Development/rust/zero2prod/scripts/init_redis.sh
