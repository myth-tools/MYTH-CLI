#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Automated Tactical Update Utility [Industry-Grade]
#  ─────────────────────────────────────────────────────────
#  Updates the MYTH binary with atomic swap, secure
#  verification, and distribution-aware rollback logic.
# ═══════════════════════════════════════════════════════════
set -euo pipefail

# ─── Early Constants ───
IS_TERMUX=false
IS_PROOT=false
IS_ALPINE=false

if [ -n "${PREFIX:-}" ] && echo "${PREFIX}" | grep -q "com.termux"; then
    IS_TERMUX=true
    if [ -d "/bin" ] && [ -d "/usr/lib" ] && [ ! -L "/bin" ]; then
        IS_PROOT=true
    fi
fi
if [ -f /etc/os-release ] && grep -q "ID=alpine" /etc/os-release; then
    IS_ALPINE=true
fi


# ─── Visual Branding (Ultra-Premium Style) ───
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}" >&3; }
ok()      { echo -e "${GREEN}✔${NC}  $1" >&3; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1" >&3; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1" >&3; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1" >&3; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}" >&3; }

# ─── Configuration ───
PAGES_URL="__PAGES_URL__"
if [[ "$PAGES_URL" == "__"*"__" ]]; then PAGES_URL="https://myth.work.gd"; fi

TMP_DIR="${TMPDIR:-/tmp}"
UPDATE_LOG="/var/log/myth/update.log"
# Attempt to create log dir if running as root
mkdir -p "/var/log/myth" 2>/dev/null || UPDATE_LOG="${TMP_DIR}/myth-update.log"

exec 3>&1 # Keep terminal stdout accessible via fd3
# Strip ANSI codes for the physical log file
exec > >(sed -u 's/\x1b\[[0-9;]*[a-zA-Z]//g' >> "$UPDATE_LOG") 2>&1

# ─── Elite Deployment Engine (Atomic & Sovereign) ───
deploy_binary_atomic() {
    local src_file="$1"
    local dest_file="$2"
    local dest_dir
    dest_dir=$(dirname "$dest_file")
    local backup_file="${dest_file}.old"
    local temp_file="${dest_file}.new"

    info "Executing Atomic Deployment..."
    
    if [ ! -w "$dest_dir" ]; then
        err "Destination directory $dest_dir is not writable."
    fi

    # 1. Create a sibling temp file for atomic swap
    cp "$src_file" "$temp_file" || err "Failed to stage binary at $temp_file"
    chmod +x "$temp_file"

    # 2. Sovereign Verification (Pre-Swap)
    if ! "$temp_file" --version &>/dev/null; then
        rm -f "$temp_file"
        err "New binary verification failed. Aborted to prevent corruption."
    fi

    # 3. Create Fallback Point
    if [ -f "$dest_file" ]; then
        cp -f "$dest_file" "$backup_file" 2>/dev/null || true
    fi

    # 4. Atomic Swap
    if mv -f "$temp_file" "$dest_file"; then
        # 5. Final Smoke Test
        if "$dest_file" --version &>/dev/null; then
            ok "Atomic deployment successful: $(basename "$dest_file")"
            return 0
        else
            warn "Final verification failed. Initiating Sovereign Rollback..."
            [ -f "$backup_file" ] && mv -f "$backup_file" "$dest_file"
            err "Deployment failed post-swap. Rollback completed."
        fi
    else
        rm -f "$temp_file"
        err "Atomic swap failed (Filesystem error)."
    fi
}


