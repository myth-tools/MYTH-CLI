#!/usr/bin/env bash
# ─── Tactical Deployment Package Builder — MYTH ───
# ─── Visual Branding (Ultra-Premium Cyber Style) ───
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

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
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

# ─── Version Intelligence ───
VERSION=$(grep "^version =" Cargo.toml | cut -d '"' -f 2)
if [ -z "$VERSION" ]; then
    err "Could not extract version from Cargo.toml"
fi

# ─── Flag Parsing ───
DRY_RUN=false
SIGN_PACKAGE=false
BUILD_ARCHES=()

for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        --sign) SIGN_PACKAGE=true ;;
        arm64|armhf|i386|amd64) BUILD_ARCHES+=("$arg") ;;
    esac
done

# If no architectures specified, use host
if [ ${#BUILD_ARCHES[@]} -eq 0 ]; then
    case "$(uname -m)" in
        x86_64)  BUILD_ARCHES=("amd64") ;;
        aarch64) BUILD_ARCHES=("arm64") ;;
        armv7*)  BUILD_ARCHES=("armhf") ;;
        i*86)    BUILD_ARCHES=("i386") ;;
        *)       BUILD_ARCHES=("amd64") ;;
    esac
fi

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ DEBIAN TACTICAL PACKAGE BUILDER — v$VERSION ]${NC}"
echo -e "  ${BOLD}Initiating package construction for: ${BUILD_ARCHES[*]}${NC}\n"

# ─── High-Fidelity Validation ───
section "Pre-Flight Validation"
require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}
require_command cargo

# Check for required tactical assets (some are optional)
for asset in "config/user.yaml" "docs/myth.1" "linux/myth.desktop" "completions/myth"; do
    if [ ! -f "$asset" ]; then
        warn "Missing optional asset: $asset. Package will be incomplete but functional."
    else
        ok "Validated: $asset"
    fi
done

# Check for cargo-deb tool
if ! cargo deb --version &>/dev/null; then
    info "Installing cargo-deb..."
    cargo install cargo-deb --locked || err "Failed to install cargo-deb. Ensure you have network access."
    ok "cargo-deb installed"
fi

section "Build Phase"
if [ "$DRY_RUN" = true ]; then
    ok "[DRY-RUN] Would build and package debs for: ${BUILD_ARCHES[*]}"
    exit 0
fi

# Map deb arch to rust target
declare -A RUST_TARGETS=(
    [amd64]="x86_64-unknown-linux-gnu"
    [arm64]="aarch64-unknown-linux-gnu"
    [armhf]="armv7-unknown-linux-gnueabihf"
    [i386]="i686-unknown-linux-gnu"
)

HOST_ARCH=$(case "$(uname -m)" in x86_64) echo amd64;; aarch64) echo arm64;; armv7*) echo armhf;; i*86) echo i386;; *) echo amd64;; esac)

for ARCH in "${BUILD_ARCHES[@]}"; do
    section "Target: $ARCH"
    
    TARGET_TRIPLE="${RUST_TARGETS[$ARCH]:-}"
    if [ -z "$TARGET_TRIPLE" ]; then
        warn "Unknown architecture: $ARCH. Skipping."
        continue
    fi

    if [ "$ARCH" = "$HOST_ARCH" ]; then
        info "Building native release binary..."
        cargo build --release --locked || err "Rust application build failed."
        ok "Release binary built."
        cargo deb --no-build
    else
        info "Invoking cross-build for $ARCH..."
        # If cross_build.sh exists, use it; otherwise fail gracefully
        if [ -f "scripts/cross_build.sh" ]; then
            bash scripts/cross_build.sh "$ARCH" || warn "Cross build failed for $ARCH."
        else
            err "Cross-build requested for $ARCH but scripts/cross_build.sh is missing."
        fi
    fi

    # ─── Show result and hash ───
    DEB_FILE=$(find target/debian -maxdepth 1 -name "*_${ARCH}.deb" -print 2>/dev/null | sort -rV | head -1 || true)
    if [ -n "$DEB_FILE" ] && [ -f "$DEB_FILE" ]; then
        SHA256=$(sha256sum "$DEB_FILE" | awk '{print $1}')
        ok "Package ready: $DEB_FILE"
        audit "SHA256: $SHA256"
        
        if [ "$SIGN_PACKAGE" = true ]; then
            info "Signing package (requires debsigs or dpkg-sig)..."
            if command -v dpkg-sig &>/dev/null; then
                dpkg-sig --sign builder "$DEB_FILE" && ok "Signed successfully."
            else
                warn "dpkg-sig not found. Skipping signature."
            fi
        fi
    else
        warn "No .deb file found in target/debian/ for $ARCH"
    fi
done

section "BUILD SUMMARY"
ok "All successful targets completed."
