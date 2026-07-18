# 27Xc — adversarial review report: solution A

AI-authored independent review, 2026-07-17.

## Verdict

**Do not harden or implement `27C` as written.** Its default sudo lane can measure a
different execution context from the one apply will use, then silently elide the apply.
Separately, its specified entry mechanism knowingly mutates during planning despite a
categorical root contract against that. Those are two independent blockers: repairing
either one does not repair the other.

This review does **not** reject measurement-in-context as a category. `27A` correctly
exposed the transport-completeness problem, and moving measurement really can remove
that premise if the entry primitive is both context-equivalent and non-mutating. The
review rejects the claim that replacing a `sudo` guest with checker bytes has those
properties. The entity-selector amendment also survives review. The response contains
real repairs; it is not safe as a package because its replacement mechanism is not one.

## Findings, highest severity first

### 27Xc:critical-substituted-guest-changes-context

**Severity:** Critical — default-path silent under-execution, Dorc's cardinal sin  
**Confidence:** High

`27C` says the default probe enters “the site's denoted context,” runs the oracle there,
and safely degrades every entry failure to can’t-say
(`Research/plans/27C-context-entry-probing-design.md:20-25`). Its actual mechanism is
to put the composed checker in the wrapped command's guest position:

```sh
sudo__enter() {
    sudo -n "$@"
}
```

That substitution is specified at
`Research/plans/27C-context-entry-probing-design.md:146-165`; successful recursive
entry is then treated as sufficient at
`Research/plans/27C-context-entry-probing-design.md:172-181`. It is not sufficient
for `sudo`, because sudo's policy and execution context can depend on the **guest command
and its arguments**.

A concrete policy witness is:

```sudoers
alice ALL = (root) CWD=/apply-world /usr/bin/original *, \
                   CWD=/probe-world /bin/sh *
```

For a book site such as `sudo /usr/bin/original relative-state`, apply runs the original
with `/apply-world` as cwd. A batched context probe of the form the design proposes,
`sudo -n sh -c '…checker…'`, runs the checker with `/probe-world` as cwd. Both entries
succeed. An honest checker can report that `relative-state` is converged in the probe
world while the original would mutate the apply world. The same witness can be built
with per-command `CHROOT`, SELinux role/type, AppArmor profile, command tags, or
command-specific defaults.

This behavior is not an obscure extension. `[A-sudoers-command-policy-current]`, the
upstream `sudoers(5)` manual packaged by Debian, specifies command-and-argument matching,
command-specific defaults applied after the command path is known, and per-command
cwd/chroot/SELinux/AppArmor settings:
<https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

The package's own wrapper definition makes the missing premise visible. A peeling
wrapper is defined to execute its remainder **verbatim**, with nothing substituted
(`Research/notes/273-wrapper-surface-redesign.md:48-58`). `27C` substitutes the
remainder, but nowhere requires the entry transformation to be guest-insensitive or
proves equivalence between the checker's and original guest's policy-selected contexts.

None of `27C`'s gates catches the witness:

- the wrapper is modeled and its entry form exists;
- probe escalation is the default
  (`Research/plans/27C-context-entry-probing-design.md:66-83`);
- the entry succeeds, so the can’t-say-on-failure rule does not fire;
- the checker can be read-only and its tolerance vouch can be true;
- no human-authored oracle statement is false; and
- the faultless-risk flag is not involved.

The result directly falsifies the claim that the default path adds zero new
under-execution risk
(`Research/plans/27C-context-entry-probing-design.md:278-284`). This is also why
tracer-based validation of checker read sets cannot repair the mechanism: it can show
what the checker read in its world, not that apply's different guest would receive the
same sudo policy or inhabit the same world.

**Required disposition:** withdraw sudo as a default entry form. A replacement needs
an explicit, falsifiable contract that entry is independent of the substituted guest,
or a mechanism that reproduces the original guest's complete policy-selected context
without executing that guest. “The wrapper was in the book” is consent, not an
equivalence proof.

### 27Xc:blocker-context-entry-mutates-plan

**Severity:** Blocker — categorical plan-safety contract violation  
**Confidence:** High

The design does not merely overlook possible sudo effects. It names an auth-log line
and timestamp refresh, then declares them “modeled, elide-alongside” as acceptable
entry self-effects (`Research/plans/27C-context-entry-probing-design.md:163-169`). Its
precursor makes the same ruling and proposes batching partly to reduce, not eliminate,
auth-log writes (`Research/notes/27B-measurement-placement-rescue.md:218-268`).

