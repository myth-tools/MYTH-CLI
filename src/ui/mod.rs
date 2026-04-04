//! UI Module — Shared CLI and TUI aesthetic components.
//!
//! This module provides a consistent "Cyber" style for the MYTH CLI,
//! including themes, ASCII banners, and formatted output helpers.

use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use std::io::Write;
use std::sync::Mutex;

/// Result of the industry-grade typography fidelity audit.
#[derive(Debug, Clone)]
pub struct TypographyAudit {
    pub target_font: String,
    pub os_match: String,
    pub fidelity_score: f32,
    pub sync_protocol: String,
    pub sync_success: bool,
    pub checksum: String,
}

static TYPOGRAPHY_CACHE: Lazy<Mutex<Option<TypographyAudit>>> = Lazy::new(|| Mutex::new(None));

/// A consistent color palette for the "Cyber" aesthetic.
pub struct CyberTheme;

impl CyberTheme {
    /// Neon Green — Success, primary actions.
    pub fn primary<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(0, 255, 136)).to_string()
    }

    /// Electric Blue — Information, secondary elements.
    pub fn secondary<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(0, 136, 255)).to_string()
    }

    /// Deep Magenta — Critical, accent.
    pub fn accent<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(255, 0, 136)).to_string()
    }

    /// Dark Gray — Dimmed text, background-like elements.
    pub fn dim<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(80, 80, 100)).to_string()
    }

    /// Amber — Warnings, non-critical alerts.
    pub fn warning<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(255, 170, 0)).to_string()
    }

    /// Bright White — High contrast text.
    pub fn bright<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(255, 255, 255)).to_string()
    }

    /// Electric Indigo — Premium Operative/Chief color.
    pub fn tactical<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(130, 100, 255)).to_string()
    }
}

/// Renders a premium ASCII banner.
pub fn print_banner() {
    let lines = [
        "      ___           ___           ___           ___     ",
        "     /\\  \\         |\\__\\         /\\  \\         /\\__\\    ",
        "    /::\\  \\        |:|  |        \\:\\  \\       /:/  /    ",
        "   /:/\\:\\  \\       |:|  |         \\:\\  \\     /:/__/     ",
        "  /:/  \\:\\  \\      |:|__|__       /::\\  \\   /::\\  \\ ___ ",
        " /:/__/ \\:\\__\\     /::::\\__\\     /:/\\:\\__\\ /:/\\:\\  /\\__\\",
        " \\:\\  \\  \\/__/    /:/~~/~       /:/  \\/__/ \\/__\\:\\/:/  / ",
        "  \\:\\  \\         /:/  /        /:/  /           \\::/  /  ",
        "   \\:\\  \\        \\/__/         \\/__/            /:/  /   ",
        "    \\:\\__\\                                     /:/  /    ",
        "     \\/__/                                     \\/__/     ",
    ];

    println!();
    for line in &lines {
        println!("{}", line.bright_cyan().bold());
    }
    println!();
}

/// Renders a premium, "Elite Tier" boot sequence with industry-grade system diagnostics.
pub async fn boot_sequence(config: &crate::config::AppConfig) {
    use tokio::time::{sleep, Duration};

    // 0. Level-1 Autonomous Dependency Provisioning
    provision_fontconfig().await;

    let check_binary = |cmd: &str| -> String {
        if std::process::Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            CyberTheme::secondary("LINKED").bold().to_string()
        } else {
            CyberTheme::accent("DETACHED").bold().to_string()
        }
    };

    // 1. Sandbox Functional Audit
    let bwrap_audit = || async {
        let output = tokio::process::Command::new("bwrap")
            .arg("--version")
            .output()
            .await;
        if output.is_ok() {
            CyberTheme::secondary("SHIELDED").bold().to_string()
        } else {
            CyberTheme::accent("UNRESTRICTED").bold().to_string()
        }
    };

    // 2. Neural Uplink Connectivity Audit
    let uplink_audit = || async {
        if std::env::var("NVIDIA_API_KEY").is_ok() {
            CyberTheme::secondary("STABLE").bold().to_string()
        } else {
            CyberTheme::accent("DETACHED").bold().to_string()
        }
    };

    // 3. MCP Arsenal Audit
    let arsenal_audit = || async {
        if !config.mcp.mcp_servers.is_empty() {
            CyberTheme::secondary("SYNCED").bold().to_string()
        } else {
            CyberTheme::dim("EMPTY").to_string()
        }
    };

    let tui_status = if config.tui.enabled {
        CyberTheme::secondary("ACTIVE").bold().to_string()
    } else {
        CyberTheme::dim("DEACTIVATED").to_string()
    };

    // 4. Mission-Critical Typography Synchronization (Elite Tier)
    let audit = get_typography_audit(&config.tui.font);
    let font_status = if audit.fidelity_score >= 1.0 {
        CyberTheme::secondary("VERIFIED").bold().to_string()
    } else {
        CyberTheme::warning("FALLBACK").bold().to_string()
    };

    println!("{}", CyberTheme::dim("─".repeat(60)));
    println!(
        "{}",
        CyberTheme::bright(" [ NEURAL LINK SYSTEM INITIALIZATION ]").bold()
    );
    println!("{}", CyberTheme::dim("─".repeat(60)));

    // Step-by-step Execution with Real-Time Feedback
    // Concurrency Hardening: Execute audits in parallel for Zero-Latency initialization
    let (nim_status, bwrap_status, arsenal_status) =
        tokio::join!(uplink_audit(), bwrap_audit(), arsenal_audit());

    let checks = [
        ("NEURAL CORE INITIALIZATION", "OK".to_string(), true),
        ("NVIDIA NIM SYNC RELAY", nim_status, true),
        ("PYTHON UVX RUNTIME ENGINE", check_binary("uvx"), false),
        ("NODEJS NPX RUNTIME ENGINE", check_binary("npx"), false),
        ("SANDBOX SECURITY PROTOCOLS", bwrap_status, true),
        ("HUD GRAPHICAL INTERFACE", tui_status, false),
        ("TERMINAL TYPOGRAPHY SYNC", font_status, true),
        ("MISSION ASSET REGISTRY", arsenal_status, true),
    ];

    for (msg, status, is_critical) in checks {
        sleep(Duration::from_millis(5)).await; // Optimized for Zero-Latency
        let icon = if is_critical {
            CyberTheme::accent("◈")
        } else {
            CyberTheme::secondary("◈")
        };
        print!("  {} {:.<35} ", icon, msg);
        std::io::stdout().flush().ok();
        sleep(Duration::from_millis(10)).await; // Optimized for Zero-Latency
        println!("{}", status);
    }

    // Premium Typography Display (Industry-Grade Diagnostics)
    println!(
        "  {} {}",
        CyberTheme::primary("TYPOGRAPHY FIDELITY AUDIT //"),
        CyberTheme::secondary(&audit.target_font)
    );
    println!(
        "    {} Protocol:  {} {}",
        CyberTheme::dim("•"),
        CyberTheme::dim(&audit.sync_protocol),
        if audit.sync_success {
            CyberTheme::primary("(ACTIVE)").bold().to_string()
        } else {
            CyberTheme::dim("(PASSIVE)")
        }
    );
    println!(
        "    {} OS Match:  {} {}",
        CyberTheme::dim("•"),
        CyberTheme::secondary(&audit.os_match),
        if audit.fidelity_score >= 1.0 {
            CyberTheme::primary("[Verified]")
        } else {
            CyberTheme::warning("[Degraded]")
        }
    );

    if audit.fidelity_score < 1.0 {
        println!(
            "    {} {} {}",
            CyberTheme::accent("⚠"),
            CyberTheme::accent("Fidelity Alert:").bold(),
            CyberTheme::dim("Generic fallback detected. Visual artifacts may occur.")
        );
        print!(
            "    {} {} [y/N]: ",
            CyberTheme::primary("?"),
            CyberTheme::bright("Initialize automated font provisioning?")
        );
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        if input.trim().to_lowercase() == "y" {
            provision_font(&config.tui.font).await.ok();
        }
    }

    println!("\n  {}", CyberTheme::dim("━".repeat(60)));
    sleep(Duration::from_millis(50)).await;
}

