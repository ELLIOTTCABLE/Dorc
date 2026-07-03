# 232 — the completeness-vouch + the oracle observable-declaration surface (r23 design dialogue)

Synthesis of the post-sweep design dialogue with the human (r23, 2026-06-15). It firms the §4 "channel-vouch / trusted-default" thread the `231` sweep opened, and **corrects `231` §4** in three places (flagged inline below). AI-authored, process-evidence, never a correctness claim. Confidence-marked (+SURE/~SUSPECT/-GUESS/--WONDER). Most of the load-bearing turns here were **human-driven** (the human caught the `oracle_effect` discrepancy, supplied the `#cache` memory, reframed proof→contract, the orthogonality, and the native-vs-invented split); those are tagged `(human)`. The conductor verified the cited source lines directly where noted.

## 0. Headline

- **`oracle_effect` is an OPEN spelling strawman, not a settled mechanism and not a vetoed-and-dropped one** — corrects `231` §4's "settled vouch mechanism" framing. The inline `: Kind#effect_class` form the human remembered is *unbuilt*; the marker form was an *agent's* choice in `193` flagged "oracle's to choose / a strain if awkward," with **no human veto found**. So the effect-spelling is the same open `dq-kOOB`/`kTYANNOT` question as the vouch-surface.
- **The completeness-vouch splits by channel-nativeness** — corrects `231` §4's "uniformly `dq-kOOB`-blocked." sh-native observables (rc, stdout, stderr, fds, files) get **off-ramp-clean real-sh-idiom** contracts (unblocked); only Dorc-modeled effect-cells (`#installed`, `#fresh`) need the **invented** `!`/annotation spelling (the `dq-kOOB`-blocked half).
- **The core §4 differentiation has a clean shape:** absence = lazy = ⊤ = run (welded); "considered-empty" needs a *positive* mark; the engine *proves* the dead sub-case for free; the author's mark is a *contract* (best-effort) for only the sub-cases the engine can't see.

## 1. The `oracle_effect` spelling — open strawman, not drift-from-a-veto (corrects `231` §4)

The human flagged: "I thought I vetoed `oracle_effect` ages ago; effects are supposed to be inline `local var : dns.reverse.Kind#effect_class = …`." Ground truth, verified:

- `oracle_effect <provider> <verb> <polarity> <selector>` is the **live** effect-declaration grammar (`spike/crates/oracle/src/lib.rs:481-543`), with the canonical example at `lib.rs:12`: `oracle_effect apt-get update establish fresh # nullary: package-index#fresh (Singleton)`. It is **not recent drift** — present since round-16, across all three spikes *and* the H2SaLS corpus oracles (97 files).
- The inline `: Kind#effect_class` form is **not built**: `effect_class` appears nowhere in the worktree; the inline annotation is **identity-only** `name : reverse.dns.Kind = value` (`oracle/src/predict/ast.rs:88-111`, `204:43`), carrying no channel/selector.
- The spelling was decided by an **agent** in `notes/193` (keystone-rekey), which chose the 4-token marker *over* the inline annotation and flagged it reviewable-not-ruled (`193:104`, `:191-197`, verbatim): *"the exact sh spelling is oracle's to choose … a 4th token … is the obvious strawman; if it reads awkward, that friction is a `notes/193` strain, not a blocker"* and *"the 4-token form is less off-ramp-hostile than the inline-`local w : T` annotation … because it lives in the oracle file, not the book, and degrades to a no-op shim."*
- **No human veto found** in `spike/CLAUDE.md` standing rulings or the rulings notes (162/198/204/205). ~SUSPECT it isn't recorded; round-16/17 not exhaustively searched, and a conversation-history veto wouldn't be in the corpus.

Net: an **open spelling choice** (the same pole-pair as `dq-kOOB`/`kTYANNOT`/`tc-vouch-surface`), the marker is the as-built strawman, the human's inline `Kind#selector` is the alternative pole. Resolves `23-rev4` to *unresolved-strain*, not drift. **Conductor mis-step corrected:** `231` §4 presented the marker/`query`-polarity vouch as settled.

## 2. The shared apt object / the cell-coordinate (the human's `#cache`/`#package`) — confirmed real

