#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH CLI — Universal Tactical Installer
#  Supports: Debian/Kali/Ubuntu/Pi, Fedora/RHEL/CentOS,
#            Arch/Manjaro, Termux (Android), and any Linux.
# ═══════════════════════════════════════════════════════════
#
#  Usage:
#    curl -sSL https://myth.work.gd/install.sh | sudo bash
#    OR
#    bash scripts/install.sh
#
set -euo pipefail

# ─── Early Constants ───
IS_TERMUX=false
REAL_USER="${SUDO_USER:-${USER:-root}}"
# Secure home directory resolution (avoiding eval echo)
REAL_HOME=$(getent passwd "$REAL_USER" | cut -d: -f6 || echo "${HOME:-/root}")
CONFIG_DIR="${REAL_HOME}/.config/myth"
TMP_DIR="${TMPDIR:-/tmp}"
BUILD_DIR="${TMP_DIR}/myth-build-$(date +%s)"

# ─── Dynamic Repository Configuration ───
# These placeholders are replaced by CI/CD during release
RELEASE_VERSION="__VERSION__"
REPO_URL="__REPO_URL__"
PAGES_URL="__PAGES_URL__"
AGENT_NAME="__AGENT_NAME__"

# Priority: 1. Environment Variable, 2. CI/CD Injected, 3. Local Discovery
VERSION="${VERSION:-$RELEASE_VERSION}"

# Fallback for local execution (if placeholders were not replaced)
if [[ "$VERSION" == "__"*"__" ]]; then
    if [ -f "config/agent.yaml" ]; then
        AGENT_NAME=$(grep "name:" config/agent.yaml | head -n 1 | sed -E "s/.*name:[[:space:]]*[\"'\":]*([^\"']+)[\"'\":]*.*/\1/" | awk '{print $1}')
        VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
        REPO_URL=$(grep "repository_url:" config/agent.yaml | head -n 1 | sed -E "s/.*repository_url:[[:space:]]*[\"'\"]?([^\"']+)[\"'\"]?.*/\1/")
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
LOG_FILE="${TMP_DIR}/myth-install-$(date +%s).log"
exec 3>&1

# ─── Tactical Installation HUD & Time Estimation (Premium Engine) ───
# Time constants (estimated durations in seconds)
TIME_OS_DETECTION=2
TIME_APT_SYNC=30
TIME_APT_DEPS=45
TIME_DNF_DEPS=50
TIME_PACMAN_DEPS=40
TIME_TERMUX_DEPS=40
TIME_GH_DOWNLOAD=25
TIME_SHA256_VERIFY=5
TIME_RUST_SETUP=60
TIME_CARGO_BUILD=900 # 15 min default for Neural Core
TIME_LP_SYNC=45
TIME_CONFIG_SETUP=10

# HUD Configuration (Elite v3.0)
HUD_ACTIVE_STEP=""
HUD_EST_SECONDS=0
HUD_TERM_SUPPORT=false
HUD_CPU_FACTOR=1.0
HUD_NET_FACTOR=1.0
HUD_TICKER_PID=""
# HUD_TICKER_PID=""

# Terminal Fidelity Discovery
if command -v tput &>/dev/null && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
    HUD_TERM_SUPPORT=true
fi

# Resource Calibration: CPU Core Scaling
calibrate_system() {
    local cores
    cores=$(nproc --all 2>/dev/null || echo 1)
    if [ "$cores" -ge 8 ]; then
        HUD_CPU_FACTOR=0.7 
    elif [ "$cores" -le 2 ]; then
        HUD_CPU_FACTOR=1.5 
    fi

    if command -v free &>/dev/null; then
        local mem_kb
        mem_kb=$(free -k | awk '/^Mem:/ {print $2}')
        if [ "${mem_kb:-0}" -lt 2097152 ]; then 
            HUD_CPU_FACTOR=$(echo "$HUD_CPU_FACTOR + 0.5" | awk '{print $1}')
        fi
    fi
}

# Network Calibration: RTT scaling
calibrate_network() {
    if ! command -v curl &>/dev/null; then return; fi
    local rtt
    rtt=$(curl -o /dev/null -sL -w "%{time_starttransfer}\n" --connect-timeout 2 "$PAGES_URL" 2>/dev/null | awk '{print int($1 * 1000)}')
    if [ "${rtt:-0}" -gt 400 ]; then
        HUD_NET_FACTOR=2.0 
    elif [ "$rtt" -gt 150 ]; then
        HUD_NET_FACTOR=1.3 
    fi
}

# Adaptive Time Multiplier (Ultra-Premium v2.0 Intelligence)
get_time_multiplier() {
    local arch
    arch=$(uname -m)
    local arch_factor=1.0
    case "$arch" in
        aarch64)  arch_factor=1.8 ;; 
        armv7*)   arch_factor=2.5 ;;
        *)        arch_factor=1.0 ;;
    esac
    echo "$arch_factor * $HUD_CPU_FACTOR * $HUD_NET_FACTOR" | awk '{print $1 * $2 * $3}'
}

# ─── Elite HUD v3.0: Concurrent Ticker & Throughput Engine ───

# Get the primary network interface (Robust Selection)
get_active_interface() {
    if [ "$IS_TERMUX" = true ]; then echo "wlan0"; return; fi
    local iface
    iface=$(ip route get 8.8.8.8 2>/dev/null | awk '{print $5}' | head -1)
    if [ -z "$iface" ]; then
        iface=$(route -n 2>/dev/null | awk '$1 == "0.0.0.0" {print $8}' | head -1)
    fi
    echo "${iface:-eth0}"
}

# Read RX bytes for bandwidth calculation
get_net_bytes() {
    local iface="$1"
    if [ -f "/sys/class/net/$iface/statistics/rx_bytes" ]; then
        cat "/sys/class/net/$iface/statistics/rx_bytes"
    elif [ -f "/proc/net/dev" ]; then
        grep "$iface" /proc/net/dev | awk '{print $2}'
    else
        echo 0
    fi
}

# Adaptive speed formatter (99.99% High-Precision Float)
format_speed() {
    local bytes="$1"
    if [ "$bytes" -ge 1048576 ]; then
        echo "$bytes" | awk '{printf "%.2f MB/s", $1 / 1048576}'
    elif [ "$bytes" -ge 1024 ]; then
        echo "$bytes" | awk '{printf "%.2f KB/s", $1 / 1024}'
    else
        echo "${bytes} B/s"
    fi
}

# Neural Pulse-Bar Renderer (v3.3)
render_bar() {
    local percent="$1"
    local bar_size=12
    local filled=$(( percent * bar_size / 100 ))
    [ "$filled" -gt "$bar_size" ] && filled=$bar_size
    local empty=$(( bar_size - filled ))
    printf "["
    for ((i=0; i<filled; i++)); do printf "■" ; done
    for ((i=0; i<empty; i++)); do printf "□" ; done
    printf "]"
}

