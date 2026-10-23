# Linux/x64 Docker runner for the general pool (ops/ci/RUNNERS.md), built and started by deploy.sh.
# The jobs read the pinned toolchain from /home/ubuntu (select_rust_toolchain.py and the
# hard-coded workflow paths), so HOME is /home/ubuntu. The runner's own state stays in
# /home/runner, a per-runner volume. run_bounded.sh uses the container's cgroup as its bound
# (JARAMBIT_CI_CONTAINER=1) and still verifies it before every bounded step.
FROM ghcr.io/actions/actions-runner:2.338.0

USER root
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       python3 python3-venv python3-pip python3-dev build-essential pkg-config ca-certificates curl \
       git jq zstd ripgrep perl util-linux procps \
       libgl1 libglu1-mesa libegl1 libxrender1 libxext6 libsm6 libxi6 libxkbcommon0 libfontconfig1 \
       libxcursor1 libxft2 libxinerama1 \
    && rm -rf /var/lib/apt/lists/* \
    && install -d -o runner -g runner /home/ubuntu /home/ubuntu/.jeryu /home/ubuntu/.jeryu/bin
ENV HOME=/home/ubuntu \
    RUSTUP_HOME=/home/ubuntu/.rustup \
    CARGO_HOME=/home/ubuntu/.cargo \
    JARAMBIT_CI_CONTAINER=1
USER runner
RUN curl --fail --location --silent --show-error \
      https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init \
      --output /tmp/rustup-init \
    && chmod 700 /tmp/rustup-init \
    && /tmp/rustup-init -y --no-modify-path --profile minimal --component clippy \
      --default-toolchain 1.85.1 --default-host x86_64-unknown-linux-gnu \
    && rm /tmp/rustup-init
ENV PATH=/home/ubuntu/.cargo/bin:${PATH}
# OpenCascade Python for the CAD battery's STEP B-rep check (battery.rs run_step_brep_check tries
# /home/ubuntu/.venvs/jarambit-sheath first), at the host runners' versions.
COPY --chown=runner:runner cad-battery-requirements.txt /home/ubuntu/.venvs/cad-battery-requirements.txt
RUN python3 -m venv /home/ubuntu/.venvs/jarambit-sheath \
    && /home/ubuntu/.venvs/jarambit-sheath/bin/python -m pip install --no-cache-dir -q -r /home/ubuntu/.venvs/cad-battery-requirements.txt \
    && /home/ubuntu/.venvs/jarambit-sheath/bin/python -c "from OCP.STEPControl import STEPControl_Reader"
COPY --chmod=755 register.sh /opt/register-runner.sh
WORKDIR /home/runner
