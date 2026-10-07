# 315 — What the corpus holds beside the identity model: positions, supersessions, names, standing rulings

> AI-authored (Fable, 2026-10-05, at the human's direction, from an autonomous run of
> 2026-10-04/05). Notes-tier extraction: it rules nothing, recommends nothing, and proposes no
> name. It is the companion of `notes/314`, which holds what that run established about ordering
> and praxis. This note holds what the run found IN the corpus: where two or more positions
> stand on one matter, what the identity model (`311`) supersedes with and without saying so,
> which names have an ack, which typed rulings nothing later reverses, and what the mechanised
> specification and the built engine each chose.
>
> Provenance. Five scouts read the corpus, a synthesiser merged them, and a drafter checked
> 274 quotes against their sources by script. That checked extraction is on the unfolded branch
> `ai/r31-identity-order-conductor` and is mostly not kept. This note is written from it without
> re-opening the sources. A quote in “curly marks” was carried from the checked extraction or
> from the conductor's own read of the passage. An entry marked `[extracted]` rests on a scout's
> reading and carries no checked quote; open its source before leaning on it. Two errors a
> reviewer found in the extraction are corrected in place and marked. Line numbers are against
> `ai/main` on 2026-10-04. `pre:N` is line N of `Research/notes/311-identity-and-relation-model.md`
> at `4b305416` (the pre-mechanisation text); `spec:N` is line N of
> `specs/311-identity.assay.md`.

## § 0-reading-a-status

A status is never upgraded here.

- `voice` — README, DESIGN, IMPLEMENTATION, TODO: the human's own text.
- `audited` — USER_STORY and KNOBS: AI-written, human-audited.
- `typed` — a ledger's [TYPED]. In `30Ya`, [HUMAN] means typed, by that ledger's header. In
  `313e`, [TYPED] marks that the human typed it and the wording is the conductor's.
- `acked` — a ledger's ACKED or "ack", with the scope it states.
- `[HUMAN]` — in `311q`, `311t`, `312a`, `312b`, `312cg`, `312d`, `312e` the header defines it as
  the human's framing, paraphrased from chat, never a ruling; a word beside it (typed, ack,
  STAMPED) is reproduced, not resolved. In `312f` only [TYPED] and [ACKED] rule, and [HUMAN] is
  undefined.
- `model` — AI text with no human mark. `conductor` — a ledger claim the human did not react
  to. `built` — spike code.
- `311a`, `311b`, `311c` to `311i`, `312a`, and `312b` were rewritten by vocabulary passes after
  the sittings they record (`[extracted]`: commits `fb8c4dd0`, `866bedbd`, `5318649b`,
  `19b1dd6a`, `81684236`, `adaaf990`, `a224d4e0`). They show acks in words that did not exist at
  the time; what was acked, in the words then in use, is in git. `312a`'s and `312b`'s "311 § N"
  references point into a first draft that was deleted (`Research/README.md` says so).

## § 1-matters-with-more-than-one-standing-position

Each: the matter in one sentence; positions as *holder · citation · status*; the latest
position the human typed; whether § 4.2 of the 311 texts registers the disagreement. Ordered by
what they decide.

### § 1.1 the flag and faultless wrongness

Whether Dorc can remove a line wrongly while every statement it relied on is true, and whether
the admin's `--risk-faultless-skips` flag is the price of that case.

- human · `IMPLEMENTATION.md:357-358` · voice: “soundly avoiding #2 in 100% of cases is
  effectively impossible”
- human · `IMPLEMENTATION.md:394-398` · voice, the passage after it sitting in a section headed
  "UNFINISHED": “nobody did something locally incorrect; but the net effect of several people's
  choices was incorrect elision for the end-user”
- human, recorded by a conductor · `23D:90-91` · r23, "the human's catch": “two honest names,
  one referent; falsehood emergent from the pair, attribution target-less”
- human · `271:662`, `271:694` · 2026-07-12 · typed: “Claims own what lines can say; the flag
  owns what no line can say.”; “when it bites, no single human was at fault”
- human or conductor · `24A:245-246` · 2026-07-03 · a clause of a paragraph headed "human-typed"
  (`24A:232`), which may be the conductor's: “mismatch without a lie = engine unsoundness, always
  red; mismatch with a lie = priced residue”
- USER_STORY · `USER_STORY.md:713-714` · audited: “That clean speech was wrong in the one way no
  machine can see: complete-looking but semantically incomplete.”
- 311 · `pre:59-61`, normative at `spec:88` · model: “A wrong answer with no false statement
  behind it refutes the model.”
- human · `312cg:169-171`, `312cg:208` · 2026-09-24 · typed: “It does NOT qualify where … a
  causor can theoretically be named who was enabled to make the correct speech and did not”;
  “every negative existential requires the flag”
- conductor · `312d:1210-1211` · 2026-09-29 · a defence in a turn that closes “otherwise ack”
  (`312d:1261`, typed) without saying which claims that covers: “no skip is faultless: every
  survival rests on a named negative existential”
- human · `312d:1247-1256` · 2026-09-29 · typed: “the flag prices FAULT …
  `attributions.filter(within contracted knowability class)`, ish”
- human · `312f:49-52` · 2026-09-30 · typed: “intentionally admits residue that is epistemically
  unknowable … Every consuming admin explicitly accepts it through the risk flag”
- human · `313e:22-24` · 2026-10-03/04 · typed, with stated uncertainty: “The flag is one boolean
  … That class rests on a negative that closes over an inherently unclosed world”
