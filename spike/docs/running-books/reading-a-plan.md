# Reading a plan

A Dorc plan is your own script, annotated. Every line you wrote is present, in the
order you wrote it, byte for byte. What changes is that some lines are commented out
(they will not run), some lines have gained a small protective wrapper (they will
check themselves at the last moment), and every line that survives carries a short
reason. This page teaches you to read those three states, and to understand the
reasons.

The renders below are illustrative - the exact formatting will evolve - but the
three states, and the honesty rules behind them, are the stable part.

## The three things that can happen to a line

A line can be elided. It appears commented out, with a reason like "converged". This
is the strongest thing Dorc ever says: the machine was probed, the specific fact this
line exists to establish was found already true, and the person who described this
tool has vouched that in that situation, not running the line is acceptable. An
elided line will genuinely not execute. This is the state that saves you both time
and - more importantly - attention: on a quiet day, a well-described book collapses
to a plan of two or three lines you actually need to think about.

A line can be guarded. It appears with a check bolted in front of it, shaped like:

```sh
( systemctl_check enable --now nginx ) \
   || systemctl enable --now nginx
```

Read that as: at apply time, in sequence, run the described check; if it confirms the
work is already done, the original command is skipped right there; if it fails,
cannot tell, or confirms drift, the original command runs, untouched. A guard is what
Dorc does when it believes a line is probably already satisfied but cannot prove that
will still be true by the time execution reaches it - usually because some
undescribed command runs earlier in your book and might change anything. Guards make
your apply fast and safe, but they do not make your plan shorter: the line still
occupies your attention, and that is deliberate, because it genuinely might run.

A line can simply run. The reason tells you why Dorc could not do better. The two
reasons you will see constantly: "diverged" (the machine really does need this line
today - this is Dorc working correctly, not failing), and "unmodeled" (nobody has
described this tool, so Dorc can neither check it nor vouch for it).

## Walls, or: why the bottom half of my plan will not improve

The reason strings on guarded and running lines often say something like "past
'hork' (line 12)". This is the single most important concept in the plan.

When a command nobody has described is actually going to run, Dorc has to assume it
could change anything on the machine - because it honestly might. Every fact Dorc
proved during probing becomes untrustworthy for the lines after that command:
probing happened before, the mystery command runs in between. Such a command casts
a shadow over the rest of the book; the documentation calls it a wall. Lines below a
wall cannot be fully elided no matter how well-described they are; the best they can
get is a guard, which re-checks reality after the wall has done whatever it does.

Two things follow from this. First, an elided line casts no wall - a command that
will not run cannot change anything - so on a converged day, a well-described book
has no walls at all, and everything below a described-and-satisfied line can still
elide. Second, the highest-value description in your book is usually the first
undescribed tool, because it is shadowing everything beneath it. The plan's hints
point this out; they know the topology.

## Drift days

The plan is a function of the machine as it is right now. The same book, planned
against the same host on a different morning, can legitimately look different: if
the config file was hand-edited yesterday, the line that fixes it comes back into
the plan, and anything that depended on staying-proved below it degrades honestly
to guards. This is Dorc working as designed. Do not read a plan's shape as a
property of your book; it is a property of your book meeting today's machine.

For the same reason, a stale plan deserves a fresh one. A plan you generated before
lunch describes a machine that may no longer exist.

## What the plan will never do

It will never hide a line that might execute. It will never reorder your book. It
will never contain a command Dorc invented - every executable byte in the plan was
written by you, or is a described tool's own published check inserted in front of
your untouched line. And it is never a promise about the future: it is a measured
statement about today, honest about exactly where its knowledge ends.

One more expectation to calibrate: Dorc's analysis improves from version to version,
on purpose, and installing one new oracle can legitimately turn a running line into
an elided one. The set of lines that elide is not a stable interface. If you find
yourself wanting to alarm on the plan's shape in CI, alarm on what it says about the
world (converged or diverged) instead.

<!-- quoted: USER_STORY.md stages 1-3; spike/CLAUDE.md rul-ternary-verdict,
     rul-attention-honesty; 276:rul-verdicts-never-stable (plan-as-API);
     IMPLEMENTATION.md guarding-and-elision -->
