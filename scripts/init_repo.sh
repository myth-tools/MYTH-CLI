#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — APT Repository Initializer
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

require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ APT REPOSITORY INFRASTRUCTURE INITIALIZER ]${NC}"
echo -e "  ${BOLD}Preparing target publishing environment...${NC}\n"

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

# 1. Check dependencies
for cmd in aptly gpg cargo; do
    require_command "$cmd"
done

# 2. Key Authority Verification
section "KEY AUTHORITY VERIFICATION"
info "Auditing GPG secret keystore..."
if ! gpg --list-secret-keys | grep -q "sec"; then
    warn "No valid signing authority detected. Initiating automated key generation..."
    # Automated key generation for ease of use
    gpg --batch --generate-key <<EOF
Key-Type: 1
Key-Length: 2048
Subkey-Type: 1
Subkey-Length: 2048
Name-Real: MYTH Official
Name-Email: release@myth.work.gd
Expire-Date: 0
%no-protection
%commit
EOF
    ok "Tactical signing authority established."
fi

# 3. Aptly Registry Synchronization
section "APTLY REGISTRY SYNCHRONIZATION"
if aptly repo show myth-repo &>/dev/null; then
    ok "Aptly registry 'myth-repo' verified."
else
    info "Constructing aptly registry: myth-repo..."
    aptly repo create -comment="MYTH Official Tactical Repository" -component=main -distribution=stable myth-repo
    ok "Registry initialized."
fi

# 4. Neural Link Publication
section "NEURAL LINK PUBLICATION"
if aptly publish list | grep -q "\./stable"; then
    ok "Publication channel operational. Synchronizing metadata..."
    KEY_ID=$(gpg --list-secret-keys --keyid-format=SHORT | grep "sec" | awk '{print $2}' | cut -d'/' -f2 | head -1)
    aptly publish update -gpg-key="$KEY_ID" -passphrase="${GPG_PASSPHRASE:-}" stable
else
    info "Establishing initial publication protocol..."
    ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")
    KEY_ID=$(gpg --list-secret-keys --keyid-format=SHORT | grep "sec" | awk '{print $2}' | cut -d'/' -f2 | head -1)
    aptly publish repo -architectures="$ARCH,all" -gpg-key="$KEY_ID" -passphrase="${GPG_PASSPHRASE:-}" -distribution=stable myth-repo
fi

# 5. Export Public Key for Clients (Industry-grade cleanup)
info "Exporting GPG Public Key for clients..."
gpg --armor --export "$KEY_ID" > myth.gpg
ok "Public key exported to: myth.gpg"

echo ""
ok "Infrastructure Ready! Public Target: $PAGES_URL"
ok "Structure established at ~/.aptly/public"
ok "You can now run 'scripts/release_local.sh' to publish versions."
