//! `differential` — seeded-random differential testing at scale (round-21 arch-7).
//!
//! The cheapest local approximation of the deferred **cm-1** product-gate (`20K` §3/§4):
//! the one gate that *observes elided sites*. For a seeded `(book, oracle-set,
//! host-state)` triple, it drives the REAL `dorc` binary, executes the bare book and
//! the eliding apply under inert mocks (the `e2e/run.sh` discipline), and asserts the
//! soundness property:
//!
//! > the apply artifact's execution trace equals the bare book's trace MODULO licensed
//! > elisions — every command absent from the apply trace is covered by a minted license
//! > (an `omit`/`replace` disposition the engine emitted), and NO command the four-outcome
//! > lattice required to run is absent (an **under-execute** is the disaster class).
//!
//! This is the local stand-in for cm-1 because cm-1's only deferral reason was the
//! human's isolation-tier pricing (`20K` §4): tens of thousands of seeds are expensive
//! per isolation-tier but cheap in-process. A `u64` seed fully determines a trial; the
//! generator's randomness flows through a seeded LCG ([`Rng`]) — never OS randomness or
//! the clock (`inv-determinism`; this crate is the sanctioned DI seam).
//!
//! ## Why drive the binary, not the in-process API
//!
//! The harness invokes `dorc` as a subprocess and `dash` to execute artifacts, exactly
//! as `e2e/run.sh` does (the pre-solved Windows/MSYS invocation pattern — heredoc-fed
//! script or absolute book path, `PATH=mocks-only`, sandbox cwd, `DORC_LOG` absolute).
//! That makes the harness logic itself dependency-light (only `std::process` + string
//! work) and exercises the true end-to-end pipeline — strictly stronger than the
//! in-memory DST tests already in `lib.rs`.
//!
//! ## The faithful host-state flow (no site-number guessing)
//!
//! Host-state is rendered into BOTH probe-results AND mock behaviors, but the probe
//! results are *derived by executing the probe artifact* under host-state-aware probe
//! mocks — the probe self-reports correctly-keyed `site N effect=… rc=…` records, so the
//! harness never has to predict the engine's `LeafId` numbering. The apply mocks, by
//! contrast, are host-state-INDEPENDENT (a mutator always logs+exits-0 in both the bare
//! and apply runs): the bare/apply trace difference must come ONLY from elision, never
//! from mock divergence (the anti-drift discipline — see the adversarial hunt-list).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A tiny deterministic linear-congruential PRNG — the harness's seeded entropy
/// (independent of [`crate::Host`]'s LCG so the generator's draw-order is local). A
/// `u64` seed fully determines a trial (`inv-determinism`): no OS randomness, no clock.
/// The multiplier/increment are the common 64-bit LCG constants (Knuth/PCG lineage).
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    /// Seed the generator. The same seed reproduces the same trial bit-for-bit.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        // Mix the raw seed once so adjacent seeds (0,1,2…) don't yield near-identical
        // first draws (a raw LCG's low bits cycle short).
        Rng(seed ^ 0x9E37_79B9_7F4A_7C15)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    /// A uniform value in `0..n` (n > 0). For `n == 0` returns 0 (defensive; callers
    /// never pass 0).
    ///
    /// Uses the HIGH 32 bits of the LCG output (`>> 32`), not the low bits: a power-of-two
    /// LCG's low bits have very short cycles (the lowest bit alternates with seed parity),
    /// so `n=2` off the low bit correlated `set -e` with seed parity (a real generator
    /// finding — 21D triage). The high bits mix well, decorrelating the draws.
    fn below(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let hi = (self.next_u64() >> 32) as u32;
        hi.checked_rem(n).unwrap_or(0)
    }

    /// A coin flip true with probability `num/den`.
    fn chance(&mut self, num: u32, den: u32) -> bool {
        den != 0 && self.below(den) < num
    }

    /// Pick a uniform index into a NON-EMPTY slice (caller contract — every call site
    /// passes a fixed non-empty static slice). Returns the index, clamped in-bounds; the
    /// caller reads `slice[idx]` via `.get()` to stay panic-free (`inv-no-throw`).
    fn pick_index(&mut self, len: usize) -> usize {
        let n = u32::try_from(len).unwrap_or(0);
        self.below(n) as usize
    }

    /// Pick one element of a NON-EMPTY slice. Total: for the contract-violating empty case
    /// it returns the slice's first element via `get`, which is also `None`, so the whole
    /// expression is `None` only when empty — callers pass non-empty slices, so this never
    /// yields `None` in practice. Implemented over `get` (no indexing-panic).
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        let i = self.pick_index(xs.len());
        match xs.get(i).or_else(|| xs.first()) {
            Some(x) => x,
            // Unreachable for a non-empty slice; a const-free total sink.
            None => Self::empty_pick_panic(),
        }
    }

    /// The unreachable empty-slice sink for [`pick`] — separated so the (dead) panic does
    /// not contaminate `pick`'s lint profile. `inv-no-throw` note: callers never pass an
    /// empty slice, so this is dead code; it exists only to keep `pick` returning `&T`.
    #[expect(
        clippy::panic,
        reason = "unreachable: every pick caller passes a non-empty static slice"
    )]
    fn empty_pick_panic<'a, T>() -> &'a T {
        panic!("Rng::pick on an empty slice (caller contract violated)")
    }
}

/// One modeled kind the generator can use — a `(kind, provider, verb, selector)` tuple
/// plus the entity vocabulary it draws from. Mirrors `package.oracle.sh`'s shape: an
/// establish effect, a simple mockable `oracle_probe_*` body, and a `<provider>__check`
/// argparse in the constrained dialect.
#[derive(Debug, Clone)]
struct KindSpec {
    kind: &'static str,
    provider: &'static str,
    verb: &'static str,
    selector: &'static str,
    /// Whether the provider's argv carries a verb (apt-get install <e>) or is bare
    /// (useradd <e>). Bare-verb commands have no oracle effect for the `check`'s verb
    /// branch — they resolve the operand directly.
    has_verb: bool,
    /// The effect polarity the oracle declares: `establish` for a mutator (the default),
    /// `query` for a read-only guard (`command -v` ⇒ a Query site whose own rc is
    /// fold-usable). A Query kind MUST declare `oracle_effect <provider> '' query <sel>`
    /// or the engine resolves the guard to Opaque and the whole `||` line poisons to ⊤
    /// (the empty-probe-results bug this field fixes).
    polarity: Polarity,
    entities: &'static [&'static str],
}

/// The effect polarity a generated kind declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Polarity {
    Establish,
    Query,
}

impl Polarity {
    fn word(self) -> &'static str {
        match self {
            Polarity::Establish => "establish",
            Polarity::Query => "query",
        }
    }
}

/// The fixed modeled-kind vocabulary. Small, distinct entities (so trace argv lines are
/// mostly unique — the judge's multiset accounting degrades gracefully on duplicates but
/// is cleanest when argvs are distinct).
const KINDS: &[KindSpec] = &[
    KindSpec {
        kind: "package",
        provider: "instpkg",
        verb: "install",
        selector: "installed",
        has_verb: true,
        polarity: Polarity::Establish,
        entities: &["alpha", "bravo", "charlie", "delta"],
    },
    KindSpec {
        kind: "fwrule",
        provider: "allowfw",
        verb: "allow",
        selector: "open",
        has_verb: true,
        polarity: Polarity::Establish,
        entities: &["p80", "p443", "p22"],
    },
    KindSpec {
        kind: "svc",
        provider: "enablesvc",
        verb: "enable",
        selector: "enabled",
        has_verb: true,
        polarity: Polarity::Establish,
        entities: &["echo", "foxtrot", "golf"],
    },
];

/// A query-guard tool the generator pairs with a mutator (the `command -v X || install`
/// idiom). A query is a read-only kind whose probe is the same simple mockable command;
/// its own rc is fold-usable (rule-query-validity) when nothing mutates upstream. It
/// declares `query` polarity (NOT establish) — `command -v` READS, it does not mutate.
const QUERY_KIND: KindSpec = KindSpec {
    kind: "tool",
    provider: "havetool",
    verb: "",
    selector: "present",
    has_verb: false,
    polarity: Polarity::Query,
    entities: &["hotel", "india", "juliet"],
};

/// The shape of the book the generator emitted — drives both the rendering and the
/// expectation the judge applies to ⊤-control trials (which must elide nothing).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Straight-line mutators (`apt-get install -y E`).
    StraightLine,
    /// `if <query>; then <mutator>; fi` guarded.
    IfGuard,
    /// `<query> || <mutator>` (the canonical idempotency idiom).
    OrGuard,
    /// `<query> && <mutator>`.
    AndGuard,
    /// `for x in a b; do <mutator>; done` (literal-list loop; member sites).
    ForLoop,
    /// `<mutator> || true` (door-3 rc-deadness).
    OrTrue,
    /// A deliberate ⊤-shape (eval / break-in-loop / background / cmdsubst / dynamic
    /// name). A CONTROL: the engine must elide NOTHING (⊤ ⇒ run), so the apply trace
    /// must equal the bare trace.
    TopControl,
}

