# 27A — Cross-context transport: the hole, its walls, and the open fork

AI-authored (Fable, rubber-duck dialogue with the human, 2026-07-13). The durable of the
transport descent that followed the `279f` adjudication. Authority: root docs and
human-TYPED rulings outrank this; **the §6 ack-ledger states exactly what the human has
typed — everything else herein is proposal-tier.** This note serves
`279f:ask-transport-disposition` (the §3 refusal of the `275` §6 ratifications stands;
this dialogue pre-chews the block-context decision) and carries the substance
`279f:ask-flag-boundary-recut` should consume. It supersedes the mechanical-closure shape
sketched in the post-crosscheck conversational summary (that shape was demoted mid-dialogue
— see §5).

> **[SUPERSEDED-IN-PART — 2026-07-16, `27C` (context-entry probing), human-ruled.]**
> This note remains the wall inventory FOR THE TRANSPORT LANE, which survives only as a
> demoted, flag-tier fallback (`27C` §5). The primary lane is now measurement in the
> site's own context under the escalation dial × tolerance vouch (`27C` §§2–4): the §1
> "standing invariant" premise was re-scoped (never human-welded; see
> `27C:rule-reuse-never-acquire`), the §3 user-axis fork is MOOT in the default cell
> (re-posed flag-tier in fallback cells only), and fence-escalating-probes graduated
> from someday-flag to the designed default lane. Read `27C` first; read this for the
> transport-lane law it still carries.

