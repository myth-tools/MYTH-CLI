//! Documentation system for the MYTH CLI.
//! Provides "man" pages and advanced usage info.

use owo_colors::OwoColorize;

pub fn get_man_page(topic: &str) -> Option<String> {
    match topic.to_lowercase().as_str() {
        "scan" | "/recon" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  scan <target>\n  /recon <target>\n\n\
             {details}\n\
             Initiates a full-spectrum reconnaissance cycle. The agent will traverse the 12-phase tactical roadmap (89 steps):\n\
             1. Advanced Asset & Identity Intelligence.\n\
             2. Deep Vulnerability & Secrets Exposure Analysis.\n\
             3. Continuous Monitoring & Synthesis.\n\
             4. Complete Professional reporting.\n",
            title = "MAN PAGE: SCAN / RECON".bold().reversed().magenta(),
            desc = "Strategic asset acquisition and deep intelligence gathering.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
        )),
        "stealth" | "/stealth" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  /stealth <target>\n\n\
             {details}\n\
             Low-signature reconnaissance optimized for evasion. \n\
             Prioritizes passive indexing and low-noise active probes. \n\
             Reduces the probability of detection by IDS/IPS systems.\n",
            title = "MAN PAGE: STEALTH MODE".bold().reversed().cyan(),
            desc = "Shadow-based intelligence ops with minimal footprint.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
        )),
        "osint" | "/osint" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  /osint <target>\n\n\
             {details}\n\
             Open Source Intelligence focus. Aggregates data from public records, \n\
             domain registers, and cloud-leak databases. No active touching of \n\
             the target infrastructure occurs in this mode.\n",
            title = "MAN PAGE: OSINT OPS".bold().reversed().green(),
            desc = "External intelligence mapping via public data streams.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
        )),
        "vuln" | "/vuln" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  /vuln <target>\n\n\
             {details}\n\
             Aggressive vulnerability assessment. Leverages the full power of \n\
             the MCP toolset to identify exploitable misconfigurations, \n\
             known CVEs, and logic flaws in target services.\n",
            title = "MAN PAGE: VULNERABILITY ASSESSMENT".bold().reversed().red(),
            desc = "Offensive-oriented surface analysis and flaw identification.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
        )),
        "graph" | "/graph" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  /graph\n\n\
             {details}\n\
             Visualizes the current understanding of the target infrastructure. \n\
             Links identified nodes (IPs, domains, services) with findings. \n\
             Essential for mapping lateral movement paths.\n",
            title = "MAN PAGE: RECON GRAPH".bold().reversed().blue(),
            desc = "Relational mapping of identified intelligence nodes.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
        )),
        "subdomains" | "/subdomains" => Some(format!(
            "\n{title}\n\n\
             {desc}\n\n\
             {usage_label}\n  /subdomains <domain> [FLAGS]\n\n\
             {details}\n\
             Advanced multi-vector subdomain discovery engine. \n\
             Aggregates data from 55+ passive sources (CT logs, DNS aggregators, \n\
             search engines, blockchain) and optional active enumeration.\n\
             This tool utilizes an elite 18-phase discovery pipeline for ~100% coverage.\n\n\
             {flags_label}\n\
             --active         Enable active brute-force and permutation scanning.\n\
             --recursive      Trigger recursive discovery on identified assets.\n\
             --stealth        Low concurrency, randomized delays, evasive signature.\n\
             --proxies        Route all discovery traffic through rotating public proxies.\n\
             --tor          Route all discovery traffic through the Tor network.\n\
             --wordlist <sz>  Size for brute-forcing: none, small, medium, large, quick, deep, mega (Default: medium)\n\
             --concurrency <n> Maximum concurrent tasks (Default: 50).\n\
             --timeout <s>    Global timeout in seconds (Default: 3600).\n\
             --depth <n>      Initial depth of subdomain gathering (Default: 1).\n\
             --recursive-depth <n> Maximum depth for recursive discovery (Default: 3).\n\n\
             {examples_label}\n\
             /subdomains target.com --active --wordlist mega\n\
             /subdomains target.com --stealth --proxies\n\
              /subdomains target.com --recursive --recursive-depth 3\n",
            title = "MAN PAGE: SUBDOMAINS DISCOVERY".bold().reversed().yellow(),
            desc = "Recursive multi-source asset indexing and surface mapping.".italic(),
            usage_label = "USAGE:".dimmed(),
            details = "DETAILS:".dimmed(),
            flags_label = "FLAGS:".dimmed(),
            examples_label = "EXAMPLES:".dimmed(),
        )),
        "subdomains_full" => Some(crate::core::commands::get_subdomains_help()),
        _ => None,
    }
}

