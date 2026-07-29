//! `validate` — the book-free oracle-side validation surface (`27S:seam-oracle-validate-factoring`).
//!
//! The oracle-lane lints the cli used to run INLINE in `run()` — the effect-map lift, the per-file
//! check-dialect lift, the dual-peel + fold-entry coherence fail-fast, the munge-reservation lint,
//! and the marker gate — factored into ONE book-free entry emitting structured [`Diag`]s. This is
//! BOTH the cli's oracle-validation source (it routes the stages to stderr) AND the lint lane's
//! `rung-oracle-solo` source (`27R` §8b — oracle files linted with no book present). Pure given the
//! interner (`inv-determinism`): no book, no world, no probe; ordered maps + source order.

use std::collections::BTreeMap;

use dorc_aid::Diag;
use dorc_aid::diag::{DiagCode, WrapperEntryIncoherent, WrapperPeelIncoherent};
use dorc_core::{Interner, Symbol};

use crate::predict::Predict;

/// One validation stage's diagnostics (`27S` §4). `stage` is the stable label the cli's `report_at`
/// prefixes (`oracle`/`check`/`wrapper`/`reserved`/`marker`); `file` is the index into the caller's
/// oracle slice for a per-file stage (so the caller resolves `(path, src)` for the region frame), or
/// `None` for a unit-wide stage.
#[derive(Debug, Clone)]
pub struct StageDiags {
    /// The stable stage label.
    pub stage: &'static str,
    /// The per-file source index, or `None` for a unit-wide stage.
    pub file: Option<usize>,
    /// The stage's diagnostics.
    pub diags: Vec<Diag>,
}

/// The whole oracle-validation result (`27S:seam-oracle-validate-factoring`): the per-stage diags
/// plus the pre-network wrapper-coherence verdict the cli fast-fails on
/// (`27C:rul-fold-entry-coherence-failfast` / `273` §5). The fail-fast moves NOWHERE — the cli just
/// reads [`wrapper_incoherent`](Self::wrapper_incoherent) here instead of a bespoke inline call.
#[derive(Debug, Clone)]
pub struct OracleValidation {
    /// The per-stage diagnostics, in stable emission order.
    pub stages: Vec<StageDiags>,
    /// Whether any wrapper's declarations genuinely contradict (the pre-network fail-fast trigger).
    pub wrapper_incoherent: bool,
}

