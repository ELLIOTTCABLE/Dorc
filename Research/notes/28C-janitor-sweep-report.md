> Conductor note (Fable, 2026-07-19): verbatim janitor-agent report, banked for human
> adjudication of the dropped-work findings. Conductor grading of finding C: CONFIRMED
> substantive — de22017 is a human-reviewed 2026-07-17 block-settle re-grade of
> ANALYZER-NEEDS (new section P; ~20 row updates) absent from ai/main; union-merge is chartered into the round-28 docs pass.

# Dorc repo janitor report — 2026-07-19

Primary checkout: `C:/Users/ec/Sync/Code/Dorc`, HEAD `ai/main` @ `40f8005` (unchanged).
Round 28 is LIVE during this run (agents committing to `ai/r28-impl`); everything r28, spike3-r23/25/26/27, and every worktree dated on/after 2026-07-18 was left untouched.

## Counts
- Worktrees removed: **15** (all plain `git worktree remove`, no `--force`)
- Branches deleted: **34** (all via self-protecting `git branch -d`; each proved merged into `ai/main`)
- Worktrees kept: 47 remain (was 62)
- Branches kept: 89 remain (was 123)
- Attempts refused by git: **3** (merged-but-dirty worktrees; left intact — see Refusals)

---

## 1. DROPPED-WORK FINDINGS (headline — nothing here was deleted)

"Genuinely absent" = the commit's patch-id is not present in `ai/main` (via `git cherry`), and the added file paths do not exist at `ai/main` HEAD. All the branches below are RETAINED. Every note listed still lives on the named branch; it is only missing from `ai/main`.

### A. Language-design research ledgers — entirely absent from ai/main
- **`ai/24Ka-langreview`** (10 absent commits; worktree `agent-a5bf93021f501c8fc` retained). A full "24Ka" language-design review arc: a pre-corpus GATE of 24 pre-registered lessons, ergonomics/shell-compat/regrets gatherers, corpus observations, and a 185-vs-4 census. Adds `Research/notes/24Ka-langreview-0X-*.md` (e.g. `-05-findings.md`, `-06-census.md`) — none present at `ai/main`.
  - Guess: a parked language-review lineage whose research notes were never folded into `ai/main`.
- **`worktree-agent-abd7ff8be88067e1b`** (4 absent commits; worktree `agent-abd7ff8be88067e1b` retained). The sibling "24Kb" ledger: skill-up source ledger (12 primaries), a GATE of 22 pre-registered language-design lessons, 4 merged gatherer ledgers, corpus observations + 8 kill-attempted findings. Adds `Research/notes/24Kb-01/03/04-*.md` — none at `ai/main`.
  - Guess: same as 24Ka — a distinct research ledger that never merged.

### B. Crosscheck review reports — only the distilled adjudication survived in ai/main
At `ai/main` the ONLY crosscheck note present is `Research/notes/279f-crosscheck-adjudication.md`. Every individual reviewer report/working-note (279a-e and 280a-e) is absent from `ai/main`. This may be intentional (the 279f adjudication is the distilled product; the raw reviewer reports are working artifacts) — flagged per instruction, not deleted.
- **279-series** (270-era block-settle / 27xxx-language-v0.1 reviews) lives on:
  - `ai/spike3-r23-crosscheck-reports` (4 absent: 279a Fable-a, 279a/b sol-n, 279d neutral-rescued, 279e ds-worker) — worktree `crosscheck-reports` retained.
  - duplicated on retained branches: `ai/spike3-r23-fable-review-a` (279a), `ai/spike3-r23-deepseek-n-review` (279d), `worktree-agent-a98db75de94f70d9e` (sol-n), `worktree-agent-aefb47d2496c17ed7` (279e red-team), `worktree-agent-af4574dec910838b6` (279e ds-worker).
- **280-series** (post-crosscheck repair-layer reviews) lives on the KEEP `ai/r28-xcheck-*` branches: 280a Fable-A (`ai/r28-xcheck-fable`), 280b sol-neutral (`sol-n`), 280c sol-adversarial (`sol-a`), 280d deepseek (`ds-n`, `ds-a`), 280e red-team report (`ds-a`). Adds `Research/notes/280a..280e-*.md` — none at `ai/main`.

