## Architecture-planning / footgun-uncovering-research-phase

- [.] (PARTIAL) deep security-dive on both the 1. language/analysis and 2. orchestrator architectures,
  - specifically flag 'including seccomp in the core'; LLM claims it's secure-by-design, but I very much do not trust that claim without a specific threat-model and analysis
- [ ] dig into control-flow hazards that will easily pass parsing (`set -e`, `trap`.) may be mined for contract-sh-spellings?
- [ ] prior-art on linking oracles' binaries-to-be-invoked to *hash*, spelled-in-sh (`if [ "$(shasum thebin)" = "abcdef..." ]; then ...`)
- [x] (DEFERRED) skim shell-script corpii to validate design

## Preparation-for-agentic-implementation 🤢 phase

- [ ] how to TDD: how do we *architect* a network-scale appliance? even miniaturely? is there any better option than "I keep a proxmox host up at all times to dynamically Become A Network Of Fake Hosts, then execute tests against that?" I need something *fast* and I just don't know how to structure. (also, sandboxed, for agents.)
- [ ] hunt for *implementation* details that *can* be decided/implemented rather upfront (like TDD, but besides TDD) - data-structures and algorithms that are *guaranteed* to be relevant to the project, and can be *seeded* as initial, black-box modules/code (or picked as upstream libraries.)
- [ ] major refactor of the planning: re-split, first, along *what-needs-to-be-known-per-component* lines, not *what did I want to research* lines (i.e. there's findings relevant to "the analyzer" all over the research-base; they need to be collated and extracted.)
  - I do not trust a single agent on this; I think probably a pass to decide *which* components; then a pass per-component to extratct-and-pointer-back; then finally a pass to break into implementation-phases (spike/soon/need/defer?)
- [ ] agentic meta-pass on "how best to structure this codebase so agents can work on it", some nightmare-slop bullshit.
  - most importantly I'm guessing deterministic triggers per-component, if possible. point agents entering certain dirs/components to authoritative design-documents for that component, so when they 'cross boundaries' to chase a fix/feature, they hit Correct Context to make it intelligently?

## Small implementation details

- [ ] probe-bodies *also* need to be proved non-mutable w/ effect analysis

---

*(below: agent-dropped during the kill-criteria session, 2026-06-01 — for human review/integration)*

*(machine-added 2026-06-03, security / threat-model round — for human review/integration; full context: `Research/plans/102` "Cross-cutting · Version drift" + `notes/100` (b+))*

- [ ] **version & binary-identity as oracle-contract-menu items (the parked versioning spike).** An oracle's grounding (e.g. "`docker --dry-run` is inert") is silently parameterised by the *exact binary* it was authored against; version-drift — or even a *same-version* distro-backport — breaks both the read-only and oracle-soundness claims without announcing it. Candidates to lift into the oracle-contract menu:
  - **binary-contents-hash guard** — oracle binds its grounding to a content-hash; runner hash-checks `$PATH` at probe/apply, fail-safe on mismatch. Jointly defends version-drift *and* injection; lightweight — *record-and-compare*, **no registry** (only Nix-style content-addressed *storage* would be "a registry").
  - **version coordinate** — cf. [package-url/purl-spec](https://github.com/package-url/purl-spec) as a prior-art identity format encoding namespace/distro/version (`pkg:apk/alpine/musl@1.2.5-r9?distro=3.21`), which distinguishes "same version string, different bytes" across distros/patch-levels. Distinct job from the hash: purl = comparable *intent*/coordinate; hash = *identity* for the soundness gate.

*(machine-added 2026-06-08, round-17 adversarial-crosscheck — for human review/integration; full context: `Research/notes/17O` R2-CHANGEDELTA + `notes/17x-strawmen/adversarial/`)*

- [ ] **run-delta convergence: the un-probeable, change-gated effect class — to write into DESIGN.** A class of effects is idempotent on a *run-delta* ("did A change *this run*"), not on host *state*: restart-on-change, `systemctl reload` after a config write, the whole Ansible `notify`/handler pattern. Their convergence ("the daemon is running the *current* config") is generally **not host-state-probeable** — `kVOLATILES` even forbids caching the freshness signal. So the probe optimizer can elide such an effect *only* via (a) the **author's own change-flag** data-flow, which Dorc must **preserve, not elide** (it is a *consumed observable* — needs `q1-precision` to track the variable across the cp→reload edge), or (b) a safe **over-execute** (run it every bump; priority-2). Dorc must **never** elide it via a *state*-probe (mapping `reload`→`is-active` = a priority-1 wrong-skip), and must **not** *synthesise* the cross-kind causal edge (`MUST-grade-to-correlate` correctly refuses). Sharpens `Research/notes/17B`'s cert→reload from "cross-facet" to "this whole class." Boundary to state in DESIGN: *the probe phase elides a delta-gated effect only by preserving the author's change-flag, else it over-executes; convergence-via-state-probe is forbidden for this class.*

*(machine-added 2026-06-09, round-19 find-3 — for human review/integration; full context: `Research/notes/19G` + `ch-shape-anno` / the foundational compiled-probe)*

- [ ] **oracle-argparse + inline kind-annotation: the entity-resolution that replaces the engine's flag-strip stand-in.** find-3: the engine must do ZERO argstring parsing — identity is *declared, never inferred* (SF-1 / `an-entity-uniqueness` / `inv-referent-agnostic`). The oracle writes a mini-argparse in our *constrained* oracle-contract dialect (NOT arbitrary sh) and inline-annotates the operand's kind; the analyzer flow-tracks the book's constant *through that argparse* to the annotation. Needs a narrow value-plane (constants + simple `$n` — the spike lacks it, `16C`) and call-edges into oracle bodies (`seam-interproc`). The strawman:

    ```sh
    apt_get__check() {                          # the oracle's check: a mini-argparse mirroring apt-get
       while [ "${1#-}" != "$1" ]; do           # skip options the way apt-get really does
          case $1 in -t|-o) shift 2 ;; *) shift ;; esac
       done
       verb=$1; shift
       pkg : com.debian.apt.Package = "$1"      # ← inline-declare: bless THIS value's kind, on THIS path
       dpkg-query -W "$pkg"
    }
    ```
