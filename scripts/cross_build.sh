#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Universal Linux Architecture Package Builder
#  Produces packages for any architecture using Docker.
#
#  Usage:
#    bash scripts/cross_build.sh               # builds all TARGETS
#    bash scripts/cross_build.sh arm64         # builds only arm64
#    bash scripts/cross_build.sh amd64 arm64   # builds specific list
#
#  Requires: Docker (running), cargo-deb, rustup
# ═══════════════════════════════════════════════════════════
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# ─── Visual Branding ───
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
# CYAN is unused but reserved for tactical consistency
MAGENTA='\033[0;35m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

# ─── Architecture Target Map ───
# Format: "deb_arch:rust_target"
declare -A ARCH_MAP=(
    [amd64]="x86_64-unknown-linux-gnu"
    [arm64]="aarch64-unknown-linux-gnu"
    [armhf]="armv7-unknown-linux-gnueabihf"
    [i386]="i686-unknown-linux-gnu"
    [musl-x64]="x86_64-unknown-linux-musl"
    [musl-arm64]="aarch64-unknown-linux-musl"
)

# ─── Portability Target List (Static binaries for Termux/Alpine) ───
PORTABILITY_TARGETS=("musl-x64" "musl-arm64")


# ─── Determine what to build ───
if [ $# -eq 0 ]; then
    # Default: build arm64 (Glibc) and all Portability targets (Static)
    BUILD_ARCHES=("arm64" "${PORTABILITY_TARGETS[@]}")
else
    BUILD_ARCHES=("$@")
fi


case "$(uname -m)" in
    x86_64)  HOST_DEB_ARCH="amd64" ;;
    aarch64) HOST_DEB_ARCH="arm64" ;;
    armv7*)  HOST_DEB_ARCH="armhf" ;;
    i*86)    HOST_DEB_ARCH="i386" ;;
    *)       HOST_DEB_ARCH="amd64" ;;
esac

section "CROSS-COMPILATION ENGINE INITIALIZATION"
info "Targets: ${BUILD_ARCHES[*]}"
info "Host architecture: $HOST_DEB_ARCH (will skip if in target list)"

# ─── Ensure Docker is running ───
if ! docker info &>/dev/null; then
    err "Docker daemon is not running. Start Docker first: sudo systemctl start docker"
fi
ok "Docker daemon: active"

# Retry loop for Docker registry access
REGISTRY_IMAGE="ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5"
IMAGE_PRESENT=false
if docker images -q "$REGISTRY_IMAGE" &>/dev/null; then
    IMAGE_PRESENT=true
    ok "Registry image present locally."
else
    info "Pre-fetching registry image (ghcr.io) with 3-attempt exponential backoff..."

    for attempt in 1 2 3; do
        if docker pull "$REGISTRY_IMAGE" &>/dev/null; then
            IMAGE_PRESENT=true
            ok "Registry image synchronized."
            break
        else
            warn "Registry pull attempt $attempt/3 timed out. Retrying in $((attempt * 5))s..."
            sleep $((attempt * 5))
        fi
    done
fi

if [ "$IMAGE_PRESENT" = false ]; then
    warn "Registry unreachable. Cross-build will attempt to use local toolchains if available."
fi


# ─── Ensure cross is installed ───
if ! command -v cross &>/dev/null; then
    info "Installing cross (Rust cross-compiler)..."
    cargo install cross --locked 2>&1 | tail -3
    ok "cross installed."
fi
ok "cross: $(cross --version 2>/dev/null | head -1)"

# ─── Ensure cargo-deb is installed ───
if ! cargo deb --version &>/dev/null; then
    info "Installing cargo-deb..."
    cargo install cargo-deb --locked 2>&1 | tail -3
    ok "cargo-deb installed."
fi
ok "cargo-deb: $(cargo deb --version 2>/dev/null)"

# Standardized Extraction: Targets the top-level version field from Cargo.toml
MYTH_VERSION=$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' Cargo.toml | head -n 1)


info "Package version: $MYTH_VERSION"

PRODUCED_DEBS=()

