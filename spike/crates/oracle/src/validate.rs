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
        if !diags.is_empty() {
            stages.push(StageDiags {
                stage: "check",
                file: Some(i),
                diags,
            });
        }
    }

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
                    detail: format!(
                        "wrapper `{}`: __enter and __lend_map disagree on argv flow (entry \
                         consumes {} leading arg(s), the lend-fold consumes {}) -- static \
                         incoherence (27C:rul-fold-entry-coherence-failfast, \
                         declarations-genuinely-contradict). The entry form drops/transforms args \
                         the fold relied on; make the entry pass the fold's guest verbatim.",
                        interner.resolve(*provider),
                        inc.entry_shifts,
                        inc.lend_shifts,
                    ),
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
                        detail: format!(
                            "wrapper `{}`: __predict and __lend_map disagree on the peel tail \
                             position (predict reaches \"$@\" after {} argv token(s), lend_map \
                             after {}) -- static incoherence (273 section 5, \
                             declarations-genuinely-contradict). The guest would start at a \
                             different token depending on which member dispatched; fix the \
                             argparse so both peel to the same tail.",
                            interner.resolve(*provider),
                            inc.predict_depth,
                            inc.lend_map_depth
                        ),
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
