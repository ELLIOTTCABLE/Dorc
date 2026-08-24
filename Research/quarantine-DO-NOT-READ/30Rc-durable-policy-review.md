# 30Rc - Durable policy review: first human-acked directions

> Tier: quarantined, living review of `plans/30R` and `quarantine/30Ra`.
> These directions are human-acked but deliberately softer than most Dorc
> rulings. Security policy remains comparatively malleable as research,
> implementation contact, and product experience improve. This is not an
> implementation plan. It will grow before an implementation-planning conductor
> reconciles it into an updated `30R`/`30Ra` pair.

## review-delta-from-prior-design

This document does not replace `30Ra`. It records the corrections and narrowed
directions that survived the subsequent research synthesis and human review.

| `30Ra` position | current review delta | status |
|---|---|---|
| One immutable whylog per invocation | The history is an event graph: one plan receipt may lead to many apply intents, each with an optional apply outcome. A successful plan-to-apply lineage normally has three documents. | ACKED, SOFT |
| One receipt contains the invocation's history | Plan reasoning, apply authorization, and apply outcome exist at different epistemic and durability boundaries and remain separate top-level types. | ACKED, SOFT |
| Durable publication failure is visible but not phase-ruled | Required pre-dispatch publication refuses by default; durable-only failure after first mutation dispatch does not abort coherent orchestration. | ACKED, SOFT |
| One readable envelope, format open | Direct readability remains a strong goal. The later binary-CBOR selection did not supersede it. | ACKED, SOFT |
| Rich/plain are separate projections | Preserve. Projection and disclosure policy may vary per document; the physical grammar leans singular. | ACKED, SOFT |
| Host influence is carried durably | Preserve, but influence tracking is a larger upstream kernel concern. The durable projects and rehydrates report-only accounts; it does not own influence semantics. | ACKED, SOFT |
| Explicit-only cleanup | Still open. Pre-dispatch gating makes unbounded growth an availability concern; no retention policy is selected here. | REVIEW |

## threat-model-narrows-principals

**[ACKED, SOFT]** The primary confidentiality threat is an unintended recipient
obtaining whylog files, filenames, listings, backups, or support attachments without
also controlling the controller process, source tree, operator account, or applicable
decryption keys. The useful question is which artifact and key material escape
together: field encryption helps a Slack attachment whose key stays home and may help
little when a full profile backup carries both receipt and key.

Same-user malicious code, a compromised operator account, controller root, and an
attacker holding both receipt and key are outside the whylog confidentiality and
historical-integrity claim. They can read inputs before encryption, invoke the
decryptor, replace Dorc, or spend fleet credentials directly. Do not build local
cryptographic machinery that implies otherwise.

Two separate in-scope surfaces remain:

- a managed host may shape records, omissions, response timing, and later narration,
  but never controller-owned target/attempt/generation/source attribution; and
- a whylog may be malformed, damaged, version-skewed, sync-conflicted, or supplied by
  an untrusted party, so reading remains bounded, sink-encoded, and report-only.

Other local users remain in scope at the ordinary OS-isolation level. Crashes, full
disks, concurrent runs, and sync clients are reliability failures whether or not an
attacker caused them.

## influence-tracking-stays-upstream

**[ACKED, SOFT]** Do not let the durable review absorb influence tracking. Influence
is a cross-cutting analysis over host-reported data and the analyzer's own control flow;
it reaches the pure kernel and stable semantic types. Its invariants are larger, more
distributed, and more project-specific than the whylog's.

The durable has only the overlap obligations: project recorded influence accounts without
laundering it, preserve missing/unknown distinctly at the conservative report posture,
continue influence through projection/render decisions, and never let a rehydrated
account become authority. Encryption faithfully preserving influenced material does not
make the material trustworthy.

## receipt-policy-is-user-configurable

**[ACKED, SOFT]** Receipt durability is controller policy, not one universal
posture. Dorc serves users ranging from time-poor homelabbers to teams using it as
glue among more battle-hardened systems. They legitimately differ on whether a
durable failure should withhold mutation.

The default leans secure and demanding. Dorc's product already earns the right to
say "no" by pairing refusal with one concrete way forward. Interactive use is the
primary workflow; a user who just typed `dorc apply` can normally investigate an
immediate pre-network refusal. CI, cron, and other unattended operation are
secondary modes whose better-resourced users can configure deliberately.

This does not imply that relaxed durability is the choice of stricter teams.
Teams running unattended or organizationally controlled applies may be especially
likely to require a crash-surviving pre-mutation receipt.

## pre-dispatch-receipt-gates-by-default