/// One generated entity touched by the book, with its seeded host-state (converged ⇒ the
/// fact holds; diverged ⇒ it doesn't). Drives both the probe mocks and the judge's
/// licensing expectations.
#[derive(Debug, Clone)]
struct EntityState {
    /// The probe command the kind's `oracle_probe_*` body calls (one per kind).
    probe_cmd: String,
    entity: String,
    converged: bool,
    /// For a QUERY guard, the guard command that appears in the BARE book (`havetool`):
    /// its apply-mock must exit per host-state (the guard's rc drives `||`/`&&`/`if`
    /// control flow), so the bare trace reflects the same convergence the probe reported.
    /// `None` for a mutator entity (its apply-mock always exits 0).
    guard_cmd: Option<String>,
}

/// A fully-generated trial: the files to stamp, the host-state, the shape, and the
/// command vocabulary needed to stamp mocks. Deterministic in the seed.
#[derive(Debug, Clone)]
pub struct Trial {
    pub seed: u64,
    pub shape: Shape,
    pub book: String,
    /// `(filename, contents)` for each oracle.
    pub oracles: Vec<(String, String)>,
    /// Mutator apply-mock command names (host-state-INDEPENDENT inert shims: log argv,
    /// exit 0 — identical in the bare and apply runs, the anti-drift discipline).
    apply_cmds: Vec<String>,
    /// Per-entity seeded host-state, rendered into probe mocks AND (for guards) the
    /// host-state-dependent query apply-mocks.
    entity_states: Vec<EntityState>,
    /// Whether this is a deliberate ⊤-control (the judge asserts runs-everything).
    pub is_top_control: bool,
}

/// Generate a trial deterministically from a seed. The generator stays STRICTLY inside
/// the modeled subset (`inv-top-reject`'s trigger list): straight-line, `if`/`&&`/`||`
/// guarded, literal-list `for`-loops, `|| true` forms, `set -e` on/off — plus occasional
/// deliberate ⊤-shapes as controls. Every emitted book is `dash -n`-clean by construction
/// (the `generator_emits_only_dash_n_clean_books` pin guards this across 200 seeds).
#[must_use]
pub fn generate(seed: u64) -> Trial {
    let mut rng = Rng::new(seed);
    let set_e = rng.chance(1, 2);

    // 1-in-9 trials is a deliberate ⊤-control (asserts runs-everything).
    let shape = if rng.chance(1, 9) {
        Shape::TopControl
    } else {
        *rng.pick(&[
            Shape::StraightLine,
            Shape::IfGuard,
            Shape::OrGuard,
            Shape::AndGuard,
            Shape::ForLoop,
            Shape::OrTrue,
        ])
    };

    let mut book = String::new();
    book.push_str("#!/bin/sh\n");
    if set_e {
        book.push_str("set -e\n");
    }

    let mut apply_cmds: Vec<String> = Vec::new();
    let mut entity_states: Vec<EntityState> = Vec::new();
    let mut used_kinds: Vec<KindSpec> = Vec::new();
    let mut used_query = false;

    // A small helper closure can't borrow rng mutably twice cleanly; inline instead.
    let n_lines = 1 + rng.below(3); // 1..=3 statements

    match shape {
        Shape::TopControl => {
            render_top_control(
                &mut rng,
                &mut book,
                &mut apply_cmds,
                &mut entity_states,
                &mut used_kinds,
            );
        }
        _ => {
            for _ in 0..n_lines {
                render_modeled_line(
                    &mut rng,
                    shape,
                    &mut book,
                    &mut apply_cmds,
                    &mut entity_states,
                    &mut used_kinds,
                    &mut used_query,
                );
            }
        }
    }

    // Always emit a trailing inert marker so the book is never empty and always has a
    // guaranteed-run line (a builtin `echo`, which the apply must keep — a useful judge
    // anchor that the apply never drops a pure builtin).
    let _ = writeln!(book, "echo generated-{seed}");

    let oracles = build_oracles(&used_kinds, used_query);

    let is_top_control = matches!(shape, Shape::TopControl);
    Trial {
        seed,
        shape,
        book,
        oracles,
        apply_cmds,
        entity_states,
        is_top_control,
    }
}

/// Render one modeled (non-⊤) statement of the given shape, accumulating the commands &
/// entity-states it introduces. Each arm pushes exactly the entity-states + apply-mocks it
/// needs (no shared pre-push — the for-loop and guarded shapes have different site sets).
fn render_modeled_line(
    rng: &mut Rng,
    shape: Shape,
    book: &mut String,
    apply_cmds: &mut Vec<String>,
    entity_states: &mut Vec<EntityState>,
    used_kinds: &mut Vec<KindSpec>,
    used_query: &mut bool,
) {
    let k = rng.pick(KINDS).clone();
    let entity = (*rng.pick(k.entities)).to_string();
    let converged = rng.chance(1, 2);
    let mutator = mutator_argv(&k, &entity);
    push_unique(apply_cmds, k.provider.to_string());
    push_kind(used_kinds, &k);

    let push_mut = |es: &mut Vec<EntityState>, e: &str, conv: bool| {
        es.push(EntityState {
            probe_cmd: probe_cmd_name(&k),
            entity: e.to_string(),
            converged: conv,
            guard_cmd: None,
        });
    };

    match shape {
        Shape::StraightLine => {
            push_mut(entity_states, &entity, converged);
            let _ = writeln!(book, "{mutator}");
        }
        Shape::OrTrue => {
            push_mut(entity_states, &entity, converged);
            let _ = writeln!(book, "{mutator} || true");
        }
        Shape::ForLoop => {
            // for x in <e1> <e2>; do <provider> <verb> "$x"; done — each member is a site.
            let e2 = (*rng.pick(k.entities)).to_string();
            let conv2 = rng.chance(1, 2);
            push_mut(entity_states, &entity, converged);
            if e2 != entity {
                push_mut(entity_states, &e2, conv2);
            }
            let verb = if k.has_verb {
                format!("{} ", k.verb)
            } else {
                String::new()
            };
            let list = if e2 == entity {
                entity.clone()
            } else {
                format!("{entity} {e2}")
            };
            let _ = writeln!(
                book,
                "for x in {list}; do {} {verb}\"$x\"; done",
                k.provider
            );
        }
        Shape::IfGuard | Shape::OrGuard | Shape::AndGuard => {
            push_mut(entity_states, &entity, converged);
            *used_query = true;
            let q = QUERY_KIND;
            let qe = (*rng.pick(q.entities)).to_string();
            let qconv = rng.chance(1, 2);
            // The query guard is BOTH a bare-book command (its apply-mock exits per
            // host-state — guard_cmd=Some) AND a probe site (dorcprobe_tool). It is NOT a
            // mutator apply-cmd (those always exit 0); its mock is host-state-dependent.
            entity_states.push(EntityState {
                probe_cmd: probe_cmd_name(&q),
                entity: qe.clone(),
                converged: qconv,
                guard_cmd: Some(q.provider.to_string()),
            });
            let guard = format!("{} {qe} >/dev/null 2>&1", q.provider);
            let line = match shape {
                Shape::AndGuard => format!("{guard} && {mutator}"),
                Shape::IfGuard => format!("if {guard}; then {mutator}; fi"),
                // OrGuard (and, defensively, any other guard shape).
                _ => format!("{guard} || {mutator}"),
            };
            let _ = writeln!(book, "{line}");
        }
        // TopControl is routed to `render_top_control` by the caller, never here; a no-op
        // keeps this total without an unreachable-panic (inv-no-throw).
        Shape::TopControl => debug_assert!(false, "⊤ handled separately by generate()"),
    }
}

/// Render a deliberate ⊤-shape (a CONTROL). Each is an `inv-top-reject` trigger; the
/// engine must collapse to ⊤ ⇒ run everything, so the apply trace must equal the bare
/// trace. We pair the ⊤ construct with a following mutator that is CONVERGED + probed —
/// absent the ⊤ it would elide, so the ONLY reason it must run is the ⊤'s havoc/poison
/// (⊤-containment). That makes `TopControlElided` a meaningful test: if the engine still
/// elides the converged mutator past the ⊤, the control fires.
fn render_top_control(
    rng: &mut Rng,
    book: &mut String,
    apply_cmds: &mut Vec<String>,
    entity_states: &mut Vec<EntityState>,
    used_kinds: &mut Vec<KindSpec>,
) {
    let k = rng.pick(KINDS).clone();
    let entity = (*rng.pick(k.entities)).to_string();
    push_unique(apply_cmds, k.provider.to_string());
    push_kind(used_kinds, &k);
    // CONVERGED, so without the ⊤ it would elide — isolating the ⊤'s poison.
    entity_states.push(EntityState {
        probe_cmd: probe_cmd_name(&k),
        entity: entity.clone(),
        converged: true,
        guard_cmd: None,
    });
    let mutator = mutator_argv(&k, &entity);

    // The ⊤ trigger precedes the mutator so its ⊤-containment poisons the mutator's
    // elision (the mutator sits in the ⊤'s havoc region). ONLY HARD ⊤s that demonstrably
    // havoc downstream belong here: `eval` and `break`-in-loop both ⊤-reject (loud
    // `syntax-unsupported` + `cfg-top-node`) AND poison the following converged install
    // (verified — note 21D triage). Constructs like `echo "$(echo sub)"` (cmdsubst in
    // ARGUMENT position) are NOT hard ⊤s — they are localized dynamic-args that do NOT
    // poison an independent downstream install, so the engine correctly elides past them;
    // including them as "controls" was a generator bug (the seed-29 false-positive).
    let trigger = *rng.pick(&["eval 'echo hi'", "for x in a b; do break; done"]);
    let _ = writeln!(book, "{trigger}");
    let _ = writeln!(book, "{mutator}");
}