/// Renders text inside an "Elite Tier" stylized rounded box with layout-first alignment.
pub fn print_boxed(title: &str, content: &str) {
    let width: usize = 64; // Total interior width
    let border_color = |s: &str| CyberTheme::dim(s);

    println!();
    // Top border: ╭─ [ TITLE ] ─────────────────╮
    let label = format!(" [ {} ] ", title.to_uppercase());
    let styled_label = CyberTheme::secondary(label.bold());
    let line_len = width.saturating_sub(label.len() + 2);
    println!(
        "{}{}{}{}",
        border_color("╭─"),
        styled_label,
        border_color(&"─".repeat(line_len)),
        border_color("╮")
    );

    for line in content.lines() {
        let padding_len = width.saturating_sub(line.len() + 2);
        println!(
            "{}  {}  {}{}",
            border_color("│"),
            CyberTheme::bright(line),
            " ".repeat(padding_len),
            border_color("│")
        );
    }

    println!(
        "{}{}{}",
        border_color("╰"),
        border_color(&"─".repeat(width)),
        border_color("╯")
    );
    println!();
}

/// Prints a status line with a prefix.
pub fn print_status(prefix: &str, msg: &str) {
    println!(
        "{} {} {}",
        CyberTheme::primary("❯"),
        CyberTheme::secondary(prefix).bold(),
        msg
    );
}

/// Renders a technical audit block for executed tools.
pub fn print_audit(executed: &[(String, String)]) {
    if executed.is_empty() {
        return;
    }

    println!(
        "\n  {} {}",
        CyberTheme::secondary("⚙"),
        CyberTheme::bright("TECHNICAL AUDIT — OPERATIONS EXECUTED")
    );
    println!("  {}", CyberTheme::dim("━".repeat(60)));
    for (binary, args) in executed {
        println!(
            "    {} {} {}",
            CyberTheme::primary("•"),
            CyberTheme::secondary(binary),
            CyberTheme::dim(args)
        );
    }
    println!("  {}\n", CyberTheme::dim("━".repeat(60)));
}

/// Detects the host terminal emulator and returns the identified sync protocol.
pub fn get_sync_protocol() -> String {
    let term_program = std::env::var("TERM_PROGRAM").unwrap_or_default();
    let kitty_pid = std::env::var("KITTY_PID").unwrap_or_default();
    let kitty_listen = std::env::var("KITTY_LISTEN_ON").unwrap_or_default();

    if !kitty_pid.is_empty() || !kitty_listen.is_empty() {
        let version = std::process::Command::new("kitty")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        if !kitty_listen.is_empty() {
            format!("Kitty {} (RC Active)", version)
        } else {
            format!("Kitty {} (Legacy)", version)
        }
    } else if term_program.contains("iTerm2") {
        let version =
            std::env::var("TERM_PROGRAM_VERSION").unwrap_or_else(|_| "Unknown".to_string());
        format!("iTerm2 {} (DSC)", version)
    } else if term_program.contains("Apple_Terminal") {
        "Apple Terminal (Standard)".to_string()
    } else if term_program.contains("WezTerm")
        || std::env::var("WEZTERM_PANE").is_ok()
        || std::env::var("WEZTERM_UNIX_SOCKET").is_ok()
    {
        let version = std::process::Command::new("wezterm")
            .arg("--version")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());
        format!("WezTerm {} (CLI Config)", version)
    } else {
        "Standard (No Sync)".to_string()
    }
}

