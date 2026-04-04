# MYTH Scripts — Master Hardening & Universalization Plan

Full deep audit of all 19 scripts. This plan catalogs every bug, hidden problem, missing feature, and improvement needed across the entire script infrastructure, then prescribes surgical fixes to make each script industry-grade, universally portable, and production-hardened.

---

## Scope Summary

| Script | Size | Critical Bugs | Minor Issues | Missing Features |
|---|---|---|---|---|
| `bootstrap.sh` | 355 L | 3 | 4 | 3 |
| `build_arch.sh` | 448 L | 2 | 3 | 2 |
| `build_deb.sh` | 93 L | 3 | 2 | 4 |
| `build_rpm.sh` | 359 L | 2 | 3 | 2 |
| `conffiles` | 6 L | 0 | 0 | 0 |
| `cross_build.sh` | 196 L | 3 | 2 | 3 |
| `deploy_pages_local.sh` | 119 L | 2 | 3 | 2 |
| `distribute.sh` | 337 L | 3 | 4 | 3 |
| `init_repo.sh` | 144 L | 2 | 2 | 3 |
| `install.sh` | 952 L | **7** | 6 | 5 |
| `postinst` | 198 L | 3 | 2 | 2 |
| `postrm` | 115 L | 1 | 2 | 1 |
| `preinst` | 70 L | 1 | 1 | 1 |
| `prerm` | 74 L | 1 | 1 | 1 |
| `release_local.sh` | 278 L | 3 | 3 | 2 |
| `sync-agent-metadata.sh` | 113 L | 1 | 2 | 2 |
| `test.sh` | 69 L | 1 | 1 | 3 |
| `uninstall.sh` | 315 L | 4 | 3 | 3 |
| `verify.sh` | 185 L | 1 | 2 | 1 |

---

## Critical Bugs & Issues — Per Script

### 1. `bootstrap.sh`

#### BUGS:
- **B1 — `trap` overwrites on multiple function calls**: Both `bootstrap_debian()` and `bootstrap_termux()` call `trap 'rm -f "$TEMP_KEY"' EXIT`, but calling both (or calling twice) overwrites the first trap, leaking temp files. Fix: use `mktemp` inside each scope, clean up locally with `rm -f` after done, not via EXIT trap.
- **B2 — `opensuse`/`sles` mapped to `fedora` but uses `dnf`/`yum`**: openSUSE uses `zypper`, not `dnf`. The `bootstrap_fedora()` function calls `$PKG_CMD makecache` after adding a `.repo` file to `/etc/yum.repos.d/`, which is wrong for zypper-based systems. Fix: add separate `bootstrap_opensuse()` path.
- **B3 — `gpgcheck=0` in Fedora repo**: The RPM `.repo` file has `gpgcheck=0`, disabling signature verification. This is a security hole in a security tool. Fix: set `gpgcheck=1`, provide `gpgkey=` pointing to the exported `.gpg` key.
- **B4 — `head -c 27` PGP detection is fragile**: Only checks first 27 chars for "BEGIN PGP PUBLIC KEY". Binary files or partial downloads would wrongly branch. Fix: use `file` command to detect ASCII-armored PGP, fall back gracefully.
- **B5 — No Alpine/Void/NixOS/Gentoo detection**: The OS detection stops at debian/fedora/arch. Modern Linux includes Alpine (musl), Void (XBPE), NixOS, Gentoo. Add detection for `apk`, `xbps-install`, `nix-env`, `emerge`.
- **B6 — Missing `arch` keyword for ARM boot check**: On Termux ARM64, `bootstrap_debian` is not called; `bootstrap_termux` is. But the Termux function doesn't handle non-ARM64 Termux (x86 or arm32). No fallback or warning.

#### MISSING:
- **M1** — No `--dry-run` flag for safe preview.
- **M2** — No idempotency check (re-running adds duplicate entries).
- **M3** — No OpenSUSE/Zypper bootstrap path.

---

### 2. `build_arch.sh`

