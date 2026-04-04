#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
# MYTH — Multi-Ecosystem Master Distribution Engine
# ─────────────────────────────────────────────────────────
# Handles the orchestration and deployment of MYTH across
# all 14 execution variants globally (NPM, PyPI, Docker, Snap).
# ═══════════════════════════════════════════════════════════
set -euo pipefail

# ANSI Colors for Terminal UI
RED='\033[1;31m'
GREEN='\033[1;32m'
CYAN='\033[1;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
RESET='\033[0m'

warn() { echo -e "${YELLOW}⚠  [WARN]  $1${RESET}"; }
GLOBAL_FAIL=0

DRY_RUN=false
PUBLISH_NPM=false
PUBLISH_PYPI=false
PUBLISH_DOCKER=false
PUBLISH_SNAP=false
PUBLISH_GITHUB=false

# Global Status Tracking
STATUS_NPM="${YELLOW}SKIPPED${RESET}"
STATUS_PYPI="${YELLOW}SKIPPED${RESET}"
STATUS_DOCKER="${YELLOW}SKIPPED${RESET}"
STATUS_SNAP="${YELLOW}SKIPPED${RESET}"
STATUS_GITHUB="${YELLOW}SKIPPED${RESET}"

LOG_FILE="logs/distribution.log"

# Trace execution to an audit log
log_mission() {
    local clean_msg
    clean_msg=$(echo -e "$1" | sed 's/\x1B\[[0-9;]*[a-zA-Z]//g')
    echo "$(date '+%Y-%m-%d %H:%M:%S') | $clean_msg" >> "$LOG_FILE"
}

cd "$(dirname "${BASH_SOURCE[0]}")/.."

# ─── Log directory (must exist before first log_mission call) ───
mkdir -p logs

# Root check
if [[ ! -d "package_runners" ]]; then
    echo -e "${RED}✘ [FATAL] Must be executed from the root of the MYTH repository.${RESET}"
    exit 1
fi

# Extract Version from Cargo.toml (Source of Truth)
[ -f Cargo.toml ] || { echo -e "${RED}✘ [FATAL] Cargo.toml not found. Run from the MYTH repository root.${RESET}"; exit 1; }
VERSION=$(grep "^version[[:space:]]*=[[:space:]]*\"" Cargo.toml | head -n1 | cut -d'"' -f2)
[ -z "$VERSION" ] && { echo -e "${RED}✘ [FATAL] Could not extract version from Cargo.toml.${RESET}"; exit 1; }

# Parsing Arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --dry-run) DRY_RUN=true ;;
        --all) PUBLISH_NPM=true; PUBLISH_PYPI=true; PUBLISH_DOCKER=true; PUBLISH_SNAP=true; PUBLISH_GITHUB=true ;;
        --npm) PUBLISH_NPM=true ;;
        --pypi) PUBLISH_PYPI=true ;;
        --docker) PUBLISH_DOCKER=true ;;
        --snap) PUBLISH_SNAP=true ;;
        --github) PUBLISH_GITHUB=true ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

if [[ "$PUBLISH_NPM" == false && "$PUBLISH_PYPI" == false && "$PUBLISH_DOCKER" == false && "$PUBLISH_SNAP" == false && "$PUBLISH_GITHUB" == false ]]; then
    echo -e "${YELLOW}Usage: ./scripts/distribute.sh [--dry-run] [--all | --npm | --pypi | --docker | --snap | --github]${RESET}"
    echo -e "Example: ./scripts/distribute.sh --dry-run --all"
    exit 0
fi

# ─── 1. MISSION PRE-FLIGHT AUDIT ───
echo -e "\n${CYAN}1. Neural Core Pre-Flight Audit...${RESET}"

# A. Toolchain Integrity Check
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}✘ [ERROR] Required tool '$1' not found.${RESET}"
        return 1
    fi
    echo -e "${GREEN}✔ [OK] Tool found: $1${RESET}"
    return 0
}

