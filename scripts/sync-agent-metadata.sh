#!/bin/bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Metadata Synchronization Script
#  ─────────────────────────────────────────────────────────
#  Syncs name, version, and urls from config/agent.yaml
#  to the frontend docs (src/data/metadata.ts).
# ═══════════════════════════════════════════════════════════

set -e

# Paths
AGENT_YAML="config/agent.yaml"
METADATA_TS="docs/src/data/metadata.ts"

# Extract fields using sed (if yq is not available)
get_yaml_val() {
    grep -E "^[[:space:]]+$1:" "$AGENT_YAML" | head -n1 | cut -d'"' -f2
}

NAME=$(get_yaml_val "name")
VERSION=$(get_yaml_val "version")
AUTHOR=$(get_yaml_val "author")
REPO_URL=$(get_yaml_val "repository_url")
PAGES_URL=$(get_yaml_val "pages_url")

echo "Synchronizing metadata for $NAME v$VERSION..."

cat <<EOF > "$METADATA_TS"
export const NAME = "$NAME";
export const VERSION = "$VERSION";
export const AUTHOR = "$AUTHOR";
export const REPOSITORY_URL = "$REPO_URL";
export const PAGES_URL = "$PAGES_URL";
EOF

echo "[OK] Metadata synchronized to docs/src/data/metadata.ts"
