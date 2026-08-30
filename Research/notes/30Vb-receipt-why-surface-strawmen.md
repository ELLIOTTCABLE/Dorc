# 30Vb — receipt-backed `dorc why`: the strawman surface set (TUI-as-spec)

> Conductor-authored (Fable, 2026-08-30), at human direction: a hand-built set
> of prospective `dorc why` renders covering the important state-space, handed
> to the render builder as the thing to build TOWARD. Spec-tier direction, not
> byte contract: STRUCTURE, SELECTION, and REGISTER are the spec; exact wording
> is strawman and final prose still flows through the loom
> (`spike/CLAUDE.md` render-form-unwelded · error-authorship-tier — goldens
> re-bless freely; `[unwritten:]` stays a legal resting state). Everything here
> is UNACKED until the human adjudicates the ask-list at the tail; the
> ack-ledger lives in `30Va`.

## §0 constraints-every-render-obeys

The laws these renders are drawn to satisfy, so the builder can check a render
against the law rather than against my taste:

- law-anchor-recovery (USER_STORY "Recovery"): every explanation is a chain;
  every link wears a label saying what KIND of thing it is; deciding and
  explaining fail in opposite directions — where certainty runs out, the chain
  says so rather than rounding up.
- `AID-NEEDS:law-trust-tier-is-syntax` — the labels (measured / vouched / ran /
  claimed / derived / consented / declined) are a typed field rendered
  uniformly; prose never hand-writes epistemics.
- `AID-NEEDS:law-selection-is-goal-derived` — each default render derives
  backwards from the asked question; curation is deeply effective or entirely
  absent; `--all` is the labeled absent-curation tier, a grown answer, never a
  different one.
- `AID-NEEDS:law-pull-runs-wide-open` — these are all pull surfaces; generous
  by default, exhaustive one labeled step away.
- `AID-NEEDS:law-plain-language-surfaces` — no jargon, no corpus vocabulary,
  no lattice symbols. ("skip" is legal here: the ban is design-layer only.)
- `AID-NEEDS:law-lineno-identity` — one line-number space, the source file's.
- `AID-NEEDS:law-whylog-is-sensitive` + `30R` encoder law — recorded host
  bytes leave the model only through the class-aware encoder; a render may
  show a value's CLASS and refuse its bytes (spelled here as
  `‹withheld: host output›`); no render ever claims "scrubbed" or "safe".
- `plans/30R` recorded-versus-rederived — v1 speaks in the recorded arm only;
  every chain must leave visible room for a later re-derived arm
  ("re-checked today: agrees/disagrees") without a layout rework.
- `271:rul-sin-ordering` — mis-attribution is the worst failure; every
  tie-break in these renders resolves toward saying less, labeled, over
  saying more, wrongly.

## §1 the-spec-decisions (each independently vetoable; slugs for cheap acks)

- dec-frame-declared-once — every receipt-backed answer opens with a one/two
  line frame: receipt kind + book + host + moment, then the three states that
  scope everything below (authenticated? complete? book drifted since?).
  Inside the frame, speak plainly in past tense; no per-line hedging.
- dec-labels-on-every-link — numbered chain links, label column left of the
  text, the seven speech-act words spelled out in English.
- dec-two-move-epilogue — every problem-shaped answer ends with the same two
  verbs: `to check:` (re-measure; the answer that does not depend on the
  receipt) and `to fix:` (the leverage point, one file:line, with the reach of
  the repair stated).
- dec-refusal-shows-both-states — an address whose bytes changed refuses the
  address-specific answer pending an explicit selector, shows BOTH source
  states, and still renders everything unrelated it can.
- dec-ambiguity-lists-candidates — where "last" or "this one" would be a
  guess, the render lists the candidates with identities and the exact
  re-invocation, and guesses nothing.
- dec-broken-leads-with-the-break — a damaged/unauthenticated receipt render
  leads with what failed, then bounded recovered structure with every
  recovered item individually labeled `unauthenticated`; ends by routing to a
  fresh plan, never to trust.