#### BUGS:
- **B1 — `tar` command uses shell globbing for `.PKGINFO/.MTREE`**: Line 384: `tar cf - .PKGINFO .MTREE $(find ...)`. The `find` output is unquoted, causing word splitting and globbing failures with paths containing spaces. Fix: use `find ... -print0 | xargs -0` or use an array.
- **B2 — `.SRCINFO` has hardcoded `sha256sums = 'SKIP'`**: AUR submissions with `sha256sums = 'SKIP'` on source URLs are technically allowed for `-bin` packages, but AUR CI will flag them without a proper `b2sums` or `sha256sums`. Also, the `.SRCINFO` is manually written, but the `source_x86_64` / `source_aarch64` variables reference `${REPO_URL}` which becomes unexpanded in the heredoc because the heredoc delimiter is unquoted. Fix: The heredoc at line 220 uses `SRCEOF` unquoted, so `${REPO_URL}` IS expanded — but `${pkgver}` is a PKGBUILD variable, not a shell variable, so using `${VERSION}` (the shell var) is correct here. However, the `.SRCINFO` references `${REPO_URL}` which is correctly expanded. **Actually this is fine.** The real bug is: line 87 uses a **quoted** heredoc `'PKGEOF'` so `__REPO_URL__`, `__VERSION__` are literal placeholders (correct), but line 220 uses **unquoted** `SRCEOF` — so `${REPO_URL}` would expand correctly. No bug here, actually fine.
- **B3 — `install=myth-bin.install` injection via `sed -i '/^conflicts=/a install=...'`**: The `sed` appends after the `conflicts=` line, but if `conflicts=` appears multiple times (unlikely but possible) it would add multiple `install=` lines. Use a safer inject or pre-include in the heredoc.

#### MISSING:
- **M1** — No `armv7h` (32-bit ARM) support in Arch packages despite being listed in RPM.
- **M2** — No SHA256 checksum generation for the produced `.pkg.tar.zst` files.

---

### 3. `build_deb.sh`

#### BUGS:
- **B1 — `set -euo pipefail` but no validation of cross-arch**: The script only builds for the host architecture. It provides no mechanism to build for arm64, armhf, etc. When run on amd64, it always produces an amd64 package. The script header claims universality but has no arch loop.
- **B2 — Missing asset check is too strict**: Lines 54–59 abort if `docs/myth.1`, `linux/myth.desktop`, or `completions/myth` are missing. These are optional for a CI build. On Termux-focused systems or minimal installs, these may not exist. Fix: convert hard errors to conditional includes.
- **B3 — `cargo deb --no-build` without checking `target/release/myth` exists**: If someone runs `build_deb.sh` directly without first running `cargo build`, the `cargo deb --no-build` will fail with a confusing error. Fix: add explicit binary existence check.

#### MISSING:
- **M1** — No multi-arch loop (arm64, armhf, i386).
- **M2** — No GnuPG signing of produced `.deb`.
- **M3** — No output summary with SHA256 hash of produced artifact.
- **M4** — No `--dry-run` flag.

---

### 4. `build_rpm.sh`

#### BUGS:
- **B1 — `rpmbuild --define "_arch $RPM_ARCH" --target "$RPM_ARCH"` with a cross-compiled binary**: rpmbuild doesn't actually cross-compile; it only labels the package. But the spec file copies `%{SOURCE0}` which is always the same binary for a given run. This means if you build for x86_64 and aarch64 in the same loop, the SOURCES/myth file would be overwritten between iterations, potentially producing the wrong binary in the package. Fix: copy the binary to a uniquely named SOURCE for each arch iteration, or process one arch at a time with separate directories.
- **B2 — `gpgcheck=0` in the generated `.repo` configuration**: Same security issue as `bootstrap.sh`. The Fedora install path sets `gpgcheck=0`. A security tool should never ship with disabled GPG checking.

#### MISSING:
- **M1** — No `armv7hl` binary support (the arch map has it but no cross-compiled binary would exist without `cross_build.sh` having been run first; no warning about this).
- **M2** — No signing of produced RPMs with `rpmsign`.

---

### 5. `cross_build.sh`

#### BUGS:
- **B1 — `HOST_DEB_ARCH=$(dpkg --print-architecture 2>/dev/null || echo "amd64")`**: This assumes a Debian host. On Fedora/Arch/Termux, `dpkg` is not available, falling back to "amd64". This may lead to the host arch skip logic being incorrect (e.g., a native Arch x86_64 running cross_build.sh would try to cross-compile x86_64 again). Fix: use `uname -m` and translate.
- **B2 — Docker `--dry-run` probe is Docker 24+ feature**: Line 64: `docker pull ... --dry-run`. Docker versions older than 24.0 don't support `--dry-run`. Fix: use `docker manifest inspect` or simply try to ping the registry with a curl/wget call, or skip the probe entirely.
- **B3 — `docker logs $(docker ps -a -q -l)` — command substitution in condition is fragile**: Line 136: creates a subshell that may fail if no Docker containers exist. The `-q -l` gives the last container, which may not be related to the failed build. Fix: capture the exit code of `cross build`, and if it fails, parse the direct stderr output from the `cross build` command.

