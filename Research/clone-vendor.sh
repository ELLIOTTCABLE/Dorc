#!/bin/sh
# clone-vendor.sh — rebuild the read-only research-source checkouts under ./Vendor/
# from the machine-queryable map in ./vendor.json.
#
# These are not submodules and are not committed (Research/.gitignore ignores
# Vendor/); vendor.json + this script are the only things git tracks. Run on a new
# machine to rebuild the corpus, then let syncthing/git ignore ./Vendor/ itself.
# Idempotent and self-healing: an already-correct checkout is left alone; a stale
# commit is fetched in place; a missing or corrupt checkout (e.g. an empty .git
# shell left by an interrupted clone) is re-cloned from scratch.
#
# Usage:
#   ./clone-vendor.sh [name-filter ...]   # clone/update (no filter = everything)
#   ./clone-vendor.sh --table             # render the manifest markdown table to stdout
#   ./clone-vendor.sh --list              # list the local paths, one per line
#   ./clone-vendor.sh --help
#
# A name-filter is a substring matched against the local path, so `colis` rebuilds
# the whole CoLiS set and `shellcheck` just the one.
#
# Env:
#   VENDOR_DIR=path   clone root (default: vendorDir from vendor.json, else ./Vendor)
#   RETRIES=n         attempts per network op before giving up (default 5)
#   BASE_DELAY=s      first backoff delay; doubles each retry, capped (default 2)
#   MAX_DELAY=s       backoff cap (default 60)
#
# Uses `gh repo clone` so clones inherit your gh auth (no anonymous rate-limit);
# heavy repos (those with a "filter" in vendor.json) clone with a partial-clone
# blob filter. Network ops are wrapped in exponential backoff + jitter so a
# 28-repo rebuild rides out transient failures and GitHub secondary-rate-limits.
#
# Requires: gh (authenticated), jq, git. Provision the pinned versions with
# `mise install` (see ../mise.toml).

set -eu

die() { echo "clone-vendor: $*" >&2; exit 1; }
need() {
   command -v "$1" >/dev/null 2>&1 \
      || die "$1 not found on PATH — run 'mise install' (see ../mise.toml)"
}

# Windows jq.exe writes stdout in text mode (\n -> \r\n); the stray CR corrupts the
# last @tsv field and every $(jq ...) capture (VENDOR_DIR, etc.). Strip it. No-op on
# *nix, where jq already emits \n.
jqr() { jq "$@" | tr -d '\r'; }

usage() {
   cat <<'EOF'
clone-vendor.sh — rebuild Research/Vendor/ from vendor.json (commit-pinned).

  ./clone-vendor.sh [name-filter ...]   clone/update (no filter = everything)
  ./clone-vendor.sh --table             render the manifest markdown table
  ./clone-vendor.sh --list              list local paths, one per line
  ./clone-vendor.sh --help

A name-filter is a substring matched against the local path.
Env: VENDOR_DIR, RETRIES, BASE_DELAY, MAX_DELAY. Requires: gh, jq, git (mise install).
EOF
}

# --help needs no tools; answer it before requiring or bootstrapping any.
for _arg in "$@"; do case $_arg in -h | --help) usage; exit 0 ;; esac; done

# Our deps are mise-pinned (../mise.toml). A non-interactive shell that never ran
# `mise activate` has no mise shims on PATH, so jq is invisible even when installed
# — re-exec once through `mise`, which puts the project toolset on PATH. (gh/git are
# usually system-wide already; jq is the canonical "is the toolset active?" probe.)
if [ -z "${CLONE_VENDOR_VIA_MISE:-}" ] && ! command -v jq >/dev/null 2>&1 \
   && command -v mise >/dev/null 2>&1; then
   export CLONE_VENDOR_VIA_MISE=1
   exec mise exec -- sh "$0" "$@"   # `sh "$0"`: robust to bare $0 / missing exec bit
fi

cd "$(dirname "$0")" || exit 1
MANIFEST=vendor.json

RETRIES=${RETRIES:-5}
BASE_DELAY=${BASE_DELAY:-2}
MAX_DELAY=${MAX_DELAY:-60}

# Run a command, retrying on failure with exponential backoff + small jitter.
# Jitter desynchronizes parallel rebuilds and softens GitHub secondary-rate-limits.
run_retry() {
   label=$1; shift
   delay=$BASE_DELAY; attempt=1
   while :; do
      "$@" && return 0       # NB: `if "$@"; then` would make $? below always 0
      status=$?
      if [ "$attempt" -ge "$RETRIES" ]; then
         echo "  $label: gave up after $attempt attempts (exit $status)" >&2
         return "$status"
      fi
      jitter=$(awk 'BEGIN { srand(); print int(rand() * 3) }')
      wait=$((delay + jitter))
      echo "  $label: attempt $attempt failed (exit $status); retry in ${wait}s" >&2
      sleep "$wait"
      delay=$((delay * 2)); [ "$delay" -gt "$MAX_DELAY" ] && delay=$MAX_DELAY
      attempt=$((attempt + 1))
   done
}

