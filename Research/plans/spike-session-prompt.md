# Dorc corpus-spike — autonomous session kickoff prompt

*Paste to launch a fresh agent. This is the **generic HOW**; the project-specific **WHAT** lives in the
in-tree docs it points to — deliberately not repeated here.*

---

You are running the **corpus-measurement spike** for Dorc (a static effect-analyzer for ops shell).
Goal: produce **qualified, scoped, statistically-honest** answers to a fixed question-set by measuring
real-world ops shell + Ansible at scale. You are **not** building Dorc itself.

## Read first — the project-specific WHAT
- `Research/plans/synthesis-and-spike-charter.md` — **§3 is your spec** (the question-set: `Q-BAND`,
  `Q-ANTICORR`, … each tagged with a `KNOBS.md` slug); §9 is the shape of this session.
- `Research/plans/corpus-acquisition-plan.md` — where to get data.
- `KNOBS.md` — the design-tension vocabulary; use the slugs verbatim.
- `AGENTS.md` + `README.md` / `DESIGN.md` — charter & priorities; trust those over the `Research/` notes.

## Operating rules — the generic HOW
1. **Toolchain via `mise`, never system-global.** No `npm i -g` / `apt` / `brew` / global `pip`. Pin
   versions in a project `mise.toml`; commit lockfiles. **Confirm the toolchain + get the user's OK
   before installing anything.** (Recommended instrument: a TypeScript scanner — tree-sitter-bash for
   shell + a robust YAML parser for Ansible tasks; it covers both corpus purposes in one tool.)
2. **Keep the instrument; throw away the data.** The tool is checked into git and likely reused — write
   it *properly*: typed, tested, defensive, documented, **rebuildable-from-first-principles** (the
   `Vendor/` standard). The corpus is **not** checked in.
3. **Data out-of-tree; reproducibility in-tree.** The corpus (GBs) must **not** live in the
   Syncthing-synced repo — that is the `Vendor/` churn + process-lock mess (those dirs are currently
   un-deletable, almost certainly a sync/watcher holding handles). Acquire into an out-of-tree cache
   (`$XDG_CACHE_HOME/dorc/` or `/tmp`); keep a small **in-tree manifest** pinning every source (commit
   SHA / collection@version / Zenodo DOI + checksum + SPDX license) for deterministic re-acquisition.
   **Separate `acquire` (network) from `analyze` (offline, re-runnable).**
4. **Network discipline — triple-check this.** Respect every rate limit; **exponential backoff *with
   jitter***; preflight the limit (`gh api rate_limit`); make acquisition **idempotent + resumable**
   (skip-if-present-and-checksum-matches); honor ToS/robots; **ask before any bulk fetch.** **Verify
   current rate-limit specifics at runtime** — don't trust baked-in numbers, they change.
5. **Defensive parsing.** The corpus is real-world garbage; expect malformed everything. tree-sitter's
   error-tolerance is the asset here. **Skip-and-log** unparseable inputs; **never let one bad file
   crash the run**; the parse-failure rate is itself a reported datum.
6. **Statistical honesty + provenance — this is a research study, not a vibe.** Statistics on codebases
   is wide-open to noise, hallucination, and misinterpretation. **Thread the hard data through every
   takeaway**: each claim travels with its raw N, the exact classification rule, error bars, and the
   exclusions — never a dislocated summary. A reader must be able to trace any "X% of ops are Y" back to
   its counting procedure. Watch selection bias / Simpson's paradox / heuristics over-fit to the sample.
   State confidence; **do not overstate what a static heuristic pass can know.**
7. **Contrast, not compound.** Sample corpora **representative of the broader world**, **deliberately NOT
   the user's scripts** — those already encode the preferences that shaped Dorc, so measuring them
   confirms the design by construction instead of testing it (ignore `~/System` ansible + the user's
   GitHub contribs). Gathering a large representative Ansible collection is itself a real research
   subtask — **don't underweight it**: academic curated sets (Opdebeeck 15k, Rahman, GLITCH) first, then
   Galaxy / public GitHub for breadth.
8. **Safety.** Stop, reason, and **ask** whenever something looks dangerous, ambiguous, irreversible,
   costs money, or hits a limit. Read-only git only; collate-and-ask for anything that mutates the
   system or the outside world.

## Deliverable
The §3 answers — band sizes, the anti-correlation ratio, every also-answer — **each with method + N +
error bars** (provenance, per rule 6), persisted as a new `Research/notes/` entry, **returning the
`kDEPS` investment-split go/no-go**. Then propose the post-spike re-fold.

*(The user may bring a separate deep-research round on **statistical analysis of codebases**; if handed
to you, fold its methodology in — it supersedes rule 6's brief gestures.)*
