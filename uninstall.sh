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

# These placeholders are replaced by CI/CD during release
AGENT_NAME="MYTH"
MYTH_VERSION="0.1.0"

# Fallback for local execution
if [[ "$AGENT_NAME" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        AGENT_NAME=$(grep "name:" config/agent.yaml | head -n 1 | sed -E 's/.*name:[[:space:]]*["'\'':]*([^"'\'']+)["'\'':]*.*/\1/' | awk '{print $1}')
        MYTH_VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
    else
        AGENT_NAME="MYTH"
        MYTH_VERSION="0.1.0"
    fi
fi

# Default Paths
TMP_DIR="${TMPDIR:-/tmp}"
LOG_FILE="${TMP_DIR}/myth-uninstall-$(date +%s).log"

# Industry Grade: Establish primary terminal link before redirection
exec 3>&1
# Redirect all subsequent stdout/stderr to log file, stripping ANSI codes
exec > >(sed -u 's/\x1b\[[0-9;]*[a-zA-Z]//g' >> "$LOG_FILE") 2>&1


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
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1" >&3; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1" >&3; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }


echo -e "${MAGENTA}${BOLD}${BANNER}${NC}" >&3
echo -e "${CYAN}  [ ${AGENT_NAME} — TACTICAL DECOMMISSIONING & SANITIZATION ]${NC}" >&3
echo -e "  ${BOLD}Initiating target neutralization (v${MYTH_VERSION})...${NC}\n" >&3
SCRUBBED_COUNT=0
info "Decommissioning logs initiated at: $LOG_FILE"


# ─── OS Detection ───
IS_TERMUX=false
DISTRO_FAMILY="unknown"
if [ -n "${PREFIX:-}" ] && echo "${PREFIX}" | grep -q "com.termux"; then
    IS_TERMUX=true
    DISTRO_FAMILY="termux"
elif [ -f /etc/os-release ]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    case "${ID:-}" in
        kali|debian|ubuntu|linuxmint|pop|raspbian|parrot)
            DISTRO_FAMILY="debian"
            ;;
        fedora|rhel|centos|rocky|alma|ol|amzn)
            DISTRO_FAMILY="fedora"
            ;;
        arch|manjaro|endeavouros|garuda|artix)
            DISTRO_FAMILY="arch"
            ;;
        *)
            if command -v apt &>/dev/null; then DISTRO_FAMILY="debian"
            elif command -v dnf &>/dev/null; then DISTRO_FAMILY="fedora"
            elif command -v pacman &>/dev/null; then DISTRO_FAMILY="arch"
            fi
            ;;
    esac
else
    if command -v apt &>/dev/null; then DISTRO_FAMILY="debian"
    elif command -v dnf &>/dev/null; then DISTRO_FAMILY="fedora"
    elif command -v pacman &>/dev/null; then DISTRO_FAMILY="arch"
    elif command -v apk &>/dev/null; then DISTRO_FAMILY="alpine"
    elif command -v zypper &>/dev/null; then DISTRO_FAMILY="opensuse"
    fi
fi

# ─── Tactical Environment Audit ───
if [ "$IS_TERMUX" = true ]; then
    ROOT_DIR="$PREFIX"
    BIN_DIR="$PREFIX/bin"
else
    ROOT_DIR=""
    BIN_DIR="/usr/bin"
    [ -d "/usr/local/bin" ] && BIN_DIR="/usr/local/bin"
fi

CONF_DIR="${ROOT_DIR}/etc/myth"
LOG_DIR="${ROOT_DIR}/var/log/myth"
# LIB_DIR is currently unused in the decommissioning logic
APT_SOURCES_DIR="${ROOT_DIR}/etc/apt/sources.list.d"
APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/keyrings"
[ ! -d "$APT_KEYRINGS_DIR" ] && APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/trusted.gpg.d"
YUM_REPOS_DIR="${ROOT_DIR}/etc/yum.repos.d"
PACMAN_CONF="${ROOT_DIR}/etc/pacman.conf"

# Check for sudo
if [ "$IS_TERMUX" = false ] && [ "$EUID" -ne 0 ]; then
    err "Privileged access required. Re-run as root (use sudo)."
fi

section "TACTICAL ASSET AUDIT"
audit "Detected OS Family: $DISTRO_FAMILY"
audit "Binary Path: $BIN_DIR"
audit "System Config: $CONF_DIR"

FOUND_ASSETS=0
[ -f "$BIN_DIR/myth" ] && audit "Found binary: myth" && ((FOUND_ASSETS++))
[ -d "$CONF_DIR" ] && audit "Found system config: $CONF_DIR" && ((FOUND_ASSETS++))
# Resolve user paths again for audit if needed, or use the vars
# We skip user paths here to avoid prompt noise, but system paths are checked.
[ "$FOUND_ASSETS" -eq 0 ] && info "No system-level assets currently active."