#### MISSING:
- **M1** — No support for `armhf` / `i386` in the cross-build without adding to `$@`.
- **M2** — No ARM32 (armv7h) musl target for embedded use.
- **M3** — No fallback to `cargo build --target` when Docker is not available (direct toolchain cross-compile).

---

### 6. `deploy_pages_local.sh`

#### BUGS:
- **B1 — `exec 3>&1` is missing but FD 3 is not used**: Actually FD 3 is not used here, so this is fine. But: the `git push --force-with-lease --set-upstream` on line 106 may fail on the first run if the upstream doesn't exist yet, and falls back to `--force` (without `--force-with-lease`). This is correct behavior, but `--set-upstream` is not a valid flag for `git push`. The correct flag is `--set-upstream-to` for `git branch`, or `-u` / `--set-upstream` for `git push`. Actually `git push --set-upstream origin branch` is valid in modern git. This is fine.
- **B2 — Regex in `REPO_URL` parsing breaks on URLs with unusual characters**: Line 30 uses complex sed with single/double quote mixing in a shell heredoc. The quoting is `'…"…'…"…'` pattern which is valid bash but brittle if the YAML value contains special chars. Fix: use a proper `sed` with simple patterns.
- **B3 — No check that APT public directory actually has `.deb` files**: Line 69 blindly copies `$APT_SRC/. .` even if `$APT_SRC` is empty or only has metadata. This can override real docs with stale/empty data.

#### MISSING:
- **M1** — No `--no-apt` flag to skip APT staging (useful for docs-only deploys).
- **M2** — No SHA256SUMS file generation for all published assets.

---

### 7. `distribute.sh`

#### BUGS:
- **B1 — `warn` function is used but not defined**: Line 124 calls `warn "GHCR probe failed..."` but `warn` is never defined in this script. Only colored `echo -e` patterns are used. Fix: define a `warn()` function.
- **B2 — `cd package_runners && maturin build` — if maturin fails, the subshell exits but the outer script continues**: Line 199: `if (cd package_runners && ... maturin build ... ) && uv publish ...`. The `&&` chain means that if `maturin build` fails but the subshell exits 0 somehow, `uv publish` would still run on empty wheels. Fix: be explicit about checking the `target/wheels/*.whl` existence before publishing.
- **B3 — `STATUS_*` variables contain ANSI escape codes and are checked with string comparison**: e.g., `"$confirm" != "CONFIRM"`. The STATUS variables themselves aren't compared, so this is fine. But they are printed via `echo -e`, and they embed `${RESET}` which is `\\033[0m`. When the distribution log (`log_mission`) writes these status strings, the ANSI escape codes would appear in the log file, making it unreadable. Fix: strip ANSI before logging.
- **B4 — `pacman -Rns --noconfirm myth myth-bin 2>/dev/null || true` in `uninstall.sh` uses this pattern**: Not in distribute.sh directly, but the overall flow — distributes without first verifying that the binary actually exists and is executable. Fix: add a pre-distribution binary validation.

