# immutable-fs-targets — round charter

> AI-authored (Fable, design-rubber-duck sitting, 2026-09-03; human present and adjudicating).
> Interactive-research round. Everything here is conversation-tier unless marked
> `human-typed`. Sibling of `ops-glue-residue` (2026-07-28), which touched this topic only
> as a transport-floor residual; this round digs it on its own.

## The question

`26K:sit-stdin-copy-exec-amendment` — Dorc ships every artifact as the remote shell's
stdin (`ssh -T host 'sh -s; printf <marker>' < artifact`). Any interior command that reads
stdin without its own redirect eats the rest of the artifact; under local-exec a pivot
book's first bare `ssh host …` line does exactly that. The amendment owed is the delivery
shape: keep the pipe, copy-then-exec (materialize a temp file on the host, `sh file`), or
an in-memory slurp (`eval "$(cat)"`-shaped).

## The human's lean (human-typed 2026-09-03, paraphrased closely)

Early decisions were taken to keep fs-unwritable targets possible as Dorc hosts; not
critical, but worth retaining as a small niche (odd heterogeneous homelab hosts: rPi,
weird routers, immutable-fs appliances), not disaster-recovery. Gentle lean:
copy-then-exec as the DEFAULT, implement now; retain the seam and rule in favour of slurp
as the FALLBACK, with lints/warnings when a book makes slurp dangerous or impossible — so
a host does not become entirely uncontrollable when its fs unexpectedly turns unwritable.
Constraint: must not add much complexity or kill velocity. FIRST: investigate feasibility,
in design-docs and prior-art, before anything is ruled.

## Fronts

- **turn01 — prior-art (Opus, Kagi, read-only):** how common non-writable-fs hosts are in
  the homelab/small-ops population; the mechanisms push tools use for temp/scratch on
  targets and their recurring pitfalls; the shell-level coupling between heredocs, temp
  files, and memory-only execution; how writability/exec-ability is detected. Graded
  source-base + verbatim excerpts in `turn01-*-notes.md`.
- **turn02 — design-docs (conductor, main context):** what the corpus already rules
  (`kBOOT`, `142` front-C, `139` tiers, `rul-probe-writes-only-what-it-owns`, the DREP
  scratch precedent, `law-artifact-floor`, the three-faces rule, the `$0` load model) and
  where copy-default + slurp-fallback + lints collides with or rides on it.

## Human-typed acks, redirects, and openings (canonical home; turn files carry pointers)

Dated; paraphrased closely; nothing here is a ruling unless it says so.

- 2026-09-03 (opening) — the lean above: copy-then-exec default now; slurp fallback retained
  with lints; investigate feasibility first.
- 2026-09-03 (redirect; originally recorded in turn03) — everything is granular-degradation;
  this round's granule is UNWRITABLE-FS. Today's hunt is LANGUAGE HOLES: sh-floor
  constructs unsafe to assume; in-language stdin/stdout usage; anything a host oddity forces
  onto the core language rather than behind a flag, a negotiation upsell, or a lint —
  finding that value early and ruling on it is most of the point. DREP and all
  host-specific mechanisms sit under `kBOOT` (early-days name); return channels the engine
  can own without multiplexing stdin/stdout/rc may deserve a front. Login-shell residue is
  known and gently open to more constraint than target architectures ("sh language first";
  an interpreter limit is more forgivable than an architecture limit) — priority behind
  interpreter specifics. Wanted: an early DIVISION of capability granules ("weird hosts" is
  not one; "nearby-sh-executor-but-outside-our-direct-floor" is another). The human is not
  an ops expert: obscure corners get an explanation-for-an-idiot; a bounding box of
  most-odd-but-frequently-real hosts poorly served by the SIBLINGS is a deliverable.
- 2026-09-03 (ack) — capability detection: everything we do is granular-degradation.
- 2026-09-03 (ledger convention) — a chat turn is usually not a research turn; turn ledgers
  are minted per research LANE; human-typed acks/redirects/openings belong in this
  non-turn document.
- 2026-09-03 (lean) — judo's "close fd 0" doctrine is never a core feature, but plausible as
  an opt-in strict/careful-tier defensive LINT on books.
