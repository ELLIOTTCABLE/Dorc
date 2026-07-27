# r26 live-smoke kit

A small Debian-12 runbook plus hand-written v0.2 oracles, built to be executed against the
throwaway VPS in `Research/notes/26E-live-target.md` — the first time this project's plan/apply pipe
meets a real machine. Everything here has been validated **hermetically only**: `dorc lint`,
`dorc strip`, and `dorc plan` fed hand-authored records. No part of it has been run anywhere, by
anyone, and no agent may run it.

## §1. What is in here

| path | what it is |
|---|---|
| `smoke-book.sh` | the book: 11 tool-lines, `#!/bin/sh` + `set -eu`, POSIX, LF-only, root, no sudo, no host guard |
| `r26-smoke.conf` | the nginx snippet the book drops (port 8088, so it cannot collide with the default site) |
| `r26-motd` | the second config drop |
| `oracles/*.oracle.sh` | five narrow oracles: `dpkg`, `apt-get`, `cp`, `systemctl`, `logger` |
| `records/*.txt` | hand-authored probe records for two worlds, plus their framed forms |
| `renders/*.txt` | the three hermetic `dorc plan` outputs, for the morning to compare against reality |
| `predictions.md` | the pre-registered ledger, and an honest account of where it was wrong |
| `frame-records.sh` | turns an authored records file into the stream `dorc plan` actually admits (§4) |

The book deliberately spans the disposition spectrum instead of maximising elision: a bare
`apt-get update` that cannot elide, four hand-guarded installs, two `cp` config drops, `systemctl
enable` + `start`, and two honest walls at the tail (`curl`, with no oracle at all, and `logger`,
with an oracle that deliberately declines).

## §2. The intended live sequence

The box is root-login Debian 12 at the address in `26E` §2; the `-F ssh_config` is load-bearing on
the Windows controller. `FIXME-cli-spelling` marks every place the executor lane's final flag names
are still pending — dorc cannot yet ship its own probe or apply over ssh, which is precisely what
r26 is building.

```sh
cd Research/trial/r26

# 1. plan-time only, no host: confirm the kit still lints clean
mise run dorc -- lint smoke-book.sh oracles/*.oracle.sh

# 2. render the probe artifact                     FIXME-cli-spelling: no host argument exists yet
mise run dorc -- probe --book=smoke-book.sh --oracle-dir oracles > probe.sh

# 3. ship + run the probe on the box, capture its records
#    Today this is manual; r26's executor lane is what replaces it.
scp -F ../apply/ssh_config -i ~/.ssh/dorc-r25 probe.sh r26-smoke.conf r26-motd root@<ip>:/root/
ssh -F ../apply/ssh_config -i ~/.ssh/dorc-r25 -T root@<ip> 'cd /root && sh probe.sh' > live-records.txt

# 4. build the apply artifact from the REAL records  FIXME-cli-spelling: --results may become --records
mise run dorc -- plan  --book=smoke-book.sh --oracle-dir oracles --results live-records.txt
mise run dorc -- apply --book=smoke-book.sh --oracle-dir oracles --results live-records.txt > apply.sh

# 5. run it on the box (cwd must hold r26-smoke.conf and r26-motd — the cp sources are relative)
ssh -F ../apply/ssh_config -i ~/.ssh/dorc-r25 -T root@<ip> 'cd /root && sh -s' < apply.sh

# 6. THE POINT: repeat 3-5. The second pass is the converged-world measurement.
```

Step 6 is the whole experiment. Run 1 provisions; run 2 is the one whose plan should differ, and
`predictions.md` is what it should be compared against. A third pass with the `apt-get update` line
deleted measures the ceiling (`renders/plan-ceiling-converged.txt` is that shape, hermetically).

`Research/trial/apply/apply-run.sh` already wraps steps 3 and 5 with rc/timeout capture and a run
directory, and needs no arguments beyond the host — prefer it over raw ssh.

## §3. Reproducing the hermetic renders

```sh
DORC=<path-to-dorc> ./frame-records.sh smoke-book.sh oracles records/converged.txt \
   > records/converged.framed.txt
dorc plan --book=smoke-book.sh --oracle-dir oracles --results records/converged.framed.txt
```

Site ids are positional and shift whenever the book gains or loses a line, so re-derive them from
`dorc probe` (the `# site N: <coord>` comments) before editing any records file.

## §4. Sharp edges found while building this — read before editing the oracles

Four engine behaviours cost real time here. The first three are **silent**: the file still parses,
`dorc lint` still reports `0 errors`, and the annotations are simply inert.

1. **Three constructs void every mark in a marked file.** A bracket test in statement position
   (`[ x ] || return 2`), a glob pattern in a `case` arm (`-*)`, `?*)` — a bare `*)` is fine), and a
   backslash continuation whose next line starts with a redirection. The tell is `dorc strip`
   leaving the marks in place; the only surfaced symptom is an incidental shellcheck `SC2154`
   ("referenced but not assigned") on a bound variable. **Check any edit with
   `dorc strip <file>` and confirm no `r26.smoke.` survives outside comments.**
2. **Any unmodeled statement before the marked line makes the site unresolvable**, so no probe ships
   and nothing can elide. `case`, assignment and `shift` are modeled; `test`, `command -v`, and
   `if <command>; then` are not. Defensive gates must be spelled `if [ "${n-}" ... ]; then return 2; fi`
   with string comparisons only — file tests (`-f`, `-e`, `-d`) void the marks per (1).
   Consequence: the contract's recommended `command -v tool >/dev/null 2>&1 || return 2` existence
   gate cannot be used. Dropping it is safe — a missing delegate exits 127, which the rc partition
   already reads as cannot-say — but it is a real gap between the documented idiom and the engine.
3. **A verdict body may not inspect `$?`.** `case $? in …` makes the site unresolvable, which
   forbids the exit-vocabulary translation the oracle contract asks for. This is why
   `systemctl enable --now` is declined outright rather than modeled: it establishes two cells,
   one status can witness one cell, and the conjunction cannot be spelled. It is also why the
   systemctl oracle leaves systemd's 3-means-inactive / 4-means-no-such-unit untranslated (safe:
   everything ≥2 runs; imprecise: a stopped unit reads "cannot say" rather than "diverged").
4. **`dorc plan --results` does not accept the grammar `dorc --help` documents.** Bare
   `site N effect=… rc=…` lines are refused with `host-evidence-admission-refused` (exit 12, and the
   diagnostic prose is an unwritten loom placeholder). Records must carry the run nonce and terminal
   token under a matching `book=` digest header — the framing `cli/tests/e2e.rs` does internally.
   `frame-records.sh` is a port of that. Committed `probe-results.txt` fixtures look feedable and
   are not. Also: a trailing `#` comment on a record line parses as a record field and refuses the
   whole stream — comments must stand on their own line.

Two further findings are about what the engine *decides*, not what it parses, and both are written
up with evidence in `predictions.md` §4: `is_converged` sites of the same command collide onto one
synthesized `dorc-auto:<cmd>@converged` cell so only one can be licensed (m-2), and only a guard
above the first mutator site can fold, which caps the `dpkg -s x || apt-get install x` idiom to the
first line of any run (m-3).

## §5. Standing cautions

- The `cp` sources are relative; run the book with this directory as cwd or both drops resolve to
  nothing.
- The kit installs `nginx`, `curl`, `jq`, `ca-certificates` — light enough for the 2 GB box.
- The oracles mint throwaway `r26.smoke.*` kinds. They are not stdlib and must never be treated as
  a naming precedent; `sm.dorc.*` is reserved.
- Teardown of the box is human-acked only (`26E` §5).
