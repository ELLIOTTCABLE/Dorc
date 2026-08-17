//! The promote act (`301` §5) — the one ceremony by which the catalogue changes.
//!
//! The lock is `@generated`, and until now nothing generated it: the header named a subcommand
//! that did not exist, and the act was a hand-edit taking the header's word for it
//! (`300:fnd-promote-subcommand-missing`). This module is that subcommand.
//!
//! # What promote may and may not write
//!
//! Two kinds of field live in a row, and they move by opposite rules.
//!
//! * **Claim inputs** — the seat, the proof path, the paired harness, the bindings — are
//!   AUTHORED. Promote takes them from the command line or carries the committed value forward
//!   unchanged; it never invents one, and a new law with no seat is a refusal rather than a
//!   guess.
//! * **Badge expectations** are DERIVED, and derived only from evidence this run actually
//!   computed. `Earned` is written when and only when the evidence said `Earned` at a tier that
//!   looked; `Todo` is written when a looking tier found the evidence absent. A badge whose
//!   engine did not run is carried forward untouched — a cheap promote therefore cannot mint a
//!   Lean or Kani claim, and cannot quietly demote one either. That is both directions of the
//!   gate's own refusal, applied at the moment of authorship.
//!
//! A typed `excepted(reason)` is the one expectation nothing derives: it survives an absence
//! rather than collapsing to `todo`, so a deliberate non-coverage a human wrote stays written.
//!
//! Review is the git diff of the generated file. That is the whole ceremony, and it works
//! because the file is small, sorted, and carries no timestamp, hash, or other churn — a diff
//! against it is a diff of claims.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::badge::{Badge, Evidence, Expectation};
use crate::catalogue::LawRow;
use crate::evidence::{self, Tier};
use crate::unit::Unit;

/// The authored claim inputs a promote carries, keyed by law slug.
#[derive(Debug, Default)]
pub struct Inputs {
    /// `--seat <Slug>=<citation>`.
    pub seats: BTreeMap<String, String>,
    /// `--proof <Slug>=<repo-relative path>`, or `=none` to withdraw one.
    pub proofs: BTreeMap<String, Option<String>>,
    /// `--harness <Slug>=<harness fn>`, or `=none` to withdraw one.
    pub harnesses: BTreeMap<String, Option<String>>,
}

impl Inputs {
    /// Parse `--seat`/`--proof`/`--harness` pairs.
    ///
    /// # Errors
    /// On an unknown flag, a flag with no value, or a value that is not `<Slug>=<value>` —
    /// each of which would otherwise be a silently ignored claim.
    pub fn parse(args: &[&str]) -> Result<Self, String> {
        let mut out = Self::default();
        let mut rest = args.iter();
        while let Some(flag) = rest.next() {
            if !flag.starts_with("--") || matches!(*flag, "--with-lean" | "--with-kani") {
                continue;
            }
            let (slug, value) = split_assignment(flag, rest.next().copied())?;
            match *flag {
                "--seat" => out.seats.insert(slug, value),
                "--proof" => {
                    out.proofs.insert(slug, withdrawable(value));
                    None
                }
                "--harness" => {
                    out.harnesses.insert(slug, withdrawable(value));
                    None
                }
                other => return Err(format!("promote: unknown option `{other}`")),
            };
        }
        Ok(out)
    }
}

/// `none` withdraws a claim; anything else is one.
fn withdrawable(value: String) -> Option<String> {
    (value != "none").then_some(value)
}

fn split_assignment(flag: &str, value: Option<&str>) -> Result<(String, String), String> {
    let raw = value.ok_or_else(|| format!("promote: `{flag}` wants `<Slug>=<value>`"))?;
    raw.split_once('=')
        .map(|(slug, value)| (slug.to_owned(), value.to_owned()))
        .ok_or_else(|| format!("promote: `{flag} {raw}` is not `<Slug>=<value>`"))
}

/// One row as promote will write it.
#[derive(Debug)]
pub struct Promoted {
    /// The law slug, which is also the unit's file stem.
    pub slug: String,
    /// The cited seat.
    pub seat: String,
    /// The claimed proof, repo-relative.
    pub proof: Option<String>,
    /// The paired Kani harness function.
    pub harness: Option<String>,
    /// Bindings, carried forward verbatim.
    pub bindings: Vec<(String, Vec<(String, String)>)>,
    /// The badge expectations this promote writes.
    pub expected: [Expectation; Badge::ALL.len()],
    /// What each badge's expectation did, for the operator's summary.
    pub movements: Vec<String>,
}

