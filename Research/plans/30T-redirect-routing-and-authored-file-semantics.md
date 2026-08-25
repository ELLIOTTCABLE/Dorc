# 30T — Authored file semantics: redirect routing, referent identity, and the ask-the-world architecture

> Tier: EXPLORATION/PROPOSAL — LLM-authored (Fable; the `r30-design-duck-file-paths-and-redirects`
> sitting, 2026-08-25; human present and steering throughout). This is a design-duck REPORT, not a
> rulings ledger: NOTHING herein is ruled. Grades: **[SESSION]** the human said/typed it in this
> sitting (steering, not a ruling) · **[PRIOR]** already ruled elsewhere, cited · **[PROPOSED]**
> this report's own synthesis, awaiting a sitting and/or adversarial review. Cite no commitments
> from this document. Subordinate to the root docs, `spike/CLAUDE.md`, `KNOBS.md`, `plans/30I`,
> `plans/30P`. Registers (FORFEITS/ANALYZER-NEEDS/KNOBS) are deliberately UNTOUCHED by this
> report; §12 lists what adoption would owe them. Written for possible adversarial review;
> uncommitted by instruction.
>
> Plain language throughout, per the human's standing instruction for this topic. All sh is
> STRAWMAN — conversation-grade, deliberately dialect-sloppy (rc-arity crimes, GNU `stat -c`
> where BSD spells `stat -f`; the platform variance living in authored files is the point).

## §1 the-problem — writes that live outside argv

Revives `26K:sit-redirect-routing-composes-oracle-channels`; the surviving open content is
`26Lb:cell-write-elision-needs-a-vouch-holder`. The founding finding
(`finding-redirect-writes-live-outside-argv`, r26 glue round):

The oracle contract is argv-keyed by law (`identity-declared-never-inferred`) — and a redirect
target never reaches argv. `cat >f <<EOF`, `printf … >f`, `cmd >>f`: the single most common way
shell mutates a file is structurally invisible to the mechanism that describes mutations. An
fs-stdlib author cannot write a `cat__disturbs` naming the target, because `cat`'s argv on that
line is empty. Consequence: every such write is an unmodeled mutation ⇒ a total wall, and the
careful admin's write-if-changed idiom *self-walls* — building the candidate file poisons every
fact below it. `26K` billed the repair "possibly the highest-value single analyzer increment"
(billing tempered in §10, `risk-billing-correction`).

`30D` [PRIOR] closed only the *channel-claim* leg (what an oracle may say about a command's
stdout/stderr). Unruled: routing (where claimed bytes go), File-coordinate binding, and who
authors the convergence compare.

## §2 the-teachability-artifact — user-facing description

Included per the human's test [SESSION]: "if we can't teach this to a user, it's the wrong
design." Draft register; capabilities and caveats only; no engine internals.

---

**How Dorc handles files.** Dorc reads shell, so it can see *that* your book writes a file.
What it cannot see, and deliberately never guesses, is what that path *means*: whether two
spellings name the same file, whether "already has these bytes" counts as done, or whether a
path is an ordinary file at all (on Linux, writing to certain paths under `/proc` reboots the
machine). Filesystems differ too much — case-insensitive disks, symlinks and bind mounts,
SELinux labels, network mounts — for built-in opinions to be safe anywhere but the machine in
question. A wrong opinion here commits the one sin Dorc refuses: skipping a command that
needed to run.

So Dorc holds no opinions about files. Every file question is answered one of three ways:

1. **An oracle answers, on the machine in question.** The filesystem oracle in the standard
   library is ordinary sh: small functions answering "are these two names the same file?" or
   "does this file already match — contents, and whatever else matters here (labels,
   ownership)?" Dorc runs them where the answer lives: on the target host, during the
   read-only probe, or as a check placed immediately in front of your own command at apply
   time. Because the answers come from your actual filesystem rather than a model of one,
   they are automatically right for *that* machine — including machines nobody designed for.
2. **Dorc verifies its own work instead of predicting it.** When Dorc itself places files on
   a host (your plan, and the oracle files it travels with), it copies, then checks every
   file landed exactly, before the first line of your book runs. A host that disagrees with
   the plan's assumptions stops the run cleanly; it never bends it.
