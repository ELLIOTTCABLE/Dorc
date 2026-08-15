//! Pre-spend resource bounds: refuse a heavy task before it eats the disk or the RAM.
//!
//! Two real incidents motivate it (`300` §2, close batch): a WSL VM OOM'd twice under
//! solver load, and `C:` reached zero bytes free mid-gate. Both cost a whole run's wall
//! clock and one of them took the human's terminals with it. The cheapest possible fix is
//! to look BEFORE spending: a directory stat and a memory read, then a loud refusal.
//!
//! Each leg checks its OWN environment. That is the whole point of the per-leg wiring —
//! the VM that OOM'd has its own ~15 GiB cap, and a Windows-side reading of Windows RAM
//! says nothing about it. `mise run both preflight <p>` is the paired form.
//!
//! Deliberately NOT on the hot loop: it costs a `cargo run` (~0.3s warm) plus the probe,
//! which is noise against a gate and a tax against `gate:quick`.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const GIB: u64 = 1024 * 1024 * 1024;

/// Which build cache a profile fills, and therefore which volume to measure.
#[derive(Debug, Clone, Copy)]
enum Cache {
    /// The cargo workspace target dir — `CARGO_TARGET_DIR`, else `<repo>/spike/target`.
    Workspace,
    /// Kani's own target dir (`verify/src/kani.rs` `build_root`). Linux/WSL only.
    Kani,
    /// The staged Lean build root (`verify/src/lib.rs` `lean_build_root`). Linux/WSL only.
    Lean,
}

/// One heavy lane's expected usage.
///
/// Two disk figures rather than one because the spread is an order of magnitude: a cold
/// tree pays the whole footprint, a warm one pays churn. Which applies is decided by an
/// O(1) existence check on the cache's witness path — never a directory walk, which on a
/// 12 GiB `target/` costs more than the gate it is guarding.
#[derive(Debug)]
struct Profile {
    name: &'static str,
    cache: Cache,
    /// Free bytes demanded when the cache is absent.
    disk_cold: u64,
    /// Free bytes demanded when it is already populated.
    disk_warm: u64,
    ram: u64,
}

/// The bounds. Every figure is a conservative round number over a measurement taken on
/// 2026-08-15; false precision here buys nothing, and a bound that fires spuriously is a
/// bound people learn to skip.
const PROFILES: [Profile; 4] = [
    Profile {
        name: "gate",
        cache: Cache::Workspace,
        // Measured `spike/target/debug` across three worktrees: 9.1 / 9.9 / 11.8 GiB
        // (deps 6.5, incremental 5.2), plus `target/clippy-clean` at 0.3.
        disk_cold: 14 * GIB,
        // Churn only: `clippy:clean`'s wipe-and-rebuild, a relink of the ~10 MiB test
        // binaries, and incremental growth before cargo's own GC.
        disk_warm: 4 * GIB,
        // Several concurrent rustc processes plus a link step. Low enough never to fire on
        // a healthy box, high enough to catch one an orphaned solver has already eaten.
        ram: 4 * GIB,
    },
    Profile {
        name: "bless",
        cache: Cache::Workspace,
        // Bless IS the gate plus a golden rewrite, and the goldens are kilobytes.
        disk_cold: 14 * GIB,
        disk_warm: 4 * GIB,
        ram: 4 * GIB,
    },
    Profile {
        name: "kani",
        cache: Cache::Kani,
        // Measured: `~/.kani` 0.5 GiB (the engine bundle `verify:kani-setup` fetches) plus
        // `~/.cache/dorc-kani-target` 0.4. Disk has never been this lane's constraint.
        disk_cold: 4 * GIB,
        disk_warm: 2 * GIB,
        // `verify/src/kani.rs` caps each harness at ADDRESS_SPACE_CAP_KB = 6_000_000 KiB
        // (~5.7 GiB) via `ulimit -v`, inherited by CBMC. A cap CBMC cannot reach is not a
        // cap, so demand it plus headroom for the kani-compiler and the VM itself.
        ram: 8 * GIB,
    },
    Profile {
        name: "lean",
        cache: Cache::Lean,
        // Measured: the staged build root `~/.cache/dorc-minispec-lean` at 7.7 GiB (mathlib's
        // olean store dominates) plus elan's toolchain store at 2.8.
        disk_cold: 12 * GIB,
        disk_warm: 3 * GIB,
        ram: 4 * GIB,
    },
];

/// Check one profile's bounds against this machine, now.
pub(crate) fn run(args: &[String]) -> ExitCode {
    let Some(name) = args.first() else {
        return usage("no profile named");
    };
    let Some(profile) = PROFILES.iter().find(|p| p.name == name) else {
        return usage(&format!("unknown profile {name:?}"));
    };

    if std::env::var("DORC_PREFLIGHT").is_ok_and(|v| v == "skip") {
        println!("preflight {name}: SKIPPED by DORC_PREFLIGHT=skip");
        return ExitCode::SUCCESS;
    }

    match profile.cache.root() {
        Err(reason) => {
            println!("preflight {name}: not applicable here — {reason}");
            ExitCode::SUCCESS
        }
        Ok(root) => report(profile, &root),
    }
}