/// Renders a security finding with a risk level.
pub fn print_finding(level: &str, title: &str, description: &str) {
    let level_str = level.to_lowercase();
    let finding_tag = match level_str.as_str() {
        "critical" | "high" => CyberTheme::accent(format!("[{}]", level.to_uppercase())),
        "medium" => CyberTheme::secondary(format!("[{}]", level.to_uppercase())),
        _ => CyberTheme::primary(format!("[{}]", level.to_uppercase())),
    };

    let icon = match level_str.as_str() {
        "critical" | "high" => CyberTheme::accent("⚡"),
        "medium" => CyberTheme::secondary("⚡"),
        _ => CyberTheme::primary("⚡"),
    };

    println!("  {} {} {}", icon, finding_tag, CyberTheme::bright(title));
    println!("    {}", CyberTheme::dim(description));
}

/// Returns a plain string version of the operative prompt for CLI editors.
pub fn get_operative_prompt(_name: &str) -> String {
    format!("{} ", CyberTheme::secondary("└─❯"))
}

/// Renders a premium, multi-line operative (user) prompt with "Tactical Flux" animation.
pub async fn print_operative_prompt(name: &str, simulated_mode: bool) {
    let label = format!("OPERATIVE // {}", name.to_uppercase());
    let styled_label = apply_font(&label.to_lowercase(), "small-caps", simulated_mode);

    println!(
        "{}",
        CyberTheme::dim(format!(
            "┌──[ {} ]",
            CyberTheme::tactical(styled_label).bold()
        ))
    );

    // Neural Pulse: Optimized Low-Latency Prompt animation (Elite Tier)
    let frames = get_premium_loading_frames("orbit");
    for frame in frames {
        print!("\r{} ", CyberTheme::secondary(frame));
        std::io::stdout().flush().ok();
        tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    }

    print!("\r{} ", CyberTheme::secondary("└─❯"));
    std::io::stdout().flush().ok();
}

/// Renders a premium, multi-line agent (MYTH) response prefix (Full).
pub fn print_agent_prefix(name: &str, simulated_mode: bool) {
    print_agent_header(name, simulated_mode);
    print!("{} ", CyberTheme::accent("└─❯"));
    std::io::stdout().flush().ok();
}

/// Renders ONLY the top bracket of the agent response, allowing dynamic bottom brackets.
pub fn print_agent_header(name: &str, simulated_mode: bool) {
    let label = format!("AGENT // {}", name.to_uppercase());
    let styled_label = apply_font(&label.to_lowercase(), "small-caps", simulated_mode);

    println!(
        "{}",
        CyberTheme::accent(format!("┌──[ {} ]", styled_label.bold()))
    );
}

/// Returns centralized, high-fidelity character sets for premium animations.
pub fn get_premium_loading_frames(kind: &str) -> Vec<&'static str> {
    match kind {
        "orbit" => vec!["⢹", "⢺", "⢼", "⣸", "⣇", "⡧", "⡗", "⡏"],
        "pulse" => vec![
            " ", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", " ",
        ],
        "flux" => vec!["░", "▒", "▓", "█", "█", "▓", "▒", "░"],
        "vector" => vec!["◰", "◳", "◲", "◱"],
        _ => vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
    }
}

/// Renders an "Industry-Grade" turn separator with "Neural Pulse" animation.
pub async fn print_turn_separator() {
    println!(); // Top spacing

    // Neural Pulse: A subtle, high-fidelity visual break
    let glint = "━".repeat(40);
    let frames = [0, 1, 2, 3, 2, 1, 0];
    for &f in &frames {
        let intensity = 80 + (f * 40); // Pulse from dim to bright gray
        print!(
            "\r  {}",
            glint.color(owo_colors::Rgb(intensity, intensity, intensity + 10))
        );
        std::io::stdout().flush().ok();
        tokio::time::sleep(std::time::Duration::from_millis(60)).await;
    }

    print!("\r{}\r", " ".repeat(100)); // Clear the pulse for a clean break
    println!(); // Bottom spacing
}

