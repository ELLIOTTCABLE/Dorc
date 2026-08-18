//! The sh-parity pin battery, pipeline tier (`30A` — the doctrine and its inventory).
//!
//! `rul-unsure-falls-toward-sh-parity` [human-typed 2026-08-16] says that where an engine choice is
//! unsure, the answer is what sh does. These pins are that rule made executable for the linguistic
//! behaviours that FORCE engine-implementation choices — load order, name binding, scoping,
//! conditional definition — and they live at this tier because each needs the whole pipeline to
//! answer: an assertion about `HelperIndex` alone cannot say whether a site kept its LICENSE.
//!
//! The oracle-tier half of the battery (resolution and closure custody, answerable from
//! `dorc_oracle` alone) is `crates/oracle/tests/sh_parity.rs`; the differential half is the
//! committed `floor28-*`/`floor30-*` manifests, measured once under `posh ∩ dash`
//! (`emitted-is-measure-once-ground-truth`).

#![expect(
    clippy::print_stderr,
    reason = "`support`'s selection reporter is compiled into every test binary that uses it; this one drives no trials and never calls it"
)]

mod support;

use std::collections::{BTreeMap, BTreeSet};

use dorc_analysis::effect::SkipClass;

/// One world, driven exactly as `cli::run` drives it, down to the per-site classes.
///
/// Copied in shape from `definition_frames.rs`'s `classes_of` rather than shared with it, because
/// these cases need to vary the ORACLE set and that one pins a single oracle constant. Tests earn
/// repetition (`spike/CLAUDE.md` Code style).
fn classes_of(oracles: &[&str], book_src: &str) -> Vec<SkipClass> {
    let mut interner = dorc_core::Interner::default();
    let mut arena = dorc_core::ProvArena::new();
    let idx = dorc_oracle::lift(&mut interner, oracles).value;
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracles
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();
    let verdicts = dorc_oracle::verdict::VerdictIndex::of(&mut interner, oracles);

    let parsed = dorc_syntax::parse(book_src).value;
    let cfg = dorc_analysis::cfg::build(&parsed).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed, &mut interner);
    let paths: Vec<String> = (0..oracles.len())
        .map(|i| format!("o{i}.oracle.sh"))
        .collect();
    let mut refs: Vec<&str> = oracles.to_vec();
    refs.push(book_src);
    let defs = dorc_cli::world::definition_table(
        &dorc_core::loadpath::Cwd::default(),
        &paths,
        &refs,
        dorc_analysis::funcenv::source_file_of_index(oracles.len()),
        &parsed,
    );
    let env = {
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        dorc_analysis::funcenv::analyze(&parsed, &cfg, &defs, &plane)
    };
    let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);

    let (classified, ..) = dorc_analysis::effect::classify_with_why_diags(
        &cfg,
        &value,
        &parsed,
        &idx,
        &checks,
        &verdicts,
        &BTreeMap::new(),
        &dorc_analysis::erase::ErasedSites::none(),
        &mut interner,
        &mut arena,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
        &mut dorc_analysis::certify::CertifierTrip::default(),
        live,
    );
    classified
        .value
        .into_iter()
        .map(|(_, class)| class)
        .collect()
}

/// An oracle that makes `hork tune` an ELIDABLE establish — the control every pin below needs, so
/// that a pin asserting a LOSS is asserting the loss of something demonstrably present.
const HORK_PLAIN: &str = "# dorc-lang/v0.2
hork__predict() {
   verb=$1; shift
   widget : sm.dorc.Widget = \"$1\"
   case $verb in
      tune) hork status -- \"$widget\"   : sm.dorc.Widget:\"$widget\"@tuned ;;
   esac
}
";

/// The same description, defined ONLY IF the name is free — the polyfill idiom, at oracle top level.
///
/// This is the shape `oracle/CLAUDE.md only-load-inert-sources-contribute` names as it discusses
/// inertness dying in literal: `command -v jq || jq() { … }`. As an oracle top level it is a
/// COMMAND, so the file is not load-inert.
const HORK_CONDITIONAL: &str = "# dorc-lang/v0.2
command -v hork__predict >/dev/null 2>&1 || hork__predict() {
   verb=$1; shift
   widget : sm.dorc.Widget = \"$1\"
   case $verb in
      tune) hork status -- \"$widget\"   : sm.dorc.Widget:\"$widget\"@tuned ;;
   esac
}
";

