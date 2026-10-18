FROM ghcr.io/actions/actions-runner:2.338.0

USER root
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 python3-venv ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY register.py /opt/register-runner.py
USER runner
WORKDIR /home/runner
