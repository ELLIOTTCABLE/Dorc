# glob-for-word-runs (`20O` find-1 / fix-1): an unquoted glob `for`-list word
# (`*.conf`) undergoes pathname expansion against the live filesystem (XCU §2.6.6) —
# its membership AND count are runtime-determined — so the for-var `f` binds ⊤, not the
# literal `*.conf`. The ⊤ flows to BOTH uses:
#   * the in-loop `echo "$f"` operand is ⊤ (and the in-loop render floor would bar elision
#     regardless); and
#   * the POST-loop `apt-get install -y "$f"` operand is ⊤ ⇒ the entity does not resolve ⇒
#     the install RUNS verbatim (the SAFE degrade, kFAIL-perform).
# The CONVERGED BAIT (probe-results.txt): a host where the glob-derived package is reported
# installed must NOT license eliding the install — pre-fix-1, `f` wrongly bound the literal
# `*.conf`, the check resolved a phantom `package:*.conf`, and a converged fact for it would
# have elided a runtime-determined install (the wrong-concrete → wrong-elision disaster
# class, 19H §1.3). fix-1 makes `f` ⊤, so no convergence claim can reach this install.
for f in *.conf; do echo "$f"; done
apt-get install -y "$f"