# B. Docker Permission & CMD Resolution
DOCKER_CMD="docker"
if [ "$PUBLISH_DOCKER" = true ]; then
    # First, check if the current user is in the docker group or has root-less access
    if ! docker ps &> /dev/null; then
        if groups | grep -q "docker"; then
             echo -e "${YELLOW}⚠ User is in docker group but socket is not accessible. Permissions may need refresh.${RESET}"
        fi
        echo -e "${YELLOW}⚠ Docker permission denied. Attempting 'sudo docker' elevation...${RESET}"
        DOCKER_CMD="sudo docker"
        if ! $DOCKER_CMD ps &> /dev/null; then
            echo -e "${RED}✘ [ERROR] 'sudo docker' failed. Verify sudoers and Docker daemon status.${RESET}"
            exit 1
        fi
    fi
fi

# C. Registry Authentication Check
check_auth() {
    local vector=$1
    
    case $vector in
        NPM)
            if [[ -z "${NODE_AUTH_TOKEN:-}" ]]; then echo -e "${YELLOW}⚠ NODE_AUTH_TOKEN missing.${RESET}"; fi
            ;;
        PyPI)
            if [[ -z "${UV_PUBLISH_TOKEN:-}" ]]; then echo -e "${YELLOW}⚠ UV_PUBLISH_TOKEN missing.${RESET}"; fi
            ;;
        OCI)
            # Check config.json for an existing GHCR credential
            if ! grep -q "ghcr.io" "${HOME}/.docker/config.json" 2>/dev/null; then
                echo -e "${YELLOW}⚠ Not logged into ghcr.io. Registry access may fail.${RESET}"
                echo -e "   Run: 'docker login ghcr.io' with a GitHub PAT."
            else
                # Proactive connectivity check
                if ! docker pull ghcr.io/myth-tools/myth:latest --dry-run &>/dev/null && \
                   ! docker pull ghcr.io/myth-tools/myth:latest --quiet &>/dev/null; then
                    warn "GHCR probe failed. Publication may require elevated permissions or a fresh login."
                else
                    echo -e "${GREEN}✔ [OK] GHCR access verified.${RESET}"
                fi
            fi
            ;;
        GitHub)
            if ! gh auth status &> /dev/null; then echo -e "${YELLOW}⚠ Not logged into GitHub CLI.${RESET}"; fi
            ;;
    esac
    return 0
}

[[ "$PUBLISH_NPM" == true ]] && check_tool "npm" && check_auth "NPM"
[[ "$PUBLISH_PYPI" == true ]] && check_tool "maturin" && check_tool "uv" && check_auth "PyPI"
[[ "$PUBLISH_DOCKER" == true ]] && check_tool "docker" && check_auth "OCI"
[[ "$PUBLISH_SNAP" == true ]] && check_tool "snapcraft"
[[ "$PUBLISH_GITHUB" == true ]] && check_tool "gh" && check_auth "GitHub"

echo -e "${GREEN}✔ Mission Pre-flight Audit Complete.${RESET}"


echo -e "${CYAN}⚡ MYTH DISTRIBUTION ENGINE ENGAGED${RESET}"
if [ "$DRY_RUN" = true ]; then 
    echo -e "${YELLOW}[DRY-RUN] Execution disabled - validating pipeline parameters...${RESET}"
else
    # ─── PRODUCTION SAFETY LOCK ───
    echo -e "${RED}${BOLD}⚠ PRODUCTION MISSION ACTIVATED${RESET}"
    echo -e "You are about to publish MYTH to global registries."
    read -r -p "Confirm mission authorization (type 'CONFIRM'): " confirm
    if [[ "$confirm" != "CONFIRM" ]]; then
        echo -e "${YELLOW}Aborted. Mission scrubbed.${RESET}"
        exit 1
    fi
    log_mission "MISSION START: All distributions authorized."
fi

# ─── 1. Metadata Synchronization ───
echo -e "\n${CYAN}1. Synchronizing Build Metadata...${RESET}"
if ! bash scripts/sync-agent-metadata.sh; then
    echo -e "${RED}✘ Synchronization failed. Aborting mission.${RESET}"
    exit 1
fi

# ─── 2. Artifact Fingerprinting (Audit) ───
echo -e "\n${CYAN}2. Auditing Neural Core Fingerprints (SHA256)...${RESET}"
if [[ -f "target/release/myth" ]]; then
    BINARY_HASH=$(sha256sum target/release/myth | cut -d' ' -f1)
    echo -e "   [OK] Core Binary ID: ${GREEN}$BINARY_HASH${RESET}"
    log_mission "SHA256 Core Fingerprint: $BINARY_HASH"
