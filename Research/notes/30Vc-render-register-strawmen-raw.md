# 30Vc — render-register strawmen: the raw examples (adjudicated)

> Raw bank for `30V` §7. STRAWMAN-TIER THROUGHOUT — stylistic state-space
> exploration only; nothing here is a spec. Each generation carries its
> verdict inline. NB the ASCII-only ruling postdates gen-3, which used
> Unicode; it is preserved verbatim as evidence, not as an example to
> follow. Gen-1 is `30Vb` and is not reproduced here.

## §1 gen-2 — single-line greppable records (sh-parseable)

One record, the survived elision, as it would have appeared:

```
webhost.sh:9 elide speaker=me world=plan/2026-07-17T06:11:52/web1/a3 license=measured:m-9f2 license=vouch:systemctl.oracle.sh:12 survives=wall:webhost.sh:8 via=claim:foobar.oracle.sh:31 consent=--risk-faultless-skips carrier=r-0611 ref=d-4a1
```

Design intent: one line per record; leading locus anchor; record-kind token;
`key=value` tail with sh quoting; repeated keys for sets; content-derived
ref last; whole output `while read -r`-consumable.

VERDICT (human): dead as the DEFAULT — `--json` owns premeditated
filtering, so the text surface's pipe-consumer has already read our format
and wants less of the same, which makes readability the first duty; and the
axes cannot be encoded readably in one line. Found the correct LEVEL of
non-prosody, though; survives as the `--oneline`-style option.

## §2 gen-3 — dense block-records + shadow gutter (verbatim scratch artifact)

VERDICT (human): good tracking of the kinds-of-information-to-surface at a
given intersection of axes — density, ordering, layout, what-to-communicate
-when. Surface itself rejected: too prosodic; Unicode is a nope. The
opaque-refs idea acked (evolved into `30V` §2 rul-refs-are-short-urls).

The artifact, verbatim (originally `strawman-why-all.tmp.md`, repo root,
since removed):

````markdown
# strawman: `dorc why --all` — the total surface, dense-case gloss

> Scratch artifact (untracked, uncommitted, disposable). Second-generation
> strawman, post the 2026-08-30 sittings; supersedes `30Vb`'s approach.
> GLOSS-MODE: three site-blocks fully enumerated, the rest skimmed. Every
> spelling below (labels, refs, rail glyphs) is strawman; the STRUCTURE is
> what's under discussion.

## §1 the problem, chosen before any drawing

- **user-class**: the mixed-maturity solo admin — the survival-buying half of
  the population — but their book contains an org-tool delegation line, so
  the org-half's permanent-wall reality appears in the same dump. (Survival
  is deliberately IN this story AND structurally impossible past line 12:
  both halves of the tune exercised at once.)
- **epistemics**: the flagship inversion — the apply looked perfect, the
  user knows something is wrong, nobody knows what. They have pulled the
  addressless total view (the back-out surface) after a first
  address-directed pull caught nothing.
