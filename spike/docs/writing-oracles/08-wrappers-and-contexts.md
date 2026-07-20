# Wrappers and contexts

Real books are full of lines like these:

```sh
sudo foobar sync-certs /etc/nginx/certs
chroot /mnt/target apt-get install -y openssh-server
env RAILS_ENV=production bin/migrate
```

Each has a command whose entire job is to run another command somewhere else: as a
different user, inside a different filesystem view, in a different network
namespace, or under a different environment. Dorc calls the outer command a
wrapper and the command it runs the guest. Until the wrapper is described, the
whole line is one opaque command - the `foobar` oracle you lovingly wrote never
even gets asked, because nothing established that the tail of a `sudo` line is a
`foobar` invocation at all.

This page covers describing wrappers, and the machinery that makes wrapped sites
first-class: probing inside the place the guest will actually run.

## Contexts and dimensions

The place a command runs is its context, and Dorc describes a context along a
small set of dimensions: which user, which filesystem view, which network
namespace, and what environment variables. `sudo` shifts the user dimension and
scrubs most of the environment; `chroot` shifts the filesystem view; `env` touches
only the environment; `nice` shifts nothing at all. A wrapped line's guest runs in
the context its wrapper chain denotes, and the same question can have different
true answers in different contexts - root's installed packages are not your user's;
a chroot's package database is not the host's. That is not a complication to
paper over; it is the actual truth of the machine, and the design's answer is to
measure *in* the denoted context rather than measure outside it and hope.

## Describing a wrapper

A wrapper's oracle has the members you already know, plus one new one, and the
familiar members do double duty.

Its `predict` models the wrapper as usual - and, in doing so, reveals the
wrapper-ness mechanically: a predict that parses a prefix of its arguments and
then runs the remainder is a peeling wrapper by construction. The analyzer sees
where `"$@"` lands and now knows how to find the guest, so the guest's own oracle
gets consulted about the tail. Nothing is declared; the model is the declaration.

The new member, `lend_map`, answers the per-dimension question: what does this
wrapper do to each dimension on the way through? Here is a coherent sudo pair:

```sh
# dorc-lang/v0.2
sudo__predict() {
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) shift 2 ;; *) shift ;; esac; done
   env -i HOME=/root "$@"
}
sudo__lend_map() {
   target=root
   while [ "${1#-}" != "$1" ]; do case "$1" in -u) target="$2"; shift 2 ;; *) shift ;; esac; done
   printf '%s\n' "$target"   : lends user
   : lends fs-view
   : lends netns
   "$@"
}
```

Read `lend_map` line by line: for the user dimension it emits a value - this
wrapper maps the user to `$target` (root unless `-u` said otherwise). For the
filesystem-view and network dimensions it emits a bare `: lends` with no value -
present but valueless means "passes through unchanged". The trailing `"$@"` is the
same peel shape as the predict, and the two must agree about where the guest
starts; disagreement between the members is caught statically and refused.

The law of this member is enumerate-every-dimension: a dimension your `lend_map`
does not mention at all is not "unchanged by default" - it is unknown, and it
walls. This inverts the usual convenience instinct on purpose. Wrappers are
precisely the commands whose whole job is changing the execution context, so an
unstated dimension in a wrapper's description is a hole in exactly the place holes
are most expensive. Say "unchanged" explicitly, once per dimension; it costs one
`lends` line.

The environment dimension has its own small vocabulary, and it is ordinary shell
read literally, mostly inside your predict's delegation line. Bare `"$@"` claims
nothing about the environment. `VAR=x "$@"` claims exactly that variable, rest
unknown. `env "$@"` claims full passthrough - the `env` syllable is the claim.
`env -i VAR=x "$@"` claims exactly-these. The sudo predict above uses the last
form: sudo scrubs, and the model says so.

## Entering a context: the entry form

Modeling tells Dorc what a wrapped site denotes. To *answer* one - to learn
whether the guest's work is already done in the place it will run - the probe has
to actually get there. That crossing has exactly one licensed seat, the wrapper's
entry form:

```sh
sudo__enter() {
   sudo -n "$@"
}
```

