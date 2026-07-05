# hhhf/capture.zsh — the authoritative transcript spine (round-25 field trial, DISPOSABLE)
#
# Stacks a preexec/precmd pair ADDITIVELY onto whatever hooks the shell already runs
# (add-zsh-hook appends; his zshrc already registers a preexec). Emits one JSONL object
# per submitted command:  {cmd, expanded, cwd, prev-rc, epoch}
#
# Source this into a REAL interactive zsh (record.sh does it via a ZDOTDIR shim; or, on any
# box, `source capture.zsh` by hand). A `zsh -c` non-interactive run will NOT fire these hooks
# — that path false-greens the instrument, so it is never a valid smoke-test.
#
# Output: ${HHHF_DIR:-$HOME/.hhhf-trial}/commands.jsonl   (append-only)

[ -n "${_HHHF_CAPTURE_LOADED:-}" ] && return
_HHHF_CAPTURE_LOADED=1

zmodload zsh/datetime            # EPOCHSECONDS
autoload -Uz add-zsh-hook

: ${HHHF_DIR:=$HOME/.hhhf-trial}
mkdir -p -- "$HHHF_DIR"
_hhhf_cmd_log="$HHHF_DIR/commands.jsonl"

# Minimal JSON string encoder. Escapes the JSON-critical bytes, then drops any surviving C0
# control char (keeps every line valid JSON even on a weird paste — faithfulness of exotic
# controls is not needed for command capture).
_hhhf_jstr() {
   local s=$1
   s=${s//\\/\\\\}
   s=${s//\"/\\\"}
   s=${s//$'\n'/\\n}
   s=${s//$'\t'/\\t}
   s=${s//$'\r'/\\r}
   s=${s//[[:cntrl:]]/ }
   print -r -- "\"$s\""
}

# $? is the previous command's rc, but only until another hook runs a command (his pure/precmd
# clobber it). Grabbing it as the FIRST precmd entry is the one reliable capture point.
typeset -g _hhhf_prev_rc=0
_hhhf_grab_rc() { _hhhf_prev_rc=$?; }
precmd_functions=(_hhhf_grab_rc ${precmd_functions:#_hhhf_grab_rc})

# preexec args: $1 = line as typed (history-expanded), $3 = fully expanded command.
_hhhf_preexec() {
   local cmd=$1 expanded=${3:-$1}
   print -r -- "{\"cmd\":$(_hhhf_jstr "$cmd"),\"expanded\":$(_hhhf_jstr "$expanded"),\"cwd\":$(_hhhf_jstr "$PWD"),\"prev-rc\":$_hhhf_prev_rc,\"epoch\":$EPOCHSECONDS}" >> "$_hhhf_cmd_log"
}
add-zsh-hook preexec _hhhf_preexec

print -r -- "[hhhf] command spine -> $_hhhf_cmd_log" >&2
