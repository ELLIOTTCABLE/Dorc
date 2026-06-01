#!/usr/bin/env python3
"""Consolidate to ONE authoritative <stem>.txt per PDF; remove supplanted pdf2text files.

- Tip'95: re-source from the born-digital CWI report (downloaded), not the scanned journal PDF.
- Cousot'77: image-only scan, no born-digital source exists -> keep sparse extraction with a warning header.
- All others: my PyMuPDF (UTF-8) extraction supplants the earlier mojibake <stem>.txt.
- Removes every <stem>.pdf.txt after folding it into <stem>.txt.
"""
import sys, pathlib, fitz

REPO = pathlib.Path("/mnt/c/Users/ec/Sync/Code/Dorc")
DIRS = [REPO / "Research/papers", REPO / "Research/learning-path",
        REPO / "Vendor/SVF/docs/images", REPO / "Vendor/infer/website/static/downloads"]

TIP = "tip-program-slicing-survey-1995"
COUSOT = "cousot-abstract-interpretation-popl1977"
TIP_BORN_DIGITAL = pathlib.Path("/tmp/tip_gatech.pdf")

TIP_HEADER = ("[text source: born-digital CWI technical report CS-R9438 (Tip, 1994), "
   "https://faculty.cc.gatech.edu/~harrold/6340/cs6340_fall2009/Readings/tip94.pdf — "
   "content-equivalent to the journal version (jpl1995) whose PDF in this directory is image-only]\n\n")
COUSOT_HEADER = ("[WARNING: image-only scan; no born-digital source exists anywhere. The text below is a "
   "sparse/garbled OCR-layer fragment only — NOT the real paper. Full text requires OCR. "
   "Source: di.ens.fr/~cousot/publications.www/CousotCousot-POPL-77-ACM-p238--252-1977.pdf]\n\n")

def extract(pdf: pathlib.Path) -> str:
   doc = fitz.open(pdf)
   parts = []
   for i, page in enumerate(doc, 1):
      parts.append(f"\n\n──────── page {i}/{doc.page_count} ────────\n\n")
      parts.append(page.get_text("text"))
   return "".join(parts)

REPLACEMENT = chr(0xFFFD)

def mojibake_count(p: pathlib.Path) -> int:
   if not p.exists():
      return -1
   return p.read_text(encoding="utf-8", errors="replace").count(REPLACEMENT)

DRY = "--dry" in sys.argv
actions = []
for d in DIRS:
   for src in sorted(d.glob("*.pdf.txt")):
      stem = src.name[:-len(".pdf.txt")]
      target = d / f"{stem}.txt"
      old_mojibake = mojibake_count(target)  # before overwrite

      if stem == TIP:
         content = TIP_HEADER + extract(TIP_BORN_DIGITAL)
         note = f"RE-SOURCED born-digital CWI report ({len(content)} ch)"
      elif stem == COUSOT:
         content = COUSOT_HEADER + src.read_text(encoding="utf-8")
         note = "FLAGGED scan-only (OCR needed)"
      else:
         content = src.read_text(encoding="utf-8")
         note = f"extraction ({len(content)} ch)"

      if not DRY:
         target.write_text(content, encoding="utf-8")
         src.unlink()  # remove the .pdf.txt duplicate
      actions.append((str(d.relative_to(REPO)), stem, note, old_mojibake))

print(f"{'dir':40} {'stem':52} {'mojibake�':>9}  action")
for d, stem, note, moji in sorted(actions):
   m = "(no old)" if moji < 0 else str(moji)
   print(f"{d:40} {stem[:52]:52} {m:>9}  {note}")
verb = "WOULD consolidate" if DRY else "Consolidated"
print(f"\n{'[DRY RUN] ' if DRY else ''}{verb} {len(actions)} PDFs -> one <stem>.txt each; "
      f"{'would remove' if DRY else 'removed'} {len(actions)} .pdf.txt duplicates.")
print("Old <stem>.txt files (where present) were overwritten; 'mojibake�' = U+FFFD count in the overwritten file.")
