# 27Xb — Design review of the post-crosscheck package (Sol-N)

AI-authored independent review, 2026-07-17. Review point: `de22017`. The reviewed
documents were left untouched; `27Xa` is the working ledger for this report.

## Verdict

This is not a package-wide rejection. The in-place 277 amendments repair the defects
they name, 275 now refuses its unsound transport premise clearly, the pipefail policy is
coherent at its chosen cost, and measuring a wrapped site in its denoted context is a
real solution to the wrong-world problem. The package nevertheless has one
release-blocking correctness defect in 27C's fallback lane and one high-consequence
steering divergence already reflected in the spike. Context entry should remain the
primary direction; the structural fallback and the role surface should not dispatch as
currently written.

## Findings, most severe first

### 27Xb:finding-structural-transport-repeats-completeness-gap — CRITICAL

**Confidence: high (0.98).**

27C's unflagged “engine-warranted carried-by” lane proves that a disclosed backing
*store* is invariant across a dimension; it does not prove that the foreign-context
*measurement computation* is invariant. That is the exact completeness gap the external
review found and the adjudication credited as its most important result.

The accepted diagnosis is explicit: a backing mark is positive disclosure with no
completeness burden, so a verdict may also depend on locale, rho, a cache, or a second
file with no false line anywhere (`Research/notes/279f-crosscheck-adjudication.md:65-77`).
27A sharpened the same rule as “store-invariance is not answer-invariance” and said a
kind owner cannot vouch for a foreign measuring body (`Research/notes/27A-cross-context-transport.md:199-208`);
its later correction says measuring-body read-completeness—not the kind line—is exactly
what forces the flag (`Research/notes/27A-cross-context-transport.md:426-435`).

27C applies that diagnosis correctly to authored ingredient invariance
(`Research/plans/27C-context-entry-probing-design.md:197-211`) but simply exempts the
adjacent structural row, asserting “nothing unsayable” because kernel state is not
filesystem state (`Research/plans/27C-context-entry-probing-design.md:193-196`). The
store may indeed be the same while the answer is not. The one-screen summary is even
internally contradictory: it permits the structural unflagged row and then says “Absent
the flag, nothing travels, ever” (`Research/plans/27C-context-entry-probing-design.md:26-30`).
The 275 supersession banner likewise describes transport as a flag-tier fallback without
this exception (`Research/notes/275-value-predictions-and-the-capture-lane.md:164-169`).

A minimal honest counterexample is:

```sh
policyctl__is_converged() {
    want=$(cat "/etc/policy/$1")
    [ "$(sysctl -n "$1")" = "$want" ] \
        :? sm.dorc.Kernel:"$1"#value
}

chroot /target policyctl apply net.ipv4.ip_forward
```

Let the ambient policy say `1`, the target policy say `0`, and the host-global kernel
cell currently be `1`. The ambient verdict is converged. The kernel backing is correctly
structural-invariant across fs-view, so 27C transports that verdict and elides the line;
the in-context verdict would be diverged and the target's requested update was needed.
The backing line is honest—it identifies a real read—and no contract requires it to list
`/etc/policy`. This is the adjudicated cardinal-sin shape, not an unreliable oracle.

The exclusion check does not rescue it. Reverse propagation does not use `same` as
separation, so that cell is merely unaffected; probe-time elision is still wrong. An
apply-time in-context guard is safe where entry exists, but this lane exists specifically
where entry does not. Both a scrappy admin and a careful oracle author are exposed, and
the counterexample uses a reliable oracle in each context.

**Required correction:** make *all* foreign measuring-body transport flag-tier, including
carried-by rows, or take 27C's recorded `honest-walls-for-worlds` v1 defer. Structural
rows may establish cell identity for keying and other store-level consumers; they cannot
by themselves transport a verdict or world-cell value. A future unflagged lane needs a
closed measurement body/read-set premise, not a stronger adjective on store topology.

