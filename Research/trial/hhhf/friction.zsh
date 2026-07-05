# hhhf/friction.zsh — the friction-button (round-25 field trial, DISPOSABLE)
#
# One keystroke, at the instant something annoys: drops a timestamped gap-marker WITHOUT
# disturbing the command line. The "why" is recovered later in the same-evening cued-recall
# debrief; this widget's only job is to pin the MOMENT before the fix-reflex erases it.
#
# Default chord: ^G  (mnemonic: Gap-log / Grr). His keymap is dense — ^I ^T ^R ^X ^L ^f and
# vicmd ; ! cs ds ys are already taken; ^G reads free. This script PRINTS the pre-existing
# binding at load so the human vets the chord live in his own shell. To move it:
#     export HHHF_FRICTION_KEY='^O'   # before sourcing
#
# Output: ${HHHF_DIR:-$HOME/.hhhf-trial}/friction.jsonl   {epoch, cwd, buffer, cursor, n}

[ -n "${_HHHF_FRICTION_LOADED:-}" ] && return
_HHHF_FRICTION_LOADED=1

zmodload zsh/datetime
: ${HHHF_DIR:=$HOME/.hhhf-trial}
: ${HHHF_FRICTION_KEY:=^G}
mkdir -p -- "$HHHF_DIR"
_hhhf_fr_log="$HHHF_DIR/friction.jsonl"
typeset -g _hhhf_fr_n=0

# Standalone JSON encoder (duplicated from capture.zsh so this file sources on its own).
_hhhf_fr_jstr() {
   local s=$1
   s=${s//\\/\\\\}
   s=${s//\"/\\\"}
   s=${s//$'\n'/\\n}
   s=${s//$'\t'/\\t}
   s=${s//$'\r'/\\r}
   s=${s//[[:cntrl:]]/ }
   print -r -- "\"$s\""
}

# ZLE widget: append a marker, flash a non-destructive confirmation. $BUFFER/$CURSOR are read,
# never written — the human keeps typing straight through the press.
_hhhf_friction() {
   (( _hhhf_fr_n++ ))
   print -r -- "{\"epoch\":$EPOCHSECONDS,\"cwd\":$(_hhhf_fr_jstr "$PWD"),\"buffer\":$(_hhhf_fr_jstr "$BUFFER"),\"cursor\":$CURSOR,\"n\":$_hhhf_fr_n}" >> "$_hhhf_fr_log"
   zle -M "[hhhf] friction #$_hhhf_fr_n marked @ $(strftime '%H:%M:%S' $EPOCHSECONDS)"
}
zle -N _hhhf_friction

print -r -- "[hhhf] friction-button chord '$HHHF_FRICTION_KEY' was bound to:" >&2
print -r -- "         viins: $(bindkey    "$HHHF_FRICTION_KEY")" >&2
print -r -- "         vicmd: $(bindkey -a "$HHHF_FRICTION_KEY")" >&2
bindkey    "$HHHF_FRICTION_KEY" _hhhf_friction    # viins / main
bindkey -a "$HHHF_FRICTION_KEY" _hhhf_friction    # vicmd
print -r -- "[hhhf] friction markers -> $_hhhf_fr_log" >&2
