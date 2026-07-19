# 279e — Red-team review of the round-27 design package (deepseek-v4-pro)

Reviewer: deepseek-v4-pro, 2026-07-13. Solo session, no subagents.

Package under review: `plans/270`, `plans/271`, `notes/272`–`278`. Authority rule:
root docs (human-authored) outrank `plans/`; typed human rulings in `271` outrank
conductor-proposed material in the notes; newest documents win disagreements.

## Summary

The package is unusually robust for AI-generated design work — every knife-edge is
priced, every escape hatch has a name, and the status tables honestly track what is
ratified vs proposed. No finding below would kill the package; several would reward
a human eyeball before the rebuild commits to representations.

---

## Findings (most severe first)

### 1. [MAJOR] The `--risk-faultless-skips` flag attempts to separate the inseparable

**Confidence: HIGH. Cites: `271:rul-flag-is-razor-residue` (line 621–665); `USER_STORY.md` line 468–475; `271:thread-razor-unsoftened-worked` (line 680–699).**

The ruling declares: "Claims own what lines can say; the flag owns what no line can
say" — positive disturbs entries ride vouch-tier, the "and nothing else" omission
rides the admin's flag. But the act of authoring a `cmd__disturbs()` body that lists
*some* entities and stops is inherently an act of claiming completeness. A human
reading `printf 'package:nginx\n'` followed by no further lines understands "and
nothing else" as what the author meant. The ruling's separation of the positive
entries (author's line, vouch-tier) from the completeness claim (flag-gated) would
hold only if there existed a spelling that enumerated items *without* implying
completeness — but the function's very existence as the disturbs-vocabulary member
for that tool IS the completeness claim.

The package's own language concedes the tension: USER_STORY calls the flag
"marketing at best … and theatre at worst" and `271:thread-razor-unsoftened-worked`
records that the human's two wants (razor-unsoftened + no-cargo-cult-flag) were
"human-graded 99%-sure irreconcilable." The resolution (invariance-speech-act for
transport, flag for survival) solves the transport side but leaves the survival
side with the original tension: a disturbs author enumerating "at most these cells"
is making a claim whose wrongness is *both* attributable to a line (they chose where
to stop) *and* structurally unsayable (the frame problem's "and nothing else").

**Why this matters.** The survival tier (USER_STORY stages 5–7) is gated entirely
by this flag. If the flag's boundary is incoherent — if the same authored act rides
two licensing tracks simultaneously — then the consent model for the design's most
dangerous corner is built on a distinction that does not hold. The double-ended
nature of the flag (author's clean claim + admin's consent) is an honest attempt
to square the circle, but the circle remains.

### 2. [MAJOR] `kind__state_stored_only_in()` carries two contracts, one invisible in the name

**Confidence: MEDIUM. Cites: `272` §10, lines 292–299; `271:rul-invariance-speech-act`, lines 601–619; `277` §4e.**

The `272` §10 collapse record explicitly rejected folding the store enumeration and
another member's emissions into one function: "one-member-with-arm-marks declined
(two contract texts in one body)." The rejection's rationale was that different
emission classes (locators vs coordinates) constitute different contracts and
deserve separate members.

However, `271:rul-invariance-speech-act` then added an `invariant:user` colon-line
to the store member's body — a *second* speech-act alongside the store enumeration.
The `only` in the member name signals the first contract (totalistic store survey).
The `invariant:user` line signals a transport-licensing contract that the name
carries no signal for.

The two contracts are related (transport depends on stores being correctly
enumerated) but are different *licenses*: an incomplete store enumeration AND a
claimed `invariant:user` line together produce wrong transport, and the knife is
the union of two failures, not one. Future readers of the member name will not
know to audit the transport claim.

**Mitigation.** The conductor could argue the invariance line is parasitic on the
store enumeration, not a separate contract. But the ruling explicitly makes the
invariance line the license *source* and re-roles the derivation to
contradiction-checker — the line is an authored speech-act, not a derivative label.
The naming convention lags the design.

### 3. [MINOR] The "knife-tier" consumer category in `271:rul-composed-bytes-defer-and-floor` is circular

**Confidence: HIGH. Cites: `271:rul-composed-bytes-defer-and-floor`, lines 511–528; `275` §4.**

The ruling sets a reversible interim floor: "probe-provenance" and "real bytes" are
both pinned to "delegation-produced, not merely probe-executed — for knife-tier
consumers." The only definition of "knife-tier" is the negative space of what the
floor currently excludes (fold-into-analysis in `275` §4; artifact-entering
substitution). No principle is stated for determining whether a *future* consumer
is knife-tier — e.g., a new warning-lane consumer that feeds hints but not licenses,
or a changed-detection fold that runs admin-authored pipelines against predicted
bytes. The "reversible" nature of the floor means lifting it later would admit new
knife consumers whose status under the floor was never decided.

### 4. [MINOR] The `only`-contract on `kind__state_stored_only_in()` is mechanically unenforceable

**Confidence: HIGH. Cites: `272` §2, lines 95–102; `272` §8, lines 240–245; `272` §9, lines 267–278.**

The `only` in the member name promises totalistic survey. The contradiction-checker
(`272` §3, amended by `271:rul-invariance-speech-act`) can detect the case where
listed stores contain who-am-I ingredients AND invariance is claimed — a logical
inconsistency between the stores the author DID list and the invariance claim.
It cannot detect the primary knife: an author who forgot to list a per-user store
entirely, then claimed invariance. The contradiction-checker sees no who-am-I
ingredients in the (incomplete) list and approves the invariance claim.

The package honestly prices this (`272` §8, §9) and leans on the differential CI
sweep as partial mitigation. The `only` in the name nonetheless overstates the
mechanical enforcement available.

### 5. [MINOR] Implicit-terminal-rc claim status is unspecified in the per-channel vocabulary

**Confidence: HIGH. Cites: `273` §2, lines 80–81; `271` merge-riders.**

The `cmd__predict()` per-channel claim vocabulary specifies: "explicit `return` =
rc claim; redirect-to-null = per-channel DECLINE." The common delegation case —
a predict body ending with `tool --dry-run "$@"` and no explicit `return` — has
an implicit terminal rc (the tool's rc). Whether this rc is claimed, declined, or
in a third state is unspecified. The ruling's merge-riders mention "per-channel
claim/decline vocabulary" but do not enumerate the implicit-terminal case. This is
the common case for delegation bodies and needs explicit treatment.

### 6. [MINOR] Value-prediction freeze-at-binding is unexamined under `set -e`

**Confidence: MEDIUM. Cites: `275` §5; `276` pipefail thread.**

`275` §5 establishes that values freeze at binding time with a patrolled window of
"apply-script-start → the apply-time binding line." But under `set -e`, a capture
assignment `v=$(cmd)` can kill the script before binding completes if `cmd` exits
non-zero and the assignment is not errexit-exempt (it is not — `var=$(cmd)` is
errexit-exempt in POSIX, but the *pipeline* variant `v=$(cmd | grep x)` or
assignments inside `&&` chains are not). The interaction between the assignment
site's errexit semantics and the engine's treatment of the partially-bound value
(was it frozen? partially frozen? not frozen?) is unexamined. The `set -e` carve
in the `276` pipefail thread discusses pipeline rc, not assignment errexit.

### 7. [MINOR] The posh∩dash weld enshrines a permanent pipefail tension

**Confidence: LOW. Cites: `276:rul-spec-two-binary-floor`, lines 192–239; `278` §1, lines 22–32.**

The weld pins the floor binary to dash 0.5.12 — deliberately chosen as the *last*
release without `set -o pipefail`. But the dialect includes pipefail, and the
strip-idiom (the self-gating `(set -o pipefail 2>/dev/null) && set -o pipefail`)
is the bridge. The consequence: every oracle that uses pipefail must carry the
gate idiom, and the floor test (run-stripped-under-both) cannot verify that
pipefail-using oracles behave correctly — the floor binary rejects pipefail. This
is a deliberate, priced choice, not an accident, but it adds ceremony to every
pipefail-using oracle permanently.

---

## Withdrawn attacks

### A. "The `kWHICHSH` scope carve creates marker ambiguity." WITHDRAWN.
The marker (`# dorc-lang/v0.1`) gates syntax; the scope carve governs which files
(books or oracles) get what *value* of analysis (full elision vs site-finding).
The marker semantics are consistent: marked files are parsed as full dialect;
unmarked files get `__role`-only. The open book-acceptance question is about which
shells' books parse correctly under the full dialect, not about what the marker means.

### B. "Disturbs claims without the flag are useless but create liability." WITHDRAWN.
Without `--risk-faultless-skips`, the survival tier is disabled and walls default
to total. An incomplete disturbs claim under a total-wall default cannot widen the
wall beyond total — it can only narrow it (via the flag). Wrong disturbs claims
without the flag cause zero harm. Liability is dormant until the flag is on.

### C. "The enumerate-every-dimension law degrades existing wrappers silently." WITHDRAWN.
New dimensions cause missing lend_map entries → ⊤ → walls → value-loss only.
The wall direction is safe: facts don't transport across the boundary, so commands
run instead of eliding. Never under-execution.

### D. "`$'…'` ruling contradicts `rul-verdicts-never-stable`." WITHDRAWN.
They govern different domains. `$'…'` is syntax (marker-gated: `# dorc-lang/v0.2`
would admit it). Verdict stability is semantic (not marker-gated). "Expensive to
retract" is about syntax changes that oracles adopt and can't un-adopt, not about
verdict-flip costs.

### E. "The `trap` treatment is potentially soundness-critical." WITHDRAWN.
The unprovable-residual class is handled by `271:rul-unprovable-rides-the-vouch`:
bodies containing trap ship on the author's vouch. The same knife-class as any
unprovable command inside a verdict body. Not a new failure mode.

### F. "The `compare()` relation has underspecified conflict resolution." WITHDRAWN.
Different generators produce different *kinds* of verdicts (same vs
provably-disjoint) that operate at different levels (entity vs cell). No two
generators can produce opposing verdicts for the same pair — they answer different
questions. Contradictions (same generator producing both verdicts) fail fast per
`272` §1's contradiction-detection row.

### G. "Block-rebuild has a hidden dependency on unratified material." WITHDRAWN.
The LIVING_STATUS gates block-rebuild dispatch on the human's adversarial pass,
which covers the unratified items. The charter's ordering is correct.
