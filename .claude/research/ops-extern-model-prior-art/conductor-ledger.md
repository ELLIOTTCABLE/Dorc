# conductor-ledger — hand-down from the gather-round conductor (Fable, 2026-09-26)

> Written for a clean-context successor who will do the adjudication and synthesis. The outgoing
> conductor's context was half full of tool noise, so the human ruled that the reasoned synthesis
> is NOT the outgoing conductor's to write. A `plan.md` the outgoing conductor wrote after the
> research was removed at the human's direction (a plan written after the fact is not a plan);
> the few things in it that were not re-derivable from the lanes' files are in
> `cross-lane-observations` below. Everything else here is mechanics, state, rulings, and
> pointers. Grades throughout are the lanes' (`graded-by: subagent`); the outgoing conductor read
> every lane's gather file and hand-back but NO primary source.

## the-question

Prior art for `Research/notes/311-identity-and-relation-model.md`: a precise, broad, abstract,
battle-tested specification of the behaviour of ops externs, at any altitude, judged on four
criteria — exhaustive · broad (filesystem/DNS/…, odd cases) · abstract · more battle-tested than
ours. The human's examples: a famous tool's RFC repository that litigated the territory; an
academic modelling project that had to build a model of ops to do its analysis.

## typed-rulings-this-session (the human's words, paraphrased where noted)

- `rul-no-further-researchers`: "don't issue further researchers after these return." Ten lanes
  were dispatched in parallel before this; the human noted the interactive-research skill's later
  phases are serial and each front should inform the next. No repair; no new fronts.
- `rul-read-reports-before-primaries`: read all Opus-level reports and summaries before deciding
  whether anything belongs in a full context window; the choice between main-context reads and a
  clean-context Fable is the human's, on ack of a plan.
- `rul-commit-skill-granular`: commit with the `commit` skill, granular, on `ai/main`, by pathspec
  (a sibling session had 311 modified in the working tree throughout; nothing outside this
  directory was ever staged).
- `rul-disk-full-episode`: drive C: filled to 0 bytes mid-round from a cause OUTSIDE this session
  (this session's whole footprint was ~470MB); all lanes were paused, then the human freed ~118G
  and ruled: finish writing, avoid new fronts, workers may be reawakened but must not download
  repositories or run compilations. A restart to free more space follows, which loses every
  agent's context.
- `rul-no-outgoing-synthesis`: "I'll no longer have you write the reasoned synthesis; instead
  simply write a conductor-ledger doc … I'll create a clean context for adjudication and
  synthesis." This file.

## state-of-the-tree (as of the last update line at the bottom)

Committed on `ai/main`, research dir only:

- `8a842836` (AI new dsn) Bank ten prior-art gather lanes before the restart
- `c387fdf0` (AI new dsn) Write the first-pass plan and lift every lane into the turn notes
- (see the update line at the bottom for the closing commit)

Files in this directory:

- `turn01-2026-09-26-notes.md` — the skill's per-turn log: round design (lane map, mechanics),
  then the Findings section holding a conductor-lifted digest of EVERY lane (each bullet names its
  lane and its slugs), then a short Citations section. This is the fastest single read.
- `gather-<lane>.md` × 10 — the lanes' own reports (Findings / Candidate table / Citations /
  Leads not pulled / Search log / Tooling problems). Per-file status:
  - `gather-standards-management-models.md` (lane 1) — COMPLETE; written by the conductor
    verbatim from the lane's inline hand-back (the lane's own writes hit ENOSPC).
  - `gather-assessment-languages-and-verifiers.md` (lane 2) — COMPLETE; six `L{…}` citation
    placeholders remain (issues 107, 83, 301, 665, 7306; discussions 261, 281) — the verbatim
    quotes are present, only the archive line numbers are missing.
  - `gather-cm-big-four-resource-semantics.md` (lane 3) — Findings/table/breadth complete;
    Citations were a placeholder at hand-back; lane resumed to fill them (check the file).
  - `gather-cm-schema-and-module-systems.md` (lane 4) — COMPLETE (some HTML sources cited by
    anchor phrase rather than line; noted inside).
  - `gather-academic-verification-of-configuration-code.md` (lane 5a) — COMPLETE.
  - `gather-academic-theory-of-system-administration.md` (lane 5b) — COMPLETE.
  - `gather-platform-specs-filesystem-packages-installers.md` (lane 6) — Findings written by the
    conductor from the lane's hand-back; candidate table / citations / search log were owed by
    the lane, which was resumed to fill them (check the file; if still marked "owed", the lane's
    context is gone with the restart and those sections are simply absent).
  - `gather-platform-specs-services-network-names.md` (lane 7) — Findings/table complete;
    Citations were empty at hand-back; lane resumed with its `l7/cites.txt` + `l7/lines.sh`
    helpers (check the file).
  - `gather-api-resource-identity-rfcs.md` (lane 8) — COMPLETE.
  - `gather-naming-theory-classics.md` (lane 9) — COMPLETE; carries a classical-term → 311
    mapping table worth reading on its own.
  - `gather-wildcard-and-counter-thesis.md` (lane 10) — COMPLETE; carries a "seams to redraw"
    section and a breadth map.
