#!/usr/bin/env bash
# Bound trusted CI shell steps without altering the shared runner service.
set -euo pipefail

inside=0
if [[ ${1:-} == --inside ]]; then
  [[ $# == 3 ]] || exit 2
  inside=1
  profile=$2
  script=$3
else
  [[ $# == 2 ]] || exit 2
  profile=$1
  script=$2
fi
case "$profile" in
  d1) memory=8G; memory_bytes=8589934592; cpus=4 ;;
  version) memory=2G; memory_bytes=2147483648; cpus=1 ;;
  datasets) memory=8G; memory_bytes=8589934592; cpus=4 ;;
  *) echo "unknown bounded CI profile" >&2; exit 2 ;;
esac
[[ -f "$script" && -r "$script" ]] || { echo "CI script missing" >&2; exit 2; }

if [[ $inside == 0 ]]; then
  # Docker runners (ops/ci/runners/docker): the container's own cgroup is the bound. The probe
  # below still verifies memory.max, swap 0, affinity and nice, so this fails closed on a host.
  container=0
  if [[ ${JARAMBIT_CI_CONTAINER:-} == 1 ]]; then
    [[ -f /.dockerenv ]] || { echo "container CI mode outside Docker" >&2; exit 1; }
    container=1
  else
    [[ -x /home/ubuntu/bin/memcap ]] || { echo "required memcap unavailable" >&2; exit 1; }
  fi
  selected=$(python3 - "$cpus" <<'PY'
import os, sys
allowed = sorted(os.sched_getaffinity(0))[:int(sys.argv[1])]
if not allowed:
    raise SystemExit("CPU affinity unavailable")
print(",".join(map(str, allowed)))
PY
  )
  if [[ $container == 1 ]]; then
    exec python3 "$(dirname "$0")/bounded_process.py" --container -- nice -n 19 taskset -c "$selected" \
      env OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1 \
      CARGO_BUILD_JOBS="$cpus" RUST_TEST_THREADS="$cpus" JARAMBIT_CI_BOUNDED=1 \
      bash "$0" --inside "$profile" "$script"
  fi
  export XDG_RUNTIME_DIR="/run/user/$(id -u)"
  export DBUS_SESSION_BUS_ADDRESS="unix:path=$XDG_RUNTIME_DIR/bus"
  exec python3 "$(dirname "$0")/bounded_process.py" \
    /home/ubuntu/bin/memcap "$memory" -- nice -n 19 taskset -c "$selected" \
    env OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1 \
    CARGO_BUILD_JOBS="$cpus" RUST_TEST_THREADS="$cpus" JARAMBIT_CI_BOUNDED=1 \
    bash "$0" --inside "$profile" "$script"
fi

# Check actual enforcement before executing the job body; a marker is insufficient.
python3 - "$memory_bytes" "$cpus" <<'PY'
import json, os, pathlib, sys, time
expected_memory, maximum_cpus = map(int, sys.argv[1:])
relative = pathlib.Path("/proc/self/cgroup").read_text().strip().split("::", 1)[1]
group = pathlib.Path("/sys/fs/cgroup") / relative.lstrip("/")
memory = (group / "memory.max").read_text().strip()
swap = (group / "memory.swap.max").read_text().strip()
affinity = sorted(os.sched_getaffinity(0))
def require(condition, message):
    if not condition:
        raise SystemExit(message)
require(memory != "max" and 0 < int(memory) <= expected_memory, "memory cap not enforced")
require(swap == "0", "swap cap not enforced")
require(0 < len(affinity) <= maximum_cpus, "CPU affinity not enforced")
require(os.getpriority(os.PRIO_PROCESS, 0) >= 19, "nice priority not enforced")
for name in ("OMP_NUM_THREADS", "OPENBLAS_NUM_THREADS", "MKL_NUM_THREADS"):
    require(os.environ.get(name) == "1", name + " not bounded")
for name in ("CARGO_BUILD_JOBS", "RUST_TEST_THREADS"):
    require(0 < int(os.environ[name]) <= maximum_cpus, name + " not bounded")
report = pathlib.Path(os.environ['JARAMBIT_CI_SCOPE_REPORT'])
ack = pathlib.Path(os.environ['JARAMBIT_CI_SCOPE_ACK'])
temporary = report.with_suffix('.pending')
with temporary.open('x') as output:
    json.dump({'pid': os.getppid(), 'cgroup': relative}, output)
temporary.replace(report)
deadline = time.monotonic() + 30
while not ack.exists():
    require(time.monotonic() < deadline, 'scope supervisor acknowledgement missing')
    time.sleep(0.02)
print("CI_RESOURCE_PROBE=" + json.dumps({"memory_max": int(memory), "swap_max": 0,
      "affinity": affinity, "native_pools": 1, "nice": os.getpriority(os.PRIO_PROCESS, 0)}), flush=True)
PY
exec bash --noprofile --norc -euo pipefail "$script"
