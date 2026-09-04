# 26O — Channels: streams, worlds, links, and the routing between them

> AI-authored design-of-record (Fable; the 2026-09-03 design-duck sitting with the human,
> adjudicated in chat; banking human-directed). Ahistorical: how streams, sinks, and return
> paths behave IF BUILT, each item graded — **[TYPED]** the human typed the substance ·
> **[ACKED]** the substance was put and the human acked it as read · **[LEAN]** the human's
> stated lean, not a ruling · **[RULED]** ruled earlier, cited · **[PROPOSED]** conductor
> synthesis · **[OPEN]** unruled. Authority: root docs, `spike/CLAUDE.md`, and the welds
> (`KNOBS:kFAIL`, `kLANG`, `kBACKFLIPS`) outrank this; it extends and never re-opens
> `plans/142` (executorless-OOB), `plans/260` §5 (the transport spec), `plans/262` §2 (the
> records lane), `plans/27C` (context entry), `plans/30W` (index-kinds), `notes/26N`
> (capabilities, delivery, `$0`). Remit: every byte that moves between the controller and a
> world Dorc stands up, in either direction, at any depth — and therefore the report sink's
> home, the raw-output question, and live apply. The strawman that exercised it is
> `notes/26Oa`. Concurrency is declared out of this document's scope (human, 2026-09-03);
> `26N` §4 is the capability system this rides.

## §0 — the design in one screen

Two streams cross every non-detaching link and nothing else does; that is the universal
floor, and it composes both ways as a meet of link capabilities. Every world Dorc stands up
carries its own scaffold — its own exclusive scratch, its own report sink, frames written at
minting with a controller-baked context, a closing count, its own measurements — so records
never cross a link as records, only as stream bytes already framed, and outer worlds forward
without re-framing. Three classes travel: **control**, **aid**, **raw**. Which stream carries
which class through which link is not fixed by the language: it is planned per run, after
capabilities are measured and the book analysed, under one hard rule (control never shares a
lane with bytes Dorc did not write) and one cost ordering (identity preserved, above freeform
collapse, above structure-in-freeform; collapse a last resort). The probe lane is always dark
and `-T`; apply is quiet by default; raw liveness is an opt-in the planner serves by
leaf-local means; live apply attaches the terminal through two sessions.

## §1 — physics (facts, not rulings)

- `law-two-streams-are-the-universal-floor` — stdout and stderr cross every non-detaching
  link (`sudo`, `su`, `doas`, `chroot`, `nsenter`, `ssh`, the exec-APIs); no other channel
  does. fds above 2 die at `sudo` by default (`closefrom`; `-C` admin-gated) and at every
  machine boundary; environment dies at `sudo`, `su -`, `systemd-run`, `ssh`; the
  filesystem changes at `chroot` and vanishes at `ssh`. Detaching wrappers (`systemd-run`
  without `--pipe`, `at`, `nohup … &`) forward nothing; request/response channels return
  after exit, truncated; one-shot channels never return (`26N` §8).
- `law-a-pty-merges-at-the-write` — a pseudo-terminal is one device; a process holding it
  on fds 1 and 2 merges them in the kernel at the write, unrecoverably. `sshd` attaches all
  three fds to one pty on `pty-req` (RFC 4254 defines one); two ptys are possible in
  principle and `sshd` does not do it. A pty is also what makes tools flush per line and
  behave interactively (`isatty`): liveness by tty and identity by pipe are in genuine
  tension.
- `law-serial-shell-bounds-reorder` — the shell is serial: a tool's buffered stdout flushes
  at its exit, so within-command reorder never crosses the next reporting point;
  cross-channel skew over `ssh` exists only under back-pressure; the only genuine crosser is
  a tool that daemonizes, which is the user's own concurrency.
- `law-the-transport-floor-is-no-pty` — `KNOBS:kBOOT`'s triple (byte-clean stdin, separate
  stderr, non-echoing) is "no pty on the path". `-T` at every hop preserves stream identity
  to arbitrary depth (extended-data at each `sshd`, fd 2 at each client); one `-t` anywhere
  merges everything beneath it, permanently. Byte-clean stdin delivery
  (`26N:rul-delivery-shape-file-backed-default`) needs the same absence.