# The Ticker v3.3: Cortex Visuals & Link Activity Heartbeat
hud_ticker() {
    local start_time="$1"
    local est_seconds="$2"
    local iface
    iface=$(get_active_interface)
    local prev_bytes
    prev_bytes=$(get_net_bytes "$iface")
    local prev_time
    prev_time=$(date +%s)
    local spinner_frames="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"
    local pulse_frames="⎓⎔⎓⎔"
    local spinner_idx=0
    local indent="    "

    # Synchronize drift: We use $(date +%s) inside instead of just sleep 1
    while true; do
        sleep 1
        local now
        now=$(date +%s)
        local elapsed=$((now - start_time))
        local remaining=$((est_seconds - elapsed))
        
        # Telemetry: Rate Calculation
        local current_bytes
        current_bytes=$(get_net_bytes "$iface")
        local byte_diff=$((current_bytes - prev_bytes))
        local time_diff=$((now - prev_time))
        [ "$time_diff" -le 0 ] && time_diff=1
        local bps=$((byte_diff / time_diff))
        local speed_str
        speed_str=$(format_speed "$bps")
        
        prev_bytes=$current_bytes
        prev_time=$now

        # Time formatting
        local h=$((remaining / 3600))
        local m=$(((remaining % 3600) / 60))
        local s=$((remaining % 60))
        [ "$remaining" -lt 0 ] && { h=0; m=0; s=0; } # Cap at 0 for primary calc
        local t_str
        t_str=$(printf "%dh %dm %ds" "$h" "$m" "$s")

        # Progress Calculation & Premium Overtime logic
        local pct=0
        if [ "$est_seconds" -gt 0 ]; then
            pct=$(( elapsed * 100 / est_seconds ))
        fi
        [ "$pct" -gt 100 ] && pct=100
        
        local bar
        bar=$(render_bar "$pct")
        local status_label="ETA"
        if [ "$remaining" -lt 0 ]; then
            status_label="OVERTIME"
            # Dynamic Scan-line Animation (moving signal pulse)
            local s_pos=$(( elapsed % 10 ))
            case "$s_pos" in
                0) bar="[•         ]" ;;
                1) bar="[ •        ]" ;;
                2) bar="[  •       ]" ;;
                3) bar="[   •      ]" ;;
                4) bar="[    •     ]" ;;
                5) bar="[     •    ]" ;;
                6) bar="[      •   ]" ;;
                7) bar="[       •  ]" ;;
                8) bar="[        • ]" ;;
                9) bar="[         •]" ;;
            esac
            t_str=$(printf "+%ds" "$(( -remaining ))")
        fi

        # Activity Pulse
        local pls="${pulse_frames:spinner_idx % 4 :1}"
        
        # Spinner Frame
        local frm="${spinner_frames:spinner_idx % 10 :1}"
        ((spinner_idx++))

        if [ "$HUD_TERM_SUPPORT" = true ]; then
            # Move cursor to Row 1 Spinner position, update spinner, then Row 2
            echo -ne "\033[1A\033[G \033[34m${frm}\033[0m\033[1B\r" >&3
            # Tactical Grid Presentation
            echo -ne "${indent}\033[0;90m${bar} \033[36m${pct}%\033[0;90m | LINK: \033[35m${speed_str}\033[0;90m ${pls} | ${status_label}: \033[37m${t_str}\033[0m\033[K\033[1A\r" >&3
        fi
    done
}

# Premium HUD: Start a mission step with an Elite v3.0 ticker
step_start() {
    local objective="$1"
    local base_est="$2"
    local multiplier
    multiplier=$(get_time_multiplier)
    
    HUD_EST_SECONDS=$(echo "$base_est * $multiplier" | awk '{print int($1 + 0.5)}')
    HUD_ACTIVE_STEP="$objective"

    if [ "$HUD_TERM_SUPPORT" = true ]; then
        # Terminal Width Truncation (Resilience Audit)
        local cols
        cols=$(tput cols 2>/dev/null || echo 80)
        local max_len=$((cols - 25))
        if [ "${#objective}" -gt "$max_len" ]; then
            objective="${objective:0:$((max_len-3))}..."
        fi
        # Display Row 1 with placeholder spinner (rotated by ticker)
        echo -e "   ${BOLD}${objective}${NC}" >&3
        
        # Spawn the Background Ticker (Elite v3.1 Precision)
        local start_t
        start_t=$(date +%s)
        hud_ticker "$start_t" "$HUD_EST_SECONDS" &
        HUD_TICKER_PID=$!
        disown "$HUD_TICKER_PID" 2>/dev/null || true
    else
        echo -e "${BLUE}⚡  [MISSION]${NC} ${BOLD}${objective}${NC}" >&3
    fi
}

# Premium HUD: Banish the Ticker and mark success
step_done() {
    local message="${1:-$HUD_ACTIVE_STEP complete.}"
    if [ "$HUD_TERM_SUPPORT" = true ]; then
        # 1. Kill the background ticker immediately
        if [ -n "$HUD_TICKER_PID" ]; then kill -9 "$HUD_TICKER_PID" 2>/dev/null || true; fi
        # 2. CLEAR the ticker line (Row 2): Move down, clear, move back up
        echo -ne "\n\033[K\033[1A" >&3
        # 3. OVERWRITE Row 1 with success check
        echo -e "\033[K\033[1A\r ${GREEN}[ OK ]${NC} ${BOLD}${message}${NC}" >&3
    else
        echo -e " ${GREEN}[ OK ]${NC} ${BOLD}${message}${NC}" >&3
    fi
    HUD_ACTIVE_STEP=""
    HUD_EST_SECONDS=0
    HUD_TICKER_PID=""
}

# Premium HUD: If a step fails, clear current HUD line and report error
step_fail() {
    local err_msg="${1:-$HUD_ACTIVE_STEP failed.}"
    if [ "$HUD_TERM_SUPPORT" = true ]; then
        if [ -n "$HUD_TICKER_PID" ]; then kill -9 "$HUD_TICKER_PID" 2>/dev/null || true; fi
        echo -ne "\n\033[K\033[1A" >&3
        echo -e "\033[K\033[1A\r ${RED}[ FAIL ]${NC} ${err_msg}" >&3
    else
        echo -e " ${RED}[ FAIL ]${NC} ${err_msg}" >&3
    fi
    HUD_ACTIVE_STEP=""
    exit 1
}

cleanup() {
    local exit_code=$?
    # Ensure any background tickers are reaped on exit
    if [ -n "$HUD_TICKER_PID" ]; then kill -9 "$HUD_TICKER_PID" 2>/dev/null || true; fi
    if [ -n "$HUD_ACTIVE_STEP" ]; then
        echo -e "\033[0m" >&3
    fi
    if [ $exit_code -ne 0 ] && [ $exit_code -ne 130 ]; then
        echo -e "\n${RED}✘  [CRITICAL] Installation aborted unexpectedly.${NC}" >&3
        echo -e "${YELLOW}⠿  Technical logs preserved at: $LOG_FILE${NC}" >&3
    fi
    jobs -p 2>/dev/null | xargs kill -9 2>/dev/null || true
    rm -rf "$BUILD_DIR" 2>/dev/null || true
    exit $exit_code
}
trap cleanup EXIT INT TERM

# High-fidelity status indicators (Industry Grade)
info()    { echo -e " ${BLUE}[ INFO ]${NC} ${BOLD}$1${NC}" >&3; }
ok()      { echo -e " ${GREEN}[ OK ]${NC} $1" >&3; }
warn()    { echo -e " ${YELLOW}[ WARN ]${NC} $1" >&3; }
err()     { echo -e " ${RED}[ FAIL ]${NC} $1" >&3; exit 1; }
audit()   { echo -e " ${CYAN}[ AUDIT ]${NC} $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }

require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "$1 is required but not installed."
    fi
}

# ─── Flag Parsing ───
FORCE_INSTALL=false
INSTALL_DEPS=true
INSTALL_BROWSER=true

# shellcheck disable=SC2034
for arg in "$@"; do
    case "$arg" in
        --version) echo "MYTH Installer v$VERSION"; exit 0 ;;
        --force) FORCE_INSTALL=true ;;
        --no-deps) INSTALL_DEPS=false ;;
        --no-browser) INSTALL_BROWSER=false ;;
        --help)
            echo "Usage: ./install.sh [options]"
            echo "Options:"
            echo "  --version      Show version"
            echo "  --force        Force re-installation"
            echo "  --no-deps      Skip dependency installation"
            echo "  --no-browser   Skip Lightpanda provisioning"
            exit 0
            ;;
    esac