- Latest typed: `313e:22-24`. Registered: no.

### § 1.2 survival without the flag

Whether a removal may stand past a line that really ran when the admin did not pass the flag.

- KNOBS · `KNOBS.md:328` · audited: “survive — an elision kept past a RUNNING wall. License: the
  vouch PLUS footprint × backing disjointness PLUS the admin's `kSURVIVAL-trusted` flag”
- USER_STORY · `USER_STORY.md:480-481` · audited: “none of this is on by default”
- 311 · `pre:818` · model: “DISJOINT licenses sparing under the same flag”
- human · `312cg:208` · typed: “every negative existential requires the flag”
- human · `313e:24-27` · typed: “A thoroughly described toy world must reach elision past a line
  that ran without such a negative.”; “USER_STORY and `KNOBS:kSURVIVAL` describe that practice,
  and the model wants the closed sense.”
- conductor · `313e:40-41`: “Nothing recorded says a scoped negative is trusted without the flag”
- Open beneath it · `313a:81-84` · OPEN: whether a fact's dependence is itself a closure.
  `notes/314` § 2 records what a model showed turns on that.
- Latest typed: `313e:24-27`. Registered: only for the wrapper sentinel's SAME.

### § 1.3 the law's premise

Whether the soundness law assumes every statement in force true, or only those an answer used.

- 311 · `pre:60` · model: “while every statement behind it is true”
- spec · `spec:82-83` · model, commentary: “while every statement in force is true”; the reading
  is the mark `enc-support-functions` (`spec:3979`, an unacked reading)
- human · `312d:1237`, `312d:1285-1286` · typed: “"and we know which" is effectively the whole
  product”; “the conductor's construction is not acked”
- "In force" has no acked name, and what it means at a line is its own open finding
  (`312d:fnd-in-force-is-book-global`, `312d:1404-1408` `[extracted]`).
- Latest typed: `312d:1285-1286`. Registered: no.

### § 1.4 what a wrong elision is measured against

Whether a removal is wrong because the end state differs from running the book bare, or because
a line that ran touched what the removed line's fact depended on.

- human · `IMPLEMENTATION.md:162-163` · voice: “to mistakenly elide a command that was necessary to
  converge system-state”
- conductor · `24A:225`, `24A:240` · 2026-07-03 · a commission, then a clause of a paragraph
  headed "human-typed" that may be the conductor's: “S_bare == S_apply”; “end-state equality”
- engine · `spike/crates/sweep/src/lib.rs` · built: two host copies evolved from one start, the
  bare book and the plan, compared for end-state equality, with each command's ground-truth
  effect held apart from its declared claim
- USER_STORY · `USER_STORY.md:722-723` · audited: “the wall really touched the cell the claim said
  it didn't”
- spec · `spec:1313` · model, `sparingIsSound`: “no (World.lineWrites[l]).*affects & f.dependsOn”
- Corrected: the extraction listed `30Ya:446` (“the outcome is always the checked statement”) and
  `312d:1633-1634` here as typed positions. In `30Ya` "the outcome" is the plan's decision per
  line, not the host's end state, and neither line takes a side.
- No typed line takes a side. Registered: no.

### § 1.5 world behaviour inside the checked model

- human · `30Ya:449-450`, `30Ya:316-317`, `30Ya:161-162` · 2026-09-27 · typed: “a measurement is a
  claim; only abstract world facts exist outside claims”; “the split of literal measurement from
  claimed measurement is the preferred approach”; “the world's behaviour enters only at the very
  end, on a host”
- spec · `spec:154`, `spec:159-165` · model, normative then an unacked reading: “A write to an
  MReferent affects the MState of the MReferents it affects”; the world relations “are the
  conductor's choice of world stratum … 311 names none of them”
- human · `312d:814` · typed: “the world stratum has not been explained to the human”
- conductor · `312d:1166-1171`: the world stratum was “invented by the conductor in the raw pass
  so the laws would have one”. The human's corrections to that explanation are `312d` § 17.3,
  typed; the turn closes “otherwise ack” (`312d:1261`).
- No document found relates the three accounts of what is so apart from what is said: the
  built chronology net (§ 1.4, § 7), `30Ya`'s two strata, and the mechaniser's world relations.
  The two-ended table (§ 3) is a fourth, never re-cut into the model.
- Registered: no.

### § 1.6 measurement as a foundation of its own

- human · `271:526-528` · 2026-07-12 · typed doctrine: “"Measurement" as a category distinct from
  authorship is borderline misleading”
- human · `30Ya:449` · typed: “a measurement is a claim”
- human · `312d:1219-1220`, `312d:1229-1230` · typed: “Dorc DOES know things about the world by
  design”; “RESERVE a slot for someday engine-measurements”
- spec · `spec:103`, `spec:106` · model: of the third foundation “this document holds none”; “The
  engine knows only syntax, authored speech, and what authored probes returned.”
- Latest typed: `312d:1219-1233`. Registered: no.

### § 1.7 the engine "vouches"

- 311 · `pre:221`, `pre:367` · model: “The engine vouches for one lookup itself”
- human · `312d:1273-1274` · typed: “the engine is not "vouch"”; applied to the specification as
  `43b8d4dd`. The same passage calls vouch a precisely defined contract term.
- `311u:163` · model, unedited: “The engine vouches the local MRoute”
- Closed by the typed line; the refuted-shapes register still carries the old sentence.