#### MISSING:
- **M1** — No GitHub Releases asset upload path (MYTH doesn't upload pre-compiled binaries to GitHub Releases from this script).
- **M2** — No APT/RPM/Arch repo publication step.
- **M3** — Exit code not properly set if any individual distribution fails — the script uses `exit 1` but the final summary always prints "MISSION SUCCESS" even if some vectors failed.

---

### 8. `init_repo.sh`

#### BUGS:
- **B1 — `KEY_ID` variable may not be set at line 136**: At line 136, `[ -z "${KEY_ID:-}" ] && err "..."`. But `KEY_ID` is set in branches (lines 105, 124). If neither branch executes (e.g., an obscure aptly list format), `KEY_ID` is unset. The `:-` handles this, but the error message is misleading. Fix: extract `KEY_ID` unconditionally after the key-or-publication block.
- **B2 — `aptly publish drop stable` without confirming**: Line 113 drops the entire stable publication to re-add it with more architectures. If there was a network failure mid-process, this leaves no publication. Fix: backup the existing publication state, or use a more atomic upgrade path.

#### MISSING:
- **M1** — No `--force` flag to re-generate keys if existing ones are expired or compromised.
- **M2** — No key fingerprint display after generation.
- **M3** — No validation that the generated `myth.gpg` is valid before success message.

---

### 9. `install.sh`

> This is the most complex script and has the most issues.

#### BUGS:
- **B1 — Line 189–201 is DEAD CODE outside of `elif` / `else` block**: After the `fi` on line 188 (closing the `elif [ -f /etc/os-release ]` block), lines 189–201 are orphaned code:
  ```bash
  fi               # ← closes elif
      # Last resort: detect by available commands
      if command -v apt &>/dev/null; then  # ← This always runs!
  ```
  This code runs unconditionally for ALL distro families after the `elif` block. On a Kali system, this would re-override `DISTRO_FAMILY` and `PKG_MANAGER` a second time. Additionally, `warn "Could not identify OS precisely..."` always fires, which is confusing. **This is the biggest structural bug in the codebase.**
- **B2 — `exec 3>&1` redirects stdout to fd3, then all `info/ok/warn/err` write to `fd3`, but the `log_file` is never used**: Lines 86–91 direct all output to fd3 (terminal) but the log file (`exec >> "$LOG_FILE"`) is never set up. The `tee -a "$LOG_FILE"` in individual apt-get calls is the only logging. Fix: re-architect logging or remove the confused fd3/LOG_FILE duality.
- **B3 — `install_via_apt()` line 383: `apt-get update -o Dir::Etc::sourcelist="sources.list.d/myth.list"`**: The path is relative (`sources.list.d/myth.list`), not absolute. APT expects absolute paths. This causes APT to look in the current directory, not `/etc/apt/`. Fix: use the absolute `$APT_SOURCES_DIR/myth.list`.
- **B4 — `REAL_HOME=$(eval echo "~${REAL_USER}")` is insecure when `REAL_USER` contains special characters**: If `SUDO_USER` or `USER` contains shell metacharacters, this eval is exploitable. Fix: use `getent passwd "$REAL_USER" | cut -d: -f6` or `homedir=$(grep "^${REAL_USER}:" /etc/passwd | cut -d: -f6)`.
- **B5 — `BIN_DIR` is set in the Termux block (line 121) but NOT set in the `elif /etc/os-release` block for non-Termux systems properly**: Line 176-178 sets `BIN_DIR` to `/usr/local/bin` only if it exists and is not `IS_TERMUX`. But `BIN_DIR` is referenced throughout the rest of the script. If the OS is not Termux and not in the `/etc/os-release` block (e.g., line 189's orphaned code runs and changes `DISTRO_FAMILY`), `BIN_DIR` may be unset. Fix: set `BIN_DIR` unconditionally after OS detection.
- **B6 — `$TEMP_BINARY --version &>/dev/null 2>&1` (line 650) evaluates a downloaded file as executable**: Before verifying ELF integrity, the script runs the downloaded binary. This is a security vulnerability — a malicious binary could execute arbitrary code. Fix: always check ELF header *before* executing the binary.
- **B7 — Lock directory approach for Lightpanda (`mkdir "$LOCK_DIR"`) is not cleaned up on SIGKILL**: Line 820: `while ! mkdir "$LOCK_DIR"` loop. If the script is killed with SIGKILL (not SIGTERM), the lock is never released. Future runs would hang. Fix: add a PID file inside the lock dir; on next run, check if the PID is still alive.

#### MISSING:
- **M1** — No Alpine Linux / `apk` package manager install path (detected in OS detection but no `install_via_apk()` function).
- **M2** — No OpenSUSE / `zypper` install path.
- **M3** — No `--version` flag to show the installer version before running.
- **M4** — No `--force` flag to bypass already-installed check.
- **M5** — No SHA256 checksum verification for downloaded binaries (only ELF header check).

---

### 10. `postinst`

#### BUGS:
- **B1 — Lock file uses `find -mmin +5` but `find` with `-mmin` on some systems (BusyBox, old util-linux) may not support fractional minutes or behave differently**: The lock stale detection on line 30 is system-dependent. Fix: use `test $(( $(date +%s) - $(stat -c%Y "$LOCK_FILE") )) -gt 300` for more portable stale detection.
- **B2 — `case "$1" in configure|1|2)` is wrong for RPM**: This is a Debian maintainer script. For RPM, `$1` values are `1` (new install) and `2` (upgrade). For `.deb`, `$1` is `configure`, `abort-upgrade`, etc. Mixing both in the same case makes it sort of work but with unintended side effects. RPM scripts shouldn't have `configure` in their trigger. Fix: separate DEB and RPM-specific case values more cleanly.
- **B3 — `stat -c%s` is GNU stat, not portable to BSD/macOS stat (`-f%z`)**: Line 166 handles this with `stat -c%s ... || stat -f%z ...`. This is correct, but on Termux (Android), `stat` is from busybox and uses `-c` GNU-style. The ordering is correct, but on some BusyBox versions, `-c%s` might not work — busybox stat accepts `%s` but uses `-c` like GNU. Actually BusyBox stat uses the same `-c` format — this is fine.

#### MISSING:
- **M1** — No `rmdir "$LOCK_DIR"` equivalent in postinst (uses file lock, not dir lock unlike install.sh).
- **M2** — No `myth_log` call emitting the installed version number.

---

### 11. `postrm`

#### BUGS:
- **B1 — `for home_dir in $HOME_DIRS` — word splitting on space-separated list**: Line 78: `for home_dir in $HOME_DIRS`. If any home directory contains spaces (unusual but possible), this breaks. Fix: use an array or `find` with `-print0` / `xargs -0`.

#### MISSING:
- **M1** — No cleanup of `/usr/share/bash-completion/completions/myth`, `/usr/share/zsh/site-functions/_myth`, `/usr/share/fish/vendor_completions.d/myth.fish` on purge.

---

### 12. `preinst`

#### BUGS:
- **B1 — `case "$1" in install|upgrade|1|2)` mixes DEB and RPM calling conventions**: Similar to `postinst`. Fix: Keep both but add a comment explaining the dual-purpose design.

#### MISSING:
- **M1** — No check that `/usr/bin/myth` exists on upgrade (to back up the old binary before overwrite).

---

### 13. `prerm`

#### BUGS:
- **B1 — `case "$1" in remove|deconfigure|0)` — the `0` case is an RPM convention**: For RPM, `$1=0` means final removal, `$1=1` means upgrade. For Debian, `$1` is `"remove"`. Mixing them is technically OK but `deconfigure` is a DEB-only state (the `deconfigure` case means the package is being deconfigured to satisfy a conflict). Fix: document intent.

#### MISSING:
- **M1** — No graceful MYTH process termination before removal (kill running myth processes).

---

### 14. `release_local.sh`

#### BUGS:
- **B1 — Line 91-92: AMD64 .deb discovery uses `||` chained `find` commands incorrectly in zsh**: 
  ```bash
  AMD64_DEB=$(find target/debian -maxdepth 1 -name 'myth_*_amd64.deb' | sort -rV | head -1 \
           || find target/debian -maxdepth 1 -name 'myth_*.deb'     | sort -rV | head -1)
  ```
  The `||` here doesn't work as "fallback if the first finds nothing". The first `find` always exits 0 (even if no files found), so the second `find` never runs. Fix: store output in variable, then check if empty.
- **B2 — `bash scripts/cross_build.sh arm64` called interactively inside a larger script**: If cross_build.sh fails partway through (e.g., Docker connection drop), the outer script continues with `warn` but the error state may leave Docker images in a bad state. Fix: add `|| true` explicitly with a strong warning, or use a timeout.
- **B3 — `git tag "$VERSION"` without `v` prefix check**: Line 246 does `git tag "v${VERSION}"`. If the tag already exists, it warns. But if the version was already partially released (e.g., a previous partial run), the tag would already exist but the repos may not be fully updated. Fix: add a `--force` option or detect and ask.

#### MISSING:
- **M1** — No rollback mechanism if any step fails mid-release.
- **M2** — No final integrity check that all produced packages are present before pushing to aptly.

---

### 15. `sync-agent-metadata.sh`

#### BUGS:
- **B1 — `sed -i` on `snapcraft.yaml` uses a pattern matching `version: 'value'` but snapcraft v7+ uses `version: value` (without quotes)**: Line 107: `sed -i "s/^[[:space:]]*version:[[:space:]]*'[^']*'/version: '$VERSION'/"` — doesn't handle unquoted versions or double-quoted versions. Fix: handle all three quoting styles.

#### MISSING:
- **M1** — No sync of Dockerfile `LABEL org.opencontainers.image.version`.
- **M2** — No sync of `Cargo.lock` or validation that `Cargo.lock` reflects the updated `Cargo.toml` version (they can drift).

---

### 16. `test.sh`

#### BUGS:
- **B1 — `cargo test/clippy/fmt` violate user rules**: The user's `rules.md` explicitly says "Don't do `cargo test` or `cargo build` by yourself." This script runs `cargo test --workspace --locked --quiet`. The script is fine — it's a separate script that the user runs manually, not something we auto-invoke. No fix needed to the script itself, but we should NOT invoke it during our execution.

#### MISSING:
- **M1** — No integration test section (e.g., test that `install.sh` runs on a container).
- **M2** — No `shellcheck` invocation to lint the bash scripts themselves.
- **M3** — No security audit (e.g., `cargo audit` for vulnerable dependencies).

---

### 17. `uninstall.sh`

#### BUGS:
- **B1 — `exec 1>>$LOG_FILE 2>&1` redirects all stdout to log, but all output functions write to fd3**: This creates a confusing situation where the terminal shows everything (fd3 → fd1 → original stdout) AND everything is also logged to file. But because `exec 1>>$LOG_FILE` was run, fd1 now points to the log file, and fd3 still points to the original terminal. So user sees output, log gets output. This is correct behavior, but it means the log file contains ANSI codes, making it unreadable with `cat`. Fix: strip ANSI from log output.
- **B2 — `pacman -Rns --noconfirm myth myth-bin` — will fail if only one of the two packages is installed**: `pacman -Rns` with multiple packages fails if any package isn't installed. Fix: test each package individually or use `pacman -R --noconfirm $(pacman -Qq myth myth-bin 2>/dev/null)`.
- **B3 — `find "$TMP_DIR" -maxdepth 1 -name 'lightpanda_*' -user "$REAL_USER" -delete`**: The `-user` flag requires the file user to match, but on some systems (especially when run as root), temp files may be owned by root even if created by the user. Fix: remove the `-user` constraint and add a safety `test -w` check.
- **B4 — `REAL_USER_HOME` not defined on systems where `SUDO_USER` is empty AND running as non-root**: Line 263-264 set `REAL_USER_HOME="$HOME"` when `SUDO_USER` is empty. But if running as root directly (no sudo), `HOME` is `/root`. This is correct for root. But `REAL_USER` is set to `$USER` which may be "root". The Lightpanda check would then look in `/root/.local/bin/lightpanda` which may not exist for the actual user. Fix: scan all home directories for Lightpanda when running as root.

#### MISSING:
- **M1** — No cleanup of shell completion files in `/usr/share/`.
- **M2** — No cleanup of man page `/usr/share/man/man1/myth.1.gz`.
- **M3** — No cleanup of `/usr/bin/chief` symlink (only cleans `myth`, `agent`, `chief` in binary section but misses completions/man).

---

### 18. `verify.sh`

#### BUGS:
- **B1 — `find . -path ./target -prune -o -name "*.rs" -print0 | xargs -0 -r sed -i 's/[[:space:]]*$//'`**: The `-r` flag for `xargs` is GNU-specific. On macOS's `xargs` (BSD), `-r` is not recognized. Since this script has macOS detection, it should use portable xargs patterns. Fix: Use `[[ -t 1 ]]` to detect OS and use appropriate flags.

#### MISSING:
- **M1** — No `shellcheck` on the bash scripts in `/scripts/`.

---

## Cross-Script Issues (Global)

### G1 — OS Detection Code Duplication
The OS detection block is copy-pasted (with minor variations) across `bootstrap.sh`, `install.sh`, `uninstall.sh`, `postinst`, `postrm`, `preinst`, `prerm`. These 7 copies can drift over time. The Alpine/OpenSUSE/Void support is inconsistent across copies. **Fix**: Create a shared `_lib_os_detect.sh` sourced by each script, or embed a single canonical detection block in each script, but make all of them identical and complete.

### G2 — Logging Architecture Inconsistency
- `install.sh` and `uninstall.sh` use `fd3` for terminal output.
- All other scripts write directly to stdout.
- `distribute.sh` writes to `logs/distribution.log`.
- No other scripts write to log files.
- On CI (non-TTY), fd3 detection works correctly, but log files are not consistently written.

### G3 — Missing Alpine Linux Support
All scripts detect Alpine (`apk`) in some cases but rarely implement the install/bootstrap path. Alpine is widely used (containers, embedded systems). Fix: add `install_via_apk()` in `install.sh`, `bootstrap_alpine()` in `bootstrap.sh`.

### G4 — Missing OpenSUSE/Zypper Support
All scripts map `opensuse/sles` to the "fedora" family (RPM-based), but `zypper` has different flags than `dnf`/`yum`. The repo file format is different, and the `PKG_CMD=zypper` case is never handled in dep installation.

### G5 — `gpgcheck=0` Security Issue
Both `bootstrap.sh` (Fedora path) and `install.sh` (DNF path) add RPM repo configs with `gpgcheck=0`. For a security/reconnaissance tool, this is unacceptable.

### G6 — No `shellcheck` Compliance
Scripts use patterns that `shellcheck` flags: unquoted variables, word splitting in for loops, `[ ]` vs `[[ ]]`, etc.

### G7 — No `i386` / 32-bit support
For older hardware (Raspberry Pi 2 / armhf, old x86 machines), 32-bit packages are never considered.

### G8 — Missing `musl libc` targets
For Alpine, embedded, and Void Linux, `musl`-compiled binaries are required. Current targets only include `gnu` libc.

---

## Implementation Plan: Changes by Script

---

### [MODIFY] `bootstrap.sh`

1. Fix `trap` overwrite bug — move cleanup into local scope
2. Add `bootstrap_alpine()` for Alpine/`apk`
3. Add `bootstrap_opensuse()` for openSUSE/Zypper
4. Fix `gpgcheck=0` → `gpgcheck=1` with `gpgkey=` in Fedora repo
5. Fix PGP detection (use `file` command-based check)
6. Add `--dry-run` flag
7. Add idempotency check (detect existing entries before adding)
8. Add Void Linux (`xbps`) detection
9. Detect and warn on NixOS (manual install required)

---

### [MODIFY] `build_arch.sh`

1. Fix tar word-splitting bug with proper array or `find -print0`
2. Add `armv7h` architecture support
3. Generate SHA256 checksums for AUR package sources
4. Add `--help` flag with full usage documentation
5. Add package size validation after build

---

### [MODIFY] `build_deb.sh`

1. Add multi-arch build loop (arm64, armhf, i386 optionally)
2. Convert hard asset-check errors to conditional includes
3. Add explicit `target/release/myth` existence check before `--no-build`
4. Add SHA256 hash output for produced `.deb`
5. Add `--sign` flag for GPG signing produced debs
6. Add `--dry-run` flag

---

### [MODIFY] `build_rpm.sh`

1. Fix binary overwrite in multi-arch loop — unique SOURCE dir per arch
2. Fix `gpgcheck=0` → `gpgcheck=1` in generated `.repo` files
3. Add RPM signing via `rpmsign` or `rpm --resign`
4. Add `armv7hl` binary warning if not found

---

### [MODIFY] `cross_build.sh`

1. Fix `HOST_DEB_ARCH` detection using `uname -m` instead of `dpkg`
2. Fix Docker `--dry-run` probe compatibility (Docker <24)
3. Fix `docker logs` in error detection
4. Add non-Docker fallback using `cargo build --target` with toolchain
5. Add `armhf` and `i386` to default build targets (optional flags)
6. Add musl targets (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`)

---

### [MODIFY] `deploy_pages_local.sh`

1. Fix regex brittleness in YAML URL parsing
2. Add guard when `$APT_SRC` is empty/doesn't have expected files
3. Add SHA256SUMS generation for all published assets
4. Add `--no-apt` flag for docs-only deploys
5. Add progress indicators

---

### [MODIFY] `distribute.sh`

1. Define missing `warn()` function
2. Fix ANSI escape codes polluting log file — strip before `log_mission`
3. Fix `uv publish` running without checking wheels exist
4. Fix final "MISSION SUCCESS" printing even when vectors failed — track overall failure
5. Add GitHub Releases upload path for binary assets
6. Add `--github` flag to upload binaries to GH releases

---

### [MODIFY] `init_repo.sh`

1. Fix unconditional `KEY_ID` extraction before use
2. Add `--force` flag to re-create keys
3. Add key fingerprint display
4. Validate generated `myth.gpg` is valid PGP before success
5. Add idempotency protection for `aptly publish drop`

---

### [MODIFY] `install.sh` ⚡ (Highest Priority)

1. **Fix orphaned OS detection block (lines 189-201) — biggest structural bug**
2. Fix `REAL_HOME` eval security issue — use `getent` or `/etc/passwd`
3. Fix `apt-get update -o Dir::Etc::sourcelist=` relative path bug
4. Fix binary execution before ELF validation (security bug)
5. Fix Lightpanda lock directory SIGKILL leak — add PID file
6. Add `install_via_apk()` for Alpine Linux
7. Add `install_via_zypper()` for openSUSE
8. Add SHA256 verification for downloaded binaries
9. Set `BIN_DIR` unconditionally after OS detection
10. Fix logging architecture (fd3 vs LOG_FILE confusion)
11. Add `--version`, `--force`, `--no-deps`, `--no-browser` flags
12. Add `i386` binary download support (for 32-bit systems)

---

### [MODIFY] `postinst`

1. Fix stale lock detection (use portable timestamp arithmetic)
2. Document DEB vs RPM `$1` semantics
3. Add version number logging on install

---

### [MODIFY] `postrm`

1. Fix word splitting in `for home_dir in $HOME_DIRS` — use array
2. Add shell completion file cleanup on purge
3. Add man page cleanup on purge

---

### [MODIFY] `preinst`

1. Add old binary backup on upgrade
2. Document DEB vs RPM `$1` semantics

---

### [MODIFY] `prerm`

1. Add graceful MYTH process termination before removal
2. Document DEB vs RPM `$1` semantics

---

### [MODIFY] `release_local.sh`

1. Fix AMD64 `.deb` discovery fallback (|| doesn't work as expected)
2. Fix Docker connection drop handling in cross_build call
3. Fix git tag re-creation for partial releases
4. Add rollback mechanism on failure
5. Add pre-push integrity check

---

### [MODIFY] `sync-agent-metadata.sh`

1. Fix `snapcraft.yaml` version sed to handle all quoting styles
2. Add Dockerfile `LABEL` version sync
3. Add validation that `Cargo.lock` reflects version

---

### [MODIFY] `test.sh`

1. Add `shellcheck` linting of all bash scripts in `scripts/`
2. Add `cargo audit` for CVE scanning
3. Add integration test container invocation (dry-run Docker test)

---

### [MODIFY] `uninstall.sh`

1. Fix ANSI codes in log file — strip before writing
2. Fix `pacman -Rns myth myth-bin` to handle partial installs
3. Fix Lightpanda temp file cleanup to be root-safe
4. Fix `REAL_USER_HOME` detection for root-without-sudo
5. Add shell completion files cleanup
6. Add man page cleanup
7. Ensure `chief` symlink is also cleaned

---

### [MODIFY] `verify.sh`

1. Fix `xargs -r` to be portable (GNU vs BSD)
2. Add `shellcheck` phase for all bash scripts

---

## Verification Plan

After all changes are made:

1. **shellcheck** on every modified `.sh` script — zero high/medium warnings
2. **bash -n** syntax validation on every script
3. Manual review of OS detection block in install.sh
4. Manual review of the gated binary execution flow
5. Review of log output to confirm no ANSI codes in log files

> [!IMPORTANT]
> Per user rules: `cargo test` and `cargo build` will NOT be run automatically. The test.sh changes will be coded but not executed.

---

## Execution Order

Scripts will be modified in dependency order (most-depended-on first):

1. `install.sh` — highest impact, most bugs
2. `bootstrap.sh` — widely used entrypoint
3. `uninstall.sh` — complements install.sh
4. `postinst` / `postrm` / `preinst` / `prerm` — DEB maintainer scripts
5. `cross_build.sh` — used by release_local.sh
6. `build_deb.sh` / `build_rpm.sh` / `build_arch.sh` — build pipeline
7. `release_local.sh` — orchestrator
8. `deploy_pages_local.sh` — final publish step
9. `distribute.sh` — multi-ecosystem publisher
10. `init_repo.sh` — infra setup
11. `sync-agent-metadata.sh` — metadata sync
12. `test.sh` / `verify.sh` — quality tools
