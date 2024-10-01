FROM mcr.microsoft.com/devcontainers/base:jammy

RUN apt-get update && apt-get install -y \
  pkg-config \
  libudev-dev \
  bash-completion \
  flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0 \
  && rm -rf /var/lib/apt/lists/*

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# RUN . "$HOME/.cargo/env" && cargo install \
#   cargo-generate

RUN . "$HOME/.cargo/env" \
  && rustup completions bash | sudo tee /etc/bash_completion.d/rustup \
  && rustup completions bash cargo | sudo tee /etc/bash_completion.d/cargo
