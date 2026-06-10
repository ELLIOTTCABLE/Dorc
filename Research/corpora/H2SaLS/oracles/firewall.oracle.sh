# LLM-GENERATED ORACLE SEED — intentionally quality-varied artificial testing
# corpus for a static-analysis project (Dorc). NOT real ops code; FROZEN EVIDENCE,
# NEVER EXECUTE. An artificial oracle cannot expose the truth of real-world ops-code.
# Validation is `dash -n` (parse-only) plus reading.
#
# Models the ufw verbs THIS book exercises:
#   - `ufw limit in <port>`        (§3 L183 `in 22`, L184 `in $SSH_PORT`; §6 L417)
#   - `ufw allow out <port>`       (§6 L421, looped over 43/53/123/80/443/$MAIL_PORT)
#   - `ufw default deny <dir>`     (§6 L413 incoming, L414 outgoing)
#   - `ufw logging on`             (§6 L409)
#   - `ufw --force enable`         (§6 L410)
#
# THE ENTITY PROBLEM (um-ufw-1, the headline finding). A `limit`/`allow` rule's
# identity is a TUPLE (action × direction × port × proto × from/to), NOT a port:
# `allow out 80` and `limit in 80` are DIFFERENT rules on the same port; direction and
# action are load-bearing. But the engine's inline annotation can denote exactly ONE
# argv WORD (`port : firewall = "$1"`) — it has no `"$*"`/join word (that lifts to an
# unmodeled expansion ⇒ ⊤). So a multi-word rule (`in 22`) CANNOT be annotated as a
# single entity, and annotating just the port (`22`) would conflate `in 22` with
# `out 22` and ship a probe that elides the wrong rule. This book's rules are ALL
# multi-word, so this oracle REFUSES to probe them (the `[ "$2" = "" ]` single-word
# guard, mirroring R2-MULTIOP): every `limit`/`allow` here resolves no probe ⇒ runs.
# Re-adding an identical ufw rule is idempotent, so running is safe; the COST is we
# never elide a converged firewall rule. The effect cells are still declared (the
# state model / poison-wall separation is real) — only the probe refuses.
#   - `default`, `logging`, `enable` are NOT per-rule entities — they are GLOBAL
#     singletons (policy / log-level / enabled-bit). `default deny incoming` and
#     `default deny outgoing` are TWO distinct global cells. They are NOT modelled
#     here (no verb arm, no effect) ⇒ they run.
#
# THE PROBE PROBLEM (um-ufw-2), why even the single-word path is weak. The natural
# probe `ufw status | grep <rule>` is a TWO-outcome test where THREE are needed:
#   - `ufw status` needs ROOT; unprivileged it errors. Piped to `grep`, a tool FAILURE
#     (rc!=0, no rows) is INDISTINGUISHABLE from "rule absent" (rc!=0, no match). Both
#     read "absent". For `establish` that means a wasteful idempotent re-add; it would
#     be a DISASTER for a `kill`/`delete` verb (can't-tell read as absent ⇒ wrongly
#     elide a deletion) — so this oracle declares NO delete/reset verb (book has none).
#   - `ufw status` prints "Status: inactive" with ZERO rules when ufw is OFF — and this
#     book ADDS rules (§3 L183-184) BEFORE enabling ufw (§6 L410). So a rule probe in
#     the pre-enable window reads absent purely because the firewall is down. A
#     grep-based probe is actively misleading here. Not fixable in sh (`inv-kfail`).
oracle_kind=firewall
oracle_probe_firewall() { ufw status 2>/dev/null | grep -qF "$1"; }
oracle_effect ufw limit establish allowed
oracle_effect ufw allow establish allowed

# command-keyed check(): `ufw <verb> <rest…>`. We bind the verb then annotate the
# operand `firewall`. The `[ "$2" = "" ]` guard refuses anything but a SINGLE-word rule
# operand (um-ufw-1) — so this book's `in 22` / `out 80` two-word rules resolve NO
# probe ⇒ run. `default`/`logging`/`enable` match no arm ⇒ ⊤ ⇒ run (um-ufw-1, global
# singletons). `--force` (in `ufw --force enable`) is a pre-verb flag, stripped.
ufw__check() {
   while [ "${1#-}" != "$1" ]; do shift; done
   verb=$1; shift
   case $verb in
      limit|allow)
         rule : firewall = "$1"
         if [ "$2" = "" ]; then ufw status 2>/dev/null | grep -qF "$rule"; fi
         ;;
   esac
}
