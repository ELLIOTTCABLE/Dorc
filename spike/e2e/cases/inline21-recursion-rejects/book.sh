# inline21-recursion-rejects (arch-2, `i-1` — the recursion exclusion): a self-calling helper.
# The OUTER `helper` call inlines, but the inner `helper` inside the body is direct RECURSION
# ⇒ ⊤-rejected (a loud `cfg-inline-refused` diagnostic naming the cycle); it stays an ordinary
# unmodeled command (Opaque). That Opaque is a body site that the all-or-nothing CALL license
# refuses ⇒ the outer `helper` call RUNS (verbatim). The Opaque also POISONS downstream: the
# `apt-get install -y curl` below reads Written (a ⊤ reached it through the spliced body) ⇒ it
# runs too (the poison stays — nothing downstream wrongly elides). Analysis-only (no mocks): a
# self-calling function infinite-loops at runtime, so it is never executed — the pins are the
# verbatim apply render + the refusal diagnostic + the curl-poison.
helper() { apt-get install -y nginx; helper; }
helper
apt-get install -y curl
