#!/usr/bin/env python3
"""Dump every PDF in the repo tree to a UTF-8 text file beside it (<name>.pdf.txt).

Throwaway tooling for one-time text extraction (PyMuPDF). Not committed.
"""
import sys, time, pathlib
import fitz  # PyMuPDF

REPO = pathlib.Path(sys.argv[1]).resolve()
LOW_TEXT_CHARS_PER_PAGE = 40  # below this avg => likely scanned/image-only

def dump(pdf: pathlib.Path) -> dict:
   out = pdf.with_suffix(pdf.suffix + ".txt")  # foo.pdf -> foo.pdf.txt
   t0 = time.time()
   doc = fitz.open(pdf)
   parts = []
   for i, page in enumerate(doc, 1):
      parts.append(f"\n\n──────── page {i}/{doc.page_count} ────────\n\n")
      parts.append(page.get_text("text"))
   text = "".join(parts)
   out.write_text(text, encoding="utf-8")
   chars = len(text)
   return {
      "pdf": str(pdf.relative_to(REPO)),
      "pages": doc.page_count,
      "chars": chars,
      "kb": out.stat().st_size // 1024,
      "secs": round(time.time() - t0, 2),
      "low_text": doc.page_count > 0 and chars / max(doc.page_count, 1) < LOW_TEXT_CHARS_PER_PAGE,
   }

pdfs = sorted(p for p in REPO.rglob("*.pdf") if ".pdf-extract" not in p.parts)
print(f"Found {len(pdfs)} PDF(s) under {REPO}\n")
rows, failures = [], []
for pdf in pdfs:
   try:
      r = dump(pdf)
      rows.append(r)
      flag = "  ⚠ LOW-TEXT (likely scanned)" if r["low_text"] else ""
      print(f"  ok  {r['pages']:>4}p  {r['chars']:>8}ch  {r['kb']:>5}KB  {r['secs']:>5}s  {r['pdf']}{flag}")
   except Exception as e:
      failures.append((str(pdf.relative_to(REPO)), repr(e)))
      print(f"  FAIL  {pdf.relative_to(REPO)}  -> {e!r}")

print(f"\nDone: {len(rows)} ok, {len(failures)} failed.")
if any(r["low_text"] for r in rows):
   print("Low-text (image-only?) PDFs — would need OCR for real text:")
   for r in rows:
      if r["low_text"]:
         print(f"  - {r['pdf']}  ({r['chars']} chars / {r['pages']} pages)")
if failures:
   print("Failures:")
   for name, err in failures:
      print(f"  - {name}: {err}")
