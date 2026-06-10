# tool-existence oracle (the R2-SHADOW blessed form): `command -v <tool>` READS
# whether <tool> resolves — a read-only QUERY of tool:<tool>#present, mutating
# nothing (task-D2: `query` polarity, NOT `establish`). A real one must confirm an
# executable FILE, not a function/alias (17O R2-SHADOW); this scrappy fixture is the
# minimal gate.
oracle_kind=tool
oracle_probe_tool() { command -v "$1" >/dev/null 2>&1; }
oracle_effect command '' query present
# command-keyed check(): `command -v <tool>` binds NO verb (verbless provider, the
# effect-map keys on the ε-verb); strip the `-v`, annotate the operand as `tool`.
command__check() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1
}
