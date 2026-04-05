#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — Universal Tactical Repository Bootstrap
#  Adds the MYTH repository to your system's native
#  package manager so you can install and update MYTH
#  using your system's standard tools.
#
#  Supports: APT (Debian/Kali/Ubuntu/Pi), DNF (Fedora),
#            Pacman (Arch), pkg (Termux)
# ═══════════════════════════════════════════════════
set -euo pipefail

# ─── Visual Branding ───
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
MYTH_VERSION="0.1.0"
PAGES_URL="__PAGES_URL__"

# ─── Flag Parsing ───
DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
    esac
done

# Fallback for local execution
if [[ "$AGENT_NAME" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        AGENT_NAME=$(grep "name:" config/agent.yaml | head -n 1 | sed -E "s/.*name:[[:space:]]*[\"'\":]*([^\"']+)[\"'\":]*.*/\1/" | awk '{print $1}')
        # Standardized Extraction: Targets the top-level version field from Cargo.toml
        MYTH_VERSION=$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)

        PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E "s/.*pages_url:[[:space:]]*[\"'\":]*([^\"']+)[\"'\":]*.*/\1/")
    else
        AGENT_NAME="MYTH"
        MYTH_VERSION="0.1.0"
        PAGES_URL="https://myth.work.gd"
    fi
fi

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

# ─── Global Cleanup & Trapping ───
CLEANUP_FILES=()
cleanup() {
    local exit_code=$?
    if [ ${#CLEANUP_FILES[@]} -gt 0 ]; then
        rm -f "${CLEANUP_FILES[@]}" 2>/dev/null || true
    fi
    exit $exit_code
}
trap cleanup EXIT INT TERM


echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ ${AGENT_NAME} — UNIVERSAL TACTICAL REPOSITORY BOOTSTRAP ]${NC}"
echo -e "  ${BOLD}Establishing neural link (v${MYTH_VERSION})...${NC}\n"

# ─── Pre-flight Checks ───
section "PRE-FLIGHT VALIDATION"

# ─── OS Detection ───
DISTRO_FAMILY="unknown"
IS_TERMUX=false

if [ -n "${PREFIX:-}" ] && [[ "$PREFIX" == *"com.termux"* ]]; then
    DISTRO_FAMILY="termux"
    IS_TERMUX=true
    ok "Environment: Termux (Android)"
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
        alpine)
            DISTRO_FAMILY="alpine"
            ;;
        void)
            DISTRO_FAMILY="void"
            ;;
        opensuse*|sles)
            DISTRO_FAMILY="opensuse"
            ;;
        nixos)
            warn "NixOS detected. Automatic bootstrap is not supported. Add the MYTH flake to your configuration.nix."
            exit 0
            ;;
        *)
            if command -v apt &>/dev/null; then DISTRO_FAMILY="debian"
            elif command -v dnf &>/dev/null; then DISTRO_FAMILY="fedora"
            elif command -v pacman &>/dev/null; then DISTRO_FAMILY="arch"
            elif command -v apk &>/dev/null; then DISTRO_FAMILY="alpine"
            elif command -v zypper &>/dev/null; then DISTRO_FAMILY="opensuse"
            fi
            ;;
    esac
    ok "OS: ${PRETTY_NAME:-$ID} (Family: $DISTRO_FAMILY)"
else
    if command -v apt &>/dev/null; then DISTRO_FAMILY="debian"
    elif command -v dnf &>/dev/null; then DISTRO_FAMILY="fedora"
    elif command -v pacman &>/dev/null; then DISTRO_FAMILY="arch"
    fi
fi

if [ "$DISTRO_FAMILY" = "unknown" ]; then
    err "Could not detect your OS. Use the full installer: curl -sSL ${PAGES_URL}/install.sh | sudo bash"
fi

# Privilege Check (not needed for Termux)
if [ "$IS_TERMUX" = false ] && [ "$(id -u)" -ne 0 ]; then
    err "Privileged access required. Re-run as: curl -sSL ${PAGES_URL}/bootstrap.sh | sudo bash"
fi
if [ "$IS_TERMUX" = false ]; then
    ok "Root privileges confirmed."
fi

# Downloader resolution
if command -v curl &>/dev/null; then
    DOWNLOADER="curl"