done

# ─── Elite HUD v3.0 Internal Audit State ───
AUDIT_OS=""
AUDIT_KERNEL=""
AUDIT_CPU=""
AUDIT_RAM=""
AUDIT_DISK=""
AUDIT_CONTEXT="Bare Metal"

# Deep Environment Scanning (v3.0 Intelligence)
detect_virtualization() {
    if [ "$IS_TERMUX" = true ]; then AUDIT_CONTEXT="Termux"; return; fi
    if [ -f /.dockerenv ]; then AUDIT_CONTEXT="Docker Container"; return; fi
    if grep -qi "microsoft" /proc/version 2>/dev/null; then
        if grep -qi "WSL2" /proc/version 2>/dev/null; then AUDIT_CONTEXT="Microsoft WSL2"; else AUDIT_CONTEXT="Microsoft WSL1"; fi
        return
    fi
    local hv
    hv=$(cat /sys/class/dmi/id/product_name 2>/dev/null || echo "Unknown")
    case "$hv" in
        "VirtualBox") AUDIT_CONTEXT="VirtualBox VM" ;;
        "VMware"*)    AUDIT_CONTEXT="VMware VM" ;;
        "KVM"*)       AUDIT_CONTEXT="KVM Hypervisor" ;;
        "Unknown")    AUDIT_CONTEXT="Linux Physical/HVM" ;;
        *)            AUDIT_CONTEXT="$hv" ;;
    esac
}

gather_hardware_specs() {
    # CPU Model & Core Count
    local model
    model=$(lscpu | grep "Model name" | cut -d: -f2 | xargs || grep "model name" /proc/cpuinfo | head -1 | cut -d: -f2 | xargs || echo "Generic CPU")
    local cores
    cores=$(nproc --all 2>/dev/null || echo 1)
    AUDIT_CPU="${model} (${cores} Cores)"
    # RAM Analysis (Total vs Free)
    if command -v free &>/dev/null; then
        local total_ram
        total_ram=$(free -h | awk '/^Mem:/ {print $2}')
        local free_ram
        free_ram=$(free -h | awk '/^Mem:/ {print $4}')
        AUDIT_RAM="${total_ram} Total (${free_ram} Free)"
    else
        AUDIT_RAM="Unknown"
    fi

    # Disk Analysis
    AUDIT_DISK="$(df -h / | tail -1 | awk '{print $4}') Headroom"
    AUDIT_KERNEL="$(uname -r) ($(uname -m))"
}

# ─── Start HUD v3.0 Calibration ───
calibrate_system
calibrate_network

# ─── Start ───
echo -e "${MAGENTA}${BOLD}${BANNER}${NC}" >&3
echo -e "${CYAN}  [ ${AGENT_NAME} — DIGITAL RECONNAISSANCE & TACTICAL AI ]${NC}" >&3
echo -e "  ${BOLD}Version: ${VERSION}${NC}\n" >&3
info "Logs: $LOG_FILE"

section "TACTICAL SYSTEM AUDIT"

# Initialization HUD: Scanning Phase
step_start "Scanning System Profile" "$TIME_OS_DETECTION"
detect_virtualization
gather_hardware_specs

# Robust Distro Identification
if [ -n "${PREFIX:-}" ] && echo "${PREFIX}" | grep -q "com.termux"; then
    DISTRO_FAMILY="termux"
    PKG_MANAGER="pkg"
    IS_TERMUX=true
    AUDIT_OS="Termux (Android)"
    BIN_DIR="$PREFIX/bin"
    APT_SOURCES_DIR="$PREFIX/etc/apt/sources.list.d"
    APT_KEYRINGS_DIR="$PREFIX/etc/apt/trusted.gpg.d"
elif [ -f /etc/os-release ]; then
    # shellcheck source=/dev/null
    . /etc/os-release
    AUDIT_OS="${PRETTY_NAME:-$ID}"
    case "${ID:-}" in
        kali|debian|ubuntu|linuxmint|pop|raspbian|parrot)
            DISTRO_FAMILY="debian"
            PKG_MANAGER="apt"
            ;;
        fedora|rhel|centos|rocky|alma|ol|amzn)
            DISTRO_FAMILY="fedora"
            if command -v dnf &>/dev/null; then PKG_MANAGER="dnf"; elif command -v yum &>/dev/null; then PKG_MANAGER="yum"; fi
            ;;
        arch|manjaro|endeavouros|garuda|artix)
            DISTRO_FAMILY="arch"
            PKG_MANAGER="pacman"
            ;;
        alpine)
            DISTRO_FAMILY="alpine"
            PKG_MANAGER="apk"
            ;;
        opensuse*|sles)
            DISTRO_FAMILY="fedora"
            PKG_MANAGER="zypper"
            ;;
        *)
            if command -v apt &>/dev/null; then
                DISTRO_FAMILY="debian"
                PKG_MANAGER="apt"
            elif command -v dnf &>/dev/null; then
                DISTRO_FAMILY="fedora"
                PKG_MANAGER="dnf"
            elif command -v pacman &>/dev/null; then
                DISTRO_FAMILY="arch"
                PKG_MANAGER="pacman"
            fi
            ;;
    esac
    if [ "$IS_TERMUX" = true ]; then
        ROOT_DIR="$PREFIX"
        BIN_DIR="$PREFIX/bin"
    else
        ROOT_DIR=""
        BIN_DIR="/usr/bin"
        [ -d "/usr/local/bin" ] && BIN_DIR="/usr/local/bin"
    fi
    APT_SOURCES_DIR="${ROOT_DIR}/etc/apt/sources.list.d"
    APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/keyrings"
    [ ! -d "$APT_KEYRINGS_DIR" ] && APT_KEYRINGS_DIR="${ROOT_DIR}/etc/apt/trusted.gpg.d"
    YUM_REPOS_DIR="${ROOT_DIR}/etc/yum.repos.d"
    PACMAN_CONF="${ROOT_DIR}/etc/pacman.conf"
else
    if command -v apt &>/dev/null; then
        DISTRO_FAMILY="debian"
        PKG_MANAGER="apt"
    elif command -v dnf &>/dev/null; then
        DISTRO_FAMILY="fedora"
        PKG_MANAGER="dnf"
    elif command -v pacman &>/dev/null; then
        DISTRO_FAMILY="arch"
        PKG_MANAGER="pacman"
    elif command -v apk &>/dev/null; then
        DISTRO_FAMILY="alpine"
        PKG_MANAGER="apk"
    fi
fi

# Final Audit Disclosure Presentation (Premium v3.0)
step_done "Profile scanning complete."

echo -e "\n${CYAN}⠿ ${BOLD}DISTRO:${NC}  $AUDIT_OS [${PKG_MANAGER}]" >&3
echo -e "${CYAN}⠿ ${BOLD}KERNEL:${NC}  $AUDIT_KERNEL" >&3
echo -e "${CYAN}⠿ ${BOLD}CPU:${NC}     $AUDIT_CPU" >&3
echo -e "${CYAN}⠿ ${BOLD}RAM:${NC}     $AUDIT_RAM" >&3
echo -e "${CYAN}⠿ ${BOLD}DISK:${NC}    $AUDIT_DISK" >&3
echo -e "${CYAN}⠿ ${BOLD}CONTEXT:${NC} $AUDIT_CONTEXT" >&3

# Logic for root requirement (Premium validation)
if [ "$DISTRO_FAMILY" != "termux" ] && [ "$EUID" -ne 0 ]; then
    warn "This distro profile typically requires root. Attempting to use sudo..."