- dec-no-outcome-is-a-plain-sentence — a missing graph edge is narrated as
  exactly what is and is not known, never rounded in either direction.
- dec-quiet-receipt-still-answers — zero problems renders a short positive
  account (counts + the one structural fact worth knowing), never zero bytes.
- dec-oracle-join-is-tri-state — oracle text can only be shown by joining
  CURRENT source against the recorded digest: matches → inline it, labeled
  "today's file, unchanged since"; drifted → name file:line + "changed since;
  the acting claim's text is not stored" and decline to inline (rich receipts
  keep book bytes, oracles are digest-only — `plans/30R`); absent → identity
  only. Never show today's text as if it acted.
- dec-hints-ride-pull-answers — enhancement hints (the first-wall nudge)
  appear inside pull answers where the asked question implicates them; they
  are the interesting thing in a quiet receipt.
- dec-counts-close-the-answer — default renders end with what was NOT shown,
  counted, and the labeled step to it (`--all`).

## §2 the surfaces

Common cast (USER_STORY): `webhost.sh` on `web1.example.net`; plan 06:11
(measurements 06:11:52), apply 06:12, outcome recorded 06:14, 2026-07-17.
Tool-lines 5–11; `foobar` oracled at stage 4 (with `disturbs`), `hork` forever
undescribed. Receipt ids (`r-0717-0612-web1-a3`) and flag spellings marked
STRAWMAN are unruled surface.

### S1 why-zero-arg-problems — "anything I should know?" × ordinary drifted-day apply (no risk flag; guards did their jobs)

```
$ dorc why
reading the newest receipt: apply of webhost.sh on web1.example.net,
2026-07-17 06:12   (authenticated · complete · the book is unchanged since)

what deserves attention, out of 11 lines:

  8   foobar sync-certs "$CERTS"        ran -- its certs were measured
                                        out-of-date at 06:11:52
  9   systemctl enable --now nginx      checked itself again live (line 8 had
                                        really run just above it), found the
                                        service fine, skipped
 10   hork tune --profile web ...       ran -- nothing describes 'hork', so it
                                        runs on every apply, and every line
                                        after it loses its morning measurement
 11   ufw allow 443/tcp                 checked itself again live (past line
                                        10), found its rule present, skipped

the other 3 tool-lines never reached the host: each was measured already-done
at 06:11 and its describing author accepts that as reason not to re-run.
`dorc why <line>` tells any line's whole story; `dorc why --all` prints every
decision, the quiet ones included.
```

notes: the zero-arg default is the problems-report (`AID-NEEDS:
aid-why-problems-report`) recast over a receipt; selection = lines that acted
or re-checked, plus the standing structural cost (line 10). The closing count
is dec-counts-close-the-answer.

### S2 why-line-flagship — "why didn't line 9 run?" × the bad morning (risk flag typed; survived elision; incomplete at-most claim)

The USER_STORY "Recovery" render is the settled register; this is it,
receipt-backed. Deltas from USER_STORY, deliberate: the frame header; source
join-state on links 2 and 4; `webhost.sh:9` not `book.sh:9` (one book, one
name).