- `law-liveness-is-a-tool-property` — through a pipe a tool block-buffers its own stdout;
  Dorc adds zero buffering, and only two leaf-local hacks move liveness without spending
  identity: `stdbuf -oL` (glibc-dynamic binaries only) and a raw pty on fd 1 alone (which
  summons interactivity: pagers, colour, prompts). Any non-shell process in a return path
  re-introduces buffering; a shell `while read` loop does not.

## §2 — worlds, links, and their capabilities

Vocabulary: a **world** is a context (`27C` dimensions, `30W` index-kinds, the controller
under local-exec); a **link** is the wrapper or transit between two worlds; a **lane** is
one ordered byte stream a link forwards; the **tree** is a book's worlds under its links; a
**frame** is one control or aid record on a lane; a **sink** is where an oracle body's
report-lane writes land; a **scaffold** is the engine-owned sh around a world's shipped
bodies; a **carrier** is how a segment's bytes reach a world. Capabilities are measured
predicates, never declared, never inferred from identity, tri-state with unmeasured reading
as absent, keyed by context (`26N:def-capability-is-a-measured-predicate`).

- `def-world-and-link-capabilities-are-distinct` **[TYPED]** — world capabilities
  (`fs-write`, `/dev/fd`, a pty helper, `stdbuf`, the interpreter set, sink form) and link
  capabilities (lanes out, fd-over-2 crossing, env crossing, fs view, cwd, pty on path,
  byte-clean stdin, returns, sibling session) are separate axes; both feed the join
  (`26N` §4.2). A sibling session is a link capability and is never assumed —
  near-exhaustion, `MaxSessions 1`, odd tunnels.
- `lean-link-stream-behaviour-rides-the-wrapper-surface` **[PROPOSED]** — what a wrapper
  does to streams (`-T` versus `-t`; `--pipe` versus the journal) is the wrapper author's
  knowledge and belongs beside `lends`, with measurement confirming it per world; the
  engine never decodes the tool (`inv-referent-agnostic`).
- `matrix-links-and-worlds` — the table that fills over time, keyed by link and by world,
  never by host. Cells: `+` measured present · `−` measured absent · `~+`/`~−` believed,
  unmeasured · `?` unknown. Today's cells are the sitting's reading; §10 names the first
  errands.

| link | lanes out | fd>2 crosses | env crosses | fs view | cwd | pty on path | byte-clean stdin | sibling session |
|---|---|---|---|---|---|---|---|---|
| `sudo` | 2 | − (`-C` admin-gated) | − (reset) | same | kept | ~− (`use_pty` wants a tty) | + | n/a |
| `su -` | 2 | ~+ | − | same | home | − | + | n/a |
| `doas` | 2 | ? | − | same | kept | − | + | n/a |
| `chroot` | 2 | + | + | new root | new root | − | + | n/a |
| `nsenter` / `ip netns exec` | 2 | + | + | same, or a new mount view | kept | − | + | n/a |
| `systemd-run` | 2 with `--pipe`, else 0 | − | − | same | the unit's | − | with `--pipe` | n/a |
| `ssh -T` | 2 | − | − (`AcceptEnv` only) | none | home | − | + | ~+ (`MaxSessions`; ControlMaster) |
| `docker exec` / `kubectl exec` | 2 (API-framed) | − | − | container's | container's | − unless `-t` | + | + |
| SSM / run-command | 2, post-hoc, truncated | − | − | none | none | − | − (no stdin) | ? |
| serial console | 1 | − | − | n/a | n/a | + (a real tty) | − (echo) | − |
| local-exec (the controller) | 3+ | + | + | same | kept | − | + | + |

| world | `fs-write` | `/dev/fd` | pty helper | `stdbuf` | sink form |
|---|---|---|---|---|---|
| Debian / Ubuntu server | + | + | `script` (util-linux flags) | + | file |
| macOS (bash-as-sh) | + | + | `script` (BSD flags) | − | file |
| busybox (OpenWrt, ESXi) | + (tmpfs) | ~+ | `script` (busybox flags) | − | file |
| FreeBSD appliance | + | − (0–2 only) | `script` (BSD) | − | file |
| hardened container | − | ~+ | − | − | fd, or none |
| failed host (ENOSPC, ro-root) | − | + | ? | ? | fd, or none |

## §3 — what travels, and which way

- `def-three-demand-classes` **[TYPED names; ACKED content]** — **control**: probe frames,
  `predicts` and `nothing-else` records, the completion marker, the `30W` witness at apply,
  anything a license or an integrity decision reads. **aid**: `decline` records, the cursor,
  anything only the why-lens reads. **raw**: a tool's own stdout and stderr, optionally
  live, never parsed. Any class may be needed in any phase; the phase changes only whose
  bytes the artifact is, which bounds what can be *emitted* without editing them and never
  what may be *needed*.