elif command -v wget &>/dev/null; then
    DOWNLOADER="wget"
else
    err "Neither 'curl' nor 'wget' found. Install one first."
fi
ok "Downloader: $DOWNLOADER"

# ─── Download a file robustly ───
download_file() {
    local url="$1"
    local dest="$2"
    if [ "$DOWNLOADER" = "curl" ]; then
        curl -fsSL --max-time 30 --retry 3 --retry-delay 2 "$url" -o "$dest"
    else
        wget -q --timeout=30 --tries=3 "$url" -O "$dest"
    fi
}

# ═══════════════════════════════════════════════════
#  DEBIAN / KALI / UBUNTU / PI (APT)
# ═══════════════════════════════════════════════════
bootstrap_debian() {
    # ─── OS Detection ───
    # ─── Tactical Environment Audit ───
    if [ "$IS_TERMUX" = true ]; then
        ROOT_DIR="$PREFIX"
    else
        ROOT_DIR=""
    fi

    APT_SOURCES_DIR="${ROOT_DIR}/etc/apt/sources.list.d"
    APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/keyrings"
    [ ! -d "$APT_KEYRINGS_DIR" ] && APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/trusted.gpg.d"

    section "SIGNING AUTHORITY EXTRACTION"
    info "Retrieving public signing authority from: ${PAGES_URL}/myth.gpg"

    mkdir -p "$APT_SOURCES_DIR" "$APT_KEYRINGS_DIR"
    
    if [ "$DRY_RUN" = true ]; then
        info "[DRY-RUN] Would download signing key and create $APT_SOURCES_DIR/myth.list"
        return 0
    fi

    rm -f "$APT_KEYRINGS_DIR/myth.gpg"

    TEMP_KEY=$(mktemp "${TMPDIR:-/tmp}/myth-key.XXXXXX")
    CLEANUP_FILES+=("$TEMP_KEY")
    
    download_file "${PAGES_URL}/myth.gpg" "$TEMP_KEY" \
        || err "Failed to download signing key."

    [ -s "$TEMP_KEY" ] || { rm -f "$TEMP_KEY"; err "Downloaded signing key is empty."; }

    # Improved PGP detection
    IS_ARMORED=false
    if command -v file &>/dev/null; then
        if file "$TEMP_KEY" | grep -q "PGP public key block"; then
            IS_ARMORED=true
        fi
    elif head -n 1 "$TEMP_KEY" | grep -q "BEGIN PGP PUBLIC KEY"; then
        IS_ARMORED=true
    fi

    if [ "$IS_ARMORED" = true ]; then
        gpg --dearmor --yes -o "$APT_KEYRINGS_DIR/myth.gpg" "$TEMP_KEY" 2>/dev/null \
            || { rm -f "$TEMP_KEY"; err "GPG dearmor failed."; }
    else
        cp "$TEMP_KEY" "$APT_KEYRINGS_DIR/myth.gpg"
    fi
    rm -f "$TEMP_KEY"

    [ -s "$APT_KEYRINGS_DIR/myth.gpg" ] || err "GPG keyring is empty after conversion."
    ok "Signing authority installed at $APT_KEYRINGS_DIR/myth.gpg"

    section "SOURCE LIST CONFIGURATION"
    info "Configuring tactical APT source..."
    echo "deb [signed-by=$APT_KEYRINGS_DIR/myth.gpg] ${PAGES_URL} stable main" \
        | tee "$APT_SOURCES_DIR/myth.list" > /dev/null
    ok "Source list created at $APT_SOURCES_DIR/myth.list"

    section "REGISTRY SYNCHRONIZATION"
    info "Synchronizing tactical package registry..."
    apt-get update -o Dir::Etc::sourcelist="$APT_SOURCES_DIR/myth.list" \
                   -o Dir::Etc::sourceparts="-" \
                   -o APT::Get::List-Cleanup="0" -qq \
        || err "apt-get update failed."
    ok "MYTH package registry synchronized."

    section "BOOTSTRAP COMPLETE"
    echo -e "${GREEN}${BOLD}  Neural link established. Install MYTH with:${NC}"
    echo -e ""
    echo -e "    ${BOLD}${CYAN}sudo apt install myth${NC}"
    echo -e ""
    echo -e "  Future updates: ${BOLD}sudo apt update && sudo apt upgrade myth${NC}"
}

