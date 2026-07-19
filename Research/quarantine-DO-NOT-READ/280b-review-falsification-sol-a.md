# 280b — solution-A review falsification ledger

AI-authored scratch (2026-07-17). This note records the decisive counterexamples
and the attacks withdrawn before the final `280c` report. It is not design authority.

## 280b:counterexample-substituted-guest-changes-sudo-context

The default repair says it enters the wrapped site's context by placing a composed
oracle invocation in guest position (`Research/plans/27C-context-entry-probing-design.md:
146-165`). That substitution is not context-preserving for `sudo`.

One concrete permitted policy shape is:

```sudoers
alice ALL = (root) ALL
Defaults!/usr/bin/original CHROOT=/apply-world
Defaults!/usr/local/libexec/dorc-check CHROOT=/probe-world
```

The site's apply guest is `/usr/bin/original`; the entry-form guest is the oracle
checker (or, under batching, `sh -c ...`). Both commands can be authorized and both
can return success. Sudo applies command-specific settings only after it knows the
command path, however, so the checker observes `/probe-world` while the original
would act in `/apply-world`. Substitute `CWD`, SELinux `ROLE`/`TYPE`, an AppArmor
profile, command tags, or command-specific environment policy for `CHROOT`; the
logical failure is the same.

Evidence:

- `27C:20-25` claims the fact is born in the site's denoted context and every entry
  failure degrades safely; here entry succeeds in the wrong context.
- `27C:146-165` specifies the guest substitution, while `27C:172-181` treats successful
  recursive entry as sufficient.
- `27C:278-284` therefore overclaims that the default adds zero under-execution risk.
- The ratified wrapper-family boundary says an actual peeling wrapper executes its
  remainder verbatim, with nothing substituted (`Research/notes/273-wrapper-surface-redesign.md:
  48-58`). The probe is intentionally not that execution.
- `[A-sudoers-command-policy-current]`, the upstream `sudoers(5)` manual packaged by
  Debian, documents command-and-argument matching, command-specific `Defaults`, and
  per-command chroot/cwd/SELinux/AppArmor settings:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

No oracle author must be wrong in this execution. The checker can faithfully answer
for the world it sees; Dorc incorrectly equates that world with the original guest's
world. No existing tolerance vouch speaks to this relation. The entry author vouches
for self-effects, not guest-insensitivity (`27C:163-169`). The escalation dial is on
by default (`27C:66-83`), and the faultless-risk flag is not required. This is a
default-lane cardinal-sin witness.

## 280b:contradiction-entry-form-mutates-plan

`27C` expressly prices an auth-log line and timestamp refresh as entry self-effects
and calls them “modeled, elide-alongside” (`27C:163-169`). The precursor makes the
same move and batches commands partly to reduce auth-log noise
(`Research/notes/27B-measurement-placement-rescue.md:218-268`). Modeling a write does
not make it non-mutative.

This contradicts the highest-authority contract:

- lifted probes are vouched non-mutative, and no probe is preferable to one that may
  mutate (`DESIGN.md:194-226`);
- “plan stage doesn't mutate” is the first correctness promise
  (`DESIGN.md:290-293`);
- there is no best-effort gradient for probe mutation
  (`IMPLEMENTATION.md:137-145`);
- the rewritten steering law repeats “probe = read-only ... never mutates” and says
  proven mutation lifts nowhere (`spike/CLAUDE.md:55-58`, `:115-124`).

The external mechanism is not hypothetical. Sudoers logs allowed commands by
default, updates credential-cache timestamps, and by default may open PAM sessions
or establish target credentials:

- `[A-sudoers-logging-default-current]`:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>;
- `[A-sudoers-timestamp-format-2024]`:
  <https://manpages.ubuntu.com/manpages/noble/man5/sudoers_timestamp.5.html>;
- `[A-sudoers-pam-effects-current]`:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

