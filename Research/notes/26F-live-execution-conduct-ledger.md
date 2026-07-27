# 26F — r26 live-execution conduct ledger (round close)

AI-authored (Fable conductor), 2026-07-27→29 overnight arc, three conductor seats (two
human rewinds). Remit (human-typed): get Dorc used against a live machine, whole intended
flow; conductor latitude for easily-reverted design decisions; builders at normal latitude.
This ledger is the conduct record + pointer index; the technical durables are the per-lane
notes and commits it points at. Chat-only detail is condensed here per nothing-lives-only-
in-chat law.

## Outcome, one paragraph

Dorc ran against a real machine for the first time: probe shipped over real ssh to the r26
Vultr target (~1s), apply installed real packages, the converged re-plan matched prediction
byte-for-byte, and the ceiling plan's `elide=4` reproduced the hermetic baseline EXACTLY
(the ladder cascade is not a fixture artifact). The whole round sits unified on
**`ai/r26-unify`** (tip `2248a1ba`): the transport/executor surface, the kernel-fix arc,
the 22-head builtin-deny, the `livetest` acceptance suite (green ~32s), the smoke kit with
live-evidence bundle, and an uncommitted `CONTRIBUTING.md` draft in the primary checkout.
Both gate legs green at close (Win 1605 / WSL 1601, 1 pre-existing skip each).

## Human acks (typed this arc; silence never counted)

- ack-remit + overnight latitude; single damageable-not-recreatable box.
- ack-wslc-installed (pre-release WSL containers; the update crashed the harness mid-arc).
- ack-cross-platform-steer: container tier runtime-GENERIC (docker-CLI seam), wslc = this
  machine's instance only; primary dev eventually macOS/*nix.
- ack-security-out-of-remit (spike-wide; whylog sensitivity acked-and-dismissed).
- ack-livetest-name-and-sketch (`prove` renamed `livetest`; sketch implemented as acked).
- ack-contributing-as-deliverable: final product shaped as an UNCOMMITTED CONTRIBUTING.md
  draft edit in the primary checkout (drafting medium; human rewrites), plus a
  sharp-edge hunt walking it as written.
- ack-kernel-arc directions (seat 2; see `26H` §1/§6) and ack-back-on-track (seat 3):
  kernel fixes mostly closed; ONE side-lane (deny unmodeled constructs); then executor
  completion.

## Conductor rulings (temporary-tier; flag-for-review; all cheap to revert)

- rul-tmp-usekeychain-option — ssh posture composes with user config + always
  `-o IgnoreUnknown=UseKeychain`; hermetic `-F` opt-in. (Built; posture argv-pinned.)
- rul-tmp-whylog-apply-outcomes — real apply outcomes MAY append write-only to the whylog
  (recording ≠ re-ingestion; rec-5 untouched). NEVER EXERCISED: D9 remains unbuilt; `26D`
  d3 stays an unspent human ruling.
- rul-tmp-base-branch — all r26 lanes off `ai/r28-unify`; integration on `ai/r26-unify`.
- rul-tmp-gitattributes — root `.gitattributes` `* text=auto eol=lf` + narrow binary pin;
  measured zero-diff at introduction. LANDED.
- rul-tmp-loom-env-key — authorized a loom-vocabulary `env` key; LAPSED UNUSED (the
  builder found the sanctioned I/O-failure canonical-payload floor covered all four
  transport codes; the vocabulary stayed closed).
- rul-tmp-exit-renumber — strawman-formats-no-compat renumbering; landed as 12/13/14/15
  (ingress-refused · host-not-reached · session-lost · apply-failed; 13≠14 is
  load-bearing: retry-safe vs may-have-run).
- rul-tmp-sizing — 2GB box over guardrail-cheapest (`255` §8). Provisioned.
- Guard-void fix authorized as "complete W-B's symmetry" with a stop-clause on licensing
  changes — the stop-clause FIRED (correctly): see fnd-classed-decline below; no fix landed.

