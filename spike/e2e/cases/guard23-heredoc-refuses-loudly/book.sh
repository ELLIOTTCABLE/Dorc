# guard23-heredoc-refuses-loudly (the ratified refuse-home posture, human ruling 2026-07-02:
# structurally-awkward homes — backgrounded commands, substitution positions, heredoc-
# carrying lines — default to RUN and refuse the guard LOUDLY, with the reason; XFAIL until
# the guard tier lands). A vouched, converged-past-wall site whose leaf carries a heredoc:
# the witness is complete (vouch reached, probe holds), but the leaf's span covers `<<EOF`
# and not the body lines, so a span edit strands the payload (the render21-heredoc-refusal
# precedent, arch-1 d-6). Desired: NO guard is inserted — the line runs VERBATIM, heredoc
# intact (kFAIL-perform: over-running a converged mutator is safe; a corrupted artifact is
# not) — and the refusal is DISCLOSED on the why-lens (gate-7 pattern: `refus`), never
# silent. The run-set is unchanged from HEAD; the loudness is the xfail. (If the built
# refusal lands error-severity, its line will match the pre-declared `guard` pattern in
# expected-diagnostics; that file is inert at HEAD.)
hork wombat
apt-get install -y nginx <<EOF
guard23 heredoc payload
EOF