- `apt-get update` → **`package-index#fresh`** (`pkgindex#fresh` in fixtures/probe). The apt index/cache *is* the kind. The human's `#cache` ≈ `package-index#fresh`; `#package` ≈ `package:nginx#installed`. Memory confirmed; only the `update` selector is `#fresh`, not `#cache`.
- The `#selector` cell-coordinate is the keystone, spelled at `193:311-314`: **Operand ⇒ `kind:entity#selector`** (`package:nginx#installed`); **Singleton ⇒ `kind#selector`** (`package-index#fresh`, no bare `:`); the selector is *always* rendered. (Also `191:108-110`, `193:37`, `198:150`, `195:58`.)
- **Nuance on "collaborate" (human, ~SUSPECT):** the keystone deliberately models `update` and `install` as **distinct, independent cells** *to stop `update` poisoning `install`* (the poison-wall fix, `193:37-39`). Cross-command *require*-edges (`install` requires a fresh cache) are **unbuilt** — the effect-map is gen/kill polarity only; `an-require` is `st=S`. The human **dropped** the inter-datum-dependency concern: -GUESS those degenerate into per-command dependency edges anyway, so a first-class shared-collaboration object is not needed. Resolves `23-rev6`.

## 3. The native-vs-invented taxonomy (the structural key) — corrects `231` §4's uniform-blockage

(human, the sharpest turn.) Observables fall in two buckets, and the vouch-spelling differs by bucket:

- **sh-native** (rc, stdout, stderr, fds, files): sh *already has syntax*, so the contract reuses a **real idiom** — e.g. the body-redirect `} >/dev/null` = "ignore stdout"; redirects, `$?`, etc. **Off-ramp-clean, no invention, not `dq-kOOB`-gated.**
- **Dorc-modeled effect-cells** (`apt.Package#installed`, `package-index#fresh`): sh has **nothing native** for "establishes package-index-is-fresh," so the contract **must be invented** — the `!`-pun / `: Kind#selector` annotation. **Off-ramp-hostile — this is the `dq-kOOB`/`kTYANNOT` home.**

So the `#!`-spelling is the *same mechanism* as `} >/dev/null` (a contracted "considered-this-channel" mark); it only needs inventing because effect-cells are the one channel-family with no native shell expression. **Correction to `231` §4:** `dc-elide-on-trusted-default` **splits by channel-nativeness** — the stdout/stderr completeness vouch is the *easy, off-ramp-clean, unblocked* half; only **modeled-effect-completeness** is the `dq-kOOB`-blocked half.

## 4. Differentiating lazy(1) / considered-empty(2) / has-content(3) — the §4 core

The whole game is telling **case 1 (lazy: never looked)** from **case 2 (considered, genuinely empty)** apart, since both look like "nothing written" in the source. Case 3 (there is content) is trivial.

- **Welded fixed point (+SURE, `inv-kfail`):** absence MUST mean case 1 (⊤ → run). If "blank = vouched-empty," a lazy author gets unsafe elisions for free. So absence is permanently reserved for "I didn't look."
- **Therefore case 2 cannot be silent — it needs a *positive* mark** (the act of considering must leave a fingerprint in the sh). Marks:
  - **effect-cells:** the `!`-pun (`17N §4` "present-key = true, `!` for false, **absent ≠ asserted-false**" — the carry-vs-compare split C6, in `oracle/CLAUDE.md`). `: Kind#cell!` = "I considered this cell; assert no-op."
  - **native output channels:** a real sh discard idiom (the body-redirect `} >/dev/null`, §7) — present-redirect = considered-empty, absence = lazy.
- **The engine PROVES the cheap sub-case for free** (+SURE): a channel *nothing downstream consumes* is provably dead (`an-observable-liveness` / `consumption_ok`, `plan/lib.rs:570-592`) — no author mark needed. So the author's mark is reserved for the sub-cases only they can know: **"it emits and is read, but it's reproducible/irrelevant" (benign, `an-benign-mutation`)** and **"silent on this path."**
- **Two traps (-GUESS, load-bearing):**
  1. **No blanket-vouch.** A single `#*!` "everything's empty" lets lazy authors cargo-cult it → back to trusting laziness. The mark must be **per-cell/per-channel**, so writing it costs ≈ considering it ("you get what you put in," enforced at the spelling level).
  2. **The mark is a CLAIM, not a proof.** Case 2 sits on the **claimed** side of the certainty axis (`an-claimed-vs-proven`) — the same place mutation-safety sits. A wrong mark ⇒ wrong-elision ⇒ under-execute (`kFAIL-perform` break), bounded to that one leaf (`16P` T12). Unverifiable at runtime; the place to catch a bad mark is **authoring-time tooling** (a container fixture that runs the real command and checks the channel really is empty), never the runtime engine.

## 5. Contract, not proof — the oracle side dictates (corrects `231`/my "claimed-in-disguise" critique)

