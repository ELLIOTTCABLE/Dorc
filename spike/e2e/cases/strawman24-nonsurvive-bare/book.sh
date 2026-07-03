# strawman24-nonsurvive-bare (plans/240 Stage 2): a running modeled wall WITHOUT a touches()
# footprint ⇒ a TOTAL wall even under --trust-footprints (silence=wall is flag-independent).
#   site 0  apt-get install oldpkg — DIVERGED wall; the oracle has predict() but NO touches().
#   site 1  apt-get install nginx  — CONVERGED, but the wall declares no footprint ⇒ total wall ⇒
#           DEMOTES (runs) even flagged. The flag only buys back walls that name what they touch;
#           an un-footprinted mutator is exactly the honest Stage-1 baseline.
apt-get install -y oldpkg
apt-get install -y nginx
