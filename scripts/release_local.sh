#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — Local CI Release Script
# ═══════════════════════════════════════════════════
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# ─── Visual Branding (Ultra-Premium Cyber Style) ───
BANNER="
  ███╗   ███╗██╗   ██╗████████╗██╗  ██╗
  ████╗ ████║╚██╗ ██╔╝╚══██╔══╝██║  ██║
  ██╔████╔██║ ╚████╔╝    ██║   ███████║
  ██║╚██╔╝██║  ╚██╔╝     ██║   ██╔══██║
  ██║ ╚═╝ ██║   ██║      ██║   ██║  ██║
  ╚═╝     ╚═╝   ╚═╝      ╚═╝   ╚═╝  ╚═╝
"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -en "${YELLOW}⚠  [WARN] ${NC} $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ LOCAL TACTICAL RELEASE UTILITY ]${NC}"
echo -e "  ${BOLD}Initiating release sequence...${NC}\n"
require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}

echo -e "${BOLD}Starting Local CI Release Flow for MYTH...${NC}"

# 0. Initialize repo if needed
if ! aptly repo show myth-repo &>/dev/null; then
    info "Aptly repository 'myth-repo' not found. Initializing..."
    bash scripts/init_repo.sh
fi

# ─── Dynamic Repository Configuration ───
if [ ! -f "config/agent.yaml" ]; then
    err "config/agent.yaml not found!"
fi
REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E 's/.*repository_url:[[:space:]]*["'\'']?([^"'\'']+)["'\'']?.*/\1/')
PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E 's/.*pages_url:[[:space:]]*["'\'']?([^"'\'']+)["'\'']?.*/\1/' || echo "")

if [ -z "$PAGES_URL" ]; then
    CLEAN_REPO_URL=$(echo "$REPO_URL" | sed -E 's|/*$||' | sed -E 's|\.git$||')
    PAGES_DOMAIN=$(echo "$CLEAN_REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')
    PAGES_URL="https://$PAGES_DOMAIN"
fi

info "Repository: $REPO_URL"
info "Pages/Repo: $PAGES_URL"

# 1. Build Release
require_command cargo
require_command gpg
require_command aptly

info "Building release binary..."
cargo build --release
ok "Binary built."

# 2. Generate .deb
info "Generating Debian package..."
if ! cargo deb --version &>/dev/null; then
    cargo install cargo-deb
fi
cargo deb --no-build
DEB_FILE=$(ls -t target/debian/myth_*.deb | head -1)
ok "Package generated: $DEB_FILE"

# 3. Add to aptly
info "Adding package to aptly repo..."
aptly repo add -force-replace myth-repo "$DEB_FILE"
ok "Added to myth-repo."

# 4. Finalize publication
info "Finalizing publication..."
KEY_ID=$(gpg --list-secret-keys --keyid-format=SHORT | grep "sec" | awk '{print $2}' | cut -d'/' -f2 | head -1)
if [ -z "$KEY_ID" ]; then
    err "Could not determine GPG Key ID."
fi

# Detect architectures
ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")

if aptly publish list | grep -q "\./stable"; then
    info "Updating existing publication..."
    aptly publish update -gpg-key="$KEY_ID" -passphrase="${GPG_PASSPHRASE:-}" -force-overwrite stable
else
    info "New publication: establishing structure..."
    aptly publish repo -architectures="$ARCH,all" -gpg-key="$KEY_ID" -passphrase="${GPG_PASSPHRASE:-}" -distribution=stable myth-repo
fi
ok "Repository is now live at ~/.aptly/public/"

echo ""
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
ok "🚀 Version ${VERSION} is now added to your local aptly repository."

# 5. Git Tagging for Visibility
echo -e "${BOLD}Do you want to tag this version as v${VERSION} and push to GitHub? [y/N]${NC}"
read -r -p "Run: git tag v${VERSION} && git push origin v${VERSION}? " response
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    info "Tagging and pushing..."
    git tag "v${VERSION}" || warn "Tag v${VERSION} already exists."
    git push origin "v${VERSION}" || err "Failed to push tag to GitHub."
    ok "Tag v${VERSION} is now live on GitHub Releases!"
else
    info "Skipping Git Tagging. Remember to tag manually for GitHub visibility."
fi

echo ""
echo -e "Next step: ${BOLD}bash scripts/deploy_pages_local.sh${NC} to push everything to GitHub."