**[ACKED, SOFT]** Under the default policy, Dorc publishes the exact applicable
decision receipt before crossing the first mutative dispatch boundary. An
immediately recognizable failure that would prevent a sane, coherent later
`dorc why` refuses before mutation and names the configuration or override that
permits proceeding.

A recent successful plan is useful evidence that the controller filesystem was
writable, but never substitutes for the apply-side publication of the exact
decision, artifact, target, context, and invocation identity.

## mutation-dispatch-flips-durable-failure

**[ACKED, SOFT]** The policy transition is one coherent event: after tunnel
standup and successful required receipt publication, immediately before Dorc
dispatches the first potentially mutative book command. Tunnel standup remains in
the fail-fast region. Once Dorc commits to dispatching command identity one, the
durability/debugging failure direction reverses.

After that boundary, a controller-side durable failure alone does not abort an
otherwise coherent apply. The operation may already be partial, the user may no
longer be present, and stopping cannot restore the missing history. Dorc continues
or aborts according to execution, transport, attribution, and orchestration
integrity, not according to whether later whylog material can still be persisted.

Crossing the boundary does not claim that the first command reached or mutated the
host. It records that Dorc spent the authority to dispatch it; remote outcome may
remain unknown.

## plan-and-apply-form-event-graph

**[ACKED, SOFT]** The plan receipt is the primary whylog. It owns host probing,
analysis, vouches, survival reasoning, dispositions, render decisions, and the exact
identity of the emitted plan. Almost none of that information is newly learned between
`dorc apply` invocation and first mutation dispatch.

The apply side contributes two different records:

```text
                              ApplyIntent A1 -> ApplyOutcome O1
                             /
PlanReceipt P -------------- ApplyIntent A2 -> no outcome
                             \
                              ApplyIntent A3 -> ApplyOutcome O3
```

One plan may be edited and applied repeatedly, to different targets or under different
policies. Plan and apply are not one transaction and do not form a fixed one-to-one-to-one
trio. Correlation is narration over immutable records, never continuity of authority.

The reader preserves these states honestly: plan-only; plan plus committed apply intent
with unknown outcome; and plan plus intent plus recorded outcome. Missing predecessors or
successors are incompleteness, never synthesized history.

## apply-intent-owns-authorized-bytes

**[ACKED, SOFT]** `ApplyIntent` is the narrow pre-mutation sliver: the exact plan the
admin ultimately authorized Dorc to dispatch, its relationship to the originating plan,
the apply-time invocation/policy/target/context, tunnel/session identity established at
standup, and the first-mutation dispatch commitment.

It is small semantically but may not be small physically. A digest identifies bytes that
remain available; it does not recover edited stdin or a deleted artifact. Preserving the
admin's action without loss therefore requires either the exact applied artifact by value
or an exact patch against plan bytes that `PlanReceipt` itself retains by value. An
informal line-oriented diff is not enough for arbitrary added, removed, multipart, or
otherwise remapped shell.

The applied bytes are opaque-value-capable and may contain everything the authored book
contains. The narrow record can therefore require rich encrypted storage despite carrying
little new analysis.

## apply-outcome-covers-graceful-termination

**[ACKED, SOFT]** `ApplyOutcome` is attempted on every graceful terminal state, not
only success: complete success, command failure, detected transport failure, integrity
abort, cancellation, partial multi-target completion, guard divergence, and unknown
remote outcome. It owns actual per-site execution and what the host said, never the
plan-time reason a site existed.

Controller crash, process kill, power loss, or an unusable post-dispatch durable sink may
leave no outcome. Absence then means exactly "no outcome was durably recorded." It does
not mean success, failure, or no mutation. `ApplyIntent` proves only that Dorc committed
to dispatching mutation; it cannot prove what reached the host.

## append-buys-partial-crash-recovery

**[REVIEW]** Do not reopen append merely to reduce three filenames to one. Separate
immutable records correspond to real temporal boundaries and naturally support one plan
feeding many apply attempts.

Append becomes valuable only if Dorc decides to preserve partial per-command outcomes
across controller crash. That is a distinct incremental-journal architecture: atomic
append, synchronization cadence, intentional valid prefixes, encrypted continuation,
torn-tail recovery, concurrent readers, and finalization. Without it, a crash may lose
in-memory outcomes for commands that completed; the durable intent and unknown remote
state remain. Record that forfeited value rather than smuggling in a mutable log for file
count aesthetics.

## convenience-profile-expands-to-closed-defaults

**[ACKED, SOFT]** Dorc may provide a deliberately easy global escape hatch for a
low-risk, accepting user. Strawman spelling: `dorc --leave-me-alone`. It is a
configuration action in its own right and is not attachable to another command.

