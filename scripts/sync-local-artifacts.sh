#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="${1:-0.0.4}"
target_dir="$repo_root/compose/registry/artifacts/re-indicators-specification/$version"
base_url="https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v${version}/generated"

mkdir -p "$target_dir"

for artifact in schema.json shacl.ttl model.ttl calculation.json; do
  echo "Downloading $artifact for RE indicators version $version"
  curl -fsSL "$base_url/$artifact" -o "$target_dir/$artifact"
done

echo "Artifacts synced to $target_dir"
