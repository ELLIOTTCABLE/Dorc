Okay, a significant design-hole has appeared around oracles, best-effort-degradation, effect-prediction, and trust.

To be clear, right upfront: the design is broken, right now. It *does not work* as documented in the corpus.

This example (in the old style, implemented in `spike3`, not the one used later in this document) is unsound:

```sh
# Oracle declares ONLY the package effect; says nothing about fs.Path.
apt-get__predict() { pkg : apt.Package = "$1"; dpkg-query -W "$pkg"; }
oracle_effect apt-get install establish installed     # ← the entire effect-map for this verb
```

`command_effect` resolves `apt-get install nginx` → `cells = effect_of(apt-get, install)` = `[establish package:nginx#installed]` → returns `[Establishes(package:nginx#installed)]`. `reach_transfer` gens that one cell. `fs.Path` is never poisoned. A downstream `[ -f /etc/nginx/nginx.conf ]` guard stays ambient and elides — even though `apt-get install` writes that file.

Meanwhile, simply omitting the oracle entirely yields *better* behaviour for the runbook: an opaque `apt-get` poisons all cells, the guard isn't elided.

However, that unsoundness is *load-bearing*: without it, *every* command poisons *every* cell it doesn't know about; and nobody can ever know about *every* piece of global state any other thing could possibly care about, so *nothing can ever be skipped.* This is the core and critical failure to pursue workarounds and repair for. (Likely a losening of previous welds, a redirect of our core-goals, or accepting failure-modes we previously worked hard to prevent - I don't see what else to do.)

## Context

----
A quick refresher:

- we have typed 'global state items' - think, a package in the system's package-manager, a systemd unit, a filesystem path, one of the wombats that `hork` manages. (this is adjacent to, and similar to, but slightly orthogonal to, the *shell-native* observables, like stdout/stderr/rc/fds.)
- oracles declare both mutations and references to, as well as dependancies on, those states, spelled-in-sh (we have a candidate spelling for some of it, but the syntax is not the important part right now)
- states currently have a very basic structure, and may evolve a bit more later (again, shouldn't be the focus right this second unless it somehow sidesteps the issue at hand): a namespace for types `dns.reverse.TypeName`, followed by stringly-keyed 'instances' that represent entities in that type-namespace (`apt.Package:nginx` vs `apt.Package:ruby`, or `fs.Dir:/etc/nginx`), and finally 'properties' on that type that can either be on-instance (`apt.Package:nginx.installed`) or singleton (`apt.Cache.fresh`).
  - right now, we're trying to keep value orthogonal from effect/vouch/dependancy - i.e. retain the ability to say "I am specifying/vouching a specific value <here> in the CFG, and that value is <it is absent>" separately from "I have not looked at this value at all, it is unknown" separately from "this value is specifically falsey."
  - vague vocab: for data, "ESTABLISH" `: T:i.p = val` (or punned, `expr : T:i.p`) vs. "OBSERVE" (i.e. depends-upon, `expr :? T:i.p`); for vouching/trust, "ACK" (an 'I checked, this *doesn't* mutate', `: T:i.p~`) vs. "POISON" (for those designs below where it's *not* the default, an explicit marking of "this may be mutated, but I'm not breaking down how", by no-op mentioning - `: T` or `: T:i` or `: T:i.p`)
- while our toy-examples below only have ~3-4 commands and ~10-ish state-properties; a realistic target 'analysis unit' may include multiple "standard libraries" a user installs, *full* of oracles of various qualities, applied to runbooks with their own, quicker/lower-quality oracles - think, say, O(100) oracle'd commands covering 0(1000) properties, all of varying quality/correctness, all with limited knowledge of eachother.

----

First of all, the hard constraints - feel free to check these, I'm not *claiming* them, they should be genuinely impossible:

hard-1. mutation is, mostly, *fundamentally un-analyzable* from the sh we have access to. We can *only* gain information about this problem by threading together and comparing/contrasting the opaque declarations of oracle-authors; reconciling those, in a best-effort-friendly way (with graceful degradation) is the whole deal.
hard-2. we *cannot* expect perfect oracles; even if we crank "wish-B" below all the way up to 11, and ask/demand that authors spend many hours writing the perfect oracle ... it'll degrade *all on its own* just by aging. (and humans are imperfect, anyway, so that's not even the usual failure-mode; just a provably inescapable ceiling.)

Now, the tensions - nearly constraints, but not *literally* impossible, and I suspect one or several of them is going to have to gently give to make this work (consider all previous design-work to be in question at this point; we've hit a critical juncture; and for this conversation, I'm un-welding all of the KNOBS):

