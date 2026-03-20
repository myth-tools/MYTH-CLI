#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH CLI — One-Line Installer for Kali Linux / Debian
# ═══════════════════════════════════════════════════════════
#
#  Usage:
#    curl -sSL https://myth.work.gd/install.sh | sudo bash
#    OR
#    bash scripts/install.sh
#
set -eo pipefail

# ─── Early Constants ───
REAL_USER="${SUDO_USER:-$USER}"
REAL_HOME=$(eval echo "~${REAL_USER}")
CONFIG_DIR="${REAL_HOME}/.config/myth"
BUILD_DIR="/tmp/myth-build-$(date +%s)"

# ─── Dynamic Repository Configuration ───
# These placeholders are replaced by CI/CD during release
REPO_URL="https://github.com/myth-tools/MYTH-CLI"
PAGES_URL="https://myth.work.gd"
VERSION="0.1.0"
AGENT_NAME="MYTH"

# Fallback for local execution (if placeholders were not replaced)
if [[ "$REPO_URL" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        AGENT_NAME=$(grep "name:" config/agent.yaml | head -n 1 | sed -E 's/.*name:[[:space:]]*["'\''":]*([^"'\'']+)["'\''":]*.*/\1/' | awk '{print $1}')
        VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
        REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E 's/.*repository_url:[[:space:]]*["'\''"]?([^"'\'']+)["'\''"]?.*/\1/')
        PAGES_DOMAIN=$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')
        PAGES_URL="https://$PAGES_DOMAIN"
    else
        DEFAULT_REPO="https://github.com/myth-tools/MYTH-CLI"
        REPO_URL=$DEFAULT_REPO
        PAGES_DOMAIN=$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')
        PAGES_URL="https://$PAGES_DOMAIN"
    fi
fi

CLEAN_REPO_URL=$(echo "$REPO_URL" | sed -E 's|/*$||' | sed -E 's|\.git$||')

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

# ─── Professional Logging & Trapping ───
LOG_FILE="/tmp/myth-install-$(date +%s).log"
exec 3>&1

cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ] && [ $exit_code -ne 130 ]; then
        echo -e "\n${RED}✘  [CRITICAL] Installation aborted unexpectedly.${NC}" >&3
        echo -e "${YELLOW}⠿  Technical logs preserved at: $LOG_FILE${NC}" >&3
    fi
    jobs -p 2>/dev/null | xargs kill -9 2>/dev/null || true
    rm -rf "$BUILD_DIR" 2>/dev/null || true
    exit $exit_code
}
trap cleanup EXIT INT TERM

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}" >&3; }
ok()      { echo -e "${GREEN}✔${NC}  $1" >&3; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1" >&3; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1" >&3; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }

require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "$1 is required but not installed."
    fi
}

# ─── Start ───
echo -e "${MAGENTA}${BOLD}${BANNER}${NC}" >&3
echo -e "${CYAN}  [ ${AGENT_NAME} — DIGITAL RECONNAISSANCE & TACTICAL AI ]${NC}" >&3
echo -e "  ${BOLD}Version: ${VERSION}${NC}\n" >&3
info "Logs: $LOG_FILE"

# ─── Check OS ───
if [ ! -f /etc/debian_version ]; then
    err "This installer requires a Debian-based system (Kali, Ubuntu, Debian)."
fi

# ─── Check Architecture ───
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  RUST_TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    *)       err "Unsupported architecture: $ARCH" ;;
esac
ok "Architecture: $ARCH ($RUST_TARGET)"

# ─── System Audit ───
section "SYSTEM AUDIT & PRE-FLIGHT"
audit "OS: $(grep PRETTY_NAME /etc/os-release | cut -d'=' -f2 | tr -d '\"')"
audit "ARCH: $ARCH ($RUST_TARGET)"
if [ -d "$CONFIG_DIR" ]; then audit "Existing profile at $CONFIG_DIR"; fi
if command -v bwrap &>/dev/null; then ok "Sandbox (Bubblewrap) verified"; else warn "Bubblewrap missing. Will install."; fi

# ─── Dependency Resolution ───
section "DEPENDENCY RESOLUTION"
info "Synchronizing tactical dependencies..."
apt-get update -qq 2>&1 | tee -a "$LOG_FILE" | tail -1 >&3 || true
apt-get install -y -qq \
    bubblewrap build-essential pkg-config libssl-dev \
    curl git wget protobuf-compiler 2>&1 | tee -a "$LOG_FILE" | tail -1 >&3 || warn "Some dependencies failed."
ok "Dependencies synchronized."

