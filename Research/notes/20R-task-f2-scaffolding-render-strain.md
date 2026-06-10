# 20R — task-F2: scaffolding-safe `render_apply` (the 20O find-2 fix); detection shape, the loop-span caveat, the refuse-the-license pin, and a cross-agent BLESS-contamination incident

> Round-20 spike note, append-only. Records task-F2 (the 20O find-2 disposition: line-granular
> Replace eats loop/`fi` scaffolding sharing its line ⇒ broken apply, partial host execution).
> The fix GENERALISES the T14 in-situ case-arm machinery (20G item-4) to all structural
> scaffolding. AI-authored, confidence-marked. Trust R/D/I/K + 19H/19I + the human rulings over
> this. Builds on 20O find-2 (the charter), 20G item-4 (`case_arm_oneliner_leaves` +
> `render::apply::inline_arm_subst` — the pattern extended), 20P (the `render` assembler API
> extended), plan/CLAUDE.md render notes (the `ap-2`/`an-render-runnable` gate, the `C-5`
> in-situ-guard `--WONDER`), spike/CLAUDE.md inv-leaf-seam + the ap-2 history. Surfaces:
> `plan/src/lib.rs` (the line-walk + two new helpers) + `plan/src/render.rs` (one new emitter)
> + three new e2e cases. ZERO change to any sibling surface (`syntax`, `analysis/value.rs`).

## §0 What landed (gates §6; all green within task-F2 scope)

- **the bug, confirmed reproduced** before the fix (§1): `…do echo "$x"\ndone; apt-get install`
  + converged ⇒ `# done; apt-get install…` + `true` ⇒ `dash -n` "end of file unexpected
  (expecting done)" ⇒ rc 2. The apply would abort MID-RUN on the host (the loop body already
  ran; the post-loop install line is malformed) — violating fail-before-network. +SURE.
- **the fix**: `render_apply` now detects AST-structurally when a `Replace`/`Omit` leaf shares
  its source line with loop/`if`/`case` scaffolding and substitutes it IN-SITU (keyword kept),
  with a real refuse-the-license fallback for the one shape in-situ cannot express. §2–§4.
- **three exec-gated e2e cases** (`post-loop-shared-done-line`, `pre-loop-shared-for-line`,
  `fi-shared-line`) + a unit-test quintet (incl. the fallback pin + a zero-churn control). §5.
- **ZERO golden churn** outside the new cases (§6) — after restoring one golden a sibling BLESS
  had contaminated with my mid-flight buggy binary (§7, the headline process-finding).

## §1 The defect class (20O find-2), reproduced and bounded

find-2 is a PRE-EXISTING, corpus-blind class: the line-granular render comments a whole line
when a `Replace` leaf is on it and no `Run` leaf is. A *structural scaffolding keyword*
(`done`/`for`/`while`/`if`/`then`/`elif`/`else`/`fi`/`case`/`esac`) is not a leaf, so when it
SHARES a line with an elidable leaf, the whole-line comment swallows it. Demonstrated (all +SURE,
driven end-to-end through the built binary):
- `done` shared: `for x in a b; do echo "$x"\ndone; apt-get install -y nginx` ⇒ `# done; …` ⇒
  unterminated loop ⇒ dash rc 2.
- `then`+`fi` shared: `if true\nthen apt-get install -y nginx; fi` ⇒ `# then …; fi` ⇒ dash rc 2.
- `fi` shared (trailing): `if true; then echo y\nfi; apt-get install -y nginx` ⇒ same class.

**find-mask-1 (the single-line masking — a real, separate finding, +SURE).** The charter's
literal corpus shapes are SINGLE-line (`for x in a b; do echo "$x"; done; apt-get install`).
On a single line the in-loop body leaf (`echo`, a `Run` leaf — the brk-1 in-loop floor) sits on
the SAME line as `done`+the install, so the line is a `run_line` ⇒ protected ⇒ rendered VERBATIM
(the install runs un-elided). That is a *missed elision*, NOT a broken artifact — so the
single-line form does not reproduce find-2. find-2's own demonstration used `\ndone` (the closer
on its own line); the bug fires only when the elidable leaf's line carries scaffolding but NO
run leaf. **Consequence: the three corpus cases are authored multi-line** (the body and the
closer on separate lines) so the install's line has no protecting run-leaf — the only layout
that exercises the fix. The single-line masking is itself worth recording: it is why a
corpus-blind class stayed hidden, and it means the elision-rate on idiomatic one-line loops is
suppressed by the in-loop-body run-leaf (orthogonal to this fix; a member-elision-render
concern, `209` brk-1(b)).