/// Measure, then print exactly one line: a pass note, or a refusal naming the remedy.
fn report(profile: &Profile, root: &Path) -> ExitCode {
    let name = profile.name;
    let warm = profile.cache.witness(root).exists();
    let need_disk = if warm {
        profile.disk_warm
    } else {
        profile.disk_cold
    };
    let state = if warm { "warm" } else { "COLD" };

    let disk = free_disk(root);
    let ram = free_ram();

    if let Ok(free) = disk
        && free < need_disk
    {
        println!(
            "preflight {name}: REFUSED — {} free on the volume holding {}, needs {} ({state} cache). \
             Free space (`mise run doctor` inventories what is reclaimable), or set \
             DORC_PREFLIGHT=skip for an emergency.",
            gib(free),
            root.display(),
            gib(need_disk)
        );
        return ExitCode::FAILURE;
    }
    if let Ok(free) = ram
        && free < profile.ram
    {
        println!(
            "preflight {name}: REFUSED — {} RAM available, needs {}. Reap what is holding it \
             (an orphaned `cbmc` outlives its driver; `pkill -9 -x cbmc`), or set \
             DORC_PREFLIGHT=skip for an emergency.",
            gib(free),
            gib(profile.ram)
        );
        return ExitCode::FAILURE;
    }

    // An unmeasurable probe warns and passes. Refusing on it would block a whole platform
    // over a missing helper, which is a worse failure than the one being guarded against.
    println!(
        "preflight {name}: ok — disk {} (needs {}, {state}), ram {} (needs {})",
        say(&disk),
        gib(need_disk),
        say(&ram),
        gib(profile.ram)
    );
    ExitCode::SUCCESS
}

fn usage(problem: &str) -> ExitCode {
    let names: Vec<&str> = PROFILES.iter().map(|p| p.name).collect();
    eprintln!("preflight: {problem}; profiles: {}", names.join(", "));
    ExitCode::from(2)
}

impl Cache {
    /// Where this cache lands, or why the profile does not apply on this platform.
    ///
    /// The two `verify` lanes are Linux/WSL only — upstream ships no Windows asset — so on
    /// Windows this reports inapplicable and lets the lane's own one-line refusal speak.
    fn root(self) -> Result<PathBuf, String> {
        match self {
            Self::Workspace => Ok(internal_tooling::target_dir()),
            // Both re-derive a path `dorc-verify` owns (`kani::build_root`,
            // `lean_build_root`). Repo plumbing may not depend on product crates, so the
            // rule is copied; if either seat moves its cache, move this with it.
            Self::Kani => user_cache().map(|c| c.join("dorc-kani-target")),
            Self::Lean => user_cache().map(|c| c.join("dorc-minispec-lean")),
        }
    }

    /// The path whose existence means this cache is already populated.
    ///
    /// Not the root itself for the workspace: cargo creates `target/` with a `CACHEDIR.TAG`
    /// and nothing else, so the root's existence would read a from-scratch tree as warm and
    /// wave through the one case that needs the full footprint.
    fn witness(self, root: &Path) -> PathBuf {
        match self {
            Self::Workspace => root.join("debug"),
            Self::Kani | Self::Lean => root.to_path_buf(),
        }
    }
}

/// `XDG_CACHE_HOME`, else `$HOME/.cache`, matching what `dorc-verify` resolves.
fn user_cache() -> Result<PathBuf, String> {
    if cfg!(windows) {
        return Err("this lane is Linux/WSL only".to_owned());
    }
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .ok_or_else(|| "neither XDG_CACHE_HOME nor HOME is set".to_owned())
}

/// Free bytes on the volume holding `path`, which need not exist yet.
fn free_disk(path: &Path) -> Result<u64, String> {
    let probe = path
        .ancestors()
        .find(|p| p.exists())
        .ok_or_else(|| format!("no existing ancestor of {}", path.display()))?;
    fs4::available_space(probe).map_err(|e| format!("{}: {e}", probe.display()))
}

/// Available RAM in bytes, or why we could not say.
fn free_ram() -> Result<u64, String> {
    #[cfg(target_os = "linux")]
    {
        meminfo_available()
    }
    #[cfg(windows)]
    {
        windows_free_physical()
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        Err("no RAM probe for this platform".to_owned())
    }
}

/// `MemAvailable` — the kernel's own estimate of what a new workload can claim, which is
/// what a build needs, not `MemFree`.
#[cfg(target_os = "linux")]
fn meminfo_available() -> Result<u64, String> {
    let text =
        std::fs::read_to_string("/proc/meminfo").map_err(|e| format!("/proc/meminfo: {e}"))?;
    text.lines()
        .find_map(|line| line.strip_prefix("MemAvailable:"))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|kib| kib.parse::<u64>().ok())
        .map(|kib| kib.saturating_mul(1024))
        .ok_or_else(|| "/proc/meminfo carries no MemAvailable".to_owned())
}

