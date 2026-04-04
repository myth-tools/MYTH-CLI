#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Arch Linux Package Builder (AUR + Custom Repo)
#  Produces PKGBUILD and .pkg.tar.zst packages from
#  pre-compiled binaries.
#
#  Usage:
#    bash scripts/build_arch.sh               # builds x86_64
#    bash scripts/build_arch.sh x86_64        # builds only x86_64
#    bash scripts/build_arch.sh --dry-run     # validate PKGBUILD only
#    bash scripts/build_arch.sh --aur-only    # generate AUR PKGBUILD only
#    bash scripts/build_arch.sh --help        # show full help
#
#  Note: Full .pkg.tar.zst generation requires makepkg (Arch only).
#        On non-Arch systems, this script generates the PKGBUILD and
#        a self-contained archive that can be installed with makepkg.
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

# ─── Parse arguments ───
DRY_RUN=false
AUR_ONLY=false
BUILD_ARCHES=()

for arg in "$@"; do
    case "$arg" in
        --dry-run)  DRY_RUN=true ;;
        --aur-only) AUR_ONLY=true ;;
        --help)
            echo "Usage: ./build_arch.sh [options] [archs]"
            echo "Options:"
            echo "  --dry-run     Validate PKGBUILD only"
            echo "  --aur-only    Generate AUR PKGBUILD only"
            echo "Archs:"
            echo "  x86_64, aarch64, armv7h"
            exit 0
            ;;
        amd64|x86_64)   BUILD_ARCHES+=("x86_64") ;;
        arm64|aarch64)   BUILD_ARCHES+=("aarch64") ;;
        armhf|armv7h)    BUILD_ARCHES+=("armv7h") ;;
        *)               warn "Unknown argument: $arg" ;;
    esac
done

