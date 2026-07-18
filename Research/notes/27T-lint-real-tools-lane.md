# 27T — `dorc lint` OPT-IN real-external-tools test lane: as-built landing note

AI-authored (Opus implementor under the Fable conductor, 2026-07-18, worktree
`agent-a2b75633e2d7e89e3`, branch `ai/r27-lint-realtools` off `245afe0` — the `ai/r27-lint-build`
tip). Sibling of `27R` (plan-of-record), `27Ra` (prior-art digest, areas 2-3 = the
shellcheck/checkbashisms interface facts this lane leans on), and `27S` (the sketch landing whose
§6 never-run-against-real-tools caveat THIS lane exists to discharge). Append-only; never edited in
place. This is the as-built ledger, the live-fire results, the default-path zero-invocation proof,
tc-* flags, seams, and confidence markers.

## §1 — What this discharges

`27S` §6 stated plainly: "the adapters have never been run against real shellcheck/checkbashisms
output; the tolerant-text tier and json1 shapes are modeled from `27Ra`'s manpage reads, not live
runs." This lane is the live fire. Headline result: **the shellcheck json1 adapter and the strip
line-map remap were CORRECT ON FIRST CONTACT — zero adapter fixes needed.** The `27Ra` manpage
model of json1 (`comments[]` with `line`/`column`/`level`/`code`/`message`) and the
`strip_file_with_map` remap both matched real `shellcheck 0.11.0` output exactly. (NEVER-vouch: this
is process-evidence from my own live runs on one box + one tool version, not a proof of correctness
across platforms/versions. The lane is version-TOLERANT by construction so it keeps earning its keep.)

## §2 — As-built ledger (what landed where)

Strictly-additive; NO Rust touched (no adapter bug ⇒ no code fix); NO existing default behavior
changed. Files:

- **`mise.toml`** (repo-root; the named "repo mise config file") — pinned `shellcheck = "0.11.0"`
  EXACT under a comment marking it opt-in-lane-only + INERT for every default gate/suite (registry
  route `aqua:koalaman/shellcheck`). Does not touch the rust toolchain or any existing gate.
- **`spike/e2e/lint-real-cases/shellcheck/book.sh`** + **`.../checkbashisms/book.sh`** — two MARKED
  dorc-lang oracle fixtures. Each strips a whole marker line (orig 2) + a bare-mark `invariant:` line
  (orig 5) — a TWO-line shift — leaving the tool-triggering command at stripped line 4 / ORIGINAL
  line 6, so the strip line-map remap is exercised against real tool output. shellcheck fixture: an
  unquoted `$1` (SC2086-class, an `info`-level, rock-stable code). checkbashisms fixture: `echo -e`
  (a bashism).
- **`spike/e2e/lint-real-tools-setup.sh`** — provisions checkbashisms (task fallback (b)): a
  version-PINNED download (devscripts git TAG `v2.23.7`, immutable) sha256-verified into the
  git-ignored `spike/e2e/.real-tools/`; GPL perl body NEVER vendored into the tracked tree.
  Idempotent (sha256-match ⇒ skip download). Platform-split launcher (§4). Invoked ONLY by the lane.
- **`spike/.gitignore`** — `/e2e/.real-tools/` (regenerated on demand; never committed).
- **`spike/e2e/run.sh`** — the OPT-IN lane: a strictly-additive block AFTER the untouched `cases/*/`
  and `lint-cases/*/` loops, before the final tally. Gated ENTIRELY on `DORC_E2E_REAL_TOOLS` (comma
  tool-list). Per listed tool: locate/provision → prepend to a lane-local PATH → run
  `dorc lint --format=jsonl --source <tool> --require-tools book.sh` from inside the case dir →
  assert STRUCTURAL properties (below). Header documents the one-line invocation.
- **`spike/CLAUDE.md`** — the `real-tools-lane-opt-in` law bullet (conductor-supplied verbatim) under
  Build / test / run.

Mechanism choice (`tc-real-tools-lane-mechanism`): an env-gated shell section in `e2e/run.sh` +
fixture cases, NOT `#[ignore]` Rust integration tests. Rationale: the env-var LIST is itself the
coverage assertion (`27R` §8b) and the `E2E` prefix + "document the invocation in run.sh's header"
both point at run.sh; the shell lane drives the REAL cli-edge `SubprocessRunner` end-to-end (the
actual non-hermetic wrapping), and structural greps stay version-tolerant where a golden diff could
not.

## §3 — What the cases assert (structural only; never message text or counts)

