#!/usr/bin/env python3
import pathlib
REPO = pathlib.Path("/mnt/c/Users/ec/Sync/Code/Dorc")
DIRS = [REPO/"Research/papers", REPO/"Research/learning-path",
        REPO/"Vendor/SVF/docs/images", REPO/"Vendor/infer/website/static/downloads"]
U = chr(0xFFFD)

pdftxt = sum(len(list(d.glob("*.pdf.txt"))) for d in DIRS)
txt = sum(len(list(d.glob("*.txt"))) for d in DIRS)
print(f"leftover .pdf.txt: {pdftxt}   total .txt: {txt}\n")

print("mojibake recheck on previously-butchered files (new .txt):")
for stem in ["heintze-tardieu-demand-pointer-analysis-pldi2001",
             "reps-cfl-reachability-survey-1998",
             "reps-horwitz-sagiv-ifds-popl1995",
             "moller-schwartzbach-static-program-analysis"]:
   p = REPO/"Research/papers"/f"{stem}.txt"
   t = p.read_text(encoding="utf-8")
   syms = sum(t.count(c) for c in "⊑⊔⊓∈⊆→α")
   print(f"  {stem[:48]:48}  U+FFFD={t.count(U):>4}  math-syms(⊑⊔⊓∈⊆→α)={syms}")

for stem in ["tip-program-slicing-survey-1995", "cousot-abstract-interpretation-popl1977"]:
   p = REPO/"Research/papers"/f"{stem}.txt"
   print(f"\n--- head of {stem}.txt ---")
   print(p.read_text(encoding="utf-8")[:360])