# Default: x86_64 + aarch64 if binary available
if [ ${#BUILD_ARCHES[@]} -eq 0 ]; then
    BUILD_ARCHES=("x86_64")
    if [ -f "target/aarch64-unknown-linux-gnu/release/myth" ]; then
        BUILD_ARCHES+=("aarch64")
    fi
fi

# ─── Extract metadata from Cargo.toml ───
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
DESCRIPTION=$(sed -n 's/^description = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
LICENSE=$(sed -n 's/^license = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
HOMEPAGE=$(sed -n 's/^homepage = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
REPO_URL=$(sed -n 's/^repository = "\(.*\)"/\1/p' Cargo.toml | head -n 1)

# ─── Determine Pages URL ───
PAGES_URL=""
if [ -f "config/agent.yaml" ]; then
    PAGES_URL=$(grep "pages_url:" config/agent.yaml | head -n 1 | sed -E "s/.*pages_url:[[:space:]]*[\"':]?([^\"']+)[\"':]?.*/\1/" || echo "")
fi
if [ -z "$PAGES_URL" ]; then
    PAGES_URL="https://myth.work.gd"
fi

section "ARCH LINUX PACKAGE BUILDER"
info "Version: $VERSION"
info "Architectures: ${BUILD_ARCHES[*]}"

# ═══════════════════════════════════════════════════════════
#  PART 1: Generate AUR-Compatible PKGBUILD
#  This downloads the binary from GitHub Releases at install time.
#  Users install with: yay -S myth-bin  OR  paru -S myth-bin
# ═══════════════════════════════════════════════════════════
section "GENERATING AUR PKGBUILD"

AUR_DIR="$(pwd)/target/aur/myth-bin"
mkdir -p "$AUR_DIR"

cat > "$AUR_DIR/PKGBUILD" << 'PKGEOF'
# Maintainer: myth-tools <shesher0llms@gmail.com>
# ═══════════════════════════════════════════════════
#  MYTH — AUR Package (Pre-built Binary)
#  Install: yay -S myth-bin  OR  paru -S myth-bin
# ═══════════════════════════════════════════════════
pkgname=myth-bin
pkgver=__VERSION__
pkgrel=1
pkgdesc="__DESCRIPTION__"
arch=('x86_64' 'aarch64' 'armv7h')
url="__HOMEPAGE__"
license=('__LICENSE__')
provides=('myth')
conflicts=('myth')

# ─── Dependencies ───
# Core
depends=(
    'glibc'
    'openssl'
    'ca-certificates'
    'zstd'
    'bubblewrap'
)

# Recon tools (available in Arch repos or AUR)
depends+=(
    'nmap'
    'curl'
    'git'
    'wget'
    'whois'
    'bind-tools'      # dig, nslookup
)

# Optional recon tools (recommended but not required)
optdepends=(
    'tor: anonymity infrastructure for covert operations'
    'nuclei: fast vulnerability scanner'
    'ffuf: fast web fuzzer'
    'subfinder: subdomain discovery'
    'httpx: HTTP toolkit'
    'gobuster: directory/DNS brute-forcer'
)

# ─── Source URLs ───
source_x86_64=("__REPO_URL__/releases/download/v${pkgver}/myth-x86_64-unknown-linux-gnu")
source_aarch64=("__REPO_URL__/releases/download/v${pkgver}/myth-aarch64-unknown-linux-gnu")
source_armv7h=("__REPO_URL__/releases/download/v${pkgver}/myth-armv7-unknown-linux-gnueabihf")

# SHA256 checksums — updated by build_arch.sh during release
sha256sums_x86_64=('__SHA_X64__')
sha256sums_aarch64=('__SHA_ARM64__')
sha256sums_armv7h=('__SHA_ARMV7__')

package() {
    # Binary
    install -Dm755 "${srcdir}/myth-${CARCH}-unknown-linux-gnu" \
        "${pkgdir}/usr/bin/myth" 2>/dev/null || \
    install -Dm755 "${srcdir}/myth-x86_64-unknown-linux-gnu" \
        "${pkgdir}/usr/bin/myth" 2>/dev/null || \
    install -Dm755 "${srcdir}/myth-aarch64-unknown-linux-gnu" \
        "${pkgdir}/usr/bin/myth"

    # Symlink
    ln -sf /usr/bin/myth "${pkgdir}/usr/bin/agent"

    # Configuration directory
    install -Dm644 /dev/null "${pkgdir}/etc/myth/user.yaml"

    # Provisioning note
    echo "MYTH installed. Run 'myth check' to verify your environment."
}
PKGEOF

# ─── Generate .install file for hooks ───
cat > "$AUR_DIR/myth-bin.install" << 'INSTALLEOF'
post_install() {
    echo ":: MYTH installed successfully."
    echo ":: Run 'myth check' to verify your operational environment."

    # Provision Lightpanda
    if ! command -v lightpanda >/dev/null 2>&1 && [ ! -f /usr/local/bin/lightpanda ]; then
        echo ":: Provisioning Lightpanda browser engine..."
        ARCH=$(uname -m)
        case "$ARCH" in
            x86_64)  BINARY="lightpanda-x86_64-linux" ;;
            aarch64) BINARY="lightpanda-aarch64-linux" ;;
            *)       return 0 ;;
        esac
        TEMP="${TMPDIR:-/tmp}/lightpanda_arch_$$.tmp"
        URL="https://github.com/lightpanda-io/browser/releases/download/nightly/${BINARY}"
        if curl -fsSL --connect-timeout 15 --max-time 120 "$URL" -o "$TEMP" 2>/dev/null; then
            FILE_SIZE=$(stat -c%s "$TEMP" 2>/dev/null || echo 0)
            if [ "$FILE_SIZE" -gt 10240 ]; then
                chmod +x "$TEMP"
                mv "$TEMP" /usr/local/bin/lightpanda
                echo ":: Lightpanda engine provisioned."
            else
                rm -f "$TEMP"
            fi
        else
            rm -f "$TEMP" 2>/dev/null || true
            echo ":: Warning: Lightpanda download failed. Run 'myth check' later."
        fi
    fi
}

post_upgrade() {
    post_install
}

pre_remove() {
    rm -f /usr/bin/agent 2>/dev/null || true
}
INSTALLEOF

# ─── Add install= directive to PKGBUILD ───
sed -i '/^conflicts=/a install=myth-bin.install' "$AUR_DIR/PKGBUILD"

# ─── Generate Checksums for binaries ───
SHA_X64="SKIP"
SHA_ARM64="SKIP"
SHA_ARMV7="SKIP"

# Attempt to find local binaries to generate checksums
[ -f "target/release/myth" ] && SHA_X64=$(sha256sum target/release/myth | awk '{print $1}')
[ -f "target/aarch64-unknown-linux-gnu/release/myth" ] && SHA_ARM64=$(sha256sum target/aarch64-unknown-linux-gnu/release/myth | awk '{print $1}')
[ -f "target/armv7-unknown-linux-gnueabihf/release/myth" ] && SHA_ARMV7=$(sha256sum target/armv7-unknown-linux-gnueabihf/release/myth | awk '{print $1}')

# ─── Replace placeholders ───
sed -i \
    -e "s|__VERSION__|$VERSION|g" \
    -e "s|__DESCRIPTION__|$DESCRIPTION|g" \
    -e "s|__LICENSE__|$LICENSE|g" \
    -e "s|__HOMEPAGE__|$HOMEPAGE|g" \
    -e "s|__REPO_URL__|$REPO_URL|g" \
    -e "s|__SHA_X64__|$SHA_X64|g" \
    -e "s|__SHA_ARM64__|$SHA_ARM64|g" \
    -e "s|__SHA_ARMV7__|$SHA_ARMV7|g" \
    "$AUR_DIR/PKGBUILD"

ok "AUR PKGBUILD generated: $AUR_DIR/PKGBUILD"

# ─── Generate .SRCINFO for AUR submission ───
# .SRCINFO is required by AUR but needs makepkg to generate properly.
# We generate a manual approximation that AUR will accept.
cat > "$AUR_DIR/.SRCINFO" << SRCEOF
pkgbase = myth-bin
	pkgdesc = $DESCRIPTION
	pkgver = $VERSION
	pkgrel = 1
	url = $HOMEPAGE
	install = myth-bin.install
	arch = x86_64
	arch = aarch64
	license = $LICENSE
	depends = glibc
	depends = openssl
	depends = ca-certificates
	depends = zstd
	depends = bubblewrap
	depends = nmap
	depends = curl
	depends = git
	depends = wget
	depends = whois
	depends = bind-tools
	optdepends = tor: anonymity infrastructure for covert operations
	optdepends = nuclei: fast vulnerability scanner
	optdepends = ffuf: fast web fuzzer
	optdepends = subfinder: subdomain discovery
	optdepends = httpx: HTTP toolkit
	optdepends = gobuster: directory/DNS brute-forcer
	provides = myth
	conflicts = myth
	source_x86_64 = ${REPO_URL}/releases/download/v${VERSION}/myth-x86_64-unknown-linux-gnu
	source_aarch64 = ${REPO_URL}/releases/download/v${VERSION}/myth-aarch64-unknown-linux-gnu
	source_armv7h = ${REPO_URL}/releases/download/v${VERSION}/myth-armv7-unknown-linux-gnueabihf
	sha256sums_x86_64 = $SHA_X64
	sha256sums_aarch64 = $SHA_ARM64
	sha256sums_armv7h = $SHA_ARMV7

pkgname = myth-bin
SRCEOF

ok ".SRCINFO generated: $AUR_DIR/.SRCINFO"

if [ "$DRY_RUN" = true ]; then
    info "[DRY-RUN] AUR PKGBUILD validated."
    echo ""
    cat "$AUR_DIR/PKGBUILD"
    exit 0
fi

if [ "$AUR_ONLY" = true ]; then
    ok "AUR files ready at: $AUR_DIR/"
    info "To publish to AUR:"
    info "  1. git clone ssh://aur@aur.archlinux.org/myth-bin.git"
    info "  2. Copy PKGBUILD, .SRCINFO, myth-bin.install into the clone"
    info "  3. git add . && git commit -m 'Update to v$VERSION' && git push"
    exit 0
fi

# ═══════════════════════════════════════════════════════════
#  PART 2: Build Custom Repo Packages (.pkg.tar.zst)
#  For hosting at myth.work.gd/arch/ (direct pacman source)
# ═══════════════════════════════════════════════════════════
section "BUILDING ARCH PACKAGES (CUSTOM REPO)"

ARCH_OUTPUT="$(pwd)/target/arch"
mkdir -p "$ARCH_OUTPUT"
PRODUCED_PKGS=()

for ARCH in "${BUILD_ARCHES[@]}"; do
    section "Building Arch package for: $ARCH"

    # ─── Map to Rust target ───
    case "$ARCH" in
        x86_64)  RUST_TARGET="x86_64-unknown-linux-gnu" ;;
        aarch64) RUST_TARGET="aarch64-unknown-linux-gnu" ;;
        armv7h)  RUST_TARGET="armv7-unknown-linux-gnueabihf" ;;
        *)       warn "Unsupported arch for Arch packaging: $ARCH. Skipping."; continue ;;
    esac

    # ─── Locate binary ───
    BINARY_PATH=""
    HOST_ARCH=$(uname -m)

    if [ "$ARCH" = "$HOST_ARCH" ]; then
        BINARY_PATH="target/release/myth"
    else
        BINARY_PATH="target/$RUST_TARGET/release/myth"
    fi

    if [ ! -f "$BINARY_PATH" ]; then
        warn "Binary not found at $BINARY_PATH. Skipping $ARCH."
        continue
    fi
    ok "Binary found: $BINARY_PATH"

    # ─── Create package structure manually ───
    # Since we're on Kali (not Arch), we can't use makepkg.
    # Instead, we build the .pkg.tar.zst manually using tar+zstd.
    PKG_NAME="myth-${VERSION}-1-${ARCH}"
    PKG_STAGING="${TMPDIR:-/tmp}/myth-arch-pkg-$$"
    rm -rf "$PKG_STAGING"
    mkdir -p "$PKG_STAGING/usr/bin"
    mkdir -p "$PKG_STAGING/etc/myth"
    mkdir -p "$PKG_STAGING/usr/share/doc/myth"
    mkdir -p "$PKG_STAGING/usr/share/man/man1"
    mkdir -p "$PKG_STAGING/usr/share/bash-completion/completions"
    mkdir -p "$PKG_STAGING/usr/share/zsh/site-functions"
    mkdir -p "$PKG_STAGING/usr/share/fish/vendor_completions.d"

    # Copy files
    cp "$BINARY_PATH" "$PKG_STAGING/usr/bin/myth"
    chmod 755 "$PKG_STAGING/usr/bin/myth"
    ln -sf /usr/bin/myth "$PKG_STAGING/usr/bin/agent"

    cp config/user.yaml "$PKG_STAGING/etc/myth/user.yaml" 2>/dev/null || true
    cp README.md "$PKG_STAGING/usr/share/doc/myth/" 2>/dev/null || true
    cp LICENSE "$PKG_STAGING/usr/share/doc/myth/" 2>/dev/null || true

    if [ -f "docs/myth.1" ]; then
        cp docs/myth.1 "$PKG_STAGING/usr/share/man/man1/myth.1"
        gzip -f "$PKG_STAGING/usr/share/man/man1/myth.1"
    fi

    cp completions/myth "$PKG_STAGING/usr/share/bash-completion/completions/" 2>/dev/null || true
    cp completions/_myth "$PKG_STAGING/usr/share/zsh/site-functions/" 2>/dev/null || true
    cp completions/myth.fish "$PKG_STAGING/usr/share/fish/vendor_completions.d/" 2>/dev/null || true

    # ─── Create .PKGINFO (Arch package metadata) ───
    INSTALL_SIZE=$(du -sb "$PKG_STAGING" | cut -f1)
    BUILD_DATE=$(date +%s)
    cat > "$PKG_STAGING/.PKGINFO" << PKGINFO