fi

[ -z "${BIN_DIR:-}" ] && BIN_DIR="/usr/local/bin"
if [ "$(id -u)" -eq 0 ]; then
    IS_ROOT=true
fi
ARCH=$(uname -m)
case "$ARCH" in
    x86_64)  RUST_TARGET="x86_64-unknown-linux-gnu" ;;
    aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    armv7l)  RUST_TARGET="armv7-unknown-linux-gnueabihf" ;;
    *)       RUST_TARGET="" ;;
esac
step_done "Environment Detection complete: ${PRETTY_NAME:-$ID} ($ARCH)"

# ─── System Audit ───
section "SYSTEM AUDIT & PRE-FLIGHT"
audit "OS Family: $DISTRO_FAMILY"
audit "Package Manager: $PKG_MANAGER"
audit "ARCH: $ARCH ($RUST_TARGET)"
if [ -d "$CONFIG_DIR" ]; then audit "Existing profile at $CONFIG_DIR"; fi
if [ "$IS_TERMUX" = true ]; then audit "Termux environment: $PREFIX"; fi

# ═══════════════════════════════════════════════════════════
#  DEPENDENCY RESOLUTION (OS-Aware)
# ═══════════════════════════════════════════════════════════
section "DEPENDENCY RESOLUTION"

install_deps_debian() {
    if [ -c /dev/tty ]; then
        echo -en "${CYAN}⠿  Install Tactical Build Tools? [Y/n]: ${NC}" >&3
        read -r INSTALL_TOOLS < /dev/tty || INSTALL_TOOLS="y"
    else
        INSTALL_TOOLS="y"
    fi

    if [[ "$INSTALL_TOOLS" =~ ^[Yy]$ ]] || [[ -z "$INSTALL_TOOLS" ]]; then
        step_start "Synchronizing Mirror Index" "$TIME_APT_SYNC"
        apt-get update -qq 2>&1 | tee -a "$LOG_FILE" | tail -1 >&3 || true
        step_done "Mirror index synchronized."

        step_start "Deploying Tactical Dependencies" "$TIME_APT_DEPS"
        apt-get install -y -qq build-essential pkg-config libssl-dev ca-certificates curl git wget unzip fontconfig 2>&1 | tee -a "$LOG_FILE" | tail -1 >&3 || warn "Some tools failed."
        step_done "Tactical dependencies synchronized."
    fi

    # Anonymity Layer
    if [ -c /dev/tty ]; then
        echo -en "${CYAN}⠿  Install Anonymity Infrastructure (Tor Engine)? [Y/n]: ${NC}" >&3
        read -r INSTALL_TOR < /dev/tty || INSTALL_TOR="y"
    else
        INSTALL_TOR="n"
    fi
    if [[ "$INSTALL_TOR" =~ ^[Yy]$ ]]; then
        apt-get install -y -qq tor 2>&1 | tee -a "$LOG_FILE" | tail -1 >&3 || warn "Tor installation failed."
        systemctl enable tor 2>/dev/null || true
        systemctl start tor 2>/dev/null || true
        ok "Tor Engine deployed."
    fi

    # Recommended recon tools for non-Kali
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
            info "Installing recon dependencies:$TO_INSTALL..."
            apt-get install -y -qq "$TO_INSTALL" 2>/dev/null || warn "Some tools failed."
            ok "Recon tools installed."
        fi
    fi
}

install_deps_fedora() {
    step_start "Synchronizing Fedora Environment" "$TIME_DNF_DEPS"
    $PKG_MANAGER install -y -q curl git wget nmap whois openssl ca-certificates bubblewrap zstd bind-utils 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3 || warn "Some packages failed."
    step_done "Fedora environment locked."

    if [ -c /dev/tty ]; then
        echo -en "${CYAN}⠿  Install Anonymity Infrastructure (Tor Engine)? [y/N]: ${NC}" >&3
        read -r INSTALL_TOR < /dev/tty || INSTALL_TOR="n"
    else
        INSTALL_TOR="n"
    fi
    if [[ "$INSTALL_TOR" =~ ^[Yy]$ ]]; then
        $PKG_MANAGER install -y -q tor 2>/dev/null || warn "Tor installation failed."
        systemctl enable tor 2>/dev/null || true
        systemctl start tor 2>/dev/null || true
        ok "Tor Engine deployed."
    fi
}

install_deps_arch() {
    step_start "Synchronizing Arch Environment" "$TIME_PACMAN_DEPS"
    pacman -Sy --noconfirm --needed curl git wget nmap whois openssl ca-certificates bubblewrap zstd bind-tools 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3 || warn "Some packages failed."
    step_done "Arch environment locked."

    if [ -c /dev/tty ]; then
        echo -en "${CYAN}⠿  Install Anonymity Infrastructure (Tor Engine)? [y/N]: ${NC}" >&3
        read -r INSTALL_TOR < /dev/tty || INSTALL_TOR="n"
    else
        INSTALL_TOR="n"
    fi
    if [[ "$INSTALL_TOR" =~ ^[Yy]$ ]]; then
        pacman -S --noconfirm tor 2>/dev/null || warn "Tor installation failed."
        systemctl enable tor 2>/dev/null || true
        systemctl start tor 2>/dev/null || true
        ok "Tor Engine deployed."
    fi
}

install_deps_termux() {
    step_start "Synchronizing Termux Environment" "$TIME_TERMUX_DEPS"
    pkg update -y 2>&1 | tail -3 >&3 || true
    pkg install -y curl git wget nmap openssl ca-certificates zstd 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3 || warn "Some packages failed."
    step_done "Termux environment locked."
}

install_deps_alpine() {
    step_start "Synchronizing Alpine Environment" "$TIME_PACMAN_DEPS"
    apk add --no-cache curl git wget nmap openssl ca-certificates bubblewrap zstd 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3 || warn "Some packages failed."
    step_done "Alpine environment locked."
}

# Execute OS-specific dependency installation
if [ "$INSTALL_DEPS" = true ]; then
    case "$DISTRO_FAMILY" in
        debian)  install_deps_debian ;;
        fedora)  install_deps_fedora ;;
        arch)    install_deps_arch ;;
        termux)  install_deps_termux ;;
        alpine)  install_deps_alpine ;;
        *)       warn "Unknown OS family. Skipping dependency installation." ;;
    esac
else
    info "Dependency installation skipped (--no-deps)."
fi

# ═══════════════════════════════════════════════════════════
#  PATH A: NATIVE PACKAGE MANAGER INSTALLATION (Preferred)
#  Each distro gets its own native repo integration so
#  future updates work via: apt upgrade / dnf upgrade / pacman -Syu / pkg upgrade
# ═══════════════════════════════════════════════════════════
section "PRIMARY DEPLOYMENT (NATIVE PACKAGE MANAGER)"
PKG_SUCCESS=false