fn is_elidable_establish(classes: &[SkipClass]) -> bool {
    classes
        .iter()
        .any(|class| matches!(class, SkipClass::EstablishAmbient(_)))
}

/// The providers a dialect lift found in `src`, and the role headers it merely DETECTED.
fn lifted_and_detected(src: &str) -> (Vec<String>, Vec<String>) {
    let mut interner = dorc_core::Interner::default();
    let predicts = dorc_oracle::predict::lift_predicts(&mut interner, src).value;
    let lifted: Vec<String> = predicts
        .providers()
        .map(|provider| interner.resolve(provider).to_owned())
        .collect();
    let detected: Vec<String> = predicts
        .unlifted()
        .map(|found| found.name.clone())
        .collect();
    (lifted, detected)
}

/// `p-blessed-toplevel-conditional` (`30A` §2 P-green, its verify-first arm resolved by measurement)
/// — a host-conditional definition at oracle top level licenses NOTHING, and this is the shape of
/// today's refusal.
///
/// The sh fact underneath: `command -v X || X() { … }` binds `X` only on a host where `X` was not
/// already resolvable, so WHETHER that definition exists is a property of the landing host, not of
/// the text. That is measured, not argued — `floor28-funcdef-as-or-operand` ran this exact shape
/// under `posh ∩ dash` and both agreed the definition lands only in a free slot.
///
/// WHAT `30A` EXPECTED, AND WHAT IS TRUE. The doctrine's guess was that the binding lands can't-say
/// through the frame solver's conditional machinery. It does not: the frame solver models the BOOK's
/// conditionals and holds no opinion about an oracle file's. What holds the line is two independent
/// refusals, both pinned here, and neither is the May-binding the doctrine described — the
/// load-inertness gate refuses the FILE, and the dialect lift never sees the funcdef at all, because
/// `parse_file` recognizes a role header only as a TOP-LEVEL ITEM and a definition in the right
/// operand of `||` is skipped whole. It is not even DETECTED, so the marks-lost backstop
/// (`oracle::validate`) stays quiet about it too.
///
/// Why an engine choice depends on this: the blessing of read-only top-level commands
/// (`oracle/CLAUDE.md only-load-inert-sources-contribute`, "INERTNESS IS DYING IN LITERAL") makes
/// this file legal oracle text and retires the first refusal. Whoever lands it must not make the lift
/// see the funcdef WITHOUT also making the binding `May` — that combination is a wrong-elision route,
/// and it is what `p-x-blessed-toplevel-conditional` pins.
#[test]
fn a_host_conditional_oracle_definition_licenses_nothing() {
    let slugs: Vec<&str> = dorc_oracle::load_inert::lint_load_inert(HORK_CONDITIONAL)
        .iter()
        .map(|diag| diag.code.slug())
        .collect();
    assert_eq!(
        slugs,
        ["oracle-file-not-load-inert"],
        "refusal one: the conditional definition is a top-level command, so the file makes no \
         dialect claim"
    );
    assert!(
        dorc_oracle::load_inert::lint_load_inert(HORK_PLAIN).is_empty(),
        "control: the unconditional spelling of the same description is inert"
    );

    let (lifted, detected) = lifted_and_detected(HORK_CONDITIONAL);
    assert!(
        lifted.is_empty() && detected.is_empty(),
        "refusal two: a role funcdef in a `||` operand is skipped as part of one top-level item, so \
         it is neither lifted nor detected — lifted={lifted:?} detected={detected:?}"
    );
    let (plain_lifted, _) = lifted_and_detected(HORK_PLAIN);
    assert_eq!(
        plain_lifted,
        vec!["hork".to_owned()],
        "control: the unconditional spelling lifts a description"
    );

    let control = classes_of(&[HORK_PLAIN], "hork tune web\n");
    assert!(
        is_elidable_establish(&control),
        "control: the unconditional description must license, else the loss below is not a loss — \
         {control:?}"
    );
    let conditional = classes_of(&[HORK_CONDITIONAL], "hork tune web\n");
    assert!(
        !is_elidable_establish(&conditional),
        "the outcome both refusals produce: the site licenses nothing — {conditional:?}"
    );
}