# ─── 0. Signal Trapping & Cleanup ───
CLEANUP_FILES=()
cleanup() {
    local exit_code=$?
    [ ${#CLEANUP_FILES[@]} -gt 0 ] && rm -rf "${CLEANUP_FILES[@]}"
    rm -rf "${TMP_DIR}/.myth_update.lock" 2>/dev/null || true
    if [ $exit_code -ne 0 ]; then
        echo -e "${RED}✘ Update aborted.${NC}" >&3
        echo -e "${YELLOW}⠿ Logs: $UPDATE_LOG${NC}" >&3
    fi
    exit $exit_code
}
trap cleanup EXIT INT TERM

# ─── 1. Pre-flight Environment Audit ───
section "NEURAL CORE UPDATE AUDIT"
info "Logs initialized at: $UPDATE_LOG"

# A. Process Locking
LOCK_DIR="${TMP_DIR}/.myth_update.lock"
if ! mkdir "$LOCK_DIR" 2>/dev/null; then
    err "Tactical update already in progress (lock held at $LOCK_DIR)."
fi

# B. Identification
if ! command -v myth &>/dev/null; then
    err "MYTH is not installed. Use the standard installer instead."
fi
BIN_PATH=$(command -v myth)
CURRENT_VERSION=$(myth --version 2>/dev/null | head -1 | awk '{print $NF}' || echo "0.0.0")
audit "Current Binary: $BIN_PATH (v$CURRENT_VERSION)"

# C. Privilege Escalation Logic
if [ ! -w "$(dirname "$BIN_PATH")" ] || [ ! -w "$BIN_PATH" ]; then
    err "Insufficient permissions to modify $BIN_PATH. Re-run as root (use sudo)."
fi

# D. Disk Space Audit (Min 50MB required for safety)
AVAILABLE_KB=$(df -k "$TMP_DIR" | tail -1 | awk '{print $4}')
if [ "$AVAILABLE_KB" -lt 51200 ]; then
    err "Insufficient disk space in $TMP_DIR ($((AVAILABLE_KB/1024))MB available, 50MB required)."
fi

# ─── 2. Distribution Awareness Check ───
# Prevent binary swap if installed via system package manager
CHECK_PKG_MANAGER=true
if [ "$CHECK_PKG_MANAGER" = true ]; then
    IS_SYSTEM_OWNED=false
    if command -v dpkg &>/dev/null && dpkg -S "$BIN_PATH" &>/dev/null; then IS_SYSTEM_OWNED=true; PKG_SYS="apt-get upgrade"; fi
    if command -v rpm &>/dev/null && rpm -qf "$BIN_PATH" &>/dev/null; then IS_SYSTEM_OWNED=true; PKG_SYS="dnf upgrade"; fi
    if command -v pacman &>/dev/null && pacman -Qo "$BIN_PATH" &>/dev/null; then IS_SYSTEM_OWNED=true; PKG_SYS="pacman -Syu"; fi
    
    if [ "$IS_SYSTEM_OWNED" = true ]; then
        warn "This binary is managed by the system package manager ($PKG_SYS)."
        warn "Manual binary updates are unsafe and will be overwritten by system updates."
        echo -en "${CYAN}⠿ Proceed with manual override anyway? [y/N]: ${NC}" >&3
        read -r response < /dev/tty || response="N"
        if [[ ! "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            info "Update aborted. Please use '$PKG_SYS myth' instead."
            exit 0
        fi
    fi
fi

# ─── 3. Query Tactical Repository ───
section "REPOSITORY SYNCHRONIZATION"
FORCE=false
CHECK_ONLY=false
for arg in "$@"; do
    [[ "$arg" == "--force" ]] && FORCE=true
    [[ "$arg" == "--check" ]] && CHECK_ONLY=true
done

info "Querying latest version standard from $PAGES_URL..."
LATEST_VERSION=$(curl -fsSL "${PAGES_URL}/version.txt" 2>/dev/null | head -n 1 | tr -d 'v' || echo "")
if [ -z "$LATEST_VERSION" ]; then
    err "Update server offline or returning empty manifest."
fi

if [ "$LATEST_VERSION" = "$CURRENT_VERSION" ] && [ "$FORCE" = false ]; then
    ok "MYTH is already at the latest tactical standard (v$CURRENT_VERSION)."
    exit 0
fi

if [ "$CHECK_ONLY" = true ]; then
    ok "Tactical upgrade available: v$CURRENT_VERSION -> v$LATEST_VERSION"
    exit 0
fi

# ─── 4. Secure Artifact Acquisition ───
section "SECURE ARTIFACT ACQUISITION"
ARCH=$(uname -m)
STATIC_TARGET=""
case "$ARCH" in
    x86_64)  GH_BINARY="myth-x86_64-unknown-linux-gnu"; STATIC_TARGET="myth-musl-x64-static" ;;
    aarch64) GH_BINARY="myth-aarch64-unknown-linux-gnu"; STATIC_TARGET="myth-musl-arm64-static" ;;
    armv7l)  GH_BINARY="myth-armv7-unknown-linux-gnueabihf"; STATIC_TARGET="" ;;
    i*86)    GH_BINARY="myth-i686-unknown-linux-gnu"; STATIC_TARGET="" ;;
    *)       err "Unsupported architecture for automated rolling update: $ARCH" ;;
