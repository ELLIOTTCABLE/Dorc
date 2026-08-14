//! The prose burn-down instrument: who wrote the registers in the two generated locks. NEVER a
//! gate. Reading generated Rust textually is sanctioned here for the reason the commit-msg ratchet
//! shares — one serializer, one register per line — and keeps this crate dependency-free.

use std::process::ExitCode;

const LOCKS: [&str; 2] = [
    "spike/crates/aid/src/catalog_lock.rs",
    "spike/crates/aid/src/arrangement_lock.rs",
];

const TIERS: [&str; 3] = ["Migrated(", "Slop(", HUMAN_TIER];
const HUMAN_TIER: &str = "WrittenByHumanOnly(";

/// One lock's per-tier occurrence counts, and the slug of every human-written register in it.
fn census(text: &str) -> ([usize; 3], Vec<String>) {
    let mut counts = [0_usize; 3];
    let mut human = Vec::new();
    let mut slug = "<before the first row>";
    for line in text.lines() {
        if let Some(rest) = line.trim_start().strip_prefix("slug: \"") {
            slug = rest.split('"').next().unwrap_or(slug);
        }
        for (count, tier) in counts.iter_mut().zip(TIERS) {
            let hits = line.matches(&format!("ProseTier::{tier}")).count();
            *count = count.saturating_add(hits);
            if hits > 0 && tier == HUMAN_TIER {
                human.push(slug.to_owned());
            }
        }
    }
    (counts, human)
}

pub(crate) fn run() -> ExitCode {
    let root = internal_tooling::repo_root();
    let mut human = Vec::new();
    for lock in LOCKS {
        let path = root.join(lock);
        let Ok(text) = std::fs::read_to_string(&path) else {
            eprintln!("prose-census: cannot read {}", path.display());
            return ExitCode::from(2);
        };
        let (counts, found) = census(&text);
        human.extend(found.into_iter().map(|slug| format!("{slug} ({lock})")));
        let written: usize = counts.iter().sum();
        println!(
            "{lock}: {} migrated, {} slop, {} human ({written} written)",
            counts.first().unwrap_or(&0),
            counts.get(1).unwrap_or(&0),
            counts.get(2).unwrap_or(&0)
        );
    }
    if human.is_empty() {
        println!("human-written registers: none");
    } else {
        println!("human-written registers ({}):", human.len());
        for entry in &human {
            println!("  {entry}");
        }
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::census;

    /// Two registers per row, attributed to the slug above them — the shape a real lock carries,
    /// stated over a synthesized one so writing prose can never redden this.
    #[test]
    fn the_census_counts_per_register_and_names_every_human_slug() {
        let lock = "// @generated\n    CatalogEntry {\n        slug: \"a-code\",\n        \
                    message: Some(ProseTier::Migrated(\"m\")),\n        \
                    help: HelpRegister::Written(ProseTier::WrittenByHumanOnly(\"h\")),\n    },\n    \
                    CatalogEntry {\n        slug: \"b-code\",\n        \
                    message: Some(ProseTier::Slop(\"s\")),\n        \
                    help: HelpRegister::Written(ProseTier::WrittenByHumanOnly(\"h\")),\n    },\n";
        assert_eq!(
            census(lock),
            ([1, 1, 2], vec!["a-code".to_owned(), "b-code".to_owned()])
        );
        assert_eq!(census("        words: None,\n"), ([0, 0, 0], Vec::new()));
    }
}