3. **Nobody answers → the line stays.** A write nobody can speak for is treated as touching
   *something unknown*: it runs, and everything below it stays in your plan — running, or
   guarded by a fresh check. Silence never removes a line; only an answer with a name
   attached can.

*What this buys you:* the most common mutation in real shell — writing a file — stops
costing you the rest of your plan. If you already write careful scripts, you have been
feeding this machinery all along: `cmp -s conf.new conf || cp conf.new conf` is exactly the
check-then-act shape Dorc lifts.

*What it asks of oracle authors:* answer what you know in plain sh; `return 2` what you
don't; *decline* the strange corners of the path namespace (`/proc`, `/dev`, network mounts)
rather than guessing — a declined path just means that write stays in the plan.

*Caveats:* everything fails toward running (missing knowledge costs attention, never
correctness) · files that don't exist yet often answer can't-say · "these are different
files" is the sharpest sentence in the system — a wrong "different" is how a needed command
gets skipped; such answers are always attributed by name, and any removal resting on one
past a command that actually ran additionally sits behind `--risk-faultless-skips`.

---

## §3 the-design-in-one-screen

- **`prop-engine-holds-no-world-facts`** [PROPOSED; the candidate invariant] — no
  platform-conditional fact about the world may be load-bearing in engine code; it may exist
  only as authored speech (oracle/kind-owner sh) or as a phase-time measured answer. The
  engine's verbs are: *parse* (sh structure), *compile questions*, *fold answers* (the
  algebra), *act-and-verify its own mutations*, and *arrange geometry* so authored sh is true
  (`30P`'s mirroring). Never: interpret the world.
- **`prop-routing-graph-third-edge`** — the shell's plumbing connects channels to three
  target types. channel→channel (pipes): built — composed predicts
  (`rul-only-oracle-bytes-ship`). channel→value (`$(…)`): designed — the capture lane
  (`275`). channel→world (`> f`): the gap this design fills. `tee f` needs nothing: a file
  named in argv is tool semantics, already oracle territory.
- **`prop-locators-not-coordinates`** — the engine's parse yields structural *locators*
  ("stdout of site S routes to word W, truncate-mode, under cwd-state C"), warranted by
  POSIX, not by any author. Binding a locator into a *coordinate* (`sm.dorc.File:…@contents`)
  is a vocabulary act and is AUTHORED — a kind-owner body receiving the locator, exactly as
  argv flows through an oracle's argparse (`rul-argv-flows-bytes-do-not`, generalized:
  structure flows through authored interpreters). Precedent for the species: `272`'s
  "addresses are never coordinates"; `kind__state_stored_only_in()` already *emits* this
  same locator species.
- **`prop-decline-is-the-authored-value`** — the strongest reason binding is authored is not
  override (thin; §6) but *decline*: which path-prefixes are not-really-files is
  world-taxonomy, platform-conditional, and the corpus already ruled such lists are
  platform-oracle speech, never an engine denylist (`30S` precedent). A declined locator
  claims nothing; the wall stands; safe.
- **`prop-questions-route-to-phases`** — every world-question has a *latest sound phase*:
  analysis may speculate only inside the audited decidable set (checked at probe standup);
  the probe measures; a guard re-measures at apply, in sequence, staleness-free by
  construction. Read {elide, guard, run} as the existing instance: elide = answered at
  probe; guard = deferred to apply; run = never answered. The engine defers what it cannot
  answer; it never invents an answer.
- **`prop-identity-is-measured-relationally`** — file identity is answered by live pairwise
  host questions (`-ef`/inode-class), not by modeling fold/symlink/mount rules: the kernel
  answering subsumes every aliasing mechanism, including unforeseen ones. Future (not yet
  existing) referents are sited by *anchoring*: living-canonicalize the parent, look up the
  exact name (the lookup is fold-aware for free — the kernel folds), and let the engine's
  CFG screen ("no other line targets this path in between") cover the program half. A
  future referent can only come to alias an existing one if some *agent* creates the alias,
  and agents are either in-book (engine-visible, wallable) or out-of-book drift (excluded
  by the identified-cause TOCTOU scope, `toctou-scope`) [SESSION — the anchoring insight is
  the human's; the closure argument is the sitting's elaboration of it].

## §4 the-walkthrough — one book, two authored functions, every phase

The strawman book and fs-stdlib file (dialect-sloppy; see header):

