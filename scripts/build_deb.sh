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

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -en "${YELLOW}⚠  [WARN] ${NC} $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ DEBIAN TACTICAL PACKAGE BUILDER ]${NC}"
echo -e "  ${BOLD}Initiating package construction...${NC}\n"
require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}

require_command cargo

echo -e "${BOLD}Building MYTH .deb package...${NC}"
echo ""

if ! cargo deb --version &>/dev/null; then
    info "Installing cargo-deb..."
    cargo install cargo-deb || err "Failed to install cargo-deb. Ensure you have network access."
    ok "cargo-deb installed"
fi

# ─── Prep ───
rm -rf target/debian
mkdir -p target/debian

# ─── Build Release ───
info "Building release binary..."
cargo build --release
ok "Release binary built: $(du -h target/release/myth | cut -f1)"

# ─── Build .deb ───
info "Packaging .deb..."
if [ ! -f "Cargo.toml" ]; then
    err "Cargo.toml not found in the project root."
fi
if ! grep -q "\[package.metadata.deb\]" Cargo.toml; then
    err "Missing [package.metadata.deb] section in Cargo.toml. Cannot build package."
fi
cargo deb --no-build
ok ".deb package created"

# ─── Show result ───
DEB_FILE=$(ls -t target/debian/*.deb 2>/dev/null | head -1)
if [ -n "$DEB_FILE" ]; then
    echo ""
    echo -e "${GREEN}${BOLD}  ✅ Package built: $DEB_FILE${NC}"
    echo -e "  Size: $(du -h "$DEB_FILE" | cut -f1)"
    echo ""
    echo -e "  Install with:  ${BOLD}sudo dpkg -i $DEB_FILE${NC}"
    echo -e "  Remove with:   ${BOLD}sudo dpkg -r myth${NC}"
else
    err "No .deb file found in target/debian/"
fi