/// Resolve every law's claim inputs, BEFORE any engine runs.
///
/// Split from [`finish`] for one reason: the harnesses this promote is about are named here,
/// and the Kani lane should verify those and not a whole battery. A run that has to enumerate
/// its own subject before it can bound its spend cannot do it in one pass.
///
/// # Errors
/// When a law has no seat to cite — the one claim input that has no default, because a seat is
/// what makes a badge evidence ABOUT something.
pub fn claims(
    units: &[Unit],
    committed: &[LawRow],
    inputs: &Inputs,
) -> Result<Vec<LawRow>, String> {
    let mut out = Vec::new();
    for unit in units {
        let was = committed.iter().find(|law| law.slug == unit.slug);
        let seat = inputs
            .seats
            .get(&unit.slug)
            .cloned()
            .or_else(|| was.map(|law| law.seat.to_owned()))
            .ok_or_else(|| {
                format!(
                    "{}: a new law needs its seat — pass `--seat {}=<crate::module::Owner::fn>`",
                    unit.slug, unit.slug
                )
            })?;
        let proof = carried(inputs.proofs.get(&unit.slug), was.and_then(|law| law.proof));
        let harness = carried(
            inputs.harnesses.get(&unit.slug),
            was.and_then(|law| law.harness),
        );
        out.push(LawRow {
            slug: leak(&unit.slug),
            seat: leak(&seat),
            proof: proof.as_deref().map(leak),
            harness: harness.as_deref().map(leak),
            bindings: was.map_or(&[], |law| law.bindings),
            expected: was.map_or([Expectation::Todo; Badge::ALL.len()], |law| law.expected),
        });
    }
    Ok(out)
}

/// Every harness some row pairs with — the exact set a promote needs verified, and nothing
/// else.
#[must_use]
pub fn paired_harnesses(rows: &[LawRow]) -> Vec<&'static str> {
    let mut names: Vec<&'static str> = rows.iter().filter_map(|row| row.harness).collect();
    names.sort_unstable();
    names.dedup();
    names
}

/// Compute each row's badges and turn the claims into the rows promote will write.
#[must_use]
pub fn finish(
    repo_root: &Path,
    tier: Tier<'_>,
    units: &[Unit],
    claimed: &[LawRow],
) -> Vec<Promoted> {
    claimed
        .iter()
        .map(|row| {
            let unit = units.iter().find(|u| u.slug == row.slug);
            let found = evidence::compute(row, unit, repo_root, tier);
            let (expected, movements) = expectations(row, &found);
            Promoted {
                slug: row.slug.to_owned(),
                seat: row.seat.to_owned(),
                proof: row.proof.map(str::to_owned),
                harness: row.harness.map(str::to_owned),
                bindings: bindings_of(row),
                expected,
                movements,
            }
        })
        .collect()
}

/// A flag overrides; absent a flag, the committed value stands.
fn carried(flag: Option<&Option<String>>, committed: Option<&'static str>) -> Option<String> {
    flag.map_or_else(|| committed.map(str::to_owned), Clone::clone)
}

/// The evidence's verdict is the ONLY thing that writes a badge, and only where it looked.
fn expectations(
    row: &LawRow,
    found: &[Evidence],
) -> ([Expectation; Badge::ALL.len()], Vec<String>) {
    let mut expected = row.expected;
    let mut movements = Vec::new();
    for (index, badge) in Badge::ALL.iter().enumerate() {
        let was = row.expectation(*badge);
        let now = match found.get(index) {
            None | Some(Evidence::NotAtThisTier) => {
                movements.push(format!(
                    "`{badge}` NOT RECOMPUTED, left at {}",
                    was.render()
                ));
                continue;
            }
            Some(Evidence::Earned) => Expectation::Earned,
            Some(Evidence::Absent(_)) => match was {
                Expectation::Excepted(why) => Expectation::Excepted(why),
                Expectation::Earned | Expectation::Todo => Expectation::Todo,
            },
        };
        if now != was {
            movements.push(format!("`{badge}` {} → {}", was.render(), now.render()));
        }
        if let Some(slot) = expected.get_mut(index) {
            *slot = now;
        }
    }
    (expected, movements)
}