### 27Xb:finding-steering-restores-deleted-diverged-role — HIGH

**Confidence: high (0.99).**

The steering compression restores `cmd__is_diverged()` even though the role was
human-typed as a hard deletion from the permanent, unversionable role-name surface.
The ruling says remove the dialect role, reserved suffix, lift, `VerdictSense`, emitted
sense-flip glue, and exercising test (`Research/notes/24C-stage-landings-and-residue.md:734-763`).
The current human-managed needs table repeats that `is_diverged` is DITCHED and that
complement sense uses `:!` (`ANALYZER-NEEDS.md:114`, `ANALYZER-NEEDS.md:342`).

In contrast, the steering law calls authoring either `cmd__is_converged()` or
`cmd__is_diverged()` the vouching act (`spike/CLAUDE.md:104-108`) and lists both in the
permanent role menu (`spike/CLAUDE.md:347-360`). This is especially dangerous under the
stated extension law—role names extend by new name only and become permanent once shipped
(`Research/plans/271-block-settle-rulings-ledger.md:269-276`). An implementing agent
reading only the compression is being directed to freeze the exact API the ruling removed.

This has already propagated beyond prose. The spike reserves the suffix
(`spike/crates/oracle/src/reserved.rs:47-50`), carries a two-case `VerdictSense` and
dual glue (`spike/crates/oracle/src/verdict.rs:70-89`), and tests the obsolete role
positively (`spike/crates/oracle/src/verdict.rs:611-625`; plan glue at
`spike/crates/plan/src/lib.rs:816-826`). A focused
`cargo test -p dorc-oracle diverged_sense -- --nocapture` passed both obsolete-role
tests during this review. The code is throwaway; the evidence matters because it shows
the compressed instruction has exactly the propagation path the review was asked to
audit.

**Required correction:** remove `is_diverged` from both steering occurrences and from
every crate brief/build checklist before rebuild dispatch. Convert the old code/tests to
the ruled explicit ternary/manual-complement spelling; pin a rejection/reservation test
so the deleted permanent name cannot silently return.

### 27Xb:finding-nonroot-authority-cells-do-not-partition — HIGH

**Confidence: high (0.94).**

27C's “four operational cells” omit a common fifth state and contradict the rule used by
the steering compression. The authority rule first says a non-root connection performs
none of the identity/view shifts, then says the only implementable predicate is whether
the connection can do the shift with zero new credentials
(`Research/plans/27C-context-entry-probing-design.md:56-64`). Its own sudo entry form is
`sudo -n "$@"` (`Research/plans/27C-context-entry-probing-design.md:154-160`). A non-root
NOPASSWD automation account satisfies the second predicate and the entry form while
violating the first sentence.

The omission is not hypothetical or newly discovered. The superseded rescue note
explicitly classified NOPASSWD `sudo -n` as authority already held and separated it from
password-gated sudo (`Research/notes/27B-measurement-placement-rescue.md:133-139`). But
27C's ruled list has root+permit, root+forbid, non-root+explicit acquisition, and
non-root+nothing—no non-root+already-held-authority cell
(`Research/plans/27C-context-entry-probing-design.md:78-83`). It simultaneously says all
four are implemented while deferring the acquisition mechanism
(`Research/plans/27C-context-entry-probing-design.md:304-326`).

The compression silently chooses the capability reading: exercise authority the
connection already holds, with no prompting or credential handling, and no privilege
ordering (`spike/CLAUDE.md:203-214`). It omits 27C's categorical non-root refusal.
Therefore an agent reading only the steering law will permit NOPASSWD entry, while one
following the centerpiece literally will wall it. The disagreement affects the default
probe lane and the same apply-guard lane, and it changes both the security/consent story
and the advertised value for a common automation deployment class.