# ─────────────────────────────────────────
#  A1: DEBIAN / KALI / UBUNTU / PI (APT)
# ─────────────────────────────────────────
install_via_apt() {
    info "Attempting deployment via APT..."

    # 1. Refresh GPG key
    step_start "Retrieving signing authority" "$TIME_CONFIG_SETUP"
    mkdir -p "$APT_KEYRINGS_DIR"
    rm -f "$APT_KEYRINGS_DIR/myth.gpg"

    if curl -fsSL "${PAGES_URL}/myth.gpg" -o "${TMP_DIR}/myth-key-download.gpg" 2>/dev/null; then
        if [ -s "${TMP_DIR}/myth-key-download.gpg" ]; then
            if head -c 20 "${TMP_DIR}/myth-key-download.gpg" | grep -q "BEGIN"; then
                gpg --dearmor --yes -o "$APT_KEYRINGS_DIR/myth.gpg" "${TMP_DIR}/myth-key-download.gpg" 2>/dev/null
            else
                cp "${TMP_DIR}/myth-key-download.gpg" "$APT_KEYRINGS_DIR/myth.gpg"
            fi
            rm -f "${TMP_DIR}/myth-key-download.gpg"

            if [ -s "$APT_KEYRINGS_DIR/myth.gpg" ]; then
                step_done "Signing authority installed."
            else
                warn "GPG key conversion failed."
                rm -f "$APT_KEYRINGS_DIR/myth.gpg"
                return 1
            fi
        else
            warn "Downloaded key is empty."
            rm -f "${TMP_DIR}/myth-key-download.gpg"
            return 1
        fi
    else
        warn "Could not download signing key from ${PAGES_URL}/myth.gpg"
        return 1
    fi

    # 2. Configure APT source & install
    if [ -f "$APT_KEYRINGS_DIR/myth.gpg" ] && [ -s "$APT_KEYRINGS_DIR/myth.gpg" ]; then
        step_start "Configuring APT Tactical Mirror" "$TIME_CONFIG_SETUP"
        echo "deb [signed-by=$APT_KEYRINGS_DIR/myth.gpg] ${PAGES_URL} stable main" | tee "$APT_SOURCES_DIR/myth.list" > /dev/null
        step_done "Mirror configured successfully."

        step_start "Synchronizing Repository index" "$TIME_APT_SYNC"
        if apt-get update -o Dir::Etc::sourcelist="$APT_SOURCES_DIR/myth.list" \
            -o Dir::Etc::sourceparts="-" \
            -o APT::Get::List-Cleanup="0" -qq 2>&1 | tee -a "$LOG_FILE"; then
            step_done "Index synchronized."

            INSTALL_TARGET="myth"
            if [ "$VERSION" != "__VERSION__" ]; then
                INSTALL_TARGET="myth=$VERSION"
                info "Targeting specific version: $VERSION"
            fi

            step_start "Deploying MYTH via APT" "$TIME_APT_DEPS"
            if apt-get install -y "$INSTALL_TARGET" 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3; then
                PKG_SUCCESS=true
                step_done "MYTH deployed via APT."
                ok "Future updates: ${BOLD}sudo apt update && sudo apt upgrade myth${NC}"
                return 0
            fi
            step_fail "APT deployment failed."
        fi
        rm -f "$APT_SOURCES_DIR/myth.list"
    fi
    return 1
}

# ─────────────────────────────────────────
#  A2: TERMUX (Android — pkg/apt)
# ─────────────────────────────────────────
install_via_termux() {
    info "Attempting deployment via Termux pkg..."

    # Termux uses apt internally but with $PREFIX paths
    TERMUX_SOURCES_DIR="$PREFIX/etc/apt/sources.list.d"
    TERMUX_KEYRINGS_DIR="$PREFIX/etc/apt/trusted.gpg.d"
    mkdir -p "$TERMUX_SOURCES_DIR" "$TERMUX_KEYRINGS_DIR"

    # 1. Import GPG key
    step_start "Retrieving signing authority for Termux" "$TIME_CONFIG_SETUP"
    if curl -fsSL "${PAGES_URL}/myth.gpg" -o "${TMP_DIR}/myth-key.gpg" 2>/dev/null && [ -s "${TMP_DIR}/myth-key.gpg" ]; then
        if head -c 20 "${TMP_DIR}/myth-key.gpg" | grep -q "BEGIN"; then
            gpg --dearmor --yes -o "$TERMUX_KEYRINGS_DIR/myth.gpg" "${TMP_DIR}/myth-key.gpg" 2>/dev/null
        else
            cp "${TMP_DIR}/myth-key.gpg" "$TERMUX_KEYRINGS_DIR/myth.gpg"
        fi
        rm -f "${TMP_DIR}/myth-key.gpg"

        if [ -s "$TERMUX_KEYRINGS_DIR/myth.gpg" ]; then
            step_done "Signing authority installed for Termux."
        else
            step_fail "GPG key setup failed."
        fi
    else
        step_fail "Could not download signing key."
    fi

    # 2. Add Termux-compatible APT source
    step_start "Configuring Termux package source" "$TIME_CONFIG_SETUP"
    echo "deb [signed-by=${TERMUX_KEYRINGS_DIR}/myth.gpg] ${PAGES_URL} stable main" > "$TERMUX_SOURCES_DIR/myth.list"
    step_done "Termux source configured."

    # 3. Update and install
    step_start "Synchronizing Termux package index" "$TIME_TERMUX_DEPS"
    if apt-get update -o Dir::Etc::sourcelist="$TERMUX_SOURCES_DIR/myth.list" \
        -o Dir::Etc::sourceparts="-" \
        -o APT::Get::List-Cleanup="0" -qq 2>&1 | tee -a "$LOG_FILE"; then
        step_done "Termux index synchronized."

        step_start "Deploying MYTH via Termux pkg" "$TIME_TERMUX_DEPS"
        if apt-get install -y myth 2>&1 | tee -a "$LOG_FILE" | tail -3 >&3; then
            PKG_SUCCESS=true
            step_done "MYTH deployed via Termux pkg."
            ok "Future updates: ${BOLD}pkg upgrade myth${NC}"
            return 0
        fi
    fi

    # If APT repo didn't work (arch mismatch, etc.), fall back to direct binary
    warn "Termux APT install failed. Falling back to direct binary deployment..."
    rm -f "$TERMUX_SOURCES_DIR/myth.list"
    return 1
}

# ─────────────────────────────────────────
#  A3: FEDORA / RHEL / CentOS (DNF/YUM)
# ─────────────────────────────────────────
install_via_dnf() {
    info "Attempting deployment via $PKG_MANAGER..."

    # 1. Import GPG key
    step_start "Importing MYTH signing key" "$TIME_CONFIG_SETUP"
    if curl -fsSL "${PAGES_URL}/myth.gpg" -o "${TMP_DIR}/myth-rpm-key.gpg" 2>/dev/null && [ -s "${TMP_DIR}/myth-rpm-key.gpg" ]; then
        rpm --import "${TMP_DIR}/myth-rpm-key.gpg" 2>/dev/null || warn "GPG key import may have failed (non-fatal)."
        rm -f "${TMP_DIR}/myth-rpm-key.gpg"
        step_done "Signing key imported."
    else
        step_done "Signining key download failed (skipped)."
    fi

    # 2. Add RPM repository
    step_start "Configuring MYTH RPM repository" "$TIME_CONFIG_SETUP"
    cat > "$YUM_REPOS_DIR/myth.repo" << REPOEOF
[myth]
name=MYTH Official Repository
baseurl=${PAGES_URL}/rpm
enabled=1
gpgcheck=0
repo_gpgcheck=0
type=rpm
REPOEOF
    step_done "RPM repository configured."

    # 3. Install
    step_start "Deploying MYTH via $PKG_MANAGER" "$TIME_DNF_DEPS"
    if $PKG_MANAGER install -y myth 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
        PKG_SUCCESS=true
        step_done "MYTH deployed via $PKG_MANAGER."
        ok "Future updates: ${BOLD}sudo $PKG_MANAGER upgrade myth${NC}"
        return 0
    fi
    rm -f "$YUM_REPOS_DIR/myth.repo"
    step_fail "$PKG_MANAGER deployment failed."
}