# 1. Native Package Neutralization
section "NATIVE PACKAGE NEUTRALIZATION"
echo -en "  ${CYAN}⠿  Purge binary and system-level neural conduits using package manager? [y/N]: ${NC}" >&3
read -r response < /dev/tty || response="N"
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    info "Purging native assets..."
    case "$DISTRO_FAMILY" in
        debian)
            if dpkg-query -W -f='${Status}' myth 2>/dev/null | grep -q "install ok installed"; then
                apt-get purge -y myth &>/dev/null || true
                ok "APT registry sanitized."
            else
                audit "No APT package identified."
            fi
            ;;
        fedora)
            PKG_CMD="dnf"
            command -v dnf &>/dev/null || PKG_CMD="yum"
            if $PKG_CMD list installed myth &>/dev/null; then
                $PKG_CMD remove -y myth &>/dev/null || true
                ok "$PKG_CMD registry sanitized."
            else
                audit "No DNF/YUM package identified."
            fi
            ;;
        arch)
            # Both yay/paru can be uninstalled via pacman standard
            INSTALLED_PKGS=$(pacman -Qq myth myth-bin 2>/dev/null || true)
            if [ -n "$INSTALLED_PKGS" ]; then
                pacman -Rns --noconfirm "$INSTALLED_PKGS" &>/dev/null || true
                ok "Pacman registry sanitized."
            else
                audit "No Pacman package identified."
            fi
            ;;
        termux)
            if dpkg-query -W -f='${Status}' myth 2>/dev/null | grep -q "install ok installed"; then
                pkg uninstall -y myth &>/dev/null || true
                ok "Termux registry sanitized."
            else
                audit "No pkg package identified."
            fi
            ;;
        *)
            audit "Skipped native uninstallation for unknown OS."
            ;;
    esac
else
    info "Native package manager assets preserved."
fi


# 1.1 Repository Sanitization
section "REPOSITORY SANITIZATION"
echo -en "  ${CYAN}⠿  Remove MYTH Repositories and GPG signing keys? [y/N]: ${NC}" >&3
read -r response < /dev/tty || response="N"
if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
    info "Deauthorizing repositories..."
    REPO_REMOVED=0
    case "$DISTRO_FAMILY" in
        debian)
            rm -f "$APT_SOURCES_DIR/myth.list" "$APT_KEYRINGS_DIR/myth.gpg"
            apt-get update -qq &>/dev/null || true
            REPO_REMOVED=1
            ;;
        fedora)
            if [ -n "$YUM_REPOS_DIR" ]; then
                rm -f "$YUM_REPOS_DIR/myth.repo"
                REPO_REMOVED=1
            fi
            ;;
        arch)
            if [ -f "$PACMAN_CONF" ] && grep -q "\[myth\]" "$PACMAN_CONF" 2>/dev/null; then
                sed -i '/\[myth\]/,+2d' "$PACMAN_CONF"
                REPO_REMOVED=1
            fi
            ;;
        termux)
            rm -f "$APT_SOURCES_DIR/myth.list" "$APT_KEYRINGS_DIR/myth.gpg"
            pkg update 2>/dev/null || true
            REPO_REMOVED=1
            ;;
    esac

    if [ "$REPO_REMOVED" -eq 1 ]; then
        ok "Tactical repository deauthorized."
    else
        audit "No repository configuration identified to remove."
    fi
else
    info "Repository configuration preserved."
fi

# 2. Global Binary Decommissioning
section "GLOBAL BINARY DECOMMISSIONING"
info "Searching for orphan binaries..."
PATHS_TO_REMOVE=("$BIN_DIR/myth" "$BIN_DIR/agent" "$BIN_DIR/chief")

DECOMMISSIONED=0
for path in "${PATHS_TO_REMOVE[@]}"; do
    if [ -f "$path" ] || [ -L "$path" ]; then
        audit "Decommissioning $path..."
        rm -f "$path"
        DECOMMISSIONED=1
    fi
done

if [ "$DECOMMISSIONED" -eq 1 ]; then
    ok "Global binaries neutralized."
else
    audit "No global binaries detected."
fi


# 3. System Asset Scrubbing
section "SYSTEM ASSET SCRUBBING"
if [ -d "$CONF_DIR" ] && [ "$CONF_DIR" != "/" ] && [ "$CONF_DIR" != "${ROOT_DIR}/etc" ]; then
    audit "System configuration detected at $CONF_DIR"
    info "Purging system assets..."
    rm -rf "$CONF_DIR"
    [ -d "$LOG_DIR" ] && [ "$LOG_DIR" != "/" ] && rm -rf "$LOG_DIR"
    ok "System-level assets scrubbed."