fn bindings_of(row: &LawRow) -> Vec<(String, Vec<(String, String)>)> {
    row.bindings
        .iter()
        .map(|binding| {
            (
                binding.case.to_owned(),
                binding
                    .assertions
                    .iter()
                    .map(|a| (a.site.to_owned(), format!("{:?}", a.decision)))
                    .collect(),
            )
        })
        .collect()
}

/// The catalogue rows are `&'static str` because the lock is compiled in; a promote builds its
/// candidate rows at runtime, so their strings are leaked for the length of the process. The
/// process writes one file and exits.
fn leak(text: &str) -> &'static str {
    Box::leak(text.to_owned().into_boxed_str())
}

/// Where the generated lock lives.
#[must_use]
pub fn path(repo_root: &Path) -> PathBuf {
    repo_root
        .join("spike")
        .join("verify")
        .join("src")
        .join("catalogue_lock.rs")
}

/// Render the whole lock file.
#[must_use]
pub fn render(rows: &[Promoted]) -> String {
    let mut out = String::new();
    out.push_str(HEADER);
    let _ = writeln!(out, "pub const LAWS: [LawRow; {}] = [", rows.len());
    for row in rows {
        let _ = writeln!(out, "    LawRow {{");
        let _ = writeln!(out, "        slug: {:?},", row.slug);
        let _ = writeln!(out, "        seat: {:?},", row.seat);
        let _ = writeln!(out, "        proof: {},", option(row.proof.as_deref()));
        let _ = writeln!(out, "        harness: {},", option(row.harness.as_deref()));
        render_bindings(&mut out, &row.bindings);
        let _ = writeln!(out, "        expected: [");
        for expectation in row.expected {
            let _ = writeln!(out, "            {},", spell(expectation));
        }
        let _ = writeln!(out, "        ],");
        let _ = writeln!(out, "    }},");
    }
    out.push_str("];\n");
    out
}

fn render_bindings(out: &mut String, bindings: &[(String, Vec<(String, String)>)]) {
    if bindings.is_empty() {
        let _ = writeln!(out, "        bindings: &[],");
        return;
    }
    let _ = writeln!(out, "        bindings: &[");
    for (case, assertions) in bindings {
        let _ = writeln!(out, "            Binding {{");
        let _ = writeln!(out, "                case: {case:?},");
        if assertions.is_empty() {
            let _ = writeln!(out, "                assertions: &[],");
        } else {
            let _ = writeln!(out, "                assertions: &[");
            for (site, decision) in assertions {
                let _ = writeln!(
                    out,
                    "                    SiteDecision {{ site: {site:?}, decision: \
                     Decision::{decision} }},"
                );
            }
            let _ = writeln!(out, "                ],");
        }
        let _ = writeln!(out, "            }},");
    }
    let _ = writeln!(out, "        ],");
}

fn option(value: Option<&str>) -> String {
    value.map_or_else(|| "None".to_owned(), |text| format!("Some({text:?})"))
}

fn spell(expectation: Expectation) -> String {
    match expectation {
        Expectation::Earned => "Expectation::Earned".to_owned(),
        Expectation::Todo => "Expectation::Todo".to_owned(),
        Expectation::Excepted(why) => format!("Expectation::Excepted({why:?})"),
    }
}

/// The lock's own header. It names this subcommand, and now that is true.
const HEADER: &str = r"//! @generated by `dorc-verify promote` — do not edit by hand.
//!
//! The promote-gated catalogue lock (`301` §5). Every row's badge expectation was COMPUTED
//! from evidence at promote time; the gate recomputes and refuses a mismatch in either
//! direction, so a silent demotion (rot) and a silent promotion (ambition) are both loud.
//!
//! Promoting is a spec-side act under `301:law-spec-touch-frontier-human-only`. Review is the
//! git diff of this file. Claim inputs a promote cannot derive — a typed `excepted(reason)`
//! above all — are authored here and carried forward by the generator untouched.

use crate::badge::Expectation;
use crate::catalogue::LawRow;

/// Every catalogued law, sorted by slug.
";

#[cfg(test)]
mod tests {
    use super::*;

    fn row(expected: [Expectation; Badge::ALL.len()]) -> LawRow {
        LawRow {
            slug: "AnyLawAtAll",
            seat: "dorc_core::sorted::SortedSet::insert",
            proof: None,
            harness: None,
            bindings: &[],
            expected,
        }
    }

