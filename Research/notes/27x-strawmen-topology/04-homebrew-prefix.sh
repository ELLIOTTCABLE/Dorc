# dorc-lang/v0.1

# ============================================================================
# 04 — Homebrew: the $(brew --prefix) dynamic root      (STRAWMAN, never run)
#
# THE REAL-WORLD FACTS. Formula state lives under $(brew --prefix)/Cellar —
# and the prefix is /opt/homebrew (ARM), /usr/local (Intel), or custom;
# knowable only by ASKING THE HOST. brew famously refuses to run as root;
# on multi-user Macs one admin user owns the prefix and other users can
# usually read it. This is the USER_STORY dotfiles-position's flagship
# package manager.
#
# WHY NON-TRIVIAL FOR D: the address root is not a constant or a ρ-variable —
# it is a CAPTURED COMMAND OUTPUT. This is D's dynamic-arm story meeting its
# first mandatory case.
# ============================================================================

# The book line under analysis (from the machine.sh glue-book pattern):
brew list --versions ripgrep >/dev/null 2>&1 || brew install ripgrep

# ---------------------------------------------------------------------------
# SHAPE A:   : sm.dorc.BrewFormula user=invariant
# — done. Cheap, honest (the Cellar is one referent), nothing else needed.
# Shape A's one clean win in this exercise.
#
# SHAPE D:
sm_dorc_BrewFormula__lives_at() {
   printf '%s/Cellar/%s\n' "$(brew --prefix)" "$1"
}
# ---------------------------------------------------------------------------

# DERIVATION WALKTHROUGH, and where it stalls. The traced address is
# CONCAT(capture("brew --prefix"), "/Cellar/", $1). The `$1` part is entity —
# fine. The capture is the problem: dependence analysis needs to know whether
# `brew --prefix`'s OUTPUT varies with the invoking user, and that is a fact
# about a black-box binary — statically unknowable, the same
# binary-interior wall as everywhere else (opaques7-finding19's shape).
#
# Without more, the honest verdict is: capture ⇒ ⊤-dependence ⇒ the axis is
# unclassifiable ⇒ silence-floor. AUTHORED BUT VALUELESS — D's member exists
# and buys nothing. (Safe. Annoying.)
#
# THE MISSING PIECE IS TASK 7'S VOCABULARY (finding f7). The read-blessing /
# capture-claim lane (adj-capture-claim; `notes/219` q-5; the
# two-observation-planes note) is already designing "what may an oracle claim
# about a command's output?" — value-bounds like "single line, no
# metacharacters". This example says that vocabulary needs ONE MORE
# DIMENSION: axis-independence — illustratively, the brew oracle blessing its
# own introspection verb:
#
#    brew --prefix    <some read-blessing carrying: output is user-invariant>
#
# With that claim, the capture's user-dependence is resolved-by-vouch, the
# concat has no user-axis input, and INVARIANT derives as usual. Without it,
# floor. Either way safe; the coupling is the point: D's dynamic arms are
# CONSUMERS of the capture-claim design, so adj-trichotomy-spelling and
# adj-capture-claim are not independent sittings — task 7 must know it has
# this second customer before its shape is settled.
#
# VERDICT. Shape A wins Homebrew today: one blind assertion vs a dynamic arm
# gated on unbuilt task-7 machinery. Noted honestly as the fallback role
# earning its keep — with the counterweight that A's win is exactly its
# usual vice (nothing checks it), and that D-plus-capture-claims eventually
# expresses strictly more (per-host prefixes, correct under custom installs)
# than A's flat token ever can.
