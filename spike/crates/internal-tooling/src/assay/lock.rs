//! The lock, schema 2 (`notes/30Yf` § 2): definite results and unmeasurements as their own kinds,
//! each row with the budget it was measured under, and an asymmetric match: a definite result may
//! be written at any budget, an unmeasurement never overwrites one and never matches one.

use crate::alloy_jvm::adapter::Size;
use crate::json::{Json, Value};

use super::tier::{CEILING_CPU_S, CEILING_HEAP_MB};

const SCHEMA: u64 = 2;

/// What a command came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Outcome {
    Sat,
    Unsat,
    Counterexample,
    NoCounterexample,
    Timeout,
    OutOfMemory,
    PlatformFail,
    UnsupportedHere,
    NotRun,
    Deferred,
    /// A deterministic refusal of the text (a scope Alloy demands at solve time, say): neither a
    /// fact about the model nor an absence of measurement.
    Error,
}

const OUTCOMES: [(Outcome, &str); 11] = [
    (Outcome::Sat, "sat"),
    (Outcome::Unsat, "unsat"),
    (Outcome::Counterexample, "counterexample"),
    (Outcome::NoCounterexample, "no-counterexample"),
    (Outcome::Timeout, "timeout"),
    (Outcome::OutOfMemory, "out-of-memory"),
    (Outcome::PlatformFail, "platform-fail"),
    (Outcome::UnsupportedHere, "unsupported-here"),
    (Outcome::NotRun, "not-run"),
    (Outcome::Deferred, "deferred"),
    (Outcome::Error, "error"),
];

impl Outcome {
    pub(super) fn name(self) -> &'static str {
        OUTCOMES
            .iter()
            .find(|(o, _)| *o == self)
            .map_or("error", |(_, n)| n)
    }

    fn parse(name: &str) -> Option<Self> {
        OUTCOMES.iter().find(|(_, n)| *n == name).map(|(o, _)| *o)
    }

    pub(super) fn definite(self) -> bool {
        matches!(
            self,
            Self::Sat | Self::Unsat | Self::Counterexample | Self::NoCounterexample
        )
    }

    pub(super) fn unmeasured(self) -> bool {
        !self.definite() && self != Self::Error
    }

    pub(super) fn green(self) -> bool {
        matches!(self, Self::Sat | Self::NoCounterexample)
    }
}

/// One lock row. `key` is absent on a row read from schema 1, which therefore counts as changed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Row {
    pub(super) module: String,
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) scope: String,
    pub(super) result: Outcome,
    pub(super) phase: Option<String>,
    pub(super) budget: Option<u64>,
    pub(super) heap: Option<u64>,
    pub(super) size: Option<Size>,
    pub(super) key: Option<String>,
    pub(super) platform: Option<String>,
    pub(super) message: Option<String>,
}

impl Row {
    pub(super) fn json(&self) -> Json {
        let mut fields = vec![
            ("module".to_owned(), Json::str(&self.module)),
            ("name".to_owned(), Json::str(&self.name)),
            ("kind".to_owned(), Json::str(&self.kind)),
            ("scope".to_owned(), Json::str(&self.scope)),
            ("result".to_owned(), Json::str(self.result.name())),
        ];
        let mut opt = |k: &str, v: Option<Json>| {
            if let Some(v) = v {
                fields.push((k.to_owned(), v));
            }
        };
        opt("phase", self.phase.as_deref().map(Json::str));
        opt("budget", self.budget.map(Json::Num));
        opt("heap", self.heap.map(Json::Num));
        opt(
            "size",
            self.size.map(|s| {
                Json::obj([
                    ("primary_vars", Json::Num(s.primary_vars)),
                    ("vars", Json::Num(s.vars)),
                    ("clauses", Json::Num(s.clauses)),
                ])
            }),
        );
        opt("key", self.key.as_deref().map(Json::str));
        opt("platform", self.platform.as_deref().map(Json::str));
        opt("message", self.message.as_deref().map(Json::str));
        Json::Obj(fields)
    }

    fn read(v: &Value, schema: u64) -> Option<Self> {
        let current = schema >= SCHEMA;
        let size = v.get("size").and_then(|s| {
            Some(Size {
                primary_vars: s.u64("primary_vars")?,
                vars: s.u64("vars")?,
                clauses: s.u64("clauses")?,
            })
        });
        Some(Self {
            module: v.str("module")?.to_owned(),
            name: v.str("name")?.to_owned(),
            kind: v.str("kind")?.to_owned(),
            scope: v.str("scope")?.to_owned(),
            result: Outcome::parse(v.str("result")?)?,
            phase: v.str("phase").map(str::to_owned),
            budget: v.u64("budget").filter(|_| current),
            heap: v.u64("heap").filter(|_| current),
            size: size.filter(|_| current),
            key: v.str("key").filter(|_| current).map(str::to_owned),
            platform: v.str("platform").map(str::to_owned),
            message: v.str("message").map(str::to_owned),
        })
    }

    fn id(&self) -> (&str, &str) {
        (&self.module, &self.name)
    }

