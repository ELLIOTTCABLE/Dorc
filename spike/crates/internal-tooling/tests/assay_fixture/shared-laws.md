> assay self-test fixture; it means nothing.

# Shared laws

The append half: spliced after the document's own definitions, so it may name them.

```alloy
check wobblyNeedsAWobble { all l: Line | wobbly[l] implies some Wobble & l.speech } for 3 but 2 Int
```