/// The mutator argv for a kind+entity (`<provider> <verb> -y <entity>` or bare
/// `<provider> <entity>`). A `-y` flag exercises the argparse flag-strip.
fn mutator_argv(k: &KindSpec, entity: &str) -> String {
    if k.has_verb {
        format!("{} {} -y {entity}", k.provider, k.verb)
    } else {
        format!("{} {entity}", k.provider)
    }
}

/// The single mockable probe command name for a kind (`dorcprobe_<kind>`). Keeping ONE
/// simple command per kind (vs. `package.oracle.sh`'s real `dpkg-query` pipeline) makes
/// the probe trivially mockable: the shim exits per host-state.
fn probe_cmd_name(k: &KindSpec) -> String {
    format!("dorcprobe_{}", k.kind)
}

fn push_unique(v: &mut Vec<String>, s: String) {
    if !v.contains(&s) {
        v.push(s);
    }
}

fn push_kind(v: &mut Vec<KindSpec>, k: &KindSpec) {
    if !v.iter().any(|x| x.kind == k.kind) {
        v.push(k.clone());
    }
}

/// Build the oracle files for the used kinds (+ the query oracle if any guard was
/// emitted). Mirrors `package.oracle.sh`: `oracle_kind`, a simple `oracle_probe_*`, an
/// `oracle_effect` marker, and a `<provider>__check` argparse in the constrained dialect.
fn build_oracles(used_kinds: &[KindSpec], used_query: bool) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for k in used_kinds {
        out.push((format!("{}.oracle.sh", k.kind), oracle_text(k)));
    }
    if used_query {
        out.push((
            format!("{}.oracle.sh", QUERY_KIND.kind),
            oracle_text(&QUERY_KIND),
        ));
    }
    out
}

/// One oracle file's text — the constrained-dialect shape the engine lifts.
fn oracle_text(k: &KindSpec) -> String {
    let probe = probe_cmd_name(k);
    let mut s = String::new();
    let _ = writeln!(s, "#!/bin/sh");
    let _ = writeln!(s, "oracle_kind={}", k.kind);
    let _ = writeln!(s, "oracle_probe_{}() {{", k.kind);
    let _ = writeln!(s, "   {probe} \"$1\"");
    let _ = writeln!(s, "}}");
    if k.has_verb {
        // Verbed effect (`<provider> <verb> establish|query <selector>`).
        let _ = writeln!(
            s,
            "oracle_effect {} {} {} {}",
            k.provider,
            k.verb,
            k.polarity.word(),
            k.selector
        );
        let _ = writeln!(s, "{}() {{", check_fn_name(k.provider));
        let _ = writeln!(s, "   while [ \"${{1#-}}\" != \"$1\" ]; do shift; done");
        let _ = writeln!(s, "   verb=$1; shift");
        let _ = writeln!(s, "   while [ \"${{1#-}}\" != \"$1\" ]; do shift; done");
        let _ = writeln!(s, "   e : {} = \"$1\"", k.kind);
        let _ = writeln!(s, "   if [ \"$2\" = \"\" ]; then {probe} \"$e\"; fi");
        let _ = writeln!(s, "}}");
    } else {
        // Verbless effect (the `command -v X` query shape): the effect-map keys on the
        // ε-verb (`''`), so the marker MUST declare it or the guard resolves to Opaque
        // (the empty-probe-results bug). Strip a single leading flag, then annotate $1.
        let _ = writeln!(
            s,
            "oracle_effect {} '' {} {}",
            k.provider,
            k.polarity.word(),
            k.selector
        );
        let _ = writeln!(s, "{}() {{", check_fn_name(k.provider));
        let _ = writeln!(s, "   while [ \"${{1#-}}\" != \"$1\" ]; do shift; done");
        let _ = writeln!(s, "   e : {} = \"$1\"", k.kind);
        let _ = writeln!(s, "   if [ \"$2\" = \"\" ]; then {probe} \"$e\"; fi");
        let _ = writeln!(s, "}}");
    }
    s
}

/// The `<provider>__check` function name: the provider with `-` → `_` (the engine keys
/// the check by the command word; `apt-get` lifts as `apt_get__check`). Our generated
/// providers have no `-`, but keep the mapping faithful.
fn check_fn_name(provider: &str) -> String {
    format!("{}__check", provider.replace('-', "_"))
}

// ===========================================================================
// The shell driver — drives the REAL dorc binary + dash, mirroring e2e/run.sh.
// ===========================================================================

/// Where the harness finds the tools it drives. The binary path mirrors `run.sh`'s
/// auto-location; `dash` is the POSIX checker/executor (`/bin/dash` on this MSYS box).
#[derive(Debug, Clone)]
pub struct Tools {
    pub dorc: PathBuf,
    pub dash: PathBuf,
}

impl Tools {
    /// Locate the tools the way `run.sh` does: `target/{debug,release}/dorc[.exe]` and
    /// `dash` (else `sh`) on PATH.
    ///
    /// # Errors
    /// Returns a descriptive message if the `dorc` binary is not built or no `dash`/`sh`
    /// is on PATH.
    pub fn locate(spike_root: &Path) -> Result<Self, String> {
        let cands = [
            "target/debug/dorc.exe",
            "target/debug/dorc",
            "target/release/dorc.exe",
            "target/release/dorc",
        ];
        let dorc = cands
            .iter()
            .map(|c| spike_root.join(c))
            .find(|p| p.is_file())
            .ok_or_else(|| {
                format!(
                    "dorc binary not found under {} — build it first (cargo build from spike/)",
                    spike_root.display()
                )
            })?;
        let dash = find_on_path("dash")
            .or_else(|| find_on_path("sh"))
            .ok_or("no dash/sh on PATH for the POSIX exec gate")?;
        Ok(Tools { dorc, dash })
    }
}

/// Find an executable on PATH (the harness needs `dash`'s absolute path — `run.sh`'s
/// `$checker_abs` — because the exec sandbox overrides PATH to mocks-only).
fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        for ext in ["", ".exe"] {
            let cand = dir.join(format!("{name}{ext}"));
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    None
}

/// A scratch directory the trial's material is stamped into, auto-removed on drop. The
/// harness writes generated files here; nothing escapes it (`inv-determinism`: the only
/// disk touched is this disposable tree).
#[derive(Debug)]
pub struct Scratch {
    dir: PathBuf,
}

impl Scratch {
    /// Create a fresh scratch dir under the system temp, salted with the seed (so a
    /// crash leaves an inspectable, attributable tree).
    fn new(seed: u64) -> std::io::Result<Self> {
        let mut dir = std::env::temp_dir();
        dir.push(format!("dorc-diff-{seed}-{}", std::process::id()));
        // Fresh: remove any stale tree from a prior aborted run of this exact seed/pid.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("mocks"))?;
        Ok(Scratch { dir })
    }

