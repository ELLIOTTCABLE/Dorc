#!/usr/bin/awk -f
# gate-2 (20B §3): the exec-sandbox redirection scanner. Reads a rendered artifact on
# stdin (our OWN render — the probe or apply sh) and REFUSES (exit 1, printing the
# offending line) any redirection whose target word is unsafe to write under the
# sandbox. The exec harness runs the artifact with cwd = a throwaway mktemp dir, so a
# *relative* target lands in disposable space; this scan stops the targets that escape
# that sandbox:
#   - ABSOLUTE  (`>/etc/x`)              — writes outside the sandbox
#   - DYNAMIC   (`>$x`, `>`backtick`)    — target unknowable at scan time ⇒ refuse
#   - ESCAPING  (`>../x`)                — climbs out of the sandbox
# Allowlist: exactly `/dev/null` (the inert sink the renders use), and fd-duplications
# (`>&2`, `2>&1`) which name a descriptor, not a file.
#
# CONSERVATIVE BY MANDATE (20B §3 "a conservative over-refusal is fine"): this is a
# lexical scan over known-shape renders, NOT a sh parser. A comment (`#…`) is skipped
# wholesale; a here-doc operator (`<<`) is treated as a refusable redirect-to-word
# unless its delimiter is a bare relative token (here-docs don't write files, but our
# renders never emit them — flag if one appears). When it over-refuses, the harness
# reports WHICH line tripped it, so a false positive is legible, not silent.
#
# Determinism: a single forward pass, no clock/rng/fs (`inv-determinism`); the artifact
# text is the sole input.

# Strip the unquoted trailing comment from a line so a `# …/abs/path` in provenance
# does not trip the scan. CONSERVATIVE: we only strip a `#` that begins a word (start of
# line, or after whitespace) and is not inside a single/double quote on that line. A `#`
# mid-word (`a#b`) is not a comment in sh, so it is left alone.
function strip_comment(line,   i, c, inS, inD, prevblank) {
   inS = 0; inD = 0; prevblank = 1
   for (i = 1; i <= length(line); i++) {
      c = substr(line, i, 1)
      if (c == "'" && !inD) inS = !inS
      else if (c == "\"" && !inS) inD = !inD
      else if (c == "#" && !inS && !inD && prevblank) return substr(line, 1, i - 1)
      prevblank = (c == " " || c == "\t")
   }
   return line
}

# Is `w` an UNSAFE redirect target? (the refuse predicate)
function unsafe_target(w) {
   if (w == "/dev/null") return 0          # the sole file allowlist
   if (w ~ /^&/) return 0                   # fd-dup (`&1`, `&2`) — a descriptor, not a file
   if (w ~ /[$`]/) return 1                  # dynamic ⇒ refuse (target unknowable)
   if (w ~ /^\//) return 1                   # absolute ⇒ refuse
   if (w ~ /(^|\/)\.\.(\/|$)/) return 1     # contains a `..` segment ⇒ escapes
   return 0                                  # a bare relative target ⇒ lands in the sandbox
}

{
   line = strip_comment($0)
   # Walk the line char-by-char, tracking quote state, looking for redirect operators
   # OUTSIDE quotes. On one, capture the following word (skipping blanks) and test it.
   n = length(line)
   inS = 0; inD = 0
   i = 1
   while (i <= n) {
      c = substr(line, i, 1)
      if (c == "'" && !inD) { inS = !inS; i++; continue }
      if (c == "\"" && !inS) { inD = !inD; i++; continue }
      if (inS || inD) { i++; continue }
      if (c == ">" || c == "<") {
         # Consume the operator run (`>`, `>>`, `<<`, and a leading/!following `&`).
         j = i + 1
         # `>>`/`<<` doubled operator
         if (substr(line, j, 1) == c) j++
         # `>&`/`<&` fd-dup operator — the target is the fd word that follows
         # skip blanks before the target word
         while (substr(line, j, 1) == " " || substr(line, j, 1) == "\t") j++
         # capture the target word: up to the next blank, redirect op, `;`, `&`, `|`,
         # `)`, or quote boundary. Keep a leading `&` (fd-dup marker) as part of it.
         word = ""
         while (j <= n) {
            cc = substr(line, j, 1)
            if (cc == " " || cc == "\t" || cc == ";" || cc == "|" || cc == ")" || cc == "(" || cc == ">" || cc == "<") break
            # an unquoted `&` ALONE terminates (background), but `&N`/`&-` right after the
            # operator is a fd-dup target — include it only if it is the first char.
            if (cc == "&" && word != "") break
            word = word cc
            j++
         }
         if (word != "" && unsafe_target(word)) {
            print $0          # the offending line → stdout; the sh caller routes it
            bad = 1
         }
         i = j
         continue
      }
      i++
   }
}

END { if (bad) exit 1 }