- `law-inbound-and-outbound-both-compose` **[TYPED]** — inbound (a Dorc-generated segment
  reaching a world) composes link by link and has no universal carrier: argv under the
  128 KiB cap and the remote re-parse, stdin from a file in the outer world's scratch, a
  heredoc where the outer shell never touches the fs for one, a PATH shim
  (`28Q:pin-ssh-entry-shape` decides the `ssh` case). Outbound has the universal floor of
  §1 and composes as the meet of every link's lane set along the return path: a shell-only
  bastion, a forced pty, a `MaxSessions 1`, a screen-scraped hop each cap what returns. The
  asymmetry is floor versus no floor, not composes versus does not.
- `law-control-never-shares-a-lane-with-freeform` **[RULED; `KNOBS:kCOMMS`]** — the
  security line restated as the one hard routing rule: control rides only a stream Dorc
  owns end to end. It is why a probe artifact owns its stdout and why nothing structured
  rides a pty session beyond the trailing marker.
- `req-rich-oob-through-every-circumstance` **[TYPED]** — rich, non-multiplexed controller
  communication through every world and every link is non-optional, probing above all;
  where a circumstance forces a mechanism, mechanisms compose along the chain, and the
  general mechanism is built first with per-wrapper oddities layered on top, never the
  reverse.
- `req-raw-output-may-stream-live` **[LEAN; treated as welded for design]** — a wrapped
  leaf's output must be optionally streamable to the controller's interactive terminal,
  real-time and unmodified, wherever world and links allow; bounded by
  `law-liveness-is-a-tool-property`; never a default.
- `law-no-multiplex-into-a-stream-we-do-not-control` **[TYPED; `26N:finding-no-multiplex-except-forced` sharpened]** —
  the caution is primarily about multiplexing INTO a stream Dorc does not fully control
  (arbitrary process output); multiplexing several streams Dorc fully controls is painful
  but much less so. Freeform stdout+stderr collapsed together is slightly less bad than
  Dorc structure inside freeform.

## §4 — the per-world scaffold

- `rul-every-world-carries-its-own-scaffold` **[ACKED]** — every world Dorc stands up (the
  command-line target; every `27C` entry; every book-spelled transit a probe descends into)
  gets, from its own carrier: an exclusive scratch at a controller-literal root created as
  that world's identity (`rul-probe-writes-only-what-it-owns` ·
  `rul-scratch-root-never-read-from-host`); a local report sink bound inside it; frames
  written at minting; a closing frame with a count; its own capability measurement,
  disclosed on its marker line. This is the meta-orchestration requirement — reliable
  probing of nested targets — stated as a mechanism.
- `rul-frames-are-written-at-minting` **[ACKED]** — a frame is one atomic write (under
  `an-marker-atomicity`'s cap; the write side is now the binding side, so free tails are
  capped at emission, not only at intake), made in the world that minted it, carrying the
  context literal the controller baked into that world's scaffold; the outer deframer
  CHECKS it against what it shipped where (`rul-attribution-is-controller-minted`) and never
  mints it.
- `rul-records-never-cross-a-link-as-records` **[ACKED]** — an inner world's control stream
  is already framed, so outer worlds forward it and never capture or re-frame it; the sink
  variable never crosses anything. Nesting would have to be built deliberately and would
  buy nothing. The report sink's "remote home" therefore dissolves: the sink's home is the
  entered world's own scaffold — the `sudo` case rides the shim carrier, the `ssh` case
  waits on its inbound carrier.
- `rul-sink-form-is-a-world-capability` **[ACKED]** — the local sink is a file drained by a
  shell loop where `fs-write` holds, an fd into a shell-loop framer where only `/dev/fd`
  holds, and absent otherwise (`26N:finding-fd-sinks-die-under-privilege` is why the file
  is the default). The author's idiom — `>>"${DREP_V1:-/dev/null}"`; write-only,
  append-only, possibly a device, never read back (`26N` §7, pinned) — never changes.
- `open-sinkless-world-record-policy` **[OPEN; the `26N` §4.5 lean]** — in a world with no
  sink, `decline` records are disclosed loss; `predicts` and `nothing-else` withhold
  licensure for that context and never fall back to default semantics (`30D` §7.3). The
  conservative reading builds without waiting; the ruling is the human's.