```sh
#!/bin/sh
# harden.sh
set -eu
CONF=/etc/ssh/sshd_config
sed 's/^#PasswordAuth.*/PasswordAuthentication no/' "$CONF.dist" > "$CONF.new"  # A: candidate, always runs
cmp -s "$CONF.new" "$CONF" || cp "$CONF.new" "$CONF"                            # B: admin's write-if-changed
systemctl reload ssh                                                            # C
```

```sh
# fs.oracle.sh   # dorc-lang/v0.2  (STRAWMAN)
sm_dorc_File__same() {                  # pairwise identity, host-run
   [ -e "$1" ] && [ -e "$2" ] || return 2
   [ "$1" -ef "$2" ]                    # 0 = same referent; 1 = distinct
}
cp__is_converged() {
   [ "$#" -eq 2 ] || return 2
   cmp -s -- "$1" "$2" || return 1                                : sm.dorc.File:"$2"@contents
   if command -v getenforce >/dev/null 2>&1; then                 # platform dispatch, in sh
      [ "$(stat -c %C -- "$2")" = "$(matchpathcon -n -- "$2")" ] \
         || return 1                                              : sm.dorc.File:"$2"@seclabel
   fi
   return 0
}
```

The SELinux knowledge is *generative*, not defensive: on an enforcing host,
bytes-equal-but-mislabeled answers diverged ⇒ the `cp` RUNS (a naive content-match would
wrongly elide the line whose whole point was the label repair); on a converged enforcing
host both dimensions pass ⇒ elide. Dorc never learns what a label is.

| phase | runs where | what happens to the authored fs code |
|---|---|---|
| analysis | controller (pure text) | *abstract* calls only: role recognition by name; the tracer walks `cp__is_converged` with `$1/$2` bound to site B's value-flowed argv, asking ONLY structure — is a verdict-marked path reachable (vouch)? which cells do reached marks name (backing)? does the effect-falsifier see provable mutation? `cmp`/`getenforce` are opaque, never evaluated. The binder's `case` arm over site A's literal redirect word is traced (decidable-set-only) ⇒ footprint claim `File:….new`, attributed to the fs stdlib. The engine also *mints questions it cannot answer*: A always runs ⇒ wall; B-below-A ⇒ a pair-question `same('….new','…config')?` joins the probe want-list |
| probe standup | target host sh | engine scaffolding: capability handshake; falsification of anything the tracer pre-evaluated (the binder arm) — host runs the same authored arm, disagreement ⇒ Refused pre-consent. Checks the tracer's reading of the author's sh, never the world |
| probe | target host sh | shipped stripped bodies run with site argv: the verdict (platform branch decided by the host at runtime) and the pairwise `__same` answering the engine's routed question. rc's return through bounded intake; the host answered multiple-choice and minted no vocabulary |
| plan mint | controller | folds only: vouch + measured-converged + (pair-answer = distinct) + the typed flag ⇒ B elides although A runs. No calls |
| apply standup | target host sh | artifact integrity: manifest byte-match, cwd parity, `$0` shape. Mismatch ⇒ stop before line one |
| apply | target host sh | the latest-phase spelling of any unanswered question: an inserted guard `( check ) || original` for guarded sites — same authored bytes, freshest ask. Site B needs no insertion ever: the admin's `cmp || cp` IS the guard (no-double-guard) |
| `dorc why` | controller | bodies displayed, never run: the reached arm inlined (show-the-code), the chain typed measured/vouched/measured/consented with names |

## §5 the-farm-out-map — what routes to authored code, what stays

**Farms out** (authored bodies, phase-answered): referent identity (living pairwise +
anchored future-siting) · convergence adequacy and its selectors (@contents/@seclabel/@xattr
— an engine-frozen Rust definition of "File converged" would be permanently wrong on
SELinux/xattr hosts) · path-taxonomy declines (/proc, /sys, /dev, tmpfs, network mounts) ·
volatility judgments · cross-kind topology (`state_stored_only_in`) · content-cell minting
(the `cmp`/heredoc establish side — the same seam as
`FORFEITS:forfeit-file-content-facts-from-exact-checks` and
`forfeit-content-establishment-by-known-write`).

**Stays engine, all platform-free** [PROPOSED as the complete residue]:
1. the algebra — lattices, the compare chokepoint and its unknown-safe-bottom, claim tiers,
   phase placement, conservative defaults;