### C. ANALYZER-NEEDS.md revision — absent from ai/main
- **`de22017` "(fix doc) Review and include new ANALYZER-NEEDS"** reworks `ANALYZER-NEEDS.md` (+62/-36). It is the shared base commit of `ai/r27-review-base` AND all five `ai/r28-xcheck-*` branches, and is genuinely absent from `ai/main` (main's `ANALYZER-NEEDS.md` evolved on its own line).
  - Guess: a review-branch snapshot of ANALYZER-NEEDS superseded by main's own edits. Worth a human `git diff ai/main de22017 -- ANALYZER-NEEDS.md` to confirm no unique content was lost. Held on `ai/r27-review-base` (retained).

### D. Round-22 adversarial gate spikes — absent, historical
- **`ai/r22-x3a`** (6 absent). PoC-B/C/D/E demonstrating tidy-gate / reachability-gate / retire-guard vacuousness; each PoC is paired with a `Revert` (net-zero code — the commits are the evidence-of-weakness record).
- **`ai/r22-x3fix`** (1 absent): "check lift_failure onto typed Diag path; severity from registry not hardcoded".
- **`ai/r22-xcheck1`** (1 absent): "evidence the erasability gate is witness-blind by omission".
  - Guess: round-22 red-team spikes; the findings were likely absorbed into later design, but these branches/commits never merged. Low urgency (superseded lineage).

### E. Old parked snapshots / misc orphans — absent
- **`ai/snapshot`** (2 absent): round-13 "platform-compat kLANG weld + sh-precondition" (`ca3ed03`) and round-14 "controller-host transport prior-art + executorless-OOB plan" (`704b4e3`). Ancient design notes.
- **`worktree-bridge-cse_01WtJZdmgzSuVfM1BTXUejRL`** (1 absent): same round-13 kLANG weld `ca3ed03`.
- **`worktree-agent-a0d05c180ac5f25c4`** (1 absent): `eb8e69c` "(AI fix ana) plan-time wall: a running modeled mutator invalidates downstream elides (23Ib-fd10)". An analyzer fix on an orphan branch — possibly superseded; worth a glance.
- **`worktree-agent-a3da0cd4a13a6d3bc`** (1 absent, 4 patch-equiv): `d5d1178` "(new doc) Seed the docs on collab/type/state". A doc-seed.

### F. Uncommitted content in RETAINED worktrees (never committed anywhere)
- **`agent-af67e0c672b0f437e`** (branch `worktree-agent-af67e0c672b0f437e`, merged): 4 untracked review notes + a `.claude-commit` sentinel:
  `Research/notes/279a-review-working-sol-n.md`, `279a-review-worklog-sol-n.md`, `279a2-review-evidence-sol-n.md`, `279b-review-report-sol-n.md`.
  `279a2-review-evidence-sol-n.md` appears unique (not among committed 279 files); the others may duplicate committed 279 content. This worktree was NOT removed (dirty).
- **`agent-a3557130737d11c12`** (branch `worktree-agent-a3557130737d11c12`, patch-equiv): modified tracked `Research/trial/observe/recon.sh` (uncommitted). Retained.
- (Non-work artifacts, ignore) `agent-a4bc512f21f7ea336`: untracked codex runtime logs (`codex-stderr.log`, `codex-stdout.jsonl`, `codex.pid`, `sol-prompt.txt`). `agent-afbbdee7f1f2d5cf6`: untracked SyncThing conflict copy `spike/crates/analysis/src/effect.sync-conflict-*.rs`.

---

## 2. Worktrees removed (15) — all branch MERGED into ai/main + clean + dated 2026-07-16/17

| Worktree dir | Branch (also deleted) |
|---|---|
| agent-a028ad9c3c4c0ff8b | ai/r27-fallback-carry |
| agent-a03d6ca4bfb3027a1 | ai/r27-wrapper-peel |
| agent-a0415c83efd23bc3f | ai/r27-entity-algebra |
| agent-a0d2e409eb5013092 | ai/r27-value-recipe |
| agent-a21746f446ca2a81e | ai/r27-typeless-floor |
| agent-a662ef9ac5385739e | ai/r27-e2e-degraduation |
| agent-a69640ca9004b366f | ai/r27-context-entry |
| agent-a70b99e65feae7d12 | ai/r27-book-integration |
| agent-a72e0ac2a0fea2eb6 | ai/r27-payload-v1 |
| agent-a7a209a30a8ce3594 | ai/r27-shim-materialization |
| agent-a7d145e27456ce950 | ai/r27-wire-records |
| agent-a901c1f45fb7140e8 | ai/r27-backing-sets |
| agent-ac59888165e631da5 | ai/r27-raw-ship-repair |
| agent-aafdd5705830a35a6 | ai/r27-degrad-continuation |
| agent-a8d4363c5bae9343d | worktree-agent-a8d4363c5bae9343d |

## 3. Branches deleted (34) — each confirmed merged by `git branch -d`
14 r27 builder branches (the 14 `ai/r27-*` above) + 20 orphan `worktree-agent-<hash>` bookkeeping branches whose commits are all in `ai/main`:
`worktree-agent-` a028ad9c3c4c0ff8b, a03d6ca4bfb3027a1, a0415c83efd23bc3f, a0d2e409eb5013092, a1ad56460b4834c8a, a1c988c80d22ac4ec, a21746f446ca2a81e, a3d545c9bd8153315, a662ef9ac5385739e, a69640ca9004b366f, a70b99e65feae7d12, a72e0ac2a0fea2eb6, a7a209a30a8ce3594, a7d145e27456ce950, a8d4363c5bae9343d, a901c1f45fb7140e8, aafdd5705830a35a6, ac59888165e631da5, ae04ab44a8871d511, afbbdee7f1f2d5cf6.

## 4. Refusals (3) — merged-but-dirty worktrees, left intact (no --force)
Verbatim:
```
fatal: '.claude/worktrees/agent-afbbdee7f1f2d5cf6' contains modified or untracked files, use --force to delete it
fatal: '.claude/worktrees/agent-a4bc512f21f7ea336' contains modified or untracked files, use --force to delete it
fatal: '.claude/worktrees/agent-af67e0c672b0f437e' contains modified or untracked files, use --force to delete it
```
(branches `ai/r27-corpus-respell`, `worktree-agent-a4bc512f21f7ea336`, `worktree-agent-af67e0c672b0f437e` are all merged; their worktrees hold uncommitted files — see Finding F.)

## 5. Kept, with reason

### Keep-list (never eligible)
- `ai/main` (primary), `main` (default branch, at ai/main tip — out of scope), `ai/r27-aid` (folded conduct stack, == ai/main tip), `worktree-r28-impl`.
- All `ai/r28-*` (LIVE round): `errorloom-crate`, `impl`, `syntax-respell`, `xcheck-ds-a`, `xcheck-ds-n`, `xcheck-fable`, `xcheck-sol-a`, `xcheck-sol-n` (+ their worktrees; two locked).
- Spike lineage (frozen): `ai/spike3-r23`, `ai/spike3-r25`, `ai/spike3-r26`, `ai/spike3-r27` (+ worktrees). Whole `ai/spike3-r23-*` family held as lineage.
- Worktrees dated on/after 2026-07-18 (in-flight agents): all nine `ai/r27-aid-*` worktrees (caret, catalog, chain, docs, evidence, evidence-wiring, legacy-kill, spans, whylog) + the two locked r28 worktrees + `r28-impl`.

### Held as DROPPED WORK (retained, see section 1)
`ai/24Ka-langreview`, `ai/r22-x3a`, `ai/r22-x3fix`, `ai/r22-xcheck1`, `ai/r27-review-base`, `ai/snapshot`, `ai/spike3-r23-crosscheck-reports`, `ai/spike3-r23-deepseek-n-review`, `ai/spike3-r23-fable-review-a`, `worktree-agent-a0d05c180ac5f25c4`, `worktree-agent-a3da0cd4a13a6d3bc`, `worktree-agent-a98db75de94f70d9e`, `worktree-agent-abd7ff8be88067e1b`, `worktree-agent-aefb47d2496c17ed7`, `worktree-agent-af4574dec910838b6`, `worktree-bridge-cse_01WtJZdmgzSuVfM1BTXUejRL`.

### MERGED but held — deletable by human at will (`git branch -d` would succeed)
Held only because they are recent (dated 2026-07-18) or r28-live-adjacent, to avoid ref churn during the live round:
- `ai/r27-lint`, `worktree-r27-lint-conduct`, `worktree-r27-aid-conduct` (all == a merged conductor tip).
- 2026-07-18 orphan bookkeeping branches of the in-flight r27-aid worktrees: `worktree-agent-` a1b23057a14580815, a2b75633e2d7e89e3, a5fa5f18707f7420c, a6469ef1cf690aba2, a6a88cba398f83b37, a6b957b0956e52256, a96f36e4227c8a887, a9d8c7f25debbb150, aa2c4feb536c9de9f, aa6b039b2dcbd6e86, af0c579f2fe0366f4, af7aa41c1d3d5f740.
- 5 orphan bookkeeping branches whose hash matches a live r28-xcheck worktree (held for live-round caution): `worktree-agent-` a6711062a695b2aa8, a795e73d1b88039fe, a939dbbe55e25118c, ac1ba6aed5c4f15fa, ae5ab3fd66672b86e.
- Merged spike3-r23 family (held as lineage, not for merge state): `ai/spike3-r23-corpus-refresh`, `ai/spike3-r23-map-status`, `ai/spike3-r23-e2eaudit` (+ named worktrees `corpus-refresh`, `map-status`).

### PATCH-EQUIVALENT — content already in ai/main under different hashes; `git branch -d` REFUSES these (human may `git branch -D`)
- In-flight r27-aid worktree branches (all patch-equiv to ai/main, but in-flight — do not touch yet): `ai/r27-aid-caret` (-51), `-catalog` (-47), `-chain` (-38), `-docs` (-54), `-evidence` (-8), `-evidence-wiring` (-15), `-legacy-kill` (-4), `-spans` (-31), `-whylog` (-21).
- spike3-r23 family: `ai/spike3-r23-pipefix` (-12), `-pipeguard` (-3), `-polish` (-8).
- Still-checked-out orphan `worktree-agent-<hash>` branches (patch-equiv): a114297d42181633d (-1), a30e02699090c0f11 (-3), a3557130737d11c12 (-1, +dirty), a3e1507cb5d436261 (-1), a82d0e54f24d81b74 (-8), a9b47b632c92f27c7 (-1), ac967f2d644931d04 (-1), aedd8299ea686beda (-1), af0fc7569873de64d (-2), af7601b0efda2d845 (-1), and free `worktree-agent-a0618edd45959e557` (-2).

## 6. Sync-conflict inventory (SyncThing device PHNHRER — HUMAN-OWNED, nothing deleted)
Branches:
| Branch | Tip | vs ai/main |
|---|---|---|
| ai/main.sync-conflict-20260718-224729-PHNHRER | f8d8add | merged (ancestor) |
| ai/r27-corpus-respell.sync-conflict-20260717-015534-PHNHRER | 4b73521 | merged (ancestor) |
| ai/r27-aid-evidence.sync-conflict-20260718-234445-PHNHRER | 9109adf | patch-equiv (-8) |
| ai/r27-aid-legacy-kill.sync-conflict-20260718-224725-PHNHRER | 000c3fe | patch-equiv (-2) |
| ai/spike3-r26.sync-conflict-20260707-022301-PHNHRER | 87e74be | patch-equiv (-1) |
| ai/spike3-r26.sync-conflict-20260707-034339-PHNHRER | 4d040cb | patch-equiv (-4) |

None carry genuinely-absent commits. Sync-conflict FILE seen (untracked, in retained worktree agent-afbbdee): `spike/crates/analysis/src/effect.sync-conflict-20260717-015527-PHNHRER.rs`.

## Method notes
- Merge state via `git merge-base --is-ancestor` + `git cherry ai/main <branch>` (`+`=absent, `-`=patch-equiv).
- Absent-file confirmation via `git cat-file -e ai/main:<path>` against the paths added by each absent commit.
- Only `git worktree remove` (no --force), `git worktree prune`, `git branch -d` (merge-checked) were used. No push, no -D, no force, no tracked-file edits.