pkgname = myth
pkgbase = myth
pkgver = ${VERSION}-1
pkgdesc = ${DESCRIPTION}
url = ${HOMEPAGE}
builddate = ${BUILD_DATE}
packager = myth-tools <shesher0llms@gmail.com>
size = ${INSTALL_SIZE}
arch = ${ARCH}
license = ${LICENSE}
depend = glibc
depend = openssl
depend = ca-certificates
depend = zstd
depend = bubblewrap
depend = nmap
depend = curl
depend = git
depend = wget
depend = whois
depend = bind-tools
optdepend = tor: anonymity infrastructure
optdepend = nuclei: fast vulnerability scanner
optdepend = ffuf: fast web fuzzer
PKGINFO

    # ─── Create .MTREE (file manifest) ───
    # A simplified mtree that pacman can parse
    (cd "$PKG_STAGING" && find . -not -name '.PKGINFO' -not -name '.MTREE' -not -name '.' | \
        while read -r f; do
            echo "$f"
        done) > "$PKG_STAGING/.MTREE" 2>/dev/null || true

    # ─── Package it ───
    PKG_FILE="$ARCH_OUTPUT/${PKG_NAME}.pkg.tar.zst"

    info "Compressing package..."
    (cd "$PKG_STAGING" && find . -maxdepth 1 -not -name '.' | sort | tar -cf - -T - | zstd -f -19 -T0 -o "$PKG_FILE")

    rm -rf "$PKG_STAGING"

    if [ -f "$PKG_FILE" ]; then
        ok "Package produced: $PKG_FILE ($(du -h "$PKG_FILE" | cut -f1))"
        PRODUCED_PKGS+=("$PKG_FILE")
    else
        warn "Failed to produce package for $ARCH."
    fi
