> assay self-test fixture; it means nothing.

# Shared

The prepend half: opened beneath every module assay generates.

```alloy
sig Speaker {}
abstract sig Blurb extends Claim { speaker: one Speaker }
sig Heard in Blurb {}
fact { all b: Blurb | b in Heard iff b in Line.speech }
fact { all l: Line | l not in l.^above }

run bookScope {} for 4 but 3 Int
```