/// Run every book-free oracle-side lint over `oracles` (source bytes), returning the structured
/// stage-diags + the coherence verdict — the rung-oracle-solo surface (`27R` §8b). Deterministic.
/// Empty per-file stages are omitted (an empty stage renders nothing either way, so a caller's
/// `report_at`/finding-lowering is byte-identical); the interner-mutating lift still runs for each.
#[must_use]
pub fn validate(interner: &mut Interner, oracles: &[&str]) -> OracleValidation {
    let mut stages = Vec::new();

    // The effect-map lift over the whole unit (stage `oracle`).
    stages.push(StageDiags {
        stage: "oracle",
        file: None,
        diags: crate::lift(interner, oracles).diags,
    });

    // The per-file check-dialect lift (stage `check`): a check body using a construct outside the
    // check dialect is a lift failure that frames into THIS oracle's source. BOTH role-lifts run
    // here: a verdict body abandons its funcdef exactly as a predict body does (one `parse_block`
    // give-up), so routing only the predict lift left every verdict-body give-up silent — the file
    // parsed, `dorc lint` said clean, and the body's binds/marks were inert (`26G` F3).
    for (i, src) in oracles.iter().enumerate() {
        let mut diags = crate::predict::lift_predicts(interner, src).diags;
        diags.extend(crate::predict::lift_verdicts_converged(interner, src).diags);
        if !src.contains("__") {
            diags.extend(crate::predict::lint_mark_subset(src));
        }
        diags.extend(unlifted_role_fns(interner, src));
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "check",
                file: Some(i),
                diags,
            });
        }
    }

    // The tolerance CORROBORATION lints, per file (stage `tolerance`). Both directions of `27C` §6
    // (a `safe-across user` over a body that visibly reads identity; a body that visibly reads
    // identity with no vouch at all), recognize-never-license: neither blocks anything, both ask.
    // They belong here rather than at either consumer because both consumers want them — the author
    // hot loop through `dorc lint`, and the plan lane through `report_at`.
    for (i, src) in oracles.iter().enumerate() {
        let verdicts = crate::predict::lift_verdicts_converged(interner, src).value;
        let mut diags = Vec::new();
        for provider in verdicts.providers() {
            let Some(verdict) = verdicts.get(provider) else {
                continue;
            };
            // The lift's OWN diags are the lint lane's already; taking them again here would
            // double-report every `safe-across` malformation on the oracle-solo rung.
            let (vouch, _) = crate::entry::lift_tolerance(verdict);
            diags.extend(crate::entry::corroborate_tolerance_over_identity(
                &vouch,
                verdict,
                interner,
                verdict.name_span,
            ));
            diags.extend(crate::entry::hint_heavy_context_no_vouch(
                &vouch,
                verdict,
                interner,
                verdict.name_span,
            ));
        }
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "tolerance",
                file: Some(i),
                diags,
            });
        }
    }

    // The lend-map dimension lint, per file (stage `lend`). `derive_lend_map`'s diags had no
    // consumer at all: the wrapper index takes its VALUE and drops them, so an unknown dimension
    // token silently walled the dimension it meant to answer.
    for (i, src) in oracles.iter().enumerate() {
        let lend_maps = crate::wrapper::lift_lend_map_set(interner, src).value;
        let mut diags = Vec::new();
        for provider in lend_maps.providers() {
            if let Some(lend_map) = lend_maps.get(provider) {
                diags.extend(crate::wrapper::derive_lend_map(lend_map).1);
            }
        }
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "lend",
                file: Some(i),
                diags,
            });
        }
    }

    // The authored axis-invariance index (stage `carry`): its netns contradiction. Lifted over the
    // whole unit here rather than inside the wrapped-site walk, which returns early when no peeling
    // wrapper is loaded — so a kind owner's false invariance line was silent until some OTHER file
    // happened to declare a wrapper.
    stages.push(StageDiags {
        stage: "carry",
        file: None,
        diags: crate::carry::InvarianceIndex::lift(interner, oracles).1,
    });

    // Dual-peel + fold-entry coherence (stage `wrapper`): the pre-network fail-fast.
    let (wrapper_diags, wrapper_incoherent) = peel_and_entry_coherence(interner, oracles);
    stages.push(StageDiags {
        stage: "wrapper",
        file: None,
        diags: wrapper_diags,
    });

    // The munge-reservation lint over the whole unit (stage `reserved`).
    stages.push(StageDiags {
        stage: "reserved",
        file: None,
        diags: crate::reserved::lint_oracle_reserved_names(interner, oracles),
    });

    // The marker gate, per file (stage `marker`).
    for (i, src) in oracles.iter().enumerate() {
        let diags = crate::marker::check_dialect_marker(interner, src);
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "marker",
                file: Some(i),
                diags,
            });
        }
    }

    for (i, src) in oracles.iter().enumerate() {
        let diags = crate::load_inert::lint_load_inert(src);
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "load",
                file: Some(i),
                diags,
            });
        }
    }

    OracleValidation {
        stages,
        wrapper_incoherent,
    }
}

