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

Each runner also carries its physical host label (`xbabe1` through `xbabe4`).
The `runner-policy` workflow runs its portable routing checks on all four hosts
on every PR and main push. Its xbabe4 job proves it is running inside Docker.
Other general jobs can use `[self-hosted, xbabe, general]` without fixing a host.

The D1, publication and render workflows additionally select a provisioned
capability. D1 requires Rust 1.85.1 with Clippy, Python venv support and the real
Linux systemd `memcap` wrapper. Publication also needs pandoc 3.1.3 and LuaHBTeX
1.17.0. Rendering needs Blender 4.2.23, ffmpeg encoders, the OCP Python environment
and a working 32 GiB/no-swap systemd scope. Do not assign a capability label before
verifying those prerequisites. Existing resource caps remain enforced.

## Routing validation

```bash
python3 -m venv /tmp/jarambit-runner-policy
/tmp/jarambit-runner-policy/bin/python -m pip install -r ops/ci/requirements-runners.txt
/tmp/jarambit-runner-policy/bin/python -B ops/ci/check_runner_policy.py
/tmp/jarambit-runner-policy/bin/python -B -m unittest discover -s ops/ci/runner_tests -v
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