**Required correction:** partition by capability, not uid: distinguish
zero-new-credential entry already available (root, NOPASSWD, or any equivalent) from a
separate pre-probe acquisition transition and from unavailable entry. State whether a
cached sudo timestamp counts as already held or acquired. Then make 27C and the steering
bullet identical; do not call a deferred acquisition transition an implemented cell.

### 27Xb:finding-nested-wrapper-composition-rider-was-dropped — MEDIUM

**Confidence: high (0.95).**

The adjudication credited a concrete prior-review gap—nested wrapper lend/rho composition
was unstated—and dispatched an explicit rule: pointwise composition with top propagation
(`Research/notes/279f-crosscheck-adjudication.md:54`,
`Research/notes/279f-crosscheck-adjudication.md:127-133`). Neither the new centerpiece
nor the steering compression carries it.

27C says only that entry forms “compose recursively” and then relies on one segment per
`(host, context)` (`Research/plans/27C-context-entry-probing-design.md:172-185`). That
does not define how nested `cmd__lend_map()` results and rho transformations determine
the context key, composition direction, or how an unknown component poisons the result.
The steering wrapper law retains the suffix/peel family and dual-peel coherence but no
nested composition algebra (`spike/CLAUDE.md:386-392`). The omission matters even when
entry succeeds: context-qualified fact identity and batching need a canonical composed
context. It also affects both directions when entry fails—transport keying and
disturbance/survival routing can consume the same wrong composition.

For example, `sudo -u alice env HOME=/srv/app nice tool ...` composes a mapped user lend,
a rho override, and two identity/full lends. Independently honest wrapper oracles do not
tell the engine whether outer or inner maps apply first; a guessed last-wins fold can
mis-key a fact, while a conservative top should wall. The unreliable-oracle cell is not
needed: this is engine composition of reliable declarations.

**Required correction:** add the dispatched pointwise fold to 27C and reproduce it in
the steering law: explicit order, identity element, mapped/full interaction, top
propagation, and canonical context-key construction. Pin nested permutations in both
transport and kill-traffic consumers.

### 27Xb:finding-residue-containment-is-weighted-as-design-before-it-is-designed — MEDIUM

**Confidence: high (0.91).**

27C uses conditional tails and the acquired-authority cell to justify its claim that the
non-entry residue is contained, yet labels both mechanics deferred. The one-screen design
says guard/conditional tails prevent one wrapped wall from costing the rest of the book
its shape (`Research/plans/27C-context-entry-probing-design.md:31-33`). Section 5 then
uses them to claim zero steady-state check tax and to revive generation probes, while
saying detailed design belongs to a later round
(`Research/plans/27C-context-entry-probing-design.md:221-236`). The residual-hole section
leans on the same feature to contain non-root drift cost
(`Research/plans/27C-context-entry-probing-design.md:266-270`). Finally, the status ledger
classifies conditional-tail mechanics as STRAWMAN and the build list defers both their
render fold and the acquisition mechanism (`Research/plans/27C-context-entry-probing-design.md:324-349`).

The direction is plausible; this review found no counterexample that kills it. The claim
weight is still premature. Multiple may-run walls, errexit/rc preservation, footprint-
specific invalidation, and may-execute rendering are precisely the mechanics that decide
whether the promised attention/check-tax benefit exists. Leaving those to an implementing
agent while using their success to grade the non-root residue “best effort” stores up
control-flow and rendering commitments in the plan representation before the later round
can choose them.

**Required correction:** separate ruled safety semantics from value projections. Until
the placement-spectrum design lands, specify ordinary guard/run as the implemented floor
and call conditional tails an experiment, not the containment result. Prototype the
multi-wall/errexit/observable cases before freezing plan IR around the flag scheme.

## Suspicions checked and withdrawn

