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
# Note: -u (nounset) is intentionally omitted for robustness
#       across different shell environments and sudo contexts.

# ─── Early Constants ───
CONFIG_DIR="${HOME}/.config/myth"
BUILD_DIR="/tmp/myth-build-$(date +%s)"
REAL_USER="${SUDO_USER:-$USER}"
REAL_HOME=$(eval echo "~${REAL_USER}")

# ─── Dynamic Repository Configuration ───
# These placeholders are replaced by CI/CD during release
REPO_URL="https://github.com/myth-tools/MYTH-CLI"
PAGES_URL="https://myth.work.gd"

# Fallback for local execution (if placeholders were not replaced)
if [[ "$REPO_URL" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
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

# URL Normalization
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
exec 3>&1 # Save stdout to fd 3

cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ] && [ $exit_code -ne 130 ]; then
        echo -e "\n${RED}✘  [CRITICAL] Installation aborted unexpectedly.${NC}" >&3
        echo -e "${YELLOW}⠿  Technical logs preserved at: $LOG_FILE${NC}" >&3
    fi
    jobs -p | xargs kill -9 2>/dev/null || true
    rm -rf "$BUILD_DIR" 2>/dev/null
    exit $exit_code
}
trap cleanup EXIT INT TERM

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}" >&3; }
ok()      { echo -e "${GREEN}✔${NC}  $1" >&3; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC} $1" >&3; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1" >&3; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }

# Dependency verification utility
require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "$1 is required but not installed. Tactical deployment aborted."
    fi
}

# ─── Start ───
echo -e "${MAGENTA}${BOLD}${BANNER}${NC}" >&3
echo -e "${CYAN}  [ DIGITAL RECONNAISSANCE & TACTICAL AI AGENT ]${NC}" >&3
echo -e "  ${BOLD}Version: 1.0.0-Stable${NC}\n" >&3
info "Detailed installation logs initiated at: $LOG_FILE"

# ─── Check OS ───
if [ ! -f /etc/debian_version ]; then
    err "This installer is for Debian-based systems only (Kali, Ubuntu, Debian)."
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

if [ -d "$CONFIG_DIR" ]; then audit "Persistent profile detected at $CONFIG_DIR"; fi
if command -v bwrap &>/dev/null; then ok "Sandboxing engine (Bubblewrap) verified"; else warn "Sandboxing engine missing."; fi

# ─── Dependency Resolution ───
section "DEPENDENCY RESOLUTION"
info "Synchronizing tactical dependencies..."
sudo apt-get update -qq 2>&1 | tail -1 || true
sudo apt-get install -y -qq \
    bubblewrap build-essential pkg-config libssl-dev \
    curl git protobuf-compiler 2>&1 | tail -1 || warn "Some dependencies may have failed to install."
ok "Tactical dependencies synchronized."

# ─── Install Recommended Security Tools (for non-Kali) ───
if ! grep -qi "kali" /etc/os-release 2>/dev/null; then
    info "Non-Kali system detected. Checking for recommended security tools..."
    RECOMMENDED_TOOLS="nmap whois curl dnsutils"
    TO_INSTALL=""
    for tool in $RECOMMENDED_TOOLS; do
        if ! command -v "$tool" &>/dev/null; then
            if [ "$tool" == "dnsutils" ] && command -v dig &>/dev/null; then continue; fi
            TO_INSTALL="$TO_INSTALL $tool"
        fi
    done

    if [ -n "$TO_INSTALL" ]; then
        info "Installing missing recommended tools:$TO_INSTALL..."
        sudo apt-get install -y -qq $TO_INSTALL 2>/dev/null || warn "Failed to install some tools."
        ok "Recommended tools installed"
    else
        ok "All recommended core tools already present"
    fi
fi

# ═══════════════════════════════════════════════════════════
#  PATH A: APT Installation (Preferred)
# ═══════════════════════════════════════════════════════════
section "PRIMARY DEPLOYMENT (APT)"
info "Attempting high-speed binary synchronization..."

APT_SUCCESS=false