2. question compilation — the locator parse; deriving which pairs are worth asking; batching
   into the single probe shot;
3. the bounded-intake edge (`rul-host-bytes-bounded-before-admission`);
4. the CFG interference screen — reaching-defs over path cells; the one question no host or
   author can see;
5. the first-contact bootstrap (the `kBOOT` floor: the first question cannot arrive via the
   machinery it establishes).

**The split, one line per cell:** *language* semantics (word expansion, dot-resolution,
errexit) exist once, in Rust, falsified by the floor differential and the standups;
*controller* filesystem = acts (open the authored spelling, identify by digest — never
lexically normalize for identity); *host* filesystem = phases and authored bodies;
*geometry* = arranged (mirroring makes `$(dirname "$0")` true), never interpreted.

**The one genuine dual-implementation tax**, named honestly [the human's core objection this
sitting; SESSION]: the decidable-set tracer is a Rust implementation of an sh subset. It is
*mandatory only in the load plane*, which is phase-zero-captive — "what program am I
analyzing" must be answered before any phase exists, so deferral has a floor and the load
plane sits on it. The tax is bounded three ways ([PRIOR], `30P:the-load-plane-stays-correct`):
closed set, growth by-name at license-review-tier, per-host standup falsification. Everywhere
else, Rust pre-evaluation of authored bodies is an *optional* speculation under the same
falsification, added only where profiling demands.

**Trust typing** [PROPOSED obligation]: every routed question is classified by its consumer's
failure direction before it may route. Degradation-grade answers (handshake-style: absent ⇒
less value; a lying host hurts only itself) may feed unflagged machinery; identity/disjointness
answers feed flag-gated, claim-tiered consumers (`an-host-as-adversary` standing). Forgetting
the classification once is a soundness hole; it wants to be typed, not conventional.

## §6 the-critical-examination — what fell, what survived

Conducted at the human's direction [SESSION], three questions.

- **Override value is thin.** The argument that made *oracles* authored does not transfer:
  tool semantics are plural and distributed; redirect semantics are singular and spec-fixed.
  A rival fs kind would fragment the coordinate space (cross-kind compare does not exist), so
  dissent destroys value rather than expressing taste. The genuine authored value is
  *decline* (world-taxonomy in sh, per-platform, patchable) and repair-without-release. The
  attribution argument is remedy-shaped, not blame-shaped [SESSION — the human's correction:
  a `dorc why` pointing at a folder of sh is trivially patchable and PR-able; better for the
  user, marginally; not carrying the day].
