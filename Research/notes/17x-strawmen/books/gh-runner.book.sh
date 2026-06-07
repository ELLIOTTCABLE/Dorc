#!/usr/bin/env bash
# setup-runner.sh
# Installs prerequisites on an Ubuntu 24.04 VM that will act as a
# GitHub Actions self-hosted runner.
#
# HOW TO USE:
#   1. Run this script as root:  sudo bash setup-runner.sh
#   2. After the script completes, register the runner MANUALLY:
#        su - github-runner
#        cd ~/actions-runner
#        ./config.sh --url https://github.com/<owner>/<repo> \
#                    --token <REGISTRATION_TOKEN>
#        sudo ./svc.sh install
#        sudo ./svc.sh start
#
# NOTE: The registration token must NOT be stored in the repo (GitHub
#       recommends it is single-use and short-lived). Obtain it from:
#       Settings → Actions → Runners → New self-hosted runner.
set -euo pipefail

RUNNER_USER="github-runner"
RUNNER_HOME="/home/${RUNNER_USER}"
RUNNER_DIR="${RUNNER_HOME}/actions-runner"
RUNNER_VERSION="2.316.1"
RUNNER_ARCH="$(uname -m)"
case "$RUNNER_ARCH" in
  x86_64)  RUNNER_ARCH="x64" ;;
  aarch64) RUNNER_ARCH="arm64" ;;
  *) echo "Unsupported architecture: $RUNNER_ARCH" >&2; exit 1 ;;
esac

require_root() {
  if [[ "${EUID:-$(id -u)}" -ne 0 ]]; then
    echo "must be run as root" >&2
    exit 1
  fi
}

install_packages() {
  apt-get update
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    ca-certificates curl git jq openssh-client unzip
}

install_docker() {
  if command -v docker &>/dev/null; then
    echo "docker already installed"
    return
  fi
  install -m 0755 -d /etc/apt/keyrings
  curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
    | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
  chmod a+r /etc/apt/keyrings/docker.gpg
  echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
    https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" \
    > /etc/apt/sources.list.d/docker.list
  apt-get update
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    docker-ce docker-ce-cli containerd.io docker-buildx-plugin
  systemctl enable --now docker
}

create_runner_user() {
  id -u "$RUNNER_USER" &>/dev/null && return 0
  useradd -m -s /bin/bash "$RUNNER_USER"
  usermod -aG docker "$RUNNER_USER"
}

download_runner() {
  if [[ -f "${RUNNER_DIR}/config.sh" ]]; then
    echo "Runner already downloaded"
    return
  fi
  local url="https://github.com/actions/runner/releases/download/v${RUNNER_VERSION}/actions-runner-linux-${RUNNER_ARCH}-${RUNNER_VERSION}.tar.gz"
  sudo -u "$RUNNER_USER" bash -c "
    mkdir -p '${RUNNER_DIR}'
    cd '${RUNNER_DIR}'
    curl -fsSL '${url}' | tar -xz
  "
}

install_runner_deps() {
  "${RUNNER_DIR}/bin/installdependencies.sh" || true
}

print_next_steps() {
  cat <<MSG
 Runner prerequisites installed.

 NEXT (manual steps — do NOT automate the token):
   su - ${RUNNER_USER}
   cd ~/actions-runner
   ./config.sh --url https://github.com/<owner>/<repo> \\
               --token <REGISTRATION_TOKEN> \\
               --labels deploy --unattended
   sudo ./svc.sh install
   sudo ./svc.sh start

 The REGISTRATION_TOKEN is single-use. Get it from:
   GitHub - Settings - Actions - Runners - New self-hosted runner
MSG
}

main() {
  require_root
  install_packages
  install_docker
  create_runner_user
  download_runner
  install_runner_deps
  print_next_steps
}

main "$@"