/// `p-x-blessed-toplevel-conditional` — THE TARGET: once the construct is legal oracle text, the
/// definition is DESCRIBED and its binding is `May`, so it still licenses nothing.
///
/// Two conjuncts, and the pin needs both, because each alone is satisfiable in a way that misses the
/// point. "Licenses nothing" is already true (the pin above), but VACUOUSLY — the engine never read
/// the file, so the author's polyfill description is discarded rather than modelled, and that is the
/// value `rul-unsure-falls-toward-sh-parity` says to capture: sh binds this name conditionally, so
/// parity is a conditional binding, not silence. "Is described" alone would be a wrong-elision route,
/// because an unconditionally-read description licenses an elision against a body the host may never
/// have bound.
///
/// FAILS TODAY on the described half (measured: zero lifted rows, zero detected headers). It greens
/// only when a lane makes the lift see through the `||` AND the environment answer `May` at the same
/// time — which is the coupling the trigger's ruling owes.
#[test]
fn a_described_host_conditional_definition_is_may_bound() {
    // Setup outside the closure: a panic in there would read as the target still failing.
    let (lifted, _) = lifted_and_detected(HORK_CONDITIONAL);
    let conditional = classes_of(&[HORK_CONDITIONAL], "hork tune web\n");
    internal_tooling::xfail::xfail_until("p-x-blessed-toplevel-conditional", || {
        assert_eq!(
            lifted,
            vec!["hork".to_owned()],
            "the author's conditional description must be READ, not skipped"
        );
        assert!(
            !is_elidable_establish(&conditional),
            "and read as May-bound, so it licenses nothing — {conditional:?}"
        );
    });
}

// ---------------------------------------------------------------------------
// The intra-compound plurality measurement (`30A` §2 `p-x-intra-compound-plurality`, first half).

/// Stage one of a composed pipe: `otelcol --version`, described by an oracle that factors part of its
/// body into `_pat_of` — and DECLARES `_pat_of` itself.
const COMPOUND_STAGE_A: &str = "# dorc-lang/v0.2
_pat_of() {
   printf '%s\\n' \"$1\"
}
otelcol__predict() {
   case $1 in
      --version)
         collector : io.opentelemetry.Collector = \"otelcol\"
         _pat_of \"$1\"
         otelcol --version :? io.opentelemetry.Collector:\"otelcol\"@version
         ;;
   esac
}
";

/// Stage two: `grep -q`, described by a DIFFERENT author who happens to have factored their own body
/// into a helper of the SAME NAME with DIFFERENT bytes. Loaded second, so sh binds THIS `_pat_of` for
/// every caller — including stage one's body.
const COMPOUND_STAGE_B: &str = "# dorc-lang/v0.2
_pat_of() {
   printf '%s' \"$1\"
}
grep__predict() {
   while [ \"${1#-}\" != \"$1\" ]; do shift; done
   pat : sm.dorc.GrepMatch = \"$1\"
   _pat_of \"$pat\"
   grep -q -- \"$pat\" :? sm.dorc.GrepMatch:\"$pat\"@matched
}
";

/// The composed book both stages come from.
const COMPOUND_BOOK: &str = "otelcol --version | grep -q 0.155.0\n";

/// Whether `provider`'s predict SHIPS at its pipe-stage site in this world.
///
/// Drives the production seat (`dorc_cli::world::ship_predict_body`), whose closure step is
/// byte-identical to the composed-stage seat's — see
/// [`the_composed_stage_seat_consults_the_closure`], which is what makes that identity a fact rather
/// than a claim.
fn stage_ships(oracles: &[&str], provider_word: &str) -> bool {
    let mut interner = dorc_core::Interner::default();
    let checks: Vec<dorc_oracle::predict::PredictSet> = oracles
        .iter()
        .map(|src| dorc_oracle::predict::lift_predicts(&mut interner, src).value)
        .collect();
    let parsed = dorc_syntax::parse(COMPOUND_BOOK).value;
    let cfg = dorc_analysis::cfg::build(&parsed).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed, &mut interner);
    let paths: Vec<String> = (0..oracles.len())
        .map(|i| format!("o{i}.oracle.sh"))
        .collect();
    let srcs: Vec<String> = oracles.iter().map(|src| (*src).to_owned()).collect();
    let mut refs: Vec<&str> = oracles.to_vec();
    refs.push(COMPOUND_BOOK);
    let defs = dorc_cli::world::definition_table(
        &dorc_core::loadpath::Cwd::default(),
        &paths,
        &refs,
        dorc_analysis::funcenv::source_file_of_index(oracles.len()),
        &parsed,
    );
    let env = {
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        dorc_analysis::funcenv::analyze(&parsed, &cfg, &defs, &plane)
    };
    let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);
    // The book is the last source, so the HELPER index sees the book census exactly as `main` builds
    // it (`main.rs`'s `HelperIndex::build(&source_refs, source_refs.len() - 1)`).
    let helpers = dorc_oracle::closure::HelperIndex::build(&refs, Some(oracles.len()));

    let provider = interner.intern(provider_word);
    let stage = cfg
        .iter()
        .filter(|(_, node)| node.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .find_map(|(id, _)| {
            let argv = value.argv_values(id);
            let dorc_analysis::value::ValueOf::Literal(word0) = argv.first()? else {
                return None;
            };
            (*word0 == provider).then(|| {
                let operands: Vec<dorc_core::Symbol> = argv
                    .get(1..)
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|value| match value {
                        dorc_analysis::value::ValueOf::Literal(sym) => Some(*sym),
                        dorc_analysis::value::ValueOf::Top(_) => None,
                    })
                    .collect();
                (id, operands)
            })
        });
    assert!(
        stage.is_some(),
        "{provider_word} has no pipe-stage command node in the composed book"
    );
    let Some((node, argv)) = stage else {
        return false;
    };

    dorc_cli::world::ship_predict_body(
        &srcs, &helpers, &checks, &interner, provider, &argv, node, live,
    )
    .is_some()
}

