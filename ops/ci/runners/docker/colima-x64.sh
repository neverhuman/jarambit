#!/usr/bin/env bash
# xbabe4 (Apple Silicon): start the Rosetta-enabled colima VM that hosts the linux/amd64 runner
# containers. They restart with the engine (--restart unless-stopped). Run at login by
# com.jarambit.ci-runner-x64.plist; the existing ARM64 profile jarambit-ci is separate.
set -euo pipefail
export PATH=/opt/homebrew/bin:/usr/local/bin:$PATH
colima status --profile jarambit-x64 >/dev/null 2>&1 || \
  colima start --profile jarambit-x64 --vm-type vz --vz-rosetta --arch aarch64 \
    --cpu 16 --memory 64 --disk 300 --runtime docker --activate=false
