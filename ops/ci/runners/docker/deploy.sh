#!/usr/bin/env bash
# Build the Linux/x64 Docker runner image and start one runner container on this host.
# usage: deploy.sh <name> <cpuset>       e.g. deploy.sh jarambit-docker-xbabe2-1 36-39,100-103
# A first start reads one short-lived repository registration token on stdin; it is passed to
# config.sh and not retained (the runner's credentials stay in its own volume). Restarting an
# existing runner needs no token. The container restarts with the Docker engine.
set -euo pipefail
name=$1 cpuset=$2
[[ $name =~ ^jarambit-docker-xbabe[1-4]-[1-9][0-9]*$ ]] || { echo "bad runner name: $name" >&2; exit 2; }
[[ $cpuset =~ ^[0-9,-]+$ ]] || { echo "bad cpuset: $cpuset" >&2; exit 2; }
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
image=jarambit-runner-x64:2.338.0
volume=$name-state
# Image, work volume and caches need room; keep the host's other services above their guards.
free_gb=$(df -BG --output=avail "$(docker info --format '{{.DockerRootDir}}')" | tail -1 | tr -dc 0-9)
(( free_gb >= 80 )) || { echo "only ${free_gb}G free under the Docker root; refusing" >&2; exit 1; }

build=$HOME/actions-runners/jarambit-docker-build
mkdir -p "$build"
cp "$here/Dockerfile" "$here/register.sh" "$build/"
install -m 755 "$HOME/.jeryu/bin/jankurai" "$build/jankurai"
docker build --pull -q -t "$image" "$build" >/dev/null

docker volume inspect "$volume" >/dev/null 2>&1 || docker volume create "$volume" >/dev/null
if ! docker run --rm -v "$volume:/home/runner" --entrypoint test "$image" -f /home/runner/.runner; then
  # A fresh volume shadows the image's runner files; copy them in once, then register.
  docker run --rm -u 0 -v "$volume:/state" --entrypoint bash "$image" -c \
    'cp -a /home/runner/. /state/ && chown -R runner:runner /state'
  docker run --rm -i -v "$volume:/home/runner" --entrypoint /opt/register-runner.sh "$image" "$name"
fi
docker rm -f "$name" >/dev/null 2>&1 || true
docker run -d --name "$name" --restart unless-stopped --init \
  --cpus 4 --cpuset-cpus "$cpuset" --memory 8g --memory-swap 8g --pids-limit 4096 \
  --cap-drop ALL --security-opt no-new-privileges \
  --log-opt max-size=10m --log-opt max-file=3 \
  --label jarambit.ci.runner=1 \
  -v "$volume:/home/runner" "$image" ./run.sh >/dev/null
echo "$name started on cpus $cpuset ($(docker inspect -f '{{.State.Status}}' "$name"))"
