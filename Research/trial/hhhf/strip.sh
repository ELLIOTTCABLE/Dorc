#!/bin/sh
# hhhf/strip.sh — the MANDATORY ANSI-stripping extractor (round-25 field trial, DISPOSABLE)
#
# asciinema is the AUDIT/redundancy rail (the JSONL command-spine is authoritative). His plugin
# stack (fast-syntax-highlighting + autosuggestions + pure) redraws the whole line on every
# keystroke, so a raw .cast is ANSI-dense to the point of being unreadable. This flattens it to
# an LLM-readable transcript: decode the cast to the raw terminal stream, drop ANSI/OSC, and
# collapse the per-keystroke carriage-return redraws down to the final rendered line.
#
# Best-effort by design (it is the redundancy rail, not the spine). A pixel-perfect transcript
# would need a terminal emulator (e.g. pyte) — an upgrade path only if the audit rail proves too
# lossy on the day.
#
# Usage:  strip.sh <session.cast> [transcript.txt]      (no out-file => stdout)

set -eu

cast=${1:?usage: strip.sh <session.cast> [out.txt]}
out=${2:-}

# Decode the .cast JSON to the raw byte stream. asciinema recorded it, so `asciinema cat` is the
# guaranteed decoder; python3 is the portable fallback for stripping on another box.
decode() {
   if command -v asciinema >/dev/null 2>&1; then
      asciinema cat "$cast"
   elif command -v python3 >/dev/null 2>&1; then
      python3 - "$cast" <<'PY'
import json, sys
with open(sys.argv[1], encoding="utf-8", errors="replace") as f:
   next(f, None)                       # header line
   for line in f:
      line = line.strip()
      if not line:
         continue
      try:
         ev = json.loads(line)
      except ValueError:
         continue
      if len(ev) >= 3 and ev[1] == "o":
         sys.stdout.write(ev[2])
PY
   else
      echo "strip.sh: need asciinema or python3 to decode $cast" >&2
      exit 1
   fi
}

strip() {
   awk '
   BEGIN {
      esc = sprintf("%c", 27)
      bel = sprintf("%c", 7)
      cr  = sprintf("%c", 13)
      bs  = sprintf("%c", 8)
      csi = esc "[[]" "[0-9;?]*" "[ -/]*" "[@-~]"   # SGR / cursor / erase ([[] = literal [)
      osc = esc "[]]" "[^" bel "]*" bel             # window-title etc, BEL-terminated ([]] = literal ])
      st  = esc "\\\\"                              # string terminator (ESC backslash)
   }
   {
      gsub(osc, "")
      gsub(csi, "")
      gsub(st,  "")
      gsub(esc "[()][0-9A-Za-z]", "")   # charset selects: ESC( / ESC)
      gsub(esc, "")                     # any lone ESC left over
      sub(".*" cr, "")                  # keep only the final overwrite on this line
      gsub(bel, ""); gsub(bs, "")
      if ($0 == "") { if (blank++) next } else blank = 0
      print
   }'
}

if [ -n "$out" ]; then
   decode | strip > "$out"
   echo "strip.sh: transcript -> $out" >&2
else
   decode | strip
fi
