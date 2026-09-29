#!/bin/sh
# Provisions a Claude Code cloud environment: pinned mise, then the locked toolset.
#
# Invoked from the environment's "Setup script" box, not from anything in this repository, so it
# never runs locally. It runs as root before Claude Code launches, and its result is cached as a
# filesystem snapshot until the setup script, the allowed hosts, or the ~7-day expiry changes it:
# a later mise.lock bump does NOT re-run it. Tools missing from a stale snapshot are auto-installed
# by `mise run`/`mise exec`, so this is a cache warm, not the source of truth.
#
# A non-zero exit fails the session's start, so tool-install failures warn instead of exiting.
set -eu

mise_version=v2026.9.17

cd "$(dirname -- "$0")/../.."

PATH="$HOME/.local/bin:$PATH"
export PATH

if ! command -v mise >/dev/null 2>&1; then
	curl -fsSL https://mise.jdx.dev/install.sh | MISE_VERSION="$mise_version" sh
fi

mise trust --yes

if ! mise install --locked; then
	printf 'cloud-setup: mise install --locked failed; continuing without the missing tools\n' >&2
fi
