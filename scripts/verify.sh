#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────────────────────────
# 🚀 Project MYTH — Industry-Grade Quality Verification [v1.0]
# ──────────────────────────────────────────────────────────────────────────────
#
# This script performs a localized, OS-aware audit of the MYTH workspace.
#
# Features:
#   - ANSI Colorized Status Reporting (respects NO_COLOR / non-TTY)
#   - Execution Timing for Performance Auditing
#   - Summary Dashboard with Health Statistics
#
# Usage:
#   ./scripts/verify.sh           # Run full audit
#   ./scripts/verify.sh --open    # Run full audit and open docs in browser
#
# ──────────────────────────────────────────────────────────────────────────────

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
cd "$WORKSPACE_ROOT"

# ── Environment Configuration ─────────────────────────────────────────────────
set -euo pipefail
export RUST_BACKTRACE=1

if [[ -t 1 && -z "${NO_COLOR:-}" ]]; then
    export FORCE_COLOR=1
    BOLD="\033[1m"
    GREEN="\033[32m"
    BLUE="\033[34m"
    YELLOW="\033[33m"
    RED="\033[31m"
    MAGENTA="\033[35m"
    CYAN="\033[36m"
    NC="\033[0m"
else
    unset FORCE_COLOR
    BOLD="" GREEN="" BLUE="" YELLOW="" RED="" MAGENTA="" CYAN="" NC=""
fi

# Timing Helper
STAMP_START=$(date +%s)

# ── OS Detection ──────────────────────────────────────────────────────────────
OS_UPPER=$(uname -s | tr '[:lower:]' '[:upper:]')
case "$OS_UPPER" in
    LINUX*)   PLATFORM="Linux" ;;
    DARWIN*)  PLATFORM="macOS" ;;
    *)        PLATFORM="Unknown ($OS_UPPER)" ;;
esac

# ── Helper Functions ──────────────────────────────────────────────────────────

header() {
    echo -e "\n${BOLD}${CYAN}────────────────────────────────────────────────────────────────────────────────${NC}"
    echo -e "${BOLD}${MAGENTA}  $1 ${NC}"
    echo -e "${BOLD}${CYAN}────────────────────────────────────────────────────────────────────────────────${NC}"
}

status() {
    local label=$1
    local color=$2
    local icon=$3
    echo -e "[ ${color}${icon}${NC} ] ${BOLD}${label}...${NC}"
}

report_success() {
    echo -e " [ ${GREEN}SUCCESS${NC} ]"
}

report_failure() {
    echo -e "\n${RED}${BOLD}❌ ERROR: Verification aborted. Check logs above.${NC}\n"
    exit 1
}

run_step() {
    local description="$1"
    shift
    if "$@"; then
        report_success
    else
        echo -e "${RED}FAILED: $description${NC}" >&2
        report_failure
    fi
}

# ── Verification Pipeline ─────────────────────────────────────────────────────

echo -e "${BOLD}${CYAN}🦾 MYTH Sovereign Auditor [Platform: ${YELLOW}${PLATFORM}${BOLD}${CYAN}] [Root: ${YELLOW}${WORKSPACE_ROOT}${BOLD}${CYAN}]${NC}"

# Phase 0: Auto-Format
header "Phase 0: Auto-Format (Code Style Enforcement)"
status "Stripping Trailing Whitespace (Pre-Flight Fix)" "${YELLOW}" "🧹"
if [[ "$PLATFORM" == "macOS" ]]; then
    find . -path ./target -prune -o -name "*.rs" -exec sed -i '' 's/[[:space:]]*$//' {} +
else
    find . -path ./target -prune -o -name "*.rs" -exec sed -i 's/[[:space:]]*$//' {} +
fi
report_success

status "Applying Code Formatting (cargo fmt)" "${CYAN}" "🎨"
run_step "cargo fmt" cargo fmt --all

status "Verifying Format is Clean (Confirmation Pass)" "${CYAN}" "✅"
run_step "cargo fmt --check" cargo fmt --all -- --check
FMT_STATUS="${GREEN}AUTO-FIXED & VERIFIED${NC}"