# ─────────────────────────────────────────
#  A4: ARCH / MANJARO (Pacman + AUR)
# ─────────────────────────────────────────
install_via_pacman() {
    info "Attempting deployment via pacman/AUR..."

    # Strategy: Try AUR helpers first (most Arch users have one),
    # then fall back to custom repo, then binary.
    AUR_SUCCESS=false

    # Try yay first
    if command -v yay &>/dev/null; then
        info "Found yay. Installing myth-bin from AUR..."
        # Run as non-root user (yay refuses to run as root)
        if [ "$IS_ROOT" = true ] && [ -n "${SUDO_USER:-}" ]; then
            if sudo -u "$SUDO_USER" yay -S --noconfirm myth-bin 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
                AUR_SUCCESS=true
            fi
        else
            if yay -S --noconfirm myth-bin 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
                AUR_SUCCESS=true
            fi
        fi
    fi

    # Try paru if yay failed
    if [ "$AUR_SUCCESS" = false ] && command -v paru &>/dev/null; then
        info "Found paru. Installing myth-bin from AUR..."
        if [ "$IS_ROOT" = true ] && [ -n "${SUDO_USER:-}" ]; then
            if sudo -u "$SUDO_USER" paru -S --noconfirm myth-bin 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
                AUR_SUCCESS=true
            fi
        else
            if paru -S --noconfirm myth-bin 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
                AUR_SUCCESS=true
            fi
        fi
    fi

    if [ "$AUR_SUCCESS" = true ]; then
        PKG_SUCCESS=true
        ok "MYTH deployed via AUR."
        ok "Future updates: ${BOLD}yay -Syu${NC} or ${BOLD}paru -Syu${NC}"
        return 0
    fi

    # Fallback: Add custom Arch repo to pacman.conf
    info "No AUR helper found. Adding custom MYTH repository..."
    if ! grep -q "\[myth\]" "$PACMAN_CONF" 2>/dev/null; then
        cat >> "$PACMAN_CONF" << ARCHEOF

[myth]
SigLevel = Optional TrustAll
Server = ${PAGES_URL}/arch
ARCHEOF
        ok "Custom repo added to $PACMAN_CONF"
    fi

    step_start "Deploying MYTH via pacman" "$TIME_PACMAN_DEPS"
    if pacman -Sy --noconfirm myth 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
        PKG_SUCCESS=true
        step_done "MYTH deployed via pacman (custom repo)."
        ok "Future updates: ${BOLD}sudo pacman -Syu${NC}"
        return 0
    fi
    # Remove custom repo entry if install failed
    sed -i '/\[myth\]/,+2d' "$PACMAN_CONF" 2>/dev/null || true
    step_fail "Pacman deployment failed."
}

# ─────────────────────────────────────────
#  A5: ALPINE LINUX (APK)
# ─────────────────────────────────────────
install_via_apk() {
    info "Attempting deployment via apk..."
    # Alpine usually requires manual binary placement or a custom repo
    # For now, we point them to the direct binary if our repo isn't ready
    return 1
}

# ─────────────────────────────────────────
#  A6: OPENSUSE (ZYPPER)
# ─────────────────────────────────────────
install_via_zypper() {
    info "Attempting deployment via zypper..."
    # Zypper uses RPM but needs different repo management
    return 1
}

# ─── Execute the right installer for this OS ───
case "$DISTRO_FAMILY" in
    debian)  install_via_apt || true ;;
    termux)  install_via_termux || true ;;
    fedora)  install_via_dnf || true ;;
    arch)    install_via_pacman || true ;;
    alpine)  install_via_apk || true ;;
    *)       info "No native package manager integration for $DISTRO_FAMILY. Using binary deployment." ;;
esac

# ═══════════════════════════════════════════════════════════
#  PATH B: Pre-Built Binary (Fast Fallback — GitHub Releases)
#  Critical for Nethunter/Termux/Pi where compiling a large
#  Rust project from source takes 45+ min and may OOM.
# ═══════════════════════════════════════════════════════════
BINARY_SUCCESS=false

