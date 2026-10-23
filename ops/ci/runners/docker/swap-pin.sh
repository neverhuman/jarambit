#!/usr/bin/env bash
# Keep swap at 0 for every jarambit runner container on a systemd-cgroup-driver Docker host.
# Docker writes memory.swap.max=0 at start, but `systemctl daemon-reload` re-applies the scope's
# own properties (MemorySwapMax=infinity). Recording it as a runtime property makes it stick.
# Without this, run_bounded.sh's probe refuses bounded steps ("swap cap not enforced"): jobs fail
# closed rather than run without the bound. Run as root by jarambit-runner-swap-pin.timer.
set -euo pipefail
for id in $(docker ps -q --no-trunc --filter label=jarambit.ci.runner=1); do
  scope=docker-$id.scope
  [[ $(cat "/sys/fs/cgroup/system.slice/$scope/memory.swap.max" 2>/dev/null) == 0 ]] && continue
  systemctl set-property --runtime "$scope" MemorySwapMax=0
done
