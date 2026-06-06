# notes/141 — executorless live OOB status channel: prior-art gather (round 14, turn 1)

**Stamp:** round 14 · 2026-06-04 · gather turn 1 · 7 primaries read-in-full + graded + registered this turn.
**Charter (from `notes/140`):** executorless (ssh + POSIX `sh` + coreutils only, BusyBox-safe) live OOB status from small-N-parallel probes, ONE reserved channel for Dorc-status, the N probe channels kept *pristine* (full rc + stdout + stderr). Systems crux: how does a script in channel-K (a separate remote process) feed the reserved channel without polluting channel-K?

## Findings (most-attended first)
- g1. **The reserved-status-lane pattern is independently reinvented across three domains** — apt (`APT::Status-Fd`), bats (fd 3), pdsh (a *second connection* for stderr) — each keeps the human/tool streams pristine and puts machine-structured status on a SEPARATE lane. Strong convergence on Dorc's "channel-10". [A-debian-apt-progress-reporting-2022] [A-bats-core-writing-tests-2024] [A-pdsh-readme-2024]
- g2. **FIFO multi-writer→single-reader is the executorless live mux.** Writes `< PIPE_BUF` (≥512 POSIX, 65536-byte pipe capacity on Linux, atomic sub-buffer 4096) are atomic — concurrent probe writers append short status lines to one `mkfifo` without tearing, drained live by one `cat`/reserved channel. `mkfifo`+`cat` are POSIX/coreutils, BusyBox-present. The prime `arch-live-oob-fifo` mechanism. [A-man7-pipe-2024]
- g3. **The hard constraint (f9 confirmed authoritatively): fds are a LOCAL OS concept; only BYTES transit ssh.** A 4th lane must be created remote-side (FIFO/file/2nd-channel/2nd-connection); a local `3<` is unreachable from the remote shell. Kills any "forward an extra fd" shortcut. [B-unix-se-ssh-fds-not-transportable-2025]
- g4. **pty MANGLES streams (f5 confirmed by an expert).** A remote `ssh -tt` tty merges + cooks stdout/stderr/prompts ("All the streams will be processed by the remote tty and this will mangle data"). The clean per-probe split *requires* no-pty (→ the block-buffering tradeoff from `notes/140` f5 stands). [B-kamil-ssh-separate-streams-tty-2021]
- g5. **The reserved-lane EOF hazard (load-bearing, new).** A backgrounded child INHERITS the reserved fd; the reader BLOCKS until ALL holders close it (bats: "read this if Bats hangs"). Dorc's FIFO/channel-10 drain will hang on EOF if any probe backgrounds a child holding the write-end. Mitigation: probes must close/redirect the reserved fd before backgrounding, or the drain must key off an explicit end-sentinel, not FIFO-EOF. [A-bats-core-writing-tests-2024]
- g6. **Even the expert executorless multi-lane solution is contorted.** Kamil's `sshe` provides separate stdin/stdout/stderr PLUS a tty, client-side, *no server companion*, by running TWO ssh connections (ControlMaster master `-M -T` + slave `-t`, one auth) and wiring `/proc/$pid/fd/N`. Takeaway: ControlMaster (many channels, one auth) is the executorless mux primitive; clean extra lanes are achievable but cost a 2nd channel + non-portable `/proc` tricks. [B-kamil-ssh-separate-streams-tty-2021]
- g7. **debconf = a pure-sh structured line-protocol over the standard streams.** Config script writes a command to stdout, reads a numeric-result-code reply from stdin (SMTP-like); the `confmodule` swaps the real stdout so the protocol owns fd 1/0. AND debconf's `config`(examine-state, **no fs mutation**) / `postinst`(act) split mirrors Dorc probe/apply — independent confirmation of the phase split. [A-debian-debconf-devel-protocol-2014]
- g8. **apt gives the concrete line format.** `pmstatus:pkgname:TotalPercentage:action-description` — colon-delimited, one record per line, distinct leading keywords (`pmstatus`/`dlstatus`/`pmerror`/`media-change`); l10n done once at the producer so every front-end gets translated strings. A ready template for Dorc's status records. [A-debian-apt-progress-reporting-2022]