# 1. Download & install GPG key
if [ ! -f "/etc/apt/keyrings/myth.gpg" ]; then
    info "Retrieving public signing authority..."
    if curl -fsSL "${PAGES_URL}/myth.gpg" -o /tmp/myth-key.gpg 2>/dev/null; then
        sudo mkdir -p /etc/apt/keyrings
        # Detect if key is already in binary (dearmored) format or ASCII-armored
        if file /tmp/myth-key.gpg | grep -qi "PGP"; then
            # ASCII-armored key: needs dearmoring
            sudo gpg --dearmor --yes -o /etc/apt/keyrings/myth.gpg /tmp/myth-key.gpg 2>/dev/null
        else
            # Already binary/dearmored: copy directly
            sudo cp /tmp/myth-key.gpg /etc/apt/keyrings/myth.gpg
        fi
        rm -f /tmp/myth-key.gpg
        ok "Signing authority installed."
    else
        warn "Signing authority unavailable at gateway."
    fi
fi

# 2. Configure APT source & attempt install
if [ -f "/etc/apt/keyrings/myth.gpg" ]; then
    info "Configuring tactical source lists..."
    echo "deb [signed-by=/etc/apt/keyrings/myth.gpg] ${PAGES_URL} stable main" | sudo tee /etc/apt/sources.list.d/myth.list > /dev/null

    info "Mirror synchronization..."
    if sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/myth.list" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -qq 2>&1 | tail -1; then
        info "Core installation..."
        if sudo apt-get install -y -qq myth 2>&1 | tail -1; then
            APT_SUCCESS=true
            ok "MYTH deployed successfully via APT."
        fi
    fi

    if [ "$APT_SUCCESS" = false ]; then
        warn "APT synchronization failed. Shifting to Source Level 3 Deployment..."
    fi
else
    warn "Gateway keys missing. Shifting to Source Level 3 Deployment..."
fi

# ═══════════════════════════════════════════════════════════
#  PATH B: Source Compilation (Fallback)
# ═══════════════════════════════════════════════════════════
if [ "$APT_SUCCESS" = false ]; then

    # ─── Ensure a working Rust/Cargo toolchain ───
    ensure_rust() {
        # Source cargo env from common locations
        for env_file in "$HOME/.cargo/env" "$REAL_HOME/.cargo/env" "/root/.cargo/env"; do
            [ -f "$env_file" ] && . "$env_file" 2>/dev/null || true
        done

        # If cargo still not found, install Rust from scratch
        if ! command -v cargo &>/dev/null; then
            info "Installing Rust toolchain..."
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable 2>&1 | tail -5
            for env_file in "$HOME/.cargo/env" "$REAL_HOME/.cargo/env"; do
                [ -f "$env_file" ] && . "$env_file" 2>/dev/null || true
            done
            ok "Rust toolchain installed."
            return
        fi

        # Cargo exists, but make sure a default toolchain is set
        if command -v rustup &>/dev/null; then
            if ! rustup show active-toolchain &>/dev/null; then
                info "Configuring default Rust toolchain..."
                rustup default stable 2>&1 | tail -1
                ok "Rust default toolchain set to stable."
            fi
        fi

        ok "Rust toolchain ready."
    }

    ensure_rust

    # Verify cargo actually works
    if ! cargo --version &>/dev/null; then
        err "Cargo is not functional after setup. Cannot compile from source."
    fi

    # ─── Build MYTH ───
    section "SOURCE DEPLOYMENT (LEVEL 3)"
    info "Initiating neural core compilation..."

    if [ -f "Cargo.toml" ]; then
        # Building from local source directory
        info "Building from local source..."
        if ! cargo build --release --quiet 2>&1; then
            err "Source compilation failed. Check logs at $LOG_FILE"
        fi
    else
        # Clone and build
        require_command git
        info "Cloning tactical repository blueprint..."
        if ! git clone --depth 1 "${CLEAN_REPO_URL}.git" "$BUILD_DIR" 2>&1; then
            err "Failed to clone repository from ${CLEAN_REPO_URL}"
        fi
        cd "$BUILD_DIR"
        info "Compiling from source (this may take a few minutes)..."
        if ! cargo build --release --quiet 2>&1; then
            err "Source compilation failed. Check logs at $LOG_FILE"
        fi
    fi

    # ─── Verify & Install Binary ───
    if [ ! -f "target/release/myth" ]; then
        err "Compilation failed. Binary not found at target/release/myth."
    fi

    ok "Neural Core Compilation Complete."

    info "Finalizing binary placement..."
    sudo cp target/release/myth /usr/local/bin/myth
    sudo ln -sf /usr/local/bin/myth /usr/local/bin/agent
    sudo ln -sf /usr/local/bin/myth /usr/local/bin/chief
    ok "Tactical binaries deployed to /usr/local/bin/"