# ═══════════════════════════════════════════════════
#  TERMUX (Android — pkg/apt)
# ═══════════════════════════════════════════════════
bootstrap_termux() {
    TERMUX_SOURCES_DIR="$PREFIX/etc/apt/sources.list.d"
    TERMUX_KEYRINGS_DIR="$PREFIX/etc/apt/trusted.gpg.d"
    mkdir -p "$TERMUX_SOURCES_DIR" "$TERMUX_KEYRINGS_DIR"

    section "SIGNING AUTHORITY EXTRACTION"
    info "Importing signing key for Termux..."

    TEMP_KEY=$(mktemp "${TMPDIR:-/tmp}/myth-key.XXXXXX")
    CLEANUP_FILES+=("$TEMP_KEY")

    download_file "${PAGES_URL}/myth.gpg" "$TEMP_KEY" \
        || err "Failed to download signing key."

    [ -s "$TEMP_KEY" ] || err "Downloaded signing key is empty."

    if head -c 27 "$TEMP_KEY" | grep -q "BEGIN PGP PUBLIC KEY"; then
        gpg --dearmor --yes -o "$TERMUX_KEYRINGS_DIR/myth.gpg" "$TEMP_KEY" 2>/dev/null \
            || err "GPG dearmor failed."
    else
        cp "$TEMP_KEY" "$TERMUX_KEYRINGS_DIR/myth.gpg"
    fi
    ok "Signing authority installed."

    section "SOURCE LIST CONFIGURATION"
    echo "deb [signed-by=${TERMUX_KEYRINGS_DIR}/myth.gpg] ${PAGES_URL} stable main" \
        > "$TERMUX_SOURCES_DIR/myth.list"
    ok "Source configured: $TERMUX_SOURCES_DIR/myth.list"

    section "REGISTRY SYNCHRONIZATION"
    apt-get update -o Dir::Etc::sourcelist="$TERMUX_SOURCES_DIR/myth.list" \
                   -o Dir::Etc::sourceparts="-" \
                   -o APT::Get::List-Cleanup="0" -qq \
        || err "apt-get update failed."
    ok "MYTH package registry synchronized."

    section "BOOTSTRAP COMPLETE"
    echo -e "${GREEN}${BOLD}  Neural link established. Install MYTH with:${NC}"
    echo -e ""
    echo -e "    ${BOLD}${CYAN}pkg install myth${NC}"
    echo -e ""
    echo -e "  Future updates: ${BOLD}pkg upgrade myth${NC}"
}

# ═══════════════════════════════════════════════════
#  FEDORA / RHEL / CentOS (DNF/YUM)
# ═══════════════════════════════════════════════════
bootstrap_fedora() {
    PKG_CMD="dnf"
    command -v dnf &>/dev/null || PKG_CMD="yum"

    section "SIGNING AUTHORITY IMPORT"
    info "Importing GPG signing key..."
    
    if [ "$DRY_RUN" = true ]; then
        info "[DRY-RUN] Would import GPG key and create /etc/yum.repos.d/myth.repo"
        return 0
    fi

    TEMP_KEY=$(mktemp "${TMPDIR:-/tmp}/myth-key.XXXXXX")
    CLEANUP_FILES+=("$TEMP_KEY")
    download_file "${PAGES_URL}/myth.gpg" "$TEMP_KEY" \
        || err "Failed to download signing key."

    rpm --import "$TEMP_KEY" 2>/dev/null \
        || warn "GPG import warning (non-fatal)."
    rm -f "$TEMP_KEY"
    ok "Signing key imported."

    section "REPOSITORY CONFIGURATION"
    info "Adding MYTH RPM repository..."
    cat > /etc/yum.repos.d/myth.repo << REPOEOF
[myth]
name=MYTH Official Repository
baseurl=${PAGES_URL}/rpm
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=${PAGES_URL}/myth.gpg
type=rpm
REPOEOF
    ok "Repository configured: /etc/yum.repos.d/myth.repo"

    section "REGISTRY SYNCHRONIZATION"
    info "Synchronizing package registry..."
    $PKG_CMD makecache -q 2>/dev/null || true
    ok "MYTH package registry synchronized."

    section "BOOTSTRAP COMPLETE"
    echo -e "${GREEN}${BOLD}  Neural link established. Install MYTH with:${NC}"
    echo -e ""
    echo -e "    ${BOLD}${CYAN}sudo $PKG_CMD install myth${NC}"
    echo -e ""
    echo -e "  Future updates: ${BOLD}sudo $PKG_CMD upgrade myth${NC}"
}