/// THE MEASUREMENT `30A` §2 ordered before `p-x-intra-compound-plurality` could be pinned: does a
/// cross-custody plural helper LICENSE anything under a composed predict ship?
///
/// It does not, and the shape of the refusal is the interesting part. Two authors each factored their
/// own predict body into a helper of the same name with different bytes. sh binds the LAST one for
/// every caller (`28R:rul-resolution-matches-shell-loading`, measured under `posh ∩ dash` by
/// `floor28-load-order-last-definition-wins`), so stage B's body resolves into its OWN custody and
/// ships, while stage A's resolves into somebody else's — which `rul-vouch-reaches-own-custody-only`
/// suspends — and ships nothing. An unshippable stage refuses the WHOLE compound
/// (`connected_check_pipes`, pinned below), so the pipe runs. No license leaks.
///
/// Why an engine choice depends on it: this is the world `p-x-intra-compound-plurality` targets. The
/// forfeit is real and named (`FORFEITS:forfeit-helper-plurality-withhold`, shape (a)) — two
/// well-written oracle packages that merely picked the same private helper name cost the compound its
/// probe. Capturing it needs PER-SEGMENT environments, which is the emission planner's job; what must
/// never happen in the meantime is the other resolution, where the compound ships with one author's
/// helper standing in for the other's.
#[test]
fn a_cross_custody_plural_helper_ships_no_composed_stage() {
    assert!(
        stage_ships(&[COMPOUND_STAGE_A], "otelcol"),
        "control: stage A ships when it is the only author of its helper"
    );
    assert!(
        stage_ships(&[COMPOUND_STAGE_B], "grep"),
        "control: stage B ships alone too"
    );

    let both = [COMPOUND_STAGE_A, COMPOUND_STAGE_B];
    assert!(
        !stage_ships(&both, "otelcol"),
        "stage A's helper now resolves, by sh's last-wins, into stage B's file: the body that would \
         serve A's description is somebody else's utterance, so A ships nothing"
    );
    assert!(
        stage_ships(&both, "grep"),
        "and the ASYMMETRY is the finding: B's resolution lands in its own custody, so B still ships \
         — which is why the compound needs per-segment environments rather than one more refusal"
    );
}