```
$ dorc why 9
receipt: apply of webhost.sh on web1.example.net, 2026-07-17 06:12
(authenticated · complete · the book is unchanged since)

webhost.sh:9  systemctl enable --now nginx
  removed from the plan (elided); did not run in that apply.

  it was removed because all of the following held together:
  1. measured:   nginx was enabled+active on web1 at plan time (06:11:52)
  2. vouched:    the service oracle's author accepts already-enabled+active as
                 reason enough not to re-run this (systemctl.oracle.sh:12 --
                 today's file, unchanged since that apply)
  3. ran above:  webhost.sh:8 `foobar sync-certs /etc/nginx/certs` really ran
                 -- ordinarily that would have sent line 9 back into the plan
                 as a live re-check --
  4. claimed:    but foobar's oracle claims sync-certs disturbs at most its
                 own certs (foobar.oracle.sh:31 -- an author's claim; nothing
                 verified it)
  5. derived:    that claim does not overlap what link 1 measured
  6. consented:  --risk-faultless-skips was typed, which is what lets 4+5 keep
                 a line out of the plan past a running mutation.

  if line 9 SHOULD have run: this receipt cannot see which link is wrong, but
  the links are not equally trustworthy -- 4 is the one unverified human claim
  in the chain. if `foobar sync-certs` also touches service state, that claim
  is what wrongly kept line 9 out.
  to check:  `dorc plan webhost.sh web1` re-measures the world as it is now.
  to fix:    foobar.oracle.sh:31 is the line to widen; every book using that
             oracle inherits the repair.
```

