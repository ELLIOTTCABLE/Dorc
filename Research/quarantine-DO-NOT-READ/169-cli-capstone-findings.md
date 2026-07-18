# 169 — cli capstone: findings from running the whole pipeline on real input

> **Status (2026-06-05): spike, round-16 capstone.** The thin `cli` (`9255b56`)
> wires the network-free pipeline and prints the plan as sh. Running it on a
> synthetic book and the real `pi-webhost.book.sh` fixture surfaced four findings —
> the spike's actual payoff (real input exercises code paths the unit tests don't).
> Append-only note; standalone. Confidence-marked.

## 0. The positive: the slice works end-to-end on real files
`dorc <book.sh> --oracle <oracle.sh> --has <kind:entity>` parses, analyzes, and
prints the plan. The happy path is correct — a synthetic `apt-get install -y nginx`
with `--has package:nginx` renders:
```
#!/bin/sh
# dorc plan (apply phase). Skipped leaves are already converged.

# skip[0]: apt-get install -y nginx
#   ↳ package:nginx already holds (probe: converged · must · ambient)
systemctl reload nginx
```
The full kernel (parse → cfg → classify → plan, with the oracle lifted from sh and
the verdict from a `--has` host model) produces a Terraform-style plan, no network.

## 1. find-cli-1 (HEADLINE) — command-substitution inner commands leak in as plan leaves
On the real fixture the plan begins with two spurious lines rendered as `#!/bin/s`.
Root cause (traced): the fixture has `case "$(hostname)" in …` and `echo "… ($(hostname)) …"`
— **two `$(hostname)` substitutions**. The CFG lowers each `$( … )` body via
`lower_word_substs` → `lower_node` → the inner `hostname` becomes a real
`CfgNodeKind::Command` node. `classify` emits it; `build_plan` renders it as a
run/skip **leaf**. Two compounding errors:
- **leaf-scope (the design problem):** an expansion-internal command (`hostname`
  inside `$(hostname)`) is *not* a standalone executable leaf — it runs during word
  expansion, not as an admin-runnable run/skip unit. The leaf-seam (dn-3) needs a
  precise definition: **top-level, branch-body, AND subshell-`( )`-body commands
  ARE leaves; word-substitution-`$()`-body commands are NOT** (they must still be
  *analyzed* for effects — a `$(rm -rf x)` matters — just not rendered as leaves).
  The CFG currently can't distinguish the two: `lower_word_substs` (word subst) and
  `lower_scoped` (subshell) both emit a bare `CfgNodeKind::ScopeEnter`, so "is this
  command inside a word-substitution?" is unanswerable downstream. A real fix adds
  that distinction (a scope-kind, or a per-Command `is_leaf` flag set during
  lowering), then `build_plan` filters to leaves.
- **span (compounding, already-documented):** `parser::parse_subst_body(inner)`
  re-lexes the substring from offset 0, so the `hostname` node's span is `[0,8)`
  *relative to "hostname"* (8 bytes). `command_text` reads `main_src[0..8)` =
  `"#!/bin/s"` (the shebang minus its `h`). The parser doc-comment already flags
  subst spans as "coarse provenance the spike accepts" — but that coarseness was
  scoped to *diagnostics*; the plan renderer reading them against the main source
  was not anticipated. Fixing leaf-scope (above) sidesteps this; a precise
  subst-span mapping (add the subst's source offset) would fix it independently.

This is the cleanest demonstration yet that **"what is a leaf" is an unsettled part
of dn-3** — and that effect-analysis scope (everything, incl. `$()` internals) and
plan-leaf scope (only admin-runnable commands) are *different* and must be tracked
separately.

## 2. find-cli-2 — the lexer does not strip a leading UTF-8 BOM
The first synthetic demo silently failed to skip: PowerShell's `Set-Content
-Encoding utf8` prepended a UTF-8 BOM (`EF BB BF`), so the first command word lexed
as `\u{FEFF}apt-get` ≠ `apt-get` → no oracle match → Opaque → ran. A book authored
on a Windows editor that adds a BOM would mis-analyze its first command. Low
severity but real for cross-platform (the project is developed on Windows/WSL2);
the lexer should skip a leading BOM. (The demo itself was fixed by writing the file
BOM-free, which then skipped correctly.)

## 3. find-cli-3 — a heredoc command renders without its body
The fixture's `cat > /etc/nginx/sites-available/pi-web.conf <<'EOF' … EOF` renders
in the plan as just `cat > … <<'EOF'` — the `Simple` node's span stops at the
redirection operator, excluding the here-document body. A run-line emitted this way
is broken sh (a heredoc with no body). Same family as find-cli-1: the node span
does not capture the full *executable text* for some constructs, so verbatim
rendering is lossy. The renderer needs spans that cover the whole leaf (incl.
heredoc bodies), or a leaf-text model richer than a single `[lo,hi)`.

## 4. find-cli-4 — precise-errexit still hits ⊤ on the fixture (sound, precision note)
The fixture run emitted `cfg-errexit-unknown: errexit state is ⊤ at one or more
commands; failure-edges added conservatively (over-approximate, sound)` despite the
top-level `set -e` and the note-166 precision work. Some construct on the fixture
(likely the `$(hostname)` substitution regions interacting with the errexit forward
pass, or a scope-restore edge) drives errexit to ⊤. It is sound (over-approximate
→ extra failure-edges, which only cost skips), but worth identifying the exact
trigger to confirm it is not a regression of a note-166 fix. Deferred to a focused
look.

## 5. Next (a focused round, before more surface)
Define the dn-3 **plan-leaf scope** precisely and separate it from effect-analysis
scope: (a) distinguish word-substitution scopes from subshell scopes in the CFG;
(b) `build_plan` emits Steps only for leaves; (c) give leaves spans that cover their
full text (heredoc bodies; offset-corrected subst spans if subst-internal commands
ever need rendering); (d) strip a leading BOM in the lexer. These are all
provenance/leaf-model issues the capstone exposed — none threaten the kernel's
soundness (they are display/scope bugs), but they block a faithful plan render.

**NOTES INDEX:** …166 CFG errexit · 167 effect adversarial review (DP-8/DP-9) · 168
round-16 build summary · 169 (this — cli capstone findings).