## Emerging executorless design (what the evidence supports) + open Qs
The convergent answer: **N probe channels stay pristine (the channel boundary IS the per-probe demux, g1/pdsh); each probe `printf`s short, nonce-prefixed, colon-delimited status records (g8) to a remote `mkfifo` (g2); one reserved `exec` channel runs `cat $fifo` and streams them live to the controller.** No-pty throughout (g4); the reserved lane is fed remote-side because fds don't transit (g3); the EOF-inheritance hazard (g5) forces either fd-hygiene-before-backgrounding or an explicit end-sentinel rather than EOF-detection; ControlMaster (g6) multiplexes the N+1 channels over one auth/hop (`notes/140` f4). The executor (`kCOMMS-executor-OOB`) remains justified ONLY where this degrades — see open Qs.
- oq1. **BusyBox/stripped-target gaps:** is `mkfifo` always present? `cat` yes; `tail -f` is the file-based alternative but `-f` polling latency + BusyBox variance. Need a corpus check.
- oq2. **Backpressure:** if the controller's reader stalls, a full FIFO (64 KiB) blocks the *probe's* `printf` → couples probe progress to controller liveness. File+`tail -f` decouples (writer never blocks) at the cost of an on-disk artifact + polling latency. The FIFO-vs-file tradeoff is the real fork.
- oq3. **Binary-safety:** status records are Dorc-controlled (safe), but if any probe's freeform leaks onto the reserved lane it can tear/forge — the nonce-prefix (`notes/140` f-sec) + keeping freeform strictly on the pristine channels handles it.
- oq4. **2nd-channel-tail vs FIFO vs file:** three executorless variants; pick per oq1/oq2. (mitogen — already in `sources.json` — is the executor counter-example; its 400-byte/8KB minimal bootstrap is the fallback if executorless loses.)
- oq5. Still-to-read for breadth: ClusterShell tree-mode grooming/aggregation (fan-in at scale), `mcp-ssh-live` SPEC (channel-per-job ring-buffer + `tail(since_line)` near-live + the block-buffering gotcha), tmux control-mode (`%output`/`%begin`/`%end` framing), gnupg `--status-fd`.

## Citations
> [A-man7-pipe-2024]:§PIPE_BUF + §"I/O on pipes and FIFOs" (relevance: +1:SURE)
> PIPE_BUF POSIX.1 says that writes of less than PIPE_BUF bytes must be atomic: the output data is written to the pipe as a contiguous sequence. Writes of more than PIPE_BUF bytes may be nonatomic: the kernel may interleave the data with data written by other processes.
> The communication channel provided by a pipe is a byte stream: there is no concept of message boundaries.
> If a process attempts to read from an empty pipe, then read(2) will block until data is available. ... Since Linux 2.6.11, the pipe capacity is 16 pages (i.e., 65,536 bytes ...).

> [A-debian-apt-progress-reporting-2022]:§"Install-progress reporting" + §pmstatus (relevance: +1:SURE)
> If the apt options `APT::Status-Fd` is set, apt will send status reports to that fd. The status information is separated with a ':' ...
> The reason for using a fd instead of an OpProgress class is that many apt front-end fork a (vte) terminal for the actual installation.
> pmstatus:pkgname:TotalPercentage:action-description ... pmstatus:3dchess:40:Unpacking 3dchess

> [B-unix-se-ssh-fds-not-transportable-2025]:accepted answer (Marcus Müller) (relevance: +1:SURE)
> There's a network connection between you and the remote host. How is the shell on somehost.com ... supposed to have access to file descriptors on your local machine? It can't! They are a local operating system concept, not transportable through a SSH stream. The only thing that is: bytes.
> If you want to use a file descriptor handle in bash on the remote host, you need to create it on the remote host, not locally.

> [A-pdsh-readme-2024]:§GOTCHAS (1) (relevance: +1:SURE)
> When executing remote commands via rsh, krb4, qsh, or ssh, pdsh uses one reserved socket for each active connection, two if it is maintaining a separate connection for stderr. It obtains these sockets by calling rresvport(), which normally draws from a pool of 256 sockets. You may exhaust these if multiple pdsh's are running simultanously on a machine, or if the fanout is set too high.

> [B-kamil-ssh-separate-streams-tty-2021]:question body + answer script (relevance: +1:SURE)
> ssh -tt will make sudo and tool read from the remote tty and write output and errors and prompts to the remote tty. Not only I won't be able to tell the output and the errors/prompts apart locally. All the streams will be processed by the remote tty and this will mangle data.
> If it's possible, I prefer a client-side solution that does not require any server-side companion program.
> ssh -M -S "$sock" "$@" -T "$host" '</dev/null echo "$$"; exec sleep 2147483647' ... [slave] exec 6</proc/$rpid/fd/0 7>/proc/$rpid/fd/1 8>/proc/$rpid/fd/2 9>/dev/tty

> [A-bats-core-writing-tests-2024]:§"File descriptor 3 (read this if Bats hangs)" (relevance: +1:SURE)
> Bats makes a separation between output from the code under test and output that forms the TAP stream (which is produced by Bats internals). This is done in order to produce TAP-compliant output. In the Printing to the terminal section, there are details on how to use file descriptor 3 to print custom text properly.
> A side effect of using file descriptor 3 is that, under some circumstances, it can cause Bats to block and execution to seem dead without reason. This can happen if a child process is spawned in the background from a test. In this case, the child process will inherit file descriptor 3. Bats, as the parent process, will wait for the file descriptor to be closed ...

> [A-debian-debconf-devel-protocol-2014]:§"THE DEBCONF PROTOCOL" + §"THE CONFIG SCRIPT" (relevance: +1:SURE)
> Config scripts communicate with debconf using the debconf protocol. This is a simple line-oriented protocol, similar to common internet protocols such as SMTP. The config script sends debconf a command by writing the command to standard output. Then it can read debconf's reply from standard input. Debconf's reply can be broken down into two parts: A numeric result code ...
> The config script should not need to modify the filesystem at all. It just examines the state of the system, and asks questions, and debconf stores the answers to be acted on later by the postinst script.
