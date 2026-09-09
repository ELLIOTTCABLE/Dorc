# ROADMAP — the direction: what is scheduled, and what is owed

> human: ON HOLD: This document spawned design-sittings that have substantially
> reworked the core language. This will need to be completely rewritten after the
> 311+-series model-redesign stabilizes. /=

> AI-authored, AI-maintained. The human moves rows between sections by *typing* an ack in
> chat; nothing else does. Siblings: `TODO.md` (the human's own list, their voice) ·
> `TODO-ADDTL.md` (the porch — what models think the human should look at, unacked, owed by
> nobody) · `Research/README.md` (the map of documents) · `SLUGS.md` (grep it for any slug).
>
> **This file and `Research/LIVING_STATUS.md` are mutually exclusive.** Nothing here is in
> flight: the moment a conductor is handed a piece and told to build it, that piece MOVES to
> LIVING_STATUS, and comes back only if it is dropped unbuilt. Work split on purpose ("build
> X1 now, X2 later") splits the same way: X1 moves, X2 stays. Contents are
> project-direction-level only; a narrow, code-level item belongs here only when it
> critically affects direction, which is rare. Arc ceremonies, steering edits, register debt,
> prose queues, and small held rulings never do — they live beside the arcs and records that
> own them.
>
> **Scheduled** holds what the human has typed "next" on: one to four chunks, almost always
> exactly one — promises rot if not dispatched within days. **Owed** holds work the human has
> acked as real but not scheduled; each row points at its design-of-record and says what
> gates it. When a later design changes a row's shape, the row gets a one-line pointer to that
> design, never a paraphrase. When a row lands, delete it (the account moves to the README
> round-map); this file is never a work-log. `rNN:` xfail horizons are mechanical reminders
> of *when to be bothered by the machinery*, never promises; a row here may hold a pin's
> intent well enough to retire the pin, and no new horizon is minted to mirror a row.

## Scheduled

### `round-r31-language-and-kernel` — announced 2026-09-02; cut 2026-09-02; test-rebuild prerequisite satisfied (`notes/30Xa` §0)

The second language/kernel round: the r30 designs-of-record built, chosen so a pivot book
(one file: stand the machine up from the controller, then converge it over `ssh`) analyzes
end to end in the plain-argv form `ssh host cmd args`. The build shape — lanes, order,
merge points, the two cut-seams — is **`plans/310`**; this row carries only direction. The
design sitting that produced the pivot algebra is banked in `notes/26M`; its design-of-record
is `plans/30W`.

> This is still the old build cut. `notes/26Ob` §15 directionally acks transport out and
> some single-shot capture in; re-cutting this row and `310` waits for design stabilization.

In, by lane (`310` §1):

- **tracers and records** — the taught decline idiom (`USER_STORY` stage 3's shape, a tracer
  bug) · `local` off the `26J` deny-list (the dialect is POSIX + `local`) · `plans/30D`'s
  prediction contract (RULED, unbuilt: literal model rc; whole decline requires DREP plus
  actual rc 2; authored stream checkpoints and separate integrity) · `plans/30U`'s
  `disturbs nothing-else` recognition and the `disturbance_reaches` respell · `30T:comp-backing-detector`. This lane is the gate before
  any real oracle is hand-authored (`notes/27Q` §2).
- **edge: local-exec and the load plane** — local-exec as a supported mode (no host means
  the controller; `TODO.md`'s cli-refresh note is the spelling authority; the `LocalDriver`
  becomes a product driver once `30X` retires its harness pin) · `30T:comp-artifact-injectivity`
  · the load-model builds, each waiting on its ruling (below).
- **kernel: the index re-key** — `plans/30S` whole (value-carried context keying, the export
  fence, pin-or-sever; the four `r31` pins) · `plans/30W` item 1 (the context slot becomes a
  product over index-kinds — retrofit-hostile, audited first) · items 2–4 with Host minimal
  (an entered index; no merging; the re-keying rule and expected-sever; the Boot/Machine
  witness on `30S` rails) · `plans/30U`'s `unrelated` answer and settle gate · the
  invariance line accepting an index-kind target. The engine-side `ssh` charset carve is
  WITHDRAWN (`26Ob:ack-charset-is-tool-knowledge`); wrapper-as-landing remains unruled.
- **file semantics** — `plans/30T` §10's chain: `comp-routing-locator` → `comp-fs-binder-member`
  → `comp-claim-consumption`. The second seam: the round is coherent without it.

Held at seams (`310` §3): **`seam-payload-forms`** — the heredoc form and the sibling-file
form both reduce to the payload-declaration speech-act (`notes/26M`); nothing builds before
its carrier survey (now ACKED, `26Ob:ack-stdin-survey`); the seam remains held
(`notes/26Ob` §15), with analysis and manipulation landing together.
**`seam-survival-last-mile`** — the file-semantics lane; cut here if the arc must shorten.
Survival as a mechanism stays IN (the human's `26M` core argument: purpose-ordered pivot
books lock their value behind it; the `26Lb` cohort lean covers deep vendor-described
survival only).

Rulings that gate lanes (the human's; each blocks the lane it names and nothing else):

- the six `plans/30W` §10 rulings — gate the kernel lane below `30S`;
  `rule-unreserve-host-as-entered-index` collides with `271:rul-axis-vocabulary-v1`, and
  `rule-incarnation-invariance-passes-the-razor` decides whether boot-invariance is
  vouch-tier or survival-tier. **Partly superseded:** current shape questions and remaining
  proposals are `notes/311a` §§2–5; these are not six untouched rulings.
- `28Q:pin-ssh-entry-shape` — the verbatim-only premise is RETIRED
  (`26Ob:ack-entry-verbatim-cannot-hold`); wrapper-as-landing is still unruled.
  Host entry moves out under the directionally acked re-cut (`notes/26Ob` §15).
- the delivery-mechanics pair from `26M`'s seam map, gating Host entry AND local-exec's use
  for pivot books: `26K:sit-stdin-copy-exec-amendment` is RULED
  (`26N:rul-delivery-shape-file-backed-default`; its build is lane 2's `unit-delivery-shape`,
  which lands before Host entry) · the report sink's remote home is DISSOLVED by
  `plans/26O` (every context Dorc stands up carries its own sink and scaffold,
  `26O:rul-every-context-carries-its-own-scaffold`; records never cross a link as records);
  the former remaining Host-entry gate was `28Q:pin-ssh-entry-shape` (updated above).
  Still unruled: the sinkless-context record policy (`26N` §4.5, the human's lean;
  `26O:rul-sinkless-context-record-policy`), whose conservative reading builds without
  waiting. **Scope correction:** sinkless policy is transport-round work, not an r31
  launch gate under `notes/26Ob` §15; required prediction-confirmation loss already
  withholds under `plans/30D` §5.
- the load-model trio — `tc-dollar-zero-is-script-anchored` is RULED yes
  (`26N:rul-dollar-zero-authority-spelling`; the flagship red
  `load30-point-havoc-and-script-relative` targets guard-at-most,
  `26N:fnd-blind-act-fixture-is-guard-at-most`) · `tc-hoisted-dot-line-spelling` +
  `tc-t2-is-narrower-than-the-ladder-says` (`30Ng` §7's hoist ladder) ·
  `30I:pin-command-v-load-model` (evidence `notes/30Ic`) — the unruled two gate their own
  small builds only.
- the acquired-source weld — `30R`'s ordered role-carrying source vector against `30I`'s
  occurrence account; a small ruling, gates one small unit.
- `26M:q-payload-declaration-speech-act` — survey prerequisite SATISFIED
  (`26Ob:ack-stdin-survey`); `seam-payload-forms` remains held, not a launch gate.
  Holes cardinal if missed: payload identity joining the `26C` Question identity; expansion-siting across the boundary; entry recipes
  carrying transport flags or declining.
- the filesystem binder member's NAME (unminted; `30T` §3.2) — a provisional mint suffices
  under `rul-strawman-formats-no-compat`, before the brief.

Sittings with no build behind them in r31: `30Ng:attn-render-refusal-feeds-the-spine`
(render refusal as a minted decision cycling to fixpoint; wants a termination argument;
hardens as artifact forms accrete) · `26K:sit-wall-transparent-delay-loops` (ruled in
substance by `26Lb:rul-wait-scope-is-just-shell-modeling` and `28Q` §3's corollary: no
bespoke machinery; a pure body under a modeled `sleep` neither elides nor walls; the
`StatusIterated` converged-at-entry carve stays a license-widening nobody is building).

## Owed, unscheduled

Human-acked as real; no date. Arcs first, then the sittings the human owes.

- **`arc-block-stdlib`** — zero non-fixture oracles; human-ruled 2026-07-27 as not blocking
  (scrappy hand-written oracles are part of the experiment). Gates, all engineering: r31's
  tracers-and-records lane · `30S:seq-stdlib-gates-on-env-identity` (r31) · the `27Q`
  preconditions; the dialect-reach gate is RULED (`30Q` §5g). Pivot priority: transit verbs
  first (`26K` concl-boot-books), then `ssh`, `sleep`, timeout · ssh-keygen · curl · getent.
  The index-kind stdlib (Boot / Machine / LoginSession reads; the Host binder beyond the
  minimal one) and the fs stdlib file ride here. On-ramp `notes/27Q`; authoring trap `24U` §2
  (converged ≠ no-op adequacy).
- **`arc-family-vocabulary-qualification`** — `plans/30J`'s P2 (a genuine `family__predict`
  qualifies the family's selector vocabulary; per-speaker dialect keyed at the backing's
  frame; the counterfactual-denial aid). Design RULED; ruled OUT of r31 against `30J` §10's
  trial clause, reasoning at `310` §4d. Trigger: the stdlib revival, a survival-authoring
  trial that evaluates selector-granular sparing, or third-party publication. Still owed
  inside it: `30J` §6.5 cross-family registration (the human's).
- **`arc-payload-declaration`** — the heredoc and sibling-file remote forms; the one
  engine core is `notes/26M`'s speech-act (carrier geometry · dialect and fidelity as an
  authored pre-applied normalizer · execution world · custody). Carrier survey ACKED
  (`26Ob:ack-stdin-survey`); the seam stays held (`notes/26Ob` §15). The sibling-file form
  pulls `FORFEITS:forfeit-plain-sh-inclusion-analysis` in with it (the splice and paste tiers) and `30I`/`30P`'s bundling; the heredoc form forces
  the first source-to-source questions inside argv/heredoc bytes — `26M`'s v1 posture is
  whole-or-nothing render for inline payloads.
- **`arc-host-capabilities`** — the capability SYSTEM of `notes/26N` §4: measured,
  context-keyed supply; a derived per-chunk requirement census; the join at the standups
  under `26N:rul-preflight-over-probe-time`; never a plan line, never a license. Its census
  question is OPEN (`26N:open-census-needs-value-plane`, the next sitting); streams are its
  first non-boolean demand, and `26O:5-the-routing-planner` consumes the join. Builds with
  `arc-payload-declaration`, when demand first becomes non-empty; r31 only reserves the seat
  (`310:unit-context-slot-product`). Recording measured supply in the receipt clears
  `rul-durable-contents-reviewed-before-design` first.
- **`arc-identity-tier`** — `30T:comp-identity-tier` (per-aspect relations, perishable
  answers; the region predicate was brought forward into the kernel by
  `311a:rul-overlaps-is-a-kernel-generator`; its name and rc contract are now RULED as
  `kind__disjoint` by `30W:rul-disjoint-is-an-rc-predicate`) · the
  referent-transparent declaration · `30W` item 6 (store-member decomposition, gated on `rule-only-decomposes-everywhere`) ·
  `30T:comp-channel-relative-speech` · `30T:comp-content-establishment` (the write-if-changed
  idiom's own elision; the FORFEITS captures). None is ruled beyond `proposal`; shares
  plumbing with `30P:mech-two-standups`.
- **`arc-r26-revival`** (reactive/capture + multi-host) — `26B`/`26C` + `260`–`262`; after
  r31. Entry gates: the un-ruled R2-entry question (freeze-in-artifact vs
  structure-preserving folds) · `26C` §1's stability confirmation · a FRESH `26C` §7
  quiet-welding audit against the current kernel before R0 (`26C` verified the r27 tip only;
  NB `30L`'s iteration axis = loop-member, `26C`'s `iter=` = probe-iteration) ·
  `26B:need-quiescence-witness-at-mint` · `26B:split-semantic-versus-concurrency-holes` ·
  `26B:ask-trial-counts-capture-walls` (size from real `dorc why` critique first). Capture
  starves until `30D` stdout claims exist (r31). The epoch×capture boundary
  (`26L:finding-future-values-remain-future`) rides here; its single-shot half is r31's.
  `notes/26L`/`26Lb` are exploration-tier — cite no commitments from them.
- **`28Q:stage-iii-world-scopes`** — `plans/30W` is the design-of-record for the index half
  (Host, transits, re-keying); what remains is the begin/end authored surface (`28Q` §10,
  RESERVED), gated on `design-world-scope-surface`, `rule-incarnation-continuity-semantics`
  (below), and the ssh oracle (⇐ stdlib).
- **`arc-why-surface-deferred`** — `notes/30V` §6: curated tiers/registers/ranking
  (post-user), the staged-search CLI, `dorc apply --why`, `--live` re-probe, site-granularity
  (held on the frozen kernel), the graph-drawing library, the `dorc r` router, the record
  register (resolves against real output under a human eye, never more strawmen).
  Report-only kernel re-derivation is its own authorized round (`30R`).
- **`arc-durable-account-export`** — the per-row influence export is built and DISABLED;
  enabling it against the receipt durable is a receipt-contents change that clears
  `rul-durable-contents-reviewed-before-design` first (`30Rk:the-account-export-died-with-its-lane`).
- **`design-mh2-version-layer`** — `.claude/research/versioning-mh2/`, `270` §4.

  > on MH2: versioning (the simpler version of the concept, focused down to 'package-as-a-type-needing-special-attention, multi-providers, etc' ... and 'mapping oracle-written-for vs being-executed-on') needs close care, but it feels very deferrable compared to some of the critical core analyzer-design things that are affected by the "sh spelling" issues. continuing to defer.

- **`lane-flux-engine-hardening`** — UNSCHEDULED (human 2026-08-21); `300:lane-flux-engine-hardening`;
  any typesystem change it needs rides the Aeneas-prep facade work, never a Flux lane.
- **`rc-vs-genkill-permanent-law`** — revisit, not now, never strike (human 2026-08-23); the
  `spike/CLAUDE.md` clarifier stands until the wider law lands (the influence-carriage lane
  does not produce it — `30Qd`).
- **`30L:pin-book-argv-value-plane`** — book positionals (`main "$@"`) read ⊤ in body flow;
  admitting book argv to the static value plane is a winner-shifting licensure widening
  that wants its own ruling. The rest of `30L` landed in r30.
- **FORFEITS captures with a stated trigger or an `r31` attention-call** —
  `forfeit-certifier-trip-evicts-elisions` (trips observed in the field) ·
  `forfeit-book-dynamic-load-analysis` (glob loads; `r31:book-load-acceptance`) · the
  four rows at `r31:kernel-punt-glance` (`forfeit-value-narrowing-by-test` ·
  `forfeit-file-content-facts-from-exact-checks` ·
  `forfeit-content-establishment-by-known-write` · `forfeit-shell-parity-immunity-model`).
  Every row carries its reds; `mise run xfail:census` lists the pins. None is in r31; the
  horizons stay attention-calls.

Sittings the human owes (direction-level only; the small held rulings stay with their
records — `30O:human-gated-rulings`, `notes/30Va`, `27U` §7). The r31 gates are listed
under the scheduled row; these are the rest:

- `design-world-scope-surface` — how authors describe context begin/end, transit respelling,
  ssh re-parsing, fan-out consent; on-ramp `28Q` §§3/8/10 + pin 11, `27C`, `24T`. `30W`
  answers the index half; the authored begin/end surface remains.
- `rule-incarnation-continuity-semantics` — when a recreated context is continuous with its
  predecessor (`28Q:res-incarnation-correlation-door`); `28Q` §3 + pin 4. Not needed by a
  create-if-absent pivot book; needed by any destroy-recreate one.
- the pivot-book cross-host-influence security review (opaque-review tier; OWED per `26M`
  `ack-cross-host-facts-scoping`, out of scope for the design work; needs the human's typed
  ack to dispatch).
- `kty-annot-punt` — the spike itself IS the kTYANNOT experiment (human 2026-07-12); "is
  not-using-EOL-comments livable?" is post-spike adjudication input, never upfront work.
