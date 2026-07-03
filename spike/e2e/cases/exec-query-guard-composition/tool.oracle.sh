# tool-existence oracle: `command -v <tool>` READS whether <tool> resolves — a
# read-only QUERY of tool:<tool>#present, mutating nothing (task-D2 `query` polarity).
# command-keyed check(): `command -v <tool>` binds NO verb (verbless ⇒ ε-verb); strip
# the `-v`, annotate the operand as `tool`.
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1 :? tool:"$tool".present
}