Per `27R` §4 agility doctrine applied to our own tests. For each tool, on the JSONL output:
(a) `dorc` exits 0 (findings present, below the `error` threshold) — a LISTED-but-absent tool instead
exits 3 (`--require-tools`), which the lane turns into a LOUD fail; (b) coverage reports the tool
`"status":"ran"`; (c/d) a finding at the ORIGINAL pre-strip **line 6** (proving remap — the tool saw
it at stripped line 4), naming the ORIGINAL path (`book.sh`, never a temp path), carrying the stable
CODE and the expected adapter TIER's fidelity: shellcheck ⇒ `code":"SC2086","remap":"exact"` (json1
MACHINE tier — if real json1 had fallen through to the tolerant-text/raw tier the code would be
`external-text`/`external-raw` and the assert FAILS, which is exactly the wrapping-bug this lane
catches); checkbashisms ⇒ `code":"external-text","remap":"approximate"` (tolerant TEXT tier — no
machine format exists). NEVER golden-pinned: message text, col value, finding counts, incidental
findings.

## §4 — Tool versions + provisioning route + live-fire results

- **shellcheck 0.11.0** — mise `aqua:koalaman/shellcheck`, native `.exe`, spawns cleanly from dorc's
  `Command::new` everywhere. LIVE-FIRED END-TO-END on Windows through `dorc lint --format=jsonl`:
  ```
  {"path":"book.sh","line":6,"col":19,"severity":"info","source":"shellcheck","code":"SC2086",
   "message":"Double quote to prevent globbing and word splitting.","remap":"exact"}
  ```
  json1 parsed at the MACHINE tier on FIRST CONTACT; remap `exact`; line 6 = the ORIGINAL (stripped
  line 4 remapped through `line_map = [1,3,4,6,7]`). +SURE the shellcheck adapter + remap are correct
  on this version/platform. No adapter fix.
- **checkbashisms v2.23.7** — devscripts perl script, absent from every mise backend; fetched pinned
  (sha256 `ef3e95808899dda7d5dfd53dc7e1f6138ee44ecc6aa0f98e51b9d449fe54bbe2`) per fallback (b). Of the
  tags I probed only `v2.23.7` resolved on salsa (`v2.24.0`/`v2.25.0` 404 — the tag scheme differs);
  pinned that. VERIFIED (each link, on Windows): `checkbashisms.pl --lint -` reads stdin and emits
  `(stdin):4:1: warning: possible bashism; echo -e` (rc 1); the adapter's `parse_text_line`
  (`splitn(4, ':')`) lowers that to line 4 / col 1 / `external-text` / `Approximate`, and the remap
  maps stripped line 4 → original line 6. Every link proven; the ONLY unproven step on *nix is the
  standard shebang `execve` of the launcher.
  - **Windows spawn block** (`tc-checkbashisms-win-spawn`): dorc could NOT be driven end-to-end
    against checkbashisms on this box. Root cause — a genuine discovery-vs-spawn mismatch the lane
    SURFACED: dorc's `tool_on_path` finds a `checkbashisms.cmd` via PATHEXT (reports available), but
    the cli-edge `SubprocessRunner`'s `Command::new("checkbashisms")` only appends `.exe` (Rust/
    Windows behavior) and returns NotFound ("program not found") for a `.cmd`; forcing the `.cmd`
    through cmd.exe additionally HANGS on piped stdin (cmd.exe goes interactive). So on Windows the
    setup writes NO PATH-discoverable launcher (only `.pl`, and `.PL ∉ PATHEXT`): dorc sees the tool
    cleanly ABSENT and a `DORC_E2E_REAL_TOOLS=...,checkbashisms` run FAILS LOUDLY-AND-FAST via
    `--require-tools` (rc 3) rather than hanging. On *nix the setup writes an EXTENSIONLESS launcher
    (`exec perl .../checkbashisms.pl "$@"`) that is both PATH-discoverable (ext="") and execve-able
    with native stdin — checkbashisms runs LIVE there by design. Disposition = task fallback (c):
    shellcheck landed fully live; checkbashisms correct-by-design + *nix-live, Windows-documented-only.

Provisioning `.cmd`-launcher note: an absolute-perl-path `checkbashisms.cmd` DOES work when spawned
from a native Windows context (verified from PowerShell); it is only dorc's `Command::new` PATH
resolution + cmd.exe stdin that break it. Recorded as `seam-runner-pathext-spawn` (§7).

## §5 — Default-path zero-invocation proof

Two independent legs:
1. **Code inspection (the gate).** The lane is one `if [ -n "${DORC_E2E_REAL_TOOLS:-}" ]; then … fi`
   block; UNSET ⇒ the block is inert (zero external spawns, zero real-tool PATH probes). Independently,
   the PRE-EXISTING default `lint-cases/*/` loop runs each case under `env PATH="$_lint_empty"` (a
   fresh EMPTY dir), so those cases probe an empty PATH and report the tools deterministically ABSENT
   regardless of the host. The round-trip `cases/*/` loop never invokes `dorc lint`. The default unit
   tier uses a fake runner (`crates/lint/tests/`), and there is NO `crates/cli/tests/` — so
   `cargo test --workspace` spawns no external linter (the only test `Command::new` is `git` in
   `core/tests/diag_tidy.rs`).
