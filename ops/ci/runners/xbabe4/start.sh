#!/usr/bin/env bash
# Run at user login; Docker owns the runner's restart policy thereafter.
set -euo pipefail
export PATH=/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin
colima start --profile jarambit-ci --vm-type vz --arch aarch64 \
  --cpu 4 --memory 8 --disk 60 --runtime docker --activate=false
cd /Users/agent/actions-runners/jarambit-xbabe4-docker
docker --context colima-jarambit-ci compose up -d
