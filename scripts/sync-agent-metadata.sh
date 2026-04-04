#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Professional Metadata Synchronization System
#  ─────────────────────────────────────────────────────────
#  Syncs name, version, and urls from config/agent.yaml
#  to the docs, package.json, pyproject.toml, and snap.
#  Enforces absolute Cargo.toml version parity.
# ═══════════════════════════════════════════════════════════
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# ANSI Colors
RED='\033[1;31m'
GREEN='\033[1;32m'
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
RESET='\033[0m'

# Paths
AGENT_YAML="config/agent.yaml"
CARGO_TOML="Cargo.toml"
METADATA_TS="docs/src/data/metadata.ts"

# ─── 1. Pre-flight Existence Audit ───
for file in "$AGENT_YAML" "$CARGO_TOML"; do
    if [[ ! -f "$file" ]]; then
        echo -e "${RED}✘ [FATAL] Critical manifest missing: $file${RESET}"
        exit 1
    fi
done

# Extract fields using strict grep (No yq dependency for Kali compatibility)
get_yaml_val() {
    # Handles: key: "value", key: 'value', key: value (unquoted), trims trailing whitespace/comments
    grep -E "^[[:space:]]+$1:" "$AGENT_YAML" | head -n1 \
        | sed -E "s|^[[:space:]]+$1:[[:space:]]*[\"']?||" \
        | sed -E "s|[\"'][[:space:]]*(#.*)?$||" \
        | sed 's/[[:space:]]*$//'
}

NAME=$(get_yaml_val "name")
VERSION=$(get_yaml_val "version")
AUTHOR=$(get_yaml_val "author")
REPO_URL=$(get_yaml_val "repository_url")
PAGES_URL=$(get_yaml_val "pages_url")

# ─── 2. Tactical Version Parity Guard ───
# Ensure Cargo.toml and agent.yaml are in perfect mathematical alignment
CARGO_VERSION=$(grep "^version[[:space:]]*=[[:space:]]*\"" "$CARGO_TOML" | head -n1 | cut -d'"' -f2)

if [[ "$VERSION" != "$CARGO_VERSION" ]]; then
    echo -e "${RED}✘ [VERSION DRIFT DETECTED]${RESET}"
    echo -e "   agent.yaml: $VERSION"
    echo -e "   Cargo.toml: $CARGO_VERSION"
    echo -e "${YELLOW}Aborting mission. Synchronize versions manually before distributing.${RESET}"
    exit 1
fi

# Guard against empty critical fields
if [[ -z "$VERSION" || -z "$NAME" || -z "$CARGO_VERSION" ]]; then
    echo -e "${RED}✘ [FATAL] One or more critical metadata fields (name, version) are empty.${RESET}"
    echo -e "        Check $AGENT_YAML for correct formatting."
    exit 1
fi

if [[ -f "Cargo.lock" ]]; then
    if ! grep -A 2 "^name = \"myth\"$" Cargo.lock | grep -q "^version = \"$VERSION\"$"; then
        echo -e "${YELLOW}⚠ [WARN] Cargo.lock version drift detected. Please run 'cargo update -p myth' or build to sync.${RESET}"
    else
        echo -e "${GREEN}✔ [OK] Cargo.lock version parity confirmed.${RESET}"
    fi
fi

echo -e "${CYAN} [SYNC] Synchronizing metadata for $NAME v$VERSION...${RESET}"

# ─── 3. Documentation Sync (React Meta) ───
if [[ -f "$METADATA_TS" ]]; then
    # Atomic write: write to temp then rename to prevent corruption on interrupt
    METADATA_TMP="${METADATA_TS}.tmp"
    cat > "$METADATA_TMP" << EOF
export const NAME = "$NAME";
export const VERSION = "$VERSION";
export const AUTHOR = "$AUTHOR";
export const REPOSITORY_URL = "$REPO_URL";
export const PAGES_URL = "$PAGES_URL";
EOF
    mv -f "$METADATA_TMP" "$METADATA_TS"
    echo -e "${GREEN}✔ [OK] Web Nexus metadata synchronized.${RESET}"
fi

# ─── 4. Distributed Ecosystems Sync ───
if [ -d "package_runners" ]; then
    # Surgical Regex Injection: Anchored to avoid false matches
    
    # package.json (NPM)
    if [[ -f "package_runners/package.json" ]]; then
        # Anchored to the top-level 2-space-indented "version" key only
        sed -i "s/^  \"version\": \"[^\"]*\"/  \"version\": \"${VERSION}\"/" package_runners/package.json
        echo -e "${GREEN}✔ [OK] NPM Manifest synchronized.${RESET}"
    fi
    
    # pyproject.toml (PyPI) - Supports PEP 440 Suffix for re-uploads
    if [[ -f "package_runners/pyproject.toml" ]]; then
        PYPI_VERSION="${VERSION}${PYPI_VERSION_SUFFIX:-}"
        if [[ -n "${PYPI_VERSION_SUFFIX:-}" ]]; then
            echo -e "${YELLOW}ℹ [PYPI] Suffix detected: $PYPI_VERSION_SUFFIX (Final: $PYPI_VERSION)${RESET}"
        fi
        sed -i "s/^[[:space:]]*version[[:space:]]*=[[:space:]]*\"[^\"]*\"/version = \"$PYPI_VERSION\"/" package_runners/pyproject.toml
        echo -e "${GREEN}✔ [OK] PyPI Manifest synchronized (Version: $PYPI_VERSION).${RESET}"
    fi
    
    # snapcraft.yaml (Canonical)
    if [[ -f "package_runners/snapcraft.yaml" ]]; then
        # Match version: value, version: 'value', or version: "value"
        sed -i "s/^[[:space:]]*version:[[:space:]]*.*$/version: '$VERSION'/" package_runners/snapcraft.yaml
        echo -e "${GREEN}✔ [OK] Snapcraft Manifest synchronized.${RESET}"
    fi

    # Dockerfile (OCI)
    if [[ -f "package_runners/Dockerfile" ]]; then
        sed -i "s/^[[:space:]]*LABEL org\.opencontainers\.image\.version=.*$/LABEL org.opencontainers.image.version=\"$VERSION\"/" package_runners/Dockerfile
        echo -e "${GREEN}✔ [OK] Dockerfile metadata synchronized.${RESET}"
    fi
fi

echo -e "${CYAN}⚡ [MISSION COMPLETE] All distribution vectors are locked and loaded.${RESET}"