    /// Would a bigger budget, or a bigger heap, owe this row another run?
    fn early(&self) -> bool {
        match self.result {
            Outcome::Timeout => self.budget.is_none_or(|b| b < CEILING_CPU_S),
            Outcome::OutOfMemory => self.heap.is_none_or(|h| h < CEILING_HEAP_MB),
            _ => false,
        }
    }
}

/// The lock file: a schema line, then one row per line, so its git diff is a row diff.
pub(super) fn render(rows: &[Row]) -> String {
    let mut lines = vec![format!(
        "  {}",
        Json::obj([("schema", Json::Num(SCHEMA))]).line()
    )];
    lines.extend(rows.iter().map(|r| format!("  {}", r.json().line())));
    format!("[\n{}\n]\n", lines.join(",\n"))
}

/// Rows of a lock file of either schema; `None` when it does not read.
pub(super) fn parse(text: &str) -> Option<Vec<Row>> {
    let Value::Arr(items) = Value::parse(text)? else {
        return None;
    };
    let schema = items.iter().find_map(|v| v.u64("schema")).unwrap_or(1);
    items
        .iter()
        .filter(|v| v.get("schema").is_none())
        .map(|v| Row::read(v, schema))
        .collect()
}

/// How one row stands against the committed lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Standing {
    Green,
    AcceptedRed,
    /// A full timeout, a full out-of-memory, a platform failure, or an unsupported command the
    /// committed lock records, on an unchanged key.
    AcceptedUnmeasured,
    AcceptedError,
    /// A definite result on an unchanged key that this run did not re-measure.
    Carried,
    Mismatch(Why),
    Unmeasured,
    /// An early timeout or out-of-memory the lock records: it owes one run at the ceiling.
    Owed,
    /// Deferred by the hot tier's design; listed, never green, never counted.
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Why {
    New,
    Moved,
}

impl Standing {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::AcceptedRed => "accepted-red",
            Self::AcceptedUnmeasured => "accepted-unmeasured",
            Self::AcceptedError => "accepted-error",
            Self::Carried => "carried",
            Self::Mismatch(Why::New) => "mismatch-new",
            Self::Mismatch(Why::Moved) => "mismatch-moved",
            Self::Unmeasured => "unmeasured",
            Self::Owed => "owed",
            Self::Deferred => "deferred",
        }
    }
}

/// The asymmetric match for one computed row (`notes/30Yf` `lock-asymmetric-match`).
pub(super) fn judge(committed: Option<&Row>, computed: &Row) -> Standing {
    let same_key = |c: &Row| c.key.is_some() && c.key == computed.key;
    let x = computed.result;
    let Some(c) = committed else {
        return match x {
            _ if x.definite() || x == Outcome::Error => Standing::Mismatch(Why::New),
            Outcome::Deferred => Standing::Deferred,
            _ => Standing::Unmeasured,
        };
    };
    if x.definite() {
        return match (c.result == x, x.green()) {
            (true, true) => Standing::Green,
            (true, false) => Standing::AcceptedRed,
            (false, _) => Standing::Mismatch(Why::Moved),
        };
    }
    if x == Outcome::Error {
        return if c.result == Outcome::Error && same_key(c) && c.message == computed.message {
            Standing::AcceptedError
        } else {
            Standing::Mismatch(Why::Moved)
        };
    }
    let unmeasured = if x == Outcome::Deferred {
        Standing::Deferred
    } else {
        Standing::Unmeasured
    };
    match c.result {
        Outcome::PlatformFail if x == Outcome::PlatformFail => Standing::AcceptedUnmeasured,
        r if r.definite() && same_key(c) => Standing::Carried,
        r if r.unmeasured() && same_key(c) => {
            if c.early() || matches!(r, Outcome::NotRun | Outcome::Deferred) {
                Standing::Owed
            } else {
                Standing::AcceptedUnmeasured
            }
        }
        _ => unmeasured,
    }
}

