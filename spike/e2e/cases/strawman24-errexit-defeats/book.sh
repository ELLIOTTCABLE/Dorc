# strawman24-errexit-defeats (plans/240 Stage-1 yardstick — a KNOWN cost, newly MEASURED, NOT a
# defect pin). Byte-identical to strawman24-all-converged-clean PLUS `set -e` at the top. Every
# site is CONVERGED and would elide (elide-fr 1.00 there); but under errexit each mutator's exit
# STATUS is consumed (the errexit region reads every command's rc), and a mutator's rc is ⊤
# (fork-mutator-rc: only its Effect/convergence arrives from the probe, never a fabricated 0). A
# consumed ⊤ status blocks the elide (inv-probe-sourced-values / consumption-coverage), so the
# converged mutators RUN — elide-fr collapses toward 0.00. This is the `206 §2` headline cost
# (the 20V doors are its named recovery program — door-3 landed; the rest is the golden-hill
# program), here QUANTIFIED as a family row so the strawman axis carries book-IDIOM quality
# (does the book open `set -eu`?) alongside oracle-coverage quality. The USER_STORY flagship book
# opens `set -eu`, so its stage-1 render depends on this program maturing.
set -e
apt-get install -y nginx
systemctl enable nginx
apt-get install -y curl
