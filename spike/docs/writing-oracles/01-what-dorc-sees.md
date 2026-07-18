# What Dorc sees, and where it is blind

Before writing anything for Dorc, you need an accurate picture of what it can figure
out on its own. Both failure directions are common: people over-estimate the analyzer
("surely it knows what `dpkg -s` does") and under-estimate it ("surely it can't
follow my variables"). This page draws the line precisely, because everything you
will write as an oracle author lives exactly on that line.

## Dorc reads shell fluently

Dorc parses a book the way a compiler parses a program, not the way a human skims a
text file. It builds a real syntax tree, follows control flow through your `if`s and
`&&`s and `case`s, and tracks how values move through variables. Given:

```sh
CERTS=/etc/nginx/certs
foobar sync-certs "$CERTS"
```

Dorc knows the second line invokes `foobar` with the arguments `sync-certs` and
`/etc/nginx/certs`. Ordinary shell habits - assigning a constant near the top,
quoting it at the use site - do not defeat the analysis; they feed it. Likewise Dorc
understands what the shell constructs themselves do: it knows `a || b` runs `b` only
when `a` fails, knows what `set -e` changes, knows which command's exit status a
given `if` is actually testing.

It is also honest about the edges of that fluency. Constructs it cannot fully reason
about - a variable whose value it cannot resolve, an `eval`-like escape into
generated code - do not get best-effort guesses. They get treated as opaque, loudly,
and the safe behavior (run everything, prove nothing) takes over for the affected
region. When Dorc is unsure, it degrades toward doing exactly what plain `sh` would
have done.

## Dorc knows nothing about commands

Here is the other half, and it is absolute: the name and arguments of an external
command carry no meaning whatsoever to Dorc. You know that `dpkg -s nginx` is a
harmless query and `apt-get install -y nginx` changes the machine. Dorc does not,
and cannot. To see your book the way Dorc does, rename everything:

```sh
hork -s wombat || wibble -y wombat
wombat -c /etc/myconfig
```

Is `hork` a safe check for `wibble`? Is the `wombat` in the first line the same
thing as the `wombat` command in the second? Nothing in the text can answer that.
Any tool that claimed to infer it would be guessing, and a wrong guess here means
either mutating a machine during a supposedly read-only phase, or silently skipping
a command someone needed. Dorc refuses to guess. This is a permanent design
position, not a missing feature.

So all command-level knowledge enters Dorc exactly one way: a human who knows the
tool writes it down, in shell, in a form Dorc can verify the shape of and run under
strict rules. That artifact is the oracle, and the person writing it is you.

## What an oracle is, concretely

An oracle is a file of ordinary POSIX shell functions whose names follow a
convention. The name does the wiring: a function called `foobar__is_converged`
declares "this body answers, for invocations of the `foobar` command, the question
'is this already satisfied?'". There is no registration step, no manifest, no
config file - the function's existence, under that name, is the entire mechanism.
The name is built mechanically from the command word: every character that is not a
letter, digit, or underscore becomes an underscore, then two underscores and the
role are appended. So `apt-get` gets `apt_get__is_converged`, and `foobar` gets
`foobar__is_converged`.

There are several roles a family of functions can fill for one command - answering
convergence, modeling behavior, declaring what the command disturbs - and later
pages introduce each when you need it. Every one of them is plain shell that runs
anywhere; the small Dorc-specific annotations you will meet later are designed to
strip away mechanically, leaving a defensive shell library any script could source.

## Silence licenses nothing

One rule governs every interaction between your descriptions and Dorc's decisions,
and you should install it in your head now: anything you do not say counts for
nothing. It does not count against you either - it is simply not evidence.

The reason is worth unfolding once, because the rule can feel bureaucratic until you
see the trap it avoids. Suppose descriptions worked the intuitive way: an oracle
lists the things a command changes, and Dorc trusts that anything unlisted is
untouched. Now someone describes `apt-get install` as "changes the package
database" - true, useful, incomplete, because installing a package also writes that
package's files all over the filesystem. Under the intuitive reading, a downstream
check on a config file would be judged safe from the install line, and could be
skipped exactly when it should not be. Notice who gets hurt: not the author of the
incomplete description, but some stranger's unrelated line. A *partial* description
would be more dangerous than none.

There is no clever default that escapes this; the problem (knowing what a described
action leaves alone) is a famous, provably permanent one. Dorc's answer is to make
silence inert. Describing some of a command's behavior never weakens anyone else's
safety - it can only add capabilities at your own tool's lines. When the day comes
that you want Dorc to act on "and it touches nothing else", you will say that
explicitly, in a form built for it, and the cost of being wrong will be priced and
attributed. That day is page seven; you do not need it for a long time.

## Walls, from the author's side

The user-facing consequence of unmodeled commands: when one actually runs, every
fact Dorc proved beforehand becomes stale for the lines after it. The undescribed
command is a wall; lines below it can at best get a runtime re-check (a guard), not
a clean elision. Two facts about walls matter enormously to you as an author:

An elided command casts no wall. If your two-minute oracle lets Dorc prove your
tool's line unnecessary today, that line will not run, so it cannot invalidate
anything - and every line below it keeps whatever certainty it had. A tiny oracle
for one noisy, early, usually-converged command frequently rescues the entire rest
of the book. This is why minimal oracles are the steepest part of the value curve,
and why Dorc's hints push them first.

And walls fall one tool at a time, each by its own author. There is no global
cleverness that removes them; there is only the next person who knows the next tool
writing eleven lines of shell. That is the ecosystem you are joining, and the next
page gets you your first working oracle.

<!-- quoted: DESIGN.md inference-limitations; IMPLEMENTATION.md guarding + collaboration;
     23O frame-problem narrative; spike/CLAUDE.md silence-licenses-nothing,
     inv-top-reject, structural-vouch-only; 24M bare munged names -->
