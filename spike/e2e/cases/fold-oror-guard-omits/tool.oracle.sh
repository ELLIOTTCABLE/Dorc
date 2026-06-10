# tool-existence oracle (the R2-SHADOW blessed form): `command -v <tool>` reports
# whether <tool> resolves, establishing tool:<tool>#present. A real one must confirm
# an executable FILE, not a function/alias (17O R2-SHADOW); this scrappy fixture is
# the minimal gate.
oracle_kind=tool
oracle_probe_tool() { command -v "$1" >/dev/null 2>&1; }
oracle_effect command '' establish present
# command-keyed check(): `command -v <tool>` binds NO verb (verbless provider, the
# effect-map keys on the ε-verb); strip the `-v`, annotate the operand as `tool`.
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
