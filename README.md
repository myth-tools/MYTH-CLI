<div align="center">

<!-- HERO BANNER -->
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/myth-tools/MYTH-CLI/main/docs/public/banner-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/myth-tools/MYTH-CLI/main/docs/public/banner-light.png">
  <img src="https://raw.githubusercontent.com/myth-tools/MYTH-CLI/main/docs/public/banner-dark.png" alt="MYTH — Autonomous AI Reconnaissance Operative" width="100%">
</picture>

<br/>

# ⚡ MYTH — Autonomous AI Reconnaissance Operative

### *The world's most advanced AI-powered offensive security agent for any Linux distribution.*

**Autonomous · Unrestricted · Zero Compromise · Production-Ready**

<br/>

[![CI/CD Release](https://img.shields.io/github/actions/workflow/status/myth-tools/MYTH-CLI/release.yml?label=CI%2FCD%20Pipeline&style=for-the-badge&logo=githubactions&logoColor=white&color=brightgreen)](https://github.com/myth-tools/MYTH-CLI/actions)
[![GitHub Release](https://img.shields.io/github/v/release/myth-tools/MYTH-CLI?style=for-the-badge&logo=github&color=blueviolet&label=Latest%20Release)](https://github.com/myth-tools/MYTH-CLI/releases/latest)
[![Crates.io](https://img.shields.io/crates/v/myth?style=for-the-badge&logo=rust&color=orange&label=crates.io)](https://crates.io/crates/myth)
[![NPM Version](https://img.shields.io/npm/v/@myth-tools/myth?style=for-the-badge&logo=npm&color=red&label=npm)](https://www.npmjs.com/package/@myth-tools/myth)
[![PyPI Version](https://img.shields.io/pypi/v/myth-cli?style=for-the-badge&logo=pypi&logoColor=white&color=blue&label=PyPI)](https://pypi.org/project/myth-cli/)
[![Docker Pulls](https://img.shields.io/badge/GHCR-myth--tools%2Fmyth-blue?style=for-the-badge&logo=docker&logoColor=white)](https://ghcr.io/myth-tools/myth)
[![Snap Store](https://img.shields.io/badge/Snap-myth-green?style=for-the-badge&logo=snapcraft&logoColor=white)](https://snapcraft.io/myth)

<br/>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Universal%20Linux-purple?style=for-the-badge&logo=linux&logoColor=white)](https://myth.work.gd)
[![Arch](https://img.shields.io/badge/Arch-amd64%20%7C%20arm64-teal?style=for-the-badge&logo=linux&logoColor=white)](https://myth.work.gd)
[![Rust](https://img.shields.io/badge/Built%20with-Rust%201.75+-orange?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![NVIDIA NIM](https://img.shields.io/badge/AI%20Engine-NVIDIA%20NIM-76b900?style=for-the-badge&logo=nvidia&logoColor=white)](https://build.nvidia.com/)
[![Stars](https://img.shields.io/github/stars/myth-tools/MYTH-CLI?style=for-the-badge&logo=github&color=gold)](https://github.com/myth-tools/MYTH-CLI/stargazers)

<br/>

> **MYTH** is a next-generation, single-binary AI reconnaissance operative written entirely in Rust.  
> It autonomously orchestrates **3,000+ Kali Linux security tools**, reasons through results in real-time  
> using **NVIDIA NIM** elite LLMs, and delivers professional-grade intelligence reports —  
> all inside a hardened, zero-trust **Bubblewrap** sandbox. No configuration. No friction. Just intelligence.

<br/>

[🚀 Quick Install](#-rapid-deployment) &nbsp;·&nbsp; [📖 Documentation](https://myth.work.gd) &nbsp;·&nbsp; [🎯 Commands](#-mission-control--command-reference) &nbsp;·&nbsp; [⚙️ Configuration](#%EF%B8%8F-configuration-reference) &nbsp;·&nbsp; [🌐 All Ecosystems](#-universal-package-runners) &nbsp;·&nbsp; [🤝 Contribute](#-contributing)

</div>

---

## 📋 Table of Contents

- [Why MYTH?](#-why-myth--the-definitive-advantage)
- [Feature Matrix](#-feature-matrix)
- [Architecture Overview](#%EF%B8%8F-architecture-overview)
- [Quick Install](#-rapid-deployment)
- [Universal Package Runners](#-universal-package-runners)
- [First-Run Setup](#%EF%B8%8F-mandatory-first-run-setup)
- [Mission Control — Commands](#-mission-control--command-reference)
- [The 13-Phase Methodology](#-the-elite-13-phase-recon-methodology)
- [Security Architecture](#%EF%B8%8F-security-architecture--defense-in-depth)
- [Configuration Reference](#%EF%B8%8F-configuration-reference)
- [Technology Stack](#-technology-stack)
- [MYTH vs Alternatives](#-myth-vs-the-competition)
- [Contributing](#-contributing)
- [FAQ](#-frequently-asked-questions)
- [Tactical Decommissioning](#-tactical-decommissioning--full-purge)
- [Creator](#-creator)
- [Legal Disclaimer](#%EF%B8%8F-legal--operational-disclaimer)
- [License](#-license)

---

## 🏆 Why MYTH? — The Definitive Advantage

Traditional offensive security workflows are broken. You manually chain hundreds of tools, parse walls of raw output, write custom wrapper scripts, and make critical pivoting decisions under time and cognitive pressure — while switching between terminals, browser tabs, and notes.

**MYTH permanently eliminates all of that friction.**

It is the world's first **fully autonomous, reasoning-capable AI reconnaissance operative**. It doesn't just run tools — it *understands* their output, *reasons* about attack vectors, and *dynamically pivots strategy* like a senior penetration tester with perfect memory and infinite patience.

<br/>

| The Old Way | The MYTH Way |
| :--- | :--- |
| ❌ Manually chain 50+ tools | ✅ One command launches a complete, orchestrated mission |
| ❌ Parse raw output with `grep` and `awk` | ✅ AI reads, understands, and acts on every line of output |
| ❌ Forget to try a technique mid-engagement | ✅ 89-step professional framework — nothing is ever skipped |
| ❌ Your host system exposed to rogue tools | ✅ Hardened Bubblewrap namespace — zero host system contact |
| ❌ Evidence left on disk after the mission | ✅ RAM-only volatility — forensic trace is zero |
| ❌ Restart from scratch each session | ✅ Persistent session memory — MYTH builds on everything it learned |
| ❌ Hours of manual OSINT correlation | ✅ AI synthesizes org graph, employees, breaches, and ASNs automatically |

---

## 💎 Feature Matrix

| Feature | Detail |
| :--- | :--- |
| 🧠 **NVIDIA NIM Neural Reasoning** | Powered by elite LLMs (Qwen3, DeepSeek-R1, Llama-3.3-70B). MYTH reads raw tool output, understands context, identifies hidden attack vectors, and autonomously adjusts strategy — just like a senior pentester. |
| ⚔️ **3,000+ Tool Arsenal** | Full, native access to the entire Kali Linux toolkit. No wrapper scripts. No configuration files. Every tool — from `nmap` to `nuclei`, `amass` to `sqlmap` — available instantly through the MCP orchestration layer. |
| 🛡️ **Zero-Trust Bubblewrap Sandbox** | Every single tool process runs inside a hardened Linux user namespace. The host filesystem is read-only. Rogue tools, hostile payloads, and compromised binaries **cannot** touch your host system. |
| 💨 **Operational Volatility** | All mission data is DRAM-resident. `myth burn` triggers instant process termination + zero forensic trace on disk. Designed for OPSEC-critical environments. |
| 🚀 **Native Rust Performance** | Built with `lto = fat`, `codegen-units = 1`, and `opt-level = 3`. Sub-2ms startup time. Single, fully static binary with zero runtime dependencies. Ships as a native `.deb` for APT. |
| 🌐 **Lightpanda Browser Engine** | Bundled Zig-based headless browser engine — **11× faster** and **9× less RAM** than Chromium. Zero-latency JavaScript-heavy reconnaissance, SPA crawling, and client-side secret extraction. |
| 🔌 **MCP Protocol Native** | Full Model Context Protocol (MCP) support for dynamic tool discovery, hot-pluggable extensions, and community tool servers. Plug in any MCP server in `~/.config/myth/user.yaml` — no code required. |
| 🔐 **AES-GCM-SIV + ChaCha20** | Military-grade encryption for any mission data that must persist. Uses `aes-gcm-siv` and `chacha20poly1305` for authenticated encryption with extended nonce support. |
| 🎨 **Elite Typography Suite** | Automatic provisioning of 10 premium terminal fonts (Nerd Fonts family). Perfect Unicode rendering across all modern terminal emulators — Alacritty, Kitty, WezTerm, iTerm2. |
| 🔁 **Persistent Session Memory** | Missions are stateful across reboots. MYTH remembers everything discovered — subdomains, open ports, credentials, CVEs — and builds on prior intelligence in every subsequent session. |
| 🌍 **Tor OPSEC Integration** | Native Tor routing via SOCKS5 proxy. Route subdomain lookups, HTTP requests, and tool traffic through the Tor network for anonymized intelligence gathering. |
| 📊 **Professional Reporting** | Structured, filterable intelligence reports with ASN graphs, CVE summaries, credential exposure analysis, and complete attack surface synthesis in markdown and JSON formats. |
| 🔄 **Automated Key Rotation** | Configure multiple NVIDIA NIM API keys. MYTH automatically rotates on rate-limit, ensuring uninterrupted long-duration missions. |
| 📡 **Real-Time TUI Dashboard** | Powered by `ratatui` — a full terminal UI mission dashboard with live phase progress, discovered asset counters, AI reasoning traces, and abort controls. |

---

## 🏛️ Architecture Overview

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                            MYTH Neural Core (Rust)                           │
│                                                                              │
│  ┌──────────────┐    ┌─────────────────┐    ┌────────────────────────────┐   │
│  │  Clap CLI    │───▶│  MCP Orchestr.  │───▶│  NVIDIA NIM AI Reasoning   │   │
│  │  Engine      │    │ (Tool Registry) │    │  (rig-core + reqwest)      │   │
│  └──────────────┘    └─────────────────┘    └────────────────────────────┘   │
│         │                    │                           │                   │
│         ▼                    ▼                           ▼                   │
│  ┌──────────────┐    ┌──────────────────────────────────────────────────┐    │
│  │ Ratatui TUI  │    │                Bubblewrap Sandbox                │    │
│  │ Dashboard    │    │  ┌─────────┐ ┌─────────┐ ┌────────┐ ┌────────┐   │    │
│  └──────────────┘    │  │  nmap   │ │  amass  │ │ nuclei │ │  ffuf  │   │    │
│         │            │  ├─────────┤ ├─────────┤ ├────────┤ ├────────┤   │    │
│  ┌──────────────┐    │  │ sqlmap  │ │ wayback │ │subfind.│ │ 3000+  │   │    │
│  │ Persistent   │    │  └─────────┘ └─────────┘ └────────┘ └────────┘   │    │
│  │ Session DB   │    └──────────────────────────────────────────────────┘    │
│  └──────────────┘                         │                                  │
│         │                                 ▼                                  │
│  ┌──────────────┐    ┌──────────────┐    ┌────────────────────────────┐      │
│  │  Report Gen  │    │  Lightpanda  │    │  Tor SOCKS5 OPSEC Router   │      │
│  │  (MD + JSON) │    │  (JS Recon)  │    │  (Anonymous Intel Gather)  │      │
│  └──────────────┘    └──────────────┘    └────────────────────────────┘      │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Distribution Matrix:**

| Ecosystem | Package | Command |
| :--- | :--- | :--- |
| 📦 **APT** (Debian/Ubuntu) | Native `.deb`, signed APT repo | `sudo apt install myth` |
| 📦 **RPM** (Fedora/RHEL) | Native `.rpm`, signed RPM repo | `sudo dnf install myth` |
| 📦 **AUR** (Arch Linux) | Native `.pkg.tar.zst` | `sudo pacman -U myth-*.pkg.tar.zst` |
| 📱 **Termux** | Native `arm64` binary | `pkg install myth` |
| 🦀 **Cargo** | `myth` on crates.io | `cargo install myth --locked` |
| 📦 **NPM / Bun / PNPM** | `@myth-tools/myth` | `npx @myth-tools/myth` |
| 🐍 **PyPI / UVX** | `myth-cli` | `uvx myth-cli` |
| 🐳 **Docker / Podman** | `ghcr.io/myth-tools/myth` | `docker run ...` |
| 🎯 **Snap** | `myth` on Ubuntu Store | `sudo snap install myth` |
| ❄️ **Nix** | Flake in `package_runners/` | `nix run github:myth-tools/MYTH-CLI?dir=package_runners` |
| ⬇️ **Raw Binary** | GitHub Releases | Auto-detect arch & download |

---

## 🚀 Rapid Deployment

### System Requirements

| Requirement | Minimum | Recommended |
| :--- | :--- | :--- |
| **OS** | Any Linux (Debian, Ubuntu, Kali, Fedora, Arch, Termux) | Kali Linux / Fedora Security |
| **Architecture** | `amd64` (x86_64) or `arm64` (aarch64) | `amd64` for full tool access |
| **RAM** | 512 MB | 2 GB+ for full-spectrum missions |
| **Disk** | 50 MB (binary only) | 500 MB (with tool suite) |
| **Privileges** | `sudo` for APT install only | Rootless runtime |
| **NVIDIA NIM Key** | Required for AI reasoning | [Get free key →](https://build.nvidia.com/) |

> [!TIP]
> **Zero Runtime Dependencies.** MYTH ships as a fully static native binary. You do **NOT** need Rust, Python, Go, Node.js, or any interpreter installed on your machine to run the agent. The binary is self-contained.

---

### ⚡ Method 1 — One-Line Installer (Recommended)

Sets up the signed APT repository, installs the binary system-wide, provisions the Lightpanda browser engine, and guides you through first-run configuration interactively:

```bash
curl -sSL https://myth.work.gd/install.sh | sudo bash
```

Install a specific pinned version:
```bash
curl -sSL https://myth.work.gd/install.sh | sudo VERSION=0.1.0 bash
```

> [!NOTE]
> The installer script verifies GPG signature, architecture compatibility, and OS version before proceeding. It is safe to run on a live Kali system.

---

### 📦 Method 2 — APT Repository (Persistent Updates via `apt upgrade`)

Configure the MYTH signed APT repository once. All future updates arrive automatically with `sudo apt upgrade`:

```bash
# 1. Add the GPG signing key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://myth.work.gd/myth.gpg \
  | sudo gpg --dearmor -o /etc/apt/keyrings/myth.gpg

# 2. Add the repository source
echo "deb [signed-by=/etc/apt/keyrings/myth.gpg arch=amd64,arm64] \
  https://myth.work.gd stable main" \
  | sudo tee /etc/apt/sources.list.d/myth.list

# 3. Install
sudo apt update && sudo apt install myth
```

Or use the bootstrap helper:
```bash
curl -sSL https://myth.work.gd/bootstrap.sh | sudo bash
sudo apt update && sudo apt install myth
```

---

### ⚡ Method 3 — Pre-Built Binary (Fastest, No APT)

Downloads the correct native binary for your hardware with automatic architecture detection:

```bash
ARCH=$(uname -m)
LATEST=$(curl -fsSL https://api.github.com/repos/myth-tools/MYTH-CLI/releases/latest \
  | grep tag_name | cut -d'"' -f4)
curl -fsSL \
  "https://github.com/myth-tools/MYTH-CLI/releases/download/${LATEST}/myth-${ARCH}-unknown-linux-gnu" \
  -o /tmp/myth
chmod +x /tmp/myth
sudo mv /tmp/myth /usr/local/bin/myth
myth --version
```

---

### 📦 Method 4 — RPM Repository (Fedora / RHEL / CentOS)

Configure the MYTH signed RPM repository for high-speed package management with `dnf` or `yum`:

```bash
# 1. Create the repository definition
sudo tee /etc/yum.repos.d/myth.repo << REPOEOF
[myth]
name=MYTH Official Repository
baseurl=https://myth.work.gd/rpm
enabled=1
gpgcheck=1
repo_gpgcheck=1
gpgkey=https://myth.work.gd/myth.gpg
type=rpm
REPOEOF

# 2. Install
sudo dnf install myth
```

---

### 📦 Method 5 — Arch Linux / Manjaro (Pacman)

Add the native MYTH repository to your `pacman.conf` for 100% industrial-grade Arch alignment:

```bash
# 1. Append the repository to pacman.conf
sudo tee -a /etc/pacman.conf << ARCHEOF

[myth]
SigLevel = PackageOptional
Server = https://myth.work.gd/arch
ARCHEOF

# 2. Sync and Install
sudo pacman -Syu myth
```

---

### 📱 Method 6 — Termux (Android / ARM64)

Automated deployment for mobile tactical operations. MYTH auto-detects its environment and adjusts all internal paths to `$PREFIX`:

```bash
# Direct Activation
pkg install myth

# Fast Run (One-liner alternative)
curl -sSL https://myth.work.gd/install.sh | bash
```

---

## 🌐 Universal Package Runners

MYTH is deployable across **7 global ecosystems**. Every runner downloads the correct native binary for your architecture — zero build time, zero Rust toolchain required.

> [!NOTE]
> Due to registry naming constraints: NPM package = `@myth-tools/myth` · PyPI package = `myth-cli` · The installed binary is always `myth`.

---

### 📦 APT / RPM / PACMAN (Native Packages — Best Integration)

```bash
# APT (Debian, Kali, Ubuntu)
sudo apt update && sudo apt install myth

# RPM (Fedora, RHEL)
sudo dnf install myth

# Pacman (Arch Linux, Manjaro)
sudo pacman -U myth-*.pkg.tar.zst

# Global Remove
sudo apt remove myth || sudo dnf remove myth || sudo pacman -R myth
```

---

### 🟨 JavaScript — NPM / Bun / PNPM

```bash
# Zero-install ephemeral execution (auto-downloads native binary on first run)
npx @myth-tools/myth scan <target>
bunx @myth-tools/myth scan <target>
pnpm dlx @myth-tools/myth scan <target>

# Permanent global install
npm install -g @myth-tools/myth
bun add -g @myth-tools/myth
```

---

### 🐍 Python — PyPI / UVX / pipx

```bash
# Recommended: zero-install ephemeral run (no pollution to your Python environment)
uvx myth-cli scan <target>

# Install into isolated environment (persistent)
uv tool install myth-cli
pipx install myth-cli

# Traditional pip (not recommended — prefer the above)
pip install myth-cli
```

---

### 🐳 Container — Docker / Podman / OCI

```bash
# Docker (with full privilege for sandbox namespaces)
docker run -it --rm --privileged ghcr.io/myth-tools/myth:latest scan <target>

# With persistent config volume
docker run -it --rm --privileged \
  -v ~/.config/myth:/root/.config/myth \
  ghcr.io/myth-tools/myth:latest scan <target>

# Rootless Podman (requires kernel user namespace support, --privileged needed for bwrap)
podman run -it --rm --privileged ghcr.io/myth-tools/myth:latest scan <target>

# Pull latest
docker pull ghcr.io/myth-tools/myth:latest
```

---

### 🎯 Snap (Ubuntu Store)

```bash
# Install from Ubuntu Snap Store
sudo snap install myth

# Run
myth scan <target>

# Update
sudo snap refresh myth
```

> [!NOTE]
> The snap uses `classic` confinement — required to allow Bubblewrap namespace creation and raw network sockets (Nmap). This is intentional and safe.

---

### 🦀 Rust / Cargo

```bash
# Fast binary install (no compile, uses pre-built binary from crates.io)
cargo binstall myth

# Compile from source with Cargo.lock pinning (reproducible build)
cargo install myth --locked

# Run from source (development)
git clone https://github.com/myth-tools/MYTH-CLI.git
cd MYTH-CLI && cargo run --release -- scan <target>
```

---

### ❄️ Nix (Flakes — Hermetic & Reproducible)

```bash
# Instant ephemeral run (no install, fully hermetic)
nix run github:myth-tools/MYTH-CLI?dir=package_runners -- scan <target>

# Open development shell with all dependencies
nix develop github:myth-tools/MYTH-CLI?dir=package_runners

# Add to your NixOS/home-manager config
{
  inputs.myth.url = "github:myth-tools/MYTH-CLI?dir=package_runners";
  # ...
}
```

---

## ⚙️ Mandatory First-Run Setup

Complete these three steps immediately after any installation method.

### Step 1 — Set Your NVIDIA NIM API Key

MYTH's AI reasoning engine requires an NVIDIA NIM API key. [**Get one free at build.nvidia.com**](https://build.nvidia.com/) — no GPU required, cloud inference runs on NVIDIA's infrastructure.

Edit your config file directly:

```bash
# Open your config file
$EDITOR ~/.config/myth/user.yaml
```

```yaml
# ~/.config/myth/user.yaml
provider:
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    # Add multiple keys for automatic rotation on rate-limit (recommended for heavy use)
    # - "nvapi-yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"

  # Verified NVIDIA NIM model slugs — full catalog: https://build.nvidia.com/explore/reasoning
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"
```

> [!IMPORTANT]
> The `api_keys` field accepts a **list** — add multiple keys to enable automatic rotation when one hits a rate limit. This is strongly recommended for long-duration missions.

---

### Step 2 — Synchronize Tool Arsenal

```bash
myth sync
```

Downloads, indexes, and caches definitions for 3,000+ Kali Linux tools for AI-assisted orchestration. Run once, then again after major Kali updates.

---

### Step 3 — Validate Operational Environment

```bash
myth check
```

Performs a complete pre-mission audit:
- ✅ Bubblewrap sandbox integrity
- ✅ NVIDIA NIM API key validity & model access
- ✅ Network connectivity & DNS resolution
- ✅ Lightpanda browser engine presence
- ✅ Tool availability and Kali arsenal coverage
- ✅ Shell completion installation
- ✅ Font provision status

---

## 🎯 Mission Control — Command Reference

### 🔥 Core Recon Operations

| Command | Description |
| :--- | :--- |
| `myth scan <target>` | **Full-Spectrum Autonomous Recon** — Executes all 13 phases with full AI reasoning. Covers org mapping, OSINT, asset discovery, active scanning, web app audit, vuln assessment, and full attack surface synthesis. |
| `myth stealth <target>` | **Passive Intelligence Gathering** — Zero active packets sent to the target. Pure OSINT: WHOIS, certificate transparency, public archives, breach databases, DNS passive records only. |
| `myth osint <target>` | **Open Source Intelligence** — Deep OSINT mode: employee enumeration, social media analysis, executive profiling, public document mining, breach exposure correlation. |
| `myth web <target>` | **Web Application Audit** — Directory discovery, API endpoint mapping, JS source analysis, parameter fuzzing, authentication boundary testing, client-side secret extraction via Lightpanda. |
| `myth vuln <target>` | **Vulnerability Assessment** — CVE-database checks, default credential sweeps, misconfiguration scanning, exposed admin panel detection, version fingerprinting. |
| `myth subdomains <target>` | **Subdomain Intelligence** — Multi-source enumeration: certificate transparency, DNS brute-force, reverse IP, Tor-routed lookups, elite wordlists (SecLists + AmassDB). |
| `myth subdomains <target> --master` | **Deep Extraction Mode** — All sources, extended wordlists, 48-hour passive monitoring with delta reporting for newly appearing subdomains. |
| `myth chat` | **Interactive AI Session** — Direct conversation with the MYTH neural core. Ask tactical questions, request targeted tool runs, analyze specific findings, or refine the engagement strategy. |
| `myth findings` | **Mission Intelligence Dashboard** — Aggregate, filter, visualize, and export all discovered assets, vulnerabilities, credentials, and subdomains for the current target. |

---

### 🔧 System & Utility Commands

| Command | Description |
| :--- | :--- |
| `myth sync` | Synchronize tool arsenal, update AI model definitions, and refresh Kali tool catalog |
| `myth check` | Full operational environment health audit — validates sandbox, API, tools, browser, fonts |
| `myth config` | View or interactively modify operative configuration (`~/.config/myth/user.yaml`) |
| `myth typography` | Provision the Elite Tier typography suite — 10 premium Nerd Fonts for perfect TUI rendering |
| `myth update` | Update MYTH binary to the latest release (via APT, binary check, or cargo) |
| `myth version` | Display version, build metadata, and dependency info |
| `myth burn` | **⚠️ Protocol Zero** — Immediate process termination, volatile data wipe, operational shutdown |
| `myth target <new-target>` | Rotate mission focus to a new target domain/IP/CIDR, preserving prior session context |
| `myth tools` | Catalog and display all synchronized mission assets, tool versions, and availability status |

---

### 🎛️ Global Flags

```bash
# Override log verbosity (default: error)
myth scan example.com --log-level debug     # trace | debug | info | warn | error

# Disable the TUI (use plain text output — useful for CI/CD or piping)
myth scan example.com --no-tui

# Disable Bubblewrap sandbox (NOT RECOMMENDED — only for debugging)
myth scan example.com --no-sandbox

# Use a custom config file
myth scan example.com --config /path/to/custom/user.yaml

# Get help on any command
myth scan --help
myth --help
```

---

### 🐚 Shell Completions

MYTH installs shell completions automatically with the APT package. For other install methods:

```bash
# Bash — add to ~/.bashrc
source /usr/share/bash-completion/completions/myth

# Zsh — add to ~/.zshrc
fpath=(/usr/share/zsh/vendor-completions $fpath)
autoload -Uz compinit && compinit

# Fish — completions activate automatically via vendor directory
# Manual path: /usr/share/fish/vendor_completions.d/myth.fish
```

Press `Tab` to get completion suggestions for commands, flags, profiles, and target arguments.

---

## 📡 The Elite 13-Phase Recon Methodology

MYTH follows a professional **89-step intelligence framework** covering all **13 phases (Phase 0 through Phase 12)** with full AI guidance, dynamic pivoting, and continuous context accumulation at every stage.

This isn't a linear checklist — it is an **adaptive, graph-driven mission** where each phase feeds intelligence into subsequent phases, and the AI makes real-time decisions about which techniques to prioritize.

<details>
<summary><b>📂 Click to Expand: All 13 Tactical Phases</b></summary>

<br/>

| Phase | Name | Core Operations |
| :---: | :--- | :--- |
| **0** | 🏢 **Organizational Mapping** | Root domain enumeration, ASN discovery and IP range mapping, reverse WHOIS lookups, corporate subsidiary graph construction, technology stack fingerprinting |
| **1** | 👤 **Identity & Credential Intel** | Employee enumeration via LinkedIn/Hunter.io, breach database correlation (HaveIBeenPwned, DehashDB), default credential inference, organizational password policy analysis |
| **2** | 🌐 **Asset Discovery** | Full-spectrum DNS enumeration, subdomain extraction (200+ sources), certificate transparency mining (crt.sh, Censys), cloud asset discovery (S3, Azure Blobs, GCS), IP range walking |
| **3** | 🔍 **Active Reconnaissance** | Port scanning (Nmap, Masscan), service fingerprinting and version detection, WAF/CDN identification, load balancer mapping, banner grabbing, network topology enumeration |
| **4** | 📁 **Content & App Discovery** | Web directory bruteforcing (ffuf + SecLists), source code leak detection, admin panel discovery, backup file scanning, API endpoint mapping and OpenAPI spec extraction |
| **5** | 🔗 **Supply Chain Analysis** | Third-party JavaScript library enumeration, CDN provider mapping, SaaS dependency identification, npm/pip package vulnerability cross-reference, dependency confusion attack surface |
| **6** | 🤖 **AI/ML Attack Surface** | ML model file detection (`.pkl`, `.pt`, `.onnx`), training data exposure, NVIDIA/HuggingFace inference endpoint discovery, model stealing vector identification |
| **7** | 🖱️ **Dynamic Interaction** | JavaScript-heavy page crawling via Lightpanda, parameter discovery and fuzzing, file upload endpoint testing, authentication boundary mapping, session token entropy analysis |
| **8** | ⚠️ **Vulnerability Assessment** | Known CVE exploitation checks, default credential sweeps, misconfiguration scanning (Nuclei templates), exposed debug interfaces, outdated software version correlation |
| **9** | 🔑 **Secrets & Exposures** | Public git repository mining (gitleaks, trufflehog), cloud storage leak detection, `.env` file exposure, API key pattern scanning in JS, Pastebin and code sharing site monitoring |
| **10** | 👁️ **Social Engineering & OSINT** | Public document metadata extraction, social media intelligence profiling, executive OSINT dossier construction, physical location correlation, organizational chart mapping |
| **11** | 📡 **Continuous Monitoring** | DNS delta tracking (new subdomains, IP rotations), certificate transparency alerting, SSL cert expiry monitoring, service change detection, new port/service alerts |
| **12** | 🎯 **Attack Surface Synthesis** | Critical asset identification and prioritization, risk scoring matrix, full attack graph construction, professional report generation (Markdown + JSON), actionable finding summaries |

</details>

---

## 🛡️ Security Architecture — Defense in Depth

> [!IMPORTANT]
> **You are fully protected.** MYTH wields some of the most powerful offensive tools ever created — yet its operator is shielded by multiple independent layers of security.

MYTH implements a **4-layer defense-in-depth model** that is active on every mission:

```
╔══════════════════════════════════════════════════════════╗
║               MYTH Defense-in-Depth Model                ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  Layer 1: Read-Only Host Namespace (Bubblewrap)          ║
║  ├─ All tool subprocesses run in ephemeral namespaces    ║
║  ├─ /etc, /usr, /home mounted read-only                  ║
║  ├─ No network namespace isolation breakout possible     ║
║  └─ Rogue tools CANNOT modify your host system           ║
║                                                          ║
║  Layer 2: OPSEC Network Routing (Tor SOCKS5)             ║
║  ├─ Optional Tor routing for passive OSINT operations    ║
║  ├─ DNS queries routed through Tor exit nodes            ║
║  └─ Full source IP anonymization for sensitive missions  ║
║                                                          ║
║  Layer 3: Weapon Hardening & Rate Limiting               ║
║  ├─ nmap: forced Connect Scan (no raw socket abuse)      ║
║  ├─ sqlmap: batch mode (no interactive hangs)            ║
║  ├─ ffuf/dirbuster: rate-limited (no accidental DoS)     ║
║  └─ AI validates tool flags before every subprocess call ║
║                                                          ║
║  Layer 4: Forensic Elimination (Protocol Zero)           ║
║  ├─ All operative data is DRAM-resident by default       ║
║  ├─ myth burn = instant SIGKILL + zero disk trace        ║
║  └─ Encrypted persistence optional (AES-GCM-SIV)         ║
╚══════════════════════════════════════════════════════════╝
```

### 🔐 Cryptographic Stack

When session persistence is enabled, MYTH encrypts all stored intelligence using:

- **`aes-gcm-siv`** — Authenticated encryption with nonce misuse resistance (AEAD)
- **`chacha20poly1305`** — High-performance stream cipher for low-latency mobile environments

### 📢 Responsible Disclosure

Found a vulnerability in MYTH itself? Email **[shesher0llms@gmail.com](mailto:shesher0llms@gmail.com)** with `[SECURITY]` in the subject line. We acknowledge within 24 hours and patch within 48 hours.

---

## ⚙️ Configuration Reference

All operator configuration lives in `~/.config/myth/user.yaml`. The system default template is in `config/user.yaml` (shipped with the package at `/etc/myth/user.yaml`).

```yaml
# ─── ~/.config/myth/user.yaml ───────────────────────────────────────────────
# Your overrides here take priority over /etc/myth/user.yaml (system defaults).
# Lines starting with # are comments — uncomment and fill in to activate.

# ─── Identity ────────────────────────────────────────────────────────────────
agent:
  user_name: "Chief"           # Your operative handle (displayed in session headers)
  log_level: "error"           # trace | debug | info | warn | error
  all_report_path: ""          # Empty = auto-resolve to ~/Downloads at runtime

# ─── AI Provider ─────────────────────────────────────────────────────────────
# Get your free NVIDIA NIM key at: https://build.nvidia.com/
provider:
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    # Add more keys for automatic rotation on rate-limit:
    # - "nvapi-yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy"

  # Primary and fallback models (verified NVIDIA NIM slugs as of 2026-Q1)
  # Full model catalog: https://build.nvidia.com/explore/reasoning
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"

  base_url: "https://integrate.api.nvidia.com/v1"
  temperature: 0.5
  max_tokens: 131072            # 128K — maximum for NIM inference endpoints

# ─── Recon Profiles ──────────────────────────────────────────────────────────
# "elite"  — Full AI autonomy, all 13 phases, all tools available
# "custom" — You define which tools are used at each phase
active_profile: "elite"

# ─── MCP Server Extensions ───────────────────────────────────────────────────
# Add external MCP tool servers for custom tool integrations:
# mcp_servers:
#   my-custom-server:
#     command: /path/to/mcp-server
#     args: ["--port", "3000"]
```

### 🌍 Environment Variable Overrides

All config values can be overridden at runtime without editing files:

```bash
# Override API key for a single session
NVIDIA_API_KEY=nvapi-xxx myth scan example.com

# Verbose logging for debugging
LOG_LEVEL=debug myth check

# Custom configuration file
myth scan example.com --config ~/configs/client-engagement.yaml

# Disable TUI for CI scripting
NO_TUI=1 myth scan example.com
```

---

## 🧰 Technology Stack

MYTH is built on a carefully selected, industry-grade dependency stack:

| Layer | Technology | Purpose |
| :--- | :--- | :--- |
| **Language** | Rust 1.75+ (MSRV) | Memory safety, native performance, static binary |
| **Async Runtime** | `tokio` (full features) | High-concurrency async orchestration |
| **AI Brain** | `rig-core` + `reqwest` | NVIDIA NIM LLM integration with streaming |
| **MCP Protocol** | `rust-mcp-sdk` | Tool discovery and extensibility |
| **TUI** | `ratatui` + `crossterm` | Mission dashboard and interactive display |
| **CLI** | `clap` (derive) + `clap_complete` | Command parsing and shell completion generation |
| **Sandbox** | Bubblewrap (`bwrap`) | Linux user namespace isolation |
| **Browser** | Lightpanda (Zig) | Headless JS-capable web reconnaissance |
| **Encryption** | `aes-gcm-siv` + `chacha20poly1305` | Mission data authenticated encryption |
| **DNS** | `trust-dns-resolver` (DNSSEC) | Secure, validating DNS resolution |
| **HTTP** | `reqwest` (SOCKS5, brotli, zstd) | HTTP with Tor proxy and modern compression |
| **OSINT** | `scraper` + `fantoccini` | HTML parsing and WebDriver automation |
| **State** | `petgraph` + `dashmap` | Concurrent state graph for mission tracking |
| **Serialization** | `serde` + `serde_yaml` + `zstd` | Config loading and compressed storage |
| **Release Opt.** | `lto = fat`, `codegen-units = 1` | Maximum binary performance |

---

## 📊 MYTH vs the Competition

| Capability | **MYTH** | Manual Kali Workflow | Commercial DAST (Burp Suite Pro) | AutoRecon | Metasploit |
| :--- | :---: | :---: | :---: | :---: | :---: |
| AI Reasoning & Autonomous Pivoting | ✅ Real-time | ❌ Manual | ⚠️ Limited AI | ❌ None | ❌ None |
| 3,000+ Tool Orchestration | ✅ Native | ⚠️ Manual chain | ❌ Vendor scope | ✅ ~30 tools | ⚠️ Modules only |
| Hardened Isolation Sandbox | ✅ Bubblewrap | ❌ None | ⚠️ VM required | ❌ None | ❌ None |
| Forensic Elimination (RAM-only) | ✅ Native | ❌ Disk logs | ❌ Cloud logs | ❌ Disk logs | ❌ Disk logs |
| Sub-2ms Startup (Native Binary) | ✅ Rust static | ⚠️ Python/Shell | ❌ JVM overhead | ⚠️ Python 3s+ | ⚠️ Ruby overhead |
| JS Reconnaissance (Headless) | ✅ Lightpanda (11×) | ⚠️ Playwright | ✅ Chromium-based | ❌ None | ❌ None |
| ARM64 / Kali Nethunter | ✅ Native binary | ✅ Manual | ❌ Typically x86 | ⚠️ Partial | ⚠️ Limited |
| MCP Protocol Extensibility | ✅ Native | ❌ Custom scripts | ❌ Proprietary API | ❌ None | ❌ None |
| Single Static Binary Deploy | ✅ Yes | ❌ Dozens of tools | ❌ Agent + server | ⚠️ Python deps | ⚠️ Ruby deps |
| OPSEC Tor Integration | ✅ Native | ⚠️ Manual proxychains | ❌ No | ❌ No | ⚠️ Manual |
| Persistent Session Memory | ✅ Cross-session | ❌ Manual notes | ⚠️ Project files | ❌ None | ⚠️ Workspaces |
| Automated Key Rotation (Multi-key) | ✅ Native | N/A | ❌ Single tenant | N/A | N/A |
| Price | ✅ **Free (MIT)** | ✅ Free | ❌ $449+/year | ✅ Free | ✅ Community Free |

---

## 🤝 Contributing

We welcome contributions from the global security research community. MYTH is built on the principle that the best offensive tools are community-hardened and openly reviewed.

### Development Setup

```bash
# 1. Clone the repository
git clone https://github.com/myth-tools/MYTH-CLI.git
cd MYTH-CLI

# 2. Install Rust stable toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# 3. Install system dependencies (Debian/Kali)
sudo apt install -y bubblewrap

# 4. Run the full quality verification suite
bash scripts/verify.sh
# → Runs: fmt, clippy, build (--locked), unit tests, doc tests, cargo doc

# 5. Build debug binary
cargo build

# 6. Run tests only
cargo test --workspace --locked

# 7. Build release binary locally
bash scripts/build_deb.sh
```

### Contribution Guidelines

1. **Fork** the repository and create a focused feature branch:
   ```bash
   git checkout -b feature/add-httpx-integration
   ```
2. **Write tests** — every behavioral change must have test coverage
3. **Run the verifier** — `bash scripts/verify.sh` must pass completely (format, clippy, build, test, doc)
4. **Commit clearly** — use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`
5. **Open a Pull Request** with a clear description of what changed and why

### What We're Looking For

| Contribution Type | Priority |
| :--- | :--- |
| 🔧 New MCP tool integrations | ⭐⭐⭐ High |
| 🐛 Bug reports & reproducers | ⭐⭐⭐ High |
| 🧪 Test coverage for Debian/Ubuntu variants | ⭐⭐ Medium |
| 📖 Documentation & tutorial improvements | ⭐⭐ Medium |
| 🌍 New reconnaissance techniques & phases | ⭐⭐⭐ High |
| 🎨 TUI/UX improvements | ⭐ Low |
| 🌐 Localization / translation | ⭐ Low |

**[→ Open an Issue](https://github.com/myth-tools/MYTH-CLI/issues)** · **[→ Browse PRs](https://github.com/myth-tools/MYTH-CLI/pulls)** · **[→ Join Discussions](https://github.com/myth-tools/MYTH-CLI/discussions)**

---

## ❓ Frequently Asked Questions

<details>
<summary><b>🔒 Is MYTH legal to use?</b></summary>

<br/>

MYTH is provided strictly for **authorized security research, penetration testing, and educational purposes**. You must have **explicit written authorization** from the target owner before running any reconnaissance or scanning. Using MYTH against unauthorized targets violates the Computer Fraud and Abuse Act (CFAA), the EU NIS2 Directive, and equivalent laws worldwide.

The creators assume **zero liability** for unauthorized use. See the full [Legal Disclaimer](#%EF%B8%8F-legal--operational-disclaimer).

</details>

<details>
<summary><b>💻 Does MYTH require an NVIDIA GPU?</b></summary>

<br/>

**No GPU required.** MYTH uses **NVIDIA NIM cloud inference** — all AI reasoning runs on NVIDIA's remote infrastructure. Any CPU-only machine, VPS, or Raspberry Pi 4 with internet connectivity works perfectly. The free NIM tier is generous enough for typical engagement workloads.

</details>

<details>
<summary><b>📱 Does MYTH work on Kali Nethunter or Termux?</b></summary>

<br/>

**Yes.** MYTH ships native `arm64` binaries specifically for Kali Nethunter, Termux on Android, and Raspberry Pi (aarch64). Install via the one-line installer (which auto-detects architecture) or download the `arm64` binary directly from [GitHub Releases](https://github.com/myth-tools/MYTH-CLI/releases/latest).

Note: Bubblewrap sandbox requires kernel user namespace support. On some Android kernels (Nethunter), this may need to be enabled via a kernel patch.

</details>

<details>
<summary><b>🐧 What if I don't have Kali Linux?</b></summary>

<br/>

MYTH runs natively on **Fedora, Arch Linux, Termux, and any Debian-based Linux** (Ubuntu, Parrot OS, Pop!_OS, Tails, etc.). Without Kali's pre-installed tool suite, MYTH will gracefully skip unavailable tools and prompt you to install them on first use. The AI automatically adapts its strategy to the tools actually available on your system.

For a minimal setup: install `nmap`, `whois`, `curl`, and `git` — MYTH's OSINT phases work with just these core tools.

</details>

<details>
<summary><b>🔄 How do I update MYTH?</b></summary>

<br/>

```bash
# Via APT (recommended — receives updates automatically with system upgrades)
sudo apt update && sudo apt upgrade myth

# Via the installer (any install method)
curl -sSL https://myth.work.gd/install.sh | sudo bash

# Via the built-in updater
myth update
```

</details>

<details>
<summary><b>🗑️ How do I completely remove MYTH and all its data?</b></summary>

<br/>

```bash
# Full sanitization via the decommission script
curl -sSL https://myth.work.gd/uninstall.sh | sudo bash

# Via APT (removes binary + system files, keeps ~/.config/myth)
sudo apt remove myth

# Via APT (removes everything including config and signing key)
sudo apt purge myth
```

The uninstall script removes: binary, APT source, signing key, `~/.config/myth/`, the Lightpanda engine, and all provisioned fonts.

</details>

<details>
<summary><b>🤖 Which AI models does MYTH support?</b></summary>

<br/>

MYTH uses the **NVIDIA NIM inference API** which hosts hundreds of models. Recommended models for penetration testing:

| Model | Best For |
| :--- | :--- |
| `deepseek-ai/deepseek-r1` | Complex multi-step reasoning (default) |
| `nvidia/llama-3.1-nemotron-70b-instruct` | Fast, reliable fallback |
| `qwen/qwen3-next-80b-a3b-thinking` | Extended context, deep analysis |
| `meta/llama-3.3-70b-instruct` | Cost-efficient bulk operations |

Browse the full catalog at [build.nvidia.com/explore/reasoning](https://build.nvidia.com/explore/reasoning).

</details>

<details>
<summary><b>🛡️ Can a malicious target compromise my machine through MYTH?</b></summary>

<br/>

By design, **no** — when operating normally with the Bubblewrap sandbox enabled. The isolation architecture ensures:

- Tool subprocesses mount the host filesystem read-only
- No subprocess can write to `/etc`, `/usr`, `/home`, or any sensitive path
- Network namespacing prevents unauthorized outbound connections
- Memory sandbox prevents process escape

Disabling the sandbox (`--no-sandbox`) removes these protections and is **never recommended**.

</details>

---

## 🗡️ Tactical Decommissioning — Full Purge

To completely remove MYTH from a system — binary, APT repository, signing key, configuration, session data, and all provisioned engines:

```bash
curl -sSL https://myth.work.gd/uninstall.sh | sudo bash
```

> [!CAUTION]
> **Total Sanitization.** This permanently removes:
> - `/usr/bin/myth` (binary)
> - `/etc/apt/sources.list.d/myth.list` (APT source)
> - `/etc/apt/keyrings/myth.gpg` (signing key)
> - `~/.config/myth/` (your configuration, session history, reports)
> - `~/.local/bin/lightpanda` (browser engine)
> - All provisioned terminal fonts
>
> **This action cannot be undone.** Back up `~/.config/myth/user.yaml` if you want to preserve your API keys.

---

## 👤 Creator

<div align="center">

**MYTH** is architected and maintained by:

### Shesher Hasan
**Chief Architect · myth-tools**

[shesher0llms@gmail.com](mailto:shesher0llms@gmail.com) &nbsp;·&nbsp; [GitHub: @myth-tools](https://github.com/myth-tools) &nbsp;·&nbsp; [myth.work.gd](https://myth.work.gd)

*OPERATIVE-LEVEL-4 · MYTH Institutional Architecture · 2026*

<br/>

[![Sponsor](https://img.shields.io/badge/Sponsor-myth--tools-ff69b4?style=for-the-badge&logo=github-sponsors&logoColor=white)](https://github.com/sponsors/myth-tools)

</div>

---

## ⚖️ Legal & Operational Disclaimer

> [!CAUTION]
> **AUTHORIZED USE ONLY — READ BEFORE PROCEEDING**
>
> MYTH is an unrestricted offensive security instrument. It is provided **for educational purposes and authorized penetration testing only**.
>
> **By using MYTH, you confirm:**
> - You hold **explicit written authorization** from the owner of every target system you test
> - You understand that unauthorized use violates the **Computer Fraud and Abuse Act (CFAA)**, the **EU NIS2 Directive**, the **UK Computer Misuse Act**, and equivalent laws in your jurisdiction
> - You accept **full personal legal responsibility** for your operational decisions
>
> **The creators, contributors, and maintainers of MYTH assume zero liability** for unauthorized, illegal, unethical, or malicious use of this software.
>
> **Deploy with precision. Operate with responsibility.**

---

## 📄 License

MIT License — see [LICENSE](LICENSE) for full terms.

```
MIT License · Copyright (c) 2026 myth-tools (Shesher Hasan)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software...
```

---

<!-- SEO FOOTER — KEYWORDS FOR SEARCH ENGINE INDEXING -->
<!-- ai penetration testing tool · kali linux ai agent · autonomous reconnaissance · osint automation · 
     nvidia nim security · rust security tool · ai hacking tool · automated recon framework · 
     kali linux automation · offensive security ai · myth cli · myth reconnaissance · 
     bubblewrap sandbox security · lightpanda browser · mcp security tools · deepseek security · 
     arm64 kali nethunter · debian security tools · ai red team · autonomous osint -->

<div align="center">

---

**⚡ MYTH — The Operative Never Sleeps ⚡**

*Autonomous Intelligence. Absolute Precision. Zero Compromise.*

<br/>

[![⭐ Star MYTH on GitHub](https://img.shields.io/github/stars/myth-tools/MYTH-CLI?style=social&label=⭐%20Star%20MYTH)](https://github.com/myth-tools/MYTH-CLI)
&nbsp;&nbsp;
[![Follow @myth-tools](https://img.shields.io/github/followers/myth-tools?style=social&label=Follow%20%40myth-tools)](https://github.com/myth-tools)
&nbsp;&nbsp;
[![Watch for Updates](https://img.shields.io/github/watchers/myth-tools/MYTH-CLI?style=social&label=Watch%20for%20Updates)](https://github.com/myth-tools/MYTH-CLI)

<br/>

[🌐 myth.work.gd](https://myth.work.gd) &nbsp;·&nbsp; [📦 All Releases](https://github.com/myth-tools/MYTH-CLI/releases) &nbsp;·&nbsp; [🐛 Report Issue](https://github.com/myth-tools/MYTH-CLI/issues) &nbsp;·&nbsp; [💬 Discussions](https://github.com/myth-tools/MYTH-CLI/discussions) &nbsp;·&nbsp; [❤️ Sponsor](https://github.com/sponsors/myth-tools)

</div>