- `rul-closing-frame-carries-finality` **[ACKED]** — each world's closing count is what
  makes absence meaningful per world (`30D` §4.4) and tells a severed inner link from a
  world that emitted nothing — the `deriv-end` shape one level up; an expected sever
  (`30W` §4) is the one absence that reads as success.

## §5 — the routing planner (`kCOMMS`, made concrete)

- `rul-streams-are-dorc-owned-routing` **[TYPED]** — sh spells redirections inside one
  process tree and has no spelling for a stream reaching the controller through three
  worlds; that routing is Dorc's, under `KNOBS:kCOMMS` and `kBOOT`. No new knob; this is
  the inflection where Dorc claims ownership rather than trying to Just Be Sh.
- `mech-stream-routing-is-planned-per-run` **[ACKED]** — demand per leaf from analysis
  (which of its streams are consumed downstream in the book, its wrap depth and the links
  on its path, whether the book touches `-x` or `PS4` above it, stdin-default commands,
  forks) against supply per world and per link; each demanded stream is assigned to a lane
  on each edge along its path, folded bottom-up in the shape of
  `27C:rul-dimension-owned-compose-ops`, collapsing where an edge lacks lanes. Decided after
  measurement and analysis, never fixed by the language; recomputed per invocation; nothing
  persists (`KNOBS:kSTATE` untouched). The net-max of capabilities bounds which program
  topologies are acceptable; the arrangement of the ones present decides how each is
  implemented.
- `rul-stream-collapse-cost-ordering` **[TYPED]** — collapse is a last resort; when forced,
  identity preserved (capture or discard one stream) sits above freeform collapse (fold one
  tool stream into the other), which sits above structure-in-freeform (control or aid inside
  a tool stream). Liveness is a separate axis moved only by leaf-local means.
- `rul-routing-is-disclosed-never-taught` **[ACKED]** — the user never reads the routing;
  the marker line discloses the arrangement per link, and the why-lens can say where a
  stream was folded and why. At-a-glance understanding returns at the report level, not the
  language level.
- `rul-tty-test-is-a-controller-fact` **[PROPOSED]** — `[ -t 0 ]` and `[ -t 1 ]` read a
  fact Dorc itself decides, so `test -t` joins the decidable set keyed on the invocation's
  pty choice; a live-aware book's other arm is omitted at plan time and the render shows the
  arm that will run.

## §6 — the arrangements the planner lands on

- `mode-probe-lane-always-dark` **[ACKED]** — `-T`, no pty ever; frames own stdout; a
  check's stdout is captured where `fs-write` holds, else discarded; a debug flag folds it
  into stderr as an explicit freeform collapse; a body claiming `predicts stdout` captures.
  Oracle bodies are non-interactive by contract, and a pty would let them stop being so.
- `dec-apply-default-ships-stderr-not-stdout` **[LEAN, human 2026-09-03, "not married"]** —
  with nothing else constraining it, apply ships stderr back and leaves stdout on the host
  for later pull (no retention policy yet); stderr is less noisy and more likely to be the
  last useful thing seen if the host goes down mid-apply. Genuine zero-configuration
  defaults stay maximum-reasonable-security and quiet; a gentler bundle of defaults follows
  the security-posture setting (NYI, discussed elsewhere). Failures route to `dorc why`,
  possibly run in full on a mid-mutation failure; that pull needs the on-host capture, which
  is `fs-write`-gated. Streaming apply is a debugging aid, not the product. Consistent with
  `inv-unaccounted-output-stays-remote-by-default` (`cli/CLAUDE.md`): this is that law's
  default half, with one stream returned.
- `mode-apply-streaming` **[ACKED]** — both raw streams pass through, split, identity
  kept, no cursor; an opt-in for a human at a terminal.