    fn path(&self) -> &Path {
        &self.dir
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if std::env::var_os("DIFF_KEEP").is_some() {
            return; // debug: leave the scratch tree for inspection
        }
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

/// One disposition the engine emitted for a site (the license ledger, parsed from
/// `--debug-argv` stderr). The `argv <leafid> <disposition> <words…>` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    pub leafid: u32,
    pub disposition: String,
    pub argv: String,
}

/// The full result of running one trial through the binary + dash: the artifacts, the
/// disposition ledger, and the two execution traces. The judge consumes this.
#[derive(Debug, Clone)]
pub struct RunOutcome {
    pub probe_art: String,
    pub apply_art: String,
    pub probe_results: String,
    pub ledger: Vec<LedgerEntry>,
    /// `ran: …`-stripped argv lines from executing the BARE book under apply-mocks.
    pub bare_trace: Vec<String>,
    /// …from executing the APPLY artifact under the SAME apply-mocks.
    pub apply_trace: Vec<String>,
    /// dorc's stderr (diagnostics) — gate-3-style error scan.
    pub dorc_stderr: String,
}

/// Stamp + run one trial end-to-end. Returns the [`RunOutcome`] the judge grades.
///
/// # Errors
/// Returns a [`RunError`] if the binary refused the generated material (a generator bug —
/// the harness owns those; `inv-top-reject` ⊤-rejects are NOT errors, they are valid
/// runs-everything controls), an artifact is empty, a render fails `dash -n`, or a disk/
/// process operation fails.
pub fn run_trial(tools: &Tools, trial: &Trial) -> Result<RunOutcome, RunError> {
    let scratch = Scratch::new(trial.seed).map_err(|e| RunError::Io(e.to_string()))?;
    let dir = scratch.path();

    // Stamp the book + oracles.
    std::fs::write(dir.join("book.sh"), &trial.book).map_err(|e| RunError::Io(e.to_string()))?;
    let mut oracle_paths: Vec<PathBuf> = Vec::new();
    for (name, text) in &trial.oracles {
        let p = dir.join(name);
        std::fs::write(&p, text).map_err(|e| RunError::Io(e.to_string()))?;
        oracle_paths.push(p);
    }

    // Stamp ALL mocks (mutator apply / query-guard apply / probe). Builtins the ⊤-control
    // may call (`echo`/`eval`/`break`/`for`) need no shim — dash provides them.
    stamp_mocks(dir, trial).map_err(|e| RunError::Io(e.to_string()))?;

    let oracle_args: Vec<String> = oracle_paths
        .iter()
        .flat_map(|p| ["-o".to_string(), p.to_string_lossy().into_owned()])
        .collect();

    // PASS A — capture the probe artifact (empty stdin; we only want the probe block).
    let pass_a = run_dorc(tools, dir, &oracle_args, "", false)?;
    let probe_art = first_shebang_block(&pass_a.stdout);
    if probe_art.trim().is_empty() {
        return Err(RunError::EmptyArtifact("probe (pass A)".into()));
    }

    // Execute the probe under PROBE mocks ⇒ correctly-keyed probe-results.
    let probe_results = exec_probe(tools, dir, &probe_art);

    // PASS B — the real round-trip with the derived probe-results + the ledger.
    let pass_b = run_dorc(tools, dir, &oracle_args, &probe_results, true)?;
    let (probe_b, apply_art) = split_shebang_blocks(&pass_b.stdout);
    let _ = probe_b;
    if apply_art.trim().is_empty() {
        return Err(RunError::EmptyArtifact("apply (pass B)".into()));
    }
    let ledger = parse_ledger(&pass_b.stderr);

    // dash -n both artifacts (parse gate).
    dash_n(tools, &probe_art).map_err(|e| RunError::DashN("probe".into(), e))?;
    dash_n(tools, &apply_art).map_err(|e| RunError::DashN("apply".into(), e))?;

    // Execute BARE book + APPLY artifact under the SAME apply-mocks (drift-free).
    let book_abs = dir.join("book.sh");
    let bare_trace = exec_under_mocks(tools, dir, ExecTarget::File(&book_abs));
    let apply_trace = exec_under_mocks(tools, dir, ExecTarget::Script(&apply_art));

    Ok(RunOutcome {
        probe_art,
        apply_art,
        probe_results,
        ledger,
        bare_trace,
        apply_trace,
        dorc_stderr: pass_b.stderr,
    })
}

/// An error stamping/running a trial (distinct from a soundness FINDING — these are
/// harness/generator faults the triage classifies).
#[derive(Debug, Clone)]
pub enum RunError {
    Io(String),
    /// dorc exited non-zero or produced an empty artifact (a generator bug: invalid sh /
    /// oracle-vocabulary error the engine refused — OR an engine crash).
    DorcFailed {
        rc: i32,
        stderr: String,
    },
    EmptyArtifact(String),
    /// A rendered artifact failed `dash -n` (a generator bug — the generated/rendered sh
    /// is not parseable; the engine should never emit non-parseable sh, so this is either
    /// a generator emitting un-renderable input OR an engine render bug → a FINDING).
    DashN(String, String),
}

/// Raw stdout/stderr/rc of one dorc invocation.
struct DorcRun {
    stdout: String,
    stderr: String,
}

fn run_dorc(
    tools: &Tools,
    dir: &Path,
    oracle_args: &[String],
    stdin: &str,
    debug_argv: bool,
) -> Result<DorcRun, RunError> {
    use std::io::Write as _;
    use std::process::Stdio;
    let mut cmd = Command::new(&tools.dorc);
    cmd.current_dir(dir);
    cmd.arg(format!("--book={}", dir.join("book.sh").display()));
    if debug_argv {
        cmd.arg("--debug-argv");
    }
    cmd.args(oracle_args);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| RunError::Io(e.to_string()))?;
    if let Some(mut si) = child.stdin.take() {
        let _ = si.write_all(stdin.as_bytes());
        // drop closes stdin (EOF) — dorc reads results to EOF then emits the apply.
    }
    let out = child
        .wait_with_output()
        .map_err(|e| RunError::Io(e.to_string()))?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        return Err(RunError::DorcFailed {
            rc: out.status.code().unwrap_or(-1),
            stderr,
        });
    }
    Ok(DorcRun { stdout, stderr })
}

/// What to execute under mocks: an absolute book path, or an in-memory script (the apply
/// artifact, fed via stdin — mirroring `run.sh`'s heredoc). Holds only borrows ⇒ `Copy`.
#[derive(Clone, Copy)]
enum ExecTarget<'a> {
    File(&'a Path),
    Script(&'a str),
}

/// Execute a target under PATH=mocks-only + sandbox cwd + an absolute `DORC_LOG`, exactly
/// the `e2e/run.sh` discipline. Returns the `ran: …`-stripped argv lines (the trace).
/// Errors are swallowed (a non-zero rc from `set -e` + a diverged guard is normal); the
/// trace is whatever was logged.
fn exec_under_mocks(tools: &Tools, dir: &Path, target: ExecTarget<'_>) -> Vec<String> {
    use std::io::Write as _;
    use std::process::Stdio;
    let mocks = dir.join("mocks");
    // A fresh per-exec sandbox cwd + log (disposable; mirrors run.sh's mktemp -d).
    let log = unique_temp(dir, "log");
    let sand = unique_temp(dir, "sand");
    let _ = std::fs::create_dir_all(&sand);

    let mut cmd = Command::new(&tools.dash);
    cmd.current_dir(&sand);
    cmd.env("DORC_LOG", &log);
    cmd.env("PATH", &mocks); // mocks-only: an un-shimmed external ⇒ command-not-found (loud)
    match target {
        ExecTarget::File(p) => {
            cmd.arg(p);
            cmd.stdin(Stdio::null());
        }
        ExecTarget::Script(_) => {
            cmd.stdin(Stdio::piped());
        }
    }
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    if let Ok(mut child) = cmd.spawn() {
        if let ExecTarget::Script(s) = target
            && let Some(mut si) = child.stdin.take()
        {
            let _ = si.write_all(s.as_bytes());
        }
        let _ = child.wait();
    }
    let trace = std::fs::read_to_string(&log).unwrap_or_default();
    let _ = std::fs::remove_dir_all(&sand);
    let _ = std::fs::remove_file(&log);
    trace
        .lines()
        .filter_map(|l| l.strip_prefix("ran: "))
        .map(normalize_argv)
        .collect()
}

/// Execute the probe artifact under PROBE mocks ⇒ the `site …` records (the probe's
/// stdout). `DORC_LOG` is set to a throwaway (the probe shims don't log; they exit per
/// host-state). PATH=mocks-only so the probe's `dorcprobe_*` calls hit our shims.
fn exec_probe(tools: &Tools, dir: &Path, probe_art: &str) -> String {
    use std::io::Write as _;
    use std::process::Stdio;
    let mocks = dir.join("mocks");
    let log = unique_temp(dir, "plog");
    let sand = unique_temp(dir, "psand");
    let _ = std::fs::create_dir_all(&sand);
    let mut cmd = Command::new(&tools.dash);
    cmd.current_dir(&sand);
    cmd.env("DORC_LOG", &log);
    cmd.env("PATH", &mocks);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::null());
    let mut records = String::new();
    if let Ok(mut child) = cmd.spawn() {
        if let Some(mut si) = child.stdin.take() {
            let _ = si.write_all(probe_art.as_bytes());
        }
        if let Ok(out) = child.wait_with_output() {
            records = String::from_utf8_lossy(&out.stdout).into_owned();
        }
    }
    let _ = std::fs::remove_dir_all(&sand);
    let _ = std::fs::remove_file(&log);
    // Keep only the records (drop the probe's own `# site …` provenance comments — dorc's
    // parser ignores comments anyway, but trimming keeps the fixture clean for findings).
    records
        .lines()
        .filter(|l| l.trim_start().starts_with("site "))
        .map(|l| l.trim_end_matches('\r').to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

/// `dash -n` a rendered artifact (the parse gate). Returns the shell's diagnostic on
/// failure.
fn dash_n(tools: &Tools, art: &str) -> Result<(), String> {
    use std::io::Write as _;
    use std::process::Stdio;
    let mut cmd = Command::new(&tools.dash);
    cmd.arg("-n");
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| e.to_string())?;
    if let Some(mut si) = child.stdin.take() {
        let _ = si.write_all(art.as_bytes());
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).into_owned())
    }
}

