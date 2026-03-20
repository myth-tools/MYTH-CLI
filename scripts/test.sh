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
echo -e "${CYAN}  [ AUTOMATED TACTICAL QUALITY SUITE ]${NC}"
echo -e "  ${BOLD}Initiating system health audit...${NC}\n"
require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}

require_command cargo

echo -e "${BOLD}Running MYTH Quality Suite...${NC}"
echo ""

# ─── Unit & Documentation Tests ───
info "Running unit and doc tests..."
cargo test --workspace --locked --quiet || err "Unit tests failed"
ok "Tests passed"

# ─── Clippy (Linter) ───
info "Running Clippy lints..."
cargo clippy --workspace -- -D warnings || err "Clippy lints failed"
ok "No lint errors found"

# ─── Formatting Check ───
info "Checking code formatting..."
cargo fmt --all -- --check || err "Formatting check failed. Run 'cargo fmt' to fix."
ok "Formatting is clean"

echo ""
echo -e "${GREEN}${BOLD}  ✅ All quality checks passed! Build is clean and ready.${NC}"
echo ""