- `sources.json` + `sources/` — the graded manifest and archived copies. Append-only; never
  hand-edited; the skill's `validate.sh` is the gate.
- `.register.lock` — a `mkdir` lock directory created by the outgoing conductor's wrapper; if it
  exists with no live `register.sh` process, remove it (`rmdir`); it is not part of the skill.

Ephemeral (scratchpad, lost at restart): `brief-shared.md` (the lanes' shared brief: the four
criteria, the 47-item breadth yardstick from `311r`, grading rules, deliverable format, clamps),
`register.sh` (the lock wrapper), `register-batch.sh` + `register-batch.log` (the serial
registration batch), `entry-*.json` (every lane's entry JSONs, ~255), lanes' reading copies
(`l1/`, `l5a/`, `l7/`). None of it is needed once the manifest is complete; the brief's
substance is reproduced in the turn notes' round design.

## registration-state-and-how-to-finish

The lanes could not register what they read because the skill's `new-source.sh` is
read-modify-write on `sources.json` with no locking; the conductor serialised it through a
`mkdir`-lock wrapper, and ten lanes then starved each other (the download runs INSIDE the lock,
`curl --retry 2` with no `--max-time`, and `mkdir` locking is not FIFO), producing 600-second
timeouts, then the disk filled. A serial batch (`register-batch.sh`) then registered the pending
entries from the lanes' entry JSONs: phase 1 = canonical-named files; phase 2 = short-named files
mapped from the lanes' hand-backs (lanes 5b, 9, and four of lane 4's).

A third phase then registered lane 4's remaining short-named entries, mapped by URL from the
scratch listing: [B-augeas-issue-68-idempotent-changes-2013] [B-bcfg2-architecture-client-2013]
[B-bcfg2-configuration-entries-2013] [B-bcfg2-literal-configuration-specification-2013]
[A-cfengine-promises-reference-2024] [A-inmanta-language-reference-2026]
[B-inmanta-unmanaged-resources-2026] [A-nix-manual-input-addressing-2026]
[A-nix-manual-store-object-2026] [A-nix-manual-store-path-2026]
[A-nixos-rfc0052-dynamic-ids-2019] [A-nixos-rfc0062-content-addressed-paths-2019]
[B-quattor-pan-language-book-2025] — all landed.

Anything `validate.sh` still flags after the closing commit (see the update line) can only be
re-registered by re-reading and re-grading once the scratchpad is gone; the lanes' grading text
survives in their gather files' candidate tables. The cheapest honest move is to leave such a
slug unregistered and say so.

