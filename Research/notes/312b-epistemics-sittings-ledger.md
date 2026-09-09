# 312b — Epistemics and assignment: the sittings ledger

> AI-authored (Fable, the `r31-prep-design-duck` sittings, from 2026-09-08). Notes-tier LIVING
> ledger for the phase that follows `notes/311`: sitting-by-sitting minutiae, experimental
> findings, and the takeaways extracted from `notes/312b-exercises/` (one record per exercise,
> not durable). The corpus import is FROZEN as `notes/312a`; the bare `312` is reserved for the
> phase's synthesis. Nothing here is ruled unless it cites a ruling by `docID:slug`; grades are
> +SURE / ~SUSPECT / -GUESS / --WONDER; **[HUMAN]** marks the human's framing, paraphrased from
> chat, never a ruling. Authority: root docs, `spike/CLAUDE.md`, the welds, `notes/311`, and
> `notes/312a` § 0 (the phase's conduct as typed) outrank this.

## § 1-typed-this-phase

Conduct (2026-09-08): plain book-sh grounds a case, never Dorc-spelled sh; the model's
response is prose in 311 vocabulary; seat spellings that fall out carry no weight; one record
per exercise in `312b-exercises/`; `GOTCHAS.md` authored freely now, chopped later into a
frontier and an extras file; the singular Fable-subagent authorization is the conductor's;
prior typed seat assignments (`312a` § 3) are re-verified on the 311 base, never carried as
bindings; short chat responses, nothing implied as ack.

Positions (2026-09-09): known-but-deferred effects stay in scope, gently, never labelled
horizon; unexpected churn (other users, host-owned cron, host-configured reactions) is
horizon, the price of play.

## § 2-exercise-takeaways

From `312b-exercises/01` (placement parametric in an observer: pipx, `sudo git config
--global`, nvm):

- `lead-computed-namespace-instance` — a namespace instance or placement can be a function of
  ρ and of another kind's read (`${PIPX_HOME:-$HOME/.local/pipx}`); `311` § 2.1's three supply
  modes have no room for it. A fourth, COMPUTED, of which SITE and AMBIENT may be special
  cases; unknown inputs make the instance unknown.
- `lead-ask-the-tool-for-its-store` — measure-in-context as the cheap rung: the tool prints
  its own store; the declared rule is the experienced rung.
- `lead-env-is-a-routing-namespace` — ρ routes command words, stores, and referents; `30S`
  covers pins and severs for verdict bodies only.
- `lead-local-route-perishes-on-rho-writes` — the engine's transit-free route claim for
  command words is keyed by `PATH` and cwd; a `PATH` write is a routing mutation.
- `lead-policy-dependent-lends-split-two-ways` — wrapper author declares the default; the
  admin overrides; the oracle may read the policy and decline.

From `312b-exercises/02` (the wrapper seat: the crontab pair, sudoers by guest, wrapper order
and env scrubbing, `$SUDO` as a variable):

- `lead-lends-may-depend-on-the-guest` — sudoers matches the guest command, so the entered
  context can differ between the probe's entry and the apply's (`27Xf`); the wrapper author
  declares guest-insensitive by default and supplies a policy read that declines.
- `lead-policy-reads-decline-on-surprise` — the general move for policy-dependent tools and
  wrappers (sudoers, APT hooks, `env_keep`): read the policy in the denoted context, decline on
  departure from the declared default.
- `lead-context-sets-at-a-site` — a two-valued wrapper (`$SUDO`) gives a site a set of entry
  chains; compare quantifies universally over the set; a fact whose key does not pass through
  User survives either way, one that does is unknown.
- `lead-narrowing-by-test-is-the-capture` — `FORFEITS:forfeit-value-narrowing-by-test` is what
  turns `$SUDO` from ⊤ into a two-element set; the idiom is dominant, so its priority rises.

From `312b-exercises/03` (store identity across hosts: NFS on two hosts, a socket on a shared
mount, the host key):

- `lead-identify-in-the-narrowest-store` — a kind identifies in the narrowest namespace
  instance its state lives in, never a coarser one assumed to partition it; a file-backed kind
  identified in Host is the highest-leverage wrong DISJOINT and a lint-able smell. The law
  routes the NFS question to the filesystem owner and the admin structurally.
- `lead-mount-lines-generate-correspondences` — the transition owner for a mount is the mount
  oracle; the correspondence derives from the argv the admin wrote.
- `lead-cross-host-same-bottoms-out-in-the-admin` — every cross-host SAME chain ends at host
  sameness, the admin's seat by typed ruling; the model needs that seat.
- `lead-reads-decline-outside-their-ontology` — a kind owner's read declines on referents its
  kind does not describe (File on a socket); the mechanical net for lazy coordinate borrowing.
- `lead-warrantless-tokens-are-witness-only` — endpoint witnesses (host keys, stamps) never
  license.

From `312b-exercises/04` (`a-host-is-not-a-partition`: one NFS export under two targets,
with and without a declared root):

- `hole-route-versus-root-reads-unspoken` (+SURE of the text) — `311` § 3.2 makes a
  route-terminated chain against a rooted one unspoken, which a finished definition can spend;
  a chain with fewer measured links compares more decisively than the same chain complete.
  Latent today (no roots declared); live on the first true root.
- `hole-different-roots-read-unspoken` (+SURE of the text) — two roots are two worlds; the
  finished definition is a within-world sentence; spending it across worlds names the wrong
  author in the why chain. Both holes reintroduce `a-host-is-not-a-partition` against
  `26Ob:ack-cross-world-wall-is-the-floor` (TYPED), which 311 otherwise reproduces exactly at
  the no-root floor.
- `hole-sole-route-orientation` (~SUSPECT; ambiguous text) — § 3.2's "every level above it"
  must mean every level between the divergence and the leaf, per § 2.2's definition; under
  either reading the NFS case rests on the File or Filesystem owner withholding sole-route for
  network mounts, with no net for one who did not think of it.
- `law-partial-measurement-never-widens` (owed) — a derivation with an unmeasured or
  route-terminated link yields at most what it yields measured.
- `lead-rootness-is-a-stdlib-only-seat` — a root's blast radius is every kind identifying into
  it, retroactively; declaring one is the highest-leverage act in the identity model and
  belongs with countability's few.
- `lead-cross-target-disjointness-is-the-admins-sole-route` — "these targets share no store"
  is sole-route at the Target scope, the admin's seat; it is the reserved posture option, and
  no rule change replaces it.
- `nit-same-is-or-across-derivations` — the key walk is "and" over levels; SAME overall is
  "or" across derivations (key walk; correspondence; provider identifier); worth stating
  plainly in 311 § 3.2, where only the key-walk clause is written.

Cross-cutting, seen in all four: the admin seat is missing and every exercise hit it
(overrides; sudoers; host pairing; the cross-target posture). Convert-to-read recurs as the
newbie rung. Policy dependence recurs as the shape that splits a claim between an author's
default and an admin's deployment. Reads that decline outside their kind's ontology recur as
the mechanical net for lazy borrowing.

## § 3-time-sitting-banked

One sitting (2026-09-08/09), rewound over by the human; the conclusions, not the argument.

Frame (**[HUMAN]**): Dorc's value is not figuring out time; it is modeling precisely what the
admin already wrote to express time, so Dorc does not diverge from that intent. Every "time"
category examined landed in one of three bins: horizon; someday-punt; or bog-standard static
sh-analysis coverage that is engine work, never ops-world-state-model work. No new soundness
class was found.

- `time-logical-owed` — failure paths (`trap … EXIT`, errexit abort edges); the concurrent
  partial order (`&` to `wait`, pipeline stages); `exec` and detach within the book's extent;
  loops that establish their condition and bound their executions; containment (built); a
  cron or timer the book installs, treated as firing immediately and reaching to end of book.
  Arbitrarily complex static analysis, owed.
- `time-wall-knowable-punted` — a book-installed schedule firing during the apply (schedule
  text plus a remote clock read plus apply duration; unplaceable at a program point, hence the
  conservative reach above); one firing after disconnect (harmless to this apply under
  statelessness, the next probe re-measures; an aid hint at most); predicting async tool
  internals instead of waiting.
- `time-bridge-in-scope-gently` — return-is-not-done: `systemctl restart` before ready, a HUP,
  deferred triggers, DNS under TTL, replication lag. Wall by nature, logical by an authored
  witness: one declaration bit (asynchronous with respect to a cell) makes the footprint reach
  forward to the next wait on that cell or to end of book; the tool author's wait in the
  verdict body or the admin's `until` and `sleep` closes it. No clock.
- `time-teaching-rules` — eventual consistency is `reaches`, never `corresponds` (SAME for
  collision is safe, SAME for transport is not); async tools put the wait in the verdict body;
  the admin's wait loop is first-class and `StatusIterated` already protects it; an
  unexplained `sleep` is a hint that a footprint omits a deferred consequence.
- `claim-guards-inherit-the-bare-book-exposure` (~SUSPECT; unattacked) — a guard and the
  command it fronts share a program point, so a deferred effect lands after both; the guarded
  book and the bare book reach the same state. If it holds, USER_STORY's "no staleness by
  construction" is overstated, not broken: no staleness relative to landed effects, the bare
  book's exposure otherwise (`236b` F1's July downgrade, never carried into the doctrine
  text). TOCTOU exposure is then a function of elision alone, bounded by walls and by plan
  age. Attack this before leaning on it.
- `time-levers-unweighted` — plan age as an aid fact at apply; a verify posture (every elision
  a guard) as the admin's TOCTOU dial, nearly free; apply-standup freshness demotion
  (review-gated, `rul-repeated-probing-reviewed-before-design`); MH2's version window,
  deferred by choice, tractable in shape.
