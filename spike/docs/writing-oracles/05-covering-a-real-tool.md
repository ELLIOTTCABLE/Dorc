# Covering a real tool honestly

A first oracle answers for one verb, in one shape, for your own book. This page is
about growing that into something worth publishing: an oracle that meets other
people's books, other people's habits, and other people's machines, without ever
answering a question it does not actually know the answer to.

The organizing idea: your argument parser is the contract's type-checker. Every
invocation of your tool flows through your `case` statement and your gates before
any answer is possible, and each arm is a decision you make once, deliberately -
answer this shape, or decline it. Breadth is added arm by arm; safety is the `*)
return 2` at the bottom that makes everything you have not yet decided about
exactly as safe as having no oracle at all.

## The enriched shape

Here is the page-four oracle, grown up:

```sh
# dorc-lang/v0.2
foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs|renew)
      dest : org.foob.Certs = "$1"
      [ "${2-}" = "" ] || return 2
      foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest"@synced
      ;;
   purge-certs) return 2 ;;
   *) return 2 ;;
   esac
}
```

Three kinds of decision are visible, and a battle-grade oracle is mostly the
discipline of knowing which one each arm deserves.

The first arm is shared coverage: `renew` turned out to establish the same cell by
the same criterion as `sync-certs`, so one answer honestly covers both. This is the
cheap kind of breadth - when two verbs truly share a convergence meaning, share the
arm. When they almost share one, do not: "almost the same" is two arms.

The `purge-certs` arm is a deliberate, permanent decline, and it is the most
instructive line on the page. The author could easily have written a check -
"do the certs look absent?" - but chose not to, because they know the tool: a purge
also clears stale residue that can be present even when the certs look gone, so
"looks absent" is a bad reason to skip a purge. That knowledge is expressed in six
characters of ordinary control flow. Nobody else could have made that call; no
analyzer could have; the person who knows the tool did, by declining to answer.
Every real tool has at least one verb like this. Finding yours is most of the work
of an honest oracle.

The `*` arm is the future: verbs you have not modeled, verbs that do not exist yet,
a colleague's typo. No claim, no answer, no license; the line runs.

## Flags, and invocations that change the question

Real invocations carry flags, and flags can quietly change what question you are
being asked. A `--root=/somewhere` or `--config=/other/file` flag can point the
whole tool at different state, making your check - which read the default state -
a confident answer to the wrong question. That is the worst kind of wrong, so the
rule is the same as for verbs: handle the flags you have modeled, decline shapes
carrying ones you have not. A common structure walks leading flags explicitly:

```sh
foobar__is_converged() {
   while [ "${1#-}" != "$1" ]; do
      case "$1" in
      -v|--verbose) shift ;;
      *) return 2 ;;
      esac
   done
   ...
}
```

Passthrough flags you know to be inert (verbosity, color) can be skipped over;
anything that could affect what state is being addressed either gets modeled into
the coordinate or declines the whole invocation.

The same logic drives the arity gate you have been writing since page two. If your
tool accepts several operands per call (`foobar sync-certs A B`), an oracle that
only checks `A` and answers 0 has silently vouched for `B` too - and on a host
where `B` has drifted, that line needed to run. Either loop over every operand and
answer only if all agree, or gate to the single-operand shape and decline the
rest. Half-checked is not a smaller yes; it is a wrong yes.

## Translating exit vocabularies

Your answer is read against the fixed table (0 holds, 1 does not, 2 and up cannot
say), but the tools you delegate to have their own dialects. `grep -q` says 0, 1,
or 2-for-error, which happens to line up; `diff` says 2 for trouble; plenty of
tools say 1 for errors and 2 for legitimate states. Where the dialects differ,
translate explicitly rather than hoping:

```sh
foobar probe -- "$dest"
case $? in
0) return 0 ;;
3) return 1 ;;   # foobar's "drifted" happens to be 3
*) return 2 ;;
esac
```

And guard the channel your answer travels on. Never negate a status with `!`,
never append `|| true`, and watch pipeline tails: in `foobar list | grep -q x`,
the function's answer is grep's status, and a crashed `foobar` upstream can still
let grep answer a confident 1. Where you must pipe, prefer full-read forms
(`grep x >/dev/null` rather than `-q`) so an early-exit consumer cannot mask or
race the producer; better yet, prefer shapes where the tool under description
produces the status directly.

## Refusing loudly

A silent `return 2` is always safe, but on a shape you *expected people to hit*,
it wastes a teaching moment: the user sees "runs", with no idea one line of
modeling would have upgraded it. For those shapes, leave a breadcrumb - a short
note on Dorc's report channel explaining what was not modeled - and then decline
exactly as before. The note is an ordinary `printf` appended to a channel Dorc
provides through an environment variable (its exact name may still shift); the
`:-/dev/null` default means the same line is a harmless no-op when the file runs
outside Dorc:

```sh
[ "${2-}" = "" ] || { printf 'multi-operand form not modeled\n' >>"${DREP_V1:-/dev/null}"; return 2; }
```

A refusal is just a decline that explains itself. The plan carries the reason, the
site runs, and the next person (possibly you, next quarter) knows exactly which arm
to grow next. Reserve breadcrumbs for genuinely-expected shapes; the `*` arm
usually stays quiet, or the report becomes noise. A free-text note like this one is
the floor; a later page sharpens it into a classified refusal that tells Dorc what
kind of decline it is, and buys you more for the same one line.

## The delegation question

Many tools ship a check verb (`chezmoi verify`, `git diff --quiet`, `brew bundle
check`). Delegating to one makes a wonderfully short oracle, but the judgment does
not delegate with it. Before leaning on a tool's own verb, satisfy yourself on
three points: that the verb's yes means what your yes must mean (re-running is
acceptable noise - not just "some state matches"); that it is genuinely read-only
in the probe sense of page three; and that its exit vocabulary translates cleanly.
Where a tool's check verb is honest, delegation is the best possible oracle - the
tool's own authors maintain your check for you. Where it is not quite honest,
model around it or decline; a famous example is any orchestrator whose check mode
skips the parts it cannot check, which makes its yes worthless to stand on.

<!-- quoted: USER_STORY.md stage 4 + rung-3 ansible decline; spike/CLAUDE.md
     rul-rc-partition, rul-zero-one-inversion-pair, sigpipe-flap-class,
     identity-declared-never-inferred, decline-class-emission; 276 pipefail
     quality-bar rider; 27W:rul-report-noise-tolerant (breadcrumb -> classed) -->
