#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════
#  MYTH — Global-Scale Release Manifest Generator [v4.0]
#  [!] Industry-Grade: Parallel Processing & Universal Arch Matrix
#  [!] Features: SHA-256 HUD, Terminal Installs, Path Integrity
# ═══════════════════════════════════════════════════════════
set -euo pipefail

TARGET_DIR="target"
OUTPUT_FILE="$TARGET_DIR/versions.json"
LOG_FILE="logs/distribution.log"
PUBLIC_DIR="$HOME/.aptly/public"
TEMP_JSON_DIR=$(mktemp -d "/tmp/myth_manifest.XXXXXX")
mkdir -p "$TARGET_DIR" "$(dirname "$LOG_FILE")"

trap 'rm -rf "$TEMP_JSON_DIR"' EXIT

# Root check
if [[ ! -d "scripts" ]]; then
    echo "✘ [FATAL] Run from the MYTH repository root."
    exit 1
fi

log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') | MANIFEST: $1" >> "$LOG_FILE"
    echo "  ⠿ $1"
}

echo "⚡ Generating Universal Parallel Manifest [v4.0]..."

# 1. Pre-flight Dependency Audit
for tool in jq dpkg-deb rpm sha256sum tar find; do
    if ! command -v "$tool" &>/dev/null; then
        echo "✘ [ERROR] Required tool '$tool' not found. Aborting manifest generation."
        exit 1
    fi
done

# Initialize with empty array
echo "[]" > "$OUTPUT_FILE"

# Helper to find relative path of a file in the public repository
resolve_rel_path() {
    local filename="$1"
    local search_dir="$2"
    local rel_path
    rel_path=$(find "$search_dir" -name "$filename" -print -quit | sed "s|$PUBLIC_DIR/||" || echo "")
    if [ -z "$rel_path" ]; then
        rel_path="$filename"
    fi
    echo "$rel_path"
}

# Helper to format size
format_size() {
    local bytes=$1
    if [ "$bytes" -ge 1048576 ]; then
        echo "$(bc <<< "scale=2; $bytes/1048576") MB"
    else
        echo "$(bc <<< "scale=2; $bytes/1024") KB"
    fi
}

# Advanced Entry Generator (Thread-Safe)
generate_entry() {
    local os="$1" ver="$2" arch="$3" size="$4" date="$5" desc="$6" maint="$7" sect="$8" file="$9" sha="${10}" rel_path="${11}"
    
    # Universal Architecture Matrix (Enhanced "etc")
    local darch="$arch"
    case "$arch" in
        amd64|x86_64)   darch="x64 (64-bit)" ;;
        arm64|aarch64)  darch="ARM64 (v8)" ;;
        armhf|armv7l|armv7h) darch="ARMv7 (32-bit)" ;;
        i386|i686)      darch="x86 (32-bit)" ;;
        riscv64)        darch="RISC-V (64-bit)" ;;
        ppc64le)        darch="PowerPC (64-bit)" ;;
        s390x)          darch="IBM System z (64-bit)" ;;
        loongarch64)    darch="LoongArch (64-bit)" ;;
    esac

    # Platform Intelligence
    local platform="Universal Linux"
    local install_cmd="sudo apt install ./$file" # Default
    if [[ "$os" == "debian" && ("$arch" =~ ^arm) || ("$arch" == "aarch64") ]]; then
        platform="Termux / Kali NetHunter"
        install_cmd="pkg install ./$file"
    elif [[ "$os" == "fedora" ]]; then
        platform="Fedora / RHEL / CentOS"
        install_cmd="sudo dnf install ./$file"
    elif [[ "$os" == "arch" ]]; then
        platform="Arch Linux / Manjaro"
        install_cmd="sudo pacman -U ./$file"
    elif [[ "$os" == "debian" ]]; then
        platform="Debian / Kali / Ubuntu"
        install_cmd="sudo apt install ./$file"
    fi

    local size_human verify_cmd date_human
    size_human=$(format_size "$size")
    verify_cmd="echo \"$sha  $(basename "$rel_path")\" | sha256sum -c"
    date_human=$(date -d "@$date" "+%Y-%m-%d" 2>/dev/null || date -r "$date" "+%Y-%m-%d")

    # Safety: Verify physical path exists in public dir (Robustness check)
    if [ ! -f "$PUBLIC_DIR/$rel_path" ] && [ "$rel_path" != "$file" ]; then
        log "✘ [WARN] Staged path $rel_path not physically found. Link may be broken."
    fi

    # Output to thread-safe temp file
    jq -n \
        --arg os "$os" \
        --arg platform "$platform" \
        --arg ver "$ver" \
        --arg arch "$arch" \
        --arg darch "$darch" \
        --arg size "$size" \
        --arg size_h "$size_human" \
        --arg date "$date" \
        --arg date_h "$date_human" \
        --arg desc "$desc" \
        --arg maint "$maint" \
        --arg sect "$sect" \
        --arg file "$file" \
        --arg path "$rel_path" \
        --arg sha "$sha" \
        --arg vcmd "$verify_cmd" \
        --arg icmd "$install_cmd" \
        '{os: $os, platform: $platform, version: $ver, arch: $arch, display_arch: $darch, size: ($size|tonumber), size_human: $size_h, date: ($date|tonumber), date_human: $date_h, description: $desc, maintainer: $maint, section: $sect, filename: $file, path: $path, sha256: $sha, verify_cmd: $vcmd, install_cmd: $icmd}' \
        > "$TEMP_JSON_DIR/$(printf "%s-%s-%s-%s" "$os" "$ver" "$arch" "${file//./_}").json"
}