esac

# Portability Optimization: Target static 'musl' binary for Termux/Alpine
if [[ ( "$IS_TERMUX" = true && "$IS_PROOT" = false ) || "$IS_ALPINE" = true ]]; then
    if [ -n "$STATIC_TARGET" ]; then
        GH_BINARY="$STATIC_TARGET"
        info "Portability optimization: Targeting static 'musl' binary."
    fi
fi


TEMP_BINARY="${TMP_DIR}/myth-download-$$"
CLEANUP_FILES+=("$TEMP_BINARY")
URL="https://github.com/myth-tools/MYTH-CLI/releases/download/v${LATEST_VERSION}/${GH_BINARY}"

info "Acquiring neural core (v$LATEST_VERSION)..."
if ! curl -fsSL --progress-bar "$URL" -o "$TEMP_BINARY"; then
    err "Artifact acquisition failed ($URL)."
fi

# A. Integrity Manifest Verification
info "Verifying SHA256 integrity manifest..."
TEMP_SUMS="${TMP_DIR}/myth-sums-$$"
CLEANUP_FILES+=("$TEMP_SUMS")
if curl -fsSL "${PAGES_URL}/SHA256SUMS" -o "$TEMP_SUMS" 2>/dev/null; then
    EXPECTED_SHA=$(grep "$GH_BINARY" "$TEMP_SUMS" | awk '{print $1}' || echo "")
    if [ -n "$EXPECTED_SHA" ]; then
        ACTUAL_SHA=$(sha256sum "$TEMP_BINARY" | awk '{print $1}')
        if [ "$EXPECTED_SHA" != "$ACTUAL_SHA" ]; then
            err "SHA256 Hash Mismatch! Calculated: $ACTUAL_SHA, Expected: $EXPECTED_SHA"
        fi
        ok "Integrity verified."
    else
        # Strict security audit: Require manifest inclusion
        err "Security verification failure: Binary not found in official manifest ($GH_BINARY)."
    fi
else
    warn "Could not download integrity manifest from $PAGES_URL. Proceeding with ELF-only validation."
fi


# B. GPG Verification (Premium Security)
if command -v gpg &>/dev/null && [ -f "$TEMP_SUMS.asc" ] 2>/dev/null; then
    info "Verifying GPG signature of manifest..."
    if gpg --verify "$TEMP_SUMS.asc" "$TEMP_SUMS" &>/dev/null; then
        ok "Cryptographic signature verified."
    else
        warn "GPG Signature failure for manifest."
    fi
fi

# C. Architecture-Aware Binary Sanity Check
info "Validating architecture-specific ELF header..."
if command -v file &>/dev/null; then
    FILE_INFO=$(file "$TEMP_BINARY")
    if ! echo "$FILE_INFO" | grep -q "ELF"; then
        err "Downloaded file is not a valid ELF binary ($FILE_INFO)."
    fi
    # Quick architecture sanity check
    case "$ARCH" in
        x86_64) echo "$FILE_INFO" | grep -q "x86-64" || warn "Architecture mismatch suspected." ;;
        aarch64) echo "$FILE_INFO" | grep -q "aarch64" || warn "Architecture mismatch suspected." ;;
    esac
fi

# ─── 5. Atomic Deploy & Sovereignty ───
section "ATOMIC DEPLOIMENT"
deploy_binary_atomic "$TEMP_BINARY" "$BIN_PATH"

# ─── 6. Final Health Check ───
section "POST-UPDATE HEALTH CHECK"
if myth check --quick 2>/dev/null; then
    ok "Neural core healthy and mission-ready."
else
    warn "Post-update health check identified potential drift. Run 'myth check' for full audit."
fi

ok "Rolling update successful. MYTH upgraded to v$LATEST_VERSION."

section "MISSION SUCCESS"
ok "System version: v$LATEST_VERSION"
info "Neural conduits stabilized at $BIN_PATH"
echo ""