fi

# ─── Install Config ───
# Use REAL_HOME for config so it goes to the actual user, not root
ACTUAL_CONFIG_DIR="${REAL_HOME}/.config/myth"
USER_YAML="$ACTUAL_CONFIG_DIR/user.yaml"

if [ ! -f "$USER_YAML" ]; then
    mkdir -p "$ACTUAL_CONFIG_DIR"
    info "Creating default configuration at $USER_YAML"
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
        cp config/mcp.json "$ACTUAL_CONFIG_DIR/mcp.json"
    fi
    # Fix ownership if running as sudo
    if [ -n "${SUDO_USER:-}" ]; then
        chown -R "$SUDO_USER:$SUDO_USER" "$ACTUAL_CONFIG_DIR"
    fi
    ok "Configured at $ACTUAL_CONFIG_DIR/"
else
    audit "Existing profile detected at $USER_YAML"
fi

# ─── Operative Identification & API Key Setup ───
section "OPERATIVE INITIALIZATION"
if [ -t 0 ]; then
    # 1. User Name
    CURRENT_NAME=$(grep "user_name:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"' | tr -d "'" || echo "")
    if [ -z "$CURRENT_NAME" ] || [ "$CURRENT_NAME" = "Chief" ]; then
        echo -en "${CYAN}⠿  Enter your Operative Handle [Default: Chief]: ${NC}" >&3
        read OPERATIVE_NAME
        OPERATIVE_NAME=${OPERATIVE_NAME:-Chief}
        sed -i "s/user_name: .*/user_name: \"$OPERATIVE_NAME\"/" "$USER_YAML"
        ok "Operative handle synchronized: $OPERATIVE_NAME"
    fi

    # 2. NVIDIA API Key
    CURRENT_KEY=$(grep "nvidia_api_key:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"' | tr -d "'" || echo "")
    if [ -z "$CURRENT_KEY" ]; then
        echo -en "${CYAN}⠿  Enter your NVIDIA NIM API Key (Optional but recommended): ${NC}" >&3
        read -s API_KEY
        echo "" >&3
        if [ -n "$API_KEY" ]; then
            sed -i "s/nvidia_api_key: .*/nvidia_api_key: \"$API_KEY\"/" "$USER_YAML"
            ok "Neural Link API Key locked into configuration."
        else
            warn "No API Key provided. Neural reasoning will be disabled until set."
        fi
    fi
else
    info "Non-interactive environment. Skipping manual identification."
fi

# ─── Final Validation ───
section "FINAL SECURITY AUDIT"
if command -v bwrap &>/dev/null; then
    ok "Sandbox Engine: VALID ($(bwrap --version 2>/dev/null | head -n 1))"
else
    warn "Sandbox Engine: MISSING. Run: sudo apt install bubblewrap"
fi

if command -v myth &>/dev/null; then
    ok "MYTH Binary: VALID ($(myth --version 2>/dev/null | head -n 1 || echo 'installed'))"
else
    warn "MYTH Binary: NOT FOUND in PATH."
fi

# ─── Done ───
echo -e "\n${GREEN}${BOLD}  ✅ MYTH DEPLOYMENT COMPLETE!${NC}" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3
echo -e "  ${BOLD}Operative Archive:${NC} $USER_YAML" >&3
echo -e "  ${BOLD}Tactical Binary:${NC}   /usr/local/bin/myth" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3
echo "" >&3
echo -e "  ${BOLD}Next Objectives:${NC}" >&3
echo -e "    1. ${CYAN}myth sync${NC}        - Synchronize 3000+ Kali tools" >&3
echo -e "    2. ${CYAN}myth check${NC}       - Perform final system health check" >&3
echo -e "    3. ${CYAN}myth scan <target>${NC} - Launch your first mission" >&3
echo "" >&3
