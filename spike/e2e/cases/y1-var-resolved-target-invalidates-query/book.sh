# y1-var-resolved-target-invalidates-query (21H §9 residual-2; the e2e companion to the unit
# pin var_resolved_redirect_target_invalidates_query + var_resolved_redirect_target_gens_concrete_cell_not_top):
# the y-1 redirect-write machinery fires on a VALUE-RESOLVED target, not just a literal one. A
# redirect target is an ordinary expansion, so `conf=nginx.conf; printf 'x' >> "$conf"` resolves
# `$conf` through the value plane to `nginx.conf` ⇒ gens `file:nginx.conf#written` (a WRITER) ⇒
# the downstream `grep -q x nginx.conf` confline Query is non-pristine ⇒ `valid: false` ⇒ the
# guard's rc is withheld ⇒ the `||` cannot resolve ⇒ the install stays LIVE (renders verbatim).
# This is the literal-target case `y1-redirect-write-invalidates-query` with the target arriving
# through a RESOLVED variable — the ⊤-vs-concrete distinction the unit pin `ecf326d` closed,
# now as an e2e case. The cell is CONCRETE (resolved to `nginx.conf`), NOT ⊤: so NO
# `dq-redir-target-top` fires (the ⊤ path is `y1-top-target-poisons`). Mocks-free (dash -n +
# golden), like the sibling y1-* cases: a `file` cell has no probe/leaf, so there is nothing to
# exec — the invalidation is a static poison, asserted by the verbatim (un-folded) render.
set -e
conf=nginx.conf
printf 'x' >> "$conf"
grep -q x nginx.conf || apt-get install -y nginx
