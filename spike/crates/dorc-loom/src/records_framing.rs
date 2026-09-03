//! The ONE seat that frames a case's raw `probe-results.txt` into the `dorc-records/1` controller
//! wire stream both loom drivers admit (`30Xa:rul-in-process-sessions-take-the-controller-intake`).
//!
//! A SECOND implementation is how the first rots: the corpus commits records RAW, so anything that
//! wants a case's MEASURED world — the process driver (`cli/tests/e2e.rs`, which frames raw fixture
//! records for the session's fd 0) and the in-process driver (`consumer.rs`'s `observe`, which
//! frames the `<` target of a `--results -` block) — has to re-frame them identically or it is
//! analysing a different world than the run (`one-definition-table-two-drivers`, in the instrument
//! rather than the product).
//!
//! Split from the probe INVOCATION deliberately: the caller supplies the compiled probe's own bytes
//! its own way — the process driver runs `dorc probe`, the in-process driver renders the engine's
//! own probe stage against `default_framing` — and this stays a pure text→text function of
//! `(probe artifact, raw records)`. The site keys the framing needs are the site-set that probe
//! self-reports (`inv-site-keyed-results`), never a second parse.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

/// The per-record nonce the spike's fixed framing stamps (`262` §2) — one spelling, the plan's own.
pub const RECORDS_NONCE: &str = dorc_plan::records::DEFAULT_NONCE;
/// The per-record terminal token (`262` §2), the plan's own constant.
pub const RECORDS_TOKEN: &str = dorc_plan::records::TERMINAL_TOKEN;

/// Re-frame a case's raw inner records into the `dorc-records/1` stream the controller intake
/// admits, given the compiled probe's own bytes (the header + the resolvable site census) and the
/// raw records the case authored (empty for a `/dev/null` target — an empty framed stream).
#[expect(
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::too_many_lines,
    reason = "moved verbatim from the e2e harness, which granted these crate-wide; the shape is the sh original's"
)]
#[must_use]
pub fn frame_records(probe: &str, raw: &str) -> String {
    let header = probe
        .lines()
        .find(|line| line.contains("dorc-records/1"))
        .and_then(|line| line.split('\'').nth(1))
        .map(|field| field.strip_suffix("\\n").unwrap_or(field).to_owned());

    let mut sites: Vec<String> = Vec::new();
    for line in probe.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        for pair in fields.windows(2) {
            let (key, value) = (pair[0], pair[1]);
            if key == "site" && is_site_key(value) && !sites.iter().any(|seen| seen == value) {
                sites.push(value.to_owned());
            }
        }
    }

    let Some(header) = header else {
        return String::new();
    };

    let wanted: BTreeSet<&str> = sites.iter().map(String::as_str).collect();
    let mut body: Vec<String> = Vec::new();
    let mut deriv_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut deriv_order: Vec<String> = Vec::new();
    let mut deriv_closed: BTreeSet<String> = BTreeSet::new();
    let mut reach_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut reach_order: Vec<String> = Vec::new();
    let mut reach_closed: BTreeSet<String> = BTreeSet::new();
    for raw_line in raw.lines() {
        if raw_line.starts_with("dorc-records/1 ") || raw_line.starts_with("dorc-records-end/1 ") {
            continue;
        }
        let stripped = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let stripped = stripped.strip_prefix("dorc ").unwrap_or(stripped);
        let line = stripped
            .strip_suffix(&format!(" {RECORDS_TOKEN}"))
            .unwrap_or(stripped)
            .to_owned();
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        match fields.first().copied() {
            Some("site") if !fields.get(1).is_some_and(|id| wanted.contains(id)) => continue,
            Some("deriv") => {
                if let Some(site) = fields.get(1) {
                    let site = (*site).to_owned();
                    *deriv_counts.entry(site.clone()).or_default() += 1;
                    if !deriv_order.contains(&site) {
                        deriv_order.push(site);
                    }
                }
            }
            Some("deriv-end") => {
                if let Some(site) = fields.get(1) {
                    deriv_closed.insert((*site).to_owned());
                }
            }
            Some("reach") => {
                if let Some(key) = reach_arm_key(&fields) {
                    *reach_counts.entry(key.clone()).or_default() += 1;
                    if !reach_order.contains(&key) {
                        reach_order.push(key);
                    }
                }
            }
            Some("reach-end") => {
                if let Some(key) = reach_arm_key(&fields) {
                    reach_closed.insert(key);
                }
            }
            _ => {}
        }
        body.push(line);
    }
    for site in &deriv_order {
        if !deriv_closed.contains(site) {
            // The close is SYNTHESIZED to agree with the authored coords, so neither gate fires on
            // authoring alone; a case exercising the body-death refusal spells its own `deriv-end`.
            body.push(format!(
                "deriv-end {site} n={} body-rc=0",
                deriv_counts.get(site).copied().unwrap_or_default()
            ));
        }
    }
    for key in &reach_order {
        if !reach_closed.contains(key) {
            body.push(format!(
                "reach-end {key} n={} body-rc=0",
                reach_counts.get(key).copied().unwrap_or_default()
            ));
        }
    }

    let mut out = String::new();
    out.push_str(&header);
    out.push('\n');
    for line in body {
        let line = if line.trim_start().starts_with("site ") && !line.contains(" rc=") {
            format!("{line} rc=0")
        } else {
            line
        };
        let _ = writeln!(out, "{RECORDS_NONCE} {line} {RECORDS_TOKEN}");
    }
    for site in &sites {
        if !out
            .lines()
            .any(|line| line.starts_with(&format!("{RECORDS_NONCE} site {site} ")))
        {
            let _ = writeln!(
                out,
                "{RECORDS_NONCE} site {site} effect=cant-tell rc=0 {RECORDS_TOKEN}"
            );
        }
    }
    let _ = writeln!(
        out,
        "dorc-records-end/1 nonce={RECORDS_NONCE} {RECORDS_TOKEN}"
    );
    out
}

/// A site key is `N` or, for an in-loop Members member, `N.M`.
fn is_site_key(value: &str) -> bool {
    let mut parts = value.split('.');
    let head = parts.next().unwrap_or_default();
    let tail = parts.next();
    parts.next().is_none()
        && !head.is_empty()
        && head.chars().all(|c| c.is_ascii_digit())
        && tail.is_none_or(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_digit()))
}

/// The `<coord> arm=<n>` key a `reach`/`reach-end` record line carries, for the authored-fixture
/// close synthesis. Whitespace-split, so a coord bearing spaces is not a fixture shape here.
fn reach_arm_key(fields: &[&str]) -> Option<String> {
    let coord = fields.get(1)?;
    let arm = fields.get(2).filter(|f| f.starts_with("arm="))?;
    Some(format!("{coord} {arm}"))
}