## §2 The detection's structural shape (the charter's first ask)

Two AST-structural inputs to the line-walk (`plan/src/lib.rs`):

- **`case_arm_oneliner_leaves`** (T14, 20G item-4 — UNCHANGED): the leaf `AstId`s that are a
  one-liner `case`-arm body (body item's line == arm first-pattern line). Routed to the EXISTING
  `inline_arm_subst` emitter with its EXISTING provenance comment (byte-identical ⇒ zero churn on
  `render-case-arm-oneliner`).
- **`scaffolding_boundary_lines`** (NEW, task-F2): the SET of source-line indices carrying a
  compound construct's scaffolding keyword. Per construct:
  - **opener line** `line_of(node.span.lo)` — `for`/`while`/`until`/`if`/`case` (catches a
    PRECEDING `install; for …`, case (b)).
  - **closer line** — `fi`/`esac` via `line_of(node.span.hi-1)` (their span INCLUDES the closer);
    `done` is SPAN-EXCLUDED (see §3) so it is found by skipping `;`/newline/whitespace after the
    loop body to the next content byte. Catches a TRAILING `done; install` / `fi; install`
    (cases (a),(c)).
  - **interior body-first lines** for `if`: `line_of(then_body.lo)`, each `elif.body.lo`,
    `else_body.lo` — where a `then`/`elif`/`else` keyword sits WHEN it shares the body's first
    command's line (catches `then install`, case (c-interior)). A loop's `do`-line is the body's
    first line too, but loop bodies are in-loop-floored to `Run`, so `do install` never elides
    (flagging it is harmless — the run-leaf wins).

**The token positions, summarised:** a keyword can share a line with an elidable leaf in exactly
three positions — AFTER a closer, BEFORE an opener, BEFORE a body's first command — and the three
line-keys above cover all three. A keyword on its OWN line carries no leaf, so the comment path
(which fires only on leaf-bearing lines) never reaches it; that is why only these leaf-bearing
boundary lines need flagging. (+SURE this is exhaustive over the modeled compound set; ~SUSPECT
nothing outside loops/`if`/`case` introduces a same-line closer in the grammar — funcdefs `}`
and `( )`/`{ }` groups were not exercised, see §8 deferred.)

**The render-time gate (`shares_line`)** — the load-bearing zero-churn mechanism. Flagging a line
is NOT enough to route to in-situ: a leaf ALONE on a boundary line (only indentation around it —
the keyword is on another line) keeps the ORIGINAL whole-line comment form. The walk computes the
leaf's in-line prefix/suffix and routes to in-situ ONLY if some non-whitespace byte brackets the
leaf. This is what keeps the `guarded` case (`if true; then\n   apt-get install -y nginx\nfi`)
byte-identical: its install is alone on its line, the `then`/`fi` are elsewhere ⇒ whole-line form,
no churn. (Without this gate the `guarded` golden moved — caught by the suite, §7.)

## §3 The loop-span caveat (the subtle structural gotcha — +SURE, verified in the parser source)

`parse_for`/`parse_while` set `span = kw.span.to(span_of(body))` — the loop node's span ends at
the BODY's last byte, **excluding `done`**. (Contrast `parse_if` `kw.to(end=fi)` and `parse_case`
`kw.to(end=esac)`, which DO include their closers.) So `line_of(loop.span.hi-1)` gives the body's
last line, NOT the `done` line — my first cut missed every `done; install` case. The fix:
`done`'s line = the line of the first non-`[ \t\n\r;]` byte after the body's `span.hi` (the parser
guarantees that inter-region gap holds only the separator and `done`). This is position-bounded
(it skips a known-shape separator run to the next content byte), NOT a free keyword text-scan — so
a command whose own text contains `done` cannot defeat it (the AGENTS.md / T14 discipline). ~SUSPECT
this is the right altitude; a cleaner long-run fix is for the parser to include `done` in the
loop span (then loops match `if`/`case` uniformly) — flagged as **tc-loop-span** for the
orchestrator, since the span is a `syntax`-crate (sibling) surface I must not touch.

## §4 The refuse-the-license fallback (the charter's explicit pin)