That is incompatible with every higher-authority statement of the probe contract:

- probes are vouched non-mutative, and not probing is preferable to possible mutation;
  even desirable probe-caused mutation is forbidden (`DESIGN.md:194-226`);
- “plan stage doesn't mutate” is a top-level correctness promise
  (`DESIGN.md:290-293`);
- mutation has no best-effort gradient (`IMPLEMENTATION.md:137-145`); and
- the rewritten agent law says probe is read-only and never mutates, while a
  proven-mutating body lifts nowhere (`spike/CLAUDE.md:55-58`,
  `spike/CLAUDE.md:115-124`).

Calling the write “modeled” cannot reconcile the contradiction. Modeling can predict
or attribute an effect; it cannot unwrite an audit record or credential-cache update
after a plan the admin may decline. Calling the entry form oracle-authored only moves
the conflict: the oracle contract itself requires shipped probe bytes to be
non-mutative. `27C` creates a licensed exception while the steering law continues to
say no exception exists.

The effects are externally confirmed:

- `[A-sudoers-logging-default-current]`: sudoers logs successful and unsuccessful
  attempts by default:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.
- `[A-sudoers-timestamp-format-2024]`: credential-cache records are writable and their
  timestamp is updated when commands run:
  <https://manpages.ubuntu.com/manpages/noble/man5/sudoers_timestamp.5.html>.
- `[A-sudoers-pam-effects-current]`: default PAM session/credential handling may alter
  utmp/wtmp or establish credentials such as Kerberos tickets:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

The issue remains even if a particular estate disables timestamps or file logging:
the default mechanism is licensed without proving those conditions, and the root
contract prefers withholding a probe whenever mutation is possible. The design's own
example therefore fails its stated admission rule.

**Required disposition:** either preserve the categorical non-mutation contract and
admit no mutating entry forms, or obtain a human-owned root-contract change before
implementation. The latter would be a product-contract change, not a routine wrapper
vouch.

### 27Xc:high-risk-flag-contract-widens

**Severity:** High — explicit consent no longer means what the public contract says  
**Confidence:** High

The public contract describes one naked-trust cell. It says cheap honesty runs out
“exactly one place”: survival past a running wall on other authors' at-most footprint
claims. It tells the admin that `--risk-faultless-skips` means trusting named authors'
at-most claims and that everywhere else Dorc trusts measurements
(`USER_STORY.md:665-720`). The named `kSURVIVAL` knob repeats that this is the design's
one naked trust (`KNOBS.md:144-148`). The plan-crate steering law still repeats the
same promise (`spike/crates/plan/CLAUDE.md:53-65`).

`27C` assigns a second license to that flag. In its fallback lane, a kind-owner may
assert that a store is invariant across a dimension, but nobody asserts that the
foreign checker depends on nothing beyond that store. The plan correctly recognizes
that missing read-completeness claim as unsayable, then makes the existing flag own
the resulting cross-context consumption
(`Research/plans/27C-context-entry-probing-design.md:187-211`). Its build list explicitly
extends `--risk-faultless-skips` to fallback outcomes
(`Research/plans/27C-context-entry-probing-design.md:310-322`).

This is not the same transaction described to the admin:

- survival trusts an author's at-most claim about what a **running mutator disturbs**;
- fallback trusts the absence of unmodeled dependencies in a **foreign measuring
  body**, a proposition no author said.

The general razor ruling does anticipate that the flag might absorb future unsayable
residue (`Research/plans/271-block-settle-rulings-ledger.md:651-665`). That makes the
new plan internally intelligible; it does not update the higher-authority user
contract, the named knob, or the implementing crate law. It also does not preserve
the advertised attribution story: `27C` itself says a fallback failure has no single
human fault (`Research/plans/27C-context-entry-probing-design.md:204-211`), whereas
the public receipt promises named authors' claims.

The self-adjudication missed this exact repair obligation. Its root-doc queue asks for
a USER_STORY caveat only if transport ships **unflagged**
(`Research/notes/279f-crosscheck-adjudication.md:186-192`). Flagged transport also
falsifies “exactly one place,” “one naked trust,” and “everywhere else.”

