# H2SaLS corpus — real-world conformance target (round-1A)

> **LLM-GENERATED TESTING CORPUS — NOT REAL SECURITY CODE.** Everything in this
> directory is AI-authored (round-1A, 2026-06; brief: `Research/notes/1A1`). Quality
> and defensiveness are *intentionally varied* to mimic multiple human authors; this is
> a testing corpus for Dorc's analyzer, an at-least-partly-artificial rendition that
> cannot expose the truth of real-world ops-code, and must never be used to actually
> secure a server.

⚠️ **FROZEN EVIDENCE — NEVER EXECUTE ANYTHING IN THIS DIRECTORY.** ⚠️

`harden.sh` (and every `*.sh` here) reconfigures SSH, firewalls, PAM, sysctl-adjacent
state, fail2ban, and unattended-upgrades. Executing it — even partially, even a
"harmless-looking" line, under any flag or fragment — on a development machine could
firewall it or lock its owner out. It is a *static-analysis target*, frozen from birth.
Validation is `dash -n` (parse-only) plus reading. No exceptions, no `sh -x`, no
"just this one function", no live `PATH`. (This rule has been violated by an agent on
this machine before. It must not happen again.)

## What this is

A faithful, plain-POSIX-sh rendition of a brutally-representative ops runbook, written
the way experienced human admins write idiomatic shell — deliberately NOT shaped to
what Dorc's analyzer currently models, and deliberately NOT annotated for Dorc. It is
the standing real-world test target: the conformance-gate measuring the gap between
real admin shell and the modeled subset. See `Research/notes/1A1` for the round plan
and `Research/plans/1A*` for the capability matrix it feeds.

Written by multiple AI authors with deliberately varied defensiveness (≈0.0–0.5 band)
to mirror a multi-author human runbook (rul-1A-defensiveness-range, 1A1 §1).

## Provenance (re-fetchable, pinned)

- **Ground truth for ordering/conditionals** — the Ansible rewrite (the human's fork):
  `https://github.com/ELLIOTTCABLE/How-To-Secure-A-Linux-Server-With-Ansible.git`
  at commit `34975f13406ec6541ee3c3a6499c0af1041e402d` (the `Vendor/HTSALSWA` submodule
  pin in the human's System repo).
- **Rationale/commentary** — "How To Secure A Linux Server", imthenachoman, pinned raw:
  `https://raw.githubusercontent.com/imthenachoman/How-To-Secure-A-Linux-Server/5abb8c77cf0bded508ca3a31a3de579563db891f/README.md`
  sha256 `0b1100dc972029cb837d9158cac3459fe39476b3300c16a73155f2fb5e99bd7a`
  (graded source `B-imthenachoman-h2sals-2026` in `Research/sources.json`).
- Section-header comments in `harden.sh` map each section to its play file and guide
  anchor/line-range at those pins.

## Contents

- `harden.sh` — the book (D1). One file, sections in the play's role order; the
  requirements-play is section 1 (run-as-root dissolves the original two-play split).
- `census/` — D2 mechanical command/construct census output over `harden.sh`
  (the tool itself is durable, in `Research/corpora/tools/`).
- `oracles/` — D3 oracle seeds (defensiveness ≈0.5–1.0 band), in the spike's oracle
  idiom. Also frozen: `dash -n` only.

Placeholders (`USERNAME_HERE`, `PASSWORD_HERE`, …) are intentional and must stay
placeholders — never substitute real values into this corpus.