/// A composed compound whose participant cannot ship refuses WHOLE — every stage becomes an orphan and
/// runs. The second half of the measurement above, at the seat that owns the decision.
#[test]
fn one_unshippable_stage_refuses_the_whole_compound() {
    use dorc_analysis::effect::FactKey;
    use dorc_core::{Context, EntityRef, KindId, OpaqueToken, SelectorId};

    let mut interner = dorc_core::Interner::default();
    let ast = dorc_syntax::parse(COMPOUND_BOOK).value;
    let cfg = dorc_analysis::cfg::build(&ast).value;
    let value = dorc_analysis::value::analyze(&cfg, &ast, &mut interner);
    let otelcol = interner.intern("otelcol");
    let grep = interner.intern("grep");
    let matched = SelectorId(interner.intern("matched"));
    let kind = KindId(interner.intern("grepmatch"));
    let mut stage_nodes = BTreeMap::new();
    let mut classes = Vec::new();
    for (id, node) in cfg.iter() {
        if node.kind != dorc_analysis::cfg::CfgNodeKind::Command {
            continue;
        }
        let argv = value.argv_values(id);
        let Some(dorc_analysis::value::ValueOf::Literal(word0)) = argv.first() else {
            continue;
        };
        let word0 = *word0;
        if word0 != otelcol && word0 != grep {
            continue;
        }
        stage_nodes.insert(word0, id);
        classes.push((
            id,
            SkipClass::QueryResolvable {
                fact: FactKey {
                    kind,
                    entity: EntityRef::Operand(OpaqueToken(word0)),
                    selector: matched,
                    context: Context::HostDefault,
                },
                valid: true,
            },
        ));
    }
    assert_eq!(stage_nodes.len(), 2, "both pipe stages were found");

    // The ONE difference from a shippable world: stage A's ship is `None`, which is exactly what a
    // `ClosureDenial` produces at the composed-stage seat.
    let ship = |_node, provider, _argv: &[dorc_core::Symbol]| {
        (provider != otelcol).then(|| dorc_plan::StageShip {
            sh: "stub__predict() { :; }".to_owned(),
            produces_real_stdout: true,
        })
    };
    let pipes = dorc_plan::connected_check_pipes(&ast, &cfg, &value, &classes, ship);
    let governor = *stage_nodes.get(&grep).expect("stage B is the governor");
    let member = *stage_nodes.get(&otelcol).expect("stage A is the member");
    assert!(
        pipes.governing_composed(governor).is_none(),
        "no composed probe ships when one participant cannot"
    );
    assert!(
        pipes.is_orphan_stage(governor) && pipes.is_orphan_stage(member),
        "both stages orphan ⇒ both run (can't-say ⇒ run), never a partial compound"
    );
}

/// THE FENCE that joins the two halves above: the composed-stage ship seat really does consult the
/// closure, so a `ClosureDenial` really does become the `None` that refuses a compound.
///
/// `ship_predict_stage` is private to the `dorc` binary, so no test can call it. Lexical, like this
/// workspace's other cross-crate fences (`plan::erase`'s `licence_mint_has_exactly_one_caller`),
/// because the property is "this seat asks that question", which no type bound expresses. If the seat
/// is renamed or stops consulting the closure, the measurement above stops composing and this says so.
#[test]
fn the_composed_stage_seat_consults_the_closure() {
    let main_rs = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("main.rs");
    let text = std::fs::read_to_string(&main_rs).expect("the driver's source is readable");
    let start = text
        .find("fn ship_predict_stage(")
        .expect("the composed-stage ship seat is named `ship_predict_stage`");
    let body = text.get(start..).unwrap_or_default();
    let end = body
        .find("\n}\n")
        .expect("a top-level fn closes at column zero under rustfmt");
    let seat = body.get(..end).unwrap_or_default();
    assert!(
        seat.contains("helpers.closure_for("),
        "the composed-stage seat must consult the closure, or a contested composition ships"
    );
    assert!(
        seat.contains(".ok()?"),
        "and a denial must become `None` — which is what refuses the compound"
    );
}

