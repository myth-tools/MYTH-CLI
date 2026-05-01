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
MYTH_VERSION=$(get_yaml_val "version")
AUTHOR=$(get_yaml_val "author")
REPO_URL=$(get_yaml_val "repository_url")
PAGES_URL=$(get_yaml_val "pages_url")

# ─── 2. Tactical Version Parity Guard ───
# Industry Grade Extraction: Targets the top-level version only
get_cargo_version() {
    grep "^version[[:space:]]*=" "$CARGO_TOML" | head -n 1 | sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p'
}

CARGO_VERSION=$(get_cargo_version)

if [[ "$MYTH_VERSION" != "$CARGO_VERSION" ]]; then
    echo -e "${RED}✘ [VERSION DRIFT DETECTED]${RESET}"
    echo -e "   agent.yaml: $MYTH_VERSION"
    echo -e "   Cargo.toml: $CARGO_VERSION"
    echo -e "${YELLOW}Aborting mission. Synchronize versions manually before distributing.${RESET}"
    exit 1
fi


# Guard against empty critical fields
if [[ -z "$MYTH_VERSION" || -z "$NAME" || -z "$CARGO_VERSION" ]]; then
    echo -e "${RED}✘ [FATAL] One or more critical metadata fields (name, version) are empty.${RESET}"
    echo -e "        Check $AGENT_YAML for correct formatting."
    exit 1
fi

if [[ -f "Cargo.lock" ]]; then
    if ! grep -A 2 "^name = \"myth\"$" Cargo.lock | grep -q "^version = \"$MYTH_VERSION\"$"; then
        echo -e "${YELLOW}⚠ [WARN] Cargo.lock version drift detected. Please run 'cargo update -p myth' or build to sync.${RESET}"
    else
        echo -e "${GREEN}✔ [OK] Cargo.lock version parity confirmed.${RESET}"
    fi
fi

echo -e "${CYAN} [SYNC] Synchronizing metadata for $NAME v$MYTH_VERSION...${RESET}"

# ─── 3. Documentation Sync (React Meta) ───
if [[ -f "$METADATA_TS" ]]; then
    # Atomic write: write to temp then rename to prevent corruption on interrupt
    METADATA_TMP="${METADATA_TS}.tmp"
    cat > "$METADATA_TMP" << EOF
export const MYTH_NAME = "$NAME";
export const MYTH_VERSION = "$MYTH_VERSION";
export const AUTHOR = "$AUTHOR";
export const REPOSITORY_URL = "$REPO_URL";
export const PAGES_URL = "$PAGES_URL";
EOF

    mv -f "$METADATA_TMP" "$METADATA_TS"
    echo -e "${GREEN}✔ [OK] Web Nexus metadata synchronized.${RESET}"

    # ─── Standardize Node.js Manifests (Industry Grade Sync) ───
    for PKG_JSON in "docs/package.json" "package_runners/package.json"; do
        if [ -f "$PKG_JSON" ]; then
            # Atomic sed update: target the top-level "version" key only
            sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$MYTH_VERSION\"/" "$PKG_JSON"
            echo -e "${GREEN}✔ [OK] Synchronized version in $(basename "$PKG_JSON").${RESET}"
        fi
    done

    # ─── Standardize Installer Scripts (Core Distribution Sync) ───
    for INSTALLER in "scripts/bootstrap.sh" "scripts/uninstall.sh"; do
        if [ -f "$INSTALLER" ]; then
            # Atomic sed update: target the MYTH_VERSION="..." definition
            sed -i "s/MYTH_VERSION=\"[^\"]*\"/MYTH_VERSION=\"$MYTH_VERSION\"/" "$INSTALLER"
            echo -e "${GREEN}✔ [OK] Synchronized version in $(basename "$INSTALLER").${RESET}"
        fi
    done

    # ─── Standardize Config & Web Metadata (Industry Grade Parity) ───
    [ -f "config/agent.yaml" ] && sed -i "s/version: \"[^\"]*\"/version: \"$MYTH_VERSION\"/" "config/agent.yaml"
    [ -f "docs/index.html" ] && sed -i "s/\"softwareVersion\": \"[^\"]*\"/\"softwareVersion\": \"$MYTH_VERSION\"/" "docs/index.html"

    [ -f "README.md" ] && sed -i "s/MYTH_VERSION=[0-9][^ ]*/MYTH_VERSION=$MYTH_VERSION/g" "README.md"
    
    # Sync React Configuration Components
    for REACT_COMP in "docs/src/components/ConfigBuilder.tsx" "docs/src/pages/ConfigurationPage.tsx" "docs/src/pages/InstallationPage.tsx"; do
        if [ -f "$REACT_COMP" ]; then
            sed -i "s/VERSION=[0-9][^ ]*/VERSION=$MYTH_VERSION/g" "$REACT_COMP"
            sed -i "s/version: \"[^\"]*\"/version: \"$MYTH_VERSION\"/" "$REACT_COMP"
            echo -e "${GREEN}✔ [OK] Synchronized version in $(basename "$REACT_COMP").${RESET}"
        fi
    done

    # Sync decorative tool banners
    if [ -f "src/builtin_tools/recon/subdomain_fetch.rs" ]; then
        sed -i "s/v[0-9][^ ]*-QUANTUM/v$MYTH_VERSION-QUANTUM/g" "src/builtin_tools/recon/subdomain_fetch.rs"
        echo -e "${GREEN}✔ [OK] Synchronized banner in subdomain_fetch.rs.${RESET}"
    fi
fi






# ─── 4. Distributed Ecosystems Sync ───
if [ -d "package_runners" ]; then
    # Surgical Regex Injection: Anchored to avoid false matches
    
    # package.json (NPM) - Handled in the standardized loop above

    
    # pyproject.toml (PyPI) - Supports PEP 440 Suffix for re-uploads
    if [[ -f "package_runners/pyproject.toml" ]]; then
        PYPI_VERSION="${MYTH_VERSION}${PYPI_VERSION_SUFFIX:-}"
        if [[ -n "${PYPI_VERSION_SUFFIX:-}" ]]; then
            echo -e "${YELLOW}ℹ [PYPI] Suffix detected: $PYPI_VERSION_SUFFIX (Final: $PYPI_VERSION)${RESET}"
        fi
        sed -i "s/^[[:space:]]*version[[:space:]]*=[[:space:]]*\"[^\"]*\"/version = \"$PYPI_VERSION\"/" package_runners/pyproject.toml
        echo -e "${GREEN}✔ [OK] PyPI Manifest synchronized (Version: $PYPI_VERSION).${RESET}"
    fi
    
    # snapcraft.yaml (Canonical)
    if [[ -f "package_runners/snapcraft.yaml" ]]; then
        # Match version: value, version: 'value', or version: "value"
        sed -i "s/^[[:space:]]*version:[[:space:]]*.*$/version: '$MYTH_VERSION'/" package_runners/snapcraft.yaml
        echo -e "${GREEN}✔ [OK] Snapcraft Manifest synchronized.${RESET}"
    fi

    # Dockerfile (OCI)
    if [[ -f "package_runners/Dockerfile" ]]; then
        sed -i "s/^[[:space:]]*LABEL org\.opencontainers\.image\.version=.*$/LABEL org.opencontainers.image.version=\"$MYTH_VERSION\"/" package_runners/Dockerfile
        echo -e "${GREEN}✔ [OK] Dockerfile metadata synchronized.${RESET}"
    fi
fi

echo -e "${CYAN}⚡ [MISSION COMPLETE] All distribution vectors are locked and loaded.${RESET}"