# ═══════════════════════════════════════════════════
#  ARCH / MANJARO (Pacman + AUR)
# ═══════════════════════════════════════════════════
bootstrap_arch() {
    section "REPOSITORY CONFIGURATION"

    # Check for AUR helpers first
    HAS_YAY=false
    HAS_PARU=false
    command -v yay &>/dev/null && HAS_YAY=true
    command -v paru &>/dev/null && HAS_PARU=true

    if [ "$HAS_YAY" = true ] || [ "$HAS_PARU" = true ]; then
        ok "AUR helper detected."
        section "BOOTSTRAP COMPLETE"
        echo -e "${GREEN}${BOLD}  Neural link established. Install MYTH with:${NC}"
        echo -e ""
        if [ "$HAS_YAY" = true ]; then
            echo -e "    ${BOLD}${CYAN}yay -S myth-bin${NC}"
        fi
        if [ "$HAS_PARU" = true ]; then
            echo -e "    ${BOLD}${CYAN}paru -S myth-bin${NC}"
        fi
        echo -e ""
        echo -e "  Future updates: ${BOLD}yay -Syu${NC} or ${BOLD}paru -Syu${NC}"
    else
        # Add custom repo to pacman.conf
        info "No AUR helper found. Adding custom MYTH repository..."
        if ! grep -q "\[myth\]" /etc/pacman.conf 2>/dev/null; then
            cat >> /etc/pacman.conf << ARCHEOF

[myth]
SigLevel = Optional TrustAll
Server = ${PAGES_URL}/arch
ARCHEOF
            ok "Custom repo added to /etc/pacman.conf"
        else
            ok "MYTH repo already in /etc/pacman.conf"
        fi

        section "REGISTRY SYNCHRONIZATION"
        info "Synchronizing package database..."
        pacman -Sy --noconfirm 2>/dev/null || true
        ok "MYTH package registry synchronized."

        section "BOOTSTRAP COMPLETE"
        echo -e "${GREEN}${BOLD}  Neural link established. Install MYTH with:${NC}"
        echo -e ""
        echo -e "    ${BOLD}${CYAN}sudo pacman -S myth${NC}"
        echo -e ""
        echo -e "  Future updates: ${BOLD}sudo pacman -Syu${NC}"
    fi
}

# ═══════════════════════════════════════════════════
#  ALPINE LINUX (APK)
# ═══════════════════════════════════════════════════
bootstrap_alpine() {
    section "REPOSITORY CONFIGURATION"
    info "Configuring Alpine repository..."
    if [ "$DRY_RUN" = true ]; then
        info "[DRY-RUN] Would add repository to /etc/apk/repositories"
        return 0
    fi
    # Implementation placeholder for Alpine repository bootstrap
    warn "Alpine repository support is experimental. Use the full installer if this fails."
}

# ═══════════════════════════════════════════════════
#  OPENSUSE (ZYPPER)
# ═══════════════════════════════════════════════════
bootstrap_opensuse() {
    section "REPOSITORY CONFIGURATION"
    info "Configuring openSUSE repository..."
    if [ "$DRY_RUN" = true ]; then
        info "[DRY-RUN] Would add repository via zypper ar"
        return 0
    fi
    zypper ar -f -n "MYTH Repository" "${PAGES_URL}/rpm" myth || true
    ok "Repository added via zypper."
}

# ─── Execute ───
case "$DISTRO_FAMILY" in
    debian)  bootstrap_debian ;;
    termux)  bootstrap_termux ;;
    fedora)  bootstrap_fedora ;;
    arch)    bootstrap_arch ;;
    alpine)  bootstrap_alpine ;;
    opensuse) bootstrap_opensuse ;;
    *)       err "Unsupported OS family: $DISTRO_FAMILY. Use the full installer instead." ;;
esac

echo -e "\n  Or visit ${PAGES_URL} for full instructions."
echo -e ""
