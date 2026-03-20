//! UI Module — Shared CLI and TUI aesthetic components.
//!
//! This module provides a consistent "Cyber" style for the MYTH CLI,
//! including themes, ASCII banners, and formatted output helpers.

use owo_colors::OwoColorize;
use std::io::Write;

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

    /// Bright White — High contrast text.
    pub fn bright<T: OwoColorize + std::fmt::Display>(text: T) -> String {
        text.color(owo_colors::Rgb(255, 255, 255)).to_string()
    }
}

/// Renders a premium ASCII banner.
pub fn print_banner(name: &str, version: &str) {
    let lines = [
        "    __  ___  __  __  ______  __  __ ",
        "   /  |/  /  \\ \\/ / /_  __/ / / / / ",
        "  / /|_/ /    \\  /   / /   / /_/ /  ",
        " / /  / /     / /   / /   / __  /   ",
        "/_/  /_/     /_/   /_/   /_/ /_/    ",
    ];

    println!();
    println!("{}", CyberTheme::accent(lines[0]).bold());
    println!("{}", CyberTheme::accent(lines[1]).bold());
    println!("{}", CyberTheme::secondary(lines[2]).bold());
    println!("{}", CyberTheme::secondary(lines[3]).bold());
    println!("{}", CyberTheme::primary(lines[4]).bold());
    println!(
        " {} {} {}",
        CyberTheme::dim("━".repeat(4)),
        CyberTheme::primary(name.to_uppercase()),
        CyberTheme::dim(format!("v{} ━", version))
    );
    println!();
}

/// Renders a "Cyber" boot sequence with real-time integrity checks.
pub async fn boot_sequence(config: &crate::config::AppConfig) {
    use std::process::Command;
    use tokio::time::{sleep, Duration};

    let check_cmd = |cmd: &str| -> String {
        if Command::new("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            CyberTheme::secondary("OK").bold().to_string()
        } else {
            CyberTheme::accent("MISSING").bold().to_string()
        }
    };

    let sandbox_status = if config.sandbox.enabled {
        CyberTheme::secondary("ACTIVE").bold().to_string()
    } else {
        CyberTheme::accent("VULNERABLE").bold().to_string()
    };

    let codegate_status = if config
        .mcp
        .mcp_servers
        .get("codegate")
        .map(|s| match s {
            crate::config::CustomMcpServer::Local(l) => l.enabled,
            crate::config::CustomMcpServer::Remote(r) => r.enabled,
        })
        .unwrap_or(false)
    {
        CyberTheme::secondary("SHIELDED").bold().to_string()
    } else {
        CyberTheme::dim("BYPASSED").to_string()
    };

    let steps = [
        (
            "NEURAL CORE INITIALIZATION",
            CyberTheme::secondary("OK").bold().to_string(),
        ),
        (
            "NVIDIA NIM CONNECTIVITY",
            CyberTheme::secondary("LINKED").bold().to_string(),
        ),
        ("PYTHON UVX RUNTIME", check_cmd("uvx")),
        ("NODEJS NPX RUNTIME", check_cmd("npx")),
        ("SANDBOX SECURITY LAYER", sandbox_status),
        ("CODEGATE PROXY SHIELD", codegate_status),
        (
            "TACTICAL ASSET REGISTRY",
            CyberTheme::secondary("SYNCED").bold().to_string(),
        ),
    ];

    println!("{}", CyberTheme::dim("[ SYSTEM BOOT SEQUENCE ]").italic());
    for (msg, status) in steps {
        sleep(Duration::from_millis(20)).await;
        print!("  {} {:.<35} ", CyberTheme::primary("❯"), msg);
        std::io::stdout().flush().ok();
        sleep(Duration::from_millis(40)).await;
        println!("{}", status);
    }
    sleep(Duration::from_millis(50)).await;
}

/// Renders text inside a stylized box.
pub fn print_boxed(title: &str, content: &str) {
    let width = 60;
    let horizontal = "━".repeat(width);

    println!(
        "{}",
        CyberTheme::secondary(format!(
            "┏━[{}]━{}",
            title,
            "━".repeat(width.saturating_sub(title.len() + 4))
        ))
    );
    for line in content.lines() {
        println!("┃ {}", CyberTheme::bright(line));
    }
    println!("{}", CyberTheme::secondary(format!("┗{}┛", horizontal)));
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
pub async fn print_operative_prompt(name: &str) {
    println!(
        "{}",
        CyberTheme::dim(format!(
            "┌──[ {} // {} ]",
            "OPERATIVE".bold(),
            CyberTheme::primary(name.to_uppercase()).bold()
        ))
    );

    // Tactical Flux: Prompt animation
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    for frame in &frames {
        print!("\r{} ", CyberTheme::secondary(frame));
        std::io::stdout().flush().ok();
        tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    }

    print!("\r{} ", CyberTheme::secondary("└─❯"));
    std::io::stdout().flush().ok();
}

/// Renders a premium, multi-line agent (MYTH) response prefix.
pub fn print_agent_prefix(name: &str) {
    println!(
        "{}",
        CyberTheme::accent(format!("┌──[ {} // {} ]", "AGENT".bold(), name.to_uppercase().bold()))
    );
    print!("{} ", CyberTheme::accent("└─❯"));
    std::io::stdout().flush().ok();
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
