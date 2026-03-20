#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — Tactical Decommissioning Utility
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

# ─── Professional Logging & Trapping ───
LOG_FILE="/tmp/myth-uninstall-$(date +%s).log"
exec 3>&1 # Save stdout to fd 3
# Redirect all subsequent stdout/stderr to log file (except for our high-fidelity UI)

cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ] && [ $exit_code -ne 130 ]; then
        echo -e "\n${RED}✘  [CRITICAL] Decommissioning interrupted.${NC}" >&3
        echo -e "${YELLOW}⠿  Technical logs preserved at: $LOG_FILE${NC}" >&3
    fi
    exit $exit_code
}
trap cleanup EXIT INT TERM

# High-fidelity status indicators (printing to fd 3)
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}" >&3; }
ok()      { echo -e "${GREEN}✔${NC}  $1" >&3; }
warn()    { echo -en "${YELLOW}⚠  [WARN] ${NC} $1" >&3; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1" >&3; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }

# Progress spinner
spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}" >&3
echo -e "${CYAN}  [ TACTICAL DECOMMISSIONING & SANITIZATION ]${NC}" >&3
echo -e "  ${BOLD}Initiating target neutralization...${NC}\n" >&3
info "Decommissioning logs initiated at: $LOG_FILE"

# Check for sudo
if [ "$EUID" -ne 0 ]; then
    err "Privileged access required. Re-run as root (use sudo)."
fi

# 1. APT Package Neutralization
section "DEBIAN PACKAGE NEUTRALIZATION"
if dpkg -l myth &>/dev/null; then
    audit "MYTH Tactical Package detected in APT registry."
    read -p "  Purge binary and system-level neural conduits? [y/N]: " response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        info "Purging APT assets..."
        apt-get purge -y myth &>/dev/null
        ok "APT registry sanitized."
    else
        info "APT assets preserved."
    fi
else
    audit "No APT package identified."
fi

# 1.1 Repository Sanitization
section "REPOSITORY SANITIZATION"
if [ -f "/etc/apt/sources.list.d/myth.list" ] || [ -f "/etc/apt/keyrings/myth.gpg" ]; then
    read -p "  Remove MYTH APT Repository and GPG signing keys? [y/N]: " response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        info "Deauthorizing repository..."
        rm -f "/etc/apt/sources.list.d/myth.list"
        rm -f "/etc/apt/keyrings/myth.gpg"
        apt-get update -qq -o Dir::Etc::sourcelist="sources.list.d/myth.list" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" || true
        ok "Tactical repository deauthorized."
    else
        info "Repository configuration preserved."
    fi
else
    audit "No repository configuration identified."
fi

# 2. Global Binary Decommissioning
section "GLOBAL BINARY DECOMMISSIONING"
info "Searching for orphan binaries in /usr/local/bin..."
PATHS_TO_REMOVE="/usr/local/bin/myth /usr/local/bin/agent /usr/local/bin/chief"
DECOMMISSIONED=0
for path in $PATHS_TO_REMOVE; do
    if [ -f "$path" ] || [ -L "$path" ]; then
        audit "Decommissioning $path..."
        rm -f "$path"
        DECOMMISSIONED=1
    fi
done

if [ $DECOMMISSIONED -eq 1 ]; then
    ok "Global binaries neutralized."
else
    audit "No global binaries detected."
fi

# 3. System Asset Scrubbing
section "SYSTEM ASSET SCRUBBING"
if [ -d "/etc/myth" ]; then
    audit "System configuration detected at /etc/myth"
    info "Purging system assets..."
    rm -rf "/etc/myth"
    ok "System-level assets scrubbed."
else
    audit "No system-level assets found."
fi

# 4. Neural Link Sanitization (User Data)
section "NEURAL LINK SANITIZATION"
if [ -z "${SUDO_USER:-}" ]; then
    REAL_USER_HOME="$HOME"
    REAL_USER="$USER"
else
    REAL_USER_HOME=$(eval echo "~$SUDO_USER")
    REAL_USER="$SUDO_USER"
fi
USER_CONFIG="$REAL_USER_HOME/.config/myth"

if [ -d "$USER_CONFIG" ]; then
    echo -e "${YELLOW}⠿  Detected active neural profile for operative: $REAL_USER${NC}"
    echo -e "   Physical Path: $USER_CONFIG"
    read -p "  Wipe all session history, profiles, and neural links? [y/N]: " response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        info "Scrubbing operative context..."
        rm -rf "$USER_CONFIG"
        ok "Neural profile data neutralized."
    else
        info "Neural profile preserved."
    fi
else
    audit "No operative context identified for $REAL_USER."
fi

section "DECOMMISSION COMPLETE"
echo -e "${GREEN}${BOLD}  MYTH Tactical AI has been neutralized and decommissioned.${NC}"
echo -e "  All tactical conduits have been scrubbed.\n"
