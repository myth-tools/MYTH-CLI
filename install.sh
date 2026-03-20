#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH CLI — One-Line Installer for Kali Linux / Debian
# ═══════════════════════════════════════════════════════════
#
#  Usage:
#    curl -sSL $RAW_REPO_URL/scripts/install.sh | bash
#    OR
#    bash scripts/install.sh
#
set -euo pipefail

# ─── Cleanup Trap ───
CONFIG_DIR="$HOME/.config/myth"
BUILD_DIR="/tmp/myth-build-$(date +%s)"
cleanup() {
    if [ -d "$BUILD_DIR" ]; then
        info "Cleaning up build artifacts..."
        rm -rf "$BUILD_DIR"
    fi
}
trap cleanup EXIT

# ─── Dynamic Repository Configuration ───
# These placeholders are replaced by CI/CD during release
REPO_URL="https://github.com/myth-tools/MYTH-CLI"
PAGES_URL="https://myth.work.gd"

# Fallback for local execution (if placeholders were not replaced)
if [[ "$REPO_URL" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E 's/.*repository_url:[[:space:]]*["'\'']?([^"'\'']+)["'\'']?.*/\1/')
        PAGES_DOMAIN=$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')
        PAGES_URL="https://$PAGES_DOMAIN"
    else
        # Use a hardcoded default only if absolutely necessary as a last resort
        DEFAULT_REPO="https://github.com/myth-tools/MYTH-CLI"
        REPO_URL=$DEFAULT_REPO
        PAGES_DOMAIN=$(echo "$REPO_URL" | sed -E 's|https?://github.com/([^/]+)/([^/]+).*|\1.github.io/\2|')
        PAGES_URL="https://$PAGES_DOMAIN"
    fi
fi

# URL Normalization (Robustness)
CLEAN_REPO_URL=$(echo "$REPO_URL" | sed -E 's|/*$||' | sed -E 's|\.git$||')
RAW_REPO_URL="${CLEAN_REPO_URL/github.com/raw.githubusercontent.com}/main"


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
LOG_FILE="/tmp/myth-install-$(date +%s).log"
exec 3>&1 # Save stdout to fd 3
# Redirect all subsequent stdout/stderr to log file (except for our high-fidelity UI)
# Note: We will use '>&3' to print to the actual terminal.

cleanup() {
    local exit_code=$?
    if [ $exit_code -ne 0 ] && [ $exit_code -ne 130 ]; then
        echo -e "\n${RED}✘  [CRITICAL] Installation aborted unexpectedly.${NC}" >&3
        echo -e "${YELLOW}⠿  Technical logs preserved at: $LOG_FILE${NC}" >&3
    fi
    # Cleanup background processes
    jobs -p | xargs kill -9 2>/dev/null || true
    rm -rf "$BUILD_DIR" 2>/dev/null
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

# Progress spinner for long operations
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

# ─── Install System Dependencies ───
section "SYSTEM AUDIT & PRE-FLIGHT"
audit "OS: $(grep PRETTY_NAME /etc/os-release | cut -d'=' -f2 | tr -d '\"')"
audit "ARCH: $ARCH ($RUST_TARGET)"

if [ -d "$CONFIG_DIR" ]; then audit "Persistent profile detected at $CONFIG_DIR"; fi
if command -v bwrap &>/dev/null; then ok "Sandboxing engine (Bubblewrap) verified"; else warn "Sandboxing engine missing. Procedural isolation will be degraded."; fi
if command -v rustc &>/dev/null; then ok "Compiler detected: $(rustc --version | head -n 1)"; fi

section "DEPENDENCY RESOLUTION"
info "Synchronizing tactical dependencies..."
(sudo apt-get update -qq && \
 sudo apt-get install -y -qq \
    bubblewrap build-essential pkg-config libssl-dev \
    curl git protobuf-compiler 2>/dev/null) &
spinner $!
ok "Tactical dependencies synchronized."

# ─── Install Recommended Security Tools (for non-Kali) ───
if ! grep -qi "kali" /etc/os-release 2>/dev/null; then
    info "Non-Kali system detected. Checking for recommended security tools..."
    RECOMMENDED_TOOLS="nmap whois curl dnsutils"
    TO_INSTALL=""
    for tool in $RECOMMENDED_TOOLS; do
        if ! command -v "$tool" &>/dev/null; then
            # Special case for dnsutils (provides dig)
            if [ "$tool" == "dnsutils" ] && command -v dig &>/dev/null; then continue; fi
            TO_INSTALL="$TO_INSTALL $tool"
        fi
    done

    if [ -n "$TO_INSTALL" ]; then
        info "Installing missing recommended tools:$TO_INSTALL..."
        sudo apt-get install -y -qq $TO_INSTALL 2>/dev/null || warn "Failed to install some tools. You may need to install them manually."
        ok "Recommended tools installed"
    else
        ok "All recommended core tools already present"
    fi
fi

# ─── Install MYTH ───
section "PRIMARY DEPLOYMENT (APT)"
info "Attempting high-speed binary synchronization..."

# 1. Download Public Key if not present
if [ ! -f "/etc/apt/keyrings/myth.gpg" ]; then
    info "Retrieving public signing authority..."
    if ! wget -q --spider "${PAGES_URL}/myth.gpg"; then
        warn "Signing authority unavailable at gateway. Falling back to source compilation."
    else
        sudo mkdir -p /etc/apt/keyrings
        wget -qO- "${PAGES_URL}/myth.gpg" | sudo gpg --dearmor --yes -o /etc/apt/keyrings/myth.gpg
    fi
fi

if [ -f "/etc/apt/keyrings/myth.gpg" ]; then
    info "Configuring tactical source lists..."
    echo "deb [signed-by=/etc/apt/keyrings/myth.gpg] ${PAGES_URL} stable main" | sudo tee /etc/apt/sources.list.d/myth.list > /dev/null
    
    info "Mirror synchronization..."
    if sudo apt-get update -o Dir::Etc::sourcelist="sources.list.d/myth.list" -o Dir::Etc::sourceparts="-" -o APT::Get::List-Cleanup="0" -qq 2>/dev/null; then
        info "Core installation..."
        if sudo apt-get install -y -qq myth 2>/dev/null; then
            ok "MYTH Tactical AI successfully deployed via APT."
            exit 0
        fi
    fi
    warn "APT synchronization failed. Shifting to Source Level 3 Deployment..."
else
    warn "Gateway keys missing. Shifting to Source Level 3 Deployment..."
fi

    # ─── Install Rust (if not present) ───
    if ! command -v cargo &>/dev/null; then
        info "Installing Rust toolchain..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env" || true
        ok "Rust installed: $(rustc --version)"
    else
        ok "Rust already installed: $(rustc --version)"
    fi

    # ─── Build MYTH ───
    section "SOURCE DEPLOYMENT (LEVEL 3)"
    info "Initiating neural core compilation..."
    if [ -f "Cargo.toml" ]; then
        # Building from source directory
        cargo build --release --quiet &
        spinner $!
    else
        # Clone and build
        require_command git
        info "Cloning tactical repository blueprint..."
        git clone --depth 1 "${CLEAN_REPO_URL}.git" "$BUILD_DIR" &>/dev/null &
        spinner $!
        cd "$BUILD_DIR"
        cargo build --release --quiet &
        spinner $!
    fi
    ok "Neural Core Compilation Complete."

    # ─── Install Binary ───
    if [ ! -f "target/release/myth" ]; then
        err "Compilation failed. Critical core components missing."
    fi

    info "Finalizing binary placement..."
    sudo cp target/release/myth /usr/local/bin/myth
    sudo ln -sf /usr/local/bin/myth /usr/local/bin/agent
    sudo ln -sf /usr/local/bin/myth /usr/local/bin/chief
    ok "Tactical binaries deployed to /usr/local/bin/"

    # ─── Success ───
    section "MISSION SUCCESS"
    echo -e "${GREEN}${BOLD}  MYTH Tactical AI is now online.${NC}"
    echo -e "  Execute '${BOLD}myth --help${NC}' to begin reconnaissance.\n"
fi

# ─── Install Config ───
USER_YAML="$CONFIG_DIR/user.yaml"

if [ ! -f "$USER_YAML" ]; then
    mkdir -p "$CONFIG_DIR" 
    info "Creating default configuration at $USER_YAML"
    if [ -f "config/user.yaml" ]; then
        cp config/user.yaml "$USER_YAML"
    else
        # Critical fallback if template is missing
        cat <<EOF > "$USER_YAML"
agent:
  user_name: "Chief"
  nvidia_api_key: ""
EOF
    fi
    if [ -f "config/mcp.json" ]; then
        cp config/mcp.json "$CONFIG_DIR/mcp.json"
    fi
    ok "Configured at $CONFIG_DIR/"
else
    audit "Existing profile detected at $USER_YAML"
fi

# ─── Operative Identification & API Key Setup ───
section "OPERATIVE INITIALIZATION"
if [ -t 0 ]; then
    # 1. User Name
    CURRENT_NAME=$(grep "user_name:" "$USER_YAML" | awk '{print $2}' | tr -d '"' | tr -d "'")
    if [[ -z "$CURRENT_NAME" || "$CURRENT_NAME" == "Chief" ]]; then
        echo -en "${CYAN}⠿  Enter your Operative Handle [Default: Chief]: ${NC}" >&3
        read OPERATIVE_NAME
        OPERATIVE_NAME=${OPERATIVE_NAME:-Chief}
        sed -i "s/user_name: .*/user_name: \"$OPERATIVE_NAME\"/" "$USER_YAML"
        ok "Operative handle synchronized: $OPERATIVE_NAME"
    fi

    # 2. NVIDIA API Key
    CURRENT_KEY=$(grep "nvidia_api_key:" "$USER_YAML" | awk '{print $2}' | tr -d '"' | tr -d "'")
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
    ok "Sandbox Engine: VALID ($(bwrap --version | head -n 1))"
else
    warn "Sandbox Engine: MISSING. Tactical isolation is compromised."
    warn "Run: sudo apt install bubblewrap"
fi

# ─── Done ───
echo -e "\n${GREEN}${BOLD}  ✅ MYTH DEPLOYMENT COMPLETE!${NC}"
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo -e "  ${BOLD}Operative Archive:${NC} $USER_YAML"
echo -e "  ${BOLD}Tactical Binary:${NC}   /usr/local/bin/myth"
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo -e "  ${BOLD}Next Objectives:${NC}"
echo -e "    1. ${CYAN}myth sync${NC}        - Synchronize 3000+ Kali tools"
echo -e "    2. ${CYAN}myth check${NC}       - Perform final system health check"
echo -e "    3. ${CYAN}myth scan <target>${NC} - Launch your first mission"
echo ""