# A fresh clone: wipe whatever is there, clone with auth, partial-filter the heavy
# repos. --no-checkout because we always detach onto the pinned commit ourselves.
do_clone() {
   _repo=$1; _dir=$2; _filter=$3
   rm -rf "$_dir" || return 1
   mkdir -p "$(dirname "$_dir")"
   if [ -n "$_filter" ]; then
      gh repo clone "$_repo" "$_dir" -- --no-checkout --quiet --filter="$_filter"
   else
      gh repo clone "$_repo" "$_dir" -- --no-checkout --quiet
   fi
}

at_commit() { [ "$(git -C "$1" rev-parse -q --verify HEAD 2>/dev/null || true)" = "$2" ]; }
is_repo()   { git -C "$1" rev-parse --git-dir >/dev/null 2>&1; }

# Bring one checkout to its pinned commit, doing the least work that suffices.
ensure_one() {
   path=$1; repo=$2; commit=$3; filter=$4
   dir=$VENDOR_DIR/$path

   if at_commit "$dir" "$commit"; then echo "ok     $path"; return 0; fi

   # A valid repo at the wrong commit: fetch the pin and detach, cheaper than a
   # full re-clone of a heavy repo whose manifest commit just moved.
   if is_repo "$dir"; then
      if run_retry "fetch $path" git -C "$dir" fetch --quiet origin "$commit" \
         && git -C "$dir" checkout --quiet --detach "$commit"; then
         echo "update $path"; return 0
      fi
      echo "  $path: in-place update failed; re-cloning" >&2
   fi

   echo "clone  $path"
   if ! run_retry "clone $path" do_clone "$repo" "$dir" "$filter"; then return 1; fi
   # Detach onto the pin; partial/full clones may need the commit fetched first.
   if ! git -C "$dir" checkout --quiet --detach "$commit" 2>/dev/null; then
      run_retry "fetch $path" git -C "$dir" fetch --quiet origin "$commit" || return 1
      git -C "$dir" checkout --quiet --detach "$commit" || return 1
   fi
}

render_table() {
   echo "<!-- generated by clone-vendor.sh --table from vendor.json; edit vendor.json, not this -->"
   jqr -r '
      "| Repo | Lang | Q | License | Dorc relevance |",
      "|---|---|---|---|---|",
      (.repos | sort_by(.path)[]
         | "| `\(.repo)` | \(.lang) | \(.grade // "—") | \(.license // "—") | \(.relevance) |")
   ' "$MANIFEST"
}

MODE=clone
FILTERS=
while [ "$#" -gt 0 ]; do
   case $1 in
      -h | --help)  usage; exit 0 ;;
      -t | --table) MODE=table ;;
      -l | --list)  MODE=list ;;
      --)           shift; break ;;
      -*)           die "unknown flag: $1 (try --help)" ;;
      *)            FILTERS="$FILTERS $1" ;;
   esac
   shift
done
for arg in "$@"; do FILTERS="$FILTERS $arg"; done

# --help needs nothing; everything else reads vendor.json via jq.
[ -f "$MANIFEST" ] || die "missing $MANIFEST next to this script"
need jq
VENDOR_DIR=${VENDOR_DIR:-$(jqr -r '.vendorDir // "Vendor"' "$MANIFEST")}

case $MODE in
   table) render_table; exit 0 ;;
   list)  jqr -r '.repos[].path' "$MANIFEST"; exit 0 ;;
esac

# Cloning additionally needs gh + git.
need gh; need git
command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1 \
   || echo "clone-vendor: warning — gh may not be authenticated (check: gh auth status)" >&2

# Forbid git from walking above the vendor root when probing a checkout's state.
# A corrupt/empty .git shell would otherwise resolve to the enclosing Dorc repo
# (unrelated HEAD), risking an in-place fetch into the wrong repository.
mkdir -p "$VENDOR_DIR"
GIT_CEILING_DIRECTORIES=$(CDPATH= cd "$VENDOR_DIR" && pwd) && export GIT_CEILING_DIRECTORIES

TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT
# Tab-separated so paths never re-split; the trailing empty filter field survives.
jqr -r '.repos[] | [.path, .repo, .commit, (.filter // "")] | @tsv' "$MANIFEST" > "$TMP"

# Read from a file, not a pipe: a piped `while` runs in a subshell and would lose
# the tallies. TAB is a literal tab so the empty filter field reads as empty.
fails=0; matched=0
TAB=$(printf '\t')
while IFS="$TAB" read -r path repo commit filter; do
   [ -n "$path" ] || continue
   if [ -n "$FILTERS" ]; then
      keep=0
      for f in $FILTERS; do case $path in *"$f"*) keep=1 ;; esac; done
      [ "$keep" -eq 1 ] || continue
   fi
   matched=$((matched + 1))
   ensure_one "$path" "$repo" "$commit" "$filter" || fails=$((fails + 1))
done < "$TMP"

[ "$matched" -gt 0 ] || die "no repos matched filter:${FILTERS:- (none given)}"
[ "$fails" -eq 0 ] || die "$fails of $matched repo(s) failed"
echo "done ($matched repo(s))"