# --- Parallel Processing Engine ---
log "Initiating Parallel Metadata Extraction..."

# Debian Jobs
if [ -d "$TARGET_DIR/debian" ]; then
    for pkg in "$TARGET_DIR"/debian/*.deb; do
        [ -e "$pkg" ] || continue
        (
            VER=$(dpkg-deb -f "$pkg" Version)
            ARCH=$(dpkg-deb -f "$pkg" Architecture)
            DESC=$(dpkg-deb -f "$pkg" Description | head -n 1)
            MAINT=$(dpkg-deb -f "$pkg" Maintainer)
            SECT=$(dpkg-deb -f "$pkg" Section)
            SIZE=$(stat -c %s "$pkg")
            DATE=$(stat -c %Y "$pkg")
            SHA256=$(sha256sum "$pkg" | awk '{print $1}')
            FILE=$(basename "$pkg")
            PATH_RESOLVED=$(resolve_rel_path "$FILE" "$PUBLIC_DIR/pool")
            generate_entry "debian" "$VER" "$ARCH" "$SIZE" "$DATE" "$DESC" "$MAINT" "$SECT" "$FILE" "$SHA256" "$PATH_RESOLVED"
        ) &
    done
fi

# RPM Jobs
if [ -d "$TARGET_DIR/rpm" ]; then
    for pkg in "$TARGET_DIR"/rpm/*/*.rpm; do
        [ -e "$pkg" ] || continue
        (
            IFS='|' read -r VER ARCH DESC MAINT SECT SIZE DATE < <(rpm -qp --qf "%{VERSION}-%{RELEASE}|%{ARCH}|%{SUMMARY}|%{PACKAGER}|%{GROUP}|%{SIZE}|%{BUILDTIME}" "$pkg" 2>/dev/null || echo "||||||")
            if [ -n "$VER" ]; then
                SHA256=$(sha256sum "$pkg" | awk '{print $1}')
                FILE=$(basename "$pkg")
                PATH_RESOLVED=$(resolve_rel_path "$FILE" "$PUBLIC_DIR/rpm")
                generate_entry "fedora" "$VER" "$ARCH" "$SIZE" "$DATE" "$DESC" "$MAINT" "$SECT" "$FILE" "$SHA256" "$PATH_RESOLVED"
            fi
        ) &
    done
fi

# Arch Jobs
if [ -d "$TARGET_DIR/arch" ]; then
    for pkg in "$TARGET_DIR"/arch/*.pkg.tar.zst; do
        [ -e "$pkg" ] || continue
        (
            PKGINFO=$(tar -a -xOf "$pkg" .PKGINFO 2>/dev/null || true)
            if [ -n "$PKGINFO" ]; then
                VER=$(echo "$PKGINFO" | grep "^pkgver =" | cut -d' ' -f3 || echo "unknown")
                ARCH=$(echo "$PKGINFO" | grep "^arch =" | cut -d' ' -f3 || echo "unknown")
                DESC=$(echo "$PKGINFO" | grep "^pkgdesc =" | cut -d' ' -f3- || echo "MYTH Tactical Build")
                MAINT=$(echo "$PKGINFO" | grep "^packager =" | cut -d' ' -f3- || echo "myth-tools")
                SECT="arch-repo"
                SIZE=$(stat -c %s "$pkg")
                DATE=$(stat -c %Y "$pkg")
                SHA256=$(sha256sum "$pkg" | awk '{print $1}')
                FILE=$(basename "$pkg")
                PATH_RESOLVED=$(resolve_rel_path "$FILE" "$PUBLIC_DIR/arch")
                generate_entry "arch" "$VER" "$ARCH" "$SIZE" "$DATE" "$DESC" "$MAINT" "$SECT" "$FILE" "$SHA256" "$PATH_RESOLVED"
            fi
        ) &
    done
fi

# Wait for all background parallel jobs
wait
log "Parallel extraction complete. Assembling final manifest..."

# Assemble final JSON array
# Guard against empty TEMP_JSON_DIR (no packages found) — preventing glob expansion failure
JSON_FILES=$(find "$TEMP_JSON_DIR" -maxdepth 1 -name "*.json" 2>/dev/null)
if [ -n "$JSON_FILES" ]; then
    # We use xargs to pass the file list safely to jq
    echo "$JSON_FILES" | xargs jq -s '.' > "$OUTPUT_FILE"
else
    log "No package entries found. Generating empty manifest."
    echo "[]" > "$OUTPUT_FILE"
fi

# Final schema validation
if jq . "$OUTPUT_FILE" >/dev/null 2>&1; then
    COUNT=$(jq length "$OUTPUT_FILE")
    log "Manifest generated successfully ($COUNT entries)"
    echo "✔ Parallel manifest generated: $OUTPUT_FILE"
else
    echo "✘ [FATAL] Manifest assembly failed or produced invalid JSON."
    exit 1
fi