/// Dual-peel + fold-entry coherence over the whole oracle unit (`273` §5 / `27C:rul-fold-entry-
/// coherence-failfast`). For every provider authoring BOTH a peeling `__predict` and a `__lend_map`,
/// assert their `"$@"` reach the SAME tail over a set of canonical probe argvs; where a wrapper also
/// authors an `__enter`, assert its argv flow agrees with the fold. A disagreement is genuine static
/// incoherence (declarations-genuinely-contradict) ⇒ a loud Error + `true`. Mints NO license — an
/// error is the safe direction (`inv-determinism`: argument-order walk, ordered maps).
fn peel_and_entry_coherence(interner: &mut Interner, oracles: &[&str]) -> (Vec<Diag>, bool) {
    use crate::entry::{check_entry_coherence, lift_entry_set};
    use crate::predict::lift_predicts;
    use crate::wrapper::{check_peel_coherence, detect_peel, lift_lend_map_set};

    // Canonical probe argvs — flags (`-a`/`-b`) exercise the flag-strip loops, then a guest +
    // operand. A coherent pair agrees on ALL; an incoherent pair disagrees on ≥1.
    const CANON: [&[&str]; 3] = [&["g"], &["-a", "g"], &["-a", "-b", "g", "x"]];

    let mut predicts: BTreeMap<Symbol, Predict> = BTreeMap::new();
    let mut lend_maps: BTreeMap<Symbol, Predict> = BTreeMap::new();
    let mut entries: BTreeMap<Symbol, Predict> = BTreeMap::new();
    for src in oracles {
        let ps = lift_predicts(interner, src).value;
        for p in ps.providers() {
            if let Some(c) = ps.get(p)
                && detect_peel(c).is_some()
            {
                predicts.entry(p).or_insert_with(|| c.clone());
            }
        }
        let ls = lift_lend_map_set(interner, src).value;
        for p in ls.providers() {
            if let Some(c) = ls.get(p) {
                lend_maps.entry(p).or_insert_with(|| c.clone());
            }
        }
        let es = lift_entry_set(interner, src).value;
        for p in es.providers() {
            if let Some(c) = es.get(p) {
                entries.entry(p).or_insert_with(|| c.clone());
            }
        }
    }

    let mut diags = Vec::new();
    // Fold-entry coherence (`27C:rul-fold-entry-coherence-failfast`): where a wrapper authors BOTH a
    // `lend_map` and an `__enter`, their argv flow must agree by STATIC sh-structure.
    for (provider, lend) in &lend_maps {
        let Some(enter) = entries.get(provider) else {
            continue;
        };
        if let Some(inc) = check_entry_coherence(enter, lend) {
            diags.push(Diag::new(
                DiagCode::WrapperEntryIncoherent(WrapperEntryIncoherent {
                    wrapper: interner.resolve(*provider).to_owned(),
                    entry_shifts: inc.entry_shifts.to_string(),
                    lend_shifts: inc.lend_shifts.to_string(),
                }),
                enter.name_span,
            ));
        }
    }
    for (provider, predict) in &predicts {
        let Some(lend) = lend_maps.get(provider) else {
            continue;
        };
        for argv in CANON {
            if let Some(inc) = check_peel_coherence(predict, lend, argv) {
                diags.push(Diag::new(
                    DiagCode::WrapperPeelIncoherent(WrapperPeelIncoherent {
                        wrapper: interner.resolve(*provider).to_owned(),
                        predict_depth: inc.predict_depth.to_string(),
                        lend_map_depth: inc.lend_map_depth.to_string(),
                    }),
                    predict.name_span,
                ));
                break; // one diagnostic per provider
            }
        }
    }

    let incoherent = !diags.is_empty();
    (diags, incoherent)
}

