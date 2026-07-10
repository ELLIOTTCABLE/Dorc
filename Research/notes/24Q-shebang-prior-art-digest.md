# 24Q — shebang-embedding prior art: research digest + the dorc-sh adjudication inputs

AI-authored (Fable conductor), 2026-07-10. Digest of a single Opus research agent's web/gh
gathering pass (30 tool-uses; brief: prior art + gotchas for "a wrapper-interpreter shebang that
routes a script through a dorc subcommand, executor-shell as an argument"). Honesty header: the
findings below are SUBAGENT-GATHERED; the conductor adjudicated the report's internal coherence
and its convergence with priors but did NOT independently re-fetch the sources this round. The
adjudicated DECISIONS live in `24P` §9; this note is the evidence base. The decision deliberately
rests only on claims that are multi-source-convergent, and NOT on the ~SUSPECT cells (§6).

## §1 — Graded sources (agent's grading; A = primary read-in-full)

[A-mascheck-shebang-2021] in-ulm.de/~mascheck/various/shebang/ — THE canonical `#!` reference ·
[A-linux-binfmt-script-2026] fs/binfmt_script.c @ master · [A-linux-exec-c-2026] fs/exec.c ·
[A-gnu-env-manual-2025] coreutils env invocation · [A-freebsd-env-man-2024] FreeBSD env(1) ·
[A-nix-shell-manual-2025] nix.dev 2.25 nix-shell shebang section · [A-stack-scripts-2023]
haskellstack script docs · [A-pep723-2023] peps.python.org/pep-0723 · [A-rfc3424-cargo-script-2023]
+ [B-rfc3502-frontmatter-2023] cargo-script/frontmatter RFCs · [A-execlineb-doc-2025]
skarnet execlineb · [A-tclsh-man-2023] tcl8.6 tclsh man · [A-perlrun-2025] perldoc perlrun ·
[A-busybox-env-src-2026] busybox coreutils/env.c · [A-sbang-readme-2020] spack/sbang README ·
[A-shellcheck-directive-2024] shellcheck Directive wiki · [B-lwn-shebang-fix-2019]
lwn.net/Articles/779997 (the 5.0 truncation regression) · [B-gilles-macos-envS-2024]
unix.SE 774127 (macOS env -S, Gilles) · [B-kernel-6eb3c3d-2019] commit 6eb3c3d0a52d
(BINPRM_BUF_SIZE→256, 5.1) · plus B/C-tier: guix manual, coreutils ML, scala SIP-46, deno docs,
nodejs#49432 (busybox), guix#9156 (patch-shebangs), virtualenv#596, stack#5326, shellcheck#1210,
rust-script.org, wikipedia/unix.SE dating, uv-shebang blogs.

## §2 — Kernel/OS mechanics (the "more complex than assumed" inventory, confirmed)

- **One-argument rule:** Linux + modern BSDs deliver everything after the interpreter path as ONE
  argv entry; macOS (xnu 792+) SPLITS into words; old SVR4/Solaris lineage delivers ONLY the
  first arg, silently dropping the rest; 4.0BSD delivered none. [A-mascheck-shebang-2021 test
  table; A-linux-binfmt-script-2026 source: one `i_name` + at most one `i_arg`.]
- **Length limits, tri-modal overflow:** Linux 127 bytes → 256 since 5.1 [B-kernel-6eb3c3d-2019].
  Current Linux: truncated interpreter PATH ⇒ ENOEXEC, but truncated ARGUMENTS still silently
  truncate (kernel comment: "Truncating the arguments is fine: the interpreter can re-read the
  script") [A-linux-binfmt-script-2026]. The 5.0 hard-fail attempt broke NixOS (whose Perl
  RELIED on truncation + Perl's own shebang re-read) and was reverted [B-lwn-shebang-fix-2019].
  macOS over-length: silently runs the script under the shell instead [A-sbang-readme-2020].
- **Interpreter-as-script:** rejected by ~all Bell/Berkeley-derived kernels; Linux ≥2.6.27.9
  allows it, capped "5 levels of binfmt rewrites before failing hard" (ELOOP) [A-linux-exec-c-2026].
  ENOEXEC fallout is shell-dependent (some shells silently self-interpret — fake portability).
- **No PATH search; `#!$SHELL` inexpressible; blanks forbidden in interpreter name; script path
  passed as-typed (relative stays relative); argv[0] conventions vary per-OS; CRLF glues `\r`
  onto the last token; interpreter-ENOENT is misreported by many shells as script-not-found.**
  [A-mascheck-shebang-2021]
- **POSIX:** `#!` line-1 behavior is formally UNSPECIFIED ("A portable application cannot use #!
  as the first line of a shell script") — universal by convention, not contract. And `sh file`,
  `. file`, stdin invocation all bypass the shebang entirely.
- **setuid:** ignored for `#!` scripts on Linux (classic race); /dev/fd handoff on some others.

## §3 — `env -S` (the standard one-arg workaround, and its holes)

FreeBSD 6.0 (2005, invented WHEN FreeBSD switched to one-arg); GNU coreutils 8.30 (2018) →
Debian 10+/RHEL 8+/Ubuntu 19.04+. Splits on unquoted whitespace, quotes/escapes, `${VAR}`
expansion, `#` starts a comment INSIDE the string. Holes: busybox env has NO -S at all (+SURE,
source-read) [A-busybox-env-src-2026, B-nodejs env issue]; pre-8.30 coreutils errors; macOS
ships -S but XNU pre-splits shebang words so quoted -S strings mis-parse [B-gilles-macos-envS-2024];
shebang-rewriting tools (guix patch-shebangs) don't understand it.

## §4 — Prior-art taxonomy (how the ecosystem smuggles config past one-arg)

- **taxonomy-envS** — `#!/usr/bin/env -S tool args…` (deno, scala-cli, guix shell, uv). Full
  in-line bandwidth; costs the §3 portability matrix.
- **taxonomy-secondline** — wrapper re-reads continuation pseudo-shebang lines from the file
  body: nix-shell (`#! nix-shell -i bash -p …`, explicitly BECAUSE "many operating systems only
  allow one argument in #! lines"; relocatable into block comments), stack (`-- stack script …`),
  sbang/long-shebang (a literal second shebang; sbang's carrier spelling
  `#!/bin/sh /path/to/sbang` also solves interpreter-must-be-binary). Ecosystem cost: linters/
  tools need teaching (shellcheck#1210).
- **taxonomy-metadata-block** — structured inline metadata; shebang carries only the runner:
  PEP 723 `# /// script` TOML (uv/pipx/hatch), cargo `---` frontmatter (RFC 3502/3503 superseded
  the fenced-doc-comment eRFC), rust-script, scala-cli `//> using`.
- **taxonomy-polyglot** — headers valid in BOTH sh and the target: tclsh's
  `#!/bin/sh` + `# next line restarts using tclsh \` + `exec tclsh "$0" ${1+"$@"}`; Perl's
  `eval 'exec perl -wS $0 ${1+"$@"}' if 0;`. Zero dependencies, works on no-`#!` systems,
  preserves $0/$@ explicitly, and is the ONLY family that keeps running when the wrapper is
  absent. Cost: per-language incantation fragility + top-of-file ceremony.
- **taxonomy-chainload** — djb discipline: `#!/command/execlineb -P` — short absolute path, one
  clustered flag, launcher execs into the final argv and never stays resident.
- (Adjacent: **dialect-declaration directives** — `# shellcheck shell=sh`, modelines, PEP 263 —
  config channels consuming zero shebang bandwidth, read by tools not the kernel.)

Executor-naming conventions observed: flag (`nix-shell -i bash`), positional-after-`--`
(`guix shell pkgs -- python3`), embedded-full-command (stack), dedicated subcommand minted FOR
shebang position (`scala-cli shebang` — its arg handling differs from `run` precisely for this),
literal second shebang (sbang). Perl's own historic behaviors are wrapper-prior-art too: it
re-reads its own shebang to recover truncated flags, and re-execs the named program when the
`#!` line lacks the word "perl".

## §5 — Ranked gotchas for a naive `#!/usr/bin/env dorc-strip dash -eu`

1. One-arg rule: env receives the single string `dorc-strip dash -eu` → ENOENT. 2. First-arg-only
kernels silently drop the flags. 3. env -S fixes it only where env -S exists (busybox: no;
<8.30: no; macOS: quoting hazard). 4. Same line yields DIFFERENT argv per OS (macOS splits).
5. Length budget 127/255 with silent argument-truncation. 6. A script-implemented wrapper is
refused as interpreter off-Linux (escape: the sbang `#!/bin/sh /abs/path` carrier, at the cost of
a hardcoded path). 7. PATH never searched. 8. POSIX-unspecified + `sh file` bypasses it anyway.
9. Misleading ENOENT attribution on exactly the dorc-less boxes the off-ramp targets.
10. CRLF/`#`-in-args/argv[0] trivia.

## §6 — Agent-flagged open threads (unverified; deliberately NOT load-bearing)

~SUSPECT unquoted `env -S a b c` recomposes harmlessly on macOS (quoted forms demonstrably
mis-parse). ~SUSPECT busybox env hard-errors on -S (vs ignoring). -GUESS macOS env -S first-ship
date. --WONDER whether modern illumos still delivers first-arg-only (Mascheck's data is SunOS
5.x-era). Not obtained: spelling frequency counts (gh legacy code-search strips `#!/`
punctuation); dedicated filename-argument-injection literature beyond the setuid-era `-i` attack.

## §7 — Pointer

The adjudicated Dorc decisions (the `dorc-sh` single-token spelling, the `--` executor contract,
strip-never-touches-the-shebang, shebang-iff-marker) live in **`24P` §9**, alongside the specimen
normalization they were applied to.