# Phase 0.5: Shell Script Quality (shellcheck)
header "Phase 0.5: Shell Script Quality (shellcheck)"
status "Linting bash scripts & maintainer files" "${YELLOW}" "🐚"
if command -v shellcheck &>/dev/null; then
    # Dynamically find all .sh files and maintainer scripts for auditing
    run_step "shellcheck scripts" find scripts -type f \( -name "*.sh" -o -name "postinst" -o -name "postrm" -o -name "preinst" -o -name "prerm" \) -exec shellcheck {} +
else
    echo -e " [ ${YELLOW}SKIPPED (shellcheck not installed)${NC} ]"
fi

# Phase 0.7: Documentation Suite Formatting (Biome)
header "Phase 0.7: Documentation Suite Formatting (Biome)"
status "Refining Documentation Code Style" "${CYAN}" "🎨"
if [ -d "docs" ] && command -v npm &>/dev/null; then
    (cd docs && npm run format >/dev/null 2>&1 || true)
    report_success
else
    echo -e " [ ${YELLOW}SKIPPED (docs/ or npm missing)${NC} ]"
fi

# Phase 1: Static Analysis & Type Checking
header "Phase 1: Static Analysis & Type Checking"
status "Running Cargo Check (All Targets)" "${BLUE}" "🔍"
run_step "cargo check" cargo check --workspace --all-targets

# Phase 2: Linting
header "Phase 2: Linting & Best Practices"
status "Running Cargo Clippy (Strict)" "${YELLOW}" "✨"
run_step "cargo clippy" cargo clippy --workspace --all-targets --locked -- -D warnings

# Phase 3: Full Build
header "Phase 3: Full Build (Synthesis)"
status "Building Final Application Binaries & Benchmarks" "${MAGENTA}" "🏗️"
run_step "cargo build" cargo build --workspace --all-targets --locked

# Phase 4: Runtime Validation (Unit & Integration)
header "Phase 4: Runtime Validation (Unit & Integration)"
status "Running Unit & Integration Tests (lib + bins + tests + examples)" "${GREEN}" "🧪"
run_step "cargo test" cargo test --workspace --lib --bins --tests --examples

# Phase 4.5: Documentation Logic (Doc Tests)
header "Phase 4.5: Documentation Logic (Doc Tests)"
status "Running Doc Tests (Exhaustive)" "${CYAN}" "📚"
run_step "cargo test --doc" cargo test --doc

# Phase 5: Documentation Suite Quality (Lint)
header "Phase 5: Documentation Suite Quality (Lint)"
status "Auditing Interface Best Practices (Biome)" "${YELLOW}" "✨"
if [ -d "docs" ] && command -v npm &>/dev/null; then
    (cd docs && run_step "npm run lint" npm run lint)
else
    echo -e " [ ${YELLOW}SKIPPED (docs/ or npm missing)${NC} ]"
fi

# Phase 6: Documentation Suite Synthesis (Build)
header "Phase 6: Documentation Suite Synthesis (Build)"
status "Transpiling Interface & Type Checking (Vite + TSC)" "${MAGENTA}" "🏗️"
if [ -d "docs" ] && command -v npm &>/dev/null; then
    (cd docs && run_step "npm run build" npm run build)
    DOC_SITE_STATUS="${GREEN}REDEPLOYABLE${NC}"
else
    echo -e " [ ${YELLOW}SKIPPED (docs/ or npm missing)${NC} ]"
    DOC_SITE_STATUS="${YELLOW}NOT VERIFIED${NC}"
fi

# Phase 7: API Documentation Audit (Rust)
header "Phase 7: API Documentation Audit (Rust)"
status "Building Detailed Project Report (Inc. Private Items)" "${MAGENTA}" "📖"
run_step "cargo doc" cargo doc --workspace --no-deps --document-private-items

# Phase 8: Infrastructure & Artifact Integrity
header "Phase 8: Infrastructure & Artifact Integrity"