/// The MARKS-LOST BACKSTOP (`26G:haz-silence-is-the-common-cause`): every role a marked file can
/// declare is lifted, and any funcdef the parse RECOGNIZED but the lift did not produce is warned
/// about by name. Deliberately cause-AGNOSTIC — it compares declared against lifted and says only
/// that the difference exists, so it catches drop paths and unrouted roles that do not exist yet.
/// That is the point: every `26G` diagnostic finding was a body going inert while `dorc lint`
/// called the file clean, and each was found by a HUMAN doing this comparison by hand.
///
/// It fires ALONGSIDE a cause-bearing diagnostic rather than deferring to one, and that is
/// deliberate: `predict-out-of-dialect` names the offending LINE, this names the FUNCTION the line
/// took down with it, and the r26 authors read the first without drawing the second. Only the
/// six roles whose lift diags reach no surface at all can be lost silently today, but nothing here
/// depends on knowing which six — that list is exactly what drifts.
///
/// An UNMARKED file is skipped: role-NAME recognition works there (`marker-and-names`), but
/// dialect syntax does not, so its funcdefs are not expected to lift and warning about them would
/// fire on every ordinary shell file holding a `__`-shaped name.
fn unlifted_role_fns(interner: &mut Interner, src: &str) -> Vec<Diag> {
    if !crate::marker::has_marker(src) {
        return Vec::new();
    }
    let roles: [fn(&mut Interner, &str) -> dorc_aid::Carrier<crate::predict::PredictSet>; 8] = [
        crate::predict::lift_predicts,
        crate::predict::lift_verdicts_converged,
        crate::predict::lift_touches,
        crate::predict::lift_reaches,
        crate::predict::lift_resolvers,
        crate::predict::lift_state_stored_only_in,
        crate::predict::lift_lend_maps,
        crate::predict::lift_enters,
    ];
    let mut diags = Vec::new();
    for lift in roles {
        for lost in lift(interner, src).value.unlifted() {
            diags.push(Diag::new(
                DiagCode::OracleRoleFnUnlifted(dorc_aid::diag::OracleRoleFnUnlifted {
                    funcname: lost.name.clone(),
                }),
                lost.name_span,
            ));
        }
    }
    diags
}

#[cfg(test)]
mod check_stage_tests {
    use super::validate;
    use dorc_aid::diag::DiagCode;
    use dorc_core::Interner;

    const VERDICT_GIVEUP: &str =
        "# dorc-lang/v0.2\nw__is_converged() {\n   [ -n \"$1\" ] || return 2\n   w q \"$1\"\n}\n";
    const PREDICT_GIVEUP: &str =
        "# dorc-lang/v0.2\nw__predict() {\n   [ -n \"$1\" ] || return 2\n   w q \"$1\"\n}\n";
    const CLEAN: &str = "# dorc-lang/v0.2\nw__is_converged() {\n   w q \"$1\"\n}\n";

    /// The per-file index of the `check` stage carrying an out-of-dialect give-up, if any.
    fn giveup_file(src: &str) -> Option<usize> {
        let mut i = Interner::default();
        validate(&mut i, &[src])
            .stages
            .into_iter()
            .find(|s| {
                s.stage == "check"
                    && s.diags
                        .iter()
                        .any(|d| matches!(d.code, DiagCode::PredictOutOfDialect(_)))
            })
            .and_then(|s| s.file)
    }

    /// A VERDICT body's out-of-dialect give-up reaches the `check` stage framed into its own file.
    /// The per-file index is the whole point: it is what lets every consumer — the cli's plan lane
    /// and the lint rung-oracle-solo lane alike — resolve `(path, src)` and render a real span.
    /// Reporting this through a fileless lane instead framed verdict give-ups at `1:1`, sourceless.
    #[test]
    fn a_verdict_body_giveup_is_a_per_file_check_stage() {
        assert_eq!(giveup_file(VERDICT_GIVEUP), Some(0));
    }

    /// Role PARITY: the identical construct in a predict body lands the same way. Both roles abandon
    /// their funcdef through one `parse_block` give-up, so one stage must carry both.
    #[test]
    fn predict_and_verdict_giveups_land_identically() {
        assert_eq!(giveup_file(PREDICT_GIVEUP), giveup_file(VERDICT_GIVEUP));
    }

    /// The negative pin: an in-dialect oracle mints no give-up at all.
    #[test]
    fn a_clean_oracle_mints_no_giveup() {
        assert_eq!(giveup_file(CLEAN), None);
    }
}

