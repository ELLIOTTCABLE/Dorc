#!/bin/sh
# hhhf/record.sh — turnkey session recorder (round-25 field trial, DISPOSABLE)
#
# Starts an asciinema-recorded, fully-instrumented INTERACTIVE zsh, then flattens the recording
# to an LLM-readable transcript on exit. One command bootstraps the whole HHHF for a session:
#   - the JSONL command-spine (capture.zsh)        <- authoritative
#   - the ^G friction-button (friction.zsh)
#   - the asciinema recording + strip.sh extractor <- audit / redundancy rail
#
# FIDELITY GATE: asciinema gives a real PTY and we launch `zsh -i` (interactive). A `zsh -c`
# script would false-green the whole instrument — the preexec/precmd/ZLE paths never fire
# non-interactively. Validate this the way 252-A3 prescribes: a human, 5 min, a throwaway zsh.
#
# LOCATION-AGNOSTIC: no host is hardcoded. His dotfiles are sourced when present (local WSL /
# macOS) and simply skipped when absent (the vanilla Debian box) — instrumenting a bare zsh.
# WHERE his real-PTY session lives is an OPEN downstream decision (256); this runs in either.
#
# Injection is via a ZDOTDIR shim, so nothing in ~/System is touched.
#
# Usage:  record.sh            start an instrumented recorded session
#         record.sh --help
# Env:    HHHF_DIR (bundle root, default ~/.hhhf-trial) · HHHF_FRICTION_KEY (default ^G)

set -eu

here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)

case "${1:-}" in
   -h|--help)
      sed -n '2,22p' "$0" | sed 's/^# \{0,1\}//'
      exit 0 ;;
esac

command -v zsh >/dev/null 2>&1 || { echo "record.sh: zsh not found (this instrument is zsh-only)" >&2; exit 1; }
if ! command -v asciinema >/dev/null 2>&1; then
   echo "record.sh: asciinema not found. Install it, then re-run." >&2
   echo "  The command-spine + friction-button still work WITHOUT it — in any interactive zsh:" >&2
   echo "    source \"$here/capture.zsh\"; source \"$here/friction.zsh\"" >&2
   exit 1
fi

: "${HHHF_DIR:=$HOME/.hhhf-trial}"
session="$HHHF_DIR/session-$(date +%Y%m%d-%H%M%S)"
shim="$session/zdotdir"
cast="$session/session.cast"
transcript="$session/transcript.txt"
mkdir -p -- "$shim"

# The whole session's spine + markers land in this one bundle dir.
HHHF_DIR="$session"
export HHHF_DIR

# ZDOTDIR shim: chain to his real config when it exists, then stack our instruments on top.
cat > "$shim/.zshenv" <<'EOF'
[ -f "$HOME/.zshenv" ] && . "$HOME/.zshenv"
EOF
cat > "$shim/.zshrc" <<EOF
[ -f "\$HOME/.zshrc" ] && source "\$HOME/.zshrc"
source "$here/capture.zsh"
source "$here/friction.zsh"
EOF

finish() {
   trap - EXIT INT TERM
   [ -f "$cast" ] && sh "$here/strip.sh" "$cast" "$transcript" || true
   echo "" >&2
   echo "[hhhf] session bundle: $session" >&2
   echo "         commands.jsonl  (spine, authoritative)" >&2
   echo "         friction.jsonl  (^G markers)" >&2
   echo "         session.cast    (raw asciinema)" >&2
   echo "         transcript.txt  (ANSI-stripped)" >&2
}
trap finish EXIT INT TERM

echo "[hhhf] recording -> $cast   (exit the shell to stop; ^G marks friction)" >&2
ZDOTDIR="$shim" asciinema rec --title "hhhf-trial $(date +%Y%m%d-%H%M%S)" --command "zsh -i" "$cast"
