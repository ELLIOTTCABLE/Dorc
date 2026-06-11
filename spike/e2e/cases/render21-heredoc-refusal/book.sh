# render21-heredoc-refusal (arch-1, note 214 d-6 — the render-capability REFUSAL): an oracled,
# CONVERGED mutator that the disposition layer LICENSES to elide (it is an EstablishAmbient
# whose effect already holds) is REFUSED by the leaf-exact render because it carries a HEREDOC
# (`<<EOF`). The leaf's AST span covers the `<<EOF` operator, NOT the body lines (the body is
# generated content the parser captures separately), so substituting the command span would
# strand `some config`/`EOF` as stray artifact lines. The render refuses the license and runs
# the command VERBATIM (kFAIL-perform — over-executing an already-converged mutator is safe; a
# broken artifact is not), and emits an `error[render-heredoc-refused]` diagnostic so the
# converged mutator silently running is NOT invisible. HOST: nginx installed (converged, so it
# WOULD have elided but for the heredoc). expected.ran: the install runs (refused).
apt-get install -y nginx <<EOF
some config
EOF