/// A unique temp path under the trial scratch (so concurrent execs within one trial don't
/// clash — the harness is single-threaded per trial, but the sweep may parallelize seeds
/// in future).
fn unique_temp(dir: &Path, tag: &str) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    let n = CTR.fetch_add(1, Ordering::Relaxed);
    dir.join(format!(".{tag}-{}-{n}", std::process::id()))
}

/// Normalize a logged/ledger argv for comparison: collapse internal whitespace runs to a
/// single space and trim. The mock logs `${0##*/} $*` (so a trailing empty arg leaves a
/// trailing space) and the ledger joins words with single spaces — normalize both sides.
fn normalize_argv(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Stamp ALL of a trial's mocks under `mocks/`, in three coherent roles:
///
/// * **mutator apply-mocks** (`apply_cmds`): host-state-INDEPENDENT inert shims — log
///   argv, exit 0, IDENTICAL in the bare and apply runs (the anti-drift discipline: a
///   mutator's exit never differs between the two runs, so a bare/apply trace difference
///   can only come from elision).
/// * **query-guard apply-mocks** (`guard_cmd` on a query entity-state): host-state-
///   DEPENDENT — log argv AND exit 0 iff the guarded entity is converged. The guard's rc
///   drives `||`/`&&`/`if` control flow in the BARE book, so it must report the SAME
///   convergence the probe does (else the bare control-flow and the engine's fold
///   disagree — the empty-probe-results / mock-drift bug).
/// * **probe-mocks** (`probe_cmd`): host-state-DEPENDENT — exit 0 iff converged, NO log
///   (used only when executing the probe artifact to derive `site …` records).
///
/// A guard's probe-mock and its apply-mock consult the SAME converged set, so they never
/// disagree on host-state.
fn stamp_mocks(dir: &Path, trial: &Trial) -> std::io::Result<()> {
    // (1) mutator apply-mocks: inert, always exit 0.
    for cmd in &trial.apply_cmds {
        let body =
            "#!/bin/sh\nprintf 'ran: %s %s\\n' \"${0##*/}\" \"$*\" >>\"$DORC_LOG\"\nexit 0\n";
        write_shim(dir, cmd, body)?;
    }

    // Group converged entities per (probe command) and per (guard command).
    let mut probe_converged: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut probe_cmds: BTreeSet<String> = BTreeSet::new();
    let mut guard_converged: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut guard_cmds: BTreeSet<String> = BTreeSet::new();
    for st in &trial.entity_states {
        probe_cmds.insert(st.probe_cmd.clone());
        if st.converged {
            probe_converged
                .entry(st.probe_cmd.clone())
                .or_default()
                .push(st.entity.clone());
        }
        if let Some(g) = &st.guard_cmd {
            guard_cmds.insert(g.clone());
            if st.converged {
                guard_converged
                    .entry(g.clone())
                    .or_default()
                    .push(st.entity.clone());
            }
        }
    }

    // (2) query-guard apply-mocks: log argv THEN exit per host-state.
    for cmd in &guard_cmds {
        let case = host_state_case(guard_converged.get(cmd).map_or(&[][..], Vec::as_slice));
        let body = format!(
            "#!/bin/sh\nprintf 'ran: %s %s\\n' \"${{0##*/}}\" \"$*\" >>\"$DORC_LOG\"\n{case}"
        );
        write_shim(dir, cmd, &body)?;
    }

    // (3) probe-mocks: exit per host-state, no log. (A guard's kind has BOTH a probe-cmd
    // and a guard-cmd — distinct names — so these never collide.)
    for cmd in &probe_cmds {
        let case = host_state_case(probe_converged.get(cmd).map_or(&[][..], Vec::as_slice));
        let body = format!("#!/bin/sh\n{case}");
        write_shim(dir, cmd, &body)?;
    }
    Ok(())
}

/// A `case "$1" in <converged>) exit 0 ;; *) exit 1 ;; esac` body — exit 0 iff `$1` is a
/// converged entity. Pure dash builtins (no external command, so it works under
/// PATH=mocks-only). Patterns are distinct + sorted (deterministic shim text).
fn host_state_case(converged: &[String]) -> String {
    let mut body = String::from("case \"$1\" in\n");
    let mut pats: Vec<&String> = converged.iter().collect();
    pats.sort();
    pats.dedup();
    if !pats.is_empty() {
        let alt = pats
            .iter()
            .map(|e| e.as_str())
            .collect::<Vec<_>>()
            .join("|");
        let _ = writeln!(body, "  {alt}) exit 0 ;;");
    }
    body.push_str("  *) exit 1 ;;\nesac\n");
    body
}

/// Write one shim file under `mocks/` and mark it executable (POSIX; a no-op bit on
/// Windows, but dash on MSYS honors the shebang via the file content regardless).
fn write_shim(dir: &Path, name: &str, body: &str) -> std::io::Result<()> {
    let p = dir.join("mocks").join(name);
    std::fs::write(&p, body)?;
    set_executable(&p);
    Ok(())
}

/// Best-effort chmod +x (no-op where unsupported; dash reads the shebang from content).
fn set_executable(p: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        if let Ok(meta) = std::fs::metadata(p) {
            let mut perm = meta.permissions();
            perm.set_mode(0o755);
            let _ = std::fs::set_permissions(p, perm);
        }
    }
    #[cfg(not(unix))]
    {
        let _ = p; // MSYS dash execs the shim by shebang from content; no mode bit needed.
    }
}

/// Split dorc stdout into the (probe, apply) blocks on their `#!/bin/sh` shebangs (the
/// first block is the probe, the second the apply — `run.sh`'s awk partition).
fn split_shebang_blocks(stdout: &str) -> (String, String) {
    let mut probe = String::new();
    let mut apply = String::new();
    let mut count = 0u32;
    for line in stdout.lines() {
        if line.starts_with("#!/bin/sh") {
            count += 1;
        }
        if count == 1 {
            probe.push_str(line);
            probe.push('\n');
        } else if count >= 2 {
            apply.push_str(line);
            apply.push('\n');
        }
    }
    (probe, apply)
}

/// The first shebang block only (the probe — pass A discards the apply).
fn first_shebang_block(stdout: &str) -> String {
    split_shebang_blocks(stdout).0
}

/// Parse the `--debug-argv` ledger lines (`argv <leafid> <disposition> <words…>`) from
/// dorc's stderr. The disposition is `run`/`replace`/`omit`; the words are the resolved
/// argv (a `TOP` token marks an unresolved word).
fn parse_ledger(stderr: &str) -> Vec<LedgerEntry> {
    let mut out = Vec::new();
    for line in stderr.lines() {
        let Some(rest) = line.strip_prefix("argv ") else {
            continue;
        };
        let mut it = rest.splitn(3, ' ');
        let Some(leaf) = it.next().and_then(|s| s.parse::<u32>().ok()) else {
            continue;
        };
        let Some(disp) = it.next() else { continue };
        let argv = it.next().unwrap_or("").to_string();
        out.push(LedgerEntry {
            leafid: leaf,
            disposition: disp.to_string(),
            argv: normalize_argv(&argv),
        });
    }
    out
}

// ===========================================================================
// The judge — the soundness property, with the under-execute disaster class loud.
// ===========================================================================

/// A graded trial outcome.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// The apply trace == bare trace modulo licensed elisions. Sound.
    Clean,
    /// A soundness FINDING (the product). Carries the class + a human diagnosis.
    Finding(Finding),
}

/// A soundness finding — a divergence from the property. Ranked by severity: an
/// `UnderExecute` is the disaster class (a needed command vanished from the apply with no
/// license); the others grade lower.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub class: FindingClass,
    pub diagnosis: String,
}

/// The finding taxonomy, severity-ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FindingClass {
    /// DISASTER: a command in the bare trace is absent from the apply trace, but its site
    /// was NOT licensed to elide (disposition `run`, or no ledger entry covers it). The
    /// engine dropped a required mutation.
    UnderExecute,
    /// A ⊤-control elided something (the engine must run everything under ⊤).
    TopControlElided,
    /// The apply trace ran a command NOT in the bare trace (a phantom / over-execute).
    OverExecute,
}

impl FindingClass {
    /// A short slug for finding-dir names.
    #[must_use]
    pub fn slug(self) -> &'static str {
        match self {
            FindingClass::UnderExecute => "under-execute",
            FindingClass::TopControlElided => "top-control-elided",
            FindingClass::OverExecute => "over-execute",
        }
    }
}