/// `p-x-intra-compound-plurality` — THE TARGET: a composed compound whose two participants need
/// same-named, differing-bytes helpers gets PER-SEGMENT environments, so both stages ship.
///
/// The sh fact this rests on, and why per-segment is the shape: sh binds a name once per
/// environment, so two bodies under one name cannot both be live at one program point — but a
/// SUBSHELL is its own environment, and a pipeline stage already runs in one. That is what makes the
/// capture available at all rather than a wish; `floor28-subshell-scoped-re-source` measured the
/// scoping under `posh ∩ dash`, and `floor28-load-order-last-definition-wins` measured the binding
/// rule the plurality collides with. Whether the planner spells it as explicit per-segment subshells
/// or as alpha-rename is `28Q:pin-emission-planner-universal`'s call; either satisfies this pin,
/// which is why the assertion is on the OUTCOME (both stages ship) and not on the emitted shape.
///
/// FAILS TODAY on stage A, measured by [`a_cross_custody_plural_helper_ships_no_composed_stage`]
/// above: A's helper resolves by last-wins into B's custody with differing bytes, so its license
/// suspends and the whole compound refuses. Safe, and a real forfeit
/// (`FORFEITS:forfeit-helper-plurality-withhold` shape (a)) — two well-written packages that merely
/// picked the same private helper name cost the compound its probe.
#[test]
fn both_participants_of_a_plural_compound_ship() {
    // Setup outside the closure: a panic in there would read as the target still failing.
    let both = [COMPOUND_STAGE_A, COMPOUND_STAGE_B];
    let a = stage_ships(&both, "otelcol");
    let b = stage_ships(&both, "grep");
    internal_tooling::xfail::xfail_until("p-x-intra-compound-plurality", || {
        assert!(
            a && b,
            "each participant needs its OWN helper live where its own body runs; per-segment \
             environments are what make that spellable — otelcol ships: {a}, grep ships: {b}"
        );
    });
}

// ---------------------------------------------------------------------------
// `p-x-definition-grade-keying` (`30A` §2) — one file, one role, two frames.

/// The role the two cells below redefine across frames, in one file.
const REKEY_ROLE: &str = "wombat__is_converged";

/// A book that is its own oracle (`cli/CLAUDE.md the-book-is-a-definition-source`) and defines one
/// role TWICE across frames — the `unset -f`-then-redefine shape `contest28-unset-f-blesses-elision`
/// establishes as blessed rather than contested.
const REKEY_BOOK: &str = "wombat__is_converged() {\n   wombat cmp -- \"$1\"\n}\n\
                          wombat sync a\n\
                          unset -f wombat__is_converged\n\
                          wombat__is_converged() {\n   wombat cmp --strict -- \"$1\"\n}\n\
                          wombat sync b\n";

/// The `wombat` command sites of [`REKEY_BOOK`], in the order a shell reaches them, plus the solved
/// environment they are asked about.
fn rekey_world() -> (
    Vec<dorc_analysis::cfg::CfgNodeId>,
    dorc_analysis::funcenv::FuncEnv,
    dorc_analysis::funcenv::DefinitionTable,
) {
    let mut interner = dorc_core::Interner::default();
    let parsed = dorc_syntax::parse(REKEY_BOOK).value;
    let cfg = dorc_analysis::cfg::build(&parsed).value;
    let value = dorc_analysis::value::analyze(&cfg, &parsed, &mut interner);
    let defs = dorc_cli::world::definition_table(
        &dorc_core::loadpath::Cwd::default(),
        &[],
        &[REKEY_BOOK],
        dorc_analysis::funcenv::source_file_of_index(0),
        &parsed,
    );
    let env = {
        let plane = dorc_analysis::funcenv::SourceLiteralPlane::new(&value, &interner);
        dorc_analysis::funcenv::analyze(&parsed, &cfg, &defs, &plane)
    };
    // `sync`, not merely `wombat`: the two funcdef BODIES also hold `wombat` command nodes, and the
    // sites this asks about are the book's own top-level calls.
    let wombat = interner.intern("wombat");
    let sync = interner.intern("sync");
    let sites: Vec<_> = cfg
        .iter()
        .filter(|(_, node)| node.kind == dorc_analysis::cfg::CfgNodeKind::Command)
        .filter(|(id, _)| {
            let argv = value.argv_values(*id);
            let literal = |slot: usize| match argv.get(slot) {
                Some(dorc_analysis::value::ValueOf::Literal(word)) => Some(*word),
                _ => None,
            };
            literal(0) == Some(wombat) && literal(1) == Some(sync)
        })
        .map(|(id, _)| id)
        .collect();
    assert_eq!(
        sites.len(),
        2,
        "the book calls the tool twice, once per frame"
    );
    (sites, env, defs)
}

