#!/usr/bin/env dorc-run
# dorc-lang/v0.2
#
#=========================== FROZEN EVIDENCE ==============================
# STRAWMAN - imagination-tier.  Features shown here MAY NOT EXIST.  This
# file is NOT RUNNABLE and must NEVER be executed, in whole or in part, by
# any human or agent.  It is a design-target written to be read.  No
# format-compat promises: pre-user, every spelling here is rename-in-place.
#==========================================================================
#
# seed-tiles.sh - ensure a PersistentVolume holds the pinned tileset, and
# that its on-disk layout has been migrated.  Baked into the init-container
# image at /usr/local/bin/seed-tiles.sh and used as its ENTRYPOINT; the
# kernel honours the shebang, so Kubernetes is not asked to cooperate in any
# way.  Manifest: k8s-initcontainer-pv-seed.manifest.yaml
#
# This runs headless, in an Alpine image, as an init container that
# Kubernetes may re-execute at any time and gives no convergence machinery
# of its own.  Its docs say so plainly: "Because init containers can be
# restarted, retried, or re-executed, init container code should be
# idempotent."  Every guard below is that sentence, answered.

set -eu

DATA=${TILE_DATA_DIR:-/var/lib/tiles}
WANT=${TILESET_VERSION:?the Deployment must pin a tileset version}
ORIGIN=${TILESET_ORIGIN:-https://tiles.example.net}

lock=$DATA/.seeding-$WANT

# A killed init container gets SIGKILL, which skips every trap we could
# install, so an interrupted run leaves a staging tree and a held lock
# behind.  Sweeping them is the first thing a re-run must do and the first
# thing nothing in Kubernetes will do for us.
find "$DATA" -maxdepth 1 -type d -name '.stage.*'    -mmin +30 -exec rm -rf {} +
find "$DATA" -maxdepth 1 -type d -name '.seeding-*'  -mmin +30 -exec rmdir {} +

# The payload directory is only ever created by renaming a fully-verified
# staging tree into place.  That is what makes the naive-looking test sound:
# its existence IS its completeness, because nothing can create it half-made.
if [ ! -d "$DATA/v/$WANT" ]; then
   if mkdir "$lock" 2>/dev/null; then
      stage=$(mktemp -d "$DATA/.stage.XXXXXX")
      curl -fsS -o "$stage/tiles.tar.gz" "$ORIGIN/$WANT/tiles.tar.gz"
      curl -fsS -o "$stage/SHA256SUMS"   "$ORIGIN/$WANT/SHA256SUMS"
      ( cd "$stage" && sha256sum -c SHA256SUMS )
      mkdir -p "$stage/payload" "$DATA/v"
      tar -C "$stage/payload" -xzf "$stage/tiles.tar.gz"

      # Re-test under the lock.  A sweep that reclaimed a slow-but-live
      # lock could have let a second writer in, and mv into an existing
      # directory nests instead of replacing.  The window is one syscall
      # wide and the loser only wastes a download.
      if [ -d "$DATA/v/$WANT" ]; then rm -rf "$stage/payload"
      else mv "$stage/payload" "$DATA/v/$WANT"
      fi

      rm -rf "$stage"
      rmdir "$lock"
   else
      # Somebody else is mid-fetch on this same volume.  ReadWriteOnce does
      # not prevent that - it binds the volume to one node, and two pods on
      # one node may both mount it; the docs are explicit that access modes
      # "do not enforce write protection once the storage has been mounted".
      # Waiting beats a duplicate 40 GiB download.
      n=0
      while [ ! -d "$DATA/v/$WANT" ]; do
         n=$((n + 1))
         [ "$n" -lt 300 ] || {
            printf 'gave up waiting for a concurrent seed of %s\n' "$WANT" >&2
            exit 1
         }
         sleep 2
      done
   fi
fi

# Deliberately a separate question from the fetch: a kill between the two
# leaves a complete payload behind a stale pointer, and this block alone
# repairs that without re-downloading anything.
if [ "$(readlink "$DATA/current" 2>/dev/null)" != "v/$WANT" ]; then
   rm -f "$DATA/.next"
   ln -s "v/$WANT" "$DATA/.next"
   mv "$DATA/.next" "$DATA/current"
fi

# The app's own tool, which is the only thing that knows its own data.
tileserv-migrate --data "$DATA/current" --to-layout 3


# ===========================================================================
# The oracle, written by whoever ships tileserv-migrate.  Two minutes of
# work, exactly once, and every book that seeds this data inherits it.
# ===========================================================================

tileserv_migrate__is_converged() {
   data=; want=
   while [ $# -gt 0 ]; do
      case "$1" in
      --data)      data=${2-}; shift 2 || return 2 ;;
      --to-layout) want=${2-}; shift 2 || return 2 ;;
      *)           return 2 ;;
      esac
   done
   [ -n "$data" ] && [ -n "$want" ] || return 2

   store : sm.tiles.Store = "$data"
   [ "$(tileserv-migrate --data "$store" --layout-version)" = "$want" ] \
      : sm.tiles.Store:"$store"@layout
}