if [ "$PKG_SUCCESS" = false ]; then
    section "BINARY DEPLOYMENT (LEVEL 2 — GITHUB RELEASES)"
    info "Attempting direct binary download from GitHub Releases..."

    # Map system arch to GitHub release asset names
    case "$ARCH" in
        x86_64)  GH_BINARY_NAME="myth-x86_64-unknown-linux-gnu" ;;
        aarch64) GH_BINARY_NAME="myth-aarch64-unknown-linux-gnu" ;;
        armv7l)  GH_BINARY_NAME="myth-armv7-unknown-linux-gnueabihf" ;;
        i386|i686) GH_BINARY_NAME="myth-i686-unknown-linux-gnu" ;;
        *)       GH_BINARY_NAME="" ;;
    esac

    if [ -z "$GH_BINARY_NAME" ]; then
        warn "No pre-built binary available for $ARCH. Will compile from source."
    else
        # Try to fetch the latest release tag from GitHub API
        LATEST_TAG=""
        if command -v curl &>/dev/null; then
            LATEST_TAG=$(curl -fsSL \
                "https://api.github.com/repos/myth-tools/MYTH-CLI/releases/latest" \
                2>/dev/null | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')
        fi

        if [ -z "$LATEST_TAG" ]; then
            warn "Could not query latest GitHub Release tag. Will compile from source."
        else
            info "Latest release tag: $LATEST_TAG"
            GH_BINARY_URL="https://github.com/myth-tools/MYTH-CLI/releases/download/${LATEST_TAG}/${GH_BINARY_NAME}"
            
            TEMP_BINARY="${TMP_DIR}/myth-binary-$$"
            step_start "Downloading Tactical Binary ($GH_BINARY_NAME)" "$TIME_GH_DOWNLOAD"

            DOWNLOAD_SUCCESS=false
            for attempt in 1 2 3; do
                if curl -fsSL --progress-bar "$GH_BINARY_URL" -o "$TEMP_BINARY" 2>&1; then
                    DOWNLOAD_SUCCESS=true
                    break
                else
                    warn "Download attempt $attempt/3 failed. Retrying..."
                    sleep 3
                fi
            done

            if [ "$DOWNLOAD_SUCCESS" = true ] && [ -s "$TEMP_BINARY" ]; then
                step_done "Binary synchronized."
                # ─── Verification Phase ───
                step_start "Verifying SHA256 Integrity" "$TIME_SHA256_VERIFY"
                
                # Download manifest for verification
                TEMP_SUMS="${TMP_DIR}/myth-sums-$$"
                if curl -fsSL "${PAGES_URL}/SHA256SUMS" -o "$TEMP_SUMS" 2>/dev/null; then
                    # Check if our binary is in the manifest
                    if grep -q "$GH_BINARY_NAME" "$TEMP_SUMS"; then
                        # Extracts the expected hash from the manifest
                        EXPECTED_SHA=$(grep "$GH_BINARY_NAME" "$TEMP_SUMS" | awk '{print $1}')
                        ACTUAL_SHA=$(sha256sum "$TEMP_BINARY" | awk '{print $1}')
                        
                        if [ "$EXPECTED_SHA" != "$ACTUAL_SHA" ]; then
                            err "SHA256 Mismatch! Expected: $EXPECTED_SHA, Got: $ACTUAL_SHA. Security breach prevented."
                        fi
                        step_done "Integrity verified: $ACTUAL_SHA"
                    else
                        step_done "Binary integrity unverified (matching manifest entry missing)."
                    fi
                else
                    warn "Could not download integrity manifest from $PAGES_URL. Falling back to ELF validation."
                fi
                rm -f "$TEMP_SUMS" 2>/dev/null || true

                chmod +x "$TEMP_BINARY"
                # Validate binary structure
                VALID=true
                if command -v file &>/dev/null; then
                    if ! file "$TEMP_BINARY" | grep -qE "ELF|executable"; then
                        VALID=false
                    fi
                fi
                
                if [ "$VALID" = true ]; then
                    mv "$TEMP_BINARY" "$BIN_DIR/myth"
                    if [ "$IS_TERMUX" = false ]; then
                        ln -sf "$BIN_DIR/myth" "$BIN_DIR/agent" 2>/dev/null || true
                        ln -sf "$BIN_DIR/myth" "$BIN_DIR/chief" 2>/dev/null || true
                    fi
                    BINARY_SUCCESS=true
                    ok "Pre-built binary deployed: $BIN_DIR/myth"
                else
                    warn "Downloaded binary failed security integrity audit. Falling back to source."
                    rm -f "$TEMP_BINARY"
                fi
            else
                warn "Download failed or empty file. Falling back to source compilation."
                rm -f "$TEMP_BINARY" 2>/dev/null || true
            fi
        fi
    fi
fi

# ═══════════════════════════════════════════════════════════
#  PATH C: Source Compilation (Last Resort)
# ═══════════════════════════════════════════════════════════
if [ "$PKG_SUCCESS" = false ] && [ "$BINARY_SUCCESS" = false ]; then

    section "SOURCE DEPLOYMENT (LEVEL 3 — LAST RESORT)"
    warn "Package manager and pre-built binary both failed. Compiling from source."
    warn "This may take 20-60 minutes on ARM devices with limited RAM."

    # ─── Ensure a WORKING Rust toolchain ───
    install_rust_fresh() {
        step_start "Installing Neural Rust Toolchain" "$TIME_RUST_SETUP"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable 2>&1 | tail -3
        if [ -f "$HOME/.cargo/env" ]; then
            # shellcheck source=/dev/null
            . "$HOME/.cargo/env"
        fi
        step_done "Rust toolchain deployed."
    }

    if cargo --version &>/dev/null 2>&1; then
        ok "Cargo is functional: $(cargo --version 2>/dev/null)"
    else
        if command -v rustup &>/dev/null && rustup --version &>/dev/null 2>&1; then
            step_start "Configuring Default Rust Toolchain" "$TIME_CONFIG_SETUP"
            rustup default stable 2>&1 | tail -1
            if cargo --version &>/dev/null 2>&1; then
                step_done "Cargo configured: $(cargo --version 2>/dev/null)"
            else
                install_rust_fresh
            fi
        else
            install_rust_fresh
        fi
    fi

    if ! cargo --version &>/dev/null 2>&1; then
        err "Cargo is not functional. Cannot compile from source. Install Rust manually: https://rustup.rs"
    fi

    # ─── Build ───
    if [ -f "Cargo.toml" ]; then
        step_start "Initiating Neural Core Compilation" "$TIME_CARGO_BUILD"
        if ! cargo build --release 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
            step_fail "Compilation failed. Check $LOG_FILE"
        fi
        step_done "Neural Core Compilation Complete."
    else
        require_command git
        step_start "Cloning Operation Repository" "$TIME_GH_DOWNLOAD"
        if ! git clone --depth 1 "${CLEAN_REPO_URL}.git" "$BUILD_DIR" 2>&1 | tee -a "$LOG_FILE"; then
            step_fail "Failed to clone ${CLEAN_REPO_URL}"
        fi
        step_done "Repository cloned."

        cd "$BUILD_DIR"
        step_start "Compiling Neural Core" "$TIME_CARGO_BUILD"
        if ! cargo build --release 2>&1 | tee -a "$LOG_FILE" | tail -5 >&3; then
            step_fail "Compilation failed. Check $LOG_FILE"
        fi
        step_done "Neural Core Compiled successfully."
    fi

    if [ ! -f "target/release/myth" ]; then
        err "Binary not found at target/release/myth after compilation."
    fi

    step_done "Neural Core Compilation Complete."
    step_start "Deploying Binaries" "$TIME_CONFIG_SETUP"
    cp target/release/myth "$BIN_DIR/myth"
    if [ "$IS_TERMUX" = false ]; then
        ln -sf "$BIN_DIR/myth" "$BIN_DIR/agent"
        ln -sf "$BIN_DIR/myth" "$BIN_DIR/chief"
    fi
    step_done "Binaries deployed to $BIN_DIR/"
fi


# ═══════════════════════════════════════════════════════════
#  Configuration & Setup
# ═══════════════════════════════════════════════════════════
USER_YAML="${CONFIG_DIR}/user.yaml"

if [ ! -f "$USER_YAML" ]; then
    mkdir -p "$CONFIG_DIR"
    step_start "Deploying Tactical Configuration" "$TIME_CONFIG_SETUP"
    # Attempt to download the FULL premium template from the web nexus
    if curl -fsSL --connect-timeout 10 --max-time 30 "${PAGES_URL}/user.yaml" -o "$USER_YAML" 2>/dev/null; then
        step_done "Premium configuration template synchronized."
    elif [ -f "config/user.yaml" ]; then
        cp config/user.yaml "$USER_YAML"
        step_done "Local configuration template deployed."
    else
        audit "Network unreachable. Deploying emergency minimal config."
        cat <<EOF > "$USER_YAML"
agent:
  user_name: "Chief"
  nvidia_api_key: ""
  all_report_path: "${REAL_HOME}/Downloads"
EOF
        step_done "Emergency config deployed."
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
if [ -c /dev/tty ]; then
    CURRENT_NAME=$(grep "user_name:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"'\' || echo "")
    if [ -z "$CURRENT_NAME" ] || [ "$CURRENT_NAME" = "Chief" ]; then
        echo -en "${CYAN}⠿  Enter your Operative Handle [Default: Chief]: ${NC}" >&3
        read -r OPERATIVE_NAME < /dev/tty || OPERATIVE_NAME=""
        OPERATIVE_NAME=${OPERATIVE_NAME:-Chief}
        sed -i "s/user_name: .*/user_name: \"$OPERATIVE_NAME\"/" "$USER_YAML"
        ok "Handle: $OPERATIVE_NAME"
    fi

    CURRENT_KEY=$(grep "nvidia_api_key:" "$USER_YAML" 2>/dev/null | awk '{print $2}' | tr -d '"'\' || echo "")
    if [ -z "$CURRENT_KEY" ]; then
        echo -en "${CYAN}⠿  NVIDIA NIM API Key (optional, press Enter to skip): ${NC}" >&3
        read -r -s API_KEY < /dev/tty || API_KEY=""
        echo "" >&3
        if [ -n "$API_KEY" ]; then
            sed -i "s/nvidia_api_key: .*/nvidia_api_key: \"$API_KEY\"/" "$USER_YAML"
            ok "API Key configured."
        else
            warn "No API Key. Neural reasoning disabled until set."
        fi
    fi
else
    info "Non-interactive environment detected. Skipping identification."
fi

# ─── Browser Engine Provisioning (Elite Compulsory) ───
section "BROWSER ENGINE PROVISIONING"
if [ "$INSTALL_BROWSER" = false ]; then
    info "Browser engine provisioning skipped (--no-browser)."
elif ! command -v lightpanda &>/dev/null && [ ! -f "${REAL_HOME}/.local/bin/lightpanda" ] && [ ! -f "$BIN_DIR/lightpanda" ]; then
    info "Initiating Level-1 Autonomous Browser Engine Provisioning..."
    
    LP_INSTALL_DIR="${REAL_HOME}/.local/bin"
    if [ "$IS_TERMUX" = true ]; then
        LP_INSTALL_DIR="$PREFIX/bin"
    fi
    mkdir -p "$LP_INSTALL_DIR"

    # Concurrency Lock with PID verification
    LOCK_DIR="${LP_INSTALL_DIR}/.lightpanda.lock"
    while ! mkdir "$LOCK_DIR" 2>/dev/null; do
        if [ -f "$LOCK_DIR/pid" ]; then
            LOCK_PID=$(cat "$LOCK_DIR/pid")
            if ! kill -0 "$LOCK_PID" 2>/dev/null; then
                rm -rf "$LOCK_DIR"
                continue
            fi
        fi
        warn "Another provisioning engine is active (PID: ${LOCK_PID:-unk}). Waiting..."
        sleep 2
    done
    echo "$$" > "$LOCK_DIR/pid"
    OLD_TRAP=$(trap -p EXIT | sed "s/^trap -- '//;s/' EXIT$//")
    # shellcheck disable=SC2064
    trap "rm -rf \"\$LOCK_DIR\" 2>/dev/null || true; ${OLD_TRAP}" EXIT

    ARCH_LP=$(uname -m)
    case $ARCH_LP in
        x86_64) BINARY="lightpanda-x86_64-linux" ;;
        aarch64) BINARY="lightpanda-aarch64-linux" ;;
        *) warn "Lightpanda not available for $ARCH_LP. Skipping browser engine (non-critical)."
           BINARY="" ;;
    esac
    
    if [ -n "$BINARY" ]; then
        TEMP_FILE="${TMP_DIR}/lightpanda_$$.tmp"
        URL="https://github.com/lightpanda-io/browser/releases/download/nightly/$BINARY"
        
        step_start "Synchronizing Tactical Browser Engine" "$TIME_LP_SYNC"
        SYNC_SUCCESS=false
        for attempt in 1 2 3; do
            if curl -fsSL --connect-timeout 30 --max-time 300 --retry 0 \
                   --progress-bar "$URL" -o "$TEMP_FILE" 2>&1; then
                SYNC_SUCCESS=true
                break
            else
                warn "Synchronization attempt $attempt/3 failed. Retrying in 5s..."
                rm -f "$TEMP_FILE" 2>/dev/null || true
                sleep 5
            fi
        done

        if [ "$SYNC_SUCCESS" = true ]; then
            step_done "Tactical browser engine synchronized."
            FILE_SIZE=$(stat -c%s "$TEMP_FILE" 2>/dev/null || echo 0)
            if [ "$FILE_SIZE" -lt 10240 ]; then
                rm -f "$TEMP_FILE"
                rmdir "$LOCK_DIR" 2>/dev/null || true
                warn "Downloaded Lightpanda binary is suspiciously small (${FILE_SIZE} bytes). Skipping."
            else
                chmod +x "$TEMP_FILE"
                mv "$TEMP_FILE" "${LP_INSTALL_DIR}/lightpanda"
                sync "${LP_INSTALL_DIR}/lightpanda" 2>/dev/null || true
                
                if [ -n "${SUDO_USER:-}" ]; then
                    chown "$SUDO_USER:$(id -gn "$SUDO_USER")" "${LP_INSTALL_DIR}/lightpanda"
                fi
                rmdir "$LOCK_DIR" 2>/dev/null || true
                ok "Engine lock established at ${LP_INSTALL_DIR}/lightpanda"
            fi
        else
            rmdir "$LOCK_DIR" 2>/dev/null || true
            warn "Lightpanda provisioning failed. MYTH will work without it (non-critical)."
        fi
    else
        rmdir "$LOCK_DIR" 2>/dev/null || true
    fi
