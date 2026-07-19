# 280a — solution-A review worklog: authority map and attack queue

AI-authored scratch (2026-07-17). This is an adversarial-review worklog, not a
design proposal and not authority. It records what was checked on the way to
`280c-review-report-sol-a.md`. Reviewed documents remain untouched.

## Review boundary

The response under review is the chain `279f` → amended `275`/`277` + ratified
`271`/`276`/`278` → `27A`/`27B` → `27C` → the rewritten spike `CLAUDE.md`
steering law. Root documents outrank every member of that chain. The two
failure directions used as the first screen are:

- probe: unsure means withhold; the probe is non-mutative;
- apply: unsure means run; never under-execute.

That is not an interpretation imported from older planning. `DESIGN.md:194-226`
requires lifted probes to be non-mutative, prefers no probe to one that may
mutate, and calls even desirable probe-caused mutation forbidden.
`DESIGN.md:290-293` makes “plan stage doesn't mutate” the first correctness
promise. `IMPLEMENTATION.md:137-145` says mutation has no best-effort gradient.
The rewritten implementation law repeats “probe = read-only, parallel, never
mutates” at `spike/CLAUDE.md:55-58`, and says proven-mutating bodies lift
nowhere at `spike/CLAUDE.md:115-124`.

## Candidate attack queue

### 280a:candidate-guest-substitution-changes-context — critical

`27C` defines context entry by replacing the wrapped site's original guest with
the measuring oracle (`Research/plans/27C-context-entry-probing-design.md:146-165`),
and its sudo example spells that as `sudo -n "$@"`. That does not generally enter
the context in which the original guest would run. Sudoers authorization and
execution settings may be selected by command path and arguments; command-specific
rules can alter environment handling, cwd/chroot, SELinux role/type, and AppArmor
profile. Thus `sudo original-mutator ...` and `sudo oracle-checker ...` (or the
batched `sudo sh -c ...`) may both succeed while entering different contexts.

This is not the already-credited transport-completeness gap. It defeats the new
default lane itself: the checker can report convergence in context C-check while
apply would mutate context C-original. No oracle line need be wrong, the entry can
succeed, and `27C` says the default carries no new under-execution risk
(`:278-284`). The missing premise is that entry semantics are guest-insensitive.
Neither `27C` nor the wrapper contract in `273:48-58` requires that premise.

Primary manual evidence: `[A-sudoers-command-policy-current]`, the upstream
`sudoers(5)` manual packaged by Debian, documents command/argument matching,
command-specific `Defaults`, and per-command role/type/profile/cwd/chroot settings:
<https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

### 280a:candidate-entry-form-mutates-during-plan — strong

`27C` makes context entry the default answer for wrapped sites
(`Research/plans/27C-context-entry-probing-design.md:20-25`) and implements sudo
entry as `sudo -n "$@"` (`:154-161`). The plan then expressly says the entry
form's self-effects include “an auth-log line, a timestamp refresh” and treats
those as modeled/elide-alongside (`:163-169`). That is persistent plan-time
mutation by Dorc's wrapper machinery, not merely an inner oracle author breaking
their read-only vouch. It directly contradicts the authority above.

External check: the sudoers manual says allowed commands are logged by default,
and the timestamp manual describes a writable timestamp-file/lock-record
mechanism. PAM session/credential setup may add further effects. Sources to cite
in the final report:

- `[A-sudoers-logging-default-2022]` — upstream sudoers manual as packaged by
  Ubuntu, `log_allowed` on by default:
  <https://manpages.ubuntu.com/manpages/jammy/man5/sudoers.5.html>.
- `[A-sudoers-timestamp-format-2024]` — upstream `sudoers_timestamp(5)` as
  packaged by Ubuntu, timestamp files and lock records:
  <https://manpages.ubuntu.com/manpages/noble/man5/sudoers_timestamp.5.html>.
- `[A-sudoers-pam-effects-current]` — upstream sudoers manual as packaged by
  Debian, PAM sessions can change utmp/wtmp and credential setup can establish
  Kerberos tickets:
  <https://manpages.debian.org/unstable/sudo/sudoers.5.en.html>.

