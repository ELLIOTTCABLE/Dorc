# 256 — ~/System recon distillation: windmill book-firming + P6 scope

AI-authored (Fable conductor), 2026-07-04, distilled from an Opus scout's read-only recon of the
human's system-repo. **Project-focused: design conclusions for the trial, NOT a catalogue of the
human's personal setup.** (The scout's fuller raw output sits at commit `6abfd3d`; squash on the next
`ai/` rebase.) Secret-discipline held: his repo's credentials are all `env`/`op://` indirections (no
literal secret), and the shell history-file was deliberately not read.

## Windmill — firms the 255 book's FLAGs + names its deliberate divergences

His real windmill runs as a **published container image** on **external/managed postgres** behind a
**non-nginx proxy**, server/workers split — i.e. NOT the book's shape. Against the book's guesses:

- **CONFIRMED:** the server binds **`:8000`**; **`DATABASE_URL`** is the verbatim env var.
- **Likely book gap:** the server wants **`BASE_URL`** (its public URL); the book's `windmill.service`
  omits it — ~SUSPECT it's required to actually serve. Add on the day / in the §8 web pass.
- **Owed, don't assume-confirmed:** `MODE=standalone` + single-process + explicit `PORT` is the
  *upstream combined* mode (real per docs, not his split practice); the **bare-binary path +
  release-asset name have no grounding** (he ships a container) — dec-2's realism-FLAG is validated
  by-absence, compose is the fallback, the asset name is owed to the §8 web pass.
- **State these as deliberate exercise-Dorc divergences, so the day isn't misread as "unrealistic":**
  the book's **on-box apt-postgres** (dec-3: a real drift-able adequacy substrate), **`su - postgres`**
  (dec-4: the opaque-wrapper wall), and **nginx** (base-stdlib elision) are *intentional* — his real
  stack has none of them. Chosen to exercise Dorc, not to mirror his habits.

## P6 (HHHF) — feasible, and cheaper than 252 assumed for the command channel

- **Command capture is cheap + idiomatic:** his shell already runs stacked `preexec` hooks, so the
  HHHF registers its own `preexec`/`precmd` **additively** — a JSONL sidecar
  (`{cmd, expanded, cwd, prev-rc, epoch}`) is the authoritative transcript spine.
- **History is a lossy secondary** (space-prefixed commands dropped; dup-expiry) — demote the
  history-diff idea to a redundancy rail; the preexec stream is primary.
- **Friction-button** = a ZLE widget on a deliberately-chosen free chord (his keymap is dense).
- **asciinema** = audit rail only, and an **ANSI-stripping extractor is mandatory** (his plugin stack
  makes raw recordings ANSI-dense per keystroke).
- **Shape:** preexec-JSONL spine + ZLE friction-button + asciinema-with-extractor — a few dozen
  throwaway lines. **Fidelity gate: a real interactive PTY** — `zsh -c` false-greens the recorder
  (structurally confirms A3's "real-PTY or a human 5-min smoke" lean; `tmux send-keys` drivers are
  fiddly under vi-mode).

**⚠ open fork (owed, downstream):** the scout's "friendly PTY" path assumed the macOS controller (his
shell env present) — but the substrate REVERSED to Windows (`§5.2`). Decide where P6's real-PTY
session lives: his local WSL/git-bash shell, or a shell on the Debian box (vanilla — his me-shaped
config isn't there). Resolve when P6 is dispatched.

**Still owed (facts about his practice, not recon gaps):** the native-binary windmill specifics
(asset / `MODE` / `PORT` / run-user) — only the §8 web-doc pass can source them.