In-situ splicing operates on ONE source line's bytes. A leaf whose SPAN CROSSES lines (e.g. an
argv operand with a literal newline: `done; apt-get install -y "multi\nline"`) cannot be
expressed as one in-line splice. For such a leaf sharing a scaffolding line, the conservative
fallback REFUSES the license: the leaf is added to `run_lines` ⇒ rendered VERBATIM ⇒ it RUNS
(`kFAIL-perform` — over-executing an already-converged mutator is safe; a broken artifact is
not). Implemented as a real code path (`inline_span` returns `None` for a multi-line span ⇒ the
`None` arm extends `run_lines`) and PINNED by
`render_multiline_leaf_on_scaffolding_line_refuses_license_and_runs_verbatim` (the operand carries
a real `\n`; the leaf is converged ⇒ would be a `Replace`, but renders verbatim with `done`
intact). +SURE the pin exercises the path (without the fix the whole-line path ate `done`, dash
rc 2 — I reproduced that pre-fix). The same `None`-fallback also guards a hypothetical multi-line
CASE-ARM body (kept from T14's shape). No CURRENT non-fallback shape needs it; it exists so the
under-modeled case degrades to run, never to a broken artifact (the charter's mandate).

## §5 The corpus (three exec-gated cases) + the unit pins

All three are multi-line (§1 find-mask-1) so the install's line carries scaffolding with no
protecting run-leaf; HOST = nginx converged (`probe-results.txt` `effect=holds`); the loop/`if`
body is `echo` (a value-flow-pure builtin — see find-2a below) so the post-construct install
stays elidable. Probe shims: `apt-get` (inert apply mock) + `dpkg-query` (inert probe shim, exit
0 ⇒ `holds`) — gate-1 parity ENFORCES (no `PROBE_RESULTS=authored` opt-out needed).

- **`post-loop-shared-done-line`** (case a): `for x in a b; do echo "$x"\ndone; apt-get install
  -y nginx` ⇒ apply `done; true   # …in situ`. dash-n clean; the install elides.
- **`pre-loop-shared-for-line`** (case b): `apt-get install -y nginx; for x in a\ndo echo "$x";
  done` ⇒ `true; for x in a   # …in situ`. The `for` opener kept; install elided.
- **`fi-shared-line`** (case c): `if true; then echo y\nfi; apt-get install -y nginx` ⇒
  `fi; true   # …in situ`. The `fi` kept; install elided.

**find-2a (the exec-witness limitation — ~SUSPECT it is intrinsic, worth a ruling).** The charter
says case (a) should prove "echo runs twice, install doesn't". The post-construct install only
stays elidable if the loop body does NOT poison it — and ANY body with a ⊤ operand (`"$x"` JOINs
a,b ⇒ ⊤) propagates `Reach::Top` havoc across the loop back-edge and OUT (the
`loop-analyzed-body-runs` mechanism), poisoning the downstream install to `run`. Tested: a
`systemctl enable "$x"` body (DIFFERENT kind) STILL poisons (the havoc is operand-⊤, not
kind-keyed). Only a value-flow-pure builtin (`echo`) keeps the install elidable — and `echo` is a
shell builtin, so under PATH=mocks-only it logs NOTHING. So `expected.ran` is EMPTY for all three
cases; "the loop ran twice" is proven INDIRECTLY (the artifact exec-runs clean under dash — if a
keyword were eaten, the exec gate errors; and `echo` does print `a`/`b`/`y` to stdout, observed
out-of-band). The DIRECT "body logged twice" witness is unattainable WITHOUT a logging body, and a
logging body is necessarily ⊤-poisoning ⇒ the install would not elide ⇒ no fix to demonstrate.
This is a genuine tension between "prove the body ran" and "keep the post-construct elision", not
a corpus-authoring miss. Flagged as **tc-loop-exec-witness**; the clean resolution is the
member-elision render (`209` brk-1(b)) which would let a body command itself elide/log
deterministically.

Unit pins (`plan/tests/observable_matrix.rs`, via `plan_and_ast`): the three in-situ shapes
(`render_post_loop_install_sharing_done_line_…`, `…pre_loop_…for_line…`, `…post_if_…fi_line…`),
the refuse-the-license fallback (§4), and **`render_own_line_then_body_keeps_whole_line_comment_form`**
— the zero-churn negative control pinning that an own-line then-body install (the `guarded`
shape) keeps the whole-line form and does NOT enter the in-situ path.

## §6 Gates (scoped per the coordination note) + golden inventory

- `cargo fmt -p dorc-plan --check`: clean. `cargo clippy -p dorc-plan -p dorc-cli --all-targets
  -D warnings`: clean — NO new `#[expect]`s (the `too_many_lines` that the additions tripped was
  resolved by SPLITTING `render_apply` into `classify_lines` (decisions) + `emit_apply_lines`
  (bytes), not by an allow; `indexing_slicing` resolved by a `first_line` binding;
  `unnecessary_sort_by` by `sort_by_key(Reverse)`). `cargo test -p dorc-plan -p dorc-cli`: cli 9,
  plan-lib 34, plan-integration 24 (+1 pre-existing HOLE#1 ignore) — all pass.
- `sh e2e/run.sh`: **62/62, ZERO xfail, exit 0, run TWICE** (62 = the prior 59 + my 3). All six
  gates (ap-2 dash-n + apply/probe exec, redirect sandbox, ordered run-set, stderr floor,
  argv-echo differential). gate-5: the elided installs are `replace`-disposed ⇒ skipped by the
  `run`-only filter (task-O), so the in-situ render does not perturb the differential.
- `typos spike` (from worktree root): clean.
- **Golden inventory (the charter's "every golden: only your new cases"):** the only `expected.out`
  /`expected.ran` files I authored are the THREE new case dirs. `guarded/expected.out` shows a
  delta in `git status` ONLY as the sibling's-BLESS contamination I RESTORED (§7) — post-restore
  it is byte-identical to HEAD. No other pre-existing golden moved. (`toprejected/expected-diagnostics`
  is the SIBLING's diagnostic-wording change, not mine.)
- **Scope honesty (coordination note):** I touched ONLY `plan/src/lib.rs`, `plan/src/render.rs`,
  `plan/tests/observable_matrix.rs`, and my three new e2e dirs. The binary built cleanly
  throughout — the sibling's mid-flight `syntax`/`value.rs` edits never broke my `-p
  dorc-cli` build during this work (one transient e2e count flux 58→59→62 was the sibling adding
  cases, not a build break). The orchestrator must run the WORKSPACE-wide gates at commit (the
  sibling's surfaces are outside my scoped runs).

## §7 strain-F2-bless-contamination (the headline PROCESS finding — +SURE, a real cross-agent hazard)

Mid-task, `git status` showed `guarded/expected.out` MODIFIED with my (then-buggy) in-situ output
(`   true   # …in situ` where HEAD has the whole-line `# apt-get…` + `   true`). I had NEVER run
`BLESS=1`. Root cause (+SURE by construction): the two concurrent agents SHARE one
`spike/target/` build dir, and a `BLESS=1 sh e2e/run.sh` (the sibling's, or any blessing run)
re-blesses ALL cases from WHATEVER `target/debug/dorc` exists at that instant — which was MY
mid-flight binary with the not-yet-fixed scaffolding bug. So a sibling's legitimate bless of THEIR
cases silently re-blessed an unrelated pre-existing golden (`guarded`) to my buggy output. I
confirmed my FIXED binary reproduces `guarded`'s committed golden byte-for-byte, then RESTORED the
file via a filesystem write of the committed bytes (`git show HEAD:… > file` — a filesystem edit,
NOT `git checkout`/index op, per the SAFETY block's "filesystem edits only; no working-tree-restore
ops"). **Implications the orchestrator must weigh:**
- a shared `target/` + a global BLESS is a contamination vector between concurrent agents; a
  blessing agent should `cargo build` to a clean state first, or bless be scoped per-case, or the
  agents not share `target/`.
- I did NOT run a global BLESS for my own cases (precisely to avoid re-contaminating the sibling's
  cases with my binary); I generated my three cases' goldens per-case by running dorc directly +
  executing the apply under mocks (writing only into my new dirs). ~SUSPECT this is the right
  discipline for concurrent work, but it means my goldens were not produced by the canonical BLESS
  path — the orchestrator may wish to re-BLESS the full suite from the merged, clean binary at
  commit (which should be a no-op for my cases if my hand-generation matched the harness exactly;
  verified my `expected.out` are dorc's exact stdout and my `expected.ran` are the exec log).

## §8 Exclusion-check (the four-by-two discipline) + deferred

- **other phase**: the fix is apply-render-only; the probe render emits flat records (no
  scaffolding lines) — unaffected. Verified the probe artifacts of all three cases are dash-n
  clean + parity-enforcing.
- **other user**: the in-situ comment (`# dorc: elided … substituted in situ (shares line with
  scaffolding)`) reads to a human (admin/engineer) as a deliberate, legible substitution — not an
  engine artifact; the kept keyword means the book's control flow still reads correctly.
- **other reliability**: the refuse-the-license fallback IS the under-modeled cell — it degrades
  to RUN (verbatim), never to a silent elision or a broken artifact (kFAIL-perform).
- **reverse propagation**: N/A — the render consumes the forward `Disposition` + the AST back-map.
- **the killer cross-cell**: the `shares_line` gate is the exclusion-check catch — flagging a
  boundary line WITHOUT it broke the `guarded` golden (an own-line then-body install). The control
  test + the suite caught it; the gate confines in-situ to leaves ACTUALLY bracketed by scaffolding.
- **DEFERRED / not-exercised cells** (~SUSPECT-to--GUESS; flagged, not closed):
  - **tc-loop-span** (§3): the loop-span-excludes-`done` asymmetry is worked around in `plan`; the
    clean fix is in `syntax` (sibling surface).
  - **tc-loop-exec-witness** (find-2a, §5): the body-pure-vs-logging tension; member-elision render
    is the real resolution.
  - **tc-group-closer — `( )` subshell / `{ }` group delimiters: SAME class, DEMONSTRATED to bite,
    deliberately NOT fixed here (a stop-and-flag — +SURE).** I constructed it: `( apt-get install -y
    nginx\n); wombat after` + nginx converged ⇒ the install (subshell body, line 0) is whole-line
    commented (`# ( apt-get…`) eating the `(`, while line 1 `); wombat after` has a RUN leaf
    (`wombat`) ⇒ verbatim ⇒ stray `)` ⇒ `dash: ")" unexpected`, rc 2. The SAME broken-artifact class
    as find-2, for group/subshell delimiters. The mechanism extends trivially (`Subshell`/`Group`
    spans INCLUDE both delimiters, `open.to(close)` — so `span_lines` + body-first-line covers them,
    I verified the extension makes all group/subshell shapes dash-n clean). BUT (a) `( )`/`{ }` are
    OUTSIDE task-F2's charter token-set (loop `do`/`done`, `if`/`then`/`else`/`elif`/`fi`, `esac`),
    and (b) the extension CHURNS the pre-existing `exec-subshell-establish` golden: its single-line
    `( apt-get install -y nginx )` (body shares the line with BOTH delimiters) moves from the
    whole-line comment form (`# ( … )` + `true`, which is dash-n CLEAN — the whole construct including
    both delimiters gets commented) to the in-situ form (`( true )   # …in situ`). Per the charter
    ("any pre-existing golden delta is stop-and-flag") + the `tc-*`-flag-up rule, I REVERTED the
    extension and flag it: the orchestrator should decide whether to (i) extend scaffolding-detection
    to `Subshell`/`Group` and re-bless `exec-subshell-establish` to the (arguably cleaner) in-situ
    form, or (ii) accept the narrower broken sub-case (multi-line group/subshell closer sharing a RUN
    line) as a separate slice. The narrow broken case is NOT in the corpus today, so it is latent,
    not active — but it IS reachable (I reproduced it). The code carries a comment at the
    `scaffolding_boundary_lines` match arm marking the deliberate omission.
  - **multiple elidable leaves on ONE scaffolding line** (`done; a; b` both converged): the
    `scaffold_subst` map holds a `Vec` and splices right-to-left, so it is SUPPORTED in code, but no
    e2e/unit case exercises >1 sub on a scaffolding line (the corpus has exactly one). ~SUSPECT it is
    correct (the right-to-left splice keeps byte offsets valid); a pin would harden it.

## §9 ~SUSPECT / confidence summary

- +SURE: the bug reproduces pre-fix (dash rc 2, traced end-to-end); the fix makes all three shapes
  dash-n clean with the keyword kept; the refuse-the-license fallback is a real, pinned path.
- +SURE: zero golden churn outside the three new cases (post-restore); 62/62 e2e ×2 zero-xfail;
  fmt/clippy-D/test/typos green within the scoped crates.
- +SURE: `guarded` was contaminated by a cross-agent BLESS of my transient binary and is now
  restored to HEAD bytes (the fixed binary reproduces it exactly).
- ~SUSPECT: the trailing in-situ comment (`true   # …`) is always comment-safe because line-end is
  a comment-safe position in every shape exercised (after `done`/`fi`/a `for`-list word). Verified
  on the three cases; not proven for every grammar position. A suffix that ended mid-token where
  `#` is not a comment boundary would corrupt — none arises in the modeled set, but flagged.
- ~SUSPECT: `shares_line`'s prefix/suffix whitespace test is the right own-line-vs-shared
  discriminator; it correctly preserved `guarded` and routed cases (a)/(b)/(c).
- -GUESS: tc-group-closer (§8) does NOT currently bite the corpus, but the class is open for
  funcdef/group closers — the next slice should exclusion-check it.
