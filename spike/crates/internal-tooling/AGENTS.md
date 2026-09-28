repo plumbing behind `mise run`; nothing depends on this crate. assay lives here: its design is `Research/notes/30Y`, the praxis for writing against it `Research/plans/30Z`.

- assay knows shell and nothing about dorc; never teach it a dorc-aware lint or default
- it reads alloy only as far as sig heads and binders; alloy itself does the rest
- no committed test runs a jvm; alloy runs are hand runs, recorded in the lane note
- the lock holds results, never timings; a red the committed lock records is a pass
- `exclusive` is re-entrant via one inherited env var; `:held` twins bypass it, on purpose
