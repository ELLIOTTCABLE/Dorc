repo plumbing behind `mise run`; nothing depends on this crate.

- `exclusive` is re-entrant via one inherited env var; `:held` twins bypass it, on purpose

## assay
assay lives here: its design is `Research/notes/30Y`, the praxis for writing against it `Research/plans/30Z`.

- assay knows shell and nothing about dorc; avoid teaching it a dorc-aware lint or default
- it reads alloy only as far as sig heads and binders; alloy itself does the rest
- the lock holds results; a set of reds the committed lock already records (with no new ones) is a pass