The category “measure inside a process context” survives this attack. A genuinely
non-mutating, guest-insensitive entry primitive could still implement it. The
specified sudo entry form cannot satisfy Dorc's existing plan contract.

## 280b:contract-drift-risk-flag-absorbs-second-license

The public contract says cheap honesty runs out “exactly one place”: survival past a
running wall on authors' at-most footprint claims. It tells the admin, in one sentence,
that `--risk-faultless-skips` means trusting named authors' at-most claims, while
everywhere else Dorc trusts measurements (`USER_STORY.md:665-720`). `KNOBS.md:144-148`
likewise calls `kSURVIVAL` the design's one naked trust.

`27C` adds a different license to that flag. For cross-context fallback, the kind
owner makes an invariance claim, but no author claims that the foreign measuring
body depends on nothing outside the invariant store. The plan acknowledges this
unsayable completeness premise, then makes the same flag own it (`27C:197-211`) and
orders implementation to extend the flag to transport outcomes (`27C:310-322`).

The general razor ruling in `Research/plans/271-block-settle-rulings-ledger.md:
651-665` does contemplate future mechanisms with the same faultless anatomy. It
therefore supports internal consistency within the new planning layer. It does not
repair the user contract that the flag still exposes, nor the claim that this is a
double opt-in by an at-most-claim author and an admin. `279f` requested a root-doc
caveat only if transport shipped unflagged (`Research/notes/279f-crosscheck-adjudication.md:
190-192`), missing the flagged contract expansion.

## 280b:steering-law-retains-deleted-role

The human-typed ruling removes `is_diverged` hard and immediately, because role names
are a permanent compatibility surface (`Research/notes/24C-stage-landings-and-residue.md:
734-758`). The response's current language reference lists only `cmd__is_converged()`
as verdict role (`Research/notes/278-dorc-lang-v0-1-reference.md:147-155`). The active
rebuild package also imports the ditch (`Research/plans/270-round27-charter.md:82-95`).

Nevertheless, the rewritten root steering law tells agents that authoring
`cmd__is_diverged()` is a vouch and lists it in the role menu
(`spike/CLAUDE.md:104-108`, `:350-362`). The plan-crate law goes further and prescribes
declared-dual sense-flip glue (`spike/crates/plan/CLAUDE.md:37-48`). An implementing
agent following the steering law can therefore preserve or rebuild the exact public
role that the package says must disappear.

## Withdrawals after falsification

- **280b:withdraw-measurement-placement-category.** `27A` is right that transport
  alone cannot repair missing read-completeness, and `27B` is right that measuring
  in the actual context would avoid that transport premise. The kill is the specified
  entry relation, not measurement placement as a category.
- **280b:withdraw-capture-gate-lost.** The crate steering law omits the hard gate
  against eliding a binding with live apply consumers (`275:141-143`, `279f:127-133`),
  but the active dispatch package imports the rider as “read-value gates”
  (`Research/LIVING_STATUS.md:83-96`). This is discoverability debt, not a proved lost
  requirement.
- **280b:withdraw-selector-backing-repair.** The amended selector rule in
  `Research/notes/277-entity-algebra-design.md:167-176` does close the credited
  selector-less-backing hole by requiring selectors on both sides.
- **280b:withdraw-errexit-dismissal.** `279f:199-201` correctly rejects the claim that
  `v=$(false)` is generally exempt from errexit; the assignment takes the substitution's
  nonzero status in the relevant shells.
- **280b:withdraw-two-binary-portability.** Agreement of pinned `bash` and `dash` is
  not proof of every shell, but `276` uses it as a deliberately defined language floor
  and separately quality-gates zsh/ash. Without a concrete admitted-by-floor,
  rejected-by-supported-shell witness, this does not establish a response defect.
- **280b:withdraw-status-fd-loss.** Sudo may close descriptors above 2, which initially
  looked fatal to the out-of-band status lane. An outer probe process can capture and
  re-encode the entered command's status, so descriptor inheritance is an implementation
  constraint rather than a design contradiction.
