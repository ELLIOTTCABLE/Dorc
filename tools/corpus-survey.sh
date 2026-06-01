#!/usr/bin/env bash
# corpus-survey.sh — enumerate candidate corpus repos for Dorc (Space 1).
#
# Status: RESEARCH SPIKE. Dry-run by default — safe to run, makes only the one
# free rate_limit call unless you pass RUN=1. It CLONES NOTHING; it emits a
# candidate manifest (TSV) for you to review and selectively fetch later.
#
# Why this shape: `gh` is authenticated (5000 req/hr core, 30 req/min search);
# system `jq` is ABSENT so we use gh's built-in `--jq`. We stay far under limits
# (small --limit caps, a preflight budget check, sleeps between search calls).
#
#   ./tools/corpus-survey.sh            # dry run: prints planned queries + budget
#   RUN=1 ./tools/corpus-survey.sh      # actually sample (small, rate-limit-aware)
#   RUN=1 PER=40 ./tools/corpus-survey.sh   # widen the per-query cap (watch limits)
set -euo pipefail

OUT="Research/corpus-candidates"
PER="${PER:-20}"            # repos per query (keep small; search caps at 30/min)
RUN="${RUN:-0}"            # 0 = dry run (default), 1 = execute searches
SLEEP="${SLEEP:-3}"        # seconds between search calls (stay under 30/min)
mkdir -p "$OUT"

need(){ command -v "$1" >/dev/null || { echo "MISSING: $1" >&2; exit 1; }; }
need gh; need curl

# --- preflight: auth + remaining search budget (the one call we always make) ---
echo "== gh auth & rate budget =="
gh auth status >/dev/null 2>&1 || { echo "gh not authenticated"; exit 1; }
SEARCH_LEFT=$(gh api rate_limit --jq '.resources.search.remaining' 2>/dev/null || echo "?")
CORE_LEFT=$(gh api rate_limit --jq '.resources.core.remaining' 2>/dev/null || echo "?")
echo "search budget remaining: $SEARCH_LEFT   core budget remaining: $CORE_LEFT"
if [ "$RUN" = "1" ] && [ "$SEARCH_LEFT" != "?" ] && [ "$SEARCH_LEFT" -lt 10 ] 2>/dev/null; then
   echo "search budget too low (<10); aborting to respect rate limits." >&2; exit 1
fi

# --- 1A: ops corpuses — homelab/gitops trees (by topic) ---------------------
# Whole-org-ish IaC trees mixing Ansible+Terraform+k8s; the user's demographic.
OPS_TOPICS="k8s-at-home homelab home-ops gitops"

# --- 1A: "fuzzy edge" code signatures — imperative work shoehorned into
#         declarative tools (the Dorc-niche detector). Code search is tighter;
#         these are templates for `gh search code` — uncomment to use, sparingly.
FUZZY_SIGNATURES=(
   'provisioner "local-exec"'
   'provisioner "remote-exec"'
   'resource "null_resource"'
   'resource "terraform_data"'
   'ansible.builtin.shell'
   'ansible.builtin.raw'
)

# --- 1B: provider corpuses — Ansible Galaxy top collections (Galaxy API, not gh)
GALAXY_API="https://galaxy.ansible.com/api/v3/plugin/ansible/content/published/collections/index/?order_by=-download_count&limit=$PER"

run_repo_query(){ # $1=label  $2..=gh search repos args
   local label="$1"; shift
   echo ">> [repos] $label : gh search repos $* --limit $PER"
   if [ "$RUN" = "1" ]; then
      gh search repos "$@" --limit "$PER" \
         --json fullName,stargazersCount,description,license,updatedAt \
         --jq '.[] | [.fullName, (.stargazersCount|tostring), (.license.key // "none"), .updatedAt, (.description // "" | gsub("\t";" "))] | @tsv' \
         | tee -a "$OUT/ops-candidates.tsv"
      sleep "$SLEEP"
   fi
}

echo; echo "== 1A ops corpuses (topic search) =="
: > "$OUT/ops-candidates.tsv" 2>/dev/null || true
for t in $OPS_TOPICS; do run_repo_query "topic:$t" --topic "$t" --sort stars; done

echo; echo "== 1A fuzzy-edge code signatures (templates; run deliberately) =="
for sig in "${FUZZY_SIGNATURES[@]}"; do
   echo ">> [code] would: gh search code '$sig' --limit $PER  (NOTE: code-search budget is separate & tight)"
   # if [ "$RUN" = "1" ]; then gh search code "$sig" --limit "$PER" --json repository --jq '.[].repository.nameWithOwner' | sort -u >> "$OUT/fuzzy-edge-repos.txt"; sleep "$SLEEP"; fi
done

echo; echo "== 1B provider corpuses (Ansible Galaxy top collections) =="
echo ">> GET $GALAXY_API"
if [ "$RUN" = "1" ]; then
   curl -sL "$GALAXY_API" \
      | gh api --jq '.data[]? | [.namespace, .name, (.download_count|tostring)] | @tsv' --input - 2>/dev/null \
      | tee "$OUT/galaxy-top-collections.tsv" || echo "(galaxy parse failed — inspect raw response)"
fi

echo; echo "Done. Dry run = no API search calls made (only the free rate_limit preflight)."
echo "Set RUN=1 to sample. Candidates land in: $OUT/  (nothing is cloned.)"
