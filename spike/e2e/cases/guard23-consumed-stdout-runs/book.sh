# guard23-consumed-stdout-runs (guards never serve consumed-output positions — a PASSING
# floor). The install's stdout feeds a pipe: a guard's pass-direction would replace the
# tool's output with the check's (silenced or worse, different) bytes — corrupting what
# the consumer reads. plans/233 lists consumed-stdout/command-substitution positions first
# among the sites guards can't serve; the ratified refuse-home posture (2026-07-02) makes
# the eventual refusal loud. Converged + vouched (maximal bait): still RUNS, whole line.
apt-get install -y nginx | tee /var/log/install.log
