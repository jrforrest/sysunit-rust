#!/bin/sh

set -e -u

unit_operation="${1:?}"
unit_path="${2:?}"

exec "$unit_path" "$unit_operation"