# ─── Recommended Security Tools (non-Kali) ───
if ! grep -qi "kali" /etc/os-release 2>/dev/null; then
    RECOMMENDED_TOOLS="nmap whois curl dnsutils"
    TO_INSTALL=""
    for tool in $RECOMMENDED_TOOLS; do
        if ! command -v "$tool" &>/dev/null; then
            if [ "$tool" = "dnsutils" ] && command -v dig &>/dev/null; then continue; fi
            TO_INSTALL="$TO_INSTALL $tool"
        fi
    done
    if [ -n "$TO_INSTALL" ]; then
        info "Installing:$TO_INSTALL..."
        apt-get install -y -qq $TO_INSTALL 2>/dev/null || warn "Some tools failed."
        ok "Security tools installed."
    fi
fi

# ═══════════════════════════════════════════════════════════
#  PATH A: APT Installation (Preferred)
# ═══════════════════════════════════════════════════════════
section "PRIMARY DEPLOYMENT (APT)"
info "Attempting binary deployment via APT..."
APT_SUCCESS=false

# 1. Always refresh the GPG key (removes stale/empty files)
info "Retrieving signing authority..."
mkdir -p /etc/apt/keyrings
rm -f /etc/apt/keyrings/myth.gpg  # Remove stale key

if curl -fsSL "${PAGES_URL}/myth.gpg" -o /tmp/myth-key-download.gpg 2>/dev/null; then
    # Check if download is non-empty
    if [ -s /tmp/myth-key-download.gpg ]; then
        # Detect format: ASCII-armored keys start with "-----BEGIN"
        if head -c 20 /tmp/myth-key-download.gpg | grep -q "BEGIN"; then
            # ASCII-armored → dearmor to binary
            gpg --dearmor --yes -o /etc/apt/keyrings/myth.gpg /tmp/myth-key-download.gpg 2>/dev/null
        else
            # Already binary → copy directly
            cp /tmp/myth-key-download.gpg /etc/apt/keyrings/myth.gpg
        fi
        rm -f /tmp/myth-key-download.gpg

        # Validate the keyring file is non-empty
        if [ -s /etc/apt/keyrings/myth.gpg ]; then
            ok "Signing authority installed."
        else
            warn "GPG key conversion failed (empty output)."
            rm -f /etc/apt/keyrings/myth.gpg
        fi
    else
        warn "Downloaded key is empty."
        rm -f /tmp/myth-key-download.gpg
    fi
else
    warn "Could not download signing key from ${PAGES_URL}/myth.gpg"
fi

# 2. Configure APT source & attempt install
if [ -f "/etc/apt/keyrings/myth.gpg" ] && [ -s "/etc/apt/keyrings/myth.gpg" ]; then
    info "Configuring source lists..."
    echo "deb [signed-by=/etc/apt/keyrings/myth.gpg] ${PAGES_URL} stable main" | tee /etc/apt/sources.list.d/myth.list > /dev/null

    info "Synchronizing mirror..."
    if apt-get update -o Dir::Etc::sourcelist="sources.list.d/myth.list" \
        -o Dir::Etc::sourceparts="-" \
        -o APT::Get::List-Cleanup="0" -qq 2>&1 | tee -a "$LOG_FILE"; then

        info "Installing MYTH package..."
        if apt-get install -y myth 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3; then
            APT_SUCCESS=true
            ok "MYTH deployed via APT."
        fi
    fi

    if [ "$APT_SUCCESS" = false ]; then
        warn "APT failed. Falling back to source compilation..."
        # Clean up failed APT source to prevent future apt-get update noise
        rm -f /etc/apt/sources.list.d/myth.list
    fi
else
    warn "No valid signing key. Falling back to source compilation..."
fi

# ═══════════════════════════════════════════════════════════
#  PATH B: Source Compilation (Fallback)
# ═══════════════════════════════════════════════════════════
if [ "$APT_SUCCESS" = false ]; then

    section "SOURCE DEPLOYMENT (LEVEL 3)"

    # ─── Ensure a WORKING Rust toolchain ───
    # Key insight: under `sudo`, the user's ~/.cargo/bin is in PATH
    # but root has no rustup/cargo installation. We must detect this.
    install_rust_fresh() {
        info "Installing fresh Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable 2>&1 | tail -3
        # Source the new env
        if [ -f "$HOME/.cargo/env" ]; then
            . "$HOME/.cargo/env"
        fi
    }

    # Test if cargo ACTUALLY works (not just exists in PATH)
    if cargo --version &>/dev/null 2>&1; then
        ok "Cargo is functional: $(cargo --version 2>/dev/null)"
    else
        # cargo exists in PATH but doesn't work (rustup shim without toolchain)
        # OR cargo doesn't exist at all. Either way, install fresh.
        if command -v rustup &>/dev/null && rustup --version &>/dev/null 2>&1; then
            # rustup works but no default toolchain
            info "Setting default Rust toolchain..."
            rustup default stable 2>&1 | tail -1
            if cargo --version &>/dev/null 2>&1; then
                ok "Cargo configured: $(cargo --version 2>/dev/null)"
            else
                install_rust_fresh
            fi
        else
            install_rust_fresh
        fi
    fi

    # Final cargo verification
    if ! cargo --version &>/dev/null 2>&1; then
        err "Cargo is not functional. Cannot compile from source. Install Rust manually: https://rustup.rs"
    fi

    # ─── Build ───
    info "Initiating neural core compilation..."
    if [ -f "Cargo.toml" ]; then
        info "Building from local source..."
        if ! cargo build --release 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
            err "Compilation failed. See $LOG_FILE for details."
        fi
    else
        require_command git
        info "Cloning repository..."
        if ! git clone --depth 1 "${CLEAN_REPO_URL}.git" "$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"; then
            err "Failed to clone ${CLEAN_REPO_URL}"
        fi
        cd "$BUILD_DIR"
        info "Compiling from source (this may take several minutes)..."
        if ! cargo build --release 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
            err "Compilation failed. See $LOG_FILE for details."
        fi
    fi

    # ─── Verify & Install Binary ───
    if [ ! -f "target/release/myth" ]; then
        err "Binary not found at target/release/myth after compilation."
    fi

    ok "Neural Core Compilation Complete."
    info "Installing binaries..."
    cp target/release/myth /usr/local/bin/myth
    ln -sf /usr/local/bin/myth /usr/local/bin/agent
    ln -sf /usr/local/bin/myth /usr/local/bin/chief
    ok "Binaries deployed to /usr/local/bin/"
