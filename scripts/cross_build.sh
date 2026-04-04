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

# ─── Determine what to build ───
if [ $# -eq 0 ]; then
    # Default: skip host arch (already built by release_local.sh), build arm64
    BUILD_ARCHES=("arm64")
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

# ─── Pre-flight Performance & Connectivity Audit ───
info "Pre-flight registry check (ghcr.io)..."
DOCKER_VER=$(docker version --format '{{.Server.Version}}' 2>/dev/null || echo "0.0.0")
DOCKER_MAJOR=$(echo "$DOCKER_VER" | cut -d. -f1)

if [ "$DOCKER_MAJOR" -ge 24 ] && docker pull ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5 --dry-run &>/dev/null; then
    ok "Registry connection verified (dry-run)."
elif docker images -q ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5 &>/dev/null; then
    ok "Registry image present locally."
else
    # Attempt a shallow pull or just assume it's okay (cross will handle it)
    info "Registry access will be verified during build."
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

VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
info "Package version: $VERSION"

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
    # Attempt 1: Cross-rs (Docker/Podman required)
    if ! cross build --release --target "$RUST_TARGET" 2>&1 | tee "$BUILD_LOG"; then
        if grep -q "denied\|unauthorized" "$BUILD_LOG"; then
            err "Registry access denied for $DEB_ARCH. Run 'docker login ghcr.io' first."
        fi
        warn "Cross-rs build failed (Docker issue?). Attempting native cargo build fallback..."
        
        # Attempt 2: Native cargo build (requires local toolchain & cross-linker)
        if ! cargo build --release --target "$RUST_TARGET" 2>&1 | tee -a "$BUILD_LOG"; then
            cat "$BUILD_LOG" | tail -10
            rm -f "$BUILD_LOG"
            warn "All cross-compilation attempts failed for $DEB_ARCH. Skipping."
            continue
        fi
        ok "Native cargo build fallback successful."
    fi
    rm -f "$BUILD_LOG"

    CROSS_BINARY="target/$RUST_TARGET/release/myth"
    if [ ! -f "$CROSS_BINARY" ]; then
        warn "Binary not found at $CROSS_BINARY after cross-compile. Skipping."
        continue
    fi
    ok "Binary compiled: $CROSS_BINARY ($(du -h "$CROSS_BINARY" | cut -f1))"

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
    DEST_DEB="target/debian/myth_${VERSION}-1_${DEB_ARCH}.deb"
    cp "$CROSS_DEB" "$DEST_DEB"

    ok "Package produced: $DEST_DEB ($(du -h "$DEST_DEB" | cut -f1))"
    PRODUCED_DEBS+=("$DEST_DEB")
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