/// Windows has no `/proc`, and `unsafe` is forbidden workspace-wide, so the reading comes
/// from a native query rather than an FFI call.
///
/// `FreePhysicalMemory` excludes the reclaimable standby cache, so it UNDERSTATES what is
/// available. That errs toward refusing, which is the safe direction, and the bounds are
/// set low enough that the understatement never fires on a healthy box.
#[cfg(windows)]
fn windows_free_physical() -> Result<u64, String> {
    wmic_free_kib()
        .or_else(|_| powershell_free_kib())
        .map(|kib| kib.saturating_mul(1024))
}

/// ~0.1s. Deprecated by Microsoft (an optional feature-on-demand since Win11 24H2), which
/// is why it is a first choice rather than the only one.
#[cfg(windows)]
fn wmic_free_kib() -> Result<u64, String> {
    let root = std::env::var("SystemRoot").map_err(|e| format!("SystemRoot: {e}"))?;
    let exe = PathBuf::from(root).join("System32/wbem/wmic.exe");
    let out = Command::new(exe)
        .args(["OS", "get", "FreePhysicalMemory", "/format:list"])
        .output()
        .map_err(|e| format!("wmic: {e}"))?;
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|line| line.trim().strip_prefix("FreePhysicalMemory="))
        .and_then(|kib| kib.trim().parse::<u64>().ok())
        .ok_or_else(|| "wmic answered no FreePhysicalMemory".to_owned())
}

/// ~1s, and the reason the fast path is tried first. Resolved by name deliberately:
/// unlike `bash`, `powershell.exe` on `PATH` is not a trap.
#[cfg(windows)]
fn powershell_free_kib() -> Result<u64, String> {
    let out = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "(Get-CimInstance Win32_OperatingSystem).FreePhysicalMemory",
        ])
        .output()
        .map_err(|e| format!("powershell: {e}"))?;
    String::from_utf8_lossy(&out.stdout)
        .trim()
        .parse::<u64>()
        .map_err(|e| format!("powershell answered no number: {e}"))
}

/// `N.N GiB`, integer-only so a display path owes no float-cast lint carve-out.
pub(crate) fn gib(bytes: u64) -> String {
    let tenths = bytes.saturating_mul(10) / GIB;
    format!("{}.{} GiB", tenths / 10, tenths % 10)
}

/// A measurement, or the reason there isn't one — never a fabricated number.
fn say(measured: &Result<u64, String>) -> String {
    match measured {
        Ok(bytes) => format!("{} free", gib(*bytes)),
        Err(why) => format!("UNMEASURED ({why})"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Cache, PROFILES, free_disk, gib};

    #[test]
    fn every_profile_demands_less_when_warm_than_when_cold() {
        for profile in &PROFILES {
            assert!(
                profile.disk_warm <= profile.disk_cold,
                "{}: a warm cache cannot cost more than a cold one",
                profile.name
            );
            assert!(
                profile.ram > 0,
                "{}: a zero RAM bound checks nothing",
                profile.name
            );
        }
    }

    #[test]
    fn the_kani_ram_bound_clears_the_drivers_own_address_space_cap() {
        // `verify/src/kani.rs` sets ADDRESS_SPACE_CAP_KB = 6_000_000 and CBMC inherits it.
        // A bound at or below that would pass a machine on which the very first harness
        // cannot fit — the exact shape of the OOM this profile exists to prevent.
        let cap = 6_000_000_u64 * 1024;
        let kani = PROFILES
            .iter()
            .find(|p| p.name == "kani")
            .expect("kani profile");
        assert!(
            kani.ram > cap,
            "kani RAM bound must exceed the per-harness cap"
        );
    }

    #[test]
    fn a_volume_is_measurable_through_a_path_that_does_not_exist_yet() {
        // The cold case: preflight runs before the target dir is created, so the probe has
        // to walk up to an ancestor that does exist rather than erroring out.
        let absent = internal_tooling::repo_root()
            .join("no-such-dir")
            .join("nor-this-one");
        assert!(free_disk(&absent).is_ok(), "cold probe must still answer");
    }

    #[test]
    fn sizes_render_without_floating_point() {
        assert_eq!(gib(0), "0.0 GiB");
        assert_eq!(gib(1024 * 1024 * 1024), "1.0 GiB");
        assert_eq!(gib(1024 * 1024 * 1024 * 3 / 2), "1.5 GiB");
    }

    #[test]
    fn the_verify_lanes_are_inapplicable_on_windows() {
        // They are Linux/WSL only, so on Windows preflight must decline to invent a bound
        // rather than measuring some invented cache path.
        assert_eq!(Cache::Kani.root().is_err(), cfg!(windows));
        assert_eq!(Cache::Lean.root().is_err(), cfg!(windows));
        assert!(Cache::Workspace.root().is_ok());
    }
}
