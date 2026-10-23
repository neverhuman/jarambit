#!/usr/bin/env bash
# Build the Linux/x64 Docker runner image and start one runner container on this host.
# usage: deploy.sh <name> <cpuset>       e.g. deploy.sh jarambit-docker-xbabe2-1 36-39,100-103
# A first start reads one short-lived repository registration token on stdin; it is passed to
# config.sh and not retained (the runner's credentials stay in its own volume). Restarting an
# existing runner needs no token. The container restarts with the Docker engine.
# The image is linux/amd64 on every host; on Apple Silicon (xbabe4) run it in a colima VM with
# Rosetta (DOCKER_CONTEXT=colima-<profile>) and point JANKURAI_BIN at the Linux x86-64 binary.
set -euo pipefail
name=$1 cpuset=$2
[[ $name =~ ^jarambit-docker-xbabe[1-4]-[1-9][0-9]*$ ]] || { echo "bad runner name: $name" >&2; exit 2; }
[[ $cpuset =~ ^[0-9,-]+$ ]] || { echo "bad cpuset: $cpuset" >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
image=jarambit-runner-x64:2.338.0
volume=$name-state
# Image, work volume and caches need room; keep the host's other services above their guards.
root=$(docker info --format '{{.DockerRootDir}}')
if [[ -d $root ]]; then
  free_gb=$(df -BG --output=avail "$root" | tail -1 | tr -dc 0-9)
else  # engine inside a VM (colima): measure its own disk
  free_gb=$(docker run --rm --platform linux/amd64 --entrypoint df ubuntu:24.04 -BG --output=avail /etc/hostname | tail -1 | tr -dc 0-9)
fi
(( free_gb >= 80 )) || { echo "only ${free_gb}G free for the Docker engine; refusing" >&2; exit 1; }

build=$HOME/actions-runners/jarambit-docker-build
mkdir -p "$build"
cp "$here/Dockerfile" "$here/register.sh" "$build/"
cp "$here/cad-battery-requirements.txt" "$build/"
install -m 755 "${JANKURAI_BIN:-$HOME/.jeryu/bin/jankurai}" "$build/jankurai"
docker build --pull -q --platform linux/amd64 -t "$image" "$build" >/dev/null
# Refuse an image missing what the general-pool jobs call (jankurai, pinned Clippy, CAD battery OCP).
docker run --rm --platform linux/amd64 --entrypoint bash "$image" -c \
  'test -x /home/ubuntu/.jeryu/bin/jankurai && /home/ubuntu/.jeryu/bin/jankurai --version >/dev/null &&
   /home/ubuntu/.rustup/toolchains/1.85.1-x86_64-unknown-linux-gnu/bin/cargo-clippy --version >/dev/null &&
   /home/ubuntu/.venvs/jarambit-sheath/bin/python -c "import OCP"' || { echo "image smoke check failed" >&2; exit 1; }

docker volume inspect "$volume" >/dev/null 2>&1 || docker volume create "$volume" >/dev/null
if ! docker run --rm -v "$volume:/home/runner" --entrypoint test "$image" -f /home/runner/.runner; then
  # A fresh volume shadows the image's runner files; copy them in once, then register.
  docker run --rm -u 0 -v "$volume:/state" --entrypoint bash "$image" -c \
    'cp -a /home/runner/. /state/ && chown -R runner:runner /state'
  docker run --rm -i -v "$volume:/home/runner" --entrypoint /opt/register-runner.sh "$image" "$name"
fi
docker rm -f "$name" >/dev/null 2>&1 || true
docker run -d --name "$name" --platform linux/amd64 --restart unless-stopped --init \
  --cpus 4 --cpuset-cpus "$cpuset" --memory 8g --memory-swap 8g --pids-limit 4096 \
  --cap-drop ALL --security-opt no-new-privileges \
  --log-opt max-size=10m --log-opt max-file=3 \
  --label jarambit.ci.runner=1 \
  -v "$volume:/home/runner" "$image" ./run.sh >/dev/null
# systemd cgroup driver: `systemctl daemon-reload` would reset the scope's swap to unlimited, and
# run_bounded.sh would then refuse every bounded step. Pin it now and keep it pinned (swap-pin.sh).
if [[ $(docker info --format '{{.CgroupDriver}}') == systemd ]]; then
  sudo -n install -D -m 755 "$here/swap-pin.sh" /usr/local/libexec/jarambit-runner-swap-pin.sh
  sudo -n install -m 644 "$here/jarambit-runner-swap-pin.service" "$here/jarambit-runner-swap-pin.timer" /etc/systemd/system/
  sudo -n systemctl daemon-reload
  sudo -n systemctl enable --now jarambit-runner-swap-pin.timer >/dev/null
  sudo -n systemctl start jarambit-runner-swap-pin.service
fi
[[ $(docker exec "$name" cat /sys/fs/cgroup/memory.swap.max) == 0 ]] || { echo "$name: swap not pinned to 0" >&2; exit 1; }
echo "$name started on cpus $cpuset ($(docker inspect -f '{{.State.Status}}' "$name"))"