2. **Poisoned-PATH live proof.** Real `shellcheck` is now reachable on the ambient PATH (the mise
   shim `…/mise/shims/shellcheck`), yet a default `sh e2e/run.sh` stays 94-green with the three
   lint-cases reporting `tool-absent` — the PATH-scrub holds even with a real tool one dir away.
   Belt-and-suspenders: a default `sh e2e/run.sh` with poison `shellcheck`/`checkbashisms` stubs
   (each `touch`es a sentinel + screams if executed) PREPENDED to PATH finished 94-GREEN with the
   sentinel ABSENT and zero `POISONED` leakage — zero external-tool invocations in the default path,
   confirmed live even with a real shellcheck one PATH-dir away.

Byte-stability of the default e2e is thus preserved by DESIGN (env gate + pre-existing PATH-scrub),
independent of whether the pinned tools are installed.

## §6 — tc-* flags (conservative leans; the human/conductor re-rules cheaply)

- **tc-real-tools-lane-mechanism** — env-gated shell lane in `e2e/run.sh` over `#[ignore]` Rust
  integration tests (§2 rationale). If a future round wants the adapter parse exercised WITHOUT the
  cli round-trip, an `#[ignore]` test in a new `crates/cli/tests/` reading the same env gate is the
  alternative; the shell lane was chosen for end-to-end fidelity + the env-list-as-coverage fit.
- **tc-checkbashisms-win-spawn** — checkbashisms is *nix-live-only for the lane; on Windows it is
  documented-not-live (dorc `Command::new` can't spawn a perl-script launcher). List only
  `shellcheck` on Windows. NOT a correctness compromise — the tool is cleanly absent (loud fail), not
  silently skipped.
- **tc-checkbashisms-pin-tag** — pinned to devscripts tag `v2.23.7` (the only probed tag that
  resolved) + sha256, not "latest". A newer checkbashisms is a one-line + one-hash bump; the lane's
  structural asserts (external-text / approximate / line 6) are version-tolerant so a bump should not
  churn them.

## §7 — Seams (named, not built)

- **seam-runner-pathext-spawn** — teach the cli-edge `SubprocessRunner` to spawn PATHEXT scripts
  (`.cmd`/`.bat`, via cmd.exe with correct stdin + CVE-2024-24576-safe arg escaping) so `tool_on_path`
  and `Command::new` agree on Windows. Discharging this makes Windows checkbashisms live AND resolves
  `27S`'s `tc-lint-e2e-stub-tools-spawn` / `seam-lint-real-tool-spawn-e2e` (this lane already
  discharged the shellcheck half of the latter). Out of THIS lane's scope (a cli-runner change beyond
  the lint crate; the primary target shellcheck works).
- **seam-real-tools-nix-ci-run** — actually EXECUTE this lane on a *nix runner to convert the
  checkbashisms correct-by-design claim into live evidence (every link is verified here except the
  standard *nix execve).

## §8 — Verification + notes for the conductor

- Comment budget: **0** added inline `//` lines (`git diff 245afe0..HEAD -- spike | grep -E '^\+' |
  grep -cE '^\+[[:space:]]*//([^/!]|$)'`) — the lane is all shell `#`, no Rust touched (no adapter bug).
- Four gates GREEN at tip: `cargo fmt --check` · `clippy --workspace --all-targets -D warnings` ·
  `cargo deny check licenses bans sources` · `typos spike`. Baseline `cargo test --workspace` = 904
  pass / 1 pre-existing ignored — UNCHANGED (no Rust added). Default `sh e2e/run.sh` = 94-green
  (unchanged); opt-in `DORC_E2E_REAL_TOOLS=shellcheck` = 95-green (`ok  lint-real/shellcheck`).
- The lane, live: `DORC_E2E_REAL_TOOLS=shellcheck sh e2e/run.sh` ⇒ `ok  lint-real/shellcheck`, tally
  `all 95 … passed` (Windows, this box). `DORC_E2E_REAL_TOOLS=shellcheck,checkbashisms` on Windows ⇒
  a LOUD `FAIL lint-real/checkbashisms` (rc-3 absent) — the correct listed-but-unrunnable behavior.
- SUITE SPEED: the default e2e is ~4.5 min on this Windows/msys box (process-spawn heavy); budget
  generous timeouts, prefer background runs. Not a lane concern (the lane adds ~1 dorc invocation
  per listed tool).
- NEVER-vouch: everything above is my own process-evidence on ONE box + ONE version of each tool.
  The lane's value is that it is opt-in + version-tolerant, so a real human/CI can keep re-firing it.