/// `p-x-definition-grade-keying`'s PRECONDITION, and green: the function environment names a
/// DIFFERENT definition at each of the two frames, within one file.
///
/// The sh fact: a redefinition rebinds the name from that point on, and `unset -f` between them
/// makes the second an override rather than a contest. So a shell running this book calls two
/// different bodies at the two sites, and `visibility-is-full-positional` says the engine must
/// answer from the definition live AT each site. That much works — the identity is
/// `DefinitionId::at(file, span)`, so two definitions in one file are distinct by construction.
/// Pinning it separately matters because the xfail below would otherwise be ambiguous about WHICH
/// half is missing.
#[test]
fn the_environment_names_a_definition_per_frame_within_one_file() {
    use dorc_core::LiveDefinition;

    let (sites, env, defs) = rekey_world();
    let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);
    let named: Vec<LiveDefinition> = sites
        .iter()
        .map(|site| live.definition_before(*site, REKEY_ROLE))
        .collect();
    let [first, second] = named.as_slice() else {
        return;
    };
    assert!(
        matches!(first, LiveDefinition::Live(_)) && matches!(second, LiveDefinition::Live(_)),
        "both frames bind something — {named:?}"
    );
    assert_ne!(
        first, second,
        "and they are DIFFERENT definitions: a shell would run two different bodies here"
    );
}

/// `p-x-definition-grade-keying` — THE TARGET: at each of those two frames, the derived row that
/// ANSWERS is the one the named definition produced.
///
/// Why an engine choice depends on it: the environment already names the right definition (pinned
/// above), so everything downstream turns on how derived rows are KEYED — and the forfeit is the
/// whole authored-in-book override idiom, where an admin who overrides a verdict mid-book in the
/// blessed spelling gets nothing from either body.
///
/// HALF LANDED, and the surviving half is a different question from the one this pin was minted
/// against. The KEYING is definition-grade now: a row carries the id of the definition its own lift
/// read, `answering_row` compares ids, and the `(file, role name)` join — with the "ambiguous"
/// state that made a within-file pair withhold at BOTH frames — is gone (`28Q` §1.1's repair).
/// What still withholds at the FIRST site is lift ARITY: `PredictSet`/`VerdictSet` keep one row per
/// `(file, role)`, so the earlier definition produces no row for any frame to find. Withholding is
/// the safe direction, and the second site now answers where it previously did not.
///
/// So this greens on a per-DEFINITION lift, not on any further keying work.
#[test]
fn a_within_file_plural_role_answers_per_definition() {
    // Setup outside the closure: a panic in there would read as the target still failing.
    let (sites, env, defs) = rekey_world();
    let live = dorc_analysis::funcenv::LiveDefinitions::new(&env, &defs);
    let mut interner = dorc_core::Interner::default();
    // The REAL lift, so the arity gap is measured rather than modelled.
    let verdicts = dorc_oracle::verdict::VerdictSet::lift(&mut interner, REKEY_BOOK).value;
    let rows: Vec<dorc_core::Span> = verdicts
        .providers()
        .filter_map(|p| verdicts.get(p).map(|v| v.span))
        .collect();
    let answers: Vec<Option<usize>> = sites
        .iter()
        .map(|site| {
            dorc_core::answering_row(live.definition_before(*site, REKEY_ROLE), rows.len(), |i| {
                rows.get(i)
                    .map(|span| dorc_analysis::funcenv::row_definition(0, *span))
            })
        })
        .collect();
    internal_tooling::xfail::xfail_until("p-x-definition-grade-keying", || {
        assert_eq!(
            answers,
            vec![Some(0), Some(1)],
            "each frame must find ITS OWN definition's row — the lift owes one row per definition, \
             and today it keeps one per (file, role), so the earlier body produced none: {answers:?}"
        );
    });
}

// ---------------------------------------------------------------------------
// `p-zero-munge-happy-corpus` — the output-quality RATCHET (`30A` §1 `d5-quality-is-a-ratchet`).

/// The cases whose committed artifacts are SUPPOSED to carry munged names, because each exists to
/// witness a defensive or collision world.
///
/// Growing this list is a REVIEWED ACT, and the test says so in its own failure message: a new entry
/// means one more book whose output went defensive-ugly, and
/// `rul-happy-path-is-a-closed-set` makes idiomatic emission a gradual-enhancement path ON TOP of the
/// defensive floor — so a case sliding into the floor is a regression until somebody says otherwise.
/// Two-way, like every allow-list in this tree: a listed case that stopped munging has stopped
/// witnessing what it was listed for.
const MUNGE_WITNESS_CASES: &[&str] = &[
    "emit30-definition-vector-munges-everything.loom",
    "emit30-two-live-verdicts-under-one-name.loom",
];