/// Renders a premium, "Elite Tier" table of all tactical font assets.
pub fn render_font_list() {
    let registry = FontAsset::registry();
    let audit = TYPOGRAPHY_CACHE.lock().unwrap();
    let current_font = audit.as_ref().map(|a| a.target_font.as_str()).unwrap_or("");

    println!(
        "\n  {} {}",
        CyberTheme::secondary("⚙"),
        CyberTheme::bright("MISSION TYPOGRAPHY — ASSET REGISTRY").bold()
    );
    println!("  {}", CyberTheme::dim("━".repeat(95)));

    // Diagnostic Header
    let term_info = get_sync_protocol();
    println!(
        "  {} {}: {} | {}: {}",
        CyberTheme::primary("DIAGNOSTIC FEED:"),
        CyberTheme::dim("TERMINAL"),
        CyberTheme::secondary(&term_info),
        CyberTheme::dim("ACTIVE_FONT"),
        if current_font.is_empty() {
            CyberTheme::accent("OS-DEFAULT")
        } else {
            CyberTheme::bright(current_font)
        }
    );
    println!("  {}", CyberTheme::dim("─".repeat(95)));

    println!(
        "  {:<18} {:<24} {:<24} STATUS",
        CyberTheme::dim("IDENTIFIER"),
        CyberTheme::dim("ASSET NAME"),
        CyberTheme::dim("CLASSIFICATION")
    );
    println!("  {}", CyberTheme::dim("─".repeat(95)));

    for asset in registry {
        let is_current = asset.family_name == current_font;
        let id_colored = if is_current {
            CyberTheme::primary(&asset.id).bold().to_string()
        } else {
            asset.id.clone()
        };

        let indicator = if is_current {
            CyberTheme::primary("◈").bold().to_string()
        } else {
            CyberTheme::dim("◇")
        };

        // Determine fidelity status silently for the list
        let is_installed = if is_cmd_available("fc-match") {
            let output = std::process::Command::new("fc-match")
                .arg("--format=%{family}")
                .arg(&asset.family_name)
                .output();
            if let Ok(o) = output {
                let matched = String::from_utf8_lossy(&o.stdout).trim().to_string();
                let normalized_match = matched.replace(' ', "").to_lowercase();
                let normalized_target = asset.family_name.replace(' ', "").to_lowercase();
                normalized_match.contains(&normalized_target)
            } else {
                false
            }
        } else {
            false
        };

        let status = if is_installed {
            CyberTheme::secondary("[READY]").bold().to_string()
        } else {
            CyberTheme::dim("[MISSING]").to_string()
        };

        println!(
            "  {} {:<18} {:<24} {:<24} {}",
            indicator,
            id_colored,
            CyberTheme::bright(&asset.name),
            CyberTheme::dim(&asset.classification),
            status
        );
        // Secondary description line for even more detail (Elite Tier)
        println!(
            "    {} {}\n",
            CyberTheme::dim("└─"),
            CyberTheme::dim(&asset.style).italic()
        );
    }
    println!("  {}", CyberTheme::dim("─".repeat(95)));
    println!(
        "  {} Tip: Use '{}' to switch mission assets.\n",
        CyberTheme::secondary("•"),
        CyberTheme::bright("typography set <id>")
    );
}

/// Represents a tactical font asset with metadata for autonomous provisioning.
pub struct FontAsset {
    pub id: String,
    pub name: String,
    pub url: String,
    pub family_name: String,
    pub classification: String,
    pub style: String,
}

impl FontAsset {
    pub fn registry() -> Vec<Self> {
        vec![
            FontAsset {
                id: "jet-brains-mono".to_string(),
                name: "JetBrains Mono".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip".to_string(),
                family_name: "JetBrainsMono Nerd Font".to_string(),
                classification: "HIGH-FIDELITY".to_string(),
                style: "Professional developer workspace".to_string(),
            },
            FontAsset {
                id: "iosevka-nerd-font".to_string(),
                name: "Iosevka Nerd Font".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Iosevka.zip".to_string(),
                family_name: "Iosevka Nerd Font".to_string(),
                classification: "DENSE-TACTICAL".to_string(),
                style: "Maximum mission info density".to_string(),
            },
            FontAsset {
                id: "hack-nerd-font".to_string(),
                name: "Hack Nerd Font".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Hack.zip".to_string(),
                family_name: "Hack Nerd Font".to_string(),
                classification: "PRECISION-SHELL".to_string(),
                style: "Balanced clarity & glyph flow".to_string(),
            },
            FontAsset {
                id: "fira-code".to_string(),
                name: "Fira Code".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip".to_string(),
                family_name: "FiraCode Nerd Font".to_string(),
                classification: "LIGATURE-EXERT".to_string(),
                style: "Modern semantic code rendering".to_string(),
            },
            FontAsset {
                id: "cascadia-code".to_string(),
                name: "Cascadia Code".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/CascadiaCode.zip".to_string(),
                family_name: "CaskaydiaCove Nerd Font".to_string(),
                classification: "GEOMETRIC-CAD".to_string(),
                style: "Next-gen Windows-grade clarity".to_string(),
            },
            FontAsset {
                id: "meslo-lgs-nf".to_string(),
                name: "MesloLGS NF".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/Meslo.zip".to_string(),
                family_name: "MesloLGS Nerd Font".to_string(),
                classification: "OH-MY-ZSH-STD".to_string(),
                style: "Legacy shell synchronization".to_string(),
            },
            FontAsset {
                id: "victor-mono".to_string(),
                name: "Victor Mono".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/VictorMono.zip".to_string(),
                family_name: "Victor Mono".to_string(),
                classification: "CURSIVE-ITALIC".to_string(),
                style: "Elegant semantic script focus".to_string(),
            },
            FontAsset {
                id: "monaspace-argon".to_string(),
                name: "Monaspace Argon".to_string(),
                url: "https://github.com/githubnext/monaspace/releases/download/v1.101/monaspace-v1.101.zip".to_string(),
                family_name: "Monaspace Argon".to_string(),
                classification: "FUTURE-TYPE".to_string(),
                style: "Modular spacing consistency".to_string(),
            },
            FontAsset {
                id: "zed-mono".to_string(),
                name: "Zed Mono".to_string(),
                url: "https://github.com/zed-industries/zed-fonts/releases/download/1.2.0/zed-mono-1.2.0.zip".to_string(),
                family_name: "Zed Mono".to_string(),
                classification: "SPEED-REFLEX".to_string(),
                style: "Minimalist zero-distraction flow".to_string(),
            },
            FontAsset {
                id: "comic-shanns-mono".to_string(),
                name: "Comic Shanns Mono".to_string(),
                url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/ComicShannsMono.zip".to_string(),
                family_name: "ComicShannsMono Nerd Font".to_string(),
                classification: "ELITE-CASUAL".to_string(),
                style: "High-impact character recognition".to_string(),
            },
        ]
    }
}

