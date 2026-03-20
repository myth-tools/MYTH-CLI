#!/usr/bin/env bash
# scripts/deploy_pages_local.sh — Deploy docs to GitHub Pages
# SAFETY: This script NEVER switches branches or runs any destructive
#         git command in your project folder. All git work happens
#         in an isolated /tmp clone that is deleted afterwards.
set -e

PAGES_BRANCH="gh-pages"
APT_SRC="$HOME/.aptly/public"

BOLD='\033[1m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
RED='\033[0;31m'
NC='\033[0m'

info()  { echo -e "${CYAN}[INFO]${NC}  $1"; }
ok()    { echo -e "${GREEN}[OK]${NC}    $1"; }
err()   { echo -e "${RED}[ERR]${NC}   $1"; exit 1; }

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${BOLD}🚀 Deploying documentation to GitHub Pages...${NC}"

# ── 1. Read config (read-only, no git) ──
[ -f "config/agent.yaml" ] || err "Tactical config (config/agent.yaml) not found!"
REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E 's/.*repository_url:[[:space:]]*["'\''"]?([^"'\'']+)["'\''"]?.*/\1/')
PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E 's/.*pages_url:[[:space:]]*["'\''"]?([^"'\'']+)["'\''"]?.*/\1/' || echo "")
[ -z "$PAGES_URL" ] && PAGES_URL="https://$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')"
ORIGIN_URL=$(git remote get-url origin)

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

# ── 3. Create isolated clone in /tmp ──
WORK=$(mktemp -d /tmp/myth_pages.XXXXXX)
trap "rm -rf '$WORK'" EXIT
info "Cloning into isolated workspace $WORK..."

if git ls-remote --heads "$ORIGIN_URL" "$PAGES_BRANCH" | grep -q "$PAGES_BRANCH"; then
    git clone --single-branch --branch "$PAGES_BRANCH" "$ORIGIN_URL" "$WORK/repo" 2>/dev/null
else
    git init "$WORK/repo"
    cd "$WORK/repo"
    git remote add origin "$ORIGIN_URL"
    git checkout --orphan "$PAGES_BRANCH"
fi

# ── 4. Replace content in the isolated clone ──
cd "$WORK/repo"
find . -maxdepth 1 ! -name ".git" ! -name "." -exec rm -rf {} +

cp -r "$PROJECT_ROOT/docs/dist"/. .
[ -d "$APT_SRC" ] && cp -a "$APT_SRC"/. . && ok "Merged APT repo"

cp "$PROJECT_ROOT/scripts/install.sh" .
cp "$PROJECT_ROOT/scripts/bootstrap.sh" .
cp "$PROJECT_ROOT/myth.gpg" .  # Copy GPG public key for APT
sed -i "s|__REPO_URL__|$REPO_URL|g;s|__PAGES_URL__|$PAGES_URL|g" install.sh bootstrap.sh

# ── 4.5. Handle Custom Domain (CNAME) ──
DOMAIN=$(echo "$PAGES_URL" | sed -E 's|https?://||; s|/.*||')
if [[ ! "$DOMAIN" =~ \.github\.io$ ]]; then
    info "Custom domain detected: $DOMAIN. Creating CNAME..."
    echo "$DOMAIN" > CNAME
fi

# ── 5. Commit and push from the isolated clone ──
info "Pushing to $PAGES_BRANCH..."
git add .
git commit -m "site: deploy $(date +%Y-%m-%d_%H-%M)" --allow-empty
git push origin "$PAGES_BRANCH" --force

# ── 6. Final Integrity Check ──
cd "$PROJECT_ROOT"
if [ -n "$(git status --porcelain)" ]; then
    warn "Uncommitted changes detected in your project (not caused by this script)."
else
    ok "Integrity Check: Project state is clean and untouched."
fi

ok "✅ Deployed to $PAGES_URL"
info "Note: All Git operations were confined to an isolated /tmp workspace."