pub fn format_tool_inspection(tool_name: &str, description: &str, category: &str) -> String {
    let header = format!(" NEURAL ASSET INSPECTION // {} ", tool_name.to_uppercase());
    format!(
        "\n{header}\n\n\
         {name_label}     {name_val}\n\
         {cat_label}  {cat_val}\n\n\
         {desc_label}\n{desc_val}\n\n\
         {status}\n",
        header = header.bold().reversed().yellow(),
        name_label = "IDENTIFIER:".dimmed(),
        name_val = tool_name.bright_white().bold(),
        cat_label = "CLASSIFICATION:".dimmed(),
        cat_val = category.bright_cyan(),
        desc_label = "MISSION PARAMETERS & INTEL:".dimmed(),
        desc_val = description,
        status = "STATUS: Asset verified and synchronized for immediate execution."
            .italic()
            .dimmed(),
    )
}

pub fn get_usage() -> String {
    format!(
        "\n{title}\n\n\
         {cmd_label}\n\
         {recon_group}\n\
         - scan <target>   Full-spectrum mission\n\
         - /stealth <t>    Low-noise passive ops\n\
         - /osint <t>      Public intel mapping\n\
         - /vuln <t>       Aggressive flaw assessment\n\
         - /subdomains <t> Multi-source discovery\n\n\
         {nav_group}\n\
         - /findings       List intel recap\n\
         - /graph          View asset layout\n\
         - /report         Generate executive summary\n\
         - /inspect <tool> Deep asset documentation\n\
         - /vitals         Check core telemetry\n\n\
         {sys_group}\n\
         - /man <topic>    Deep-dive documentation\n\
         - /logs           View tactical event log\n\
         - /burn           Emergency data shred\n\
         - /quit           Exit mission protocol\n",
        title = "MYTH COMMAND USAGE ARCHIVE".bold().magenta(),
        cmd_label = "CORE PROTOCOLS:".dimmed(),
        recon_group = "RECONNAISSANCE".cyan().bold(),
        nav_group = "INTELLIGENCE".green().bold(),
        sys_group = "SYSTEM".red().bold(),
    )
}

pub fn get_version_long(name: &str, version: &str) -> String {
    let arch_val = format!("{}-linux-x86_64", std::env::consts::OS);
    format!(
        "\n{header}\n\n\
         {name_label}    {name_val}\n\
         {ver_label}    {ver_val}\n\
         {build_label}  {build_val}\n\
         {arch_label}   {arch_val}\n\n\
         {desc}\n",
        header = "MYTH NEURAL ENGINE TELEMETRY".bold().reversed().cyan(),
        name_label = "IDENTIFIER:".dimmed(),
        name_val = name.bright_white().bold(),
        ver_label = "ITERATION: ".dimmed(),
        ver_val = version.bright_yellow(),
        build_label = "BUILD DATE:".dimmed(),
        build_val = concat!(env!("CARGO_PKG_VERSION"), "-STABLE"),
        arch_label = "ENVIRONMENT:".dimmed(),
        arch_val = arch_val,
        desc = "Built for high-speed, volatile cybersecurity operations in Kali Linux."
            .italic()
            .dimmed(),
    )
}
