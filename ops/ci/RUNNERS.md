# Local general CI runners

Every GitHub Actions job in this repository must require the three labels
`self-hosted`, `xbabe`, and `general`. These identify the local general pool on
xbabe1, xbabe2, xbabe3 and xbabe4. Label lists are cumulative: a capability label
narrows the pool; it never falls back to a GitHub-hosted machine.

| Host | Runner names | Environment | Capabilities |
| --- | --- | --- | --- |
| xbabe1 | `jarambit-xbabe1` | Ubuntu 24.04, x64, systemd service | `jarambit-d1` |
| xbabe2 | `jarambit-xbabe2`, `jarambit-xbabe2-2`, `jarambit-xbabe2-3` | Ubuntu 24.04, x64 | `jarambit-d1`, `jarambit-publication`, `jarambit-render` |
| xbabe3 | `jarambit-xbabe3` | Ubuntu 24.04, x64, systemd service | `jarambit-d1`, `jarambit-publication`, `jarambit-render` |
| xbabe4 | `jarambit-xbabe4` | Docker Linux/ARM64 on Apple Silicon | Portable general jobs; `docker` |
| xbabe1, xbabe2, xbabe4 | `jarambit-docker-<host>-<n>` | Docker Linux/x64, one runner per container (4 CPUs, 8 GiB, no swap); on xbabe4 under Rosetta in a colima VM | `jarambit-d1`, `docker`; no host label |

Each host runner also carries its physical host label (`xbabe1` through `xbabe4`). The x64 Docker
runners do not, so host-pinned jobs (`runner-policy`, the xbabe1 synthetic benchmark and the
xbabe2 publication build) never land on them.
The `runner-policy` workflow runs its portable routing checks on all four hosts
on every PR and main push. Its xbabe4 job proves it is running inside Docker.
Other general jobs can use `[self-hosted, xbabe, general]` without fixing a host.

The D1, publication and render workflows additionally select a provisioned
capability. D1 requires Rust 1.85.1 with Clippy, Python venv support and an enforced
resource bound: the real Linux systemd `memcap` wrapper on a host, or the container's own cgroup
on an x64 Docker runner (below). Publication also needs pandoc 3.1.3 and LuaHBTeX
1.17.0. Rendering needs Blender 4.2.23, ffmpeg encoders, the OCP Python environment
and a working 32 GiB/no-swap systemd scope. Do not assign a capability label before
verifying those prerequisites. Existing resource caps remain enforced.

## Routing validation

```bash
cargo test --locked --manifest-path crates/runner-policy/Cargo.toml
cargo run --locked --manifest-path crates/runner-policy/Cargo.toml
actionlint -shellcheck=
```

The checker reads all `.yml` and `.yaml` workflows, rejects duplicate YAML keys,
requires literal pool labels and rejects external reusable workflows whose
runner selection cannot be audited here. Local reusable workflows are audited
alongside their callers. Expression-based extra capability/host labels are
allowed only alongside the three fixed pool labels.

## xbabe4 Docker deployment

Source: `ops/ci/runners/xbabe4/{Dockerfile,compose.yaml}`. Deployment directory:
`/Users/agent/actions-runners/jarambit-xbabe4-docker` on `ssh xbabe4`.
The official Actions runner image runs as user `runner` in a Linux/ARM64 Docker
container with 2 CPUs, 4 GiB RAM, no swap allowance and a 512-process limit.
The container has its own persistent state volume and bounded logs. It does not
mount the Mac home directory or Docker socket. Runner credentials stay in the
Docker volume, outside the repository; no GitHub PAT is retained.

The dedicated Docker engine is Colima profile `jarambit-ci` (4 CPUs, 8 GiB,
60 GiB disk). It uses Docker context `colima-jarambit-ci` explicitly, preserving
the user's default Docker context. The container restarts when its engine does.
LaunchAgent `com.jarambit.ci-runner` starts the dedicated engine and deployment at
user login using the checked-in `start.sh` and plist.

