//! The shipped `dorc` binary: the composition root driven with the all-production seam bundle.
//!
//! Everything below the seam is [`dorc_cli::compose::run`], shared byte for byte with `dorc-harness`
//! (`30X:bin-harness-sibling-not-produced-cli`). The ONLY difference is the bundle, and this one is
//! [`Seams::os`], which reads NO harness-shaped environment at all
//! (`30X:inv-fixture-state-never-typeable-into-main`) and is the sole constructor of the production
//! variants. This file is edge VALUES plus one call (`30X:inv-division-at-the-narrowest-edge`).
#![forbid(unsafe_code)]

use std::process::ExitCode;

use dorc_cli::seam::Seams;

fn main() -> ExitCode {
    dorc_cli::compose::run(&Seams::os())
}
