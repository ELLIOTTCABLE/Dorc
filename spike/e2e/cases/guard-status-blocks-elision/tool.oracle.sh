# tool-existence oracle (the R2-SHADOW blessed form): `command -v <tool>` reports
# whether <tool> resolves. Modeled as establishing tool:<tool>#present so an admin's
# `command -v nginx` idempotency guard stops poisoning downstream (notes/193 strain-5).
# A real one must confirm an executable FILE, not a function/alias (17O R2-SHADOW);
# this scrappy fixture is the minimal gate, and the apply is never executed against
# this body (only the book's rendered apply runs, and only command -v is a builtin).
# command-keyed predict(): `command -v <tool>` binds NO verb (verbless provider, the
# effect-map keys on the ε-verb); strip the `-v`, annotate the operand as `tool`.
# NB this R2-SHADOW fixture models `command -v` as ESTABLISH (`:` not `:?`) — so the
# idempotency guard blocks the install's elision. The other tool oracles are read-only
# OBSERVE (`:?`, query).
command__predict() {
   case $1 in -v) shift ;; esac
   tool : tool = "$1"
   command -v -- "$tool" >/dev/null 2>&1 : tool:"$tool".present
}
