FROM ghcr.io/actions/actions-runner:2.338.0

USER root
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 python3-venv ca-certificates curl build-essential \
    && rm -rf /var/lib/apt/lists/*
ENV CARGO_HOME=/opt/cargo \
    RUSTUP_HOME=/opt/rustup \
    PATH=/opt/cargo/bin:/opt/rustup/toolchains/1.85.1-aarch64-unknown-linux-gnu/bin:${PATH}
RUN curl --fail --location --silent --show-error \
      https://static.rust-lang.org/rustup/dist/aarch64-unknown-linux-gnu/rustup-init \
      --output /tmp/rustup-init \
    && chmod 700 /tmp/rustup-init \
    && /tmp/rustup-init -y --no-modify-path --profile minimal \
      --default-toolchain 1.85.1 --default-host aarch64-unknown-linux-gnu \
    && rm /tmp/rustup-init \
    && chown -R runner:runner /opt/cargo /opt/rustup
COPY register.py /opt/register-runner.py
USER runner
WORKDIR /home/runner
