/-!
# minispec

The law corpus's root module — deliberately EMPTY of imports. The lakefile's
`Minispec.+` glob builds every unit as its own target, so a unit that stops
elaborating fails the build on its own; a hand-maintained import list here adds
nothing to that and can only drift (one did, silently, within days of the
corpus's minting — caught by the r30 review pair, `30B`/`30C`/`30D`).

A new unit needs no edit here, or anywhere: file it under `Minispec/` and the
glob owns it.
-/
