# notes/143 — round-14 env / toolchain hand-off (ops, not research)

**Stamp:** round 14 · 2026-06-04 · operational context for the next agent. No sources cited (no `validate.sh` slugs).

## Host
- Windows 11 Home, x86-64 (`win32`, 10.0.26200). PowerShell is the primary shell; **msys/git-bash backs the Bash tool**. `mise` reports `windows-x64` (2026.4.28).
- `curl` at `/mingw64/bin`; `sha256sum`, `mktemp` at `/bin`; `jq.exe` at `/c/Users/ec/AppData/Local/mise/installs/jq/1.8.1/`.

## Worktree
- CWD: `C:\Users\ec\Sync\Code\Dorc\.claude\worktrees\bridge-cse_01485LTyERskV7uSD5CajDcZ`, branch `worktree-bridge-cse_01485LTyERskV7uSD5CajDcZ`.
- **Fast-forwarded to `ai/snapshot`/`706f17d` this session** (was `4c1d789`, rounds 8&9). Reversible: `git reset --hard 4c1d789`.
- There are **5 sibling `bridge-cse_*` session worktrees**, all locked, all parked at `4c1d789`. One holds an **in-flight round-13**; this worktree is **round-14** (decades don't collide: 13x vs 14x).
- Main (`ai/snapshot`) carries **uncommitted** human edits to `KNOBS.md` (adds `kCOMMS`, `kAGENTLESS` was committed; the `kOOB` clarification), `DESIGN.md`, `AGENTS.md`, `plans/128`. A plain FF does NOT bring those — read root docs from the **main** path for freshest human intent.

## Toolchain gotcha (load-bearing — sharper than `notes/127`)
The interactive-research scripts (`new-source.sh`/`validate.sh`/`new-turn.sh`) self-relaunch via `mise exec` **only when `jq` is absent from PATH** (see their line ~27). A *fresh worktree's* `mise.toml` is **untrusted**, so that relaunch dies (`mise ERROR ... not trusted`). Fix — no `mise trust`, nothing persistent mutated:
```sh
export PATH="/c/Users/ec/AppData/Local/mise/installs/jq/1.8.1:$PATH"   # real jq → skips the mise relaunch
cd <worktree>
printf '%s' "$json" | sh /c/Users/ec/.claude/skills/interactive-research/scripts/new-source.sh Research <slug>
sh /c/Users/ec/.claude/skills/interactive-research/scripts/validate.sh Research
```
`new-source.sh` curls the URL itself + stamps sha256; heredoc the JSON to dodge apostrophe-quoting. Research-dir = `Research` (project re-uses it; notes are hand-authored `NNN-name.md`, NOT `new-turn.sh` `turnNN` files).

## Git latitude (per `~/.claude/CLAUDE.md` edit, 2026-06-04)
Local *reversible* git on `ai/*` branches and `.claude` worktrees is now **no-prompt**; pushes/remote + non-git-restorable steps stay gated. So committing round-14 work *in this worktree* needs no prompt; merging to `ai/snapshot` is the human's call.
