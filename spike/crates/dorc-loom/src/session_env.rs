//! The in-process session's modelled environment (`30X:loom-seams-are-sh-lines`,
//! `30X:dogfood-the-sh-engine`).
//!
//! Seam selection is spelled as sh IN the session (`$ export DORC_SEAM_CLOCK=pinned:7`); the process
//! driver exports the selections into a real shell, and the in-process driver models the same
//! environment and reads its EXPORTED subset through the ONE parser both drivers use
//! (`HarnessSeams::from_env`). The model is dash semantics: a shell-local `NAME=word` never reaches
//! a child, `export NAME` marks, and `export NAME=word` sets and marks — so a variable carries a
//! value AND an exported bit, and only an exported, non-empty variable is a seam.
//!
//! The runner seeds the map with its per-block defaults and re-injects the clock per block EXACTLY
//! as `drive_session`'s shadow line does (`30Xa:rul-runner-varies-only-what-it-set`): only while the
//! current `DORC_SEAM_CLOCK` still equals the value the runner last injected, so an author's
//! `export DORC_SEAM_CLOCK=…` (or `export DORC_SEED=…`) then governs every later block.

use std::collections::BTreeMap;

use dorc_cli::seam::SeamEnv;
use dorc_testbed::run_seed::run_seed;
use dorc_testbed::seam_vars::{CLOCK_ENV, SEED_ENV};

use crate::runner_seams::{SESSION_ROOT, clock_seam_value, roots_seam_pair, value_seam_pairs};

/// One modelled variable: a value (`None` = marked but unset, so no child sees it) and whether it is
/// exported.
#[derive(Clone, Debug)]
struct Var {
    value: Option<String>,
    exported: bool,
}

/// The session's modelled environment across its blocks, plus the runner's clock shadow.
#[derive(Clone, Debug)]
pub(crate) struct SessionEnv {
    vars: BTreeMap<String, Var>,
    /// The clock value the runner last injected — its OWN framing state, never a seam and never read
    /// by `dorc` (the shell spells it `__DORC_RUNNER_CLOCK`, a shell-local).
    runner_clock: String,
    /// How many `dorc` INVOCATION blocks have run so far — the clock ordinal. Only invocations tick
    /// the clock, so a `$ export DORC_SEED=<n>` pin line is ordinal-neutral (`30Xa:Checkpoint C3`).
    invocation_ordinal: usize,
}

impl SessionEnv {
    /// Seed the map with the runner's session-start defaults (all exported, as the shell's
    /// `command.env` makes them), with the clock shadow starting EQUAL to invocation-0's clock.
    #[must_use]
    pub(crate) fn seeded() -> Self {
        let mut vars: BTreeMap<String, Var> = BTreeMap::new();
        for (name, value) in value_seam_pairs(0) {
            vars.insert(
                name.to_owned(),
                Var {
                    value: Some(value),
                    exported: true,
                },
            );
        }
        let (root_name, root_value) = roots_seam_pair(SESSION_ROOT);
        vars.insert(
            root_name.to_owned(),
            Var {
                value: Some(root_value),
                exported: true,
            },
        );
        let runner_clock = vars
            .get(CLOCK_ENV)
            .and_then(|var| var.value.clone())
            .unwrap_or_default();
        Self {
            vars,
            runner_clock,
            invocation_ordinal: 0,
        }
    }

    /// Set this INVOCATION's default clock, then advance the ordinal — called once per `dorc`
    /// invocation block, never for an `export`/`cd`/`echo $?`/`cat` (`30Xa:Checkpoint C3`: the clock
    /// ticks one day per invocation). The clock derives from the CURRENT `DORC_SEED` and the
    /// invocation ordinal by [`clock_seam_value`], so an author's `export DORC_SEED` governs it, and
    /// varying happens ONLY while the runner still owns the variable (`30Xa:rul-runner-varies-only-what-it-set`):
    /// if the current `DORC_SEAM_CLOCK` still equals the runner's last injected value the runner
    /// re-sets it, otherwise the author has taken it and the runner never touches it again.
    pub(crate) fn inject_invocation_clock(&mut self) {
        let clock_value = clock_seam_value(self.current_seed(), self.invocation_ordinal);
        if self
            .vars
            .get(CLOCK_ENV)
            .and_then(|var| var.value.as_deref())
            == Some(&self.runner_clock)
        {
            self.vars.insert(
                CLOCK_ENV.to_owned(),
                Var {
                    value: Some(clock_value.clone()),
                    exported: true,
                },
            );
            self.runner_clock = clock_value;
        }
        self.invocation_ordinal = self.invocation_ordinal.saturating_add(1);
    }

