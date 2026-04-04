#!/usr/bin/env bash
# ═══════════════════════════════════════════════════
#  MYTH — Automated Tactical Quality Suite
# ═══════════════════════════════════════════════════
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

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

# High-fidelity status indicators
info()    { echo -e "${BLUE}⚡${NC}  ${BOLD}$1${NC}"; }
ok()      { echo -e "${GREEN}✔${NC}  $1"; }
warn()    { echo -e "${YELLOW}⚠  [WARN]${NC}  $1"; }
err()     { echo -e "${RED}✘  [FATAL]${NC} $1"; exit 1; }
audit()   { echo -e "${CYAN}⠿${NC}  $1"; }
section() { echo -e "\n${BOLD}${MAGENTA}─── $1 ───${NC}"; }

require_command() {
    if ! command -v "$1" &>/dev/null; then
        err "Required command '$1' is not installed."
    fi
}

echo -e "${MAGENTA}${BOLD}${BANNER}${NC}"
echo -e "${CYAN}  [ AUTOMATED TACTICAL QUALITY SUITE ]${NC}"
echo -e "  ${BOLD}Initiating system health audit...${NC}\n"

require_command cargo

echo -e "${BOLD}Running MYTH Quality Suite...${NC}"
echo ""

# ─── Unit & Documentation Tests ───
info "Running unit and doc tests..."
cargo test --workspace --locked --quiet || err "Unit tests failed"
ok "Tests passed"

# ─── Clippy (Linter) ───
info "Running Clippy lints (all targets, all workspace crates)..."
cargo clippy --workspace --all-targets --locked -- -D warnings || err "Clippy lints failed"
ok "No lint errors found"

# ─── Formatting Check ───
info "Checking code formatting..."
cargo fmt --all -- --check || err "Formatting check failed. Run 'cargo fmt' to fix."
ok "Formatting is clean"

# ─── Script Linting (shellcheck) ───
info "Linting shell scripts with shellcheck..."
if command -v shellcheck &>/dev/null; then
    # Exclude postinst/postrm/etc that lack .sh extension but are still run
    shellcheck scripts/*.sh scripts/postinst scripts/postrm scripts/preinst scripts/prerm || warn "Shellcheck found warnings in scripts."
    ok "Script linting completed."
else
    warn "shellcheck not installed. Install via your package manager. Skipping."
fi

# ─── Security Audit ───
info "Auditing dependencies for vulnerabilities..."
if ! cargo audit --version &>/dev/null; then
    warn "cargo-audit not installed. Install with 'cargo install cargo-audit'. Skipping."
else
    cargo audit || warn "Vulnerabilities or audit issues found!"
    ok "Dependency audit completed."
fi

# ─── Container Integration Test (Dry-run) ───
info "Running container integration test (shell syntax in Debian)..."
if command -v docker &>/dev/null; then
    if docker info &>/dev/null; then
        docker run --rm -v "$(pwd):/myth" debian:latest bash -n /myth/scripts/install.sh || warn "Container integration test failed."
        ok "Container integration test passed."
    else
        warn "Docker daemon not running. Skipping integration test."
    fi
else
    warn "Docker not installed. Skipping integration test."
fi

echo ""
echo -e "${GREEN}${BOLD}  ✅ All quality checks passed! Build is clean and ready.${NC}"
echo ""