/// Judge a run outcome against the soundness property. The license ledger (the engine's
/// own `--debug-argv` dispositions) is the authority on which elisions are licensed.
///
/// The checks, in severity order:
/// 1. **Under-execute (disaster):** every command in `bare_trace \ apply_trace` (by
///    count) must be covered by a site the ledger marks `omit`/`replace`. A bare command
///    missing from apply whose only matching ledger entries are `run` ⇒ the engine
///    dropped a required mutation.
/// 2. **⊤-control:** a control trial must elide NOTHING — its apply trace must equal its
///    bare trace (as a multiset).
/// 3. **Over-execute:** `apply_trace \ bare_trace` must be empty — the apply never runs a
///    command the bare book didn't (elision only removes; it never adds).
///
/// (A gate-5-style argv⊆bare check is deliberately NOT here: gate-5/cm-2 already runs on
/// the e2e corpus; for the generated control-flow shapes a `run` site behind a
/// short-circuiting guard legitimately never executes in the bare book, so a `run ⊆ bare`
/// assertion false-positives. The bare-vs-apply differential IS cm-1; a value-flow
/// mis-resolution that licenses a wrong elision surfaces as under-/over-execute. — 21D
/// triage, the seed-13 false-positive.)
#[must_use]
pub fn judge(trial: &Trial, outcome: &RunOutcome, shimmed: &[String]) -> Verdict {
    let bare = Multiset::from(&outcome.bare_trace);
    let apply = Multiset::from(&outcome.apply_trace);

    // (2) ⊤-control must run everything: apply == bare.
    if trial.is_top_control && bare != apply {
        let removed = bare.difference(&apply);
        return Verdict::Finding(Finding {
            class: FindingClass::TopControlElided,
            diagnosis: format!(
                "⊤-control trial (shape {:?}) elided commands the engine must run under ⊤ \
                 (⊤ ⇒ run-everything): bare\\apply = {removed:?}. inv-top-reject says an \
                 unmodeled construct never licenses elision.",
                trial.shape
            ),
        });
    }

    // (3) over-execute: apply must be ⊆ bare.
    let added = apply.difference(&bare);
    if !added.is_empty() {
        return Verdict::Finding(Finding {
            class: FindingClass::OverExecute,
            diagnosis: format!(
                "the apply trace ran commands absent from the bare trace (elision only \
                 removes, never adds): apply\\bare = {added:?}.",
            ),
        });
    }

    // (1) under-execute (disaster): every removed command must be licensed.
    // A trace argv X is removed `bare.count(X) - apply.count(X)` times; each removal must
    // be covered by an `omit`/`replace` ledger entry that MATCHES X. Matching is TOP-aware
    // (21D triage, the seed-6 for-loop false-positive): a loop body's site reports its argv
    // with the loop var as `TOP` (e.g. `replace enablesvc enable TOP`), but the bare trace
    // has the concrete per-member argvs (`enablesvc enable foxtrot`). A `TOP`-bearing
    // license is a WILDCARD that covers every concrete member of its (fully-converged,
    // hence replaced) loop — a partially-diverged loop disposes `run`, never `replace`, so
    // a wildcard `replace`/`omit` cannot mask a needed member (verified seed-6 loop-2).
    let licenses: Vec<&LedgerEntry> = outcome
        .ledger
        .iter()
        .filter(|e| e.disposition == "omit" || e.disposition == "replace")
        .collect();
    for (argv, removed_n) in bare.removed_relative_to(&apply) {
        // Exact (no-TOP) licenses are counted (one unit each). A TOP-wildcard license (a
        // loop body site) covers a removed member ONLY if that member's host-state is
        // CONVERGED — an INDEPENDENT cross-check (`trial.entity_states`) so the judge does
        // not blindly trust the engine's loop disposition: if the engine wrongly elided a
        // DIVERGED member under a wildcard `replace`, the host-state check still fires the
        // disaster (the wildcard cannot mask a needed mutation — adversarial hunt-list).
        let wildcard_ok = licenses
            .iter()
            .any(|e| e.argv.contains("TOP") && argv_matches(&e.argv, argv))
            && removed_line_is_converged(trial, argv);
        let exact = licenses.iter().filter(|e| &e.argv == argv).count();
        let covered = wildcard_ok || exact >= removed_n;
        if !covered {
            let wildcard_present = licenses
                .iter()
                .any(|e| e.argv.contains("TOP") && argv_matches(&e.argv, argv));
            return Verdict::Finding(Finding {
                class: FindingClass::UnderExecute,
                diagnosis: format!(
                    "UNDER-EXECUTE (disaster class): the command `{argv}` ran {removed_n} \
                     more time(s) in the bare book than in the apply, but no license covers \
                     it ({exact} exact; TOP-wildcard present={wildcard_present} but the \
                     removed entity is NOT converged in the host-state, so a wildcard cannot \
                     justify it). The engine dropped a required mutation. Licenses: {:?}",
                    licenses.iter().map(|e| &e.argv).collect::<Vec<_>>()
                ),
            });
        }
    }

    // `shimmed` is retained in the signature for callers/findings even though the gate-5
    // argv⊆bare check was removed (21D triage) — silence the unused-param lint without
    // widening the API churn.
    let _ = shimmed;
    Verdict::Clean
}

/// Does a ledger argv (which may contain `TOP` placeholders for unresolved words — e.g. a
/// loop body's loop-var position) MATCH a concrete trace argv? True iff same arity and
/// every position is either equal or `TOP` in the ledger. The TOP-as-wildcard rule lets a
/// loop's single `replace … TOP` site license its concrete per-member trace lines (21D
/// triage). A purely-exact ledger argv (no TOP) matches only itself.
fn argv_matches(ledger_argv: &str, trace_argv: &str) -> bool {
    let l: Vec<&str> = ledger_argv.split_whitespace().collect();
    let t: Vec<&str> = trace_argv.split_whitespace().collect();
    l.len() == t.len()
        && l.iter()
            .zip(t.iter())
            .all(|(lw, tw)| *lw == "TOP" || lw == tw)
}

/// Is the entity in a removed trace argv CONVERGED in the trial's host-state? The entity
/// is the LAST token of a generated mutator argv (`enablesvc enable foxtrot` ⇒ `foxtrot`;
/// `instpkg install -y alpha` ⇒ `alpha`). The independent host-state cross-check that
/// guards the TOP-wildcard license (21D triage): a wildcard `replace`/`omit` may only
/// justify removing a member whose fact the host holds — eliding a diverged member is an
/// under-execute regardless of what the engine's loop disposition claimed. A line whose
/// entity is unknown to the host-state (no matching record) is treated as NOT converged
/// (conservative — `kFAIL-perform`: an unrecognized removal is a finding, not a pass).
fn removed_line_is_converged(trial: &Trial, trace_argv: &str) -> bool {
    let Some(entity) = trace_argv.split_whitespace().last() else {
        return false;
    };
    trial
        .entity_states
        .iter()
        .any(|st| st.entity == entity && st.converged)
}

/// A small multiset of argv strings (the trace, where a command can repeat).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Multiset {
    counts: BTreeMap<String, usize>,
}

impl Multiset {
    fn from(lines: &[String]) -> Self {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for l in lines {
            *counts.entry(l.clone()).or_default() += 1;
        }
        Multiset { counts }
    }

    /// Elements in `self` beyond `other`'s counts (self \ other), flattened with
    /// multiplicity for display.
    fn difference(&self, other: &Multiset) -> Vec<String> {
        let mut out = Vec::new();
        for (k, &n) in &self.counts {
            let m = other.counts.get(k).copied().unwrap_or(0);
            for _ in m..n {
                out.push(k.clone());
            }
        }
        out
    }

    /// For each argv, how many MORE times it appears in `self` than in `other` (the count
    /// removed when going self→other). Only positive deltas.
    fn removed_relative_to<'a>(&'a self, other: &Multiset) -> Vec<(&'a String, usize)> {
        let mut out = Vec::new();
        for (k, &n) in &self.counts {
            let m = other.counts.get(k).copied().unwrap_or(0);
            if n > m {
                out.push((k, n - m));
            }
        }
        out
    }
}

/// Deterministically MINIMIZE a finding: drop book statements one at a time, keeping a
/// drop iff the divergence persists (the same finding class). Returns the minimized
/// trial. A statement is a top-level line of the book (the modeled subset has no nested
/// multi-line statements except the `for … done` one-liner the generator emits, so a
/// line-granular drop is sound). The `set -e`/shebang/trailing-echo lines are preserved.
#[must_use]
pub fn minimize(tools: &Tools, trial: &Trial, want: FindingClass, shimmed: &[String]) -> Trial {
    let mut current = trial.clone();
    let lines: Vec<String> = current.book.lines().map(str::to_string).collect();
    // Indices that are droppable candidates (skip shebang, set -e, the trailing echo).
    let droppable: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| {
            let t = l.trim();
            !t.is_empty()
                && !t.starts_with("#!")
                && t != "set -e"
                && !t.starts_with("echo generated-")
        })
        .map(|(i, _)| i)
        .collect();

    // Greedily try dropping each candidate; keep the drop if the finding persists.
    let mut removed: Vec<usize> = Vec::new();
    for &idx in &droppable {
        let candidate_book = rebuild_book(&lines, &{
            let mut r = removed.clone();
            r.push(idx);
            r
        });
        let mut candidate = current.clone();
        candidate.book = candidate_book;
        if reproduces(tools, &candidate, want, shimmed) {
            removed.push(idx);
            current.book = rebuild_book(&lines, &removed);
        }
    }
    current
}