```bash
# On xbabe4, with /opt/homebrew/bin:/usr/local/bin on PATH:
colima start --profile jarambit-ci --vm-type vz --arch aarch64 \
  --cpu 4 --memory 8 --disk 60 --runtime docker --activate=false
cd /Users/agent/actions-runners/jarambit-xbabe4-docker
docker --context colima-jarambit-ci compose build
# Initial registration only: obtain a short-lived repository registration token
# via `gh api --method POST repos/neverhuman/jarambit/actions/runners/registration-token`
# on an authenticated administration machine; pass only its token over SSH stdin
# to this initial registration command (register.py reads stdin; no PAT retained):
docker --context colima-jarambit-ci compose run --rm -T \
  --entrypoint python3 runner /opt/register-runner.py
docker --context colima-jarambit-ci compose up -d
docker --context colima-jarambit-ci compose ps
docker --context colima-jarambit-ci compose logs --tail 30
cp com.jarambit.ci-runner.plist ~/Library/LaunchAgents/
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.jarambit.ci-runner.plist
```

Starting the existing container requires no registration token. Preserve the
`jarambit-xbabe4-runner-state` volume when rebuilding. A native macOS runner is
not part of this deployment. GitHub runner inventory and actual workflow job
metadata are the authority for online/busy state and execution placement.

## x64 Docker runners (xbabe1, xbabe2, xbabe4)

Source: `ops/ci/runners/docker/{Dockerfile,register.sh,deploy.sh}`. Each runner is one persistent
container from image `jarambit-runner-x64:2.338.0` (official Actions runner 2.338.0, Ubuntu 24.04,
Rust 1.85.1 with Clippy under `/home/ubuntu`, Python venv, the OpenCascade/VTK system libraries,
zstd, ripgrep and the jankurai binary). It runs with 4 CPUs on its own cpuset, 8 GiB memory with
`--memory-swap` equal (swap 0), a 4096-process limit, all capabilities dropped and
`no-new-privileges`; it mounts neither the host home nor the Docker socket. Its labels are
`self-hosted, Linux, X64, xbabe, general, jarambit-d1, docker`.

`run_bounded.sh` runs in container mode there (`JARAMBIT_CI_CONTAINER=1`, set by the image, and
`/.dockerenv` required). The container's cgroup is the bound, and the probe still verifies
`memory.max` ≤ the profile, `memory.swap.max` = 0, a CPU affinity no wider than the profile and
nice 19 before the step body runs; a profile narrower than the container (`version`, 2 GiB)
therefore fails closed. `bounded_process.py --container` admits only a reporter in the container's
root cgroup and, at the end or on cancellation, signals every process the step started, sparing
its own ancestors (the runner). One job runs per container at a time.

```bash
# On an authenticated administration machine (repository admin), per new runner:
gh api --method POST repos/neverhuman/jarambit/actions/runners/registration-token --jq .token \
  | ssh <host> 'bash <checkout of main>/ops/ci/runners/docker/deploy.sh jarambit-docker-<host>-<n> <cpuset>'
```

On hosts whose Docker engine uses the systemd cgroup driver (xbabe1, xbabe2), `systemctl daemon-reload`
re-applies each container scope's own properties and resets swap to unlimited; the probe then refuses
bounded steps. `swap-pin.sh`, run every minute as root by `jarambit-runner-swap-pin.timer`, records
`MemorySwapMax=0` as a runtime property of every runner container's scope, which survives reloads.
`deploy.sh` installs it and checks swap 0 after start. xbabe4's colima engine uses cgroupfs.

`deploy.sh` refuses with less than 80 GB free under the Docker root, builds the image (staging
the host's `~/.jeryu/bin/jankurai`), registers once into the runner's own volume
(`jarambit-docker-<host>-<n>-state`; the token is not retained) and starts the container with
`--restart unless-stopped`. Re-running it without a token rebuilds and restarts an existing
runner. Choose a cpuset of four physical cores and their SMT siblings inside the host's
`system.slice` CPUs (`0-50,64-114`), distinct from other runners' affinity.

On xbabe4 (Apple M3 Ultra) the same linux/amd64 image runs under Rosetta in a dedicated colima
VM, profile `jarambit-x64` (16 CPUs, 64 GiB, 300 GiB disk; Docker context `colima-jarambit-x64`),
separate from the ARM64 profile `jarambit-ci`. `colima-x64.sh` starts it, and LaunchAgent
`com.jarambit.ci-runner-x64` runs that at login; the runner containers restart with the engine.
Deploy with `DOCKER_CONTEXT=colima-jarambit-x64 JANKURAI_BIN=<Linux x86-64 jankurai>` and cpusets
`0-3`, `4-7`, `8-11`, `12-15`. A bounded release build of `jarambit` there took 151 s from scratch.