Product-tier vocabulary used throughout (the human's, minted this dialogue): **product A**
= actual elision (the attention product; the golden hill); **product B** = guards (fast
and safe, never shorter); **product C** = flag-gated risk acceptance; **product D** =
lints/hints. Design work aims at A. B is the floor. C and D are fences, never solutions
("non-provably-complete claims don't drive design-decisions about provably-complete
claims").

## §1 — The problem, stated fully (NOT SOLVED)

Probes execute in the SSH user's context and never escalate (standing invariant —
**[27C, 2026-07-16: provenance-audited as proposal-tier, never human-welded; re-scoped
to reuse-never-acquire]**). Real
books wrap most mutating lines in context-changers: `sudo`/`su` (the user axis),
`chroot`/mount-tricks (fs-view), `ip netns exec` (netns), `env` (ρ). Every plan decision
about a wrapped line therefore consumes measurements taken in a *different context* than
the one the line will run in. The question is what licenses that consumption.

**The license gap** (`279f` §3, sharpened here): the `:?` backing mark is a positive
disclosure — "this read reads X" — with, by ruling, *no completeness burden* (`24D`). Any
transport argument that treats the backing as "everything the answer depends on" smuggles
in a completeness-plus-determinism premise that no authored mark provides. A check body
can honestly disclose one read while its *answer* also depends on an unmarked input; the
transported wrongness then has **no wrong line anywhere** — un-attributable, which by the
horizon doctrine (IMPLEMENTATION: errors we can't attribute are our fault) makes it Dorc's
own defect, not a priced user trade. The failure shape is the cardinal sin
(under-execution), re-entering through the fact and value planes.

**The gap decomposes into two halves with different natures:**
- the *sh-visible* half — the check body itself reads asker-identity (`$HOME`, `$USER`,
  `id` captures). Mechanically detectable, deterministic, hygiene-tier — and detecting it
  SOLVES NOTHING (human grading: borderline inconsequential; build it, move on).
- the *tool-internal* half — the delegated-to binary consults per-asker state invisibly
  (a cache under `~`, an identity lookup). Frame problem, **permanent**. No analysis,
  mark, or cleverness ever sees it.

### The hole firing end-to-end (two full walks, one compressed)

**Walk 1 — user axis, tool-internal state (the knife cell).** The book line:

```sh
sudo pipx install poddle          # root's pipx tree: /root/.local
```

The oracle — babby-minimal, honest, sh-clean (STRAWMAN spelling):

```sh
pipx__is_converged() {
   verb="$1"; shift
   case "$verb" in
   install) pipx list --short 2>/dev/null | grep -q "^$1 " ;;
   *) return 2 ;;
   esac
}
```

The walk: (1) the probe runs the check as alice; `pipx list` consults pipx's per-user
tree — *inside the binary*; `$HOME` never appears in the oracle's text — and reads
`/home/alice/.local/pipx`. (2) Alice happens to have poddle (she uses it for dev):
rc 0, converged. (3) The site's world is root's tree, `/root/.local`, where poddle is
absent. (4) The line elides; root never gets poddle. Under-execution — and **no line
anywhere is wrong**: the book is idiomatic, the oracle honestly delegates, pipx behaved
as documented. Un-attributable ⇒ ours.

**Walk 2 — fs-view axis, chroot provisioning (near-100% fire rate).** The book:

```sh
mount /dev/sdb2 /mnt/target
chroot /mnt/target apt-get install -y openssh-server
```

The oracle: the *stdlib* dpkg/apt oracle — battle-tested, correct:

```sh
apt_get__is_converged() {
   # ...
   install) dpkg -s "$1" >/dev/null 2>&1 ;;
}
```

The walk: (1) the probe runs `dpkg -s openssh-server` in the host context, reading the
HOST's `/var/lib/dpkg/status`. (2) The host has sshd — you are SSH'd into it — so:
converged. (3) The site's world is `/mnt/target`'s database, where it is absent. (4) The
line elides; the image ships without sshd. Note the viciousness: the host nearly always
already has the packages being installed into a target, so this fires almost *every*
time — and the failing oracle is the best-quality oracle in the ecosystem, measuring
perfectly, in the wrong world.

**Walk 3 — netns, compressed.** `ip netns exec blue sysctl -w net.ipv4.ip_forward=1`;
the sysctl oracle compares `sysctl -n net.ipv4.ip_forward` output — read in the DEFAULT
namespace, where forwarding is already on. `ip_forward` is per-namespace kernel state:
converged outside, diverged in `blue`, wrongly elided. (Keep this specimen: the same
sysctl kind is fs-INVARIANT — a chroot does not change kernel state — yet netns-VARIANT.
One kind, two axes, opposite answers: transport polarity cannot be global or per-kind;
it is per-axis.)

**The impossible bar:** asking authors to assert "my check reads nothing else" is a
negative about a world they cannot see — a 233-tier impossible bar. Both per-read authored
completeness speech-acts and wholesale deferral were ruled non-options (deferral just
punts; the bar can never be met).

**Why this is whole-book-tier, not one-line-tier — the guard-cascade:** a guarded line is
a *may-run wall*. Plan-time decisions must be sound under both guard outcomes, so one
guarded `sudo` line degrades the entire tail to guard/run — product B for the rest of the
book, every apply, forever. (The guard-floor is not "this line pays a check"; it is "the
whole rest of the book loses its shape.") Since sudo wraps most lines in real ops books,
the wrapped line's *elision* is load-bearing for the whole tail. "Sudo is a permanent
wall" was the broken-product hypothesis this descent tested.

**Referential agnosticism bounds every scheme:** the engine knows only claims and probe
outcomes. There is no engine-visible "world-readable state" category; a wrong-context
check can *succeed while measuring the wrong world*, silently (root-only state usually
fails loudly and fail-safes; alice-readable-but-per-asker state answers cleanly and
wrongly).

**The proven structure (human-acked):** the trilemma. {sound-by-default ·
minimal-oracle-stays-minimal · unflagged product A under wrappers} — any claims-based
design picks two. Corners: *corner-marked* (per-verb invariance annotation: sound,
unflagged, but relocates a full-enumeration promise from a <10%-of-authors location to a
>90% location — breaks gradual enhancement); *corner-broad-vouch* (the vouch covers all
contexts: minimal and unflagged, but fails-toward-elide in a cell the author never
considered — the cardinal sin, unless massaged; see §3); *corner-flagged-broad* (broad
reading gated behind the existing survival-class flag: sound and minimal, not unflagged).

**Status: OPEN.** The outstanding typed decision is the user-axis polarity (§3).
Everything else in this note is fencing. **[27C, 2026-07-16: CLOSED — the polarity
decision dissolved rather than being taken; measurement-in-context (the `27C` design)
answers the default cell, and the fallback cells inherit corner-flagged-broad.]**

## §2 — Banked walls (fence-tier; none is a design fork)

Sorted per the human's directive: invisible, provably-correct-in-their-corner fences
first; product-B; then flags; lints/teaching last.

### Sound and complete in their corner (protect product A invisibly)

- **wall-axis-default-polarity (fs-view, netns):** facts and values never cross a changed
  fs-view or netns boundary at v1 — default-wall, licensed by nothing. Pure refusal;
  provably the safe direction. fs-view: the chroot-provisioning strawman (`chroot
  /mnt/target apt-get install -y nginx`; the check reads the HOST's dpkg database) refuted
  any permissive reading — *all* state is view-dependent, the author fair-chance test is
  degenerate, and a permissive default there is a nobody's-fault poison. netns:
  dependence is knowable but not author-actionable (the axis has no syntax; wrong-ns
  answers return cleanly; no remover exists). Human-acked as a *real-world* statement:
  "flagless attention under chroot/netns doesn't exist." The only future re-opening is
  kind-tier `invariant:<axis>` (grammar already present in `277`), exceptional by nature
  (kernel-state kinds, e.g. sysctl, are genuinely fs-invariant).
  *Non-bite:* under the wall, Walk 2's line renders honest —
  `chroot /mnt/target apt-get install -y openssh-server   # runs: fact cannot cross the fs boundary`
  — it runs, the image gets sshd. Nothing crossed, nothing lied.
- **wall-conjunction-composition:** mixed wrappers (`sudo chroot … cmd`) cross several
  axes; transport requires EVERY crossed axis to consume; any walling axis walls.
  *Non-bite:* `sudo chroot /mnt/target apt-get install -y curl` — even with a user-axis
  license in hand, the fs axis walls, so the line stays honest.
- **wall-measurement-reach (cell-unmeasurable):** claims license the *interpretation* of
  measurements, never access. State readable only inside a context yields no verdict from
  outside; no vouch, mark, or flag short of escalated probing changes that. HARD-ACKED,
  ceded, tautological. The per-line cell taxonomy under any wrapper, flagless:
  measurable+licensed → elide · measurable+unlicensed → guard · unmeasurable → guard
  forever · unmodeled → run, wall.
  *Non-bite:* `sudo ufw allow 443/tcp` — the probe, as alice, runs `ufw status` and gets
  `ERROR: You need to be root` → can't-say → the line guards. The missing read cannot be
  conjured by any claim — so it can never be *wrongly* elided either.
- **wall-verdict-locality:** verdict-facts are consumed at their own tool's sites; and a
  wrongly-elided line leaves downstream decisions *consistent* — they conditioned on
  "won't run" and it indeed didn't run, so the harm is confined to the line's own missing
  convergence; nothing propagates. This theorem is what the outcome-centric flag-razor
  (local bite → vouch-tier flagless; non-local → flag) cuts with.
  *Non-bite:*

  ```sh
  sudo pipx install poddle       # Walk 1's wrong elision
  sudo apt-get install -y jq     # own fact, own license — its elision stands, correctly
  ```

  The skipped line ran nothing and disturbed nothing; the neighbor's license is
  untouched. The harm stays inside line 1.
- **wall-statement-located-on-its-subject** (human-prompted audit principle): a claim that
  licenses behaviour must live ON the artifact whose behaviour it licenses. Application:
  kind-located `invariant:` may not license *measurement*-transport — it under-claims
  (store-invariance ≠ answer-invariance) and over-reaches (a kind-owner would be vouching
  foreign measuring bodies they've never seen). Kind declarations keep their
  store-topology jobs.
  *Non-bite (the contrast):* `invariant:user` on `sm.dorc.pkg` keeps doing its store job;
  what it can never answer for is a foreign measuring body backing on that kind —
  `pkgpeek__is_converged() { test -f "$HOME/.pkgpeek/cache/$1" ;}` — because the claim
  isn't ON the thing misbehaving. A license located on the measuring body has no such gap.
- **wall-agnosticism-homes:** all world-knowledge in the context machinery is authored.
  Wrapper-oracles declare their axis-transforms (an unmodeled wrapper is never peeled —
  opaque line, wall, fail-safe); kind-owners declare per-axis store properties; the only
  "noticing" anywhere is visible text tokens plus observed exit codes. The engine owns
  only the *closed axis vocabulary* and the polarity table; axis-minting is
  engine-release-tier. (Human first reaction: engine-internal variance around a small,
  essential, highly-abstracted class is reasonable — direction, not ack.)
  *Non-bite:* `doas pkg_add nginx` with no doas oracle — never recognized as a wrapper at
  all; the whole line is opaque, runs, walls. Unauthored recognition cannot misfire
  because it does not exist.
- **wall-empirical-rc:** permission-denied and other rc≥2 outcomes are can't-say → the
  site runs or guards. Unprivileged probes of privileged state usually fail LOUDLY; the
  tools' own permission errors are a free fail-safe. (Asker-identity can *block* an
  answer; the dangerous cell is only where it *bends* one.)
  *Non-bite (the plumbing idiom):*

  ```sh
  wireguard__is_converged() {
     out=$(wg show wg0 2>&1) || return 2   # as alice: "Operation not permitted" → can't-say
     # ...
  }
  ```

  The tool's own refusal becomes the safe verdict; the site guards.
- **wall-values-same-context:** the value plane keeps the stricter floor — captures do not
  cross contexts flagless regardless of the fact-side outcome. `275` §6 remains refused;
  the attention product is fact-verdicts.
  *Non-bite:*

  ```sh
  CHAN=$(foobar channel)            # world-read, measured as alice
  case "$CHAN" in beta) … ;; esac   # may fold — same-context consumer
  sudo foobar switch "$CHAN"        # no fold licensed inside the sudo region
  ```
- **wall-kind-store-protect-downward:** where a kind DOES declare per-asker stores, the
  engine auto-declines cross-context consumption for facts backed on it — overriding a
  naive oracle's silence. Protection flows *downward* from the rare tier to the many;
  only-removes, never licenses.
  *Non-bite — the one line that defuses Walk 1 (STRAWMAN spelling):*

  ```sh
  pipx__state_stored_only_in() { printf '%s/.local/pipx\n' "$HOME" ;}   # kind-owner, once
  ```

  The engine sees a per-asker store, auto-declines cross-user consumption, and
  `sudo pipx install poddle` guards instead of wrongly eliding — the naive oracle's
  silence overridden from above.

### Product B (the floor)

- **wall-guard-without-escalation:** apply-lane guards run in-context by riding the
  book's OWN wrapper bytes — `( sudo x_check … ) || sudo x …` — under credentials the
  apply plainly has. No probe escalation, no flag, no new invariant breached; sound. This
  is the universal flagless floor for wrapped lines. Its honest price, per the
  guard-cascade above: product B for the whole tail. Marginal-risk note: oracle sh now
  executes as root in the apply lane (a lane already running the book's own root
  mutations).
  *Non-bite (the render):*

  ```sh
  ( sudo -u postgres psql_check -c '…' ) \
     || sudo -u postgres psql -c 'CREATE DATABASE app'
  ```

  The check rides the book's own sudo, in-sequence, at apply; nothing probed, nothing
  escalated, no TOCTOU beyond the line's own position.

### Product C (opt-in fences; none default; none a solution)

- **fence-strict-posture-ratchet:** shipped-permissive / offered-strict /
  ecosystem-ratchets (the TypeScript `strict`/`noImplicitAny` precedent). A
  safety-POLARITY flag: everyone-eventually-typing-it is the desired ratchet, not
  cargo-cult rot — that failure mode belongs to risk-*accepting* flags. The human's
  preferred step of the whole dialogue: "a wall that *grows* against the bad-ness."
  Direction-liked; not design-acked.
  *Shape:* `dorc plan --strict-contexts book.sh web1` (STRAWMAN name) — every wrapped
  site without a context-license renders as a guard; greenfield teams type it from day
  one, and the ecosystem ratchets.
- **fence-escalating-probes:** **[27C, 2026-07-16: graduated — this is now the PRIMARY
  lane, designed at `27C` §§2–4 as reuse-never-acquire under the ternary dial; the
  taxonomy below seeded it and stands]** the someday full-strength flag — probes genuinely entering
  contexts. Banked taxonomy for the later dedicated pass (human-directed): the three
  wrappers pose three different probe-inside problems — fs-view = an *addressing* problem
  (same substrate, different names; sometimes zero-privilege from outside; the `24S`
  ladder), netns = an *isolation* problem (entry needed; privilege instrumental), sudo =
  an *authority* problem (entering IS acquiring; the context is the privilege) — which is
  why sudo alone is security-tier and the sudo=chroot=netns gloss melts exactly there.
  *Shape:* `dorc plan --escalating-probes …` — §2's ufw read becomes possible at all;
  authority-tier consent, never a default.
- **fence-deferred-verification (the "DLC"):** the hoisted apply-start in-context
  re-verification wave — a milder cousin of escalating-probes ("accept a plan that may
  immediately confess"). REJECTED as a flagless-tier solution (human: probing-at-apply is
  an oxymoron — that's applying; the attention product is ONE plan shown ONCE, and
  post-ack attention spend is attention-DLC; it also introduces TOCTOU that in-place
  guards don't have). Ack-but-set-aside: the machinery is OWED if such an opt-in ever
  ships, with the confession firing *immediately* after plan-ack. Recorded attacks:
  consent-wall (it unparks `23O` §4's own fence), replan-livelock (a stateless replan
  reproduces the vetoed plan; breaking the loop needs human action or kSTATE-fenced
  memory), permanent check-tax, and wrapper-reconstruction fidelity (rides task-14 /
  `273` §6).
  *Shape:* ack the plan → within seconds, before any mutation:
  `assumption broken: line 3 ('poddle' answers differently as root); nothing applied;
  replan` — the confession must fire immediately after ack, or the tier is a lie.
- **fence-survival-flag-scope:** `kSURVIVAL-trusted` keeps its exact current meaning
  (traveled at-most claims under running walls). Do not overload it with vouch-scope
  trust — mixing trust-species in one flag starts ambient-flag rot from the other end.

### Product D (lints, hints, teaching — never design-carrying)

- **lint-who-am-I-taint:** the `272` r2 token set over check bodies (visible `$HOME`,
  `$USER`, `id` captures ⇒ the fact is context-keyed ⇒ same-context only). Deterministic,
  analysis-time, only-removes. Human grading: borderline inconsequential — build it, it
  solves nothing. Fail-direction warning: under any permissive default it fails OPEN (a
  missed token spelling widens the knife, not the wall).
  *Non-bite:* `test -f "$HOME/.foobar/synced"` — the doorway is in visible text; the
  fact auto-keys to its context and wrapped sites guard, no trust consumed.
- **lint-differential-two-user-CI:** stdlib quality-bar MUST — run owned oracles' checks
  as two users and compare answers. The mechanical first-line defense on what would be the
  most-stamped trust path; empties the naive cell exactly where adoption concentrates.
  *Non-bite:* the harness runs `pipx__is_converged install poddle` as two different
  users and compares — Walk 1's oracle is flagged before publication ever happens.
- **teach-honest-read-idiom:** a per-asker tool's check should read the state where it
  lives (`test -f "$HOME/.foobar/synced" || return 2`) — the visible `$HOME` keys it
  automatically. The decline-spelling is ordinary defensive sh, not vocabulary:
  metadata-spelled-in-sh doing precisely its founding job.
- **teach-at-consumption:** the why-lens carries the context note on every cross-context
  consumption ("checked as alice; answering for root — X's vouch"), so the education
  lands on the person consuming the trust at the consuming moment. It also owes the
  debuggability answer "which context change blocked this transport" (the
  implicit-parameters lesson).
- **teach-smell-sudo-per-user-tool:** wrapping a per-asker tool in `sudo` is frequently a
  book bug in its own right (about to write root's copy); the hint has value independent
  of transport.
  *Shape:* `hint: 'sudo pipx …' wraps a per-user tool — this manages root's copy; intended?`
- **docs-gradual-typing-frame:** oracle-as-declaration-file; "sound modulo the
  declarations you feed it." Human: fence-sitting — usable in docs, buys little for the
  authoring-time education that actually matters.

## §3 — The open fork: the user-axis polarity (NOT decided)

The one axis where a permissive default is even *arguable*, and the decision the whole
descent narrowed to.

**The candidate (proposal-tier): the site-scoped vouch.** One ruling: *a
verdict-function's yes answers the question as the book asked it — the site as written,
wrapper included; the probe executing from the SSH user's context is an implementation
reality, not a scope-limiter.* Precedent framing: `24M`'s rungs-default (an unmarked
verdict-function carries the full license, permanently; opt-downs arrive later) meeting a
new axis. ABI unchanged — the oracle never sees the wrapper (consistent with the `24S`
§2c no-wrapper-awareness posture); the author's obligation is stated purely in tool
terms: **"your check's answer describes the tool's state, not you, the asker."** The
author speaks about the tool; the engine routes the answer.

**Massage-conditions** (the human's bar for shipping any broad-vouch: the exact shape of
the already-accepted risk, zero new admin documentation-weight): same victim (own tool's
line — wall-verdict-locality) ✓; same admin risk-sentence (USER_STORY's "a vouch that is
wrong endangers its own author's own tool's line", verbatim) ✓; same repair path
(`dorc why` → the named yes → one honest-read line) ✓; precedent-shape (rungs-default) ✓;
**fair-chance — THE thin condition:** "does my tool keep per-asker state?" argued as
first-fact knowledge about one's own tool (not a completeness negative), carried by one
contract sentence of the same species as converged≠no-op. This is where the design gives,
if it gives.

**The knife cell, named exactly:** {tool-internal per-asker state} ∧ {sh-clean check} ∧
{context-unaware author} ∧ {undeclared kind} → wrong elision of that tool's own wrapped
line. Who bleeds: overwhelmingly the babby case (author = admin; self-inflicted,
attributed, one-line repair); second-hand admins of published naive oracles (inside the
already-acked risk class); the stdlib gets machine-emptied by the differential CI.

**Why the user axis is even permissive-eligible — the criterion this descent mined:**
three axis-facts (situation-independent state is the *norm*; the exceptional dependence
is author-*knowable*; its honest spelling is *visible sh*), compressed to
**criterion-syntactic-visibility**: *an axis may default to transport only where reads of
it are conventionally syntactically visible in idiomatic sh.* User-identity has names in
the language (`$HOME`, `$USER`, `id`); the fs root and the netns have no spelling at all
— they are resolved silently by the kernel beneath every path and socket operation. Sudo
stands apart twice, and the reasons rhyme: the only authority-tier probe-inside, and the
only syntactically-utterable axis.

**Candidate general ruling — default-the-norm, declare-the-exception:** per-axis
consumption polarity, engine-owned, set once per axis at axis-build time. Declarations
always state the *exception* for their axis (per-asker stores on user; `invariant:<axis>`
on fs/netns), which keeps annotation rare by construction on every axis. The axis sweep
therefore does NOT force a >90% annotation — it forces the polarity table plus the
admission that hard axes lose flagless value (wall-axis-default-polarity, acked).

**The fork's options, all live:** (i) permissive user-axis default (site-scoped vouch) —
buys the fenced breach: silence-buys-nothing-risky is violated in the one knife cell,
knowingly; (ii) corner-flagged-broad — transport behind the existing survival-class flag;
flagless wrapped lines are product B; (iii) strict user-axis default — walls unless
declared; kills babby-under-sudo value. fence-strict-posture-ratchet composes with (i) as
the growing wall and is the human's best-liked element. **No option is acked. The human
has typed neither ack nor nack on (i); "cardinal sin" stands against any unmassaged
broad-vouch; the ratchet is liked as direction.**

## §4 — The PLT frame (mined vocabulary; useful, not load-bearing)

Diagnosis: this was never a datatype problem — it is an *effects* problem. sh has maximal
ambient authority: asker-identity, fs root, netns are dynamically-scoped implicit
parameters no call syntax mentions; every tool invocation is an untyped FFI call; a
wrapper is a scoped rebinding of the ambient row (`HOME=/root cmd` is the native
miniature; `sudo`/`chroot`/`nsenter` rebind rows the language cannot spell). Transport is
then *memoization validity across an environment rebinding* — reuse requires hermeticity
in the rebound parts, which is the `kVOLATILES` weld's own logic; the permissive cell is
accepting a memo hit on a signature rather than a proof. Mappings: axes = effect labels ·
`invariant:<axis>` = effect-absence declaration · the taint = effect inference, possible
exactly where the effect has syntax · the vouch = a trusted foreign signature.
Warnings mined: the colored-functions/effect-row plague ⇒ the axis vocabulary must NEVER
surface at the 90% authoring tier (authors write context-blind checks; the axes are
engine-and-kind-owner words only); implicit-resolution debuggability ⇒ the why-lens
obligation in teach-at-consumption; memo-keying ⇒ context-keyed fact identity (which
same-context-only consumption already is).

## §5 — Refuted this dialogue — do not revive

- The **universal** site-scoped reading (yes-covers-every-axis): killed by fs-view
  (poison default; degenerate fair-chance; nobody's-fault cell).
- **Kind-located invariance as a flagless measurement-license**: kinds are shared by
  design ⇒ non-local blast ⇒ flag-tier by the razor; also mis-located
  (wall-statement-located-on-its-subject). **[CORRECTED 2026-07-16, `plans/27C` §4:
  the CONCLUSION stands (flagless composed transport stays refused) but both reasons
  were wrong — the razor (`271:rul-flag-is-razor-residue`) is sayability-centric, not
  locality-centric, and the kind-owner's invariance line is a sayable, attributable
  claim (`271:rul-invariance-speech-act`, typed the day before this note, which this
  entry re-litigated unflagged). What forces the flag is the OTHER joint riding the
  composition: the measuring body's unsayable read-completeness (`279f` §3) —
  faultless anatomy ⇒ `--risk-faultless-skips`.]**
- **Recipe-closure as a solution** (the conversational summary's transport-closure core):
  closure scopes which values are candidates; it never touches tool-internal reads.
  Surface-shrinking, hygiene-tier.
- The **"unprivileged-readable state" knowledge category**: breaks referential
  agnosticism — only claims and probe outcomes exist; succeed-while-measuring-wrong is
  silent.
- The **apply-start wave as a flagless-tier solution**: attention-DLC ruling (§2
  fence-deferred-verification records the salvage).
- **Per-read authored completeness** and **wholesale deferral**: the 233-bar non-options.

## §6 — Ack-ledger (only what the human TYPED counts)

- HARD-ACK + ceded: cell-unmeasurable / wall-measurement-reach.
- FIRM-ISH ACK: unmassaged broad-vouch = the cardinal sin; ships only massaged to
  zero-doc-weight, single-universal-layman-risk shape, acked once for both.
- ACK: the trilemma ("I ack your triangle").
- ACK (as a real-world statement): flagless attention under chroot/netns doesn't exist;
  needs a later probe-inside pass (fs≠netns≠sudo taxonomy).
- ACK-BUT-SET-ASIDE: the deferred-verification machinery, owed if a DLC-tier opt-in ever
  ships.
- GRADED: sh-visible taint = borderline inconsequential; build-but-moving-on.
- LIKED (direction, not ack): fence-strict-posture-ratchet ("best step taken all round");
  the engine-internal abstract axis class ("reasonable first reaction").
- NITS ABSORBED: the guard-floor guards the entire rest of the book (may-run wall);
  A/B/C/D product framing (the human's); full-word slug discipline.
- NOT ACKED (proposal-tier only): the site-scoped vouch; the per-axis polarity table;
  criterion-syntactic-visibility; default-the-norm-declare-the-exception; everything in
  §3.

## §7 — Consumers and re-entry pointers

- **block-context implementation-planning** owns resolving
  `279f:ask-transport-disposition`; it must consume §3 (the fork) and §2's fences; the
  wrapper-oracle briefs carry the axis-transform declarations (wall-agnosticism-homes).
- **`279f:ask-flag-boundary-recut`** should consume §3's razor applications
  (wall-verdict-locality; hole-shared-kinds; fence-survival-flag-scope).
- **The probe-inside pass** (human-directed, later date): fence-escalating-probes'
  address/isolation/authority taxonomy is its seed.
- **stdlib quality-bar**: lint-differential-two-user-CI as a must; teach-honest-read-idiom
  in the babby template.
- **KNOBS**: nothing minted here; if the §3 fork survives contact, the user-axis polarity
  is a knob-candidate to *report*, not mint (KNOBS is human-authoritative on naming).
