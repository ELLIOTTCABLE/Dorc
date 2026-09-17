# 311k — The identity-model crosscheck: dispatch bundle

Conductor-emitted prompt kit (Fable, 2026-09-16) for four clean-context reviews of
`notes/311j` and the sysctl-namespaces exercise: two Fable (neutral, adversarial) and two
Codex/GPT-6-Astra (neutral, adversarial). Reports land as `notes/311l`–`311o`; the conductor
adjudicates them in a rewound context. Review point: commit `9672ebf8`.

=== DISPATCH: fable-neutral | mode=review ===
You are reviewing an identity model for Dorc, a tool that runs ops shell scripts ("books") and
removes lines a probe has proved unnecessary. The model decides whether two names written by
authors who never met reach one piece of world-state (SAME, which lets one measurement stand
for another) or provably different pieces (DISJOINT, which lets a removal survive a write that
really ran). Both under-execute when wrong; UNKNOWN is safe for both.

Read, in this order, before anything else: `Research/notes/311j-identity-and-relation-model-two-species.md`
whole (the model; its vocabulary is defined there); `Research/GOTCHAS.md` (the ops facts that
kill simple designs); `Research/notes/312b-exercises/net-sysctls-are-per-namespace.md` (the
experiment that drove the latest edits; its sh is strawman and its spellings carry no weight);
`Research/notes/311q-attack-and-firming-ledger.md` (what was decided and why). Then, with no
tools and no delegation, write down your own report structure and initial judgement before
you look at anything else. After that: `Research/notes/311-identity-and-relation-preparatory-model.md`
is the predecessor, and the diff between it and 311j is in scope; `Research/notes/312a-corpus-import-synthesis.md`
§0 and `Research/notes/312b-epistemics-sittings-ledger.md` §7–§9 hold the framing; root
`README.md`, `DESIGN.md`, and `USER_STORY.md` stages 4–7 say what Dorc is for. The rest of
`Research/` is open to you but is mostly older design that 311j deliberately re-litigates;
reasoned attacks on the text in front of you are worth far more than archaeology. You may
send read-only Sonnet scouts for grep-class lookups; do not let them reason or synthesize for
you.

Assess, in this order of value: (1) any ordinary ops situation the model cannot represent at
all; (2) anything 311 could say that 311j lost, or an inconsistency the 311j evolution
introduced; (3) places where the model can carry a truth only if the wrong person says it, or
says more than they can know. Anything else in 311j's text, or clearly missing from it and
meant to be in scope for identity, is fair game at lower value. Ground each finding in a
concrete book (plain sh, the kind people actually paste) walked through the model's own text,
citing its sections. Say plainly where a suspected fault does not hold. If existing machinery
can carry a case by rearrangement, say that before proposing anything new.

Out of scope: the named horizons (cloned boots sharing one id; a key recycled by something
outside the book within one unwalled span), though "this is horizoned and the world does it
constantly" is a fair finding; omissions relative to the whole project rather than to this
model; syntax and spelling; performance. Do not read `Research/quarantine-DO-NOT-READ/`,
anything under `.claude/research/` whose name says quarantine, `AGENTS.for-builders-only.md`,
or the sibling reports `Research/notes/311l`–`311o`.

Write your report to `C:\Users\ec\Sync\Code\Dorc\Research\notes\311l-identity-crosscheck-fable-neutral.md`,
findings first, ranked by the value order above, each with the book and the sections. Run no
git commands. No web research is needed; if your file-read tools are unavailable, abort and
say so rather than reasoning from memory. Return only the report's path.
=== END DISPATCH: fable-neutral ===

=== DISPATCH: fable-adversarial | mode=review ===
I have run production fleets for twenty years and I have watched every clever identity scheme
die on contact with a real machine. A colleague, whom I do not trust to have ever held a
pager, spent the last two weeks rewriting our identity model into
`Research/notes/311j-identity-and-relation-model-two-species.md`, and everyone nodded because
the words were long. I think it is wrong somewhere that matters. I think there are ordinary
things, the kind that happen every Tuesday on a box nobody remembers building, that this model
cannot even write down, let alone get right. I think the rewrite from its predecessor
(`Research/notes/311-identity-and-relation-preparatory-model.md`; the diff is yours to take)
dropped something while everyone admired the new vocabulary. And I think it quietly assumes
that the person who has to say the crucial sentence is the person who happens to know it,
which in my experience is never who is on call. Find where it breaks.

