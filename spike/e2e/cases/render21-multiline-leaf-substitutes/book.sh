# render21-multiline-leaf-substitutes (arch-1, note 214 d-6 — the NEWLY-EXPRESSIBLE multi-line
# substitution pin): a leaf whose source span crosses MULTIPLE lines (an argv operand with a
# LITERAL NEWLINE) is substitutable under the leaf-exact render — a span edit may cover several
# lines, collapsing them to the single-line stand-in. The round-21 line-granular render REFUSED
# a multi-line leaf (it could not splice across a line boundary); the leaf-exact render retires
# that refusal. The converged install (operand spans two lines) collapses to `true`; the
# unoracled `systemctl reload` runs. The provenance comment flattens the operand's interior
# newline (else the `#` comment would split into a stray unterminated-quote line). HOST: the
# (oddly-named) package is installed. expected.ran: only systemctl (the multi-line install elided).
apt-get install -y "multi
line"
systemctl reload nginx