fi

# ═══════════════════════════════════════════════════════════
#  Configuration & Setup
# ═══════════════════════════════════════════════════════════
USER_YAML="${CONFIG_DIR}/user.yaml"

if [ ! -f "$USER_YAML" ]; then
    mkdir -p "$CONFIG_DIR"
    info "Creating config at $USER_YAML"
    if [ -f "config/user.yaml" ]; then
        cp config/user.yaml "$USER_YAML"
    else
        cat <<EOF > "$USER_YAML"
agent:
  user_name: "Chief"
  nvidia_api_key: ""
EOF
    fi
    if [ -f "config/mcp.json" ]; then
        cp config/mcp.json "$CONFIG_DIR/mcp.json"
    fi
    # Fix ownership so the real user owns their config
    if [ -n "${SUDO_USER:-}" ]; then
        chown -R "$SUDO_USER:$(id -gn "$SUDO_USER")" "$CONFIG_DIR"
    fi
    ok "Configured at $CONFIG_DIR/"
else
    audit "Existing profile at $USER_YAML"
fi

# ─── Operative Identification ───
section "OPERATIVE INITIALIZATION"
if [ -t 0 ]; then
    CURRENT_NAME=$(grep "user_name:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"'\''' || echo "")
    if [ -z "$CURRENT_NAME" ] || [ "$CURRENT_NAME" = "Chief" ]; then
        echo -en "${CYAN}⠿  Enter your Operative Handle [Default: Chief]: ${NC}" >&3
        read OPERATIVE_NAME || OPERATIVE_NAME=""
        OPERATIVE_NAME=${OPERATIVE_NAME:-Chief}
        sed -i "s/user_name: .*/user_name: \"$OPERATIVE_NAME\"/" "$USER_YAML"
        ok "Handle: $OPERATIVE_NAME"
    fi

    CURRENT_KEY=$(grep "nvidia_api_key:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"'\''' || echo "")
    if [ -z "$CURRENT_KEY" ]; then
        echo -en "${CYAN}⠿  NVIDIA NIM API Key (optional, press Enter to skip): ${NC}" >&3
        read -s API_KEY || API_KEY=""
        echo "" >&3
        if [ -n "$API_KEY" ]; then
            sed -i "s/nvidia_api_key: .*/nvidia_api_key: \"$API_KEY\"/" "$USER_YAML"
            ok "API Key configured."
        else
            warn "No API Key. Neural reasoning disabled until set."
        fi
    fi
else
    info "Non-interactive mode. Skipping identification."
fi

# ─── Final Validation ───
section "FINAL SECURITY AUDIT"
if command -v bwrap &>/dev/null; then
    ok "Sandbox: $(bwrap --version 2>/dev/null | head -1)"
else
    warn "Sandbox missing. Run: sudo apt install bubblewrap"
fi

if command -v myth &>/dev/null; then
    ok "MYTH Binary: $(myth --version 2>/dev/null | head -1 || echo 'installed')"
else
    warn "MYTH not found in PATH."
fi

# ─── Done ───
echo -e "\n${GREEN}${BOLD}  ✅ MYTH DEPLOYMENT COMPLETE!${NC}" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3
echo -e "  ${BOLD}Config:${NC}  $USER_YAML" >&3
echo -e "  ${BOLD}Binary:${NC}  /usr/local/bin/myth" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3
echo "" >&3
echo -e "  ${BOLD}Next:${NC}" >&3
echo -e "    1. ${CYAN}myth sync${NC}        - Synchronize 3000+ Kali tools" >&3
echo -e "    2. ${CYAN}myth check${NC}       - System health check" >&3
echo -e "    3. ${CYAN}myth scan <target>${NC} - First mission" >&3
echo "" >&3
