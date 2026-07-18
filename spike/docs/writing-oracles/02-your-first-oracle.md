# Your first oracle

This page gets you from nothing to a working oracle in about two minutes of typing,
and then explains exactly what you just signed. The order matters: the shape is
easy, and the meaning is the part people get wrong.

## The situation

Your book has a line like this in it:

```sh
foobar sync-certs /etc/nginx/certs
```

`foobar` is your own little tool - or a vendor's tool you operate every day. Nobody
has described it to Dorc, so the line runs on every apply, and (because an
undescribed command might change anything) it degrades what Dorc can prove about
every line below it. The plan has been hinting that a description of `foobar` would
be the single most valuable thing you could add.

Like most reasonable tools, `foobar` has some way to ask whether its work is
already done - a status query, a check verb, a verify subcommand - that exits 0 for
"all good" and nonzero otherwise.

## The minimal oracle

Create a file next to your book (or append to the book itself; both work) and write
one function:

```sh
foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs)
      [ "${1-}" != "" ] || return 2
      [ "${2-}" = ""  ] || return 2
      foobar status --certs-current -- "$1"
      ;;
   *) return 2 ;;
   esac
}
```

That is the whole thing. No registration, no metadata, no new language. The
function name is the wiring: `foobar__is_converged` means "for invocations of the
command `foobar`, this body answers the question in its name". The name is derived
mechanically from the command word - replace anything that is not a letter, digit,
or underscore with an underscore, then append the role - so the oracle for
`apt-get` would be named `apt_get__is_converged`.

When Dorc plans a book containing a `foobar` site, it calls this function with that
line's actual arguments (everything after the command word, exactly as your book's
variables resolved). Your body inspects them like any argument parser would, and
answers through its exit status.

## The exit-status contract

The answer rides the function's final exit status, read against one fixed table:

- `0` means: the state this invocation exists to establish already holds.
- `1` means: it does not hold - the machine has drifted, the line is needed.
- `2` or anything higher means: I cannot say. The line runs. Always.

That third row is the load-bearing one, and it is why the example above is safe.
The `*) return 2` arm means an invocation you never thought about - a verb added
next year, a colleague's creative flag usage - gets no answer at all, which leaves
that line exactly as safe as if your oracle did not exist. The two `return 2`
lines above it are argument gates for the same purpose: a `sync-certs` call with a
missing operand, or with two operands when you only modeled one, declines rather
than half-answering. Declining is ordinary control flow, costs one line, and is
always the right move at any edge of your knowledge. When in doubt, `return 2`.

Two mechanical consequences of answering through exit status. First, your last real
command must be the last thing that affects the function's status - do not append
`|| true`, do not negate with `!`, and be careful that a trailing cleanup line does
not overwrite your answer. Second, if the underlying tool has a rich exit
vocabulary (curl, grep-with-errors, diff), map it explicitly rather than passing it
through: a tool's `2` might mean "definitely drifted", but in this contract `2`
means "cannot say", so translate with a `case $? in` if the meanings differ.

Prefer the shape shown - the tool itself performs the check, and its exit status
flows out as your answer - over capturing output and comparing strings. Exit codes
are the native currency of this contract, the direct form is easier for both humans
and the analyzer to reason about, and later features reward it.

## What you just signed

Here is the part to read slowly. By writing a function with that name, you did not
just provide information - you granted a license. When your function answers `0`,
Dorc may insert your check ahead of the original line as a last-moment guard, and,
when the probe can prove the answer applies, remove the line from the plan
entirely. Your yes makes commands not run.

So the name is a contract, and your yes must mean something quite specific: "in
this situation, not running this command is an outcome I accept - whatever else
this invocation might have done is noise I am fine with." That is a judgment call
about the tool, and only someone who knows the tool can make it. The classic
subtlety: a package can be installed and have an upgrade pending. Is that
converged? If the install line would have upgraded it, your `0` just declined the
upgrade on the user's behalf. Whether that is correct is not Dorc's decision - it
is yours, made once, in this function, with your name on it. Every elision and
guard the plan produces is attributed to the function that answered; when the
answer is wrong, there is a person to be wrong.

The other thing you signed is stricter, and has its own page next: this body will
be executed during planning, on machines you do not own, under a promise Dorc has
already made that planning never mutates anything. Your function is now part of
keeping that promise.

## What it buys

Plan the book again. On a converged day, the `foobar` line is commented out with a
reason naming your function - and, because an elided line casts no wall, every
described line below it regains full strength too. One small oracle frequently
un-walls half a book; that is normal, not luck. On a drifted day the same plan
honestly re-degrades: your check answers `1`, the line comes back, and downstream
lines fall back to guards for that apply. The plan is a function of the day; your
oracle is what lets it be.

If your tool has a single self-check verb that already means exactly what you want,
the whole oracle can be a one-liner delegating to it:

```sh
chezmoi__is_converged() {
   case "$1" in
   apply) chezmoi verify ;;
   *)     return 2 ;;
   esac
}
```

Note what even this tiny example declines: everything except `apply`. The author of
this oracle knew that `chezmoi update` pulls from a remote first, so local
verification cannot answer for it - a `return 2` is that knowledge, expressed in
one line. Delegation is a shape, not a free lunch; the judgment stays yours.

<!-- quoted: USER_STORY.md stage 3 + chezmoi story; spike/CLAUDE.md
     rul-vouch-is-verdict-authoring, rul-rc-partition, rc-naming-discipline;
     23O rul-role-split; 27Q teach-marked-command-not-cmdsub -->