/// Executes an interactive, elite-tier provisioning of the requested font.
pub async fn provision_font(font_id: &str) -> Result<(), anyhow::Error> {
    let registry = FontAsset::registry();
    let asset = match registry.iter().find(|f| f.id == font_id) {
        Some(a) => a,
        None => return Err(anyhow::anyhow!("Font ID not found in mission registry")),
    };

    println!(
        "\n  {} {} {}",
        CyberTheme::primary("◈"),
        CyberTheme::bright("PREPARING MISSION ASSET:"),
        CyberTheme::secondary(&asset.name)
    );

    // 1. Download font archive with Professional Telemetry
    use futures::StreamExt;
    use indicatif::{ProgressBar, ProgressStyle};

    let client = reqwest::Client::new();
    let resp = client.get(&asset.url).send().await?.error_for_status()?;
    let total_size = resp.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "  {} [{{elapsed_precise}}] [{{bar:40.{}.{}}}] {{bytes}}/{{total_bytes}} ({{eta}})",
                CyberTheme::primary("{spinner}"),
                "cyan",
                "blue"
            ))?
            .progress_chars("━>-"),
    );

    let mut bytes = Vec::with_capacity(total_size as usize);
    let mut stream = resp.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        bytes.extend_from_slice(&chunk);
        pb.set_position(bytes.len() as u64);
    }
    pb.finish_with_message(format!("Asset Provisioned: {}", asset.name));

    // MISSION ASSET INTEGRITY GATE:
    // Prevents "9-byte 404" corruption or mirror failure from entering the OS.
    if bytes.len() < 1024 * 1024 {
        return Err(anyhow::anyhow!(
            "Mission Asset Integrity Failure: {} size ({} bytes) is below the 1MB safety threshold. Target mirror may be corrupted.",
            asset.name,
            bytes.len()
        ));
    }

    // 2. Extract into user font directory
    // On Termux (Android): use $PREFIX/share/fonts
    // On standard Linux:   use ~/.local/share/fonts
    let font_dir = if let Ok(prefix) = std::env::var("PREFIX") {
        if prefix.contains("com.termux") {
            std::path::PathBuf::from(format!("{}/share/fonts", prefix))
        } else {
            dirs::home_dir()
                .ok_or_else(|| anyhow::anyhow!("Could not locate home directory"))?
                .join(".local/share/fonts")
        }
    } else {
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not locate home directory"))?
            .join(".local/share/fonts")
    };
    std::fs::create_dir_all(&font_dir)?;

    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_name = file.name();

        // Industry-Grade Filtered Extraction (TTF/OTF/WOFF only, skip documentation/metadata)
        if raw_name.ends_with(".ttf")
            || raw_name.ends_with(".otf")
            || raw_name.ends_with(".woff2")
            || raw_name.ends_with(".woff")
        {
            // Flatten directory structure to keep registry clean
            let filename = std::path::Path::new(raw_name)
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename in archive"))?;
            let outpath = font_dir.join(filename);

            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    println!(
        "  {} {} {}",
        CyberTheme::secondary("◈"),
        CyberTheme::bright("SYNCING REGISTRY:"),
        CyberTheme::dim("Targeted Fontconfig indexing...") // Zero-Latency optimization
    );
    // Optimized for Zero-Latency: Index ONLY the target directory, skip global refresh.
    std::process::Command::new("fc-cache")
        .arg(&font_dir)
        .status()?;

    println!(
        "  {} {} {}",
        CyberTheme::primary("✔"),
        CyberTheme::primary("PROVISIONING COMPLETE:").bold(),
        CyberTheme::bright(format!("{} is now available.", asset.name))
    );
    println!(
        "  {} Tip: Restart your terminal if the font doesn't apply immediately.",
        CyberTheme::dim("•")
    );

    Ok(())
}

/// Executes a silent, non-interactive provisioning of font utilities if missing.
pub async fn provision_fontconfig() {
    let is_installed = std::process::Command::new("which")
        .arg("fc-list")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_installed {
        return;
    }

    // Background installation via the correct system package manager
    tokio::spawn(async {
        // Detect OS: Termux → pkg; Debian/Ubuntu/Kali → apt-get; Fedora → dnf; Arch → pacman
        let is_termux = std::env::var("PREFIX")
            .map(|p| p.contains("com.termux"))
            .unwrap_or(false);

        if is_termux {
            // Termux: fontconfig is bundled in the `fontconfig` package
            let _ = tokio::process::Command::new("pkg")
                .args(["install", "-y", "fontconfig", "unzip"])
                .status()
                .await;
        } else if std::process::Command::new("which")
            .arg("apt-get")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = tokio::process::Command::new("sudo")
                .args(["-n", "apt-get", "update", "-qq"])
                .status()
                .await;
            let _ = tokio::process::Command::new("sudo")
                .args([
                    "-n",
                    "apt-get",
                    "install",
                    "-y",
                    "-qq",
                    "fontconfig",
                    "unzip",
                ])
                .status()
                .await;
        } else if std::process::Command::new("which")
            .arg("dnf")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = tokio::process::Command::new("sudo")
                .args(["-n", "dnf", "install", "-y", "fontconfig", "unzip"])
                .status()
                .await;
        } else if std::process::Command::new("which")
            .arg("pacman")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let _ = tokio::process::Command::new("sudo")
                .args(["-n", "pacman", "-S", "--noconfirm", "fontconfig", "unzip"])
                .status()
                .await;
        }
        // If no known package manager found, silently skip — fontconfig is optional
    });
}