/// Every munged emitted name in `text`, using the engine's OWN role vocabulary to decide.
///
/// A munge is `<authored role name>_h<8 hex>` (`plan`'s `short_digest`), so the test is: strip the
/// suffix and ask `dorc_oracle::reserved::role_family` whether what is left is a role name. Reading it
/// that way rather than from a hand-written suffix list means the detector tracks the vocabulary
/// instead of drifting from it.
fn munged_names(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (at, _) in text.char_indices() {
        let Some(rest) = text.get(at..) else { continue };
        if !rest.starts_with("_h") {
            continue;
        }
        let Some(digest) = rest.get(2..10) else {
            continue;
        };
        if digest.len() != 8 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
            continue;
        }
        let head = text.get(..at).unwrap_or_default();
        let start = head
            .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .map_or(0, |boundary| boundary.saturating_add(1));
        let Some(authored) = head.get(start..) else {
            continue;
        };
        if dorc_oracle::reserved::role_family(authored).is_some() {
            out.insert(format!("{authored}_h{digest}"));
        }
    }
    out
}

/// Every committed artifact text in the corpus: the dir cases' `expected.out` and the loom cases'
/// whole text (their transcripts live inside them).
fn committed_artifacts() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for root in support::case_roots() {
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
        paths.sort();
        for path in paths {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            if name.contains(".sync-conflict-") {
                continue; // sync residue is never a case
            }
            if path.is_dir() {
                let golden = path.join("expected.out");
                if let Ok(text) = std::fs::read_to_string(&golden) {
                    out.push((name.clone(), text));
                }
                let inner = path.join(format!("{name}.loom"));
                if let Ok(text) = std::fs::read_to_string(&inner) {
                    out.push((format!("{name}.loom"), text));
                }
            } else if std::path::Path::new(&name)
                .extension()
                .is_some_and(|ext| ext == "loom")
                && let Ok(text) = std::fs::read_to_string(&path)
            {
                out.push((name, text));
            }
        }
    }
    out.sort();
    out
}

/// `p-zero-munge-happy-corpus` — the happy-path corpus emits ZERO munged names.
///
/// This is `d5-quality-is-a-ratchet`: output-idiomatic-quality pinned MECHANICALLY rather than by
/// taste. `rul-happy-path-is-a-closed-set` [human-typed 2026-08-16] makes defensive munge-everything
/// the floor and idiomatic emission a gradual path above it — which means the floor is exactly what a
/// regression looks like, and a regression into it is invisible to a golden diff (the golden simply
/// records the new, uglier bytes). So the assertion is over the whole committed corpus at once, with
/// the deliberate witnesses enumerated.
///
/// Non-vacuity is load-bearing here: if the detector matched nothing, this test would pass over an
/// entirely munged corpus, so it also demands that every enumerated witness IS detected.
#[test]
fn the_happy_path_corpus_emits_no_munged_names() {
    let artifacts = committed_artifacts();
    assert!(
        !artifacts.is_empty(),
        "discovery floor: no committed artifact was found, so this ratchet proves nothing"
    );

    let mut unlisted: Vec<String> = Vec::new();
    let mut covered: BTreeSet<&str> = BTreeSet::new();
    for (case, text) in &artifacts {
        let munged = munged_names(text);
        if munged.is_empty() {
            continue;
        }
        if let Some(listed) = MUNGE_WITNESS_CASES.iter().find(|listed| *listed == case) {
            covered.insert(listed);
            continue;
        }
        for name in munged {
            unlisted.push(format!("{case}: {name}"));
        }
    }

    assert!(
        unlisted.is_empty(),
        "{} munged name(s) in {} case(s) that are not enumerated munge witnesses. A book whose \
         output went defensive is a QUALITY REGRESSION under `rul-happy-path-is-a-closed-set`, not a \
         golden to re-bless: find what stopped being enumerable. Adding to \
         `MUNGE_WITNESS_CASES` is a reviewed act, never a fix.\n  {}",
        unlisted.len(),
        artifacts.len(),
        unlisted.join("\n  ")
    );
    let stale: Vec<&&str> = MUNGE_WITNESS_CASES
        .iter()
        .filter(|listed| !covered.contains(**listed))
        .collect();
    assert!(
        stale.is_empty(),
        "enumerated munge witness(es) carry no munged name, so either they stopped witnessing or the \
         detector stopped detecting — and a detector that matches nothing would pass this test over a \
         wholly munged corpus: {stale:?}"
    );
}