#[cfg(test)]
mod marks_lost_backstop_tests {
    //! The cause-agnostic backstop (`26G:haz-silence-is-the-common-cause`). Every case here is a
    //! whole oracle file run through `validate`, because the backstop's whole claim is about what
    //! a FILE declares versus what the lift produced.
    use super::validate;
    use dorc_aid::diag::DiagCode;
    use dorc_core::Interner;

    /// The funcnames the backstop reports lost, in emission order.
    fn lost(src: &str) -> Vec<String> {
        let mut i = Interner::default();
        validate(&mut i, &[src])
            .stages
            .into_iter()
            .flat_map(|s| s.diags)
            .filter_map(|d| match d.code {
                DiagCode::OracleRoleFnUnlifted(p) => Some(p.funcname),
                _ => None,
            })
            .collect()
    }

    /// The three `26G` F3 constructs, each in its own file: a bracket test in statement position, a
    /// `case` arm with a glob pattern, and a continuation into a redirection. Each takes its whole
    /// funcdef down — which is the fact the authors missed while reading a per-LINE complaint.
    #[test]
    fn each_voiding_construct_names_the_funcdef_it_took_down() {
        let bracket = "# dorc-lang/v0.2\nw__predict() {\n   [ -n \"$1\" ] || return 2\n   thing : sm.dorc.Thing = \"$1\"\n   w q \"$thing\"\n}\n";
        let case_arm = "# dorc-lang/v0.2\nw__predict() {\n   case \"$1\" in -*) return 2 ;; esac\n   thing : sm.dorc.Thing = \"$1\"\n   w q \"$thing\"\n}\n";
        let continuation =
            "# dorc-lang/v0.2\nw__predict() {\n   w q \"$1\" \\\n      >/dev/null 2>&1\n}\n";
        for src in [bracket, case_arm, continuation] {
            assert_eq!(lost(src), vec!["w__predict".to_owned()], "{src}");
        }
    }

    /// A role whose give-up diagnostics reach NO surface — the class the backstop exists for. A
    /// `__lend_map` body is lifted by `crate::wrapper` and its lift diags are dropped there, so
    /// before this backstop a voided lend-map was invisible from every surface dorc has.
    #[test]
    fn an_unrouted_roles_loss_is_reported_too() {
        let src = "# dorc-lang/v0.2\nsudo__lend_map() {\n   [ -n \"$1\" ] || return 2\n   printf '%s\\n' \"$1\" : lends user\n}\n";
        assert_eq!(lost(src), vec!["sudo__lend_map".to_owned()]);
    }

    /// NEGATIVE PIN — a legitimately markless oracle stays silent. It declares a funcdef and
    /// carries no binds or marks at all; nothing was lost, so nothing is said. Without this the
    /// backstop could "pass" by warning about every oracle in the corpus.
    #[test]
    fn a_markless_oracle_stays_silent() {
        let src = "# dorc-lang/v0.2\nw__is_converged() {\n   w q \"$1\"\n}\n";
        assert!(lost(src).is_empty());
    }

    /// NEGATIVE PIN — the bare `*)` arm the r26 trial reports working must stay quiet. It is the
    /// discriminator against a naive "any case arm voids the body" over-trigger; the glob arm above
    /// fires and this one does not, so the backstop is tracking the lift, not the syntax.
    #[test]
    fn a_bare_default_case_arm_stays_silent() {
        let src = "# dorc-lang/v0.2\nw__predict() {\n   case \"$1\" in *) ;; esac\n   thing : sm.dorc.Thing = \"$1\"\n   w q \"$thing\"\n}\n";
        assert!(lost(src).is_empty(), "{:?}", lost(src));
    }

    /// NEGATIVE PIN — an UNMARKED file is out of scope. `__role` names are recognized without the
    /// marker but dialect syntax is not, so its funcdefs are not expected to lift.
    #[test]
    fn an_unmarked_file_is_out_of_scope() {
        let src = "w__predict() {\n   [ -n \"$1\" ] || return 2\n   w q \"$1\"\n}\n";
        assert!(lost(src).is_empty());
    }
}