(human.) The oracle side is **by-contract / by-dictate** (`IMPLEMENTATION` "by-contract and by-dictate"): the author may be *told* "spell it THIS way if you want X." The book side is best-effort *inference*. So the `} >/dev/null` / `!`-mark is a **contract spelling**, read as the author's declaration, trusted best-effort — *not* a proof we must defend. If the author does fd-fuckery and escapes it, that's them breaking their own contract (a bounded-blast-radius self-own). My earlier "claimed-in-disguise, dangerous" lens was the *book*-side lens (where look-proven-but-isn't is a trap); on the oracle side an openly-contracted spelling is exactly right.

## 6. Orthogonality — oracle-spelling ⊥ book-spelling (corrects my "can't reach the book's line")

(human.) The oracle author declares the **modeled command's** observables **centrally** (once for `apt-get install` — its rc, fds, stdout/stderr, effect-cells), applied *wherever* any book calls it, so the in-a-hurry admin **never writes the guard**. My framing of the book's `x=$(apt-get install)` as a problem ("the author can't redirect that line") was backwards: they're *not supposed to* — central declaration is the feature, not a gap. The claim is the oracle's declaration (best-effort, distrusted like everything on that side). And the **effect-channels (named `#cells`) ARE the cross-oracle "amongst-themselves" coordination vocabulary** (`an-cross-oracle-coherence`; apt-get's and dpkg's oracles agree on `apt.Package#installed`, the named kind being the only anchor).

## 7. The body-redirect sh fact (demonstrated) + the Ramalingam-bounded fd stretch-goal

**The clean "ignore stdout for this body" idiom — adopt now (+SURE, demonstrated):** `f() { …; } >/dev/null` attaches the redirect to the function *body*; it is applied on **every call**, **scoped** to each call (no leak), and **re-expanded at call time** (POSIX grammar `function_body : compound_command redirect_list` — **not a bashism**; demonstrated in msys `sh`, human confirmed on `dash`). Contrast: `exec >/dev/null` *inside* a body **leaks** (sh functions don't open a subshell, so `exec` repoints the caller's fd 1 permanently). So the body-redirect is the **simplest completely-unambiguous, off-ramp-clean** declaration: runs identically standalone, scoped, statically obvious.

**The fd-state stretch-goal (human; DEFER):** a bounded forward fd-state analysis — model only **static** fd redirects, conclude "stdout is aliased to `/dev/null` at end-of-body" within the decidable fragment, drop to ⊤ on anything dynamic (`>&$x`, `eval`, computed fd). This is the **same ⊤-on-unknown discipline as `eval → ⊤`** (`inv-top-reject`), bailing safe (can't-prove-`/dev/null` ⇒ assume content ⇒ consumer blocks ⇒ run). **What it buys / what it does NOT:** it makes the ignore-stdout *declaration* robust under a complex body (mechanically extract net stdout-routing); it does **not** prove the real mutator is silent — we never run it, so the mutator's actual stdout stays the author's best-effort *claim*. So it is "make the contract robust," not "claim → proof." Ties to `an-fd-state` (`st=D`, "deliberately unresolved beyond the structural floor", `16P` §3.2). The undecidability behind *why* you can only do the bounded version: precise fd-aliasing = precise footprint = **undecidable** (Ramalingam 1994; the `W2` wall, `093`/`099`), so you model the decidable fragment and ⊤ the rest — the same reason Dorc over-approximates everywhere and prefers oracle contracts over analysis on the hard cases.

## 8. What this changes for r23 / refined open flags

- **`231` §4 corrected:** `oracle_effect` = open strawman (not settled, not drift); `dc-elide-on-trusted-default` splits by channel-nativeness; orthogonality (central oracle declaration, not a cross-actor gap); contract-not-proof.
- **`tc-vouch-surface` refined → SPLIT:** native channels reuse real sh idioms (decide *which* idiom per channel — body-redirect for stdout, etc.); modeled-effect cells use the invented `!`/annotation (the `dq-kOOB`/`kTYANNOT` decision). The effect-spelling (`oracle_effect` marker vs inline `Kind#selector`) is the *same* open choice.
- **Next design question (where the dialogue paused):** the full **per-channel observable-declaration surface** — for each observable (rc, fds, stdout, stderr, effect-cells), which has a native sh idiom Dorc can simply adopt as the contract spelling, vs which needs invention. The native ones are the cheap, off-ramp-clean wins; the invented one (effect-cells) is the `dq-kOOB` lock.
- **`an-require` (cross-command/cross-datum dependency edges):** unbuilt, and the human de-prioritized it (degenerates to per-command edges). Logged, not in scope.
