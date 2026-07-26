//! Rendering a [`RunInstant`] — the describe plane's one seat for turning a controller-minted
//! moment into bytes a reader can date a receipt by.
//!
//! Every instant Dorc holds is CONTROLLER-minted (`28F:rul-probe-instants-host-says-no-times`,
//! human-typed: the host says no times, ever). So this module converts one number the controller
//! read from its own clock; it never interprets anything a managed host said, and it never reads a
//! clock itself (`aid-is-dst-clean` — a pure function of its argument).
//!
//! UTC, always. A receipt that dated itself in the reader's local zone would be a different
//! sentence on two machines reading one durable, and the durable stores no zone to reconstruct.

use dorc_core::RunInstant;

/// A civil date-and-time, broken out of an epoch instant.
///
/// Fields are display material only; nothing here is comparable, so no decision can be spelled
/// against one (the two-plane seal is by construction — there is no route back).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Civil {
    pub year: u64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
    pub second: u64,
}

/// Break an instant into its UTC civil parts.
///
/// The days-to-civil arithmetic is the standard shift-the-epoch-to-March algorithm, which needs
/// no table and no dependency — the crate's zero-nondeterminism bar rules out reaching for a date
/// library (`aid-is-dst-clean`). It runs entirely in unsigned arithmetic, which the epoch origin
/// makes total: a [`RunInstant`] is milliseconds SINCE the epoch, so no intermediate here can go
/// negative and every subtraction below is ordered by construction.
#[must_use]
pub fn civil(at: RunInstant) -> Civil {
    let seconds = at.0 / 1_000;
    let time_of_day = seconds % 86_400;

    let shifted = (seconds / 86_400).saturating_add(719_468);
    let era = shifted / 146_097;
    let day_of_era = shifted % 146_097;
    let year_of_era = day_of_era
        .saturating_sub(day_of_era / 1_460)
        .saturating_add(day_of_era / 36_524)
        .saturating_sub(day_of_era / 146_096)
        / 365;
    let day_of_year = day_of_era.saturating_sub(
        year_of_era
            .saturating_mul(365)
            .saturating_add(year_of_era / 4)
            .saturating_sub(year_of_era / 100),
    );
    let march_month = day_of_year.saturating_mul(5).saturating_add(2) / 153;
    let day = day_of_year
        .saturating_sub(march_month.saturating_mul(153).saturating_add(2) / 5)
        .saturating_add(1);
    let month = if march_month < 10 {
        march_month.saturating_add(3)
    } else {
        march_month.saturating_sub(9)
    };

    Civil {
        year: year_of_era
            .saturating_add(era.saturating_mul(400))
            .saturating_add(u64::from(month <= 2)),
        month,
        day,
        hour: time_of_day / 3_600,
        minute: time_of_day % 3_600 / 60,
        second: time_of_day % 60,
    }
}

/// `YYYY-MM-DD HH:MM:SS` — the receipt header's dating of a whole run.
#[must_use]
pub fn date_time_text(at: RunInstant) -> String {
    let c = civil(at);
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        c.year, c.month, c.day, c.hour, c.minute, c.second
    )
}

/// `HH:MM:SS` — a within-the-run moment, where the receipt header already carried the date.
#[must_use]
pub fn time_text(at: RunInstant) -> String {
    let c = civil(at);
    format!("{:02}:{:02}:{:02}", c.hour, c.minute, c.second)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The conversion is pinned against instants computed independently of this code, because a
    /// receipt that dates a run wrongly is a mis-attribution, the worst class of aid failure
    /// (`271:rul-sin-ordering`) — and an off-by-one in the era arithmetic is invisible by eye.
    #[test]
    fn known_epoch_instants_render_their_known_civil_time() {
        assert_eq!(date_time_text(RunInstant(0)), "1970-01-01 00:00:00");
        assert_eq!(
            date_time_text(RunInstant(1_769_306_437_000)),
            "2026-01-25 02:00:37"
        );
        // A leap day, and the day after it: the March-shifted arithmetic is exactly where a
        // hand-rolled converter goes wrong, so both sides of one are pinned.
        assert_eq!(
            date_time_text(RunInstant(1_709_164_800_000)),
            "2024-02-29 00:00:00"
        );
        assert_eq!(
            date_time_text(RunInstant(1_709_251_200_000)),
            "2024-03-01 00:00:00"
        );
    }

    /// Sub-second precision is dropped rather than rounded: a receipt says when a thing happened,
    /// and rounding 01:59:52.9 up to 01:59:53 would place an event after a later one that
    /// genuinely happened at 01:59:53.0.
    #[test]
    fn milliseconds_truncate_toward_the_second_that_contained_them() {
        assert_eq!(time_text(RunInstant(1_999)), "00:00:01");
    }
}