- `mode-apply-cursor-via-xtrace` **[ACKED shape; PROPOSED spelling]** — `sh -x` with a
  controller-minted `PS4` beginning `+ ` and carrying context, `${LINENO}` (the user's own
  line numbers, `law-lineno-identity`), and `$?` (the previous command's status), so
  per-line boundaries and rc ride the shell's own trace with the plan bytes untouched
  (`KNOBS:kBACKFLIPS` intact — only Dorc's invocation spelling changes, the same seat the
  constant wrapper lives in). Used only when the name-observation census
  (`an-name-observation-census`) proves the book never touches `-x` or `PS4` on a reachable
  path with no havoc above; a stomp flag overrides; a defend-stdout flag forbids the fold; an
  admin `PS4=` mid-book degrades the cursor from that line, disclosed; the admin's own
  `set +x` is the sh-native opt-out and is honoured. Aid-grade only, never license
  (`law-two-planes-opposite-fail`); expanded argv appears exactly as `sh -x` would, on the
  user's own terminal, never persisted by default (`an-diag-secret-taint`). POSIX pins
  xtrace to fd 2; fd 3 can carry a bash trace (`BASH_XTRACEFD`) and DREP writes
  (`/dev/fd/3`), never the floor cursor — fd 3 and xtrace are destinations, not
  alternatives, for the cursor.
- `mode-live-attaches-the-terminal` **[ACKED as direction; not for build now]** — two
  sessions per host where the link allows: a `-T` session delivers the tree and measures;
  a `-t` session runs `sh /scratch/<the book's filename>` with the user's terminal attached,
  local tty raw, signals and window size relayed by the tools themselves. Control leaves the
  pty: the marker trails in the merged stream; anything more (the `30W` witness, a drained
  sink) rides the delivery session through a FIFO in the scratch — `fs-write`-gated,
  sibling-session-gated, and where neither holds live degrades to marker-only. Nested hops
  interact only where the admin's own bytes say `-t`. The admin owns interactivity, told
  once in the flag's description: pagers page, prompts prompt, `read` blocks, `&` is theirs,
  `/dev/tty` may be absent inside a `chroot`, nothing is captured unless asked and a capture
  is a pty transcript. What flips in Dorc's favour: `requiretty` hosts work; first-contact
  prompts (a root password, a host-key confirmation) are answered by the human — the
  pivot's identity-at-creation cell answered by a person; `stdin-live` sites become supplied
  rather than linted; Dorc-as-parent hands a real terminal to `ansible-playbook`,
  `terraform apply`, and their prompts.
- `rul-verdict-bodies-are-tty-proof` **[PROPOSED; an oracle-contract line]** — a guard's
  check runs on the terminal under live; a body that pages hangs Dorc's own inserted line.
  The engine cannot defend it (`</dev/null` does nothing, `less` reads `/dev/tty`; a
  `PAGER=` prefix is tool knowledge the scaffold may not hold), so the contract says
  tty-proof, the quality bar flags known pager-invoking commands in verdict bodies, and the
  fault is the author's.
- `mech-third-lane-upgrades` **[ACKED]** — fd 3 across same-kernel links that forward it,
  most valuable on local-exec, pivot, and dotfiles-bootstrap books, where all three streams
  stay clean at one's own machine; a FIFO drained by a sibling session on hosts with
  `fs-write` and a multiplexed connection (`plans/142`'s layout arriving as a capability).
  Never the floor.
- `mech-leaf-local-liveness` **[ACKED]** — `stdbuf -oL` and the raw fd-1 pty are wrappers
  in Dorc's own vocabulary: authored entry forms behind world capabilities, never engine
  features; `script`'s flag dialect differs across util-linux, BSD, and busybox, which is the
  argv-fidelity carve showing up again.

## §7 — pivots and transits

- Host is an entered index; a transit disturbs an index-value cell and re-keys downstream;
  expected sever derives; the witness is integrity-plane only (`plans/30W` §4–§5)
  **[RULED]**.
- Every entered world, including a remote one reached mid-book, gets the §4 scaffold from
  its own carrier **[ACKED]**; delivery per world follows `26N` §2 **[RULED]**; the `ssh`
  inbound carrier is `28Q:pin-ssh-entry-shape`, unruled here **[OPEN]**. Finding for that
  ruling: the argv form (`ssh host "$(cat <<'EOF' … EOF)"`) is what the incumbents do
  (`26N:finding-memory-route-is-not-novel`) and what a live-aware admin writes, because it
  keeps stdin free for the terminal; it re-parses once per hop, which is hostile through a
  shell-only bastion, and `-J` avoids the second re-parse.
- Per-host sessions are separate lanes at the controller by construction, so multi-host
  rendering is a controller problem — prefix at the renderer, never on the wire; the one
  true interleaving inside a lane is the user's own `&` **[ACKED]**.
- Live mode is where a pivot's first contact belongs **[ACKED as direction]**.

## §8 — what the strawman taught (`notes/26Oa`)

One book, day zero through day N, laptop through bastion to a VPS with a provider-issued
root password, plain sh throughout, `--live` making one test true. Findings, by how hard
they bite:

- Value capture sits ahead of everything in this document for pivot books:
  `ip=$(terraform output -raw web_ip)` is ⊤ until the capture lane lands
  (`ROADMAP:arc-r26-revival`; `271:rul-value-prediction-species`), so every `"$ip"` site's
  Host operand is ⊤ and the remote half runs on day N regardless of streams.
- `fnd-tty-test-is-a-controller-fact` — the whole live/batch dial is
  `[ -t 0 ] && [ -t 1 ]`, two literals, and `${batch:+…}`/`$tty` expansions; foldable at
  plan time (§5).
- `fnd-expected-sever-has-a-plain-spelling` — `|| [ "$?" -eq 255 ]` is the admin's spelling
  of what `30W` derives from a reboot's footprint; recognizing it is how derived sever and
  authored acceptance agree, and how a real 255 stays disclosed rather than swallowed.
- `fnd-tty-is-a-wrapper-dimension` — `-t` belongs in the `ssh` oracle's argparse as a
  dimension the entry form and `lend_map` both see, since probes enter `-T` while the apply
  line may carry `-t` (`26M:hole-probe-path-transport-divergence` made concrete).
- The expanding heredoc for one laptop value beside a quoted one for the rest is
  `26M:hole-expansion-siting-across-the-boundary`, written by a careful admin.
- `$SUDO apt-get …` is `26M:ack-command-position-constant-prop-owed`; the `ssh … true`
  guards are the reads `26K:ack-connection-dance-oracles-core` names (reachable ≠
  provisioned); both gate whether the bootstrap and converge sections ever elide.
- Both inline payloads are opaque under r31's argv form until `310:seam-payload-forms`.
- The author has the tools: the six idioms they already use to make a script safe both by
  hand and from cron — `test -t`, `${var:+flags}`, the argv-form payload,
  `-o BatchMode=yes` with `sudo -n`, `--no-pager`, and the sever acceptance. The either-or is
  real in exactly one place, password entry, which the book names and exits on.

## §9 — invariants adopted

1. `inv-26O-control-owns-its-stream` — control rides only a stream Dorc owns end to end; a
   pty session carries no control beyond the trailing marker.
2. `inv-26O-frames-never-re-framed` — a frame is written once, in its world, and forwarded
   verbatim by every outer world.
3. `inv-26O-no-process-in-the-return-path` — between a frame's write and the controller sit
   only shells and the links' own relays; a framer is a shell loop, never `sed` or `awk`.
4. `inv-26O-routing-is-not-language` — no stream assignment is fixed by the dialect or
   taught to authors; authors write sh, Dorc plans lanes.
5. `inv-26O-collapse-is-chosen-and-disclosed` — every collapse is a planner decision, named
   on the marker and answerable in `why`.
6. `inv-26O-probe-never-holds-a-pty` — the probe lane is `-T` everywhere, permanently.

## §10 — open, measurements, and register state

Open (the human's): `open-sinkless-world-record-policy` · `28Q:pin-ssh-entry-shape` ·
the apply default (§6, a lean) · whether
`lean-link-stream-behaviour-rides-the-wrapper-surface` earns a strawman ·
`26N:open-census-needs-value-plane`, now carrying stream demands as its first non-boolean
input.

Measurement errands (ride `26N:lane-interpreter-and-login-shell-measurement`; each fills a
`~` or `?` cell in §2): `$?` and `LINENO` expansion inside `PS4` under dash, posh, busybox
ash, mksh, and FreeBSD sh, and whether one trace line is one write · `sudo`'s `use_pty`
with no controlling tty · `doas` and `su` fd forwarding · whether dash repeats `PS4`'s first
character per depth as bash and ksh do · ConPTY on a Windows-controller `ssh.exe` under
`-t`.

Register state (pointers only; content lives here): `ROADMAP` (the r31 gate text;
`arc-host-capabilities` gains stream demand) · `plans/310` (lane 3's gate) ·
`KNOBS:kCOMMS` and `kBOOT` (pointers) · `ANALYZER-NEEDS` (`an-stream-demand-per-leaf`,
`an-link-and-world-capability-supply`) · `notes/26N` §10 (superseded pointer) · `plans/26K`
§0c · `TODO-ADDTL` (the unowned arrangements). Steering (`spike/CLAUDE.md`) waits for the
lane that builds.