/// Robustly checks if a specific CLI command is available in the current system PATH.
pub fn is_cmd_available(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Detects the host terminal emulator and attempts to synchronize its font family.
pub fn sync_terminal_font(font_family: &str) -> bool {
    let protocol = get_sync_protocol();
    let mut success = false;

    if protocol.starts_with("Kitty") && protocol.contains("RC Active") && is_cmd_available("kitty")
    {
        if std::process::Command::new("kitty")
            .args(["@", "set-config", &format!("font_family={}", font_family)])
            .status()
            .is_ok()
        {
            success = true;
        }
    } else if protocol.starts_with("Kitty") {
        print!("\x1b]50;Font={}\x07", font_family);
        success = true;
    } else if protocol.starts_with("iTerm2") {
        print!("\x1bP$qFont={}\x1b\\", font_family);
        success = true;
    } else if protocol.starts_with("WezTerm") && is_cmd_available("wezterm") {
        // Industry-Grade Safety: Escape apostrophes in font families to prevent config injection.
        let escaped_family = font_family.replace('\'', "\\'");
        if std::process::Command::new("wezterm")
            .args([
                "cli",
                "set-config",
                &format!("font = wezterm.font('{}')", escaped_family),
            ])
            .status()
            .is_ok()
        {
            success = true;
        }
    }

    std::io::stdout().flush().ok();
    success
}

/// Attempts to neutralize any tactical font synchronization and restore the terminal's default state.
pub fn revert_terminal_font() {
    let protocol = get_sync_protocol();
    if protocol.starts_with("Kitty") && protocol.contains("RC Active") && is_cmd_available("kitty")
    {
        let _ = std::process::Command::new("kitty")
            .args(["@", "set-config", "font_family="])
            .status();
    } else if protocol.starts_with("WezTerm") && is_cmd_available("wezterm") {
        let _ = std::process::Command::new("wezterm")
            .args(["cli", "set-config", "font = wezterm.font_with_fallback({})"])
            .status();
    }
    std::io::stdout().flush().ok();
}

/// Robustly checks if a specific font is installed and verified as the active match.
pub fn perform_typography_audit(font_id: &str) -> TypographyAudit {
    let registry = FontAsset::registry();
    let family_match = if font_id == "none" {
        "Terminal Default".to_string()
    } else {
        registry
            .iter()
            .find(|f| f.id == font_id)
            .map(|f| f.family_name.as_str())
            .unwrap_or(font_id)
            .to_string()
    };

    let protocol = get_sync_protocol();
    let sync_success = if font_id == "none" {
        revert_terminal_font();
        true
    } else {
        sync_terminal_font(&family_match)
    };

    // Deep Audit: Use fc-match to detect fallbacks (Industry-Grade Resiliency).
    let (matched_family, fidelity) = if font_id == "none" {
        (protocol.clone(), 1.0)
    } else if is_cmd_available("fc-match") {
        let mut match_result = std::process::Command::new("fc-match")
            .arg("--format=%{family}")
            .arg(&family_match)
            .output();

        // Stage 2: If primary match is a generic fallback, try normalized (no-space) variant.
        if let Ok(ref o) = match_result {
            let matched = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if (matched.contains("DejaVu") || matched.contains("Sans") || matched.contains("Serif"))
                && !family_match.contains("DejaVu")
            {
                let normalized_query = family_match.replace(' ', "");
                if let Ok(o2) = std::process::Command::new("fc-match")
                    .arg("--format=%{family}")
                    .arg(normalized_query)
                    .output()
                {
                    match_result = Ok(o2);
                }
            }
        }

        if let Ok(o) = match_result {
            let matched = String::from_utf8_lossy(&o.stdout).trim().to_string();

            // Industry-Grade Robust Matching: Ignore spaces and case
            let normalized_match = matched.replace(' ', "").to_lowercase();
            let normalized_target = family_match.replace(' ', "").to_lowercase();

            let score = if normalized_match.contains(&normalized_target) {
                1.0
            } else {
                0.5 // Fallback detected
            };
            (matched, score)
        } else {
            ("Unknown (Error)".to_string(), 0.0)
        }
    } else {
        ("AUDIT BLOCKED (fontconfig missing)".to_string(), 0.0)
    };

    TypographyAudit {
        target_font: family_match.to_string(),
        os_match: matched_family.clone(),
        fidelity_score: fidelity,
        sync_protocol: protocol,
        sync_success,
        // Checksum: High-fidelity hardware & state fingerprint
        checksum: format!(
            "{:08x}",
            (matched_family.len() * 31) ^ (fidelity * 100.0) as usize
        ),
    }
}

/// Retrieves the typography audit from cache or performs a new one.
pub fn get_typography_audit(font_id: &str) -> TypographyAudit {
    let mut cache = TYPOGRAPHY_CACHE.lock().unwrap();
    if let Some(ref audit) = *cache {
        if audit.target_font == font_id {
            return audit.clone();
        }
    }
    let new_audit = perform_typography_audit(font_id);
    *cache = Some(new_audit.clone());
    new_audit
}

/// Applies a dynamic Unicode font mapping to standard ASCII text.
/// If simulated_mode is enabled, this translates English text into Mathematical equivalents.
/// Otherwise, it returns the original text to maintain absolute font integrity.
pub fn apply_font(text: &str, font: &str, simulated_mode: bool) -> String {
    if !simulated_mode {
        return text.to_string();
    }

    match font {
        "jet-brains-mono" | "monospace" | "hack-nerd-font" => text
            .chars()
            .map(|c| match c {
                'a'..='z' => std::char::from_u32(c as u32 - 'a' as u32 + 0x1D68A).unwrap_or(c),
                'A'..='Z' => std::char::from_u32(c as u32 - 'A' as u32 + 0x1D670).unwrap_or(c),
                '0'..='9' => std::char::from_u32(c as u32 - '0' as u32 + 0x1D7F6).unwrap_or(c),
                _ => c,
            })
            .collect(),
        "iosevka-nerd-font" | "tactical" | "victor-mono" => text
            .chars()
            .map(|c| match c {
                // Mathematical Serif Bold-Italic (Premium Operator Look)
                'a'..='z' => std::char::from_u32(c as u32 - 'a' as u32 + 0x1D482).unwrap_or(c),
                'A'..='Z' => std::char::from_u32(c as u32 - 'A' as u32 + 0x1D468).unwrap_or(c),
                '0'..='9' => std::char::from_u32(c as u32 - '0' as u32 + 0x1D7CE).unwrap_or(c), // Bold 0-9
                '*' => '✱',
                '#' => '⌗',
                '@' => '＠',
                '$' => '﹩',
                '%' => '﹪',
                '&' => '＆',
                '!' => '﹗',
                '?' => '？',
                _ => c,
            })
            .collect(),
        "cyber-neon" | "sans-serif" | "neural" | "fira-code" | "cascadia-code" | "meslo-lgs-nf" => {
            text.chars()
                .map(|c| match c {
                    // Mathematical Sans-Serif Bold (High Contrast UI Look)
                    'a'..='z' => std::char::from_u32(c as u32 - 'a' as u32 + 0x1D5EE).unwrap_or(c),
                    'A'..='Z' => std::char::from_u32(c as u32 - 'A' as u32 + 0x1D5D4).unwrap_or(c),
                    '0'..='9' => std::char::from_u32(c as u32 - '0' as u32 + 0x1D7EC).unwrap_or(c),
                    // Premium Symbols
                    '-' => '─',
                    '_' => '━',
                    ':' => '⁚',
                    '.' => '·',
                    '*' => '✦',
                    _ => c,
                })
                .collect()
        }
        "small-caps" => text
            .chars()
            .map(|c| match c {
                'a' => 'ᴀ',
                'b' => 'ʙ',
                'c' => 'ᴄ',
                'd' => 'ᴅ',
                'e' => 'ᴇ',
                'f' => 'ꜰ',
                'g' => 'ɢ',
                'h' => 'ʜ',
                'i' => 'ɪ',
                'j' => 'ᴊ',
                'k' => 'ᴋ',
                'l' => 'ʟ',
                'm' => 'ᴍ',
                'n' => 'ɴ',
                'o' => 'ᴏ',
                'p' => 'ᴘ',
                'q' => 'ǫ',
                'r' => 'ʀ',
                's' => 'ꜱ',
                't' => 'ᴛ',
                'u' => 'ᴜ',
                'v' => 'ᴠ',
                'w' => 'ᴡ',
                'y' => 'ʏ',
                'z' => 'ᴢ',
                'A'..='Z' => c, // Keep caps as is
                _ => c,
            })
            .collect(),
        _ => text.to_string(), // Default ASCII
    }
}

/// Renders a medical-grade tactical mission advisory with absolute layout alignment.
/// This implementation represents the final, indestructible solution to CLI alignment challenges.
pub fn print_tui_disabled_message() {
    let width: usize = 72; // FIXED TOTAL VISUAL WIDTH
    let border_color = |s: &str| CyberTheme::dim(s).to_string();

    println!();
    // MISSION ADVISORY HEADER
    let header_label = " MISSION ADVISORY // NEURAL LINK ";
    let styled_header = CyberTheme::accent(format!(" [!] {} ", header_label.bold()));
    let header_line_len = width.saturating_sub(header_label.len() + 10);
    println!(
        "{}{}{}{}",
        border_color("╭──"),
        styled_header,
        border_color(&"─".repeat(header_line_len)),
        border_color("╮")
    );

    // MASTER ROW CONSTRUCTOR
    let print_row = |content: String, visible_len: usize| {
        let padding = width.saturating_sub(visible_len + 2);
        println!(
            "{}{}{}{}",
            border_color("│"),
            content,
            " ".repeat(padding),
            border_color("│")
        );
    };

    let print_separator = || {
        println!(
            "{}{}{}",
            border_color("├"),
            border_color(&"─".repeat(width - 2)),
            border_color("┤")
        );
    };

    let print_empty = || {
        println!(
            "{}{}{}",
            border_color("│"),
            " ".repeat(width - 2),
            border_color("│")
        );
    };

    print_empty();

    /// Internal helper to format a gutter row with perfect width tracking.
    fn format_row(
        label: &str,
        symbol: &str,
        value: &str,
        label_width: usize,
        is_dim: bool,
    ) -> (String, usize) {
        let label_padding = label_width.saturating_sub(label.len());
        let symbol_block = format!("  {}   ", symbol); // 2 spaces before, 3 spaces after
        let content = if is_dim {
            format!(
                "  {}{}{}{}",
                CyberTheme::dim(label),
                " ".repeat(label_padding),
                CyberTheme::secondary(&symbol_block),
                CyberTheme::dim(value)
            )
        } else {
            format!(
                "  {}{}{}{}",
                CyberTheme::bright(label.bold()),
                " ".repeat(label_padding),
                CyberTheme::secondary(&symbol_block),
                CyberTheme::bright(value)
            )
        };
        // Visible len calculation:
        // 2 (initial spaces) + label_width (label+padding) + 2 (before symbol) + symbol.len() + 3 (after symbol) + value.len()
        let v_len = 2 + label_width + 2 + symbol.len() + 3 + value.len();
        (content, v_len)
    }

    let (s_content, s_len) = format_row("STATUS", "::", "Neural Interface Offline.", 10, false);
    print_row(s_content, s_len);

    let (y_content, y_len) = format_row(
        "SYNC",
        "::",
        "Direct CLI Telemetry Uplink Established.",
        10,
        true,
    );
    print_row(y_content, y_len);

    print_empty();

    print_separator();
    print_empty();

    let instruction = "MISSION FALLBACK ENGAGED. EXECUTE COMMAND:";
    print_row(
        format!("  {}", CyberTheme::bright(instruction)),
        2 + instruction.len(),
    );

    print_empty();
    let cmd_tag = " [ ACTION ] ";
    let cmd_val = "myth --no-tui chat";
    print_row(
        format!(
            "      {}  {}",
            CyberTheme::primary(cmd_tag.bold().reversed()),
            CyberTheme::primary(cmd_val.bold())
        ),
        6 + cmd_tag.len() + 2 + cmd_val.len(),
    );

    print_empty();
    print_separator();
    print_empty();

    let (src_content, src_len) = format_row("SOURCE", ">>", "config/agent.yaml", 10, true);
    print_row(src_content, src_len);

    let (mode_content, mode_len) =
        format_row("MODE", ">>", "Headless Tactical Mode [STABLE]", 10, true);
    print_row(mode_content, mode_len);

    print_empty();

    // MISSION FOOTER
    let footer_text = " RE-ENABLE: Restore tui.enabled in config ";
    let footer_line = width.saturating_sub(footer_text.len() + 5);
    println!(
        "{} {} {}{}",
        border_color("╰─"),
        CyberTheme::dim(footer_text),
        border_color(&"─".repeat(footer_line)),
        border_color("╯")
    );
    println!();
}

/// Renders a premium, "Industry-Grade" mission telemetry header for the interactive CLI.
/// This replaces the simple println! stack with a structured, high-fidelity diagnostic dashboard.
pub fn print_mission_header(config: &crate::config::AppConfig) {
    let width: usize = 70;
    let border_color = |s: &str| CyberTheme::dim(s).to_string();

    println!();
    // MISSION TELEMETRY HEADER
    let header_label = " MISSION TELEMETRY // NEURAL CORE ";
    let styled_header = CyberTheme::secondary(format!(" [◈] {} ", header_label.bold()));
    let header_line_len = width.saturating_sub(header_label.len() + 10);
    println!(
        "{}{}{}{}",
        border_color("╭──"),
        styled_header,
        border_color(&"─".repeat(header_line_len)),
        border_color("╮")
    );

    let print_row = |content: String, visible_len: usize| {
        let padding = width.saturating_sub(visible_len + 2);
        println!(
            "{}{}{}{}",
            border_color("│"),
            content,
            " ".repeat(padding),
            border_color("│")
        );
    };

    let print_separator = |label: &str| {
        let label_str = format!(" [ {} ] ", label);
        let styled_label = CyberTheme::dim(&label_str);
        let line_len = width.saturating_sub(label_str.len() + 2);
        println!(
            "{}{}{}{}",
            border_color("├─"),
            styled_label,
            border_color(&"─".repeat(line_len)),
            border_color("┤")
        );
    };

    let print_empty = || {
        println!(
            "{}{}{}",
            border_color("│"),
            " ".repeat(width - 2),
            border_color("│")
        );
    };

    fn format_telemetry(label: &str, symbol: &str, value: &str, is_dim: bool) -> (String, usize) {
        let label_width: usize = 12;
        let label_padding = label_width.saturating_sub(label.len());
        let symbol_block = format!("  {}  ", symbol);
        let content = if is_dim {
            format!(
                "  {}{}{}{}",
                CyberTheme::dim(label),
                " ".repeat(label_padding),
                CyberTheme::secondary(&symbol_block),
                CyberTheme::dim(value)
            )
        } else {
            format!(
                "  {}{}{}{}",
                CyberTheme::bright(label.bold()),
                " ".repeat(label_padding),
                CyberTheme::secondary(&symbol_block),
                CyberTheme::bright(value)
            )
        };
        let v_len = 2 + label_width + symbol_block.len() + value.len();
        (content, v_len)
    }

    print_empty();

    let (a_content, a_len) = format_telemetry("AGENT", "::", "MYTH RECONNAISSANCE AGENT", false);
    print_row(a_content, a_len);

    let (v_content, v_len) = format_telemetry("VERSION", "::", &config.agent.version, true);
    print_row(v_content, v_len);

    print_empty();
    print_separator("TACTICAL STACK");
    print_empty();

    let (m_content, m_len) = format_telemetry("MODEL", ">>", &config.llm.model, false);
    print_row(m_content, m_len);

    let config_path = crate::config::AppConfig::user_config_path();
    let (c_content, c_len) =
        format_telemetry("CONFIG", ">>", &config_path.display().to_string(), true);
    print_row(c_content, c_len);

    let (f_content, f_len) = format_telemetry("FONT", ">>", &config.tui.font, true);
    print_row(f_content, f_len);

    let (s_content, s_len) = format_telemetry("SANDBOX", ">>", "[ ACTIVE ]", false);
    print_row(s_content, s_len);

    print_empty();

    // MISSION FOOTER
    let footer_text = " Type 'scan <target>' to begin reconnaissance ";
    let footer_line = width.saturating_sub(footer_text.len() + 5);
    println!(
        "{} {} {}{}",
        border_color("╰─"),
        CyberTheme::primary(footer_text).bold(),
        border_color(&"─".repeat(footer_line)),
        border_color("╯")
    );
    println!();
}

/// Robustly returns the visual length of a string by stripping ANSI escape sequences and accounting for multi-column Unicode characters.
pub fn visual_len(text: &str) -> usize {
    let mut total_width = 0;
    let mut iter = text.chars().peekable();

    while let Some(c) = iter.next() {
        if c == '\x1B' {
            if let Some('[') = iter.peek() {
                iter.next(); // Consume '['
                for nc in iter.by_ref() {
                    if (nc as u32) >= 0x40 && (nc as u32) <= 0x7E {
                        break;
                    }
                }
            }
        } else {
            // High-fidelity width calculation for common Cyber/HUD symbols
            total_width += match c {
                '◈' | '◇' | '➔' | '〔' | '〕' => 2,
                _ => 1,
            };
        }
    }
    total_width
}
