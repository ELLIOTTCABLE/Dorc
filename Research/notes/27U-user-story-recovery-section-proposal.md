# 27U — proposed USER_STORY section: recovery (how knowledge reaches the user)

AI-authored (Fable, 2026-07-18, same sitting as `27T`/`AID-NEEDS`). USER_STORY.md is
human-edit-only; this note carries PROPOSED text for the human to audit, edit, and place.
Suggested placement: immediately after "The bought unsoundness: one corner, fully fenced"
and before "The final ledger" — the section's worked example is that corner's bite
actually landing, so the continuity is direct. The render inherits the document's
standing ILLUSTRATIVE hedge. Theory grounding (for the reviewer, not for the section):
`26C` §5b (two lanes, opposite fail directions) · `AID-NEEDS` Law (trust-tier-as-syntax;
pull-runs-wide-open; one-attention-moment) · `271:rul-sin-ordering` ·
`22A:concl-10` (thin durable / replay) · the `27T` flagship case, whose render this is.

---- PROPOSED TEXT BEGINS ----

## Recovery: how you find out, and what you do about it

Everything above is about what Dorc *does*. This section is about how you *learn* — which
is not a side-topic, because an orchestrator like Dorc is a tool that sometimes decides to
*not* do things. A tool with that power owes you an account of its decisions; and it owes
you that account most of all on the morning something is wrong, which is precisely when
you are least patient with it.

Three principles govern every way Dorc talks to you.

First: knowledge arrives at the moments you were already looking. There is one plan and
one moment of consent; Dorc never adds interaction moments beyond it. What Dorc
*volunteers* — the plan's annotations, a hint that an oracle would recover three sites —
is rationed ruthlessly, because your attention is the product being conserved. What you
*ask for* is the opposite: unbounded. `dorc why` answers a question you chose to ask, so
it holds nothing back; and it works *after* everything is apparently done, with nothing
you had to set up beforehand. The receipt it reads is small and boring — what was
measured, what was decided, what actually ran — and the full explanation is recomputed
from it on demand, which is why there is no log-ocean to rotate and no history database
to maintain. Ask tomorrow; ask next week.

Second: every explanation is a chain, and every link in the chain wears a label saying
what kind of thing it is. *Measured* — a probe read the world, at a stated time.
*Vouched* — a named author accepted a judgment, at a stated file and line. *Claimed* — a
named author asserted something no machine verified. *Derived* — Dorc computed a
consequence. *Consented* — you typed a flag. These labels are not writing style; the
machinery cannot render a claim in measurement's clothing, and a person's name is
attached wherever a person is load-bearing. When an explanation is wrong, the labels are
what keep it honestly wrong: Dorc ranks its own possible failures, and telling you a
*wrong* cause is worse than admitting it cannot tell — so where certainty runs out, the
chain says so rather than rounding up.

Third: deciding and explaining fail in opposite directions, on purpose. When Dorc
*decides*, doubt makes it act less — an unsure line runs, an unsure probe never ships.
When Dorc *explains*, doubt makes it say more, with the doubt attached — the explanation
may draw on knowledge the deciding machinery is forbidden to touch, provided every such
statement carries its label. A narrower rule would make the safe tool useless at
exactly the moment it owes you the most.

Here is what that feels like, on the worst morning this document describes. Recall the
bought-unsoundness corner: `--risk-faultless-skips` typed, foobar's author's at-most
claim trusted, and — the one-in-a-thousand day — the claim was incomplete: `sync-certs`
also rewrites a systemd drop-in, which its author forgot. The certs drifted; foobar ran;
the `systemctl` line's elision survived on the strength of the claim; the service came
up wrong overnight. You know none of this yet. You know the site is down, and that
yesterday Dorc said everything was fine.

```
$ dorc why 9
book.sh:9  systemctl enable --now nginx
  removed from the plan (elided); did not run in the apply of 2026-07-17 06:12.

  it was removed because all of the following held together:
  1. measured:   nginx was enabled+active on web1 at plan time (06:11:52)
  2. vouched:    the service oracle's author accepts already-enabled+active as
                 reason enough not to re-run this (systemctl.oracle.sh:12)
  3. ran above:  book.sh:8 `foobar sync-certs /etc/nginx/certs` really ran --
                 ordinarily that would send line 9 back into the plan as a
                 re-check --
  4. claimed:    but foobar's oracle claims sync-certs disturbs at most its own
                 certs (foobar.oracle.sh:31 -- an author's claim; nothing
                 verifies it)
  5. derived:    that claim does not overlap what link 1 measured
  6. consented:  --risk-faultless-skips was set, which is what lets 4+5 keep a
                 line out of the plan past a running mutation.

  if line 9 SHOULD have run: dorc cannot see which link is wrong, but the links
  are not equally trustworthy -- 4 is the one unverified human claim in this
  chain. if `foobar sync-certs` also touches service state, that claim is what
  wrongly kept line 9 out.
  to check:  `dorc plan book.sh web1` re-measures the world as it is now.
  to fix:    foobar.oracle.sh:31 is the line to widen; every book using that
             oracle inherits the repair.
```

Notice what the answer does not do. It does not guess. The epilogue's finger points at
link 4 not because Dorc detected the wrongness — it cannot, that is what the frame
problem means — but because the *design* knows which link in this chain was unverified by
construction: it is the one place this document said a naked human promise ships. Stating
that is not an accusation; it is the receipt for the trust you spent when you typed the
flag.

And notice the shape of recovery, because it is always the same two moves. *Re-measure*:
the next plan reads reality, not memory — Dorc kept no state that can stay wrong, so the
broken fact comes back diverged and the line returns on its own. *Fix at the leverage
point*: one file, one line, named — and because that claim was the shared artifact, the
repair reaches every book on every host that trusts it, including people who will never
know the outage happened. Gradual enhancement runs on exactly this loop: the same channel
that nags you toward the first two-minute oracle is the one that, on the bad morning,
tells you which two minutes to spend next.

What recovery is *not*: there is no drift daemon, no fleet history, no stored suspicion.
The receipt is not a cache and never feeds a decision; it exists so that one question —
"why?" — always has an answer, at the moment you are angriest, with names on it. The plan
is the promise; the why is the receipt.

---- PROPOSED TEXT ENDS ----

Reviewer notes (not part of the section): (a) the render is the `27T` flagship case's
target output — if the human edits the prose here, the flagship's defining-case prose
should be authored to match the audited voice; (b) the section deliberately does not
mention surfaces that don't exist yet beyond the whylog it motivates (`dorc why` with no
argument, lint, TUI are all absent — scope discipline); (c) "one-in-a-thousand day" is
rhetorical, not a measured rate — strike if it reads as a quantified claim; (d) the
sin-ordering appears in user terms ("telling you a wrong cause is worse than admitting it
cannot tell") without corpus vocabulary, per the document's register.