wish-A. *correct:* we do not want to wrongly elide ("under-execute" in IMPLEMENTATION.md / "Priorities" in DESIGN.md);
wish-B. *gradual:* we do not want to demand that oracle-authors write a perfect oracle up-front to gain most of the benefit ("Priorities" in DESIGN.md); *some* benefit (i.e. apply-time elision) should come *early*, when the oracle has very little metadata ("Rationale" in README.md);
wish-C. *community-written:* we do not want to maintain a library of types/properties; they should be dynamically declared, opaquely, by the oracles;
wish-D. *composable:* we do not want to force oracles to decide who is 'authoritative' for a given type; they should (where possible) compose dynamically from multiple references/declarations, as long as they're structurally compatible (not conflicting)
wish-E. *eventually complete:* because they're shared between community members, or at least between one's own runbooks, they should *accrue correctness and functionality gradually*, and generally *encourage* more completeness/precision/safety, even if they best-effort *support* less-completeness/precision/safety.
wish-F. *inert:* we shouldn't introduce mutation into the probe-time script (this is mostly orthogonal, but bears repeating)

The end-goal here is basically "to be useful." This matches the priority-list in DESIGN.md: somewhat-in-order, 1. pushing upwards correctness (which really, in this setting, translates into "minimizing nasty surprises"), while 2. pushing downwards upfront-demanded-user-effort (in this setting, we already *have* a solution that's as-close-to-correct-as-we-can-get ... it just demands so much from the user as to become useless, while still, of course, not reaching perfect safety.)

## Approaches tried

So, with that context: here are several positions in the state-space we've somewhat explored at various points, and turn out to be in conflict, that we're focused on reconciling here: the "fully-correct" (which seems to turn out to mean 'always-enumerate-the-world'), one particular middle-ground (still lots of enumeration, *and* full of footguns), and the "fully-best-effort" (which turns out to mean 'can't-be-correct'.)

### 1. Fully correct, enumerate-the-world

The majority of our research-corpus assumed that incomplete oracles would 'poison' the parts of the global-state-observables that they don't declare any interaction with ("if the `hork` oracle's author didn't express any information to us *about* freeble-state, we are forced to assume they didn't *know* somebody would *care* about freeble-state, and thus did not investigate the upstream `hork` behaviour around freebles.") This is the drive-towards-Top-if-undeclared behaviour, and it is maximally defensive, achieving *correct* behaviour in the presence of half-complete oracles (gradual enhancement); but in a little practical experimentation, it looks like it quickly balloons the *overall* author-effort to an absurd ceiling.

Consider (spelling/syntax is just a strawman for the most part; focus on the value-flow and knowledge/trust/vouching-boundaries):

```sh
apt-get.predict() {
   verb="$1"; shift
   while [ "${1#-}" != "$1" ]; do shift; done
   pkg : apt.Package = "$1"; shift  # identity (no cell)

   # error/warning channel; rc discarded, `UNK` carries the state-information ("abandon hope, drive to Top", this call is unmodeled)
   [ "$1" = "" ] || { printf 'UNK multi-operand apt-get\n' >>"$DORC_REPORT"; exit 254; }

   # we must list *every property* of *every type* that we have *checked*, and verified that we don't mutate
   : apt.Cache.fresh~
   : fs.Path:.exists~         # note that these are *incorrect* (apt-get can modify arbitrary filesystem locations), and lost in the sea of exclusions. footgun firing in practice.
   : fs.Path:.is_directory~
   : fs.Path:.is_empty~
   : fs.Path:.is_executable~
   : fs.Path:.size~
   : systemd.Service:.active~
   : systemd.Service:.enabled~
   : systemd.Service:.present~

   case "$verb" in
   install)
      dpkg-query -W -f='${Status}\n' "$pkg" 2>/dev/null \
         | grep -q '^install ok installed$'   : apt.Package:"$pkg".installed   # rc-0 => installed=true, rc>0  => installed=false
      : apt.Package:.held~  # NO EFFECT: declares 'I considered this, install leaves :.held alone, this effect is untouched'
      ;;
   purge)
      dpkg-query -W -f='${Status}\n' "$pkg" 2>/dev/null \
         | grep -q '^install ok installed$'   : apt.Package:"$pkg".installed!  # flipped: rc-0 => installed=false, rc>0 => installed=true
      ;;
   *) printf 'UNK unmodeled verb: %s\n' "$verb" >>"$DORC_REPORT"; exit 254 ;;
   esac
}

systemctl.predict() {
   verb="$1"; shift
   now=
   while [ "${1#-}" != "$1" ]; do [ "$1" = --now ] && now=1; shift; done
   svc : systemd.Service = "$1"; shift
   [ "$1" = "" ] || { printf 'UNK multi-operand systemctl\n' >>"$DORC_REPORT"; exit 254; }

   : apt.Cache.fresh~
   : apt.Package:.held~
   : apt.Package:.installed~
   : fs.Path:.exists~
   : fs.Path:.is_directory~
   : fs.Path:.is_empty~
   : fs.Path:.is_executable~
   : fs.Path:.size~
   : systemd.Service:.present~

   case "$verb" in
   enable)
      if systemctl is-enabled --quiet "$svc"; then : "$svc" : systemd.Service:"$svc".enabled
      else                                         : "$svc" : systemd.Service:"$svc".enabled = false
      fi
      if [ "$now" ]; then                            # `--now` touches :.active ONLY on this path
         if systemctl is-active --quiet "$svc"; then : "$svc" : systemd.Service:"$svc".active
         else                                        : "$svc" : systemd.Service:"$svc".active = false
         fi
      else
         # nothing checked at probe-time, no claims established about :.active on this path
         : systemd.Service:.active~
      fi
      ;;
   disable)
      # `!` as a sugar/punning for "invert the rc for setting this property"
      # yes, is-disabled doesn't exist; just a strawman
      systemctl is-disabled --quiet "$svc" : systemd.Service:"$svc".enabled!
      : systemd.Service:.active~
      ;;
   *) printf 'UNK unmodeled verb: %s\n' "$verb" >>"$DORC_REPORT"; exit 254 ;;
   esac
}

cp.predict() {
   src="$1"; dst="$2"
   [ "$3" = "" ] || { printf 'UNK cp arity\n' >>"$DORC_REPORT"; exit 254; }
   file : fs.Path = "$dst"
   [ -f "$dst" ] : fs.Path:"$dst".exists # again, follows rc of annotated CFG-node

   # note that the simpler the oracle, the more overwhelming and frustrating this is:
   : apt.Cache.fresh~
   : apt.Package:.held~
   : apt.Package:.installed~
   : fs.Path:.is_directory~
   : fs.Path:.is_empty~
   : fs.Path:.is_executable~
   : fs.Path:.size~
   : systemd.Service:.active~
   : systemd.Service:.enabled~
   : systemd.Service:.present~
}

# But the takeaway is that the following is now *reasonably* complete and sound:
apt-get install -y nginx
cp /etc/nginx/nginx.conf.new /etc/nginx/nginx.conf
systemctl enable --now nginx
```

... and the moment a new type enters the universe, i.e. you install `widget`, *all* elisions instantly stop until *every* oracle *fully* enumerates every property of `widget.Hunk` ... and thus Dorc becomes utterly useless in every single real-world scenario.

However, under this paradigm, the system is protected *both* from 'new state-properties manipulated by other tools' *and* 'new tools manipulating state':

story-1. imagine adding a `scan_cve` tool that whose oracle imposes an `apt.Package:.cve_clean` *and* that `apt-get install nginx` inherently clears that property (a version-bump, if it happens, means `scan_cve` needs to re-run.) Under the above, the other oracle simply *saying it depends on* `apt.Package:.cve_clean` means `apt-get.predict()` is no longer complete w.r.t. `apt.Package`, thus `apt-get` poisons `apt.Package:.cve_clean`, and thus `scan_cve` can't ever safely be elided (correctly, in this case.)
story-2. separately, imagine adding an opaque `hork` tool, with an `hork nginx` command that's fully unmodeled. After an opaque `hork nginx` in the runbook, *all* commands with *any* state-dependency declared are poisoned; as we have no idea what `hork` does (i.e. it could be a little-known third-party package-mutation tool with no oracle; we can't ever elide an `apt` command after it ... and that logic extends to *every* single piece of potential state in the universe.)

Clearly, every single oracle having to explicitly *list* every single thing that it *does not touch* is something of a non-starter, that would balloon to such size, so quickly.

(This also, unfortunately, bites *worse* with *smaller and more inconsequential things*. Having to enumerate an *ocean* of state that you're promising you don't touch, when you're just adding a meaningless 'this does nothing' oracle for a logging/printing tool you use, is something of a nightmare scenario.)

### 2. Less correct, still frustratingly enumerated

One mitigation I considered is establising a 'default-vouch' stance:

```sh
apt-get.predict() {
   # ... see above
   [ "$1" = "" ] || { printf 'UNK multi-operand apt-get\n' >>"$DORC_REPORT"; exit 254; }

   # declaring broad 'NO EFFECT', as the default - effectively saying "I have checked
   # every aspect of this type, and how this command can affect it."
   : apt.Package~
   # NOTE: even under this 'more-defensive' regime, there's still danger
   #     here: this approach *encourages* users to do this, but it's also a
   #     footgun, because it still introduces in-escapable incorrectness around
   #     any *future* changes to the type by other oracles (i.e. this implicitly
   #     declares "I'm not affected, nor do I affect, `apt.Package:.version`;
   #     which is patently untrue if something else depends on it.)

   # aaaaand still listing every single *ptype* we checked-manaully-and-are-declaring-to-be-unaffected, even if we don't have to enumerate their properties:
   : fs.Path~
   : systemd.Service~

   # ... and so on
}

systemctl.predict() {
   # ... see above

   : systemd.Service~   # "this is complete"
   : apt.Package~       # "considered this too, checked everything"
   : fs.Path~

   # ... and so on
}

cp.predict() {
   # ... see above

   : fs.Path~           # complete on fs.Path; ignore any future-added properties
   : apt.Package~       # ← AND: considered apt.Package, cp touches none of it
   : systemd.Service~   # see the tension? every oracle would need dozens of these
}

# Now we've done a ton of extra work, *and* this is unsound:
apt-get install -y nginx
cp /etc/nginx/nginx.conf.new /etc/nginx/nginx.conf
systemctl enable --now nginx
```

This reduces per-property enumeration, at the cost of `story-1` soundness ... and still requires a painful amount of enumeration ... and, worse, it's *still* not fully safe; due to `hard-1`: the oracle-authors will make mistakes, no matter how much work they do; they will fail to consider some aspect a later author needs to declare dependency on. (i.e. the `story-1` author writing `apt-get.predict()` couldn't possibly know that somebody was going to write a cve-checking tool)

And this still does nothing to help with `story-2`.

### 3. Dangerous, but easy

Contrast with the approach of duplicating the above, but removing the 'blanket vouch' lines (`: fs.Path~`) as well. Instead, let's say we made that effectively "the default": any unknown command is assumed not to touch any state it doesn't mention at all.

(This is close to how the spike implemented it in code, somewhat by accident; although it's not nearly this thought-through and principaled. To be clear, *all* four cases assume that completely-unmodeled commands push all value-tracking to Top, there's just nothing you can assume about those, not even that a human *considered* them from the perspective of Dorc.)

```sh
hork.predict() { hork --dry-run $@ ;}

apt-get install -y nginx
hork nginx
systemctl enable --now nginx
```

Now, we *want* this 'floor oracle', minimally-written, tossed out quickly while the author is working on something, to be somehow useful; so let's say we default to assuming it doesn't affect any type it doesn't mention.

However, in *this* case, let's say it's a third-party, not-often-used package-management tool. By not 'poisoning' the state of `apt.Package:nginx`, we'll now derive incorrect elisions from this limited information, in an attempt to be helpful/friendly.

### 4. Kind-scoped

This approach tries to allow the 'broad default-poisonless' stance for "potentially irrelevant" properties: default to assuming that a given function-body *doesn't* interact with any kinds; then have them opt-in to analysis by using/mentioning any type. However, for any type they *do* interact with, the default is full-poison, forcing them to enumerate just the *properties*, not all the *kinds*:

```sh
apt-get.predict() {
   # ... verb/opt parse ...
   pkg : apt.Package = "$1"; shift
   [ "$1" = "" ] || { printf 'UNK multi-operand\n' >>"$DORC_REPORT"; exit 254; }

   case "$verb" in
   install)
      dpkg-query -W -f='${Status}\n' "$pkg" | grep -q '^install ok installed$'  : apt.Package:"$pkg".installed    # ESTABLISH
      : apt.Package:"$pkg".held~                                                                                  # ACK
      : fs.Path                                                                                                   # POISON
      ;;
   purge)
      dpkg-query -W -f='${Status}\n' "$pkg" | grep -q '^install ok installed$'  : apt.Package:"$pkg".installed!  # ESTABLISH
      ;;
   *) printf 'UNK unmodeled verb: %s\n' "$verb" >>"$DORC_REPORT"; exit 254 ;;
   esac
}
# - mentions apt.Package, but doesn't specifically mention apt.Package:"$pkg".version
#   or :"$pkg".cve_clean, so poisons those two;
# - mentions, and handles *none* of, fs.Path, and so `apt-get install` is
#   assumed to broadly posion/invalidate *any* probed fs.Path unless it is elided;
# - does not mention, and so is trusted not to touch: systemd.Service

cp.predict() {
   src="$1"; dst="$2"; [ "$3" = "" ] || { printf 'UNK cp arity\n' >>"$DORC_REPORT"; exit 254; }
   file : fs.Path = "$dst"
   [ -f "$dst" ]   : fs.Path:"$dst".exists                                       # ESTABLISH
}
# apt.Package, systemd never named; so auto-clear. the whole verbosity win.

systemctl.predict() {
   # ... svc : systemd.Service = "$1" ; --now ⟹ now=1 ...
   case "$verb" in
   enable)
      systemctl is-enabled --quiet "$svc"     : systemd.Service:"$svc".enabled   # ESTABLISH
      if [ "$now" ]; then
         systemctl is-active --quiet "$svc"   : systemd.Service:"$svc".active    # ESTABLISH
      else
         : systemd.Service:"$svc".active~                                        # ACK
      fi
      ;;
   esac
}
# no interaction with File or Package; all branches poison any systemd.Service property except :.active and :.enabled

scan_cve.predict() {
   x : apt.Package = "$1"
   cve-tool --check "$x"   :? apt.Package:"$x".cve_clean                         # OBSERVE
}
# now that this is loaded in (and used in a book), it retroactively enforces
# that all apt.Package-interactors must *declare that they handle it*, or it
# cannot be elided safely

hork.predict() { hork --dry-run "$@"; }
# no ESTABLISH/ACK/OBSERVE/POISON ⟹ engages nothing ⟹ poisons nothing
# i.e. the dangerous floor (while an opaque hork with NO .predict() would poison
# all, and therefor be safe)
```

## Update, 2026-07-01

One potential escape-hatch in the above constraints lies outside the described problem-space entirely: trying to enrich the *resultant* vocabulary from a boolean `{elide, run}` to a ternary `{elide, guard, run}`. The poison-wall - the "frame problem" - is something we can try to constrain down to poisoning the *transition from `guard` to `elide`*, but perhaps we can convince ourselves that's less-bad than collapsing all the way to `run`.

This effectively splits 'the product' in two yet again, with fairly different value propositions on both ends:

1. when the world *is* fairly fully-described (i.e. staying within the above-described framework), Dorc functions as I originally intended it: an *attention-saver*, as much as, or moreso, than a performance-optimizer. *Literally* removing entire lines of your runbook from the plan isn't just a perf optimization, it's a sanity-retaining tool in a messy ops world.
2. where the above bites, and the poison-wall sets in, we try to eke out some sound *performance* benefit, and retain *as much* sanity-benefit as we can, by predicting the effects of commands (i.e. "should be a no-op based on our probe, but we can't prove it because of interposed opaques") and presenting them as hints. however, nonetheless, the `dorc plan` dumped into the user's face still retains many (all?) of the commands post-poison-wall, with additional perf-saving inline guards that *don't* depend on the frame-problem/umnodeled-abstract-analysis-of-runtime-state.

A key observation that enables this is an asymmetry of effort/risk: the global-state modeling (effects) aren't just "global" to the machine's state, but are also "global" in a social sense: global to your other runbooks, global to your other commands, global to other users if you ship your oracle work. *Without* them, you're risking *yourself*, but not *licensing incorrect elision of other commands*.

*(from here down: machine-written 2026-07-01, synthesizing the design conversation - for human review/edit)*

To be blunt about what this proposal *is*, before any detail: it is a decision to **give up the attention-product in order to save the performance-product, wherever the world is undescribed.** Nothing below repairs the loss that 233-as-a-whole documents - "full elision from partial description" is dead, and stays dead; past a poison-wall, the runbook does *not* get shorter, and that is structural, not an implementation gap to close later. What the rest of this section does is rescue the *other* values (perf, safety, monotonicity, gradual-enhancement) at *new, additional* cost (the check-tax, a fatter artifact, a whole new contract-surface). That trade - lesser-value-preserved, at-a-price, where description runs out - *is* the entire proposal; read everything below in its light. The only candidate for buying the attention-product back past an opaque is the open fork at the very bottom, and it is genuinely open and may resolve to "no."

Under this paradigm, the user gets the gradual-enhancement curve back, per-oracle, without the landmine: an un-oracled command runs bare (nothing bought, nothing risked; it stands as a poison-wall for the *elide*-tier downstream, exactly as today); an oracle making only *local* claims - a probe predicate and a verb-model for its own tool, knowing nothing about the rest of the universe - immediately buys its own call-sites the `guard` treatment; and only family-participation (every retained command upstream of mine, vouched with respect to my state) buys true `elide`, and with it the attention-value-add. Crucially, *silence stops meaning anything*: it neither vouches (the approach-3/-4 sin) nor collapses everything to `run` (the approach-1 cost); it merely fails to upgrade `guard` to `elide`. The absolute minimal value-add that *cannot* be saved - the forgone upgrade to `elide` - is all that remains load-bearing on an author's omissions.

What a `guard` concretely *is* - and the mindshare-cost question hinges on this - is itself an unsettled *gradient*, not a decided mechanism. In every case it's the oracle's read-only convergence predicate, compiled *in-sequence* into the apply artifact, gating the original command's untouched bytes; the open dimension is *how hard the compiler leans on paring the oracle down* to only the fragment relevant to this one invocation:

1. apply-guard-thin: at the far pole, heavy "compile-mixin" machinery - partial-evaluate the oracle's own argparse against the book's argv (the engine already walks this path for compile-time entity-resolution) and inline only the specialized residue, aspiring to a guard that reads as if a diligent human had written it by hand:

   ```sh
   dpkg-query -W -f='${Status}\n' nginx 2>/dev/null | grep -q '^install ok installed$' \
      || apt-get install -y nginx
   ```

   Treat that block with suspicion - it's dangerously simple-seeming. Even leaning *hard* into analysis, the vast majority of real-world guards are probably a predict-body with genuine control-flow (case-on-verb, option-handling, capability fallbacks), not a single natural-looking command; whether *any* realistic oracle reduces to this rendering is an open question. This is the aspirational ceiling, and it must not drive the design.

2. apply-guard-fat: at the near pole, ship a (perhaps mildly pared-down) `predict()`-body into the artifact as a function, and invoke it: `apt_get_check install -y nginx || apt-get install -y nginx`. No new machinery - this is the probe-compiler's output relocated into the apply lane - but the artifact reads as calls into an opaque blob of checking-code, and the mindshare/render cost concentrates here.

Everything between the poles is the same knob at different settings (how much provably-irrelevant oracle-code gets dropped: unreached verb-branches, other selectors' probes, dead options). Soundness is identical at every setting; only artifact-readability and engineering-cost vary. Where the real-world equilibrium lands is an open, empirical question - so the design has to *work* at the fat pole, and treat thinning as a progressive upgrade, not an assumption.

An in-sequence check is *frame-free* at every point on that gradient: whatever the interposed opaques did has already happened by the time it runs, so there's nothing left to predict - which is why its license needs only local self-knowledge, and composes with zero cross-oracle vocabulary agreement.

(Prior-art: round-21 designed nearly this exact mechanism as "door-4"/guard-insertion - `Research/notes/218a`, motivated then by the errexit/canary question - including the mechanics that matter: `check || cmd` rather than an `if`-form, because an `||`-left is errexit-exempt, so a failing-or-can't-tell guard can never itself crash the book; the preamble-function shipping form; refusal to double-guard a command the admin already guarded by hand. It was flag-gated and ruled build-last / product-hard-defers. This update proposes *re-basing* it as the default middle verdict - a deliberate reversal of that ruling's posture, to be re-welded consciously, not drifted into.)

Upsides:

- We retain the ability to *claim* no-op to the user, just not *prove* it. (That is, the thing we're guarding against with this entire architecture is *hopefully* an edge-case - accidental, unmodeled mutation between *the start of the runbook* and command N.) For most of those commands, in most cases, we can *hope* that the linear section "between" `poison-wall` and `first-meaningfully-mutating-command` will *mostly* be no-ops, re-verified by fast (hopefully) applytime guards; and we can pass that hope onto the user ("expected: 1 change, 96 no-op" in a 100-command runbook 'poisoned' by an unmodeled third command) - although that's objectively of less value than passing *proof* on to the user, in the form of fully eliding something and removing it from their limited attention-window.
- Monotonicity is restored, which was the original sin of §3/§4: a partial oracle helps its own sites and *cannot* endanger anyone else's. The §4 "dangerous floor" oracle (`hork.predict() { hork --dry-run "$@" ;}`) stops being a landmine - there is no silence-minted license left for it to accidentally grant - while still buying hork's own sites their guard.
- The safe default becomes *affordable*, which is what actually dissolves the §0 dilemma: full-poison now degrades a converged site from `elide` to `guard` (~one host-local read, hiding in the shadow of whatever real mutation forced the wall) instead of to `run`-in-full. Enumeration/completeness-vouching demotes from entry-fee-for-any-value to opt-in upgrade - where it's a normal engineering cost instead of an absurd ceiling.
- Both stories degrade gracefully with zero coordination: `scan_cve`'s own guard re-runs at its position after any `apt-get` that actually executed, so cross-oracle vocabulary becomes necessary only to be *fast*, never to be *correct* (story-1); an opaque `hork` mid-book costs exactly "everything downstream keeps its guards" - degradation proportional to the opacity introduced (story-2).
- The artifact keeps the check-then-execute *shape* a diligent author writes by hand, and runs identically without Dorc (the off-ramp holds at both poles of the thin↔fat gradient); how natural the guard itself reads *is* that gradient - thin approaches hand-written, fat is honest plain sh but visibly machine-shipped library-code. Re-analysis of a transformed book recognizes an inserted guard the same way it recognizes a hand-written one, so nothing accretes.

Downsides:

- The attention-product is *not* bought by guards - and past an opaque, it isn't buyable at all without new trust-machinery. The 100-command book with an unmodeled third command yields a ~97-guard artifact *forever*, under arbitrarily-good oracles for the other 97, because an opaque is unvouchable by definition. Display-compression ("1 change, 96 verify-no-op") is claims-not-proof; and the render inherits a heavy obligation: guards must read as one uniform, visually-inert, foldable construct, or the mindshare cost is total.
- The check-tax: converged sites past the wall pay their check-cost on *every* apply. Fine for stat/dpkg-grade checks; real for expensive ones. (The existing probe-vs-just-run banding governs - an expensive check either earns its vouch or just-runs - but the *default experience* of a mid-book change now includes a guard-tail.)
- Guards can't serve every site; these stay bare `run`: consumed-stdout/command-substitution positions (the guard's short-circuit would corrupt the captured value); commands whose rc the admin's own control-flow consumes (`&&`/`||`/`$?`/`if` - the admin's spelled intent wins, always); restart-class "run-delta" verbs, where a *state*-guard is precisely the forbidden wrong-skip, so an oracle must be able to *decline* to offer a predicate; loops and multi-operand invocations, at least initially.
- The guard-license is a real new contract-surface hiding a conflation we must not encourage: "what does this verb establish" (install ⟹ `.installed`) is not "is skipping this site acceptable" (`dpkg -s nginx` passes while `apt-get install nginx` would still *upgrade* an outdated package - and whether that upgrade is noise is a *judgment*, not a fact). Never synthesize a guard from the headline establish alone; the settled license is below. (Adjacent: "converged" vs "noop" probably eventually wants first-class modeling; TODO'd separately, gently deferred.)
- For when the vouch/elide tier gets designed, the blast-radius asymmetry to keep in face: a wrong guard-claim hurts *its own site* (local, attributable - "your oracle lied about your own tool"); a wrong completeness-vouch statically deletes *someone else's* command - cross-site, silent, rot-activated. The vouch tier is permanently the sharp-knife tier.
- Posture/weld changes to make deliberately if this proceeds: the apply lane now executes reads the book never spelled (bounded - read-only, structurally self-vouched, the same bytes the probe lane already ships - but a genuine posture shift, and the reversal of round-21's door-4 deferral); the spike's replacement-values weld (`inv-probe-sourced-values`: a stand-in may only reproduce probe-observed values) needs its already-reserved carve-out exercised, since a guard *reproduces nothing*; and every place "elision" is defined as licensed-by-probe-facts-only needs the third verdict written in.

Correction to §4's parenthetical, from the adversarial crosscheck (`notes/234`, code-verified): "close to how the spike implemented it" is wrong at the *unmodeled* floor - the spike poisons-all for a command with no oracle at all (safe). The trust-silence behaviour exists one level in: a *modeled* command poisons only the cells its oracle declares, leaving un-mentioned cells alone. So the §0 headline unsoundness is real at HEAD, but the baseline is "safe floor, dangerous middle" - not "dangerous floor."

### The guard-license (settled in-conversation, same day)

A guard is a real transform - its pass-direction suppresses a command the admin wrote unconditionally (guard a `restart` behind an is-active check and you've eaten a restart the book demanded) - so minting one requires a license. Settled:

- The license is an *explicit, spelled-in-sh oracle claim*, the **converged-vouch**: "when this establish-set holds, I judge this site skippable; whatever the command would still do is noise I know of, or residue I accept." Explicitly a *fallible judgment* (in exactly the terminology-sense of 'converged' fixed above), never a fact-claim. Treatment follows: claimed-tier trust, disclosed in the plan ("skipped-when-converged, per oracle X's vouch"), blame attributable to the mark. Oracle authors are making an opinionated *default* about what's-worth-caring-about for the command they model; we can't prevent that, and don't want to (we want a wide design-space for oracles that make admins' lives easier in sane ways) - so what we do is *attribute*. The floor-guarantee, honestly stated: a vouch-licensed guard behaves identically to the hand-written `check || cmd` idiom - no stronger - plus attribution.
- Dead, refuted same-day: any universally-quantified license ("when converged, re-running does *nothing*"). Its quantifier ranges over exactly the observables the author never attended to, so it's vacuous as human testimony (the class: `hork` writing bytes into apt's nominally-private binary cache - no sane apt-oracle-author models a cell for a file they don't know exists, and the hork-user broke no contract). Corollary banked as a principle: *tooling never rescues a contract* - if a claim is only correct when a future build-tool maximally guards it, it's incorrect.
- "Per-verb" is sloppy vocabulary - the engine has no notion of a *verb*, only control-flow and constant-propagation. Precisely: the vouch is a mark *on a path* through the oracle's own predict-body; its scope is whichever invocations constant-propagate to reach it; and the guard's predicate is the establish-probes *on that reached path*, verbatim. (One source of truth: plan-prediction and apply-guard run the same code, so plan-vs-apply divergence can only be world-drift, never model-disagreement.) Unpropagatable argv ⟹ no path reached ⟹ no vouch ⟹ run.
- Fence: a converged-vouch licenses guards at *its own command's sites only*, and is inadmissible in any other site's elide/poison reasoning - using it there would launder a local skip-judgment into a global non-interference claim (the "noise" the author dismissed is exactly where someone else's fact may live), rebuilding §0's disaster one storey up. Implementation-shape: a witness-type mintable only from a matching (call-site, reached vouch, probe-verdict) triple; and the vouch never enters the fact-plane at all, so it structurally *cannot* soften poison.
- New hazard-class to carry forward: the guard tier *collapses the fact-indirection*. Probe-code previously affected wrong-elision only through the sanitized fact layer (probe → fact → license); a guard puts probe-code *directly in the execution path*, inside the *book's* shell environment (its `set -e`/`set -u`, its function namespace - the `218a` hazard list applies). Body-trust machinery must be inherited onto the execution path, and the disclosure/blame machinery must treat guard-code as execution-affecting.
- Still open: the concrete spelling (the same open vouch-surface family as every other mark); the admin's per-site "always run this, ignore the vouch" idiom (parked).

What this does *not* solve - the open fork, next to settle: whether any sanctioned spelling exists by which a human vouches global-shaped knowledge about an opaque thing ("this weird tool of mine touches only X and Y" - positive, enumerated, per-cell, never blanket), because that is the *only* path to attention-friendly full elision past an opaque. If that's a hard no (pinkie-promises, rot, and cargo-culting all argue it might be), the honest product statement becomes: *Dorc narrows your attention only where the world is described; elsewhere it makes your book fast and safe, but not shorter.*

<!-- /* LATER-WORK ANNOTATION (2026-07-02, conductor, human-authorized — the sanctioned
frozen-doc pointer; the document above remains stamped/unedited). Continuations of this
document's problem: the 3-agent adversarial crosscheck (notes/236a,236b,236c) and its
adjudication incl. post-adjudication corrections (notes/237); the ceiling exploration —
horizon-bounded claims, derived footprints (notes/238); the crisis-closure package / re-weld
deltas (plans/239). Corrections a future reader MUST honor:
(1) [correction REVERSED 2026-07-02, human ground-truth] apply-guard-fat's "ship a predict()-body"
was RIGHT as written: at design level the predict() IS the oracle — arbitrary sh whose added syntax
is strip-only (annotations removed; `name.predict()` → `name_predict()`; output is runnable sh) —
and the stripped oracle body is exactly what ships, in BOTH lanes. The spike's st-2 check/probe
split is spike-internal implementation, not design truth (build-vs-design divergence, to
reconcile). Any lifted/deconstructed guard form is an optional analyzer edge-case and must be
byte-identical to a substring of the oracle body; engine-synthesized sh never ships.
(2) "hiding in the shadow of whatever real mutation forced the wall" overclaims — false when the
wall-former is cheap; the check-tax bullet is the honest accounting (237 convergence-4).
(3) "family-participation buys true elide" over-promises as written — authored-static
completeness-vouches are unwritable for arbitrary-payload commands, cp included (237
convergence-2; 238 §1 claim-4); the elide-goal survives via derived-at-probe-time footprints
(238 §3) — the goal is NOT deprecated; its licensing basis moved from testimony to derivation.
(4) Read the frontloaded trade with the two-halves doctrine (239 §1): the guard-half is sister
and PERMANENT fallback; full elision remains THE goal, never aspirational-tier.
2026-07-03: mechanical check()→predict() rename applied per ruling (23L addendum); content otherwise untouched. */ -->
