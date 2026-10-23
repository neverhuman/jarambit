#!/usr/bin/env bash
# Register this container's runner with one short-lived token read from stdin, without retaining it.
# usage: register-runner.sh jarambit-docker-<host>-<n>
set -euo pipefail
labels=xbabe,general,jarambit-d1,docker
[[ $# == 1 && $1 =~ ^jarambit-docker-xbabe[1-4]-[1-9][0-9]*$ ]] || { echo "usage: register-runner.sh jarambit-docker-<host>-<n>" >&2; exit 2; }
[[ ! -e .runner ]] || { echo "Runner already configured; preserve its existing state volume." >&2; exit 1; }
token=$(cat)
[[ -n $token && $token != *[[:space:]]* ]] || { echo "Expected one repository registration token on stdin." >&2; exit 2; }
exec ./config.sh --unattended --url https://github.com/neverhuman/jarambit \
  --token "$token" --name "$1" --labels "$labels" --work _work