/// Re-emit a book with the given line indices removed.
fn rebuild_book(lines: &[String], removed: &[usize]) -> String {
    let mut s = String::new();
    for (i, l) in lines.iter().enumerate() {
        if removed.contains(&i) {
            continue;
        }
        s.push_str(l);
        s.push('\n');
    }
    s
}

/// Does this (possibly-reduced) trial still reproduce the wanted finding class?
fn reproduces(tools: &Tools, trial: &Trial, want: FindingClass, shimmed: &[String]) -> bool {
    match run_trial(tools, trial) {
        Ok(outcome) => {
            matches!(judge(trial, &outcome, shimmed), Verdict::Finding(f) if f.class == want)
        }
        Err(_) => false,
    }
}

/// The set of shimmed command names a trial stamps (apply mocks + probe mocks) — gate-5
/// uses the apply-mock set to decide which `run` argvs are checkable.
#[must_use]
pub fn shimmed_apply_cmds(trial: &Trial) -> Vec<String> {
    trial.apply_cmds.clone()
}

/// Write a ready-to-adopt finding case-dir draft under `e2e/findings/<seed>-<slug>/`:
/// book / oracles / probe-results / mocks / a `DIAGNOSIS` with observed-vs-expected. The
/// findings dir is a DRAFT for the human to adopt into `e2e/cases/`; the harness never
/// edits the live corpus (`e2e corpus untouched`).
///
/// # Errors
/// Returns an [`std::io::Error`] if the draft dir cannot be created or a file cannot be
/// written.
pub fn emit_finding(
    findings_root: &Path,
    trial: &Trial,
    outcome: &RunOutcome,
    finding: &Finding,
) -> std::io::Result<PathBuf> {
    let dir = findings_root.join(format!("{}-{}", trial.seed, finding.class.slug()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("mocks"))?;
    std::fs::write(dir.join("book.sh"), &trial.book)?;
    for (name, text) in &trial.oracles {
        std::fs::write(dir.join(name), text)?;
    }
    std::fs::write(dir.join("probe-results.txt"), &outcome.probe_results)?;
    // Re-stamp ALL mocks so the finding case-dir is self-contained.
    stamp_mocks(&dir, trial)?;

    let mut diag = String::new();
    let _ = writeln!(
        diag,
        "# Finding {} (seed {})",
        finding.class.slug(),
        trial.seed
    );
    let _ = writeln!(diag, "\nshape: {:?}", trial.shape);
    let _ = writeln!(diag, "\n## Diagnosis\n{}", finding.diagnosis);
    let _ = writeln!(diag, "\n## Disposition ledger (--debug-argv)");
    for e in &outcome.ledger {
        let _ = writeln!(diag, "argv {} {} {}", e.leafid, e.disposition, e.argv);
    }
    let _ = writeln!(diag, "\n## Bare trace (book under mocks)");
    for l in &outcome.bare_trace {
        let _ = writeln!(diag, "ran: {l}");
    }
    let _ = writeln!(diag, "\n## Apply trace (apply artifact under mocks)");
    for l in &outcome.apply_trace {
        let _ = writeln!(diag, "ran: {l}");
    }
    let _ = writeln!(diag, "\n## Book\n{}", trial.book);
    std::fs::write(dir.join("DIAGNOSIS.md"), diag)?;
    Ok(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Locate the spike root from this crate's manifest dir (`crates/hostsim` → `spike`).
    fn spike_root() -> PathBuf {
        // CARGO_MANIFEST_DIR = .../spike/crates/hostsim
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.pop(); // crates
        p.pop(); // spike
        p
    }

    fn tools() -> Option<Tools> {
        Tools::locate(&spike_root()).ok()
    }

    #[test]
    fn rng_is_deterministic_in_seed() {
        let mut a = Rng::new(123);
        let mut b = Rng::new(123);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64(), "same seed ⇒ same stream");
        }
        let mut c = Rng::new(124);
        // Different seed ⇒ (almost surely) different first draw.
        assert_ne!(Rng::new(123).next_u64(), c.next_u64());
    }

    #[test]
    fn generate_is_deterministic_in_seed() {
        for seed in 0..50u64 {
            let a = generate(seed);
            let b = generate(seed);
            assert_eq!(a.book, b.book, "seed {seed}: book reproduces");
            assert_eq!(a.oracles, b.oracles, "seed {seed}: oracles reproduce");
        }
    }

    /// THE generator-validity pin (mandatory): every generated book is `dash -n`-clean
    /// across 200 seeds. A generator emitting invalid sh is OUR bug; this catches it
    /// before the binary ever sees it.
    #[test]
    fn generator_emits_only_dash_n_clean_books() {
        let Some(tools) = tools() else {
            // No binary/dash available (e.g. a docs-only CI) — skip, don't fail.
            eprintln!("skip: tools not located");
            return;
        };
        let mut bad = Vec::new();
        for seed in 0..200u64 {
            let trial = generate(seed);
            if let Err(e) = dash_n(&tools, &trial.book) {
                bad.push((seed, trial.shape, e, trial.book));
            }
        }
        assert!(
            bad.is_empty(),
            "generator emitted {} non-dash-n-clean book(s): {:#?}",
            bad.len(),
            bad
        );
    }

    /// Trace-capture round-trips a known fixture: a straight-line book of two distinct
    /// installs, with a host where both are diverged, must produce a bare trace with both
    /// installs (and, since diverged ⇒ run, an apply trace with both too).
    #[test]
    fn trace_capture_round_trips_known_fixture() {
        let Some(tools) = tools() else {
            eprintln!("skip: tools not located");
            return;
        };
        // Hand-build a deterministic trial: two diverged packages, no set -e.
        let trial = Trial {
            seed: 9_000_001,
            shape: Shape::StraightLine,
            book: "#!/bin/sh\ninstpkg install -y alpha\ninstpkg install -y bravo\necho generated\n"
                .into(),
            oracles: build_oracles(std::slice::from_ref(&KINDS[0]), false),
            apply_cmds: vec!["instpkg".into()],
            entity_states: vec![
                EntityState {
                    probe_cmd: "dorcprobe_package".into(),
                    entity: "alpha".into(),
                    converged: false,
                    guard_cmd: None,
                },
                EntityState {
                    probe_cmd: "dorcprobe_package".into(),
                    entity: "bravo".into(),
                    converged: false,
                    guard_cmd: None,
                },
            ],
            is_top_control: false,
        };
        let outcome = run_trial(&tools, &trial).expect("trial runs");
        assert!(
            outcome
                .bare_trace
                .iter()
                .any(|l| l == "instpkg install -y alpha"),
            "bare trace has alpha install: {:?}",
            outcome.bare_trace
        );
        assert!(
            outcome
                .bare_trace
                .iter()
                .any(|l| l == "instpkg install -y bravo"),
            "bare trace has bravo install: {:?}",
            outcome.bare_trace
        );
        // Both diverged ⇒ both run in apply too (no elision).
        assert_eq!(
            Multiset::from(&outcome.bare_trace),
            Multiset::from(&outcome.apply_trace),
            "diverged installs are not elided ⇒ apply == bare"
        );
        // …and the judge agrees it's clean.
        assert_eq!(
            judge(&trial, &outcome, &shimmed_apply_cmds(&trial)),
            Verdict::Clean
        );
    }

    /// A known-GOOD converged case passes the judge: a converged install is `replace`d
    /// (licensed) ⇒ absent from the apply trace ⇒ judged Clean.
    #[test]
    fn judge_passes_known_good_converged_elision() {
        let Some(tools) = tools() else {
            eprintln!("skip: tools not located");
            return;
        };
        let trial = Trial {
            seed: 9_000_002,
            shape: Shape::StraightLine,
            book: "#!/bin/sh\ninstpkg install -y alpha\necho generated\n".into(),
            oracles: build_oracles(std::slice::from_ref(&KINDS[0]), false),
            apply_cmds: vec!["instpkg".into()],
            entity_states: vec![EntityState {
                probe_cmd: "dorcprobe_package".into(),
                entity: "alpha".into(),
                converged: true, // converged ⇒ elided
                guard_cmd: None,
            }],
            is_top_control: false,
        };
        let outcome = run_trial(&tools, &trial).expect("trial runs");
        // alpha is converged ⇒ replaced ⇒ NOT in the apply trace.
        assert!(
            !outcome
                .apply_trace
                .iter()
                .any(|l| l == "instpkg install -y alpha"),
            "converged install elided from apply: {:?}",
            outcome.apply_trace
        );
        // …but IS in the bare trace, and the ledger licenses it (replace).
        assert!(
            outcome
                .bare_trace
                .iter()
                .any(|l| l == "instpkg install -y alpha")
        );
        assert!(
            outcome
                .ledger
                .iter()
                .any(|e| e.argv == "instpkg install -y alpha" && e.disposition == "replace"),
            "ledger licenses the elision: {:?}",
            outcome.ledger
        );
        assert_eq!(
            judge(&trial, &outcome, &shimmed_apply_cmds(&trial)),
            Verdict::Clean,
            "a licensed converged elision is Clean"
        );
    }

    /// THE MANDATORY planted-under-execute test: corrupt a known-good case's APPLY trace
    /// by deleting a REQUIRED (diverged ⇒ run, unlicensed) command, and assert the judge
    /// SCREAMS `UnderExecute`. A judge blind to the disaster class is worse than none.
    ///
    /// We plant the corruption at the JUDGE's input boundary (not by mutating the engine):
    /// take a real clean outcome, delete a `run`-disposition command from the apply trace,
    /// and re-judge. The deleted command has no omit/replace license ⇒ under-execute.
    #[test]
    fn judge_screams_on_planted_under_execute() {
        let Some(tools) = tools() else {
            eprintln!("skip: tools not located");
            return;
        };
        // A diverged install: it MUST run (no license to elide).
        let trial = Trial {
            seed: 9_000_003,
            shape: Shape::StraightLine,
            book: "#!/bin/sh\ninstpkg install -y bravo\necho generated\n".into(),
            oracles: build_oracles(std::slice::from_ref(&KINDS[0]), false),
            apply_cmds: vec!["instpkg".into()],
            entity_states: vec![EntityState {
                probe_cmd: "dorcprobe_package".into(),
                entity: "bravo".into(),
                converged: false, // diverged ⇒ MUST run
                guard_cmd: None,
            }],
            is_top_control: false,
        };
        let mut outcome = run_trial(&tools, &trial).expect("trial runs");
        // Sanity: the un-corrupted case is Clean, the ledger marks bravo `run`.
        assert_eq!(
            judge(&trial, &outcome, &shimmed_apply_cmds(&trial)),
            Verdict::Clean,
            "precondition: the honest case is Clean"
        );
        assert!(
            outcome
                .ledger
                .iter()
                .any(|e| e.argv == "instpkg install -y bravo" && e.disposition == "run"),
            "precondition: bravo is a `run` site (not licensed to elide): {:?}",
            outcome.ledger
        );
        // PLANT: delete the required command from the apply trace (simulate an engine that
        // wrongly dropped a needed mutation).
        outcome
            .apply_trace
            .retain(|l| l != "instpkg install -y bravo");
        let verdict = judge(&trial, &outcome, &shimmed_apply_cmds(&trial));
        match verdict {
            Verdict::Finding(f) => assert_eq!(
                f.class,
                FindingClass::UnderExecute,
                "the planted drop of a required command must be flagged UnderExecute, got: {f:?}"
            ),
            Verdict::Clean => {
                panic!("JUDGE BLIND TO THE DISASTER CLASS: a planted under-execute judged Clean")
            }
        }
    }

    /// A planted ⊤-control elision is flagged: a control trial whose apply trace drops a
    /// command (which must never happen under ⊤) is `TopControlElided`.
    #[test]
    fn judge_flags_top_control_elision() {
        // Pure judge test (no binary needed): construct an outcome by hand.
        let trial = Trial {
            seed: 9_000_004,
            shape: Shape::TopControl,
            book: "#!/bin/sh\neval 'echo hi'\ninstpkg install -y alpha\necho generated\n".into(),
            oracles: vec![],
            apply_cmds: vec!["instpkg".into()],
            entity_states: vec![],
            is_top_control: true,
        };
        let outcome = RunOutcome {
            probe_art: String::new(),
            apply_art: String::new(),
            probe_results: String::new(),
            ledger: vec![],
            bare_trace: vec!["instpkg install -y alpha".into()],
            apply_trace: vec![], // the control WRONGLY elided the install
            dorc_stderr: String::new(),
        };
        let v = judge(&trial, &outcome, &["instpkg".to_string()]);
        assert!(
            matches!(v, Verdict::Finding(ref f) if f.class == FindingClass::TopControlElided),
            "a ⊤-control that elided must be flagged: {v:?}"
        );
    }

    /// The judge flags an over-execute (apply ran something bare didn't).
    #[test]
    fn judge_flags_over_execute() {
        let trial = Trial {
            seed: 9_000_005,
            shape: Shape::StraightLine,
            book: "#!/bin/sh\necho generated\n".into(),
            oracles: vec![],
            apply_cmds: vec!["instpkg".into()],
            entity_states: vec![],
            is_top_control: false,
        };
        let outcome = RunOutcome {
            probe_art: String::new(),
            apply_art: String::new(),
            probe_results: String::new(),
            ledger: vec![],
            bare_trace: vec![],
            apply_trace: vec!["instpkg install -y phantom".into()], // not in bare
            dorc_stderr: String::new(),
        };
        let v = judge(&trial, &outcome, &["instpkg".to_string()]);
        assert!(
            matches!(v, Verdict::Finding(ref f) if f.class == FindingClass::OverExecute),
            "apply running a non-bare command is over-execute: {v:?}"
        );
    }

    /// A converged loop's TOP-wildcard `replace` license covers its concrete per-member
    /// trace lines (the 21D seed-6 fix): a loop body's site reports `replace enablesvc
    /// enable TOP`, the bare trace has `enablesvc enable foxtrot`/`golf` (both converged),
    /// and the judge must judge Clean — the wildcard + host-state-converged check licenses
    /// removing the members. Pure judge test (hand-built outcome).
    #[test]
    fn judge_top_wildcard_licenses_converged_loop_members() {
        let trial = Trial {
            seed: 9_000_006,
            shape: Shape::ForLoop,
            book: "#!/bin/sh\nfor x in foxtrot golf; do enablesvc enable \"$x\"; done\n".into(),
            oracles: vec![],
            apply_cmds: vec!["enablesvc".into()],
            entity_states: vec![
                EntityState {
                    probe_cmd: "dorcprobe_svc".into(),
                    entity: "foxtrot".into(),
                    converged: true,
                    guard_cmd: None,
                },
                EntityState {
                    probe_cmd: "dorcprobe_svc".into(),
                    entity: "golf".into(),
                    converged: true,
                    guard_cmd: None,
                },
            ],
            is_top_control: false,
        };
        let outcome = RunOutcome {
            probe_art: String::new(),
            apply_art: String::new(),
            probe_results: String::new(),
            // The loop site replaced, argv carries TOP for the loop var.
            ledger: vec![LedgerEntry {
                leafid: 0,
                disposition: "replace".into(),
                argv: "enablesvc enable TOP".into(),
            }],
            bare_trace: vec![
                "enablesvc enable foxtrot".into(),
                "enablesvc enable golf".into(),
            ],
            apply_trace: vec![], // both members elided (converged loop)
            dorc_stderr: String::new(),
        };
        assert_eq!(
            judge(&trial, &outcome, &["enablesvc".to_string()]),
            Verdict::Clean,
            "a converged loop's TOP-wildcard replace licenses its concrete members"
        );
    }

    /// THE adversarial guard on the wildcard (judge-blindness risk #1, the hunt-list): a
    /// TOP-wildcard `replace` must NOT mask the elision of a DIVERGED member. We plant an
    /// outcome where the engine (wrongly) replaced a loop with a diverged member `golf` and
    /// the diverged member is absent from the apply trace; the host-state cross-check must
    /// still scream `UnderExecute` — the wildcard cannot license eliding a needed mutation.
    #[test]
    fn judge_top_wildcard_does_not_mask_diverged_member_under_execute() {
        let trial = Trial {
            seed: 9_000_007,
            shape: Shape::ForLoop,
            book: "#!/bin/sh\nfor x in foxtrot golf; do enablesvc enable \"$x\"; done\n".into(),
            oracles: vec![],
            apply_cmds: vec!["enablesvc".into()],
            entity_states: vec![
                EntityState {
                    probe_cmd: "dorcprobe_svc".into(),
                    entity: "foxtrot".into(),
                    converged: true,
                    guard_cmd: None,
                },
                EntityState {
                    probe_cmd: "dorcprobe_svc".into(),
                    entity: "golf".into(),
                    converged: false, // DIVERGED — must run; eliding it is a disaster
                    guard_cmd: None,
                },
            ],
            is_top_control: false,
        };
        let outcome = RunOutcome {
            probe_art: String::new(),
            apply_art: String::new(),
            probe_results: String::new(),
            ledger: vec![LedgerEntry {
                leafid: 0,
                disposition: "replace".into(), // a (hypothetical) wrong full-loop replace
                argv: "enablesvc enable TOP".into(),
            }],
            bare_trace: vec![
                "enablesvc enable foxtrot".into(),
                "enablesvc enable golf".into(),
            ],
            // BUG planted: the diverged `golf` was elided too (only would-be-correct if all
            // members converged). foxtrot (converged) legitimately gone; golf (diverged) is
            // the disaster.
            apply_trace: vec![],
            dorc_stderr: String::new(),
        };
        let v = judge(&trial, &outcome, &["enablesvc".to_string()]);
        match v {
            Verdict::Finding(ref f) => assert_eq!(
                f.class,
                FindingClass::UnderExecute,
                "eliding a DIVERGED loop member under a wildcard must scream UnderExecute: {f:?}"
            ),
            Verdict::Clean => {
                panic!(
                    "WILDCARD MASKED A DISASTER: a diverged member elided under TOP judged Clean"
                )
            }
        }
    }
}