    /// The session's current `DORC_SEED`, parsed from the exported subset (the runner's default, or
    /// an author's `export DORC_SEED`); the drawn run seed backs an unset or unparseable value, which
    /// only arises when the session is already declining on a bad seam.
    fn current_seed(&self) -> u64 {
        self.var(SEED_ENV)
            .and_then(|raw| raw.parse::<u64>().ok())
            .unwrap_or_else(run_seed)
    }

    /// `export NAME=word`: set the value and mark exported.
    pub(crate) fn export_assign(&mut self, name: &str, value: &str) {
        self.vars.insert(
            name.to_owned(),
            Var {
                value: Some(value.to_owned()),
                exported: true,
            },
        );
    }

    /// `export NAME`: mark the (possibly unset) variable exported without changing its value.
    pub(crate) fn export_mark(&mut self, name: &str) {
        self.vars
            .entry(name.to_owned())
            .and_modify(|var| var.exported = true)
            .or_insert(Var {
                value: None,
                exported: true,
            });
    }
}

impl SeamEnv for SessionEnv {
    /// The exported subset, matching `ProcessSeamEnv`: an exported variable with a non-empty value,
    /// else `None`.
    fn var(&self, name: &str) -> Option<String> {
        self.vars
            .get(name)
            .filter(|var| var.exported)
            .and_then(|var| var.value.clone())
            .filter(|value| !value.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::SessionEnv;
    use crate::runner_seams::clock_seam_value;
    use dorc_cli::seam::SeamEnv;
    use dorc_testbed::run_seed::run_seed;
    use dorc_testbed::seam_vars::{CLOCK_ENV, SEED_ENV};

    /// No author touch: the runner ticks the clock once per invocation, so two publishes take
    /// distinct order tokens (`30Xa:rul-runner-varies-only-what-it-set`), each folded from the run
    /// seed and the invocation ordinal.
    #[test]
    fn the_runner_ticks_the_clock_once_per_invocation() {
        let seed = run_seed();
        let mut env = SessionEnv::seeded();
        env.inject_invocation_clock();
        assert_eq!(
            env.var(CLOCK_ENV).as_deref(),
            Some(clock_seam_value(seed, 0).as_str())
        );
        env.inject_invocation_clock();
        assert_eq!(
            env.var(CLOCK_ENV).as_deref(),
            Some(clock_seam_value(seed, 1).as_str())
        );
    }

    /// An author's `export DORC_SEED` pins every later invocation's clock (and its ids) with one
    /// line, because the runner still owns the clock variable and recomputes it from the new seed.
    #[test]
    fn an_author_seed_export_pins_every_later_clock() {
        let mut env = SessionEnv::seeded();
        env.inject_invocation_clock();
        env.export_assign(SEED_ENV, "7");
        env.inject_invocation_clock();
        assert_eq!(
            env.var(CLOCK_ENV).as_deref(),
            Some(clock_seam_value(7, 1).as_str())
        );
    }

    /// The `durable-receipt-ambiguous` shape: an author's pinned clock across two publishes, so they
    /// share an order and "the last one" names a cohort — the runner never touches it again.
    #[test]
    fn an_author_export_of_the_clock_ends_the_runners_control_of_it() {
        let mut env = SessionEnv::seeded();
        env.inject_invocation_clock();
        env.export_assign(CLOCK_ENV, "pinned:1769306437000");
        env.inject_invocation_clock();
        env.inject_invocation_clock();
        assert_eq!(env.var(CLOCK_ENV).as_deref(), Some("pinned:1769306437000"));
    }

    /// The seam view is the exported subset with a non-empty value, matching `ProcessSeamEnv`: a
    /// later author export overrides a seeded default, a bare `export NAME` of an unset variable
    /// supplies no value, and an empty exported value reads as `None`.
    #[test]
    fn only_an_exported_non_empty_variable_is_a_seam() {
        let mut env = SessionEnv::seeded();
        env.export_assign(SEED_ENV, "7");
        assert_eq!(env.var(SEED_ENV).as_deref(), Some("7"));
        env.export_mark("DORC_NOT_SET");
        assert_eq!(env.var("DORC_NOT_SET"), None);
        env.export_assign("DORC_EMPTY", "");
        assert_eq!(env.var("DORC_EMPTY"), None);
    }
}