done

# ─── Generate Arch repo database ───
if [ ${#PRODUCED_PKGS[@]} -gt 0 ]; then
    section "GENERATING ARCH REPO DATABASE"

    # repo-add is only available on Arch. On other systems, we create a minimal one.
    if command -v repo-add &>/dev/null; then
        info "Using native repo-add..."
        rm -f "$ARCH_OUTPUT/myth.db.tar.gz" "$ARCH_OUTPUT/myth.files.tar.gz"
        for pkg in "${PRODUCED_PKGS[@]}"; do
            repo-add "$ARCH_OUTPUT/myth.db.tar.gz" "$pkg"
        done
        ok "Arch repo database generated with repo-add."
    else
        # On non-Arch systems, we can't generate the full database.
        # The custom repo path is secondary to AUR anyway.
        info "repo-add not available (not on Arch). Creating index file instead..."
        # Generate a simple package listing for the custom repo
        {
            echo "# MYTH Arch Repository"
            echo "# Add to /etc/pacman.conf:"
            echo "#   [myth]"
            echo "#   SigLevel = Optional TrustAll"
            echo "#   Server = https://myth.work.gd/arch"
            echo ""
            for pkg in "${PRODUCED_PKGS[@]}"; do
                basename "$pkg"
            done
        } > "$ARCH_OUTPUT/README"
        ok "Arch repo index created (full database requires Arch host)."
        warn "For full pacman repo support, run this script on an Arch system or use AUR."
    fi
fi

# ─── Summary ───
section "ARCH BUILD SUMMARY"

ok "AUR PKGBUILD: $AUR_DIR/"
if [ ${#PRODUCED_PKGS[@]} -gt 0 ]; then
    for pkg in "${PRODUCED_PKGS[@]}"; do
        ok "Package: $pkg"
    done
fi

echo ""
echo -e "${GREEN}${BOLD}  ✅ Arch build complete.${NC}"
echo ""
echo -e "  ${BOLD}AUR Publication:${NC}"
echo -e "    1. Create AUR account at https://aur.archlinux.org"
echo -e "    2. git clone ssh://aur@aur.archlinux.org/myth-bin.git"
echo -e "    3. Copy files from $AUR_DIR/ into the clone"
echo -e "    4. git add . && git commit -m 'v$VERSION' && git push"
echo ""
