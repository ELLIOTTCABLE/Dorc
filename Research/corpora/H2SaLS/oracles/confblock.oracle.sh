# ============================================================================
# LLM-GENERATED ORACLE SEED — NOT REAL OPS CODE. Part of an intentionally
# quality-varied artificial testing corpus for the Dorc static-analysis
# project; it cannot expose the truth of real-world ops-code and must never be
# run. FROZEN EVIDENCE: the probe bodies below name real read-only commands
# (cmp, test) but are NEVER executed, under any flag or fragment. Validation is
# `dash -n` plus reading. See Research/corpora/H2SaLS/README.md.
# ============================================================================
#
# confblock — the WHOLE-FILE / managed-block end of the file-state spectrum:
# the entity is a file whose ENTIRE content is owned by the book and converges
# to a fixed byte-image. Models the book's `cat > file <<EOF` overwrites
# (51myunattended-upgrades L340-394, jail.local L475-491, ssh.local L494-502,
# msmtprc L516-531, cisofy-lynis.list L656-658) and the cmp-gated mozilla block
# (L239-259). Identity is the PATH alone (one operand → clean annotation —
# this end of the spectrum has NO identity problem, unlike confline).
#
# THE CONVERGE-BY-CONSTRUCTION PROBLEM (the headline finding — 1A9 §confblock).
# `cat > file <<EOF` overwrites unconditionally: after it runs, the file IS the
# heredoc bytes, definitionally. So "did it converge?" is trivially yes
# post-write — which means the USEFUL probe is the PRE-write one: "does the file
# ALREADY equal the intended bytes?" (If yes, the overwrite is a no-op and the
# apply could elide it.) That requires the oracle to KNOW the intended bytes.
#
# AND IT USUALLY CANNOT (the wall — 1A9 §confblock). The intended bytes are the
# heredoc body, but most of this book's heredocs are UNQUOTED (`<<EOF`, census:
# 8 unquoted vs 2 quoted) and interpolate runtime `$VAR`s the oracle cannot
# resolve statically:
#   - jail.local:  `destemail = $MAIL_TO`, `port = $SSH_PORT`     (L481/L498)
#   - msmtprc:     `port $MAIL_PORT`, `host $MAIL_SMTP_SERVER`, `password $MAIL_PW`
#   - cisofy.list: fixed bytes (no $VAR) — the ONE statically-knowable case.
# So a content-equality probe is honest ONLY for the QUOTED-heredoc / no-$VAR
# blocks (the unattended file L340 `<<'EOF'`, cisofy.list). For unquoted
# blocks, the intended image is not knowable until the controller expands it at
# run time — the oracle can't carry it, and `cmp` against unknown bytes is a
# lie. This kind models content-equality where the bytes are static, and
# REFUSES (probe present-only, or nothing) where they are runtime-expanded.

oracle_kind=confblock

# Kind-default probe: existence. `[ -e <path> ]` — the file is present at all.
# A read-only QUERY, the weakest honest claim, valid for EVERY confblock target
# regardless of $VAR-expansion. (Present != converged; the overwrite still runs
# to fix content. But present-and-nonempty is a real, probeable floor.)
oracle_probe_confblock() { [ -s "$1" ]; }

# Content-equality selector — the STRONG claim, honest ONLY when the intended
# bytes are statically known (quoted heredoc / no interpolation). `cmp -s
# <path> <reference>` is a read-only QUERY of byte-identity. `$1` is the path;
# `$2` is a reference image the resolver would supply ONLY for a static-bytes
# block. For runtime-$VAR blocks the resolver supplies no reference and this
# selector is not used (the site runs). THREE-OUTCOME HAZARD: `cmp` rc is
# 0=identical / 1=differ / 2=trouble (missing file) — same conflation as grep
# (1A9 um-file-2); a converged-vs-couldn't-read gap the two-outcome wrapper
# hides.
oracle_probe_confblock_content() { cmp -s "$1" "$2" >/dev/null 2>&1; }

# WHICH PROVIDERS GET EFFECT LINES (the confblock breakdown, 1A9 §confblock):
# the writer is `cat > file <<EOF`. `cat` reads the heredoc to STDOUT; the file
# write is the `>` REDIRECT — again, NO command token to key an establish on
# (identical to confline's printf/>> finding: the mutating verb is shell
# syntax). `cat` itself is a generic stream tool, not a file-establish verb.
# So confblock, like confline and sshdconf, declares ONLY a read-side Query
# (existence/content via test/cmp). The overwrite has no provider verb to bind.
oracle_effect cmp '' query content

# command-keyed check(): `cmp [-s] <fileA> <fileB>`. The entity-resolver. cmp
# compares TWO paths; the book's convergence-gate form is `cmp -s "$current" -`
# (heredoc on the OTHER side, L250) or `cmp file file.tmp` (L303). The entity
# we key on is the MANAGED file (the first operand, the one the book owns).
# Annotate it; the second operand is the reference detail. REFUSE the `-`
# (stdin) and process-substitution forms — no stable path to key (bind nothing
# ⇒ run).
cmp__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   managed : confblock = "$1"; shift
   reference=$1
   # refuse when the managed operand is stdin/empty, or no reference operand:
   # the byte-image identity is not pinned to a file, so bind nothing and run.
   if [ "$managed" != "" ] && [ "$managed" != "-" ] && [ "$reference" != "" ]; then
      cmp -s "$managed" "$reference" >/dev/null 2>&1
   fi
}