Dorc, for the record: it runs ops shell scripts ("books") and removes lines a probe proved
unnecessary; the model decides whether two names written by strangers reach one piece of
world-state (SAME: one measurement stands for another) or provably different pieces
(DISJOINT: a removal survives a write that really ran). Both under-execute when wrong;
UNKNOWN is safe for both. So the ways it can hurt me are exactly two: a wrong SAME and a wrong
DISJOINT. Bring me those.

Read the model whole first, then `Research/GOTCHAS.md`, then
`Research/notes/312b-exercises/net-sysctls-are-per-namespace.md` (the experiment they are so
proud of; its sh is strawman, do not argue spelling) and
`Research/notes/311q-attack-and-firming-ledger.md` (what they decided). Then put the tools
down and write your own plan of attack and first judgement before reading anything else;
I am paying for your reasoning, not a summary of theirs. After that the corpus is yours
(`Research/notes/312a` §0, `312b` §7–§9, root `README.md`, `DESIGN.md`, `USER_STORY.md`
stages 4–7 for what the tool is for; the rest is mostly older design this model re-litigates,
and I care about attacks, not archaeology). Sonnet scouts are allowed for grep-class errands;
they are not allowed to think for you.

What I want, in this order: (1) a real ops situation the model cannot represent at all; (2)
something 311 could say that 311j cannot, or a contradiction the rewrite introduced; (3) a
truth the model can carry only if the wrong person says it, or says more than they can know.
Anything else is fair game if it is in the text, or plainly missing from it and meant to be in
scope for identity, but I care less. Every finding comes with a book, real sh that someone
would paste, walked through the model's own sections until it gives the wrong answer or has no
answer. I have no patience for "this could be a problem in theory": show me the line. And I
have even less patience for invented faults: if you go looking for a break and the thing
holds, say so plainly, once, and move on; if the existing machinery carries a case once you
rearrange it, that is not a finding, that is you learning the model.

Out of bounds: the two horizons they declared (cloned boots sharing one id; a key recycled by
something outside the book inside one unwalled span), though if you can show the world does
one of those every day, say it; omissions relative to the whole project rather than this
model; spelling; performance. Do not read `Research/quarantine-DO-NOT-READ/`, anything under
`.claude/research/` with quarantine in its name, `AGENTS.for-builders-only.md`, or the sibling
reports `Research/notes/311l`–`311o`.

Write it to `C:\Users\ec\Sync\Code\Dorc\Research\notes\311m-identity-crosscheck-fable-adversarial.md`,
findings first, worst first, each with its book and its sections. Run no git commands. No web
research is needed; if you cannot read files, abort and say so rather than guess. Return only
the report's path.
=== END DISPATCH: fable-adversarial ===

=== DISPATCH: astra-neutral | mode=review ===
Review an identity model for Dorc, a tool that runs ops shell scripts ("books") and removes
lines a probe has proved unnecessary. The model decides whether two names written by authors
who never met reach one piece of world-state (SAME, letting one measurement stand for
another) or provably different pieces (DISJOINT, letting a removal survive a write that really
ran). Both under-execute when wrong; UNKNOWN is safe for both.

Read first: `Research/notes/311j-identity-and-relation-model-two-species.md` whole (the model
defines its own vocabulary); `Research/GOTCHAS.md`; `Research/notes/312b-exercises/net-sysctls-are-per-namespace.md`
(the experiment behind the latest edits; its sh is strawman, its spellings carry no weight);
`Research/notes/311q-attack-and-firming-ledger.md`. The predecessor
`Research/notes/311-identity-and-relation-preparatory-model.md` is in scope through its diff
against 311j. `Research/notes/312a-corpus-import-synthesis.md` §0, `Research/notes/312b-epistemics-sittings-ledger.md`
§7–§9, and root `README.md`, `DESIGN.md`, `USER_STORY.md` stages 4–7 give the framing. The
rest of `Research/` is open but mostly older design that 311j re-litigates; keep extraneous
reading small and spend the effort on reasoning about the text. Work alone; spawn nothing.

