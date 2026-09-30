#!/bin/sh
# Provisions a Claude Code cloud environment: lineage branches, pinned mise, trusted configs, then
# the locked toolset.
#
# Invoked from the environment's "Setup script" box, not from anything in this repository, so it
# never runs locally. It runs as root before Claude Code launches, and its result is cached as a
# filesystem snapshot until the setup script, the allowed hosts, or the ~7-day expiry changes it:
# a later mise.lock bump does NOT re-run it. Tools missing from a stale snapshot are auto-installed
# by `mise run`/`mise exec`, so this is a cache warm, not the source of truth.
#
# A non-zero exit fails the session's start, so everything after the mise install warns instead.
set -eu

mise_version=v2026.9.17

warn() { printf 'cloud-setup: %s\n' "$*" >&2; }

cd "$(dirname -- "$0")/../.."

# hk's `default_branch` and internal-tooling's lineage name these as local branches, but a cloud
# clone is shallow and carries only the session's own branch. `hk --pr` needs a merge-base, which a
# shallow history usually cannot reach; without one it silently widens to every file. An existing
# local branch is never moved. fetch exits 0 even when it rejects a ref, hence the re-check.
if [ "$(git rev-parse --is-shallow-repository)" = true ]; then
	git fetch -q --unshallow --no-tags origin || warn "could not unshallow; merge-bases may not resolve"
fi
for branch in main ai/main; do
	git rev-parse -q --verify "refs/heads/$branch" >/dev/null && continue
	git fetch -q --update-shallow --no-tags origin "refs/heads/$branch:refs/remotes/origin/$branch" 2>/dev/null || :
	if git rev-parse -q --verify "refs/remotes/origin/$branch" >/dev/null; then
		git branch -q --no-track "$branch" "refs/remotes/origin/$branch" || warn "could not create $branch"
	else
		warn "origin has no $branch; hk --pr will diff against a different base"
	fi
done

PATH="$HOME/.local/bin:$PATH"
export PATH

if ! command -v mise >/dev/null 2>&1; then
	curl -fsSL https://mise.jdx.dev/install.sh | MISE_VERSION="$mise_version" sh
fi

# Every tracked config, nested ones included: mise refuses an untrusted config at the moment a
# task first reaches it, which for the nested lanes is long after setup.
git ls-files -- mise.toml '*/mise.toml' ':!:Research/quarantine-DO-NOT-READ' |
	while IFS= read -r config; do
		mise trust --yes "$config" || warn "could not trust $config"
	done

if ! mise install --locked; then
	printf 'cloud-setup: mise install --locked failed; continuing without the missing tools\n' >&2
fi

# A fresh clone carries no hooks, so without this every commit here skips the pre-commit lint floor
# and the commit-msg gitlabels rules. This clone is the container's own; its .git/config is shared
# with nothing.
mise run hk-install || warn "could not install the git hooks; commits here are unchecked"