else
    ok "Browser Engine: $(lightpanda version 2>/dev/null | head -1 || command -v lightpanda || echo 'Active')"
fi

# ─── Final Validation ───
section "FINAL SECURITY AUDIT"
if command -v bwrap &>/dev/null; then
    ok "Sandbox: $(bwrap --version 2>/dev/null | head -1)"
elif [ "$IS_TERMUX" = true ]; then
    audit "Sandbox: Not applicable in Termux environment."
else
    warn "Sandbox missing. Run: sudo apt install bubblewrap"
fi

if command -v myth &>/dev/null; then
    ok "MYTH Binary: $(myth --version 2>/dev/null | head -1 || echo 'installed')"
elif [ -f "$BIN_DIR/myth" ]; then
    ok "MYTH Binary: $BIN_DIR/myth (may need PATH update)"
else
    warn "MYTH not found in PATH."
fi

# ─── Done ───
echo -e "\n${GREEN}${BOLD}  ✅ MYTH DEPLOYMENT COMPLETE!${NC}" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3
echo -e "  ${BOLD}Config:${NC}  $USER_YAML" >&3
echo -e "  ${BOLD}Binary:${NC}  $BIN_DIR/myth" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3

# ─── Deployment Method Summary ───
section "DEPLOYMENT SUMMARY"
if [ "$PKG_SUCCESS" = true ]; then
    case "$DISTRO_FAMILY" in
        debian)  audit "Install Method: ${GREEN}APT Repository${NC}"; audit "Update Command: ${BOLD}sudo apt update && sudo apt upgrade myth${NC}" ;;
        termux)  audit "Install Method: ${GREEN}Termux pkg${NC}"; audit "Update Command: ${BOLD}pkg upgrade myth${NC}" ;;
        fedora)  audit "Install Method: ${GREEN}$PKG_MANAGER Repository${NC}"; audit "Update Command: ${BOLD}sudo $PKG_MANAGER upgrade myth${NC}" ;;
        arch)    audit "Install Method: ${GREEN}AUR / pacman${NC}"; audit "Update Command: ${BOLD}yay -Syu${NC} or ${BOLD}sudo pacman -Syu${NC}" ;;
    esac
elif [ "$BINARY_SUCCESS" = true ]; then
    audit "Install Method: ${YELLOW}Direct Binary (GitHub Releases)${NC}"
    audit "Update Command: ${BOLD}Re-run this installer${NC} or run: curl -sSL ${PAGES_URL}/install.sh | sudo bash"
else
    audit "Install Method: ${YELLOW}Source Compilation${NC}"
    audit "Update Command: ${BOLD}Re-run this installer${NC}"
fi

# Dynamic Reminders
section "POST-MISSION CHECKLIST"

CURRENT_NAME=$(grep "user_name:" "$USER_YAML" 2>/dev/null | awk '{print $NF}' | tr -d '"'\' || echo "Chief")
if [ "$CURRENT_NAME" = "Chief" ]; then
    audit "${YELLOW}Identity Unset:${NC}  Update your handle in $USER_YAML"
fi

CURRENT_KEY=$(grep "nvidia_api_key:" "$USER_YAML" 2>/dev/null | awk '{print $NF}' | tr -d '"'\' || echo "")
if [ -z "$CURRENT_KEY" ]; then
    audit "${RED}Neural Link Offline:${NC} NVIDIA API Key is missing."
    audit "        ↳ Get one at: https://build.nvidia.com/"
    audit "        ↳ Set it in: $USER_YAML"
fi

audit "${CYAN}Arsenal Preparation:${NC} Run 'myth sync' to download the 3000+ tool definitions."

echo -e "\n  ${BOLD}TACTICAL NEXT STEPS:${NC}" >&3
echo -e "    1. ${CYAN}myth sync${NC}           - Synchronize mission tools & metadata" >&3
echo -e "    2. ${CYAN}myth check${NC}          - Operational environment audit" >&3
echo -e "    3. ${CYAN}myth scan <target>${NC}    - Initiate autonomous reconnaissance" >&3

echo -e "\n  ${BOLD}TACTICAL NEXUS (Full Docs):${NC}" >&3
echo -e "    ${PAGES_URL} (Check /installation and /quickstart)" >&3
echo -e "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" >&3

# Record installation metric (local)
date +%s > "${CONFIG_DIR}/.installed_at" 2>/dev/null || true
echo "" >&3
