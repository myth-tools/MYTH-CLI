#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — Tactical Bootstrap Utility (One-Liner)
# ═══════════════════════════════════════════════════
set -euo pipefail

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

# These placeholders are replaced by CI/CD during release
AGENT_NAME="__AGENT_NAME__"
VERSION="__VERSION__"
PAGES_URL="__PAGES_URL__"

# Fallback for local execution
if [[ "$AGENT_NAME" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        AGENT_NAME=$(grep "name:" config/agent.yaml | head -n 1 | sed -E 's/.*name:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/' | awk '{print $1}')
        VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
        PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E 's/.*pages_url:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/')
    else
        AGENT_NAME="MYTH"
        VERSION="0.1.0"
        PAGES_URL="https://myth.work.gd"
    fi
fi

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -en "${YELLOW}⚠  [WARN] ${NC} $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ ${AGENT_NAME} — TACTICAL REPOSITORY BOOTSTRAP ]${NC}"
echo -e "  ${BOLD}Establishing neural link (v${VERSION})...${NC}\n"

# ─── Configuration Discovery ───

# 1. Signing Authority Extraction
section "SIGNING AUTHORITY EXTRACTION"
info "Retrieving public signing authority from gateway..."
if ! wget -q --spider "${PAGES_URL}/myth.gpg"; then
    err "Gateway authority unavailable. Check network connectivity."
fi

sudo mkdir -p /etc/apt/keyrings
wget -qO- "${PAGES_URL}/myth.gpg" | sudo gpg --dearmor --yes -o /etc/apt/keyrings/myth.gpg
ok "Signing authority established."

# 2. Source List Configuration
section "SOURCE LIST CONFIGURATION"
info "Synchronizing tactical source lists..."
echo "deb [signed-by=/etc/apt/keyrings/myth.gpg] ${PAGES_URL} stable main" | sudo tee /etc/apt/sources.list.d/myth.list > /dev/null
ok "Source lists synchronized."

# 3. Core Synchronization
section "CORE SYNCHRONIZATION"
info "Updating tactical registries..."
sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/myth.list" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -qq
ok "Registries updated."

section "BOOTSTRAP COMPLETE"
echo -e "${GREEN}${BOLD}  Neural link established. You can now install MYTH via APT:${NC}"
echo -e "  ${BOLD}sudo apt install myth${NC}\n"