    #[test]
    fn a_tier_that_did_not_look_moves_nothing_in_either_direction() {
        // The load-bearing rule. A cheap promote — the one somebody runs to record a harness
        // pairing — must not mint the Lean claim it cannot check, and must not withdraw one
        // that a Lean-tier promote earned. Carrying forward is what makes both impossible.
        let was = row([
            Expectation::Earned,
            Expectation::Earned,
            Expectation::Todo,
            Expectation::Todo,
            Expectation::Todo,
            Expectation::Todo,
        ]);
        let blind = vec![Evidence::NotAtThisTier; Badge::ALL.len()];
        let (now, movements) = expectations(&was, &blind);
        assert_eq!(now, was.expected);
        assert!(movements.iter().all(|m| m.contains("NOT RECOMPUTED")));
    }

    #[test]
    fn only_evidence_writes_earned_and_only_evidence_withdraws_it() {
        let was = row([Expectation::Todo; Badge::ALL.len()]);
        let mut found = vec![Evidence::NotAtThisTier; Badge::ALL.len()];
        found[2] = Evidence::Earned;
        let (now, movements) = expectations(&was, &found);
        assert_eq!(now[2], Expectation::Earned, "the badge that was looked at");
        assert_eq!(now[0], Expectation::Todo, "and no other");
        assert!(movements.iter().any(|m| m.contains("todo → earned")));

        // …and the reverse, which is the rot direction: evidence gone, claim withdrawn.
        let claimed = row([Expectation::Earned; Badge::ALL.len()]);
        let mut lost = vec![Evidence::NotAtThisTier; Badge::ALL.len()];
        lost[2] = Evidence::Absent("harness deleted".to_owned());
        let (after, _) = expectations(&claimed, &lost);
        assert_eq!(after[2], Expectation::Todo);
        assert_eq!(after[0], Expectation::Earned, "untouched where unlooked");
    }

    #[test]
    fn a_typed_exception_survives_the_absence_it_describes() {
        // `excepted(reason)` IS a statement about an absence. Collapsing it to `todo` on the
        // next promote would erase a human's deliberate ruling as a side effect of running a
        // generator, which is the one thing an authored field must never do.
        let mut expected = [Expectation::Todo; Badge::ALL.len()];
        expected[4] = Expectation::Excepted("no product surface to bind yet");
        let was = row(expected);
        let mut found = vec![Evidence::NotAtThisTier; Badge::ALL.len()];
        found[4] = Evidence::Absent("no accepted binding".to_owned());
        let (now, _) = expectations(&was, &found);
        assert_eq!(
            now[4],
            Expectation::Excepted("no product surface to bind yet")
        );
    }

    #[test]
    fn the_rendered_lock_is_the_shape_the_crate_compiles() {
        let rendered = render(&[Promoted {
            slug: "JoinIsIdempotent".to_owned(),
            seat: "dorc_analysis::lattice::Flat::join".to_owned(),
            proof: None,
            harness: Some("flat_obeys_the_binary_laws".to_owned()),
            bindings: Vec::new(),
            expected: [Expectation::Todo; Badge::ALL.len()],
            movements: Vec::new(),
        }]);
        assert!(rendered.starts_with("//! @generated by `dorc-verify promote`"));
        assert!(rendered.contains("pub const LAWS: [LawRow; 1] = ["));
        assert!(rendered.contains("harness: Some(\"flat_obeys_the_binary_laws\"),"));
        assert!(rendered.contains("bindings: &[],"));
        assert!(rendered.trim_end().ends_with("];"));
    }

    #[test]
    fn an_option_flag_is_a_slug_assignment_or_a_refusal() {
        // A mistyped claim input that parsed as nothing would leave the operator believing they
        // had recorded a pairing they had not.
        let parsed = Inputs::parse(&["--harness", "JoinIsIdempotent=flat_obeys_the_binary_laws"])
            .expect("a well-formed assignment");
        assert_eq!(
            parsed.harnesses.get("JoinIsIdempotent"),
            Some(&Some("flat_obeys_the_binary_laws".to_owned()))
        );
        assert_eq!(
            Inputs::parse(&["--harness", "JoinIsIdempotent=none"])
                .expect("withdrawal parses")
                .harnesses
                .get("JoinIsIdempotent"),
            Some(&None),
            "`none` withdraws rather than naming a harness called none"
        );
        assert!(Inputs::parse(&["--harness", "no-equals-sign"]).is_err());
        assert!(Inputs::parse(&["--harness"]).is_err());
        assert!(Inputs::parse(&["--seet", "X=y"]).is_err(), "a typo refuses");
    }
}