- 2026-09-03 (ledger convention, sharpened) — THIS FILE is the round's running ledger
  (human-typed material AND conductor state); turn files are per-lane gathering records and
  stay historical (turn03 is left as-is, historical); `plan.md` is the synthesis surface.
- 2026-09-03 (curiosity; gentle tune, not a product statement) — how do people configure and
  control Talos and its siblings day-2? Suspicion: OUR long-tail is not everyone else's. The
  Weird Lil Guys almost nothing supports are exactly where every admin falls back to a
  Dorc-shape ("all my other infra with `terraform apply`, then remember to run this extra
  shell script over serial to reconfigure the widget"). Proposed target-metric: whether
  somebody somewhere has put a ton of effort into supporting <weird thing> — NOT user-numbers,
  and NOT number-of-orchestrators-that-support (very nearly the inverse set of what we should
  side-eye for inclusion).
- 2026-09-03 (scope) — "no fs at all" may be blocked to us STRUCTURALLY (does sshd/ash even
  stand up a session with no filesystem?); worth being sure someday by explicit test before it
  enters the outer bounds of interest, so effort is not spent where we are not the blocker.
  The interest is steady-state weird-lil-guy inclusion, not damaged-host reach.
- 2026-09-03 (KISS) — no reason seen to separate out a negotiation exchange or a new tunnel;
  not ruled out; if it ever exists it will be because some unforeseen mechanism forces it.
- 2026-09-03 (ack) — the transport protocol carrying metadata about itself is defensive and
  smart. `mode` is too generic: v1 is `capabilities`, an unordered simple set of keywords;
  at least `fs-write` and `fs-durable`. `fs-durable` cannot be tested, only intuited (kSTATE:
  only a pivot book that itself reboots the host while we watch could ever know); it will
  need user declaration or analysis lift someday — DEFER, probably unimportant here.
- 2026-09-03 (scope) — a no-fs fallback for COMMUNICATION is IN. Seed for out-of-the-box
  thinking (probably a non-starter): stand up an HTTPS server on the controller and stream
  host→controller. Do not overengineer; poke for early leads; highest value is anything that
  forces careful attention in the language surface.
- 2026-09-03 (ruling-shaped lean) — plan-time WARNINGS are expensive and fraught. The shape
  is: a normal LINT during authorship, and a HARD DENY at apply-time — we see the book, count
  the bytes, know there is no fs, know it is bash 5.1, so we KNOW a section will misbehave;
  if the section mapped to that host fails necessary-capability detection, fail-fast before
  shipping a single mutative byte. Product design stays capable across a wide range of hosts;
  making <every line> × <every host> work is not our job.
- 2026-09-03 (language/analyzer seam, low/medium priority; PINNED) — required capabilities
  must be pushed INTO/THROUGH the lattice: sections of a book may run on different hosts, so
  each eventually-transport-bound chunk of code needs LOCAL analysis of the host capabilities
  it requires, so failure is fail-fast AND granular, without One Weird Lil' Guy forcing all
  infra to avoid heredocs. Pin for the sitting on "how much do we stuff our fingers into
  argv/heredocs/herestrings" — tightly coupled.
- 2026-09-03 (front-4 SEED; dispatch after front-3 returns) — if other orchestrators have NOT
  taken the slurp-into-memory approach, the human wants to know WHY; a novel route is not
  often the correct one. Next front: fully focused on prior-art transport/standup and
  weird-lil-guy pains: (a) issues/threads with many upvotes/complaints where <something
  popular> is poorly supported for fundamental standup/transport reasons; (b) side-projects
  with a lot of love (>250 commits, or >3 years of activity, or >500 stars) working around,
  patching, or managing <orchestrator>'s poor handling of <weird-lil-guy>; (c) MOST
  IMPORTANTLY, pivots where a product paid heavy engineering cost or broke backwards
  compatibility to WALK BACK fledgling assumptions about transport/standup.
- 2026-09-03 (intuition; not ruled) — the strawman wrapper (host chooses the route inside
  the body) makes the human lean slightly toward two tunnels — or a delayed read from stdin:
  ideally the CONTROLLER makes EMISSION decisions from measured capabilities, not a
  decision tree shipped AS the body. Sketch: the wrapper tests, prints capabilities on
  stdout, THEN takes over stdin with a `cat` and waits for the controller to send the body.
- 2026-09-03 (capability) — probably need detection for MULTIPLE-file / DIRECTORY-TREE
  write, not just one file: a host may allow a file in cwd but not a replicated tree;
  matters for the single-stream / hybrid-bundle / full-tree-copy forms the admin chooses.
- 2026-09-03 (ack) — file route vs in-memory is effectively a USER choice for in-language
  scripting reasons; partly covered in the bundle/loader/transport planning docs.
- 2026-09-03 (lean; not ruled) — a "per-host escape" would precisely BE: not using `sh -c`,
  and hand-writing host-resolution / sh-obtainment into the BOOK. Unsure how to support
  `start shell`-class hosts. Windows is a FIRST-CLASS target; the Windows-support
  conversations (`139`, `13A`) covered this slightly — if the Windows mechanism can be
  general enough for other targets, grand.
- 2026-09-03 (BANKED for a joint turn; do not answer in passing) — there is the shape of
  something language-surface-tier here: how CAN you spell-in-sh, without special
  command-line flags or non-sh markers, that some particular host is Windows and needs
  some `cmd` stuff first?
- 2026-09-03 (read-in the conductor missed; human-typed) — THREE FLOORS, not one:
  (1) the LANGUAGE floor = posh∩dash, what `dorc-lang` files may use (narrow, singular: a
  single described surface exactly where the two agree); (2) the TRANSPORT floor = where the
  engine-generated standup/shims are promised and tested to work (this discussion);
  (3) the ANALYSIS/BOOK floor = what language(s) we support IN BOOKS for analysis — may be
  strictly higher than the other two (a bash book on bash hosts is fine as long as standup
  works; making hosts support your book is the user's job). Effect on the eval route: it is
  potentially another capability, separating "host-transport-standup floor IS book floor"
  from "book analysis escaped the transport floor and needs full execution by e.g. `bash`
  post-standup" (thus no eval-as-a-solution). (3) is a SUBSET and a secondary target:
  users are pushed toward POSIX sh and simplicity; bash/zsh-book support is gradual-
  enhancement to ease migration of old ops material, carefully carved protections, not
  what we want people doing.
- 2026-09-03 (caution, strong) — be very careful of "bracket on stderr": the project
  started with, then fought hard to escape, MULTIPLEXING channels; for those hosts that is
  catastrophic failure, not an easy fix. Multiplexing may be FORCED back in as a fallback
  for specific/degraded/weird hosts, but is definitely not the default; stay away wherever
  possible. The conductor's defensive DREP contract rules (sink is write-only, append-only,
  possibly a device; never read back) are PINNED, assuming no better option turns up.

- 2026-09-03 (the THREE FLOORS, expanded; human-typed, for the synthesis) — different
  PRIORITIES produce the three floors. (1) The oracle/dorc-lang "shell language" needs a
  MAXIMAL floor: users write it, write a lot of it, and must off-ramp it without hating us;
  but its requirements ("let authors use `local` and other defensive habits") mean we target
  slightly ABOVE the nominal standard and opt into doing the compatibility work ourselves,
  deciding per feature what authors may write. (2) The transport floor may be slightly
  LARGER: it is constrained neither by the wider-compatible-sh mission oracles carry nor by
  backwards compatibility; we continuously measure hosts, map interpreters to capabilities,
  and do whatever stands up transport and boots the user's book effectively — it must be
  strictly larger than (1) (observed as a pure-predicate-logic result, unjustified inline).
  (3) The book languages supported are a fully disjoint set at base and GENTLY a maybe-
  superset of (1): aspiration to bash/zsh support for newcomers' existing scripts, yet books
  must be able to execute oracle code; unruled, lean wide / few restrictions. Possible only
  because standup/transport/probing is DECOUPLED from book-byte shipping and evaluation
  (use ash/dash for our purposes, then run `bash` for the mutative step if the host has it).
  Keep this door open.
- 2026-09-03 (punt) — the no-channel-returns capability class: without probing Dorc is
  literally not a product there; the residue is minimal live-alongside support in a
  multi-orchestrator estate (does analysis descend into cloud-init user-data? is there
  value?). Punted. no-channel-interactive-but-returns is the interesting cell and probably
  needs robust support.
- 2026-09-03 (lean, not ruled) — eval as the transport: leaning toward it, will likely try
  it for the spike, suspicious of unseen dark corners. Ack that bash/128 KiB/eval is a map
  of capabilities-to-tests-to-fail-fasts.
- 2026-09-03 (dogfood) — the project's own lab can exercise a Ubiquiti gateway and a
  legacy Synology unit; Home Assistant OS is wanted. Anything standable in a container or
  cheap VPS is a comparatively excellent frontier entry; most of the weirdest hosts are
  bare-metal and hard to exercise. Development is Windows-first on purpose (to force
  Windows compatibility) though daily work is macOS.
- 2026-09-03 (attention) — cloud-init is especially attention-worthy: relevant to
  whole-universe-standup meta-orchestration (probably dominated by Terraform for its
  users), and parallel to the earlier Dorc-as-child posture (Dorc as an idempotency wrapper
  of a dispatched sh chunk inside a primary orchestrator).
- 2026-09-03 (research angles, adjudicated) — no measurement lane now (constrained
  machine); embedding-contracts NEEDS investigation (possibly a successor, with the human);
  request/response channel mechanics mildly less so; capability-axes pursued now from
  context, abstract SHAPE and CARDINALITY first, coverage-per-target someday (expensive,
  needs a robust capability system first).

## Second sitting (2026-09-03, design duck over the returned fronts) — human-typed acks; text in `notes/26N`

- ack `rul-delivery-shape-file-backed-default` (the sh-visible behaviour; specifics may vary
  per platform and by user configuration; Windows needs its own tune) · ack the
  single-streamed tree, for the spike, suspicious · ack `rul-preflight-over-probe-time` (typed:
  anything not affecting the reviewed plan preflights immediately before mutation; probe-time
  only for late decisions that reduce review load; forward human attention nearly never owed)
  · ack the on-demand, tree-shaped, single-artifact standup; "full and upfront" is a lean ·
  NACK ship-both forms ("we hold the line on how we hide things") · ack the `$0` authority
  spelling after the both-worlds argument (the lattice carries the Dorc world and the off-ramp
  world; no claim survives the off-ramp; bytes untouched; nothing taught) · lints dropped as
  meaningless, all hints deferred · materialise under the input filename · Dorc moves no file
  whose loading path it does not manage · TABLED the shebang as dialect declaration · ack that
  the flagship fixture is guard-at-most; havoc is never a denial · HARD ACK
  `rul-dorc-acts-are-withhold-shaped` ("first, do no harm") · ack `rul-unresolvable-book-load-refuses`
  as no-ship, the custody split reserved for dorc-lang · ack the floor-unevenness gloss (trumped
  by kFAIL-withhold and secrets) · ack the `30Q` supersessions · the census question left OPEN
  as the next sitting.

## Conductor state (running; overwrite in place)

- All four fronts returned and folded (`plan.md` addenda 1–3; turn01/04/05/06 are their
  records); manifest at 117 sources (all `graded-by: subagent`; re-verification of the
  load-bearing grades is owed if any is ever cited as load-bearing in a stamped plan). The
  design sitting over them is CLOSED with the acks above; the round's direction-setting
  synthesis is **`Research/notes/26N-host-capabilities-and-the-weird-host-frontier.md`** —
  read that, not this directory, for takeaways; this directory is the evidence base. No
  front in flight; no measurement lane (constrained machine). Human-as-debugger asks
  outstanding: the Ubiquiti page (archiver 403), HPE iLO's CLP guide (empty bodies), a
  fish/csh login shell for the marker-line measurement, and the population/habit question.
  Next sitting: `26N:open-census-needs-value-plane`.

## Exclusions

- No code-level reads of `spike/` beyond what turn02 already holds (the codebase churns).
- Nothing under any `quarantine-DO-NOT-READ` path is read by anyone in this round.
- No rulings are minted here; the round produces evidence for the sitting, not the sitting.
