#!/bin/sh
# DEV-ONLY scratch (not committed): stamp inert PATH-shims into a case's mocks/ dir.
#   sh mkmocks.sh <case-dir> <name> [name...]
# Each shim logs `ran: <name> <args>` to $DORC_LOG and exits 0 — nothing mutating.
set -eu
dir=$1; shift
mocks="$dir/mocks"
mkdir -p "$mocks"
for name in "$@"; do
   cat > "$mocks/$name" <<'EOF'
#!/bin/sh
# INERT mock (ap-2 exec harness): logs that it ran, mutates NOTHING, exits 0. The
# book calls the real name; under the harness's PATH=mocks-only this shim is the only
# thing that runs, so executing a rendered apply is provably safe. `${0##*/}` (not
# `basename`) reads the invoked name with no external command (PATH is mocks-only).
printf 'ran: %s %s\n' "${0##*/}" "$*" >>"$DORC_LOG"
exit 0
EOF
   chmod +x "$mocks/$name"
done
echo "stamped $* into $mocks"