## Lane index (tips; all folded into `ai/r26-unify` @ `2248a1ba`)

- live-target — box provisioned + `26E` note (`755f7352` line). Box: `140.82.10.231`,
  vc2-1c-2gb/ewr/Debian-12, tag `dorc-r26`, pristine snapshot `ff770de9`; left RUNNING;
  teardown human-ack-only.
- engine-executor — `dorc-transport` (std-only, dep-free), 3 drivers, sentinel, posture,
  real records identity, CRLF gate, `--host [user@]host[:port]`, T1 closed loop.
- kernel-fix arc (seat 2) — `26G` (+3 corrections) · `26H` plan · `26I` adversarial
  review; W-A/W-A2/W-D/W-B/W-C landed.
- builtin-deny — `26J`; 22 heads ⊤-degrade at both tracers' plain-command arms;
  `26I:fnd-state-builtins-silently-mis-key` closed, evidence reproduced-then-pinned.
- smoke-kit + livetest + live blood — `Research/trial/r26/` (book · oracles · predictions
  with re-registrations · renders · records · live-evidence/ · README one-pager);
  `spike/e2e/livetest.sh` + tasks `livetest{,:target,:clean,:remote,:baselines}`;
  runtime seam `DORC_CONTAINER_CLI` → absolute-path autodetect.
- unify/close — merge graph, exit-code family, `ControlMaster=no`/`ControlPath=none`
  pinned (a user config's `%p` colon breaks Windows sockets; `260` §5 rewritten).

## Findings index (live where cited)

- fnd-classed-decline-unwalls-guard-tier — THE open ruling (`trial/r26/predictions.md` §7;
  case pair `guard26-*`): a verdict-bearing site stops walling even when it DECLINES, so
  vouched drops below it reach neither elide nor guard (guard tier keyed
  `EstablishWritten` only) — classed honesty yields a strictly worse plan than silence,
  inverting contract monotonicity. Supersedes-and-absorbs the W-B "ambient-past-wall has
  no guard tier" banked item (same root). Repair widens guard licensing ⇒ HUMAN.
  NB `del-authored-coordinate-voids-guard` was a misdiagnosis (confounded A/B via the
  auto-cell stomp); W-B's keying is symmetric and innocent.
- fnd-smoke-book-never-reloads-nginx — real book bug only live execution could find
  (postinst starts the daemon before the config drop; `start` no-ops on a running unit);
  deliberately unfixed to preserve pre-registration.
- `26J` residue — lint silent on deny-dead oracles (which-roles-to-scan ruling wanted) ·
  verdict-lane ⊤ reasons reach no surface (pre-existing, general) · book-side `set --`
  sub-form decision.
- `local` is denied but dialect-blessed ("POSIX + `local`") — modeling local-assignment
  in both tracers is the obvious next increment before real oracle authorship.
- Sharp edges, all verified: transport diags all `[unwritten:]` · `--host` absent from
  `--help` · CRLF refusal prints no line number · known-hosts churn (livetest self-scopes;
  `ssh-keygen -R` by hand) · WSL-loopback asymmetry (containers reachable from git-bash
  only) · `MSYS_NO_PATHCONV` scoping · wslc PATH-not-inherited (absolute-path probe).
- 26E erratum: curl/jq WERE present on the box at recon (note claimed absent).

## Ops notes

- wslc (`C:\Program Files\WSL\wslc.exe`, 2.9.4.0): verified verb intersection
  `pull · run --rm -i … sh -s · run -d --name -p · exec -i · remove -f · --format json`;
  no `--privileged`/`--cap-add`/socket ⇒ no systemd-in-container (hence
  `container-book.sh`). WSL login shells post-update hang on a mise prompt — automation
  uses `--shell-type none`; fresh worktrees need `mise trust` BOTH sides.
- Vultr guardrails held: one instance ever; `eurydice` untouched (tag-guards structural);
  KEEP_ON_FAIL=1 deviation recorded (auto-destroy vs no-destroy fence; never fired).
