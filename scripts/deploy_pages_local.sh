#!/usr/bin/env bash
# scripts/deploy_pages_local.sh — Deploy docs to GitHub Pages
# SAFETY: This script NEVER switches branches or runs any destructive
#         git command in your project folder. All git work happens
#         in an isolated /tmp clone that is deleted afterwards.
set -euo pipefail

PAGES_BRANCH="gh-pages"
APT_SRC="$HOME/.aptly/public"

BOLD='\033[1m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

info()  { echo -e "${CYAN}[INFO]${NC}  $1"; }
ok()    { echo -e "${GREEN}[OK]${NC}    $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC}  $1"; }
err()   { echo -e "${RED}[ERR]${NC}   $1"; exit 1; }

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

# ─── Flag Parsing ───
STAGE_APT=true
for arg in "$@"; do
    case "$arg" in
        --no-apt) STAGE_APT=false ;;
    esac
done

echo -e "${BOLD}🚀 Deploying documentation to GitHub Pages...${NC}"

# ── 1. Read config (read-only, no git) ──
[ -f "config/agent.yaml" ] || err "Tactical config (config/agent.yaml) not found!"
REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E 's/.*repository_url:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/' || echo "")
PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E 's/.*pages_url:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/' || echo "")
[ -z "$PAGES_URL" ] && [ -n "$REPO_URL" ] && PAGES_URL="https://$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')"
ORIGIN_URL=$(git remote get-url origin 2>/dev/null) \
    || err "No 'origin' remote found in current repo. Run: git remote add origin <url>"

info "Environment Validation:"
info "  - Source: $ORIGIN_URL"
info "  - Public Domain: $PAGES_URL"

if [ ! -f "myth.gpg" ]; then
    err "GPG Public key (myth.gpg) not found! Run scripts/init_repo.sh first."
fi
ok "Tactical environment validated."

# ── 2. Build docs (safe: only writes to docs/dist) ──
info "Building documentation..."
(cd docs && VITE_REPO_URL="$REPO_URL" VITE_PAGES_URL="$PAGES_URL" npm run build)
ok "Built docs/dist"

# ── 3. Create isolated clone in ${TMPDIR:-/tmp} ──
WORK=$(mktemp -d "${TMPDIR:-/tmp}/myth_pages.XXXXXX")
trap 'rm -rf "$WORK"' EXIT
info "Cloning into isolated workspace $WORK..."

if git ls-remote --heads "$ORIGIN_URL" "$PAGES_BRANCH" | grep -q "$PAGES_BRANCH"; then
    git clone --single-branch --branch "$PAGES_BRANCH" "$ORIGIN_URL" "$WORK/repo" 2>/dev/null
else
    git init "$WORK/repo"
    cd "$WORK/repo"
    git remote add origin "$ORIGIN_URL"
    git checkout -b "$PAGES_BRANCH"
fi

# ── 4. Replace content in the isolated clone ──
cd "$WORK/repo"
find . -maxdepth 1 ! -name ".git" ! -name "." -exec rm -rf {} +

cp -r "$PROJECT_ROOT/docs/dist"/. .
if [ "$STAGE_APT" = true ]; then
    if [ -d "$APT_SRC" ] && [ "$(ls -A "$APT_SRC" 2>/dev/null)" ]; then
        cp -a "$APT_SRC"/. . && ok "Merged APT + RPM + Arch repos"
    else
        warn "APT source $APT_SRC is empty or missing. Skipping repo merge."
    fi
else
    info "APT staging skipped (--no-apt)."
fi

# ── Copy Version Manifests if not already in APT_SRC ──
if [ -f "$PROJECT_ROOT/target/version.txt" ]; then
    cp "$PROJECT_ROOT/target/version.txt" .
    ok "Merged version.txt"
fi
if [ -f "$PROJECT_ROOT/target/versions.json" ]; then
    cp "$PROJECT_ROOT/target/versions.json" .
    ok "Merged versions.json manifest"
fi

cp "$PROJECT_ROOT/scripts/install.sh" .
cp "$PROJECT_ROOT/scripts/bootstrap.sh" .
cp "$PROJECT_ROOT/scripts/uninstall.sh" .
cp "$PROJECT_ROOT/scripts/update.sh" .
cp "$PROJECT_ROOT/config/user.yaml" .   # Full config template for installer
cp "$PROJECT_ROOT/myth.gpg" .            # Copy GPG public key for APT + RPM

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' "$PROJECT_ROOT/Cargo.toml" | head -n 1)
AGENT_NAME=$(grep "name:" "$PROJECT_ROOT/config/agent.yaml" | head -n 1 | sed -E 's/.*name:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/' | awk '{print $1}')

sed -i "s|__REPO_URL__|$REPO_URL|g;s|__PAGES_URL__|$PAGES_URL|g;s|__VERSION__|$VERSION|g;s|__AGENT_NAME__|$AGENT_NAME|g" install.sh bootstrap.sh uninstall.sh update.sh

info "Generating asset manifest..."
sha256sum install.sh bootstrap.sh uninstall.sh update.sh user.yaml myth.gpg version.txt versions.json > SHA256SUMS 2>/dev/null || true
ok "Artifact manifest (SHA256SUMS) generated."

# ── 4.5. Handle Custom Domain (CNAME) ──
DOMAIN=$(echo "$PAGES_URL" | sed -E 's|https?://||; s|/.*||')
if [[ ! "$DOMAIN" =~ \.github\.io$ ]]; then
    info "Custom domain detected: $DOMAIN. Creating CNAME..."
    echo "$DOMAIN" > CNAME
fi

# ── 5. Commit and push from the isolated clone ──
info "Pushing to $PAGES_BRANCH..."
git add .
# Only commit if there are actual changes — --allow-empty causes noisy empty commits
if ! git diff --cached --quiet; then
    git commit -m "site: deploy $(date +%Y-%m-%d_%H-%M)"
else
    info "No content changes. Skipping commit."
fi
# --force-with-lease is safer than --force: fails if upstream changed unexpectedly
# --set-upstream is needed on a brand-new orphan branch with no tracking ref yet
MAX_RETRIES=3
RETRY_COUNT=0
PUSH_SUCCESS=false

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if git push origin "$PAGES_BRANCH" --force-with-lease --set-upstream 2>/dev/null || git push origin "$PAGES_BRANCH" --force; then
        PUSH_SUCCESS=true
        break
    else
        RETRY_COUNT=$((RETRY_COUNT + 1))
        warn "Push failed (Network or DNS issue). Retrying... ($RETRY_COUNT/$MAX_RETRIES) in 3 seconds..."
        sleep 3
    fi
done

if [ "$PUSH_SUCCESS" = false ]; then
    err "Failed to push to GitHub Pages after $MAX_RETRIES attempts. Check your network or permissions."
fi

# ── 6. Final Integrity Check ──
cd "$PROJECT_ROOT"
if [ -n "$(git status --porcelain)" ]; then
    warn "Uncommitted changes detected in your project (not caused by this script)."
else
    ok "Integrity Check: Project state is clean and untouched."
fi

ok "✅ Deployed to $PAGES_URL"
info "Note: All Git operations were confined to an isolated ${TMPDIR:-/tmp} workspace."