notes: dec-oracle-join-is-tri-state shows on link 2 ("today's file, unchanged
since"). Had `foobar.oracle.sh` drifted, link 4 reads
`(foobar.oracle.sh:31 -- an author's claim; that file has changed since this
apply, and the acting claim's text is not stored)` and inlines nothing.
Layout leaves the re-derived arm room: a future
`re-checked today: nginx is enabled but NOT active (disagrees with link 1)`
slots under link 1 without rework.

### S3 why-address-refused — "why did line 9 …" × the line's own bytes changed since

```
$ dorc why 9
receipt: apply of webhost.sh on web1.example.net, 2026-07-17 06:12
(authenticated · complete · the book has changed since, at this line)

webhost.sh:9 is not the line that apply saw:
  recorded:  systemctl enable --now nginx
  today:     systemctl reload nginx

those are different operations, and an answer about "line 9" would be about
the wrong one. say which you mean:
  dorc why 9 --as-recorded      the line the apply actually decided about
  dorc why 9 --as-written       today's line (no receipt speaks for it yet)

the receipt still answers for the rest of the book: `dorc why` for its
summary.
```

notes: `plans/30R`'s refusal-pending-selector, embodied; both states shown
(dec-refusal-shows-both-states); unrelated value still offered. Selector
spellings STRAWMAN (ask-selector-spellings). `--as-written` deliberately
answers "no receipt speaks for it yet" rather than pretending an answer.

### S4 why-historical-label — line bytes match, other book bytes drifted

Header-only delta; body renders as S2 with one added frame line:

```
(authenticated · complete · the book has changed since -- not at this line;
 this answer explains the 06:12 apply, not today's book)
```

notes: `plans/30R`: recorded answer stays available under a book-drift
warning, labeled historical until a precise dependency comparison exists.
The label lives in the frame, once — dec-frame-declared-once.

### S5 why-run-cause — "why does line 10 run every time?" × the permanent wall

```
$ dorc why 10
receipt: apply of webhost.sh on web1.example.net, 2026-07-17 06:12
(authenticated · complete · the book is unchanged since)

webhost.sh:10  hork tune --profile web >>/var/log/hork.log 2>&1
  ran, as it does on every apply.

  1. derived:  nothing describes 'hork' -- no loaded file names it, so Dorc
               may neither test whether this line is needed nor leave it out.
               silence never earns a skip.
  2. derived:  because it always runs and is undescribed, every measurement
               from above it stops being trustworthy below it: lines 11+ pay
               for this line by re-checking live instead of staying out of
               the plan.

  what would change this: a small convergence check for 'hork', written as a
  function named hork__is_converged -- in this book itself is fine. its yes
  would let this line skip when converged, and un-wall line 11 the same
  morning.
```

notes: dec-hints-ride-pull-answers — the nudge is the honest answer to this
question, not an intrusion. No epilogue verbs: nothing is broken
(dec-two-move-epilogue binds problem-shaped answers only).

### S6 why-outcome-divergence — "what did the apply actually do?" × guard caught live drift

```
$ dorc why --receipt-id r-0717-0612-web1-a3
receipt: apply outcome for webhost.sh on web1.example.net, recorded
2026-07-17 06:14   (authenticated · complete · cites: intent 06:12:03,
plan 06:11)

the apply ran to completion. 2 commands ran, 2 checked themselves live and
skipped, 3 were never dispatched. one thing went differently than planned:

 11  ufw allow 443/tcp
     planned:   check live, expected already-done          derived
     happened:  the live check answered not-done, so the   measured
                original command ran; it exited 0
     nothing to do -- this is what a check-in-place is for. noted because it
     means the firewall changed between 06:11's measurement and 06:12's
     apply.
```

notes: `rul-divergence-proceed` narrated post-hoc: divergence is a report
item, never an alarm; the render says so in one sentence. DATA-DEPENDENCY:
v1 `ApplyOutcome` carries only its DST-route fields — this render is the
target shape; early builds may honestly render fewer rows.

### S7 why-intent-no-outcome — "what happened last night?" × controller died mid-apply

```
$ dorc why --receipt-last
the newest receipt is an apply INTENT: the moment before dispatch,
2026-07-17 06:12:03, webhost.sh on web1.example.net. no outcome receipt
exists beside it.

what that means, plainly: Dorc committed to running the plan below and no
record of what happened next was written. results held in memory are lost if
the controller dies mid-apply -- so the host may hold any prefix of that
work, including all or none of it.

what IS recorded (at 06:12:03): the exact scripts and files that apply would
use, 2 commands expected to run, 2 to check themselves, 3 left out.
`--all` lists the adopted assignment images.

to check: `dorc plan webhost.sh web1` re-measures the world as it is now --
its answer does not depend on this gap, and the next receipt will be whole.
```

notes: dec-no-outcome-is-a-plain-sentence; missing edges imply nothing in
either direction (`inv-graph-edges-are-explicit`, narrated). The one-move
epilogue: there is no `to fix:` because nothing is attributably broken.

### S8 why-last-ambiguous — `--receipt-last` × two incomparable terminal roots

```
$ dorc why
two receipts are equally newest and unrelated to each other:

  r-0717-0612-web1-a3   apply of webhost.sh on web1.example.net, 06:12
  r-0717-0609-db3-11f   plan of dbhost.sh on db3.example.net, 06:09

neither leads to the other, so "the last one" would be a guess. pick one:
  dorc why --receipt-id r-0717-0612-web1-a3
```

notes: `plans/30R`: incomparable roots report ambiguity, never an arbitrary
tie-break; graph ancestors already collapsed beneath terminal members, so the
list stays short (dec-ambiguity-lists-candidates).

### S9 why-damaged-receipt — an edited/truncated/converted file

```
$ dorc why --receipt ./ops/receipts/jul17-apply.dorcr
this file does not authenticate: its validation record does not match its own
bytes. editing, truncation, or newline conversion is enough to cause this --
a CRLF rewrite by an editor or a sync tool is the common case.

nothing below is trustworthy enough to rely on. what could still be read:

  unauthenticated   names itself: apply outcome, webhost.sh on
                    web1.example.net, 2026-07-17 06:14
  unauthenticated   7 of 9 line records parse; the record for lines 10-11 is
                    cut off mid-entry

this partial reading can answer curiosity and nothing else; it fills no gap
in the receipt graph and licenses nothing.
to check: `dorc plan webhost.sh web1` re-measures the world as it is now.
```

notes: dec-broken-leads-with-the-break; `plans/30R`: aid does not stop merely
because trust was lost — bounded report-only recovery with the break
attached; every recovered item individually labeled. The CRLF sentence earns
its place (`an-crlf-hazard`; receipts are byte-identity documents).

### S10 why-quiet-receipt — everything converged, nothing ran

```
$ dorc why
reading the newest receipt: apply of webhost.sh on web1.example.net,
2026-07-17 06:12   (authenticated · complete · the book is unchanged since)

nothing needed doing: all 5 described tool-lines were measured already-done
at 06:11 and left out; 'hork' (line 10) ran as always, and line 11 re-checked
itself behind it, found its rule present, and skipped. no divergence, no
refusals, nothing unexplained.

the standing cost in this book is line 10: 'hork' is undescribed, so it runs
every time and taxes line 11 forever. a small hork__is_converged function
would retire both. otherwise: this book is as quiet as it can get.
```

notes: dec-quiet-receipt-still-answers — the quiet render is the trust
calibration for the loud ones; in an otherwise-quiet receipt the quiet
classes ARE the interesting thing (`law-selection-is-goal-derived`, worked
consequence).

### S11 why-all-depth — the labeled absent-curation step (excerpt)

```
$ dorc why 9 --all
[ the S2 chain, unchanged, then: ]

  everything else this receipt holds about line 9, quiet decisions included:
  7. derived:   its measurement came from the service oracle's own check,
                shipped read-only at 06:11:52; the check exited 0
  8. measured:  probe timing: 240ms, third of 5 parallel checks on web1
  9. derived:   no other loaded oracle's claims reached this line's
                measurement; 2 candidate claims compared as unrelated
  ...
```

notes: `--all` GROWS the same chain — same numbering continues, same labels,
never a re-arranged or contradictory answer (`law-pull-runs-wide-open`; the
exhaustive tier is one labeled step). At its deepest it prints everything the
report lane received, noise included, attributed, never silently dropped.

### S12 why-plan-unapplied — the newest receipt never acted

```
$ dorc why
the newest receipt is a PLAN: webhost.sh on web1.example.net, 2026-07-17
06:11. no apply cites it -- nothing here has acted on the host.

it proposed: 3 lines left out (measured already-done), 2 to check themselves
in place, 2 to run. `dorc why <line>` explains any of those decisions;
whether it was ever carried out, no receipt says.
```

notes: the plan/intent/outcome species distinction surfaces to the user as
plain sentences about what kind of moment each receipt records.

## §3 cross-cutting notes for the render builder

- The renders above are selection + arrangement targets. Where prose is owed,
  build the arrangement with `[unwritten: <slug>]` registers per the loom
  flow; my wording here may be lifted into registers only through
  `dorc-loom publish` (slop-tier) — never hand-pasted into the catalog.
- Every value interpolated above (paths, argv, rc's, timestamps) must reach
  the render through the encoder-mediated exit; any class the encoder
  declines renders as a typed withhold (`‹withheld: …›`), never elided
  silently.
- Column discipline: the label column must survive a future second arm
  (re-derived) beside the recorded one; do not design a layout that assumes
  one arm.
- Counts in frames and closers ("2 ran, 2 checked, 3 left out") come from the
  decision digest, not recomputed from rendered rows.

## §4 ask-list (adjudication queue for the human)

- ask-selector-spellings — `--as-recorded` / `--as-written` for the S3
  refusal fork: keep, or other names?
- ask-receipt-id-shape — ids render here as `r-0717-0612-web1-a3` STRAWMAN;
  the real id shape is the implementation's. Any constraints you want
  (host-in-name is already ruled OUT of filenames by `plans/30R` policy —
  does the RENDERED id get the same treatment?).
- ask-impersonal-register — these renders never say "I"; Dorc speaks
  impersonally ("this receipt cannot see…"). Confirm the register.
- ask-lowercase-voice — sentences are lowercase-terse, matching USER_STORY's
  settled render. Confirm as the register for pull surfaces.
- ask-hints-at-default-depth — S5/S10 include the enhancement nudge in the
  default pull render (my reading of law-selection-is-goal-derived). Confirm,
  or restrict hints to `--all`/lint surfaces.
- ask-probe-timing-in-all — S11 link 8 exposes probe timing under `--all`;
  timing is receipt content with review implications
  (`rul-durable-contents-reviewed-before-design` governs the DURABLE side —
  if timings are not in the sealed model today, this link dies rather than
  widening the durable).