else
    audit "No system-level assets found or path is unsafe."
fi


# 4. Neural Link Sanitization (User Data)
section "NEURAL LINK SANITIZATION"
if [ "$IS_TERMUX" = true ]; then
    REAL_USER_HOME="$HOME"
    REAL_USER="$(whoami)"
else
    if [ -z "${SUDO_USER:-}" ]; then
        REAL_USER_HOME="$HOME"
        REAL_USER="$(whoami)"
    else
        # Multi-stage secure home resolution (Industry Standard)
        REAL_USER_HOME=$(getent passwd "$SUDO_USER" | cut -d: -f6 2>/dev/null || echo "/home/$SUDO_USER")
        if [ ! -d "$REAL_USER_HOME" ]; then
            # Fallback to eval home (handle special shells)
            REAL_USER_HOME=$(eval echo "~$SUDO_USER")
        fi
        REAL_USER="$SUDO_USER"
    fi
fi
USER_CONFIG="$REAL_USER_HOME/.config/myth"

if [ -d "$USER_CONFIG" ]; then
    echo -e "${YELLOW}⠿  Detected active neural profile for operative: $REAL_USER${NC}" >&3
    [ -f "$USER_CONFIG/.myth_history.db" ] && audit "Found Mission Vault (SQLite): .myth_history.db"
    [ -f "$USER_CONFIG/.myth_history" ] && audit "Found Legacy History: .myth_history"
    echo -e "   Physical Path: $USER_CONFIG" >&3
    echo -en "${CYAN}⠿  Wipe institutional-grade mission vault and operative context? [y/N]: ${NC}" >&3
    read -r response < /dev/tty || response="N"
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        info "Neutralizing operative context..."
        rm -rf "$USER_CONFIG"
        ok "Neural profile data and encrypted vault neutralized."
        ((SCRUBBED_COUNT++))
    else
        info "Operative context preserved."
    fi
else
    audit "No operative context identified for $REAL_USER."
fi

# 5. External Tactical Assets (Lightpanda, etc)
section "EXTERNAL ASSET SANITIZATION"
if [ "$IS_TERMUX" = true ]; then
    LIGHTPANDA_BIN="$PREFIX/bin/lightpanda"
else
    LIGHTPANDA_BIN="$REAL_USER_HOME/.local/bin/lightpanda"
fi

if [ -f "$LIGHTPANDA_BIN" ]; then
    audit "Detected provisioned browser engine: $LIGHTPANDA_BIN"
    echo -en "${CYAN}⠿  Neutralize external browser engine (Lightpanda)? [y/N]: ${NC}" >&3
    read -r response < /dev/tty || response="N"
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        info "Neutralizing browser engine..."
        rm -f "$LIGHTPANDA_BIN"
        # Cleanup temp artifacts regardless of ownership if running as root
        find "$TMP_DIR" -maxdepth 1 -name 'lightpanda_*' -delete 2>/dev/null || true
        ok "External engine and temporary artifacts neutralized."
    else
        info "External engine preserved."
    fi
else
    audit "No external browser engines identified for $REAL_USER."
fi

# 6. Documentation & Completions Cleanup
section "MANUALS & COMPLETIONS SANITIZATION"
MAN_PAGE="${ROOT_DIR}/usr/share/man/man1/myth.1.gz"
COMPLETIONS_DIR="${ROOT_DIR}/usr/share/bash-completion/completions"
ZSH_COMPLETIONS="${ROOT_DIR}/usr/share/zsh/site-functions/_myth"
FISH_COMPLETIONS="${ROOT_DIR}/usr/share/fish/vendor_completions.d/myth.fish"

for file in "$MAN_PAGE" "$COMPLETIONS_DIR/myth" "$ZSH_COMPLETIONS" "$FISH_COMPLETIONS"; do
    if [ -f "$file" ] || [ -L "$file" ]; then
        audit "Neutralizing $file..."
        rm -f "$file"
    fi
done

section "DECOMMISSION SUMMARY"
echo -e "${GREEN}${BOLD}  MYTH Tactical AI has been neutralized and decommissioned.${NC}" >&3
echo -e "  Summary: $SCRUBBED_COUNT major sectors sanitized." >&3
echo -e "  All tactical conduits have been scrubbed.\n" >&3
echo -e "${YELLOW}⠿  You may now safely delete the decommissioning log: $LOG_FILE${NC}" >&3