else
    echo -e "${YELLOW}⚠ Native binary not found. Skipping hash generation.${RESET}"
fi

# ─── 3. PyPI (Python) Deployment ───
if [ "$PUBLISH_PYPI" = true ]; then
    echo -e "\n${CYAN}3. Deploying Python Vector (Maturin + UV + PyPI)${RESET}"
    if ! command -v maturin &> /dev/null; then
        echo -e "${RED}✘ Maturin not found. MISSION FAILURE.${RESET}"
        exit 1
    fi
    if ! command -v uv &> /dev/null; then
        echo -e "${RED}✘ uv not found. MISSION FAILURE.${RESET}"
        exit 1
    fi
    
    if [[ -z "${UV_PUBLISH_TOKEN:-}" ]] && [[ "$DRY_RUN" == false ]]; then
        echo -e "${YELLOW}⚠ UV_PUBLISH_TOKEN not detected in environment. Interactive authentication may be required.${RESET}"
    else
        echo -e "${GREEN}✔ UV_PUBLISH_TOKEN detected. Automated publication enabled.${RESET}"
    fi

    if [ "$DRY_RUN" = true ]; then
        echo -e "${GREEN}✔ [DRY-RUN] Maturin build & UV publish simulation success.${RESET}"
        STATUS_PYPI="${CYAN}DRY-RUN${RESET}"
    else
        read -r -p "⚠ Confirm PyPI push for v$VERSION? [y/N]: " confirm_pypi
        if [[ "$confirm_pypi" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            log_mission "DISTRIBUTION: Initiating PyPI release via Maturin and UV..."
            if (cd package_runners && rm -rf ../target/wheels && maturin build --release --out ../target/wheels) && ls target/wheels/*.whl >/dev/null 2>&1 && uv publish target/wheels/*; then
                log_mission "SUCCESS: PyPI release confirmed."
                STATUS_PYPI="${GREEN}SUCCESS${RESET}"
            else
                STATUS_PYPI="${RED}FAILED${RESET}"
                GLOBAL_FAIL=1
            fi
        else
            echo -e "${YELLOW}PyPI distribution deferred by operator.${RESET}"
            STATUS_PYPI="${YELLOW}CANCELLED${RESET}"
        fi
    fi
fi

# ─── 4. NPM (JavaScript) Deployment ───
if [ "$PUBLISH_NPM" = true ]; then
    echo -e "\n${CYAN}4. Deploying JS Vector (Node + NPM)${RESET}"
    if ! command -v npm &> /dev/null; then
        echo -e "${RED}✘ NPM not found. MISSION FAILURE.${RESET}"
        exit 1
    fi
    
    if [[ -z "${NODE_AUTH_TOKEN:-}" ]] && [[ "$DRY_RUN" == false ]]; then
        echo -e "${YELLOW}⚠ NODE_AUTH_TOKEN not detected in environment. Interactive authentication manually required.${RESET}"
    else
        echo -e "${GREEN}✔ NODE_AUTH_TOKEN detected. Automated publication enabled.${RESET}"
    fi
    
    if [ "$DRY_RUN" = true ]; then
        echo -e "${GREEN}✔ [DRY-RUN] NPM publish simulation success.${RESET}"
        STATUS_NPM="${CYAN}DRY-RUN${RESET}"
    else
        read -r -p "⚠ Confirm NPM push for v$VERSION? [y/N]: " confirm_npm
        if [[ "$confirm_npm" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            log_mission "DISTRIBUTION: Initiating NPM release..."
            if (cd package_runners && npm publish --access public); then
                log_mission "SUCCESS: NPM release confirmed."
                STATUS_NPM="${GREEN}SUCCESS${RESET}"
            else
                STATUS_NPM="${RED}FAILED${RESET}"
                GLOBAL_FAIL=1
            fi
        else
            echo -e "${YELLOW}NPM distribution deferred by operator.${RESET}"
            STATUS_NPM="${YELLOW}CANCELLED${RESET}"
        fi
    fi
fi

# ─── 5. Docker (Container) Deployment ───
if [ "$PUBLISH_DOCKER" = true ]; then
    echo -e "\n${CYAN}5. Deploying Container Vector (Docker + GHCR)${RESET}"
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}✘ Docker engine not available. MISSION FAILURE.${RESET}"
        exit 1
    fi
    
    if [[ -n "${GITHUB_PERSONAL_ACCESS_TOKEN:-}" ]] && [[ "$DRY_RUN" == false ]]; then
        echo -e "${GREEN}✔ [OK] GITHUB_PERSONAL_ACCESS_TOKEN detected. Automated GHCR authentication initiated.${RESET}"
        echo "$GITHUB_PERSONAL_ACCESS_TOKEN" | $DOCKER_CMD login ghcr.io -u "myth-tools" --password-stdin &> /dev/null
    fi

    if [ "$DRY_RUN" = true ]; then
        echo -e "${GREEN}✔ [DRY-RUN] Docker build simulation success (Tag: $VERSION).${RESET}"
        STATUS_DOCKER="${CYAN}DRY-RUN${RESET}"
    else
        read -r -p "⚠ Confirm Docker build & push for v$VERSION? [y/N]: " confirm_docker
        if [[ "$confirm_docker" =~ ^([yY][eE][sS]|[yY])$ ]]; then
            log_mission "DISTRIBUTION: Initiating Docker build (v$VERSION)..."
            if $DOCKER_CMD build -t ghcr.io/myth-tools/myth:"$VERSION" -t ghcr.io/myth-tools/myth:latest -f package_runners/Dockerfile . && \
               $DOCKER_CMD push ghcr.io/myth-tools/myth:"$VERSION" && \
               $DOCKER_CMD push ghcr.io/myth-tools/myth:latest; then
                log_mission "SUCCESS: Docker image pushed to GHCR."
                STATUS_DOCKER="${GREEN}SUCCESS${RESET}"
            else
                STATUS_DOCKER="${RED}FAILED${RESET}"
                GLOBAL_FAIL=1
            fi
        else
            echo -e "${YELLOW}Docker distribution deferred by operator.${RESET}"
            STATUS_DOCKER="${YELLOW}CANCELLED${RESET}"
        fi
    fi
fi

# ─── 6. Canonical (Snap) Deployment ───
if [ "$PUBLISH_SNAP" = true ]; then
    echo -e "\n${CYAN}6. Deploying Canonical Vector (Snapcraft)${RESET}"
    if ! command -v snapcraft &> /dev/null; then
        echo -e "${YELLOW}⚠ Snapcraft toolchain not detected. Skipping local build.${RESET}"
    else
        if [ "$DRY_RUN" = true ]; then
            echo -e "${GREEN}✔ [DRY-RUN] Snapcraft simulation success.${RESET}"
            STATUS_SNAP="${CYAN}DRY-RUN${RESET}"
        else
        read -r -p "⚠ Confirm Snapcraft build & push for v$VERSION? [y/N]: " confirm_snap
            if [[ "$confirm_snap" =~ ^([yY][eE][sS]|[yY])$ ]]; then
                log_mission "DISTRIBUTION: Initiating Canonical Snapcraft build..."
                
                if (cd package_runners && snapcraft pack --destructive-mode); then
                    # NOTE: --destructive-mode installs build deps directly on the host system.
                    # This is acceptable for a dedicated CI machine but should not run on a dev workstation.
                    SNAP_FILE=$(find package_runners -maxdepth 1 -name 'myth_*.snap' | head -1)
                    if [ -z "$SNAP_FILE" ]; then
                        STATUS_SNAP="${RED}BUILD FAILED (no .snap found)${RESET}"
                        exit 1
                    fi
                    # Clean up build artifacts immediately
                    (cd package_runners && { snapcraft clean &>/dev/null || rm -rf parts stage prime overlay .craft-state; })
                    if snapcraft upload --release=stable "$SNAP_FILE"; then
                        log_mission "SUCCESS: Snap uploaded to Ubuntu Store."
                        STATUS_SNAP="${GREEN}SUCCESS${RESET}"
                    else
                        STATUS_SNAP="${RED}UPLOAD FAILED${RESET}"
                        GLOBAL_FAIL=1
                    fi
                else
                    STATUS_SNAP="${RED}BUILD FAILED${RESET}"
                    GLOBAL_FAIL=1
                fi
            else
                echo -e "${YELLOW}Snap distribution deferred by operator.${RESET}"
                STATUS_SNAP="${YELLOW}CANCELLED${RESET}"
            fi
        fi
    fi
fi

# ─── 7. GitHub Releases Deployment ───
if [ "$PUBLISH_GITHUB" = true ]; then
    echo -e "\n${CYAN}7. Deploying Binary Vector (GitHub Releases)${RESET}"
    if ! command -v gh &> /dev/null; then
        echo -e "${RED}✘ GitHub CLI (gh) not found. MISSION FAILURE.${RESET}"
        GLOBAL_FAIL=1
    else
        if [ "$DRY_RUN" = true ]; then
            echo -e "${GREEN}✔ [DRY-RUN] GitHub publish simulation success.${RESET}"
            STATUS_GITHUB="${CYAN}DRY-RUN${RESET}"
        else
            read -r -p "⚠ Confirm GitHub Assets push for v$VERSION? [y/N]: " confirm_github
            if [[ "$confirm_github" =~ ^([yY][eE][sS]|[yY])$ ]]; then
                log_mission "DISTRIBUTION: Initiating GitHub Assets release..."
                
                # Ensure tag exists
                if ! git rev-parse "v${VERSION}" >/dev/null 2>&1; then
                    echo -e "${YELLOW}⚠ Tag v${VERSION} not found. Creating local tag...${RESET}"
                    git tag "v${VERSION}" || true
                    git push origin "v${VERSION}" || true
                fi
                
                # Check if release exists, if not create it
                if ! gh release view "v${VERSION}" >/dev/null 2>&1; then
                    gh release create "v${VERSION}" --title "v${VERSION}" --notes "MYTH v${VERSION} Release" --draft
                fi
                
                # Enumerate binaries
                GH_ASSETS=()
                [ -f "target/release/myth" ] && GH_ASSETS+=("target/release/myth#myth-amd64-linux")
                [ -f "target/x86_64-unknown-linux-gnu/release/myth" ] && GH_ASSETS+=("target/x86_64-unknown-linux-gnu/release/myth#myth-x86_64-unknown-linux-gnu")
                [ -f "target/aarch64-unknown-linux-gnu/release/myth" ] && GH_ASSETS+=("target/aarch64-unknown-linux-gnu/release/myth#myth-aarch64-unknown-linux-gnu")
                [ -f "target/armv7-unknown-linux-gnueabihf/release/myth" ] && GH_ASSETS+=("target/armv7-unknown-linux-gnueabihf/release/myth#myth-armv7-unknown-linux-gnueabihf")
                
                if [ ${#GH_ASSETS[@]} -gt 0 ]; then
                    if gh release upload "v${VERSION}" "${GH_ASSETS[@]}" --clobber; then
                        log_mission "SUCCESS: GitHub release confirmed."
                        STATUS_GITHUB="${GREEN}SUCCESS${RESET}"
                    else
                        STATUS_GITHUB="${RED}UPLOAD FAILED${RESET}"
                        GLOBAL_FAIL=1
                    fi
                else
                    warn "No binary assets found to upload to GitHub."
                    STATUS_GITHUB="${YELLOW}NO ASSETS${RESET}"
                fi
            else
                echo -e "${YELLOW}GitHub distribution deferred by operator.${RESET}"
                STATUS_GITHUB="${YELLOW}CANCELLED${RESET}"
            fi
        fi
    fi
fi

# ─── FINAL MISSION SUMMARY ───
echo -e "\n${BOLD}${CYAN}⠿ GLOBAL DISTRIBUTION SUMMARY${RESET}"
echo -e "   Vector: NPM/Bun         - [ $STATUS_NPM ]"
echo -e "   Vector: PyPI/UVX        - [ $STATUS_PYPI ]"
echo -e "   Vector: OCI/Docker      - [ $STATUS_DOCKER ]"
echo -e "   Vector: Snap/Canonical  - [ $STATUS_SNAP ]"
echo -e "   Vector: GitHub Binaries - [ $STATUS_GITHUB ]"

if [ "$GLOBAL_FAIL" -eq 0 ]; then
    echo -e "\n${GREEN}✔ [MISSION SUCCESS] Global Distribution Status Finalized.${RESET}"
    log_mission "MISSION END: Distribution completed successfully."
    exit 0
else
    echo -e "\n${RED}✘ [MISSION FAILED] Some distribution vectors failed.${RESET}"
    log_mission "MISSION END: Distribution finished with errors."
    exit 1
fi