/// The lock a write leaves (`notes/30Yf` `lock-tier-invariant`): definite and error rows as
/// computed; an unmeasurement never replaces a committed row, which stays with its old key; and
/// only a tier that makes the lock whole writes an unmeasurement at all.
pub(super) fn next(committed: &[Row], computed: &[Row], whole: bool) -> Vec<Row> {
    computed
        .iter()
        .filter_map(|x| {
            let c = committed.iter().find(|c| c.id() == x.id());
            if !x.result.unmeasured() || whole {
                Some(x.clone())
            } else {
                c.cloned()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Outcome, Row, Standing, Why, judge, next, parse, render};

    fn row(name: &str, result: Outcome, key: &str) -> Row {
        Row {
            module: "laws".to_owned(),
            name: name.to_owned(),
            kind: "check".to_owned(),
            scope: "6".to_owned(),
            result,
            phase: None,
            budget: Some(120),
            heap: Some(2048),
            size: None,
            key: Some(key.to_owned()),
            platform: Some("linux".to_owned()),
            message: None,
        }
    }

    #[test]
    fn a_definite_result_needs_its_equal_and_a_budget_never_decides_it() {
        // A counterexample found at any budget against a locked no-counterexample is wrong, and a
        // faster machine finding the same verdict elsewhere is the same verdict.
        let locked = row("k", Outcome::NoCounterexample, "a");
        let mut again = row("k", Outcome::NoCounterexample, "b");
        again.budget = Some(1800);
        assert_eq!(judge(Some(&locked), &again), Standing::Green);
        assert_eq!(
            judge(Some(&locked), &row("k", Outcome::Counterexample, "a")),
            Standing::Mismatch(Why::Moved)
        );
        let red = row("k", Outcome::Counterexample, "a");
        assert_eq!(judge(Some(&red), &red.clone()), Standing::AcceptedRed);
        assert_eq!(judge(None, &locked), Standing::Mismatch(Why::New));
    }

    #[test]
    fn an_unmeasurement_never_matches_a_definite_result_on_a_changed_key() {
        // The unsoundness this schema removes: a slow machine passing a new red by timing out.
        let locked = row("k", Outcome::NoCounterexample, "a");
        assert_eq!(
            judge(Some(&locked), &row("k", Outcome::Timeout, "b")),
            Standing::Unmeasured
        );
        assert_eq!(
            judge(Some(&locked), &row("k", Outcome::Timeout, "a")),
            Standing::Carried
        );
        assert_eq!(
            judge(Some(&locked), &row("k", Outcome::Deferred, "b")),
            Standing::Deferred
        );
        let mut v1 = locked.clone();
        v1.key = None;
        assert_eq!(
            judge(Some(&v1), &row("k", Outcome::Timeout, "a")),
            Standing::Unmeasured,
            "an unknown key cannot prove the text unchanged"
        );
    }

    #[test]
    fn a_timeout_is_early_below_the_ceiling_and_accepted_at_it() {
        // Early owes one run at the ceiling; full owes analysis or acceptance, and committing the
        // lock is the acceptance.
        let early = row("k", Outcome::Timeout, "a");
        assert_eq!(
            judge(Some(&early), &row("k", Outcome::Timeout, "a")),
            Standing::Owed
        );
        let mut full = early.clone();
        full.budget = Some(1800);
        assert_eq!(
            judge(Some(&full), &row("k", Outcome::Timeout, "a")),
            Standing::AcceptedUnmeasured
        );
        assert_eq!(
            judge(Some(&full), &row("k", Outcome::Timeout, "b")),
            Standing::Unmeasured
        );
        let mut oom = row("k", Outcome::OutOfMemory, "a");
        assert_eq!(
            judge(Some(&oom), &oom.clone()),
            Standing::Owed,
            "2048 MB is below the ceiling heap"
        );
        oom.heap = Some(4096);
        assert_eq!(
            judge(Some(&oom), &oom.clone()),
            Standing::AcceptedUnmeasured
        );
        assert_eq!(
            judge(Some(&full), &row("k", Outcome::NoCounterexample, "a")),
            Standing::Mismatch(Why::Moved),
            "a measurement where the lock had none moves the lock"
        );
    }

    #[test]
    fn an_error_is_accepted_only_as_itself_on_its_own_key() {
        let mut e = row("k", Outcome::Error, "a");
        e.message = Some("scope".to_owned());
        assert_eq!(judge(Some(&e), &e.clone()), Standing::AcceptedError);
        let mut other = e.clone();
        other.message = Some("another".to_owned());
        assert_eq!(judge(Some(&e), &other), Standing::Mismatch(Why::Moved));
        let mut rekeyed = e.clone();
        rekeyed.key = Some("b".to_owned());
        assert_eq!(judge(Some(&e), &rekeyed), Standing::Mismatch(Why::Moved));
    }

    #[test]
    fn a_lower_tier_never_makes_the_lock_worse() {
        // Only the official tier writes an unmeasurement; below it a committed row survives,
        // old key and all, so its next check still sees the text as changed.
        let committed = vec![row("kept", Outcome::NoCounterexample, "a")];
        let computed = vec![
            row("kept", Outcome::Timeout, "b"),
            row("fresh", Outcome::Sat, "c"),
            row("slow", Outcome::Timeout, "d"),
        ];
        let partial = next(&committed, &computed, false);
        assert_eq!(partial, vec![committed[0].clone(), computed[1].clone()]);
        assert_eq!(next(&committed, &computed, true), computed);
    }

    #[test]
    fn the_file_round_trips_and_schema_one_reads_as_unkeyed() {
        let mut r = row("k", Outcome::Timeout, "a");
        r.phase = Some("translating".to_owned());
        let rows = vec![r, row("j", Outcome::Sat, "b")];
        assert_eq!(parse(&render(&rows)), Some(rows));
        let v1 = "[\n  {\"module\": \"laws\", \"name\": \"k\", \"kind\": \"check\", \"scope\": \"6\", \"result\": \"timeout\", \"premise\": \"sat\", \"hash\": \"0123456789abcdef\"}\n]\n";
        let old = parse(v1).expect("schema 1 reads");
        assert_eq!((old[0].key.as_deref(), old[0].budget), (None, None));
    }
}
