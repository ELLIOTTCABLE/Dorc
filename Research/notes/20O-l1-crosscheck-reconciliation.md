# 20O — L1 loop-lowering crosscheck reconciliation + fix dispositions

> Post-L1 adversarial pass (single adversarial agent, parser-surface target per the
> standing discipline; commit under review: 5122589 with B3 arriving mid-review —
> findings checked against both states). Orchestrator-traced; dispositions below.

- **find-1 (priority-1, demonstrated): unquoted glob/tilde LITERALS are wrong concretes.**
  `for f in *.conf` binds the literal; dash expands per-filesystem (count AND membership
  wrong); driven end-to-end to a wrong elision via the post-loop var. Pre-existing for
  straight-line argv (`install *.deb`); L1 widened the channel. The glob guard exists in
  `sem` (GLOB_CHARS) but was wired only into split-result fields. **Fix dispatched
  (task-F1): an unquoted literal fragment containing `* ? [`, or a word-leading unquoted
  `~`, degrades the word to ⊤ — quoted forms stay concrete (dash-literal).**
- **find-2 (demonstrated): line-granular Replace eats loop/`fi` scaffolding sharing its
  line** (`done; install` → commented `done` → broken apply, partial host execution).
  Pre-existing class (`fi;` too); corpus-blind. **Deferred one slice: task-R is mid-flight
  consolidating the render-assembler; the fix (extend the T14 in-situ machinery to all
  structural-scaffolding lines, or refuse-and-Run) lands on the consolidated assembler.**
  Tracked here; the corpus gains the boundary cases with that fix.
- **find-3 (demonstrated): construct-trailing redirections (`done < file`, `fi > log`)
  silently misparse into a phantom empty-argv command** — zero diagnostics on THE most
  idiomatic loop shape (`while read line; … done < input`), contradicting loud-⊤; currently
  contained (phantom is MustRun; body floored) with three recorded latents. **Fix
  dispatched (task-F1): loud ⊤-reject of construct-trailing redirects (honest interim;
  full modeling = body-consumption marking, deferred to the member-elision slice's
  preconditions).**
- **find-4 (demonstrated, both directions): for-list wordlist terminates at any reserved
  word; dash ends it only at `;`/newline.** Engine-accepts/dash-rejects is the bad
  direction (plans/probes for an unrunnable book). **Fix dispatched (task-F1): wordlist
  ends only at `;`/newline; reserved words in list position are ordinary words.**
- **find-5: condition-position `break` not detected** (body-only check) — sound via
  Opaque-poison but inconsistent with the ⊤-reject message. **Fix dispatched (task-F1):
  detect in condition region too.**
- **find-6 (latent, floor-masked — recorded as PRECONDITIONS for the member-elision
  slice):** post-loop `$?` after `while` marks the condition's last command, not the
  body's (dash: body-last or 0); `done > file` body-stdout consumption unmarked; in-loop
  Queries still compiled into the probe (wasted remote work + find-1's argument channel
  until fixed). None bites while the in-loop floor stands.
- **Did-not-survive list** (verified by the pass; valuable negatives): until-sense,
  errexit-across-back-edge joins, loop-in-condition exemptions, function-mediated break
  (dash itself doesn't propagate it — the full-iteration model is faithful), empty-list
  semantics, brace-expansion absence, post-loop pristine-prefix flow, nested convergence,
  pipeline-stage loops.
