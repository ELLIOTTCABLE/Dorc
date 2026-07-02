Okay, a significant design-hole has appeared around oracles, best-effort-degradation, effect-prediction, and trust.

To be clear, right upfront: the design is broken, right now. It *does not work* as documented in the corpus.

This example (in the old style, implemented in `spike3`, not the one used later in this document) is unsound:

```sh
# Oracle declares ONLY the package effect; says nothing about fs.Path.
apt-get__check() { pkg : apt.Package = "$1"; dpkg-query -W "$pkg"; }
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
apt-get.check() {
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

systemctl.check() {
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

cp.check() {
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

story-1. imagine adding a `scan_cve` tool that whose oracle imposes an `apt.Package:.cve_clean` *and* that `apt-get install nginx` inherently clears that property (a version-bump, if it happens, means `scan_cve` needs to re-run.) Under the above, the other oracle simply *saying it depends on* `apt.Package:.cve_clean` means `apt-get.check()` is no longer complete w.r.t. `apt.Package`, thus `apt-get` poisons `apt.Package:.cve_clean`, and thus `scan_cve` can't ever safely be elided (correctly, in this case.)
story-2. separately, imagine adding an opaque `hork` tool, with an `hork nginx` command that's fully unmodeled. After an opaque `hork nginx` in the runbook, *all* commands with *any* state-dependency declared are poisoned; as we have no idea what `hork` does (i.e. it could be a little-known third-party package-mutation tool with no oracle; we can't ever elide an `apt` command after it ... and that logic extends to *every* single piece of potential state in the universe.)

Clearly, every single oracle having to explicitly *list* every single thing that it *does not touch* is something of a non-starter, that would balloon to such size, so quickly.

(This also, unfortunately, bites *worse* with *smaller and more inconsequential things*. Having to enumerate an *ocean* of state that you're promising you don't touch, when you're just adding a meaningless 'this does nothing' oracle for a logging/printing tool you use, is something of a nightmare scenario.)

### 2. Less correct, still frustratingly enumerated

One mitigation I considered is establising a 'default-vouch' stance:

```sh
apt-get.check() {
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

systemctl.check() {
   # ... see above

   : systemd.Service~   # "this is complete"
   : apt.Package~       # "considered this too, checked everything"
   : fs.Path~

   # ... and so on
}

cp.check() {
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

This reduces per-property enumeration, at the cost of `story-1` soundness ... and still requires a painful amount of enumeration ... and, worse, it's *still* not fully safe; due to `hard-1`: the oracle-authors will make mistakes, no matter how much work they do; they will fail to consider some aspect a later author needs to declare dependency on. (i.e. the `story-1` author writing `apt-get.check()` couldn't possibly know that somebody was going to write a cve-checking tool)

And this still does nothing to help with `story-2`.

### 3. Dangerous, but easy

Contrast with the approach of duplicating the above, but removing the 'blanket vouch' lines (`: fs.Path~`) as well. Instead, let's say we made that effectively "the default": any unknown command is assumed not to touch any state it doesn't mention at all.

(This is close to how the spike implemented it in code, somewhat by accident; although it's not nearly this thought-through and principaled. To be clear, *all* four cases assume that completely-unmodeled commands push all value-tracking to Top, there's just nothing you can assume about those, not even that a human *considered* them from the perspective of Dorc.)

```sh
hork.check() { hork --dry-run $@ ;}

apt-get install -y nginx
hork nginx
systemctl enable --now nginx
```

Now, we *want* this 'floor oracle', minimally-written, tossed out quickly while the author is working on something, to be somehow useful; so let's say we default to assuming it doesn't affect any type it doesn't mention.

However, in *this* case, let's say it's a third-party, not-often-used package-management tool. By not 'poisoning' the state of `apt.Package:nginx`, we'll now derive incorrect elisions from this limited information, in an attempt to be helpful/friendly.

### 4. Kind-scoped

This approach tries to allow the 'broad default-poisonless' stance for "potentially irrelevant" properties: default to assuming that a given function-body *doesn't* interact with any kinds; then have them opt-in to analysis by using/mentioning any type. However, for any type they *do* interact with, the default is full-poison, forcing them to enumerate just the *properties*, not all the *kinds*:

```sh
apt-get.check() {
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

cp.check() {
   src="$1"; dst="$2"; [ "$3" = "" ] || { printf 'UNK cp arity\n' >>"$DORC_REPORT"; exit 254; }
   file : fs.Path = "$dst"
   [ -f "$dst" ]   : fs.Path:"$dst".exists                                       # ESTABLISH
}
# apt.Package, systemd never named; so auto-clear. the whole verbosity win.

systemctl.check() {
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

scan_cve.check() {
   x : apt.Package = "$1"
   cve-tool --check "$x"   :? apt.Package:"$x".cve_clean                         # OBSERVE
}
# now that this is loaded in (and used in a book), it retroactively enforces
# that all apt.Package-interactors must *declare that they handle it*, or it
# cannot be elided safely

hork.check() { hork --dry-run "$@"; }
# no ESTABLISH/ACK/OBSERVE/POISON ⟹ engages nothing ⟹ poisons nothing
# i.e. the dangerous floor (while an opaque hork with NO .check() would poison
# all, and therefor be safe)
```