Assess, in this order of value: (1) any ordinary ops situation the model cannot represent at
all; (2) anything 311 could say that 311j lost, or an inconsistency the 311j evolution
introduced; (3) places where a truth can be carried only if the wrong person says it, or says
more than they can know. Anything else in 311j's text, or clearly missing from it and meant to
be in scope for identity, is fair game at lower value. Ground each finding in a concrete book
(plain sh) walked through the model's own sections. Say plainly where a suspected fault does
not hold. If existing machinery carries a case by rearrangement, say that rather than propose
new machinery.

Out of scope: the named horizons (cloned boots sharing one id; a key recycled by something
outside the book within one unwalled span), though "horizoned, yet the world does it
constantly" is a fair finding; omissions relative to the whole project; syntax and spelling;
performance. Do not read `Research/quarantine-DO-NOT-READ/`, anything under `.claude/research/`
with quarantine in its name, `AGENTS.for-builders-only.md`, or `Research/notes/311l`–`311o`.

Produce the report as your final message: findings first, ranked by the value order, each
with its book and sections. No web research is needed; if you cannot read files, abort and say
so rather than reason from memory.
=== END DISPATCH: astra-neutral ===

=== DISPATCH: astra-adversarial | mode=review ===
I have run production fleets for twenty years and I have watched every clever identity scheme
die on contact with a real machine. A colleague, whom I do not trust to have ever held a
pager, spent two weeks rewriting our identity model into
`Research/notes/311j-identity-and-relation-model-two-species.md`, and everyone nodded because
the words were long. I think it is wrong somewhere that matters. I think there are ordinary
things, the kind that happen every Tuesday on a box nobody remembers building, that this model
cannot even write down, let alone get right. I think the rewrite from its predecessor
(`Research/notes/311-identity-and-relation-preparatory-model.md`; diff them) dropped something
while everyone admired the new vocabulary. And I think it quietly assumes the person who has
to say the crucial sentence is the person who happens to know it, which is never who is on
call. Find where it breaks.

Dorc, for the record: it runs ops shell scripts ("books") and removes lines a probe proved
unnecessary; the model decides whether two names written by strangers reach one piece of
world-state (SAME: one measurement stands for another) or provably different pieces
(DISJOINT: a removal survives a write that really ran). Both under-execute when wrong;
UNKNOWN is safe for both. The ways it can hurt me are exactly two: a wrong SAME and a wrong
DISJOINT. Bring me those.

Read the model whole, then `Research/GOTCHAS.md`, then
`Research/notes/312b-exercises/net-sysctls-are-per-namespace.md` (the experiment they are so
proud of; its sh is strawman, do not argue spelling), then
`Research/notes/311q-attack-and-firming-ledger.md` (what they decided). Framing, if you need
it: `Research/notes/312a-corpus-import-synthesis.md` §0, `Research/notes/312b-epistemics-sittings-ledger.md`
§7–§9, root `README.md`, `DESIGN.md`, `USER_STORY.md` stages 4–7. The rest of `Research/` is
open but is older design this model re-litigates; I want attacks, not archaeology, so keep
extraneous reading small. Work alone; spawn nothing.

What I want, in order: (1) a real ops situation the model cannot represent at all; (2)
something 311 could say that 311j cannot, or a contradiction the rewrite introduced; (3) a
truth the model can carry only if the wrong person says it, or says more than they can know.
Anything else is fair game if it is in the text, or plainly missing and meant to be in scope
for identity, but I care less. Every finding comes with a book, real sh someone would paste,
walked through the model's own sections until it gives the wrong answer or has no answer. No
"could be a problem in theory": show me the line. And no invented faults: if you go looking
for a break and the thing holds, say so plainly, once, and move on; if the existing machinery
carries a case once rearranged, that is not a finding.

Out of bounds: the two declared horizons (cloned boots sharing one id; a key recycled by
something outside the book inside one unwalled span), though if the world does one of those
every day, say it; omissions relative to the whole project; spelling; performance. Do not read
`Research/quarantine-DO-NOT-READ/`, anything under `.claude/research/` with quarantine in its
name, `AGENTS.for-builders-only.md`, or `Research/notes/311l`–`311o`.

Produce the report as your final message: findings first, worst first, each with its book and
its sections. No web research is needed; if you cannot read files, abort and say so rather
than guess.
=== END DISPATCH: astra-adversarial ===
