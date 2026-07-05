#!/usr/bin/env bash
# Regenerates PROMETHEUS_HAPP_STAGING_CHECKSUMS.txt from the git-tracked tree only.
# Run from anywhere in the repo. Never hand-edit the checksums file.
#
# This is a LOCAL reproducibility aid, not the authoritative integrity record.
# The signed integrity anchor for a release is the GPG-signed SHA256SUMS in the
# corresponding release (see release/ and Permaculture-DAO/prometheus-canonical-releases).
#
# Verify with:  sha256sum -c PROMETHEUS_HAPP_STAGING_CHECKSUMS.txt
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

out="PROMETHEUS_HAPP_STAGING_CHECKSUMS.txt"
tmp="$(mktemp)"

{
  echo "# PROMETHEUS_HAPP_STAGING_CHECKSUMS.txt"
  echo "# Reproducible SHA-256 of every git-tracked file at commit $(git rev-parse --short HEAD), $(date -u +%Y-%m-%dT%H:%M:%SZ)."
  echo "# Regenerate with scripts/generate_staging_checksums.sh after changing tracked files."
  echo "# Not the signed integrity record; the authoritative anchor is the release SHA256SUMS.asc."
  git ls-files -z -- . ":(exclude)$out" | sort -z | while IFS= read -r -d '' f; do
    sha256sum "./$f"
  done
} > "$tmp"

mv "$tmp" "$out"
echo "Wrote $(grep -c '^[0-9a-f]' "$out") checksummed entries to $out"
