# x2b — the records-admission seam (PROPOSAL; nothing built)

Answers `28L:tc-harness-records-vs-controller-scope`. Written by the X2b builder; the conductor
reviews before any of it is built. Nothing in this document is landed.

## §1 — What is blocked, and by exactly what

~35 arrangement prose-components in the `why-*` families (survival, guard, measured) never render
from the loom harness, because every fact in a harness world is ⊤ and every site RUNS. X2a stated
the cause in `cli/src/world.rs`'s own header, and it is not a missing parameter: a fact becomes
non-⊤ only by way of ADMITTED probe records, and admission binds records to controller-minted
attribution (`rul-attribution-is-controller-minted`). Today the whole segment that turns bytes into
attributed facts lives inside the `dorc` BINARY:

| step | today | where |
|---|---|---|
| bound the bytes | `records::read_host_evidence(reader, limits)` | `dorc-plan`, public |
| check framing, type the records | `records::admit_unscoped_host_records(&bytes, &framing, limits)` | `dorc-plan`, public |
| mint the run's identity | `WidthOneAttemptScope::new(&framing, book_name, book, paths, srcs)` | `cli/src/main.rs`, **private** |
| type the records into facts | `parse_admitted_results(&records, &mut RunClock, &mut Interner)` | `cli/src/main.rs`, **private** |
| keep identity attached | `ScopedHostEvidence::new(scope, results)` | `cli/src/main.rs`, **private** |

The first two are already reachable — `dorc-loom`'s `fire_records_admission` calls both, and uses the
public fixture framing `records::Framing::spike(book_digest)`. Only the last three are out of reach,
and they are out of reach because they are `main.rs`-private, not because anything about them is
impure: `WidthOneAttemptScope`, `SiteResults`, `RecordKey`, `SiteRecord` and `ScopedHostEvidence` are
plain owned data, and `parse_admitted_results` is a total function of `(records, clock, interner)`.

## §2 — The proposed seam

**One move, one new constructor, no new mechanism.**

