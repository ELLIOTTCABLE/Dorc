#!/bin/sh
# Provision a version-PINNED checkbashisms into the git-ignored e2e/.real-tools/ dir, for the
# OPT-IN real-tools lint lane ONLY (spike/CLAUDE.md real-tools-lane-opt-in; Research/notes/27T).
# NEVER run by a default `sh e2e/run.sh` — run.sh calls this only inside its DORC_E2E_REAL_TOOLS
# block. shellcheck comes from mise (registry aqua:koalaman/shellcheck); checkbashisms is a Debian
# devscripts PERL script absent from every mise backend, so it is fetched here (task fallback (b)):
# a version-pinned copy, sha256-verified, GPL body NEVER vendored into the tracked tree.
#
# Prints the absolute .real-tools dir on stdout (for the caller to prepend to PATH); all human
# chatter goes to stderr. Idempotent: an already-present, sha256-matching copy skips the download.
set -eu

# The pin (task: record exact versions). Immutable git TAG, never a moving branch, so the bytes —
# and thus the sha256 — can never drift under us.
CB_TAG=v2.23.7
CB_URL="https://salsa.debian.org/debian/devscripts/-/raw/${CB_TAG}/scripts/checkbashisms.pl"
CB_SHA256=ef3e95808899dda7d5dfd53dc7e1f6138ee44ecc6aa0f98e51b9d449fe54bbe2

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
tools_dir="$here/.real-tools"
mkdir -p "$tools_dir"
cb_pl="$tools_dir/checkbashisms.pl"

sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  elif command -v shasum >/dev/null 2>&1; then shasum -a 256 "$1" | cut -d' ' -f1
  else echo "no sha256sum/shasum available to verify the pinned download" >&2; return 1
  fi
}

need_download=1
if [ -f "$cb_pl" ] && [ "$(sha256_of "$cb_pl")" = "$CB_SHA256" ]; then
  need_download=0
fi
if [ "$need_download" = 1 ]; then
  echo "real-tools: fetching pinned checkbashisms ${CB_TAG}" >&2
  if command -v curl >/dev/null 2>&1; then curl -fsSL -o "$cb_pl" "$CB_URL"
  elif command -v wget >/dev/null 2>&1; then wget -qO "$cb_pl" "$CB_URL"
  else echo "no curl/wget to fetch checkbashisms" >&2; exit 1
  fi
  got=$(sha256_of "$cb_pl")
  if [ "$got" != "$CB_SHA256" ]; then
    echo "real-tools: checkbashisms sha256 MISMATCH (want $CB_SHA256, got $got) — refusing" >&2
    rm -f "$cb_pl"
    exit 1
  fi
fi

# The launcher dorc's SubprocessRunner spawns as `checkbashisms`. On *nix an EXTENSIONLESS executable
# is both PATH-discoverable (tool_on_path, ext="") and execve-able (shebang honored, stdin piped
# natively) — checkbashisms runs live there. On Windows dorc's `Command::new` only appends `.exe`, so
# NO perl-script launcher is spawnable (the discovery-vs-spawn mismatch 27T records): we deliberately
# leave only checkbashisms.pl (`.PL` is not in PATHEXT), so dorc sees the tool cleanly ABSENT and a
# `DORC_E2E_REAL_TOOLS=...,checkbashisms` run FAILS LOUDLY-AND-FAST via --require-tools rather than
# hanging on a cmd.exe launcher. On Windows list only `shellcheck`; run the checkbashisms half on *nix.
# Clear any stale launcher first, so a re-run (or a cross-platform tree) can never leave a launcher
# dorc would spawn on the wrong OS.
rm -f "$tools_dir/checkbashisms" "$tools_dir/checkbashisms.cmd"
case "$(uname -s 2>/dev/null || echo unknown)" in
  MINGW* | MSYS* | CYGWIN*)
    echo "real-tools: on Windows checkbashisms is *nix-only for the lane (dorc cannot spawn a perl-script launcher); provisioned .pl for manual \`perl\` use only" >&2
    ;;
  *)
    printf '#!/bin/sh\nexec perl "%s/checkbashisms.pl" "$@"\n' "$tools_dir" > "$tools_dir/checkbashisms"
    chmod +x "$tools_dir/checkbashisms"
    ;;
esac

echo "$tools_dir"