- **Single-inhabitant, as charged.** The v1 census of a general "locator registry" is one
  subscriber (fs) and one-and-a-half members. `redirect-in` is worthless by ruled design
  (reads don't kill facts); cwd needs no kind hook; heredoc-payload's consumer is a forfeited
  capture path. Resolution: build narrow (one fs role member, no registry), let the second
  *kind*-subscriber force generalization (rule-of-three);
  `rul-strawman-formats-no-compat` makes the later rename free. The human's condition
  [SESSION]: a general mechanism is *preferred* if it pays for itself — the shared-currency
  case is that `state_stored_only_in` emits and the reflexive-inertness falsifier consumes
  the same locator species; both, honestly, are unbuilt.
- **The hardcode stays on the table.** Engine-minted File coordinates (one blessed
  vocabulary act + an engine-resident decline list) may genuinely be cheaper; the sitting
  should compare it undismissed. ~SUSPECT narrow-authored wins on the decline-taxonomy
  point alone; it is a close call, not a slam dunk.

## §7 the-platform-wedges — how bad the paths space is

Six classes; the fix-location verdict per class:

1. `wedge-writes-are-actions` — /proc/sys (sysctl), /sys knobs (`/sys/power/state`: write
   "mem", the machine suspends; `/proc/sysrq-trigger`: write "b", it reboots), /dev/kmsg,
   raw block devices. Platform-conditional existence: macOS and OpenBSD have no procfs;
   FreeBSD's is deprecated; Cygwin fakes one *including `/proc/registry`*. → authored
   declines / future re-kinding. Farms out.
2. `wedge-paths-that-are-channels` — /dev/stdout, /dev/fd/N, /proc/self/fd/N: path-shaped
   spellings of routing edges; binding them as File is category error. The engine/author
   boundary is not clean along path syntax. → decline-list pragmatically; arguably engine
   (fd semantics). Small.
3. `wedge-identity-is-relational` — symlink farms (/etc/resolv.conf → systemd-resolved's
   stub), Debian alternatives, usrmerge (`/bin/sh` ≡ `/usr/bin/sh` on merged distros only),
   hardlinks (no canonical name exists; only pairwise dev+inode answers), bind mounts
   (realpath gives different canonicals for one referent), macOS firmlinks; case: APFS/HFS+
   and NTFS case-insensitive by default, ext4 casefold and NTFS case-sensitivity
   *per-directory*; Unicode normalization (HFS+ NFD; APFS normalization-insensitive
   ~SUSPECT on fine detail). → farms out via living pairwise questions; the taxonomy
   collapses when the kernel answers. The largest class.
4. `wedge-identity-is-contextual` — mount namespaces (systemd `PrivateTmp=yes`: "the same
   /tmp" is two referents), chroot, containers, WSL /mnt/c ↔ C:\. → the coordinate's
   context slot + `27C` context-entry; engine-keyed, authored-answered; already designed.
5. `wedge-cross-kind-shadowing` — a unit drop-in under /etc/systemd/… changes the *Service*
   entity; ld.so.cache; PATH. Not solved by either design; the designed counter-mechanism
   is the Service kind declaring where its state lives (`state_stored_only_in`), which
   *consumes File coordinates* — this design is its prerequisite feeder.
6. `wedge-adequacy-and-volatility` — setcap/xattr and SELinux contexts make cmp-equal ≠
   converged; tmpfs lifetime; NFS close-to-open (a "local" File that is secretly cross-host
   state, `an-cross-host-kind`); automounts, where a probe's *stat* mutates mount state
   (through `hermeticity-precondition`). → authored selectors and declines. Farms out.

Verdict [the sitting's honest sizing]: the abstraction buys out the taxonomy and adequacy
fiddle entirely, and the identity fiddle *almost* entirely (via §3's relational re-cut —
this corrects an earlier within-sitting "identity is neutral" claim, which had scored
engine-manhours where the human's stated metric was blessed engine semantics [SESSION]).
The genuinely-owed engine work that remains — pairwise question plumbing, pair enumeration,
context keying — is generic and platform-free.

## §8 the-identity-residues — where can't-say genuinely remains

- `residue-two-future-spellings` — `D/Foo` and `D/foo`, neither exists: folding is
  undecidable without directory-attribute reads (`lsattr` casefold on ext4; ~SUSPECT
  pathconf-style answers on macOS) — authorable, platform-oracle content, ugly; resting at
  can't-say is safe and narrow.
- `residue-second-exchange` — pairs involving a *dynamically-derived* footprint are not
  known until first-round results return; asking them is a second host exchange, squarely
  behind `rul-repeated-probing-reviewed-before-design`. The static majority ships in the
  ordinary single shot; the dynamic cell opens as collide-until-reviewed.
- `residue-floor-membership` — `test -ef` is a common extension, -GUESS on posh∩dash floor
  membership; must be *measured* (atlas-style) before any stdlib body leans on it; a
  fallback spelling (`stat` inode compare) has its own portability sludge, which is exactly
  platform-oracle content.

## §9 integration-with-r30 — cwd, loading, bundling

Read against `plans/30I` + `plans/30P` (this sitting):

- **The load plane is the built prototype, not a counterexample.** `mech-two-standups` +
  the controller-expectation/host-check pattern (host influence strictly boolean: stop,
  never steer) IS the ask-the-world architecture, already ruled. And
  `rul-static-predict-sites-loads` already *breaches* the naive "license-plane never farms"
  wall in exactly the gated form this design wants: an authored stdlib body (`dirname`)
  sites loads, engine-evaluated (P-static), call-graph-blessed by name (P-blessed),
  standup-verified, why-cited, priced as a named trust edge. The redirect/identity work
  should copy this gate, not invent one.
- **cwd stays engine** — shell state, not world state (`rul-unsure-falls-toward-sh-parity`).
  The binder consumes cwd-state as an input: a locator is *(word, cwd-state, mode)*, and
  `law-no-unsoundness-below-a-blind-act` bounds it for free — below a havoc, cwd is ⊤ ⇒ the
  derived File coordinate is ⊤ ⇒ the wall stays total. State this explicitly in any binder
  ruling so nobody recovers a coordinate from the literal word below a havoc.
- **Bundling needs no target-fs model.** The wedge: tree-mirror ships authored paths, so
  two paths differing only by case silently collide on an APFS/NTFS/drvfs target. The
  design already holds the answer: the apply-standup manifest check (every file present and
  byte-matching) is the falsifier — one of the two fails byte-match ⇒ stop before line one.
  Cost of not-modeling is only *lateness*; the cheap recovery is an aid-tier collision scan
  for early warning, zero soundness role.
- **Controller fs by acts** — the `30Pb` builder-check (lexical `..` vs the real open under
  symlinks/case-folding) resolves by act-don't-model: open the authored spelling (the
  kernel resolves symlinks), identify by content digest and open-time identity; never
  lexically normalize a path for identity. Same sentence the host-side pairwise design
  produced — evidence the principle is load-bearing rather than local.
- **Arrange-the-world is the third leg** — `rul-rewrite-permission-is-derived`'s mirroring
  keeps `$(dirname "$0")/x` verbatim and constructs geometry so it lands (`30P` spots the
  same move in PyInstaller). Measure / act / arrange: the engine's complete fs repertoire,
  none of it interpretive.
- **Sequencing** — the binder inherits whatever cwd precision `lane-load-plane-precision`'s
  residue lands (`an-cwd-state` full book-flow still owed, `30I` §3.2); scheduling the
  redirect work after it is free correctness.

## §10 the-risk-register

1. `risk-cross-kind-referent-aliasing` — the sharpest: `sysctl -w net.ipv4.ip_forward=1`
   (Sysctl-kind via its oracle) and `printf 1 > /proc/sys/net/ipv4/ip_forward` (File-bound
   via the binder) are one referent in two kinds; per stage-7 semantics cross-kind
   footprint-vs-backing reads survival-compatible ⇒ a File-bound wall can spare a
   Sysctl-backed fact it really mutated. Under-execution, flag-gated; within-kind
   `resolve()` cannot see it. Mitigation is authored declines of the known special
   namespaces; the safe spelling is backwards from instinct (an allowlist of "boring paths"
   is unwritable, so it is denylist-plus-claim-the-rest, which fails unsafe on unknown
   special namespaces). The residue must be priced at the sitting.
2. `risk-resolver-goes-hot-path` — every `>` in every book mints a File coordinate; the fs
   identity machinery becomes load-bearing at book scale. Probe-lane cost plus a much wider
   aliasing-knife surface.
3. `risk-oracle-body-scoping` — resolved in-principle this sitting, must be ruled: the
   parse applies everywhere, but consumers split along the ownership split
   (`rul-probe-mutation-ownership-split`): book sites → the fact plane; oracle-body
   redirects → the reflexive-inertness falsifier (falsification-first, never a completeness
   gate; vouched residue otherwise). `DREP_V1` is exempt *by construction*, not by grammar:
   the sink value is engine-supplied, so the target is an engine-owned resource. Without
   the split, every emitting oracle's `>>"${DREP_V1:-/dev/null}"` would mint File
   disturbances inside probe bodies.
4. `risk-exec-redirect-routing-state` — `exec > f` makes routing persistent CFG-carried
   state; fd-dup chains ride the deferred `an-fd-state`; group redirects, `>|`, noclobber.
   The sitting must carve an EXACT-or-havoc boundary for routing, mirroring
   `30P:rul-load-head-is-exact-or-havoc`, or the routing fact is quietly wrong below an
   `exec`-redirect.
5. `risk-decidable-set-governance` — one decidable set, one fence: the redirect work and
   any fs-oracle static tier must feed the same by-name, license-review-tier list
   (`30P`/`30Q` §5g posture), never mint a sibling set with laxer review. The standing
   temptation is widening the tracer without the standup check, because it is faster and
   usually right.
6. `risk-billing-correction` — `26K`'s "highest-value single analyzer increment" needs
   tempering: the sparing value requires modeled-producer ∧ flag-on ∧ fs-kind-loaded ∧
   identity-adequate; an unmodeled producer's own ⊤ keeps its wall total regardless. The
   unconditional part is kill-side conservatism plus the attention story for modeled
   producers. Still likely the right next increment; -GUESS no longer obviously the
   highest-value one.
7. `risk-wall-license-conflation` — a phrasing trap caught by the human [SESSION]: "can
   only narrow a wall, never license an elision" conflates causal accounting (narrower
   walls DO yield more surviving elisions — the point) with license-source accounting (the
   derived coordinate participates only on the wall side of the disjointness compare; every
   survival still rests on its own vouch + measurement + flag). Any ruling text must keep
   the two ledgers separate or it will be argued against successfully.

## §11 candidate-rulings-and-asks — for a sitting or review

1. `ask-adopt-no-world-facts-invariant` — adopt §3's `prop-engine-holds-no-world-facts` as
   steering law, with §5's five-item residue list as the complete permitted exceptions
   (future work argues against the list, never quietly grows it).
2. `ask-question-species-census` — enumerate the closed set of authored answer surfaces
   (the `277:an-generator-registry` concept, generalized): verdict-rc · mark→cell ·
   emission→footprint · locator→claim · pairwise-identity · the blessed lift. Authors fill
   surfaces; only the engine mints a new species. The census's size and generality is the
   design core; everything in §1–§8 is evidence about which surfaces earn existence.
3. `ask-binder-narrow-vs-registry` — one narrowly-named fs role member now vs the locator
   registry; rule-of-three governs generalization; the engine-hardcode compared undismissed
   (§6).
4. `ask-pairwise-identity-member` — the `kind__same()` shape: name-bias per
   `an-name-as-contract` (0 = same, the lazy-safe direction; "distinct" must be the
   deliberate claim); floor membership of `-ef` measured first (§8).
5. `ask-kind-species-verdict-member` — structural sites (redirect/heredoc writes) need a
   convergence judgment-holder that is not argv-reachable: a KIND-species verdict member,
   by-new-name. This is `26Lb:cell-write-elision-needs-a-vouch-holder`'s answer-shape: the
   authorship menu (lifted admin idiom / tool oracle / kind verdict) is not a choice — all
   three coexist as rungs of the standard gradient.
6. `ask-oracle-body-scoping-rule` — rule §10's split-routed-consumers shape explicitly.
7. `ask-exec-redirect-carve` — the routing EXACT-or-havoc boundary (§10 item 4).
8. `ask-second-exchange-posture` — dynamic-pair questions open as collide-until-reviewed
   behind `rul-repeated-probing-reviewed-before-design` (§8).
9. `ask-phase-provenance-field` — record which phase answered each consumed answer
   (extends the `SpeechAct`/`Knowability` seat: "measured-at-probe, screened" vs
   "live-at-guard"); cheap, and `dorc why` gets honester.
10. `ask-case-collision-red` — an e2e case pinning "apply standup stops on a case-folded
    manifest collision" when the multipart lane next opens; the cell greens silently on
    every Linux CI box and bites the first macOS target.
11. `ask-payload-staging-sitting` — where candidate payload bytes live when a kind verdict
    compares them on-host (`rul-probe-writes-only-what-it-owns` permits owned scratch; the
    staging story is unexamined).

## §12 register-impact-if-adopted (nothing edited by this report)

- **FORFEITS** — `forfeit-file-content-facts-from-exact-checks` and
  `forfeit-content-establishment-by-known-write` sit on this seam (the establish side of
  the same routing edge); adopting the design rewrites their CAPTURE paths through the
  locator/binder machinery. No new rows owed until a sitting declines something.
- **ANALYZER-NEEDS** — candidate rows: `an-routing-locators` (the structural species; note
  `state_stored_only_in` as its existing emitter) · `an-pairwise-referent-identity` (the
  relational compare generator; pair enumeration; the second-exchange gate) ·
  `an-structural-site-convergence` (the kind-species verdict). §D's
  `an-redirection-effect` row gains the routed-coordinate consumer.
- **KNOBS** — no new tension claimed. One candidate worth checking against the registry
  rule (report, don't mint): phase-routing purity vs phase-zero diagnostic quality (lint
  mode knows less under this design than under a Rust world-model that guesses) — possibly
  a genuine A-vs-B, possibly just `kPRECISION` wearing new clothes.
- **USER_STORY** — unaffected; stage 1's guard-lift continuity bet is exactly what the
  write-if-changed story extends. The `cp`-elides-but-`cat >f`-cannot asymmetry (identical
  semantics, different authored homes) is the crispest motivating example for any future
  stage-text, if the human ever wants one.
