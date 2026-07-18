# Naming state: kinds, entities, and selectors

The oracle you wrote on the previous pages works, and if all you want is your own
tool's line eliding, you can stop there. This page is where you start giving Dorc
something more: names for the pieces of machine state your tool deals in, so that
your facts and other people's facts can interact.

## Why names have to be explicit

Consider two lines from different authors' worlds meeting in one book:

```sh
apt-get install -y nginx
systemctl enable --now nginx
```

You know these are related - the second manipulates a service that the first
installed. But that "nginx" is the same nginx is command-level knowledge, exactly
the kind Dorc refuses to guess at. The apt oracle's author and the systemctl
oracle's author have never met; if their facts are going to compose (and features
you will meet soon depend on knowing when two facts concern the same state, and
when they provably do not), they need a shared way to say what a fact is about.

Dorc's answer is a small addressing scheme, written inline in your oracle. An
address is called a coordinate, and it has up to three parts:

```
sm.dorc.Service:"$svc"#enabled
\____________/ \____/ \______/
    kind       entity selector
```

The kind names a vocabulary: a namespace of one type of describable state, spelled
like reverse DNS with at least two dots (`sm.dorc.Service`, `org.foob.Certs`).
The base library's kinds live under its own prefix; when you need a kind nobody has
minted, you mint one under a domain you plausibly answer for, the same social rule
as Java package names. There is no registry and no approval step - a kind only has
to agree with itself - but reusing someone else's kind means reading how its owner
documents it and following that. Names you made up under someone else's domain are
a contract violation, not a clever shortcut.

The entity says which one: a package name, a unit name, a path. Simple values
(letters, digits, `.`, `_`, `-`, `/`) can be written bare; anything else, and
anything coming from a variable, takes double quotes with normal shell
interpolation: `"$dest"`.

The selector, introduced by `#`, names one aspect of the entity - one cell of
state. A systemd unit's `#enabled` and `#active` are different cells: `enable --now`
establishes both, but observing one tells you nothing about the other, and a good
oracle keeps them separate. Selector tokens are simple identifiers (letter or
underscore first, then letters, digits, underscores). Leaving the selector off
means the whole entity, which is a much blunter claim - useful, but it interacts
with everything about that entity, so prefer a selector when you can name the
aspect you actually mean.

One honest subtlety to file away: a coordinate is a name, and names are not the
things themselves. Two names can refer to one underlying thing (a package under an
alias, a path through a symlink), and machinery exists for a kind's owner to teach
Dorc about that - it comes on the kind-ownership page. Until then, just know the
system is aware that name-inequality does not prove thing-inequality.

## The marker line

The plain-shell oracle from page two needed nothing special. Coordinates, and the
marks below, are extra syntax that a stock shell would not understand, so a file
that uses them must declare the dialect once, near the top, on its own line:

```sh
# dorc-lang/v0.1
```

To every other tool on earth that line is a comment. To Dorc it switches on the
extra syntax for this file (and pins which version of it, so files never rot as
the dialect evolves). A file without the marker is treated as plain shell, full
stop. Function names like `foobar__is_converged` are recognized either way - they
are ordinary POSIX names and need no dialect - so the marker becomes necessary
exactly when you first write a bind or a mark.

## Marks: attaching a fact to a statement

A mark rides at the end of a runnable statement, after some whitespace, and states
what that statement's exit status means in coordinate terms:

```sh
foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest"#synced
```

Read it as: this command reads the world, and its exit status answers for that
cell - 0 means the cell holds, 1 means it does not. Dorc never interprets the
token `synced`; it is opaque, meaningful only because this line consistently means
it. What the mark buys you: the fact now has an address. The plan can report it
("converged: org.foob.Certs:/etc/nginx/certs#synced" instead of a vague "check
passed"), and the machinery that tracks which commands disturb which state can be
precise about yours instead of conservative about everything.

There are three mark sigils, and each marked line asserts exactly one thing:

```sh
tool query "$x"      : some.kind.Name:"$x"#present     # verdict: 0 means it holds
tool absent "$x"     :! some.kind.Name:"$x"#present    # verdict, complement sense
tool peek "$y"       :? some.kind.Name:"$y"#mode       # observe: this line reads that cell
```

The plain `:` form you have seen. The `:!` form is for arms whose converged state
is an absence - a `remove` verb's check, where exit 0 must mean "the thing is
gone", which is the complement of `#present` holding. Spelling the sense on the
mark keeps a hard rule intact: you never invert an exit status yourself (no `!`,
no exit-code gymnastics), because hand-inverted statuses are how "cannot say"
accidentally becomes "yes".

The `:?` observe form asserts nothing about convergence; it discloses a read. Use
it when your verdict genuinely depends on some other cell along the way: it tells
Dorc your fact also goes stale when *that* cell is disturbed, which keeps your
fact honest. Disclosing reads never weakens your oracle - it widens the set of
events that invalidate your fact, which is exactly the safe direction to be more
truthful in.

If one statement genuinely establishes two cells, that is two probe lines, each
with its own mark. Resist compressing; per-line marks are what make every fact
attributable to the one line that measured it.

## Binds: naming the entity as it flows

The last piece of the dialect on this page. When an argument is an entity, you can
bind its name inline, at the moment you receive it:

```sh
dest : org.foob.Certs = "$1"
```

This is an assignment (`dest="$1"`) plus a declaration: the value flowing through
`dest` names an entity of kind `org.foob.Certs`. Binds name entities, never cells
- facts about cells come from marks on the statements that actually measure them.
The payoff is that the analyzer can follow the entity through your body and out to
the book: the book wrote `foobar sync-certs "$CERTS"`, the analyzer resolved
`$CERTS` to `/etc/nginx/certs`, your bind names it, and the plan's reason line can
say precisely which certs directory it proved converged.

Putting the page together, the first oracle grown into its named form:

```sh
# dorc-lang/v0.1
foobar__is_converged() {
   verb="$1"; shift
   case "$verb" in
   sync-certs)
      dest : org.foob.Certs = "$1"
      [ "${2-}" = "" ] || return 2
      foobar status --certs-current -- "$dest"   : org.foob.Certs:"$dest"#synced
      ;;
   *) return 2 ;;
   esac
}
```

Same behavior as before on its own line; but its fact is now addressed, reportable,
and ready to participate in everything the later pages add. And all of it reduces
mechanically back to plain shell: strip the file and the bind becomes an ordinary
assignment, the marks vanish whole, and what remains is the defensive check
function you would have wanted in your shell library anyway.

<!-- quoted: 277 sections 1, 3, 4a-4f; 278 authored-additions; 271
     rul-selector-introducer-hash, rul-binds-entity-only-provisional;
     24M reverse-dns kinds; USER_STORY.md stage 3; spike/CLAUDE.md
     coordinate-semantics, marker-gates-syntax-only -->