- **27Xb:withdrawn-amendments-only-rename-findings.** The 277 amendments do resolve the
  prior findings rather than relabel them: selector sparing now requires selectors on
  both sides (`Research/notes/277-entity-algebra-design.md:167-176`); the property list
  admits within-family collide-to-spare flips and downgrades attribution honestly
  (`Research/notes/277-entity-algebra-design.md:179-205`); backing sets use a universal,
  order-independent meet (`Research/notes/277-entity-algebra-design.md:388-415`); and
  divergent cross-family meaning is frontloaded as a documented, differential-tested
  limitation (`Research/notes/277-entity-algebra-design.md:446-486`). I found no residual
  defect in those amendments beyond their recorded flag-gated risk.
- **27Xb:withdrawn-primary-entry-renames-transport.** It does not. Measurement in the
  site's denoted context creates the fact in the right world and removes the need for an
  answer-invariance claim (`Research/plans/27C-context-entry-probing-design.md:20-25`,
  `Research/plans/27C-context-entry-probing-design.md:144-185`). The critical finding is
  confined to the fallback that still transports a foreign answer.
- **27Xb:withdrawn-271-versus-27c-authority-conflict.** 271's stopping-point ruling cites
  `24S:imp-1`; the cited passage now contains an in-place re-scope to
  reuse-never-acquire (`Research/plans/24S-wrapper-contexts-and-the-context-algebra.md:56-67`).
  Newest-wins plus that annotation resolves the apparent conflict. The non-root cell
  contradiction above is internal to the resulting rule, not an authority-order attack.
- **27Xb:withdrawn-entry-self-effects-hidden.** Authentication logging/timestamp refresh
  initially looked like a breach of the no-mutation probe law. The root story explicitly
  refuses to equate read-only with side-effect-free
  (`USER_STORY.md:956-964`), and the threat model transfers incidental probe effects to
  the oracle contract while warning against a stronger promise
  (`Research/plans/102-dorc-threat-model.md:55-75`). 27C names and attributes these costs
  (`Research/plans/27C-context-entry-probing-design.md:163-169`). They remain an operational
  cost, but not a quiet contradiction on the evidence reviewed.
- **27Xb:withdrawn-forged-verdict-defense.** The hostsim phrase “kFAIL defense” sounded as
  though Must-licensing could make a hostile host truthful. Its actual required assertion
  is narrower: forged convergence must not bypass the independent Must license
  (`spike/crates/hostsim/CLAUDE.md:31-36`). The maintained needs table correctly leaves
  malicious-host verdict forgery open (`ANALYZER-NEEDS.md:290`). No design claim closes
  that threat.
- **27Xb:withdrawn-pipefail-offramp-remains-cracked.** The response now separates parser
  acceptance from base-dialect conformance: durable generated text never emits bare
  pipefail, authored bare pipefail is linted but not rewritten, and the two-binary floor
  is the conformance gate (`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:192-211`;
  `Research/notes/278-dorc-lang-v0-1-reference.md:193-200`). This is deliberate ceremony,
  not a hidden off-ramp guarantee. The steering compression is faithful here
  (`spike/CLAUDE.md:434-441`).
- **27Xb:withdrawn-pipefail-handshake-must-already-be-context-keyed.** A chroot/container
  can select a different shell than the ambient host, so a future executor should qualify
  the handshake by the actual evaluator context. I did not find a supported-envelope
  wrong-elision today: failure to enable pipefail before the body yields no trustworthy
  record and therefore run, while pipefail-less executors are explicitly unsupported
  (`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:163-179`). This is a useful
  implementation pin, not a design finding at present.
- **27Xb:withdrawn-entry-abi-impossibility.** The `sudo -n "$@"` strawman does not by itself
  explain how shell function bodies cross exec/chroot boundaries. Synthesized CFG
  scaffolding is explicitly allowed alongside oracle bytes (`spike/CLAUDE.md:148-150`),
  so nested `sh -c`/stdin forms can provide an ABI and fail toward run when the target
  lacks an evaluator. The details are underdesigned, but I could not establish an
  impossibility or unsafe default distinct from the maturity finding above.