Manifest hygiene to RECORD, not fix (the manifest is append-only by the skill's rule):

- `hyg-txos-registered-twice` — RESOLVED by the human's ruling "keep only one grade, conductor's
  choice": the TxOS paper had been registered under two slugs by two lanes; the conductor kept
  [A-porter-txos-operating-system-transactions-2009] (lane 10, a full read) and removed lane 5a's
  B-graded entry and archive (lane 5a's own read was self-declared partial); lane 5a's citations
  were re-pointed (same PDF, same `pdftotext -layout` lines). The one hand-edit of the manifest
  this round, done under the batch lock at the human's direction.
- `hyg-via-mislabel`: [A-nix-manual-store-object-2026]'s `via` names the GitHub MCP tool; the
  real call was `gh api` from Bash (lane 4 flagged it).
- `hyg-oval-docx-as-html`: [A-oval-language-specification-5112-2016] is a `.docx` archived
  under `.html`; lane 2's line cites are from its own text conversion and give the section name
  alongside.
- `hyg-crossplane-archive`: lane 8 believed [A-crossplane-managed-resources-external-name-2026]
  registered without an archived file; the manifest-vs-archive count matched afterwards, so
  probably fine — verify with `ls sources/A-crossplane-managed-resources-external-name-2026.*`.
- `hyg-registered-without-archive-audit`: after any ENOSPC, run
  `for s in $(jq -r 'keys[]' sources.json); do ls sources/"$s".* >/dev/null 2>&1 || echo "$s"; done`.

## tooling-defects (patched at the human's direction, "fix new-source as you see fit; keep it narrow")

`~/.claude/skills/interactive-research/scripts/new-source.sh` (outside the repo; the human's
dotfiles) now carries two narrow changes, `sh -n` clean and exercised by roughly ninety
registrations of this round's serial batch after the edit (one registration that read the script
mid-edit failed with a spurious syntax error and was retried cleanly):

- `--max-time 120` on the artifact download (`curl -fsSL --retry 2 --max-time 120`); the
  out-of-band landing-page probe already had `--max-time 20`.
- a `mkdir`-based `manifest_lock` around ONLY the `sources.json` append (never the download),
  with a 120-second wait, an EXIT trap to release, and a re-check of `has($slug)` under the lock
  so two concurrent registrants of one slug cannot both append. Lock path:
  `<research-dir>/.manifest.lock`; a stale one from a killed run is removed with `rmdir`.

Residual, not patched: the lock is not FIFO (ten bursting lanes can still starve one another,
but each critical section is now milliseconds, not a download); lane 3's per-lane staging-file
design remains the better shape if that ever bites. The outgoing conductor's session wrapper
(`register.sh`, scratchpad) is now redundant and was used only to finish this round's batch.
- `obs-servicenow-robots`: servicenow.com's robots.txt refuses the fetch tool; lane 10 respected
  it (a human can open the IRE page if the CMDB problem statement is wanted).

## what-each-lane-concluded (index only; the gather files carry the substance)

- lane-1 standards (CIM / YANG+NMDA / SNMP): counter-thesis +SURE for write-reach; identity
  minted per scope; CIM weak/propagated keys ≈ `:identified-in`; CIM namespace rule =
  unique-referent without unique-name; `Correlatable` removed in v3; NMDA + `origin` the closest
  two-form seam; IF-MIB ifIndex litigation the best battle-test record; YANG avoids positional
  identity by design.
- lane-2 assessment (OVAL / osquery / InSpec / Testinfra / Goss / Facter / ohai): counter-thesis
  +SURE (pure reads; no interference); OVAL's three-layer absence partition; the 2025 OVAL-board
  `chainlink` proposal ≈ the emitted traversal; OpenSCAP enumerate-then-match gets `is-active`
  right; tools disagree on what a path names.
- lane-3 CM big four: identity = string key per type; Puppet concedes the strangers case and the
  path-is-not-a-referent case in its tracker; isomorphism flag = "equal names license nothing";
  autorequire = the one type-author cross-type table (create-order only); RFC repos silent.
- lane-4 schema systems (CFEngine / NixOS / Guix / DSC / Quattor / Bcfg2 / inmanta / Augeas): the
  Nix store as the most precise abstract identity model; "two entries are one thing" rulings
  narrow and local (NixOS `/etc` target, Guix refuse, DSC `IsSingleInstance`, inmanta `index`);
  Augeas/CFEngine escape positional identity via content identity.
- lane-5a verification (CoLiS / Rehearsal / Tortoise / Citac / FSMove / SibylFS / Ntzik–Gardner /
  TxOS / GRoot / Mancoosi): counter-thesis holds for every verify/test/repair project (path-keyed,
  aliasing assumed away); SibylFS the one non-punt; Ntzik–Gardner the closest twin of §2.9/§3.3;
  Mancoosi an unseeded find.
- lane-5b theory (Burgess / Couch / Traugott / Anderson / Desai / Delaet / Prodspec / ConfSolve):
  identity presupposed by every operator algebra; Burgess & Couch define identity as accessor
  equality; Couch 2003's "problem of referents" punt; promise theory = committee law but assumes
  away §0; multi-author handled by separation everywhere; Delaet's framework has no identity row.
- lane-6 platform fs/packages/installers: RFC 8881 spells both warrants as separate promises
  (`unique_handles`); POSIX "at any given time"; overlayfs's own refutation of stable inodes;
  dpkg triggers match by spelling; MSI component rules as a cross-vendor naming contract; OCI's
  several identifiers per image.
- lane-7 platform services/names: Solaris FMRI logical/universal schemes with authority; RFC 4007
  zones (manual assignment); RFC 2181 splits the two warrants as 311 does; RFC 9499 split DNS
  contradicts the "DNS MRoot" EXAMPLE; kernel pid/uid maps as transition-owner `:corresponds`;
  wrappers write down their lends; systemd `is-active` rc 4 but still prints `inactive`; SMF
  "in-conflict" the one deployed refuse-both.
- lane-8 API/IaC RFC repos: no gold-mine; single minting authority everywhere; Crossplane
  `external-name` the one two-authority meeting (fails closed; per-type provider warrant); RFC
  3986 §6 = 311's asymmetry; CPE 2.3 = the refuted shape deployed at NVD with UNKNOWN removed;
  k8s SSA = committee law with attribution; Terraform unknowns = monotone refinement; Repology =
  human curation at scale.
- lane-9 naming classics: Saltzer & Kaashoek state the two uniqueness rules as independent
  per-scheme choices, neither default; OntoClean's sufficient/necessary criteria and
  supplies/carries; Kent's aliases-vs-ambiguity and his case AGAINST qualified identification;
  counter-thesis SPLIT (thin interference models exist: Lampson's aliasing axis and GNS
  arc-bounded caches, V's on-use staleness ≈ `witness()`); no write-set or region model anywhere;
  the lane's mapping table lists KNOWN_UNSPOKEN, may-read/may-write, region test, `:places` as
  GAPS.
- lane-10 wildcard: DEP-17 the strongest punt; SMI-S Clause 7 the nearest deployed `compare()`;
  RETRO/BackTracker the nearest committee-law + traversal; CMDBf/X.720/SACM/ProvMark punts;
  seams to redraw: read X.720 before CIM; DEP-17 beside dpkg; add a security/provenance seam and
  a trace-based build-systems seam; SMI-S port naming for the network seam.

## leads-needing-the-human (aggregate; each lane's file has the detail)

- Couch & Sun 2004 "On observed reproducibility in network configuration management" (SCP,
  paywalled) — lane 5b's best unpulled lead (observed vs actual state).
- Comer & Peterson 1989 / the 1984 Purdue precursor (403 to curl) · Needham "Names" (Mullender
  ch.) · Watson 1981 · Mann's Stanford PhD 1987 — lane 9.
- Chen et al. FSE 2020 "Understanding and Discovering Software Configuration Dependencies"
  (cs.cornell.edu PDF) · CfgNet TSE 2023 — lane 10; may-read/entailment territory.
- Burgess DSOM 2005 · Vanbrabant IM 2013 (dl.ifip.org was down) · Chiarini's dissertation — lane 5b.
- Ntzik et al. ECOOP 2018 · the Ntzik–Gardner tech report with full symlink rules · BuildFS
  OOPSLA 2020 · Lepiller TACAS 2021 — lane 5a.
- Puppet's Felix Frank "constraints" thread (Google Groups) — lane 3.
- MultiarchSpec (wiki.ubuntu.com 504) · T10 VPD 0x83 / multipath `uid_attribute` ·
  `sharedsubtree.rst` — lane 6.
- SELinux refpolicy `.fc`/`.if` as a footprint corpus (needs a dedicated pass, not a man-page
  read) — lanes 7 and 10; a NEW front, hence deferred by ruling.
- ServiceNow IRE docs (robots.txt) · 3GPP TS 32.300 (ETSI 403) — lane 10.

## the-humans-open-asks (posed in chat; unanswered at hand-down)

- `ask-reading-lane-choice`: main-context scoped reads vs a clean-context Fable vs both vs
  neither — now moot in form (the successor IS the clean context) but the question of WHICH
  primaries get a full read stands. The outgoing conductor's unranked candidates, by how much of
  311 each lane says they cover: SMI-S Clause 7 (~12 printed pages); RFC 8881 §4 plus the
  `unique_handles` attribute text; Ntzik & Gardner 2015; RETRO §§4–5; then the long ones
  (Saltzer & Kaashoek ch.2 §2.2 + ch.3; RFC 8342; DSP0004's identity clauses; OntoClean 2000).
- `ask-contradiction-sitting-next`: whether to sit on the deployed CONTRADICTIONS the lanes
  surfaced (collated in `cross-lane-observations`) before any reads.
- `ask-human-fetches`: which of the leads above to fetch out-of-band.
- `ask-txos-duplicate`: keep both entries or note one superseded.
- `ask-tooling-repair`: the three defects above; the outgoing conductor offered to draft the patch.

## notes-for-the-successor (mechanics, not synthesis)

- 311 moved under this round: read at `ae745a80`, then five commits landed (`63e49f29`) retiring
  `:hierarchical` for the lookup-EMITTED MTraversal and defining "touches a traversal member" as
  any `compare()` answer other than DISJOINT; every lane read `63e49f29`; a sibling session kept
  editing 311 afterwards. Run `git log --oneline -- Research/notes/311*.md` before citing a
  section number.
- Suggested read order for the gather files, by yield: 10, 9, 5a, 6, 7, 1, 8, 2, 3, 4, 5b — or
  the turn notes' Findings first, which cover all ten in ~130 bullets.
- The lanes were told to judge each candidate on (a) breadth against the 47 `311r` items,
  (b) 311 analogues by 311's own vocabulary, (c) how it was battle-tested, (d) explicit punts,
  (e) altitude; their candidate tables carry those columns.
- Crosscheck skepticism applies: every grade is a subagent's on a single read; the outgoing
  conductor re-read none. Lane 9's "verbatim" claim about Saltzer & Kaashoek p.64 is ~SUSPECT as
  "verbatim" (the quote states the two rules as alternatives across schemes, which implies
  independence rather than stating it); RFC 8881's `unique_handles` is the stronger precedent for
  independent, absent-by-default warrants.
- The skill's `references/after-first-pass.md` → `narrow-and-regrade.md` govern what comes next:
  the skill's plan gate with the human, then fronts worked serially. The human has already said
  the serial discipline is the point.

## cross-lane-observations (the outgoing conductor's, made while reading the ten reports; not lane claims; the only content salvaged from the removed `plan.md`)

- Ten independent lanes each confirmed the counter-thesis: no project models ops-extern identity
  abstractly and broadly across mutually-unknowing authors. Lane 10's four punt-shapes — (i) the
  name is the thing, (ii) one minting authority, (iii) delegated to humans or heuristics, (iv) one
  domain only — recur in every other lane's evidence, which is corroboration lane 10 could not see.
- Four 311 objects had NO precedent in any lane's mapping (lane 9's table names the first three
  as gaps; the fourth is the conductor's collation): KNOWN_UNSPOKEN as distinct from UNKNOWN; the
  may-write entailment plus the finished definition as an authored, closed writeset; the region
  test and `:places`; `:aliases-nothing-else` as a store's self-knowledge. ~SUSPECT on the last:
  RFC 8881's `unique_handles` (lane 6) is a server's statement about its own store and may be
  exactly that precedent — a reason to read RFC 8881 §4 first.
- Deployed prior art that CONTRADICTS a 311 rule or example, collated from the lanes for a
  possible sitting: CPE 2.3 name matching (unequal strings ⇒ DISJOINT; UNKNOWN removed; live at
  NVD — lane 8, evidence FOR 311); OntoClean's "every domain element must instantiate some
  property carrying an IC" vs 311's absent-by-default warrants (lane 9); Kent against qualified
  identification unless qualifiers are invariant vs 311's MParent-Store-relative primary keys
  (lane 9; ~SUSPECT a real tension); Couch & Chiarini's "declared consistency is intractable,
  observe instead" vs 311's speech-declared footprints (lane 5b); YANG's designed avoidance of
  positional identity vs §2.9's positional catalogs, and DEP-17's "eliminate the aliases" as the
  same move (lanes 1, 10); RFC 9499 split DNS vs the "DNS MRoot" EXAMPLE in §1.6 (lane 7;
  example-level); promise theory's "overriding control" assuming away §0's premise (lane 5b).
- Lane 9's "verbatim" for Saltzer & Kaashoek p.64 overstates: the quote presents the two
  uniqueness rules as alternatives across schemes, which implies their independence rather than
  stating it; RFC 8881's separate MUST/SHOULD/`unique_handles` is the stronger precedent for two
  independent, absent-by-default warrants.
- A footprint-corpus seam (SELinux refpolicy `.fc`/`.if`, AppArmor abstractions, pledge/unveil,
  PaSh annotations) was flagged by lanes 7 and 10 and by the conductor as bearing on §2.5/§2.6
  rather than identity; it is a NEW front and was not opened, per the human's ruling.

## update-line

- 2026-09-26 (hand-down written): registration batch running; lanes 3, 6, 7 resumed for
  citations only.
- 2026-09-26 (closing): every lane's entries registered — `sources.json` holds 255 entries and
  `sources/` 255 archived copies, one-to-one; `validate.sh` reports zero unregistered citations
  and 16 warnings, all bare slugs inside lanes' Tooling-problems inventories (left as inventories,
  not citations). Lane 3 filled its Citations from local copies (29 slugs; eight cited by phrase;
  it corrected several line numbers from its hand-back); lane 6's citation pass ran in a FRESH
  context (its own had not survived), rebuilt the candidate table from the entry JSONs, cited four
  sources by section only, and found one misattribution in the banked Findings (fixed in place with
  a note); lane 7 resolved all 72 "archive line pending" markers to real lines (95 quotes from 33
  sources; one GitHub comment quoted from the `gh` API because the archived HTML lazy-loads
  comments; the rc-3-vs-rc-4 claim rests on a PR diff with no quotable line). Lane 2's six
  `L{…}` placeholders remain. The Plan 9 overview needed four download attempts (cat-v.org TLS
  flake on Windows). No new fronts were opened after the human's ruling. Closing commit: the
  next `(AI new dsn)` commit touching only this directory, after this line.