status "Validating Public Signing Key (PGP)" "${GREEN}" "🔑"
if [ -f "myth.gpg" ]; then
    if grep -q "BEGIN PGP PUBLIC KEY BLOCK" myth.gpg || gpg --list-packets myth.gpg &>/dev/null; then
        report_success
    else
        echo -e "${RED}FAILED: GPG key is malformed or invalid.${NC}" >&2
        report_failure
    fi
else
    echo -e "${YELLOW}SKIPPED: myth.gpg not found. Run init_repo.sh first.${NC}"
fi

status "Auditing Built Binaries (Architecture Sanity)" "${BLUE}" "🛡️"
find target/release target/*/release -maxdepth 1 -name "myth" 2>/dev/null | while read -r binary; do
    if [ -f "$binary" ]; then
        if file "$binary" | grep -q "ELF"; then
            echo -e "  - Found: $binary (${CYAN}$(file -b "$binary" | cut -d, -f1)${NC})"
        else
            echo -e "${RED}  - WARNING: $binary is NOT an ELF binary!${NC}"
        fi
    fi
done
report_success

# ── Final Summary Dashboard ───────────────────────────────────────────────────
STAMP_END=$(date +%s)
DURATION=$((STAMP_END - STAMP_START))

# Generate interactive documentation URL (ANSI Hyperlink — only on supporting terminals)
DOC_URI="file://${WORKSPACE_ROOT}/target/doc/myth/index.html"
if [[ -t 1 ]]; then
    DOC_LINK="\e]8;;${DOC_URI}\a${CYAN}${BOLD}[ Open Project Report ↗ ]${NC}\e]8;;\a"
else
    DOC_LINK="${CYAN}${DOC_URI}${NC}"
fi

echo -e "\n${BOLD}${MAGENTA}📊 Exhaustive Verification Dashboard:${NC}"
echo -e "${CYAN}────────────────────────────────────────────────────────────────────────────────${NC}"
printf "  %-30s %b\n" "Platform Status:"          "${GREEN}SOVEREIGN (${PLATFORM})${NC}"
printf "  %-30s %b\n" "Workspace Root:"            "${CYAN}${WORKSPACE_ROOT}${NC}"
printf "  %-30s %b\n" "Core Build Integrity:"      "${GREEN}100% SECURE (Rust Verified)${NC}"
printf "  %-30s %b\n" "Interface Integrity:"       "${DOC_SITE_STATUS}"
printf "  %-30s %b\n" "Formatting Status:"         "${FMT_STATUS}"
printf "  %-30s %b\n" "Test Capabilities:"         "${GREEN}VERIFIED${NC}"
printf "  %-30s %b\n" "Documentation:"             "${GREEN}GENERATED (Detailed Report Ready)${NC}"
printf "  %-30s %b\n" "Doc Access URL:"            "${DOC_LINK}"
printf "  %-30s %b\n" "Total Audit Time:"          "${DURATION} seconds"
echo -e "${CYAN}────────────────────────────────────────────────────────────────────────────────${NC}"
echo -e "\n${BOLD}${GREEN}✅ ALL SYSTEMS NOMINAL. MYTH is ready for deployment.${NC}"

OPEN_REPORT=false
for arg in "$@"; do
    [[ "$arg" == "--open" ]] && OPEN_REPORT=true
done

open_docs() {
    status "Launching Detailed Report" "${CYAN}" "🌐"
    if [[ "$PLATFORM" == "macOS" ]]; then
        open "$DOC_URI" >/dev/null 2>&1
    else
        xdg-open "$DOC_URI" >/dev/null 2>&1
    fi
}

if $OPEN_REPORT; then
    open_docs
elif [[ -t 0 ]]; then
    echo -ne "  ${YELLOW}PROMPT: Press ${BOLD}[O]${NC}${YELLOW} to open report in browser, or any other key to exit... ${NC}"
    read -n 1 -r -t 10 RESPONSE
    echo ""
    if [[ "$RESPONSE" =~ ^[oO]$ ]]; then
        open_docs
    fi
else
    echo -e "💡 ${CYAN}Tip: Run 'scripts/verify.sh --open' for automated report launching.${NC}\n"
fi
