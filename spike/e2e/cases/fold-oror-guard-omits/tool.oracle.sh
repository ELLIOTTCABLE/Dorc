# tool-existence oracle (the R2-SHADOW blessed form): `command -v <tool>` reports
# whether <tool> resolves, establishing tool:<tool>#present. A real one must confirm
# an executable FILE, not a function/alias (17O R2-SHADOW); this scrappy fixture is
# the minimal gate.
oracle_kind=tool
oracle_probe_tool() { command -v "$1" >/dev/null 2>&1; }
oracle_effect command -v establish present
