# while-read-file-rejects (`20O` find-3 / fix-2): the idiomatic `while read … done < file`
# shape. A redirection TRAILING a construct (`done < packages.txt`) redirects the WHOLE
# loop's stdin in dash — but Dorc does not yet model that (body-stdin consumption marking is
# a recorded later slice). BEFORE fix-2 the parser silently dropped the `< packages.txt` into
# a phantom empty-argv command with ZERO diagnostics — a silent ⊤ contradicting
# `inv-top-reject`. NOW the `while` collapses to a loud absorbing ⊤ (a `syntax-unsupported`
# naming the construct-redirect + a `cfg-top-node` pair); the ⊤ poisons everything below it.
# The apply emits the loop VERBATIM (never silently eliding past an unmodeled construct — the
# SAFE degrade, kFAIL-perform). Honest interim per 20O: full modeling of the construct's I/O
# is a later slice; the contract here is "loud, not silent".
while read line; do
   apt-get install -y "$line"
done < packages.txt