**(a) Move the three private items to `cli/src/lib.rs`, fields still private.** `WidthOneAttemptScope`,
`ScopedHostEvidence<T>`, `SiteResults` (+ `RecordKey`/`SiteRecord`) and `parse_admitted_results` cross
to the lib target under the existing `lib-target-is-a-loom-seam` law. Every field stays private and
no field-wise constructor is added, so a payload still cannot mint a scope
(`rul-attribution-is-controller-minted`: "a payload frame may be CHECKED against expected controller
values; it never mints them"). `RunClock` moves with them — it is already a pure enum whose one
non-deterministic variant (`Ticking`) is fed from the binary's edge; the lib never reads a clock, it
is handed instants.

**(b) One fixture-tier entry point, whose signature cannot produce a remote scope.**

```rust
// dorc_cli — the ONLY constructor a non-binary caller can reach.
pub fn admit_fixture_records(
    book_name: &str, book: &str,
    oracle_paths: &[String], oracle_srcs: &[String],
    stream: &[u8],
) -> Admission<ScopedHostEvidence<SiteResults>>
```

It builds `Framing::spike(book_digest(book))` INTERNALLY and takes no `RemoteIdentity`, no host, no
nonce, no attempt. That is the whole fence, and it is the fence the corpus already uses:
`rul-fixture-identity-never-production` — *"comments are not a fence — absence of a constructor is"* —
and `records::Framing`'s own doc, which explains that `spike` "is structurally unable to reach a
managed host because reaching one requires a `RemoteIdentity`, which this constructor does not produce
and cannot be turned into". `Framing::for_remote` stays where it is and stays the binary's; a fixture
caller has no way to name one. The existing lexical gate
`spike_identity_is_not_reachable_from_transport` keeps holding, and a sibling lexical gate should
assert that `admit_fixture_records` has exactly the callers we intend (the `licence_mint_has_exactly_
one_caller` precedent in `dorc-plan`).

**(c) The records reach the harness the way a real run's do: through the case's own `< file`.** A
`dorc plan --book=book.sh -o pkg.oracle.sh < probe-results.txt` replay already parses in the harness
(`DirectPlan::input`); today those bytes only reach `fire_records_admission`, which requires a
REFUSAL. The proposal is that the same bytes, when they are ADMITTED, feed `admit_fixture_records`
and the resulting `ScopedHostEvidence<SiteResults>` becomes an optional input to X2a's
`WhyWorld::analyze`, replacing its `|_| Observable::verdict_only(Verdict::Unknown)` fold with the
real records-fed one. Nothing else about the world derivation changes.

**(d) The case's section carries the FULL framed form.** `28H:rul-fixture-records-enriched-not-reduced`:
the committed `probe-results.txt` section is a real `dorc-records/1` stream — header line with
`nonce=`/`attempt=`/`host=`/`book=`/`sites=`, one `dorc <record> @@dorc@@` line per record, and the
`dorc-records-end/1` sentinel — not the legacy headerless `site 0 effect=holds` shorthand the e2e
runner synthesizes framing around. Two consequences, both wanted: the case is a real test of the
intake (a hand-mangled header REFUSES, which is what makes `records-*` cases possible on the same
seam later), and the `book=` digest binds the section to the exact book bytes beside it, so editing
the book without re-deriving the digest fails loudly rather than folding a stale measurement.

Authoring aid: the digest is `dorc_plan::invocation::book_digest`, and `dorc-loom` should print the
expected header in the refusal when a case's stream fails the `book=` check — `28L:rul-refusals-name-
the-next-command` makes that mandatory, not optional.

## §3 — Every law this touches, and how it is left standing

- **`rul-attribution-is-controller-minted`** — the scope is still minted from immutable
  controller-owned invocation context; the controller is now `dorc-loom` for its own in-process runs,
  exactly per the conductor's guidance. Payload bytes still mint nothing; the header is still only
  CHECKED against the framing the caller supplies.
- **the re-entry trigger in the same rule** ("any second scope becoming representable at all — real
  transport, concurrency, retry, cross-host reuse, saved approval") — **not tripped, and this needs
  the conductor's explicit agreement.** A loom run mints exactly ONE scope, for one book, in one
  process, with no transport and no retry; two scopes never co-exist in one analysis. What becomes
  representable is a second *controller*, not a second *scope*. If the conductor reads the rule the
  other way, this whole proposal fails and §5's fallback applies instead.
- **`rul-host-bytes-bounded-before-admission`** — untouched: the same `read_host_evidence` bound, the
  same `HostEvidenceLimits::spike_default()`, the same closed-grammar parse. The harness gains no
  parser and no leniency; in particular `LegacyPolicy::Tolerate` is NOT reachable from this path.
- **`rul-admission-is-a-closed-outcome`** — the harness must honour all three arms, not just the two
  it wants: `Admitted` feeds the fold, `NoObservation` yields the all-⊤ world it has today, and
  `Refused` refuses the CASE (the transcript renders the refusal diagnostic, which is exactly what
  `fire_records_admission` already does). Collapsing `Refused` into "no records, carry on" would
  reproduce the precise failure the law names.
- **`rul-fixture-identity-never-production`** — one named substitution point, not copies: the fixture
  framing stays `Framing::spike`, the fixture clock stays the existing harness clock pin, and
  `admit_fixture_records` adds a third that is spelled the same way (a constructor that cannot name
  the production input).
- **`lib-target-is-a-loom-seam`** — VALUES cross, QUERIES do not. `stream: &[u8]` and the source
  strings cross; opening the file, reading stdin, and reading the clock stay in `main.rs`.
- **`inv-determinism`** — the moved segment reads no clock; the fixture path passes
  `RunClock::Recorded(BTreeMap::new())` (every ordinal answers `None`), so a transcript is a fixpoint
  by construction rather than by a normalizer — which matters, because
  `seam-tolerated-nondeterminism-stops-at-the-run-log` means a rendered surface has no declared-class
  escape hatch.
- **`inv-site-keyed-results` / `results-fold-to-run`** — unchanged; the harness uses the same fold, so
  a missing fact still folds to run.
- **`two-phases-opposite-fail-directions`** — unchanged; nothing here decides anything.

## §4 — Alternatives considered and rejected

1. **An unframed side-door** — hand `SiteResults` a case's `site 0 effect=holds` lines directly,
   skipping `read_host_evidence`/`admit_unscoped_host_records`. REJECTED: it is the explicit
   no-unframed-side-doors constraint, it makes the `< file` decorative again after D3 made it
   load-bearing, and it contradicts `28H:rul-fixture-records-enriched-not-reduced`. It would also
   permanently foreclose honest `records-*` cases, which need the real refusal path.
2. **Make `WidthOneAttemptScope`'s fields public (or add `WidthOneAttemptScope::new` as `pub`)** —
   REJECTED: a public field-wise constructor IS a payload minting a scope, and the type's own
   doc-comment ("Payload records never construct or refresh this scope") is the rule it would break.
   The fixture entry point exists precisely so the scope type never needs a public constructor.
3. **`#[cfg(test)]`-gate the exposure** — REJECTED on two counts: `dorc-loom compile/promote` is a
   real non-test binary, so a test gate cannot reach it at all; and `rul-fixture-identity-never-
   production` already rules that "environment presence alone never grants parser authority", which
   is the same shape of fence.
4. **Re-implement the intake inside `dorc-loom`** — REJECTED: a second implementation of admission is
   the exact dishonesty `289:rul-worldless-route-honest-trigger` exists to stop, and the case would
   pin dorc-loom's parser rather than the product's.
5. **Add a "harness mode" to `Framing`/`admit_records`** — REJECTED: `rul-authority-map` forbids a
   permissive default or compatibility fallback on this surface without opaque review, and a mode
   flag on the admission path is precisely that.
6. **Widen `Framing::spike` into a general scope factory** (`Framing::spike_with_host(...)`) —
   REJECTED: it re-opens the `RemoteIdentity` fence by giving a fixture caller a way to name a host.
7. **Drive the real binary from the looms runner, as the e2e runner does** — NOT rejected as wrong,
   but it does not answer the question: the looms runner is in-process BY DESIGN
   (`one-fixpoint-authority-per-case` hands executed cases to `test:e2e`), and an editable transcript
   has to come from the in-process consumer's stamped part stream. See §5.

## §5 — If the re-entry reading goes the other way

If the conductor reads `rul-attribution-is-controller-minted`'s re-entry trigger as fired by a second
controller (not just a second scope), the honest fallback is NOT to build a weaker version of this.
It is: leave the ~35 measured/survival prose-components on the loomability ledger as
**blocked-on: controller-scope re-entry**, cite the rule, and let the round that turns "carrying the
scope" into "checking the scope" unblock them. That is a ledger row with a law citation, which the
arc's own `dir-finish-the-loom-arc` explicitly allows ("or on an explicit law-cited never-looms
ledger").

## §6 — Size, and what it is worth

Code motion of ~250 lines from `main.rs` to `cli/src/lib.rs` (`SiteResults` and its two helper types
are the bulk), one ~30-line constructor, one lexical caller gate, one arm in the harness's plan route,
and one optional parameter on `WhyWorld::analyze`. No new mechanism, no new format, no new law. The
sequencing risk is that it collides with X2a's `world.rs`, so it wants to land AFTER the X2a fold, in
the lane that owns the why surface.

Worth flagging for that lane: the same seam is what makes the eight ratcheted `records-*` codes
honestly fireable, IF the conductor's separate `rul-no-emitter-codes-are-blocked-rows` question
resolves toward keeping the `262` records lane. Their emitter today is `records::deframe`, which is
`#[cfg(test)]`-only — a different blocker, on the same wire.