- **Dorc mechanisms**: elide · omit (a folded guard's dead arm) · guard ·
  run · survive (committee-licensed, flag-consented) · two wall species
  (unmodeled-total and declined-total) · a described mutator that really ran
  (a permeable, claims-scoped shadow) · a loop region with per-member
  verdicts · a dorc-lang `.` load (locus stages, custody) · a lifted admin
  guard (Half-B) · a SIGPIPE-flap can't-tell · an apply-time guard firing
  (divergence) · book drift since the apply (né-namespace).
- **world-states**: probe 07:02, plan 07:02, consent 07:03, apply 07:04,
  outcome 07:05 (2026-08-14, host `edge1.example.net`, attempt `a-7c2`);
  book edited after the apply at one line; receipt graph complete
  (plan → intent → outcome); receipt format v1 (one carrier-absence hole
  exercised); `--risk-faultless-skips` was typed.
- **CFG**: linear head · a compound guard (`||`) · a three-member `for` loop
  · variable dataflow (`$REGION` minted at :4, consumed at :8) · a load at
  :3 whose definitions the oracle layer consumes.

## §2 the pretend book (`edge-fleet.sh`, as recorded)

```sh
 1  #!/bin/sh
 2  set -eu
 3  . ./lib/certs.dorc.sh
 4  REGION=eu-west
 5  apt-get update
 6  dpkg -s wireguard >/dev/null 2>&1 || apt-get install -y wireguard
 7  for node in alpha beta gamma; do
 8     provision-node "$node" "$REGION"
 9  done
10  systemctl reload wg-mesh
11  vendor-mesh tune --profile edge
12  ansible-playbook site.yml --tags certs
13  ufw allow 51820/udp
```

Oracles loaded: the stdlib (apt/dpkg/systemctl/ufw) · `provision.oracle.sh`
(a named third-party author; verdict + `disturbs` emitting
`org.ex.Node:$node@provisioned`) · the `org.ex.Node` kind-owner's file
(`disturbance_reaches` → `sm.dorc.File`, with a reached `nothing-else`) ·
an ansible oracle that is one decline (`return 2`, class: unsound) ·
`lib/certs.dorc.sh` via the book's own `.` line. `vendor-mesh` is described
by nobody. Current book differs from recorded at :10 only
(`reload` → `restart`).

Per-line fates, for orientation (each gets a block in the real output):
:5 ran (probe can't-tell — rc-141 flap) · :6 elided + install-arm omitted
(the admin's own lifted guard) · :8 alpha/gamma elided, **beta ran** ·
:10 elided, SURVIVING beta's run (committee + flag) · :11 ran (unmodeled,
total wall) · :12 ran (declined, total wall) · :13 guarded; the guard FIRED
at apply and the original ran (divergence, rc 0).

## §3 packing it into 80 columns and a pager

Column budget: `[gutter 5][label 10][content 65]`. One drawn channel only —
the **shadow rails** (walls + described-mutator reach), everything else as
refs. Rails are REDUNDANT BY LAW: every rail-event is also a field on some
block line, so the gutter can decorate but cannot lose (and a test-only
inverse parser can check drawn columns against stated fields — separate
dev-lineage from the generator, generative not golden).

- Outer sort: program-lexical (= apply order, per no-reorder). Inner sort:
  moment-minor within each block (probe → plan → consent → apply → outcome).
- Blocks are multi-line and readable-first; labels are stable greppable
  words in a fixed column (`grep -C` is the expected re-read tool; `--json`
  owns premeditated filtering; a `--oneline` projection exists but is not
  this surface).
- Loop regions render ONCE at the definition, with per-member instance
  sub-blocks in member order.
- The whole-run head is data, not prose: identities, consent, sources with
  digests and drift, the phase timeline, the rail legend.
- Refs `[x-…]` are content-derived and stable across re-invocation (the
  future dig-dive addresses). `see :N` cross-references stay textual.
- Rails: `.` none · `!` a permeable shadow (described mutator; claims may
  carry facts across) · `#` a total shadow (unmodeled or declined; nothing
  crosses). A rail column is born at its caster and runs to end-of-book.
  Glyphs/charset entirely unruled; semantics are the point.

## §4 the render, glossed

```
dorc why --all · receipt r-out-0705 (outcome) ← r-int-0703 ← r-plan-0702
book      edge-fleet.sh (recorded); current file DIFFERS at :10 (see block)
host      edge1.example.net · attempt a-7c2 · applied 2026-08-14 07:04
consent   --risk-faultless-skips (you, 07:03:02)                  [k-flag-1]
sources   lib/certs.dorc.sh (dorc-lang, digest ok) · provision.oracle.sh
          (digest ok) · org.ex.Node owner file (digest ok) · stdlib …
          ansible.oracle.sh (digest ok) · current-vs-recorded: 1 line né
phases    probe 07:02:11-14 · plan 07:02:41 · consent 07:03 · apply 07:04
rails     A = :8[beta] provision-node (permeable — claims-scoped)
          B = :11 vendor-mesh (total — undescribed)
          C = :12 ansible-playbook (total — its oracle declines to answer)
counts    from decision digest: 4 elided · 1 omitted-arm · 1 guarded ·
          5 ran (3 by member) · 1 survival                        [digest]

...   ┌─ edge-fleet.sh:5 · ran · apt-get update
...   │  subject    sm.dorc.PkgIndex@fresh                       [s-idx-1]
...   │  measured   probe 07:02:11: CAN'T-TELL — check exited 141
...   │             (likely benign early-exit race; known flap class)
...   │  derived    I cannot elide on a can't-tell; the line runs [d-run-5]
...   │  outcome    ran at 07:04:02, exited 0
      [ :6 block — fully enumerated below in §5a ]
      [ :7-9 region block — §5b; alpha/gamma skimmed, beta enumerated ]
      [ :10 block — §5c, the survival + né site ]
A..   ┌─ edge-fleet.sh:11 · ran · vendor-mesh tune --profile edge
A#.   │  derived    nobody describes 'vendor-mesh'; silence licenses
A#.   │             nothing — it runs, and every measurement from above
A#.   │             is untrustworthy below this line (total shadow B)
A#.   │  outcome    ran at 07:04:19, exited 0                    [d-run-11]
A#.   ┌─ edge-fleet.sh:12 · ran · ansible-playbook site.yml --tags certs
A##   │  declined   the ansible oracle answers can't-say for every verb —
A##   │             a play's check-mode has no trustworthy whole-play yes
A##   │             (its author's judgment, ansible.oracle.sh:2) [n-dec-12]
A##   │  derived    a declined delegation runs, always, and walls (C)
A##   │  outcome    ran at 07:04:31, exited 0
      [ :13 block — §5d, the fired guard ]
```

### §5a — the admin's own guard (elide + omit + a carrier hole)

```
...   ┌─ edge-fleet.sh:6 · elided · dpkg -s wireguard … || apt-get install…
...   │  subject    sm.dorc.Package:wireguard@installed         [s-pkg-wg]
...   │  measured   probe 07:02:12: your own guard's read, lifted —
...   │             dpkg -s exited 0 (through dpkg.oracle argparse)
...   │  vouched    installed-is-enough — stdlib dpkg author    [v-dpkg-1]
...   │  derived    the || arm is dead on this host: the install
...   │             never runs (omitted)                        [d-omit-6]
...   │  decision   I removed the whole line from the apply      [d-el-6]
...   │  lattice    absent from carrier: receipt v1 recorded the
...   │             decision, not the possibility-set I collapsed from
...   │  timing     not yet piped into this reconstruction (NYI)
...   │  outcome    did not run (r-out-0705)
```

Notes: the guard is YOUR line (Half-B) — the lift is credited to the book
author's own sh, through the oracle's argparse, never engine token-matching.
The two wrapper states land here deliberately: `lattice` is
known-absent-from-THIS-carrier (immutable v1 bytes; not NYI — piping cannot
help, and not unknowable — future receipt versions can record it); `timing`
is NYI (recorded, but this reconstruction doesn't thread it yet).

### §5b — the loop region (one definition, three fates)

```
...   ┌─ edge-fleet.sh:7-9 · region · for node in alpha beta gamma
...   │  region     one authored body; three member instances; the edit
...   │             unit is the region, verdicts are per-member [r-loop-7]
...   │  value      "$REGION" = eu-west (from :4, program-text grade)
...   ├─ [alpha] elided · measured provisioned 07:02:13 · vouched
...   │          (provision author) · did not run          [d-el-8a]
...   ├─ [beta]  RAN · measured DIVERGED 07:02:13 → ran 07:04:0
...   │          rc 0; casts shadow A from here down; its at-most
...   │          claim is what scopes A (see :10)          [d-run-8b]
...   ├─ [gamma] elided · as alpha                          [d-el-8c]
```

### §5c — the survival, the committee, and the né line

```
A..   ┌─ edge-fleet.sh:10 · elided (SURVIVED shadow A) · systemctl reload…
A..   │  né         the current book differs HERE: recorded 'reload',
A..   │             on disk today 'restart'. this block speaks the
A..   │             RECORDED line; no receipt speaks for today's
A..   │  subject    sm.dorc.Service:wg-mesh@active            [s-svc-1]
A..   │  measured   probe 07:02:13: is-active rc 0 — the oracle's own
A..   │             check (systemctl.oracle.sh:14; file today matches
A..   │             the recorded digest)
A..   │  vouched    already-active accepted as reason-not-to-rerun —
A..   │             stdlib service author [v-svc-2]; authored-when:
A..   │             unknowable (authorship moments untracked, by design)
A..   │  ran-above  :8[beta] really ran — ordinarily that sends this
A..   │             line back into the apply as a live re-check
A..   │  claimed    at-most org.ex.Node:beta — provision.oracle.sh:31
A..   │             (the provision author; nothing verified it)
A..   │  claimed    a Node reaches its Files, and nothing else —
A..   │             org.ex.Node's kind owner (owner file :9)
A..   │  derived    I compared Service:wg-mesh@active against that
A..   │             at-most set: provably disjoint. JOINT SPEECH: both
A..   │             claims are load-bearing and I cannot apportion
A..   │             blame between them                        [d-cmp-4]
A..   │  consented  --risk-faultless-skips (you)              [k-flag-1]
A..   │  decision   I kept this line out of the apply, across a real
A..   │             run above — the one naked-trust cell       [d-sv-10]
A..   │  outcome    did not run (r-out-0705)
```

### §5d — the guard that fired

```
A##   ┌─ edge-fleet.sh:13 · guarded · ufw allow 51820/udp
A##   │  subject    sm.dorc.Firewall:51820/udp@allowed        [s-fw-1]
A##   │  measured   probe 07:02:14: rule present
A##   │  derived    below B and C nothing measured above survives; the
A##   │             best I may do is re-check in place          [d-gd-13]
A##   │  planned    ( ufw_check … ) || ufw allow 51820/udp
A##   │  outcome    DIVERGED from expectation: at 07:04:44 the live
A##   │             check answered not-present, so your original line
A##   │             ran (rc 0). something changed the firewall between
A##   │             07:02:14 and 07:04:44 — that is what the guard is
A##   │             for; noted, not an error                    [d-dv-13]
```

## §6 axis coverage — every axis's durable home

| axis / state | home in this render |
|---|---|
| speaker (7 acts + committee + I/you) | the label column + named authors; joint-speech in §5c; "your own guard" §5a |
| world-coordinate | head phases + per-line stamps; attempt/host in head; moments minor within blocks |
| subject (site/region/instance/cell/value) | block headers; region/member sub-blocks §5b; `[s-…]` refs |
| payload: measurement / prediction-set / decision / license / value / negative-space / narrative | measured lines · `lattice` row (absent) · decision rows · vouch/claim/consent rows · `value` row §5b · can't-tell :5, decline :12, walls, carrier-absence §5a · flap-note :5 |
| source-of-delivery | head receipt chain; per-block outcome cites its receipt; (live-producer unexercised — field exists, noted) |
| wrapper (Knowable/NYI/Unknowable) | §5a `lattice` (carrier-absent, Knowable-domain) · §5a `timing` (NYI) · §5c `authored-when` (Unknowable) |
| structure: derivation / program / aggregation / receipt-graph / locus DAG | `[d-…]` operand refs · program-lexical spine · region §5b · head chain · `. ./lib` source row (stages skimmed) |
| né-namespace | head drift row + §5c `né` row |
| derived views (trust-order, remedies, relevance) | ABSENT BY DESIGN — `--all` renders data, never advice; the curated tiers own those |

Admitted misses (allowed-to-fail): descope (needs a `kSCOPE-asked` run —
different invocation, not forced into this book) · multi-host (out of v1) ·
the live producer (`dorc apply --why`) · a damaged/unauthenticated carrier
(belongs in a sibling story; mixing it here would bury the axis under the
drama).

## §7 questions this drawing surfaced

1. Rail birth at a loop member (`A` starts mid-region) — rails are
   per-instance events but columns are per-line; the `[beta]` sub-block owns
   the cast, the column starts at the region's last line. Awkward; wants a
   ruling when real.
2. The né-site renders the RECORDED line and refuses to speak for today's —
   is one `né` row enough, or does the current-line text belong in the block
   too (I leaned: head says the book drifted; the block shows recorded only,
   plus the pointer)?
3. Outcome lines folded into site blocks (chosen here) vs a separate
   outcome section mirroring the spine (rejected: kills locality) — but the
   fold means one block mixes three carriers (plan, intent, outcome
   receipts); the per-row carrier is implicit from the moment. Acceptable?
4. `counts` in the head come from the decision digest verbatim — the head
   never counts rendered rows (that would be a second derivation of the
   same fact, drift-capable).
5. Label vocabulary (`measured/vouched/claimed/derived/consented/declined/
   ran-above/né/lattice/timing…`) — the first six are the typed speech-acts;
   the rest are payload-kind labels. Two vocabularies in one column; is the
   visual flattening of that distinction acceptable at this tier?
````

## §3 gen-4 — code-woven spine, terse debugging register (in-chat)

Same story, lines :7–:11; full structural verbosity (ASCII gutter +
regenerated CFG) with factoids as near-debugging dumps:

```
$ dorc why edge-fleet.sh --all        [fragment: :7-:11 of 13]

.    7   for node in alpha beta gamma; do       # region, 3 members  [r7]
.    .     # val REGION=eu-west <-:4 grade=text
.    8      provision-node "$node" "$REGION"    # 2 elide, 1 ran
.    .     # [alpha] meas node:alpha@prov=y p07:02:13 -> elide      [d8a]
A    .     # [beta]  meas @prov=N -> RAN a07:04:07 rc=0 +shadowA    [d8b]
:    .     # [gamma] =alpha                                         [d8c]
:    9   done
:    10  systemctl reload wg-mesh               # elide, SURVIVED A [d10]
:    .     # NE! disk=restart recorded=reload (this block: recorded)
:    .     # meas svc:wg-mesh@active=y p07:02:13 via systemctl.o:14 [m3]
:    .     # vouch y stdlib-svc.o:14 [v2]  authored=UNKNOWABLE
:    .     # claim at-most{node:beta} provision.o:31                [c31]
:    .     # claim node->files+NOTHING-ELSE node-owner.o:9          [c9]
:    .     # cmp svc@active x at-most -> DISJOINT  joint{c31,c9}    [d4]
:    .     # consent --risk-faultless-skips you@07:03:02            [k1]
:    .     # lattice=ABSENT(carrier:v1)  timing=NYI
:    .     # out not-run  rOut0705
:B   11  vendor-mesh tune ..                    # RAN, undescribed  [d11]
```

VERDICT (human): mostly turned out WORSE — over-corrected past the
readable middle. Loop note that survives it: for sufficiently-local loops,
constant-prop silently and write the body once per member (the admin knows
they didn't write `provision-node beta` — repetition is understood);
several styles owed with a heuristic cutover for arbitrary CFGs.
