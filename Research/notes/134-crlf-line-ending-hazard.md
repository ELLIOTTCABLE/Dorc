# 134 — F-CRLF: the Windows-authored line-ending hazard (round 13, 2026-06-04)

> Durable per-front notes for the platform-compatibility round (plan: `plans/130-platform-compatibility-research-plan.md`).
> interactive-research front **F-CRLF** — does a Windows-authored `.dorc.sh` (CRLF) break on a *nix target, and
> what's the mitigation? Small, bounded front (well-understood phenomenon); 2 graded sources. Gather-and-grade.

## Findings (most-attended first)
- **+SURE the hazard is real and *specifically* dangerous because it fails below the shell.** A shebang with a
  trailing `\r` (`#!/bin/sh\r`) is a **kernel-level exec failure** — the kernel tries to run the interpreter
  `/bin/sh\r`, which doesn't exist (`bad interpreter: No such file or directory`). "Bash never sees the file; the
  error is when trying to find which interpreter to use." ⇒ **un-guardable from *within* the script** (the script
  never starts). [C-unix-se-crlf-shebang-2016]
- **+SURE even a clean shebang doesn't save you — `\r` silently corrupts other lines.** If Dorc invokes `sh script`
  (no shebang dependence), trailing `\r` still poisons string comparisons (`[ "$x" = yes ]` vs `yes\r`), heredoc
  terminators (`EOF\r` ≠ `EOF`), `read`, and `case` patterns — mostly *silent* or baffling failures, not clean errors.
- **+SURE the standard fixes are repo-layer and tool-layer**, both well-trodden: `.gitattributes` `*.sh text eol=lf`
  (normalizes to LF in the index on check-in, even retroactively) [A-git-gitattributes-2024]; `dos2unix` / `tr -d '\r'`
  at the boundary [C-unix-se-crlf-shebang-2016]. Diagnostic: `file x.sh` → "with CRLF line terminators".
- **⇒ Dorc implication (the kWINLOCAL persona's first footgun).** Dorc controls the wire (it ships sh to the
  target), so the clean options are **normalize-to-LF on ship** or **detect-and-fail-clear** (DESIGN: fail-fast,
  fail-helpfully). This is *exactly* the Windows-authoring (`kWINLOCAL`) audience's most likely first breakage, so
  handling it well is high-leverage for that persona. Design tensions to surface in the synthesis:
  - vs `kLANG` / what-you-type-is-what-is-run: CRLF→LF is a *benign normalization restoring intent* (the `\r` is an
    editor accident, never semantically meant in POSIX sh) — but it is still a byte-transform Dorc performs. Pick a
    conscious policy: silent-normalize (slick, mild what-you-type bend) vs detect-and-warn/refuse (purest) vs
    `kFIDELITY-faithful` preserves bytes verbatim (and would *reproduce* the breakage — which may be the honest default for `--faithful`).
  - the analyzer parses the AST regardless, so **detection is free** (Dorc can always *see* the `\r` even if it ships verbatim).

## Citations
> [C-unix-se-crlf-shebang-2016]:§Answer+comments (relevance: +1:SURE)
> The problem is the error is happening before bash runs. When you run a script starting with #! then the kernel
> looks at the rest of the line and runs that... So when you have a file in DOS format the kernel will see
> /bin/bash^M and try to run that as the program. Which, of course, doesn't exist.
> [comment, terdon] Bash never sees the file, the error is when trying to find which interpreter to use. Since that
> fails, the script is never given to bash at all.
> [fix] dos2unix foo  /  tr -d '\r' < $1 > ...   ·  diagnostic: file foo -> "with CRLF line terminators".

> [A-git-gitattributes-2024]:§EFFECTS-text (relevance: -0:SUSPECT)
> #### text — This attribute marks the path as a text file, which enables end-of-line conversion: When a matching
> file is added to the index, the file's line endings are normalized to LF in the index... Line endings are
> normalized to LF in the index every time the file is checked in, even if the file was previously added to Git
> with CRLF line endings.