The attack must distinguish two claims: measuring *inside* a changed process
context is not itself a system mutation; invoking a policy/audit wrapper that
writes logs, timestamps, sessions, or credentials is. The former rescue can
survive if implemented with a genuinely non-mutating entry primitive. The
specified sudo default cannot.

### 280a:candidate-flag-contract-widened-silently — strong

The authoritative public contract defines `--risk-faultless-skips` as consent
to at-most footprint claims in the survival tier, and calls that the only naked
trust cell: `USER_STORY.md:665-720`, `KNOBS.md:147-148`, and
`KNOBS.md:276-284`. `27C` silently adds a second mechanism: cross-context
consumption where no line asserts the measuring body's read-completeness
(`27C:197-211`), and directs implementation to extend the same flag to those
outcomes (`27C:317-318`). This is exactly the completeness gap credited by
`279f:65-96`, now accepted rather than repaired.

The mismatch is consent-relevant, not terminological. A user following the
root risk sentence believes the flag means “trust named authors' at-most
claims”; under `27C`, it can also elide because the engine assumed a foreign
measurement depended on nothing beyond an invariant store, a proposition no
author asserted. `279f`'s root-doc rider only requested a caveat if transport
shipped *unflagged* (`279f:190-192`), so it failed to notice that flagged
transport also falsifies the “exactly one place” and “everywhere else” clauses.

### 280a:candidate-dead-verdict-role-in-steering-law — strong, narrower

The response's one-page current language reference lists only
`cmd__is_converged()` (`Research/notes/278-dorc-lang-v0-1-reference.md:147-155`),
and the typed hard-ditch ruling removes `is_diverged` (source:
`Research/notes/24C-stage-landings-and-residue.md:734-758`). The rewritten
steering law nevertheless tells implementing agents twice that
`cmd__is_diverged()` remains a verdict member (`spike/CLAUDE.md:104-108` and
`:353-360`), while `spike/crates/plan/CLAUDE.md` still prescribes its
declared-dual sense-flip glue. This is a direct steering-law/spec divergence on
a permanent `__role` name surface.

### 280a:candidate-capture-binding-gate-missing-from-law — confirmed, narrower

`275:141-143`, `279f:57`, and `279f:127-133` agree that eliding a capture
assignment can unbind live apply-time consumers and therefore needs a hard
gate. The rewritten spike steering law mentions freeze-at-binding and the
representation seams but does not carry this hard gate. The active block package
does import the rider generically as “read-value gates”
(`Research/LIVING_STATUS.md:83-96`), so the requirement is not lost from the whole
corpus. The defect is a steering-law divergence: the seven crate laws are supposed
to be what rebuilding agents consume, yet an acknowledged fail-closed condition
exists only behind an opaque dispatch-package reference. Credit as a bounded
implementation-guidance failure, not as proof that the design forgot the gate.

### 280a:candidate-two-binary-floor-overclaims-portability — investigate

`276:218-227` defines the base dialect by agreement of two pinned binaries,
while the product prose promises broad stripped-oracle reuse. Agreement of two
implementations is not logically sufficient to imply agreement with zsh,
busybox ash, or every POSIX host, especially because `local` is intentionally
outside POSIX. This may be only a wording/product-envelope mismatch because
`276` explicitly makes zsh compatibility a separate quality-bar discipline and
excludes ksh93. Seek a concrete accepted-by-both/rejected-by-care-shell witness
before crediting; otherwise withdraw.

## Early withdrawals

- `280a:withdraw-measure-in-context-as-category`: the repair is right that a
  measurement taken in the site's denoted context avoids the cross-context
  completeness premise. `27A` proved an impossibility only for transport; that
  does not refute in-context measurement. The attack lands on the chosen entry
  implementation and its plan-time effects, not on measurement placement.
- `280a:withdraw-set-e-dismissal`: `279f:199-201` is correct to reject the claim
  that `v=$(false)` is generally errexit-exempt. The assignment's status is the
  substitution's nonzero status in the relevant shells; this dismissal is not a
  repair failure.
- `280a:withdraw-selector-top-patch`: the amended `277:167-176` does repair the
  specific selector-less-backing bug: sparing now requires minted selectors on
  both sides. Broader dialect-growth risk remains deliberate and flag-gated; do
  not recycle the already-adjudicated bug.