At probe time, Dorc composes the guest oracle's check into guest position and
enters through this form, so a `sudo foobar sync-certs X` site is answered by your
foobar check, run where the site's bytes would run. Facts born this way are keyed
to their context; nothing leaks between worlds.

Authoring an entry form is a serious vouch, in three parts. You vouch it is
non-interactive by construction: `-n` fails rather than prompts, and an entry form
must never be able to hang a probe waiting for a password. You vouch its
self-effects are acceptable probe residue - sudo writes an auth-log line and
refreshes a timestamp; you are answering for that, by name, under the page-three
contract. And you vouch for siting: that entering through your form actually lands
in the same context the site's own bytes would land in. That last one is subtler
than it looks for policy-driven wrappers - sudo policies can route different
command lines to different targets - so an entry form that cannot verify its
siting on a given host should decline (exit 2 or higher) rather than measure the
wrong world. A confident answer measured in the wrong context is the worst object
this system can produce; declining is always available and always safe. A wrapper
whose entry cost is a real mutation simply gets no entry form, and its contexts
are never entered.

One boundary to keep crisp: real entry lives only in the entry form. Your
`predict` models; it never escalates, never enters anything. The two members exist
so that modeling can be everywhere and crossing can be one auditable place.

## Consent: who has to say yes

A probe shifting context on someone's production host is not a thing that happens
because one author felt confident. Three parties align before any entered
measurement runs, and any missing yes lands on can't-say, then guard or run:

The admin holds a dial. By default, the probe re-uses only authority the
connection already holds (it never prompts, never handles credentials, never
acquires anything new), and applies it only to vouched oracle bodies; a stricter
setting forbids all context-shifting of oracle code, and a looser one exists for
admins who accept unvouched bodies. The plan discloses, up front, which contexts
it intends to enter and under what authority.

The guest's author consents per function. A verdict body written for the ambient
world might not be read-only elsewhere (page three's privilege-starvation trap, in
reverse: a body that never wrote as an unprivileged user may write as root). The
consent is a one-line mark in the body, per dimension, in the same bare-mark shape
you know:

```sh
foobar__is_converged() {
   : safe-across user
   ...
}
```

`safe-across user` says: this body's effects are read-only by design, and running
it shifted along the user dimension will not mutate. It says nothing about the
answer - answers are supposed to differ per context - and nothing about any other
function. Vouch the dimensions you have actually thought about; brace alternation
covers several (`: safe-across {user,fs-view}`).

And the wrapper's author consented by publishing the entry form at all.

Chains compose: `sudo chroot /mnt/target cmd` peels link by link, each link's
dimensions folding together, and entry composes the same way. One unmodeled link
walls the dimensions it might touch, and everything downstream of the doubt takes
the safe path. As everywhere: silence licenses nothing, every refusal is cheap,
and every crossing that does happen is attributed to the three consents that
licensed it.

## The escape hatch, and inviting analysis back in

Adjacent to wrappers proper: commands that run *code* rather than a command -
`sh -c 'some; script'`, and friends. Dorc's stance is a deliberate pair. A bare
`sh -c '...'` is the escape hatch: analysis will look inside for hints and
warnings, but licenses nothing - the payload runs, opaque, a wall, and that is a
feature. It is the one honest spelling of "Dorc, keep out", it will never grow a
second version, and books can rely on it forever. Writing `dorc:sh -c '...'`
instead is the opposite speech act: an invitation for full analysis of the
payload, as if it were book text. The `dorc:` prefix is Dorc-only spelling; the
strip pass erases it back to plain `sh`, and on a machine without Dorc the
unstripped form fails loudly rather than half-working. Everything between those
two - annotations hidden inside opaque strings, cleverness about quoting - is
refused at plan time. Code is either invited in, or walled out, never smuggled.

<!-- quoted: plans/27C sections 0-3, 6; 273 wrapper surface; plans/281 mark
     grammar v0.2 (safe-across, lends spellings); spike/CLAUDE.md
     role-menu lend_map, rho-claim-ladder, wrapper-law, dorc-sh-trio,
     context-entry-probing; 274 reentry trio; 271 rul-lend-map,
     rul-env-claim-inversion, rul-dorc-prefix-head-synthesis;
     e2e context-entry-babby fixtures -->
