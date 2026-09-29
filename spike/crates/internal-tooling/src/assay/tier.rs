use crate::alloy_jvm::adapter::{Budget, Machine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Tier {
    Hot,
    Gate,
    Official,
}

pub(super) const CEILING_CPU_S: u64 = 1800;
pub(super) const CEILING_HEAP_MB: u64 = 4096;
pub(super) const DEFER_CLAUSES: u64 = 2_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Caps {
    pub(super) budget: Budget,
    pub(super) machine: Machine,
    pub(super) batch_s: Option<u64>,
}

impl Tier {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Hot => "hot",
            Self::Gate => "gate",
            Self::Official => "official",
        }
    }

    pub(super) fn caps(self) -> Caps {
        let (cpu_s, heap_mb, batch_s) = match self {
            Self::Hot => (120, 2048, Some(540)),
            Self::Gate => (600, 2048, Some(540)),
            Self::Official => (CEILING_CPU_S, CEILING_HEAP_MB, None),
        };
        Caps {
            budget: Budget {
                cpu_s,
                wall_s: cpu_s.saturating_mul(2),
            },
            machine: Machine { heap_mb, procs: 2 },
            batch_s,
        }
    }

    pub(super) fn trusts_keys(self) -> bool {
        self != Self::Official
    }

    pub(super) fn defers(self) -> bool {
        self == Self::Hot
    }
}

pub(super) fn with_overrides(mut caps: Caps, args: &[String]) -> Result<Caps, String> {
    let mut it = args.iter();
    let (mut wall_given, mut cpu_given) = (false, false);
    while let Some(flag) = it.next() {
        let n = it
            .next()
            .and_then(|v| v.parse::<u64>().ok())
            .ok_or_else(|| format!("{flag} takes a number"))?;
        match flag.as_str() {
            "--cpu" => {
                caps.budget.cpu_s = n;
                cpu_given = true;
            }
            "--timeout" => {
                caps.budget.wall_s = n;
                wall_given = true;
            }
            "--heap" => caps.machine.heap_mb = n,
            "--procs" => {
                caps.machine.procs = u32::try_from(n).map_err(|_| "--procs is too large")?;
            }
            "--batch-timeout" => caps.batch_s = Some(n),
            other => return Err(format!("unknown cap {other}")),
        }
    }
    match (wall_given, cpu_given) {
        (true, false) => caps.budget.cpu_s = caps.budget.wall_s,
        (false, true) => caps.budget.wall_s = caps.budget.cpu_s.saturating_mul(2),
        _ => {}
    }
    Ok(caps)
}

#[cfg(test)]
mod tests {
    use super::{Tier, with_overrides};

    #[test]
    fn caps_override_a_tier_and_wall_follows_cpu_unless_named() {
        let caps = with_overrides(Tier::Hot.caps(), &["--cpu".to_owned(), "300".to_owned()])
            .expect("a number");
        assert_eq!((caps.budget.cpu_s, caps.budget.wall_s), (300, 600));
        let caps = with_overrides(
            Tier::Gate.caps(),
            &[
                "--timeout".to_owned(),
                "30".to_owned(),
                "--batch-timeout".to_owned(),
                "60".to_owned(),
            ],
        )
        .expect("numbers");
        assert_eq!(
            (caps.budget.cpu_s, caps.budget.wall_s, caps.batch_s),
            (30, 30, Some(60)),
            "a wall cap alone caps CPU too, as `mise run alloy` reads it"
        );
        assert!(with_overrides(Tier::Hot.caps(), &["--cpu".to_owned()]).is_err());
        assert!(!Tier::Official.trusts_keys() && Tier::Official.caps().batch_s.is_none());
    }
}
