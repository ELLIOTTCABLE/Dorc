# Publishing, and the off-ramp

An oracle becomes worth the most the day someone else uses it. This page covers
the mechanics of that step (small) and the responsibilities it creates (real but
bounded). It also closes the loop on a promise these docs have been gesturing at
since page one: everything you wrote can leave Dorc's world intact.

## The off-ramp: strip

`dorc strip` reduces a dialect file to plain portable shell, mechanically:

- an inline bind (`dest : org.foob.Certs = "$1"`) becomes the ordinary
  assignment (`dest="$1"`);
- trailing marks vanish from their statements; a statement that was *only* a mark
  (the bare `:` colon-lines) is deleted whole, like a comment;
- a `dorc:sh` prefix reverts to bare `sh`; a Dorc shebang is rewritten to a
  plain one;
- and nothing else changes. Function names are untouched (they were plain POSIX
  names all along), comments survive, your logic is byte-for-byte yours.

What remains is exactly the defensive shell library a careful person would have
wanted anyway: a `foobar__is_converged`-shaped check function any script in any
Bourne-family shell can source and call. This is the product's deepest design
promise, and as an author you are one of its keepers - it holds only if your file
is good *as shell*, which is what the shell-dialect page was for. One mechanical
consequence to know: strip guarantees your function's last real command is still
its last status-affecting statement, so stripped checks answer exactly as the
oracle did.

The conformance test doubles as your release check: strip the file, then parse
and run it under both pinned floor shells (`dash` and `posh`). If both agree, the
file is in-dialect; make that, plus `shellcheck` and `checkbashisms`, the small
CI of your oracle repository.

## Publishing is pushing a file

There is no registry, no packaging step, no signing ceremony. Publishing an
oracle is pushing a file where people can fetch it; adopting one is downloading
it next to a book. This is deliberate - the artifact stays legible, reviewable,
and forkable, and its distribution rides infrastructure that already exists.
Which means the things that are usually a registry's job land, lightly, on you
and your file:

Say what the oracle covers. A short header comment - which tool, which verbs are
modeled, which are deliberately declined and why - costs ten lines and is the
difference between a consumer trusting your judgment and merely inheriting it.
The judgment calls especially: the purge-verb decline from page five deserves its
one-line reason in the file, because a future collaborator who cannot see the
reasoning will "fix" the decline.

Document your kinds. If you minted one, you own a vocabulary (page nine); its
meaning lives in your documentation, and other authors will use it exactly as
carefully as you explained it.

## What you own after shipping

A published oracle is a standing judgment with your name inside it. Dorc's
attribution machinery is not decorative: every elision cites the function that
answered, every survived elision cites the footprint it rested on, and when
something goes wrong the whole point of the design is that the trail ends at an
author who can fix it. The comforting inverse is equally true - when your claim
was wrong, fixing your one file repairs every book downstream of it at once. You
are not signing up for perfection; you are signing up for being findable, and for
the fix being yours to make.

Names are forever. Your function names and kind names are a compatibility
surface: books, other oracles, and Dorc itself find your work by name, and a
rename is a break for every consumer. Evolve additively - new arms, new members,
new selector tokens - and treat a rename with the gravity of a major version.

Verdicts are not yours to promise. Dorc's analysis deepens release by release,
and a book that guards today may elide tomorrow with your oracle unchanged. Never
promise consumers a specific plan shape; promise the meaning of your claims -
that your yes means what page two says a yes must mean - and let the engine's
improvement be the good news it is.

Tools drift under you. Your oracle encodes the tool as you knew it; a future
version can change a verb's meaning, an exit code, an output format. Richer
version-awareness is on Dorc's roadmap, but the durable defense is authorial and
available today: gate on what you can observe cheaply (an unfamiliar exit status
lands in your `case $? in` fallback as a `2`; an existence check fails toward
can't-say), and decline shapes whose behavior you have reason to think version-
dependent. An oracle that quietly gets *more* conservative as the world drifts
away from it is aging exactly as designed.

That is the whole arc. From here, the contract reference is your working
companion: every obligation and license from these twelve pages, collected in
one place, for the days you are writing something you intend strangers to trust.

<!-- quoted: 278 section 3 strip semantics; spike/CLAUDE.md strip-is-pure-erasure,
     stability-ledger, two-binary-floor; 276 rul-verdicts-never-stable;
     USER_STORY.md stage 4 publication; 24M names-permanent -->