### § 1.8 the read side's completeness burden

Whether what a probe read must be declared complete before a write can be excused from
disturbing that probe's fact.

- USER_STORY · `USER_STORY.md:467-468` · audited: “a fact's backing simply is what its probe reads”
- conductor · `23O:185` · r23: “so backing-completeness is a non-worry”
- stage-3 spec · `24D:164` · model, under "human review refinements": “carries no completeness
  burden”; built at `spike/crates/plan/src/survival.rs:470`
- `27C:148-149` · model: “A false input-completeness claim in consumption position fails as a
  silent wrong elision of someone else's line”
- human · `279f:91-92` · 2026-07-16 · "human-ruled", refusing an authored completeness act among
  the options put: “(d) measure in the site's own context”
- `30U:67` · model: “The gate is unary and attaches to the claimed (footprint) side only”
- human · `TODO.md:7` · voice: “"I'm converged in live state"” against “"this is a no-op in live
  state"”
- 311 and spec · `pre:560`, `spec:1174` · model: “A closed may-read set is knife-tier”; “The vouch
  is true when the measured answer depended on no MReferent outside what the marked reads reach.”
- human · `312cg:391-393` · 2026-09-24 · typed: “closure is necessary for the dangerous actions,
  cross-author survival and the like”
- conductor · `312f:558`: “The human distinguished this from the read side, about which they
  remain concerned.”
- Latest typed: `312cg:391-394`. Registered: no.

### § 1.9 first-order or total effects

Whether "this command writes at most these" covers what the command does, or every consequence,
chains and other parties' reactions included.

- human · `IMPLEMENTATION.md:273`, `:649-650` · voice: “reporting about hork's actual first-order
  footprint”; of a maintainer script, “Dorc cannot hope to attribute that”
- `238:47-48` · 2026-07-02 · model, header “human-acked for stamping”, its round parked: “ALL
  effect-claims (establishes, footprints, vouches) cover first-order tool-contract effects ONLY”
- human · `312b:22-23` · typed positions: “unexpected churn (other users, host-owned cron,
  host-configured reactions) is horizon, the price of play.”
- human · `312cg:551-552` · 2026-09-26 · typed, of a write list: “what will happen at apply time”.
  Its context rules against a probe-time file list standing in for an upgrade's writes; it does
  not say a claim covers every chain of effects.
- human · `312f:56-57` · typed: “a write described only as a write to an image can affect the
  whole image”
- 311 · `pre:603-604` · model: the entailment “carries every cross-MSort consequence no
  MFullyQualifiedKey expresses”
- spec · `spec:1308`, `spec:1313` · model: the finished record's truth bounds one step of
  `affects`; `sparingIsSound` reads every chain
- conductor · `312f:64`: “The human has not chosen a replacement effect algebra.”
- The older word "first-order" is not used in r31. Latest typed: `312f:56-57`. Registered: no.

### § 1.10 how far "and nothing else" reaches

- `30U:34` · spelling typed, content model: “and nothing else, in any vocabulary”
- USER_STORY · `USER_STORY.md:643-644` · audited: “including ones I have never heard of”
- human · `311b:341-342` · 2026-09-07 · hard ACK: avoid new enumerations of the kind “including
  things I have never heard of”
- human · `312b:191-193` · 2026-09-10 · ACKED, seen then with "mKinds'": “"nothing else" means
  "nothing outside my declared stores and my reached MSorts' stores"”
- human · `311q:673` · [HUMAN] lean: “`disturbs no-other:sm.File`”
- human · `312f:53-54` · typed: “Closing a list does not require proof of an unknowable universal.”
- Latest typed: `312f:53-54`. Registered: no.

### § 1.11 a finished definition: a word's meaning, or a claim about the world

- human · `30U:43-45` · 2026-08-29 · typed framing (`rul-definitions-not-surveys`): “The reaches
  body is part of the word's definition … never a measurement of the world.”
- spec · `spec:1308` · model: `true_FinishesEntailment` reads the world relation `affects`
- conductor · `312e:714`: “as mechanized, the knife is the may-read closure”
- Latest typed: `30U:43-45`. Registered: `30U` only as a generator.

### § 1.12 a bare key in a write list

- human · `311t:1561` · 2026-09-23 · [HUMAN], typed: “a bare key in an entry is deep”
- human · `312cg:546-548` · 2026-09-26 · typed: “A bare key in an entry names its referent.
  Reaching children takes an explicit mark … This supersedes the cross-root ack”
- The mark has no acked name; it was spelled `:*` in chat only.
- Latest typed: `312cg:546-548`, reversing the first.

### § 1.13 whether USER_STORY stage 5 survives

- USER_STORY · `USER_STORY.md:529` · audited: “all disjoint, all survive”
- human · `312b:195-196` · 2026-09-10 · ACKED: “USER_STORY stages 5 and 7 survive it unchanged
  (checked).”
- human · `312f:627-628` · 2026-10-01 · ACKED, "the problem statement only": “cannot survive,
  however precise the describers are”
- human · `312cg:1232-1233`, `312cg:1189-1190` · typed: “311 is allowed to result in zero value
  anywhere until the stdlib speaks”; “The core analysis is not weakened in default settings to
  recoup elision value.”
- Registered: no.

### § 1.14 to § 1.27, the identity half and the rest

