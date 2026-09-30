//! Where a solving pass's progress lines go: one `tracing` subscriber on stderr, never stdout,
//! which carries the report (`notes/30Y` § 2.7). Only `--check` and `--write` install it, so every
//! other mode's events, and the runner's, go nowhere.

use std::time::Instant;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

use crate::alloy_jvm::adapter::human;

/// Every line leads with the time since the pass began, so the last line on a dead terminal says
/// how long it had run.
struct SinceInstalled(Instant);

impl FormatTime for SinceInstalled {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "assay +{}", human(self.0.elapsed()))
    }
}

/// Progress is `info`; `--quiet` admits `warn` and above, which today is nothing.
fn subscriber<W>(quiet: bool, writer: W) -> impl tracing::Subscriber + Send + Sync + 'static
where
    W: for<'w> MakeWriter<'w> + Send + Sync + 'static,
{
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .with_target(false)
        .with_level(false)
        .with_timer(SinceInstalled(Instant::now()))
        .with_max_level(if quiet {
            LevelFilter::WARN
        } else {
            LevelFilter::INFO
        })
        .finish()
}

pub(super) fn install(quiet: bool) {
    // Only fails if a subscriber is already set, which leaves that one in charge.
    let _ = tracing::subscriber::set_global_default(subscriber(quiet, std::io::stderr));
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    use super::subscriber;

    #[derive(Clone, Default)]
    struct Shared(Arc<Mutex<Vec<u8>>>);

    impl Write for Shared {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0
                .lock()
                .map_err(|_| std::io::Error::other("poisoned"))?
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn heard(quiet: bool) -> String {
        let sink = Shared::default();
        let writer = sink.clone();
        tracing::subscriber::with_default(subscriber(quiet, move || writer.clone()), || {
            tracing::info_span!("solve", module = "laws").in_scope(|| {
                tracing::info!("solving");
            });
        });
        let bytes = sink.0.lock().map(|b| b.clone()).unwrap_or_default();
        String::from_utf8(bytes).expect("utf-8")
    }

    #[test]
    fn quiet_writes_nothing_and_loud_writes_one_plain_line() {
        // `--quiet` is what CI and agents rely on for zero bytes; the audible form must be one
        // terminated line with no escape codes, or concurrent children could split each other's
        // lines and a log file would carry colour codes.
        assert!(heard(true).is_empty());
        let loud = heard(false);
        assert_eq!(loud.matches('\n').count(), 1);
        assert!(loud.starts_with("assay +") && loud.contains("solving"));
        assert!(!loud.contains('\u{1b}'));
    }
}