for DEB_ARCH in "${BUILD_ARCHES[@]}"; do
    section "Building for: $DEB_ARCH"

    # ─── Resolve Rust target ───
    RUST_TARGET="${ARCH_MAP[$DEB_ARCH]:-}"
    if [ -z "$RUST_TARGET" ]; then
        warn "Unknown architecture '$DEB_ARCH'. Supported: ${!ARCH_MAP[*]}. Skipping."
        continue
    fi

    # ─── Skip if this is the host arch (avoid double-building) ───
    if [ "$DEB_ARCH" = "$HOST_DEB_ARCH" ]; then
        info "Skipping $DEB_ARCH (host architecture — already built by release_local.sh)"
        # Verify the host .deb already exists
        HOST_DEB=$(find target/debian -maxdepth 1 -name "myth_*_${DEB_ARCH}.deb" -print 2>/dev/null | sort -rV | head -1 || true)
        if [ -n "$HOST_DEB" ]; then
            ok "Host .deb confirmed: $HOST_DEB"
            PRODUCED_DEBS+=("$HOST_DEB")
        else
            warn "Host .deb not found (run release_local.sh first or build manually)."
        fi
        continue
    fi

    # ─── Add Rust target if not present ───
    if ! rustup target list --installed | grep -q "$RUST_TARGET"; then
        info "Adding Rust target: $RUST_TARGET..."
        rustup target add "$RUST_TARGET"
        ok "Target added: $RUST_TARGET"
    else
        ok "Rust target installed: $RUST_TARGET"
    fi

    # ─── Cross-compile the binary (with Docker fallback) ───
    info "Cross-compiling for $RUST_TARGET..."
    
    BUILD_LOG=$(mktemp)
    CLEANUP_FILES+=("$BUILD_LOG")
    
    # Attempt 1: Cross-rs (Docker requirement)
    CROSS_SUCCESS=false
    if cross build --release --target "$RUST_TARGET" 2>&1 | tee "$BUILD_LOG"; then
        CROSS_SUCCESS=true
    else
        warn "Cross-rs build failed (Docker timeout or registry issue)."
        
        # Attempt 2: Native cargo build (Requires local toolchain)
        # Determine the required linker for this target
        LINKER=""
        REMEDIATION=""
        case "$RUST_TARGET" in
            aarch64-unknown-linux-gnu*) LINKER="aarch64-linux-gnu-gcc"; REMEDIATION="sudo apt install gcc-aarch64-linux-gnu" ;;
            armv7-unknown-linux-gnueabihf*) LINKER="arm-linux-gnueabihf-gcc"; REMEDIATION="sudo apt install gcc-arm-linux-gnueabihf" ;;
            i686-unknown-linux-gnu*) LINKER="gcc-multilib"; REMEDIATION="sudo apt install gcc-multilib" ;;
            *musl*) # Musl targets often need specialized cross-linkers or 'musl-tools'
                LINKER="musl-gcc"; REMEDIATION="sudo apt install musl-tools" ;;
        esac

        if [ -n "$LINKER" ] && ! command -v "$LINKER" &>/dev/null; then
            warn "Local toolchain for $RUST_TARGET is missing (Required: $LINKER)."
            echo -e "${YELLOW}  ↳ [REMEDIATION] Run: ${CYAN}${BOLD}$REMEDIATION${NC}"
            warn "All compilation paths exhausted for $DEB_ARCH."
        else
            info "Attempting native cargo build fallback (using host toolchain)..."
            if cargo build --release --target "$RUST_TARGET" 2>&1 | tee -a "$BUILD_LOG"; then
                CROSS_SUCCESS=true
                ok "Native cargo build fallback successful."
            else
                cat "$BUILD_LOG" | tail -10
                warn "Native cargo build failed for $DEB_ARCH."
            fi
        fi
    fi

    if [ "$CROSS_SUCCESS" = false ]; then
        warn "Failed to produce binary for architecture: $DEB_ARCH."
        # We continue to the next architecture instead of exiting, allowing partial success
        continue
    fi


    CROSS_BINARY="target/$RUST_TARGET/release/myth"
    if [ ! -f "$CROSS_BINARY" ]; then
        warn "Binary not found at $CROSS_BINARY after cross-compile. Skipping."
        continue
    fi
    ok "Binary compiled: $CROSS_BINARY ($(du -h "$CROSS_BINARY" | cut -f1))"

    # ─── Package Selection Logic ───
    # Portability targets are kept as raw static binaries. Standard targets are packaged as .deb.
    IS_STATIC=false
    for pt in "${PORTABILITY_TARGETS[@]}"; do
        if [ "$DEB_ARCH" = "$pt" ]; then IS_STATIC=true; break; fi
    done

    if [ "$IS_STATIC" = true ]; then
        info "Preserving static binary for portability archive..."
        mkdir -p target/portability
        DEST_BIN="target/portability/myth-${DEB_ARCH}-static"
        cp "$CROSS_BINARY" "$DEST_BIN"
        chmod +x "$DEST_BIN"
        ok "Static binary ready: $DEST_BIN ($(du -h "$DEST_BIN" | cut -f1))"
        # We don't add to PRODUCED_DEBS, but we track it for the summary if needed
    else
        # ─── Package with cargo-deb ───
        # cargo-deb reads the binary path from --target and sets architecture automatically
        info "Packaging .deb for $DEB_ARCH..."
        cargo deb --no-build --target "$RUST_TARGET" 2>&1 | tail -5

        # cargo-deb places the .deb under target/<RUST_TARGET>/debian/
        CROSS_DEB=$(find "target/$RUST_TARGET/debian" -maxdepth 1 -name 'myth_*.deb' | sort -rV | head -1 || true)

        if [ -z "$CROSS_DEB" ]; then
            warn "No .deb found in target/$RUST_TARGET/debian/ — skipping $DEB_ARCH."
            continue
        fi

        # ─── Normalize filename and copy to target/debian/ ───
        # This keeps all .deb files in one predictable location
        mkdir -p target/debian
        DEST_DEB="target/debian/myth_${MYTH_VERSION}-1_${DEB_ARCH}.deb"
        cp "$CROSS_DEB" "$DEST_DEB"

        ok "Package produced: $DEST_DEB ($(du -h "$DEST_DEB" | cut -f1))"
        PRODUCED_DEBS+=("$DEST_DEB")
    fi
done


# ─── Summary ───
section "BUILD SUMMARY"

if [ ${#PRODUCED_DEBS[@]} -eq 0 ]; then
    # Check if skipping was intentional (all targets matched host arch)
    ALL_HOST=true
    for DEB_ARCH in "${BUILD_ARCHES[@]}"; do
        [ "$DEB_ARCH" = "$HOST_DEB_ARCH" ] || ALL_HOST=false
    done
    if [ "$ALL_HOST" = true ]; then
        warn "All requested targets match host arch ($HOST_DEB_ARCH). No cross-compilation performed."
    else
        err "No packages produced. Check logs above for errors."
    fi
fi

for deb in "${PRODUCED_DEBS[@]}"; do
    ok "Ready: $deb"
done

echo ""
echo -e "${GREEN}${BOLD}  ✅ Cross-build complete. ${#PRODUCED_DEBS[@]} package(s) ready.${NC}"
echo ""