- § 1.14 `the shape of a coordinate` — `271:rul-coordinate-shape-flat-three-place` (typed;
  `spike/AGENTS.md` coordinate-semantics: “recursive coordinate shapes DECLINED”) against the
  recursive MFullyQualifiedKey of `pre:309-313`, which `311q:27` and `311t:1557` presuppose. No
  typed reversal. Registered: no (§ 4.2 registers `30W`'s context product, not this ruling).
- § 1.15 `who resolves a name` — `USER_STORY.md:593`, `KNOBS.md:44` (audited): “Keyed by the KIND,
  not by a command”; “per-KIND `resolve`/`disturbance_reaches`”; `24F:184` and the built resolver
  key it by kind. Against `pre:131`: “An MSort has no MKeys and no `resolve()`.” `312b:330`
  [HUMAN] hard nack: “a naming system is a term language, never a sort.” `311t:1641` [HUMAN]
  NACK of a shape: “collaboration joins at the SORT”. Corrected: the extraction also listed
  `312f:734` here; in its source "this speech" is store-level speech about naming, not
  `resolve()`. Registered: no.
- § 1.16 `the name-comparison floor` — `USER_STORY.md:598-599` (audited): “A kind with no resolver
  keeps plain name-comparison — today's floor, nothing revoked”; `24F:73` and `300:195`
  `[extracted]` keep it, the latter as `300:rul-reference-entity-name-floor`, which `312a:65`
  gives as “ruled; human-corrected 2026-08-15”. Against `pre:991`: the model excludes “an
  engine-side name floor”. `26Ob:702-707` (typed): “two keys being different must never be read
  as disjointness when the hosts may be the same”. Registered: no.
- § 1.17 `two sorts with no shared speech` — three answers. Built (`spike/crates/core/src/coord.rs`,
  near line 325): a kind mismatch is always provably disjoint, and the enum has no fourth
  answer. `30U:15-23` (acked 2026-08-29): cross-kind sparing “only when the wall's claimed kind
  carries a finished definition”, else the comparison “answers *unrelated*”; `271:212-221`
  (typed): “disjointness across vocabularies is earned speech, never construction”. 311
  (`pre:859-860`): “KNOWN_UNSPOKEN never spares and never transports, whatever either side has
  declared finished”. `311t:1676-1678` (typed characterisation): “every sort walls everything
  around it until it is mapped into the filesystem, spiritually, allowing for the few roots”.
  `spike/AGENTS.md` compare-consumer-map states the `30U` form as law while the code has three
  answers. `notes/314` § 2 records what a model showed of the three. Registered: `30U` and the ANALYZER-NEEDS rows, yes; the code, no.
- § 1.18 `sibling cells` — `271:222-234` (typed, “as ruled for now … spike-provisional”): a
  selector dialect separates two selectors of one entity. Against `pre:343`: “Two cells of one
  MParent are two MSorts.” Registered: yes.
- § 1.19 `the invariance, disjointness, and lend speech acts` — `271:640-651`
  `rul-invariance-speech-act` (typed 2026-07-12); `30W:83-88` `rul-disjoint-is-an-rc-predicate`
  (typed); `271:rul-lend-map` (typed 2026-07-11; `spike/AGENTS.md`: “a MISSING dimension = ⊤,
  walls (the enumerate-every-dimension law; absent-means-full-lend is REJECTED)”). The model
  dissolves the first two (registered) and replaces the third with a completion sentinel under
  which unlent sorts inherit (`pre:920-923`; not registered).
- § 1.20 `context as part of identity` — the built fact key and `26Ob` `[extracted]` carry it.
  Against `pre:355-357`: the MVantage “is not part of any MKey's identity.” No typed line.
  Registered: for `30W` and `26Ob`; not for the code.
- § 1.21 `which way a wrong merge fails` — `USER_STORY.md:600-602` (audited): “a resolver that
  wrongly MERGES two entities only over-verifies; one that wrongly SPLITS one referent re-opens
  the silent skip”. Against `28Q:312-315` (acked): “for hosts a wrong MERGE is not conservative”,
  and `311b:297-302` (typed): “sameness is not a correctness freebie. Some consumers are
  endangered by wrong sameness, others by wrong distinctness.” Registered: no.
- § 1.22 `the count of primary schemes` — one per sort (`311q:30` `[extracted]`), then
  `311t:1557-1563` (typed acks): “the minimum of one scheme per sort DROPPED”, then a lean to
  remove the coherence sentence (`312d` § 21 `[extracted]`).
- § 1.23 `property graduation` — accepted at `311q:402`, killed at `311q:538` `[extracted]`.
- § 1.24 `a recreated thing` — `28Q:279`, `28Q:681` `[extracted]`: the door to a recreated
  instance being the same thing is held open. Against `pre:74-75`: “It does not survive
  destruction and recreation under its old MKey.” `311b:334-340` (typed):
  `rul-reuse-incarnation-invalidation`, a nack of refresh-specific machinery. Registered: no.
- § 1.25 `re-reading at apply time` — `24F:86` `[extracted]`: no apply-time resolution;
  `spike/AGENTS.md` toctou-scope: identified-cause re-verification is in, unattributed-drift
  machinery is out. Against the standup `witness()` of `pre:376-380`. `26Ob:169-171` (typed):
  “Dorc guards anything it can reach but only elides what it can witness … never to soften the
  rule.” Registered: no.
- § 1.26 `which of mechanism and prose outranks` — `30Ya:181` and `30Z:pos-mechanical-trumps-prose`
  (typed lean, “by law”) against `312d:21` `[extracted]`, which is for the mechanisation arc.
- § 1.27 `six registered matters with no later typed reaction` — `272` § 5 (locators never
  compared against File facts), `30T` per-aspect identity, `30T`'s v0 floor, `30W` § 4
  containment among stores, `272` § 3 with `27C` § 4 (an engine table that derives keying), and
  the compare-consumer-map. All six are in § 4.2.

## § 2-what-the-model-supersedes

### § 2.1 registered in § 4.2 of the 311 texts

Fifteen entries, the same in both texts (`pre:995-1098`). Each as *earlier text → the model*.

1. `30U:rul-cross-kind-sparing-needs-a-finished-definition`, with `30T:rul-binder-claims-are-ordinary`:
   a finished definition generates cross-kind disjoint verdicts → it stays necessary for sparing
   across sorts and generates no DISJOINT.
2. `ANALYZER-NEEDS` `an-kind-reach`, `an-compare-chokepoint`, `an-disjointness`: `unrelated` only
   absent a finished definition → `unrelated` is KNOWN_UNSPOKEN and never spares.
3. `30W` § 1 and § 5, `26Ob:res-per-index-relation-table`: one referent-transparent grade → two
   independent warrants per matched shape, and `:root` apart.
4. `30W` § 1 and § 10, `26Ob:res-worlds-compare-through-the-chokepoint`: the context slot is a
   product over index-kinds → it is an MVantage, part of no key's identity.
5. `30W` § 2 and § 3 `kind__disjoint()`, `30W:rul-disjoint-is-an-rc-predicate`, `30T:file-identity`:
   an owner-authored region predicate → none; containment is membership in a traversal.
6. `30W` § 2 to § 4, `26Ob`'s filtered meet and target pin, `27C` § 4(A),
   `271:rul-invariance-speech-act`: an invariance line licenses transport across an axis → no
   invariance line; reach is the shape of the fully qualified key, with observer-independence as
   the one remaining speech.
7. `272` § 3, `27C` § 4: an engine-owned table and taint derive keying → the engine holds no
   table that generates SAME.
8. `272` § 5: emitted locators are never compared against File facts → a may-read entry is a key
   that is compared against every writeset entry.
9. `30T:file-identity` per aspect → each aspect is a cell, a singleton sort.
10. `30T:file-identity` v0 floor: entry-mutating verbs make no at-most claims → creation,
    deletion, and rename are routing writes the verb's author claims.
11. `30W` § 4: containment among stores is a declared `reaches` → it is the parent chain.
12. `277` § 3, `30J` § 12, `spike/AGENTS.md` sparing-algebra: a selector dialect → none; two cells
    of one parent are two sorts.
13. `spike/AGENTS.md` compare-consumer-map, `311a:note-transport-single-consented-sparing-double`:
    every SAME is unflagged → a SAME resting on a wrapper's sentinel rides the flag.
14. `KNOBS:kSURVIVAL`, `ANALYZER-NEEDS` `an-mode-gate`: the flag gates sparing → it also gates
    that SAME.
15. `USER_STORY.md`, the bought-unsoundness section: past the flag, only authors' at-most claims
    → also wrappers' sentinels.

`312a` § 2 (`312a:44-73`) lists seven typed rulings the model modifies, headed “Each needs a
typed reversal or a re-ack; chat leans do not overturn typed rulings.” Its slugs:
`reack-invariance-speech-act`, `reack-disjoint-rc-predicate`, `reack-lend-map-sentinel`,
`reack-selector-and-dialect`, `reack-name-floor-default`, `reack-cross-sort-same-now-exists`,
`check-posture-options-lost-their-home`. No later ledger cites any of them `[extracted]`.

### § 2.2 not registered

Earlier text the model departs from that § 4.2 does not name. Root documents are the human's.

- `IMPLEMENTATION.md` on unattributed elision and the flag, `271:rul-flag-is-razor-residue`, and
  `23D:90-91` (§ 1.1).
- `USER_STORY.md` stage 5 on the backing, `24D:164`, `23O:185`, `30U:67`, and `279f` § 3 with
  `27C` (§ 1.8).
- `238` § 3 and `IMPLEMENTATION.md` on first-order effects and the horizon (§ 1.9).
- `24A` § 1e and the built chronology net, as an account of what a wrong elision is (§ 1.4).
- `271:rul-coordinate-shape-flat-three-place` and `spike/AGENTS.md` coordinate-semantics (§ 1.14).
- `USER_STORY.md` stage 6 and `KNOBS:kBURDEN` on one resolver per kind (§ 1.15).
- `USER_STORY.md` stage 6, `24F`, and `300` on the name floor (§ 1.16).
- `271:rul-lend-map` and the enumerate-every-dimension law (§ 1.19).
- `23O:synonym-cell-scoped` and the second clause of `spike/AGENTS.md` top-identifies-with-nothing
  (“cross-kind *same* does not exist”), against two schemes yielding one primary key as a
  same-referent generator across sorts (`pre:860-862`; `312a` `reack-cross-sort-same-now-exists`).
- `26Ob:ack-rich-internals-keyed-by-the-matrix` and `26Ob:ack-cross-world-wall-is-the-floor`
  (typed): they reserve a posture flag, and the model does not say where one would attach
  (`312a:71-73`).
- `30U:rul-definitions-not-surveys`, against the specification's world truth for the finished
  record (§ 1.11).
- `28Q` on a recreated instance (§ 1.24); `USER_STORY.md` stage 6 on merges (§ 1.21).
- `24F:86` and `spike/AGENTS.md` toctou-scope, against the standup `witness()` and the three
  mutator species (§ 1.25).
- Stale bridges: root `AGENTS.md` terminology firming still says `MKey-CatalogStore` and
  `MKey-PrimaryStore`, which `spec:4311-4312` gives as superseded by MParent-Catalog and
  MParent-Store; `311u:163` still says the engine vouches (§ 1.7).
- The held-work inventory also lists, as owed at a freeze and not in § 4.2: `spike/AGENTS.md`
  pure-predicate-carry, `USER_STORY.md` stages 5 and 7, and `IMPLEMENTATION.md`'s definition of
  the flag.

## § 3-names

- Acked: MReferent, MState, MValue (`311t:1464`, [HUMAN] ACKED, typed: “MReferent, MState, MValue
  as three terms”). KNOWN_UNSPOKEN, by the human's own commit `adaaf990`. The flag's name
  (`271:686-687`, typed). Foundation (`312d:1271-1272`, typed: “Foundation, not Ground; not to be
  bikeshedded”). `:aliases-nothing-else` as the store's closure (`311t:1171-1172`, STAMPED), with
  “many names will churn if the table shape is taken”.
- No typed ack found: MSort (only [HUMAN] paraphrase lines react to it); SAME, DISJOINT, UNKNOWN;
  MScheme; the warrants' names.
- Nacked or disliked, each as its ledger records it: “statement” as the genus (`312d:1292`,
  typed); the engine as “vouch” (`312d:1273-1274`, typed); “Hands out the address”, “mints”,
  “Holds”, and “lives in” as phrasing that pierces referential agnosticism (`311t:614-626`,
  [HUMAN] gentle nacks; the last two as glosses for where a thing's state is); “stranger” as a
  term of the engine's (`311t:1143`, [HUMAN]); “view”, “mildly unsold” (`311t:1164`); and, `[extracted]`,
  “store” (`311b:367`), “observer” (`311q:332`), `:rootness` (`312cg:714`), “perish” (`312cg:670`).
- The world relations: `holds`, `owns`, `affects`, `passes`, `reaches`, `dependsOn`, `lineWrites`,
  and the terms "world stratum" and "truth predicate", are the mechaniser's. None has an ack.
  `312d:210-211` (typed): the human read the truth predicates “as duplicating each English
  sentence twice”; `312d:834-835`: “the human does not know what the term means”.
- The two-ended table by what is true in the world: `311t:660` [HUMAN] lean, “re-cut 311 ENTIRELY
  according to this table”, on condition that it first prove “MORE CORRECT than the current
  model on concrete ops strawmen”; `312d:224` (typed): “the two tables of 311 § 5 are
  non-normative”; `312d:857-858` (typed): “the human's mental model of this corner is effectively
  the two tables of § 5”; `312f:290-291`: “a useful search pattern, not authority”. The re-cut
  was not done.
- Coinages of `312f` and its exercise records with no definition: "carrier" in the file sense
  (the pre-mechanisation text uses "carrier" for an MSort, `pre:105-110`), "virtual MSort",
  "location MScheme", "naming stability", "the token rule", "member-write leg", "in force".
- Things with no clear name anywhere, each with the plainest description the corpus gives:
   1. the thing's-end closure: `pre:1175`, “this MReferent has no other home”
   2. the hole `312f` is about: `312f:629-631`, “"is the write inside the store?"” in place of
      “"does the write change how the store names things?"”
   3. the store-level naming-stability statement: `312f:319-320`, “A direct, conditional
      stability promise by a naming-system owner”
   4. the parent category of a catalog: `312cg:864`, “the term for "the thing it was resolved
      inside"”
   5. the world relations as a set: `312d:1168`, “a subject for "true" and "wrong", which 311
      deliberately lacks”
   6. what a fact depends on, from the write end: `pre:526`; `312cg:1258`, “`depends-on` edges”
   7. what a warrant is: `311t:1567`, open; `pre:402`, “Every warrant is typed speech, never an
      exit status.”
   8. the licensing law of a two-ended row: `312cg:559-561`, “Whether both ends are called at all
      is undecided”. `312f:290-293` sets it aside by name `[extracted]`.
   9. the named read: `311t:1409-1410`, “an author's question over one or more fields”
   10. the kind of a write: `312f:583-585`, “does not necessarily distinguish a content update
       from deletion.”
   11. a key's referent changing at a line: `312f:703`, “only `reaches` needs to vary, and only by
       line”
   12. the closing act's form: `pre:286-287`, “The lookup's owner closes it by an explicit act”
   13. direct against total effect: `312f:43-44`, “whether a finished declaration names direct
       effects or all eventual consequences”
   14. one lifetime of a target: `26Ob:320-321`, “one continuous period during which the entered
       thing exists and runs”; `26Ob:325`, “name unchosen”
   15. the item-or-children mark: `311t:1376-1377`
   16. the identifying store's word: `spec:4349`, “One word is owed”
   17. a negative that names both of its sides (the route to a flag-free floor in `313e` § 1), and
       whoever describes a store: neither is in any naming list.

## § 4-rulings-nothing-later-reverses

Typed or acked lines with no later typed narrowing found, each with its status. Seven of them
are the latest line on a matter of § 1 and are marked (§ 1.x); confirming one of those answers
that matter.

Identity:

- `311b:327-329` (hard ACK): “identity means exclusively identity. It supplies an input to other
  actions; those consumers determine what it permits.”
- `311b:330-333` (hard ACK): “same/disjoint/unknown must be available.”
- `311b:297-302` (typed): “sameness is not a correctness freebie.” (§ 1.21)
- `311b:436-441` (typed): “Expect most identity descriptions to do nothing without substantial
  contributions from several humans; no cheap implicit baseline is owed.”
- `311b:559-561` (typed): “assume generalized storage in arbitrary abstract MSorts … File-only is
  insufficient.” `311b:715-718` (typed): “User, NetNS, and the other context dimensions are
  ordinary stdlib types, not engine-special types or built-in MWorld semantics.”
- `311q:27`, `311q:32-33` ([HUMAN], "typed this sitting"): “B(b) with MScheme is the design,
  unqualified”; “An arm is implementation, not model”.
- `311q:357-360` (typed): “a correspondence weaker than sameness of the whole thing (a part, a
  view, a correlate) does not compose with DISJOINT”.
- `311t:1118-1122` ([HUMAN] hard ack): “an author who mints an MSort and hangs it on nothing
  comparable stays guard-only under the flag.”
- `311t:1136-1144` ([HUMAN] ack, a paraphrase tag in that ledger): “separation is only ever
  concluded from a single definition's own distinctions”.
- `311t:1557-1563` (typed acks, less the deep-key clause): “the engine never reads key syntax at
  the model level”.
- `26Ob:702-707` (typed). `312cg:173-175` (typed): “there is no machine-crossing class visible to
  Dorc”. `312cg:1212-1214` (typed): “the MRoute is the vantage-root and is not glossed as "the
  machine". A boot is not a machine.” `312cg:1229-1231` (typed): “"nobody has said the disk and
  the file are different things" is understood and internally consistent; any repair must
  acknowledge it”.
- `312f:442-448` (acked): “Argument-to-output dataflow grants no semantic identity”.
- `271:212-221` (typed) (§ 1.17). `30W:83-88` and `271:640-651` (typed): contradicted by the model
  and registered, with no typed reversal (§ 1.19).

Effects and relations:

- `312cg:549-556` (typed): “Every Dorc API function describes what Dorc needs to know, which is
  what will happen at apply time, and not what the tool easily gives.”
- `312cg:557-562` (typed): on a detected disagreement between two ends, “the safest conjunct for
  each consumer … Whether both ends are called at all is undecided.”
- `312cg:308-315` (typed): “No cell licenses danger unflagged.” `312cg:391-394` (typed) (§ 1.8).
  `312cg:545-548` (typed) (§ 1.12).
- `312cg:1189-1191`, `1206-1207`, `1232-1237` (typed): fail safe; “missing falls toward ⊤, as
  usual, no exception”; “A high stdlib burden before elision is evidence of a correctly formed
  311.”
- `312b:192-196` (acked) (§ 1.10). `30U:30-46` (typed): the `disturbs nothing-else` record, the
  witness law, and definitions-not-surveys (§ 1.11).
- `312f:56-59` (typed): “Dorc knows no image semantics.” `312f:283-288` (typed): “A single warrant
  is therefore not necessarily uniform across a scheme.” `312f:552-556` (typed sub-ruling):
  “`__resolve` authors must not describe the effects of writes to the parent store … Knowledge and
  responsibility for effects on other items' naming belong at the database/store level.”
- `312f:609-631` (acked, "the problem statement only") and `312f:633-636` (acked "generally", the
  path): “park everything off the critical path; firm the identity half; rule the effect world,
  time-boxed to about two sittings, with the token clause as its first worked case; and, if that
  does not converge, take the floor on every value-only hole, close the soundness holes, and
  freeze.”
- `279f:91-94` (human-ruled): cross-context transport by measuring in the site's own context.

Time, attribution, the flag, the engine:

- `312b:21-23` (typed positions): “known-but-deferred effects stay in scope”. `311q:342-344`
  ([HUMAN] gentle ack): warrants hold while a resolution or token stands, reissue from outside
  the book being the horizon. Not a ruling: “Dorc models the disturbing act, never a duration”
  (`312f` § 14.2) is recorded as a gentle, probable nack, “Open to pushback”; `30W:254-259`
  (“No metric time”) sits in a section that awaits ruling.
- `311a:217-220` (typed): “'every step toward same or disjoint has an author's name on it' is
  literal”. `30Ya:202-203` (typed): “attribution is the product.” `312d:1257-1260` (typed): a hard
  nack on “any frequency terminology about outcomes”.
- `312cg:138-143` (typed): “enumerate every piece of external state a move can change, or wall.
  Hence the flag.” `26Ob:169-171` (typed). The welds of KNOBS.
- `312d:1219-1233` (typed): the engine-known against the spoken, and the reserved measurement
  slot. `312d:1633-1635` (typed): “"which worlds the checker considers" IS design … The only
  worlds the checker may be boxed out of are the worlds chosen, and documented, as horizon.”

## § 5-where-the-models-objects-came-from

`[extracted]` throughout; the design-archaeology scout's reading.

- Earlier forms: may-read from an "invalidation-basis" alternative (`236b`) through
  `kind__state_stored_only_in` (`272`, name typed) to `30W` § 3's stored-in with its closure;
  may-write from `disturbs`; the entailment and finished record from `disturbance_reaches` and
  the `disturbs nothing-else` record (`30U`); resolution and traversal from `30T` § 6's
  "perishable"; the placeholder and the standup witness from `26Ob`; the § 0 law from `24A` § 1e
  with `272` § 1.
- New in r31, with no earlier form found: the scheme as an object apart from the sort; one
  parent per key with its three seats; the split of the lookup warrant in two; the per-key
  `alias nothing-else`; the walk with its levels and tops; the upward `:places` lookup.
- Time has earlier vocabulary the model did not take up: `28Q:20-25` (acked) has incarnation,
  lifecycle event, and availability window; `28Q:485` has piecewise truth over program points
  with kill events.
- The first draft of the model was written without its author reading `311b`, at the human's
  direction; that fence was lifted on 2026-09-16 (`311c:13-14`).

## § 6-the-mechanised-specification

- Its world side is the mechaniser's (§ 1.5). `312d` § 17.1 reduces fourteen unacked marks to
  five questions on that vocabulary: `ask-holds`, `ask-owns`, `ask-affects-and-the-chain`,
  `ask-passes`, `ask-the-actual-bits`. Later ledgers still list them as held `[extracted]`.
- Thirteen sites carry an unacked reading `[extracted]`: `spec:159`, `448`, `556`, `963`, `1044`,
  `1184`, `1478`, `1937`, `2100`, `2350`, `2830`, `3761`, `3979`. One of them,
  `enc-primary-owner-is-sort-owner`, refuses a case the human kept in scope at `312f:741-742`: a
  stranger's sort naming one's scheme as its primary.
- Of its nine holes: five are posed in the world relations, three of those being shapes of the
  one cell the pre-mechanisation table leaves open (§ 3, item 1); two sit on truth predicates
  whose bodies are empty (`true_DeclaresCell`, `spec:724`; `true_DeclaresComposite`, `spec:2339`);
  two are questions about the region test and read no world relation.
- 9 of its 28 truth predicates have an empty body. Four species have no fact saying who speaks
  them `[extracted]`: `VerdictFact`, `DeclaresAliasesNothingElse`, `DeclaresMayWrite`,
  `ClosesMayWrite`.
- Its § 0 sentence “The true answer is reachable once those who can know have spoken”
  (`spec:87`) is normative prose that no law checks, and no ledger says what it ranges over.
- `law_exclusion_readings_agree` is a recorded counterexample: one sentence of the
  pre-mechanisation § 2.6 has two readings that disagree `[extracted]`.
- State and time are not in its fences (`spec:177`).
- Against the lock, `312e` miscounts `[extracted]`: it says thirteen checks where there are
  fourteen, and fourteen satisfiable kills where the lock has seventeen, with five unsatisfiable
  and two timed out.

## § 7-the-built-engine-against-the-model

`[extracted]`; the implementation scout's reading.

- The one built account of what is so, apart from what is declared, is the chronology net
  (`spike/crates/sweep` with `hostsim`). Its world is a set of the engine's own fact keys. It has
  no things apart from their names; aliasing and reach are encoded by a generator writing a
  delta that kills the victim's cell.
- The sparing reference model is not a world semantics; its header says disjoint “GIVEN the
  contracted claims — never machine-established referent-inequality”. The Lean corpus holds
  three laws, none about identity or sparing. The bounded-verification harnesses over `compare`
  are structural.
- The comparison has three answers in code. Nothing in production consumes SAME. The finished
  record and the completion record do not exist in code. The entailment role is built under the
  name `__disturbance_reaches_only`.
- With no built counterpart: the scheme, the parent chain, the warrant split,
  `:aliases-nothing-else`, `:root`, traversals and the region test, the may-read closure, and the
  three mutator species.

## § 8-open-questions-in-the-humans-own-terms

- `312f` § 13.4 ([HUMAN], put for a next turn): whether the primary scheme's `__resolve` is the
  store owner's naming half; whether, since “no store-level API endpoint for an MScheme was
  designed, unless forgotten”, this speech belongs on the scheme and not the sort; whether it is
  meaningful only for primary schemes; and that this “may be the first time MSchemes get
  significant, dangerous descriptive power”, which must fail safe with no collaboration.
- `312f` § 14.3 ([HUMAN]): “Define the new member or members that would close this issue
  precisely and completely, so that the problem has a name. Only then test whether the member is
  unnecessary, another member in disguise, or derivable by engine logic.” And: find a concrete
  store that offers both directions of a naming map as separate commands.
- `312f` § 14.2 ([HUMAN]): the two ends of a row exist also for performance, one store-level call
  against a per-key call across every key; dpkg's files-of-a-package and package-of-a-file are
  the example.
- `312f:622-623` (acked, in the problem statement): “if three or four rulings on what the world
  relations mean do not collapse most effect-side holes, the framing is wrong.” `notes/314` § 3
  records what four such rulings did.
- `312d` § 17.3 (typed): the horizon is drawn by the specification, and how the specification
  “mechanically says "TOCTOU is out of scope" to the adversary” was asked for as a strawman.
- `313e` § 1 (typed): “Answers are about one instant” drew hesitation and was never acked; six
  cells must be buildable; a closure line is assertion and consent, one object, revocable.
- Held when mechanisation began (`312cg:1412-1414`, `1449-1461` `[extracted]`):
  `hold-region-against-region-floor`, `hold-stdlib-key-space-over-the-boots-children`,
  `hold-cells-sparing-or-freshness`, `hold-exclusion-keyed-on-the-container`,
  `hold-cross-world-entry-cost`, and the within-sort reading of the finished record.
