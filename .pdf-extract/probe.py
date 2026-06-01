#!/usr/bin/env python3
"""Probe candidate PDFs: report extractable-text volume + a sample, to tell born-digital from scanned."""
import sys, fitz

for path in sys.argv[1:]:
   try:
      d = fitz.open(path)
      txt = "".join(p.get_text("text") for p in d)
      per_page = len(txt) / max(d.page_count, 1)
      verdict = "BORN-DIGITAL" if per_page > 800 else "SCANNED/garbage (needs OCR)"
      print(f"{len(txt):>8} chars  {d.page_count:>3}p  {per_page:6.0f} ch/pg  {verdict}  {path}")
      print(f"   sample: {txt[:200]!r}")
   except Exception as e:
      print(f"FAIL {path}: {e!r}")