**Required disposition:** do not ship fallback under the existing flag contract.
Either keep honest walls for foreign-world measurements, introduce separately named
and explained consent, or human-ratify a broadened root contract and update every
consumer-facing and agent-facing statement before implementation.

### 27Xc:medium-deleted-role-remains-steering

**Severity:** Medium — contradictory rebuild instructions on a permanent public name  
**Confidence:** High

The human-typed ruling removes `is_diverged` hard and immediately, explicitly because
role names are a permanent compatibility surface
(`Research/notes/24C-stage-landings-and-residue.md:734-758`). The current language
reference lists only `cmd__is_converged()` as the verdict role
(`Research/notes/278-dorc-lang-v0-1-reference.md:147-155`), and the active rebuild
package imports the hard ditch (`Research/plans/270-round27-charter.md:82-95`).

The rewritten steering law nevertheless tells implementing agents that authoring
`cmd__is_diverged()` is a vouch (`spike/CLAUDE.md:104-108`) and includes it in the role
menu (`spike/CLAUDE.md:350-362`). The plan-crate law then prescribes the removed role's
declared-dual sense-flip glue (`spike/crates/plan/CLAUDE.md:37-48`).

This is not harmless historical prose. The files are the law consumed by rebuilding
agents, and they direct those agents to recognize or reconstruct a role that the build
package says must be deleted before published oracles make deletion costly. Following
either instruction violates the other.

**Required disposition:** remove the dead role and dual glue from the steering law
before dispatching the rebuild; add a negative acceptance pin that `__is_diverged` is
neither reserved nor recognized.

## Suspicions checked and withdrawn

### 27Xc:withdraw-measurement-placement-category

The original cross-context finding was real. The adjudication accurately states that
the transport chain consumes backing as a completeness claim no authored line makes
(`Research/notes/279f-crosscheck-adjudication.md:65-90`). Measuring in the **actual**
target context would remove that transport premise. The failure above is that sudo
guest substitution does not establish actual-context equivalence, not that in-context
measurement is impossible.

### 27Xc:withdraw-capture-gate-was-lost

The rewritten crate laws do not spell out the hard gate against eliding a capture
binding with live apply-time consumers. That initially looked like a dropped credited
finding (`Research/notes/275-value-predictions-and-the-capture-lane.md:141-143`;
`Research/notes/279f-crosscheck-adjudication.md:127-133`). The active dispatch package,
however, imports the §5 riders, including “read-value gates”
(`Research/LIVING_STATUS.md:83-96`). This is poor discoverability, but the requirement
is not proved lost.

### 27Xc:withdraw-selector-backing-amendment

The amended sparing rule genuinely repairs the credited selector-less-backing defect:
it now requires minted selectors on both entities before dialect-disjoint sparing
(`Research/notes/277-entity-algebra-design.md:167-176`). Broader dialect-growth risk is
deliberate and flag-gated; recycling the original finding would be unfair.

### 27Xc:withdraw-errexit-dismissal

The adjudication is right to dismiss the claim that `v=$(false)` is generally exempt
from `errexit` (`Research/notes/279f-crosscheck-adjudication.md:194-201`). In the
relevant shells the assignment receives the command substitution's nonzero status.
This does not expose a repair failure.

### 27Xc:withdraw-two-binary-portability

Agreement between two pinned bash/dash binaries does not logically prove every POSIX-ish
shell. But `276` defines that agreement as the base language floor and separately puts
zsh/ash behind quality-bar testing. Without a concrete construct accepted by the floor
and rejected by a supported care-shell, this remains a design trade, not a demonstrated
defect (`Research/notes/276-language-sitting-kwhichsh-unsafe-churn.md:218-227`).

### 27Xc:withdraw-status-descriptor-loss

Sudo commonly closes inherited descriptors above stderr, which first appeared to break
the out-of-band probe-status lane. An outer process can capture the entered command's
status and re-encode it after sudo exits. That creates implementation and batching
constraints, but it does not by itself contradict the design.

## Bottom line

The response correctly refused its first transport proposal and found the right
*direction*—measure where the fact lives. It then promoted one implementation of that
direction without proving its load-bearing equivalence relation, while expressly
waiving the project's other categorical probe invariant. Because the first flaw can
silently under-execute on the default path and the second mutates during a plan, the
package has not crossed its own bar for dispatch. Freeze `27C`; keep the valid
transport impossibility result and the measurement-placement insight; redesign entry
from those constraints rather than treating sudo invocation as context identity.