The action expands once into a closed, predetermined set of ordinary option
values. It immediately and synchronously prints every changed option, the cost the
user accepted, and the command that reverses that individual change. A common
intended reaction is to keep the convenience profile while manually walking back
one or more changes.

Future options remain at their unconfigured high-security defaults. They never
silently join an earlier convenience-profile expansion. The user must rerun the
profile or configure the new option individually. The effective individual
policies, not a generic "security off" bit, enter decision identity and durable
explanation.

Illustrative user-story wording supplied during the ruling, not settled prose:

```text
dorc just configured your system to show more plaintext in permanent, on-disk
logs. Reverse this one with `dorc --set durables=cbor`.
```

The example's option names and format remain strawman. The stable direction is
Show Our Work: convenience is allowed, hidden convenience is not.

## tty-presence-never-means-availability

**[ACKED]** TTY presence is an explicit Dorc mode signal for an active terminal
and the Unix pipe workflow. It does not imply that the user will remain available
during probing or apply, and it never grants consent to weaker security or
durability policy.

## direct-readable-structure-remains-goal

**[ACKED, SOFT]** Preserve the strong lean toward a plaintext-like, directly
inspectable structural envelope for as much material as can be justified. The UX
benefits in firefighting, old-version inspection, quoting, diffing, and external
debug handoff are load-bearing product value and are not sold away merely because
a binary candidate has a stronger parser story.

Readability and disclosure policy may differ across users and fields. That does not
yet justify multiple physical durable grammars.

## one-canonical-grammar-unless-intractable

**[ACKED, SOFT]** Lean toward one canonical physical durable grammar. Rich versus
plain projection, filename disclosure, field retention, and receipt policy are
configurable and recorded per document; the underlying format is not presently a
user configuration surface. Multiple canonical formats would multiply hostile
parsers, damage models, compatibility obligations, and cryptographic joins.

Binary is not automatically the secure pole and text is not automatically the
insecure pole. The canonical grammar question remains open to implementation and
focused research. If one readable grammar proves intractable, format plurality may
be reopened explicitly rather than pre-paid now.

## binary-selection-did-not-close-fork

**[REVIEW]** The later recommendation of deterministic CBOR plus a tool-rendered
diagnostic form changes direct readability into tool-mediated inspectability. A magic
prefix visible in a hexdump identifies a format; it does not make the receipt readable.
That recommendation is useful evidence for the binary counter-thesis, not a ruling.

The physical format is not itself a secure-to-insecure configuration ladder. A strict
small text grammar may be safer than a permissive binary decoder; deterministic binary
may eliminate parser differentials while requiring a compatible Dorc to inspect the
artifact. Secure defaults should select receipt gating, rich/plain projection, field
disclosure, and filename policy without assuming that "secure" means CBOR.

## research-stack-remains-unselected

**[REVIEW]** No parser, format, cryptographic construction, or storage crate is selected.
In particular, the proposed CBOR plus C2SP integration did not specify one coherent
encryption topology: one document salt, per-record context, and independently encrypted
record units do not become one C2SP construction without additional key derivation or a
separate skeleton/blob binding. The claimed authored-seam count is therefore unproven.

The pending readable-envelope and publication-contract fronts exist to answer these
questions before an implementation plan selects dependencies. Exact cryptographic review
waits for the envelope to choose per-record, grouped, or document-blob encryption.

## policy-changes-never-rewrite-history

**[ACKED, SOFT]** Every durable records the exact projection and policy under which
it was written. Later configuration changes apply only to future invocations. They
never rewrite, downgrade, upgrade, or reinterpret historical durable contents.

## planning-handoff-updates-prior-plan

The implementation-planning conductor should update `30R`, not treat this review as a
second implementation plan. The update should, at minimum:

- replace one-whylog-per-invocation with the plan-receipt/apply-intent/apply-outcome
  event graph;
- give each record its own publication moment, content purpose, and failure policy;
- bind `ApplyIntent` to exact admin-authorized executable bytes and its originating
  `PlanReceipt`;
- make outcome absence and missing graph edges explicit report states;
- place the durable failure-direction transition at first mutation dispatch;
- preserve configurable receipt policy, the closed convenience-profile expansion,
  and non-retroactive per-document policy;
- retain direct readability and one canonical grammar as strong soft directions; and
- keep influence semantics outside the durable subsystem.

## review-residue-remains-open

This review has not selected names, a text grammar, an encryption topology, key
custody, retention defaults, or the exact pre-dispatch publication artifact. It has
only narrowed the policy shape those later decisions must satisfy.
