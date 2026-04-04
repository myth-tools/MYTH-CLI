export const sidebarNav = [
	{
		title: "Mission Directives",
		items: [
			{ title: "Operational Briefing", path: "/" },
			{ title: "Asset Deployment", path: "/installation" },
			{ title: "Neural Telemetry", path: "/versions" },
			{ title: "Rapid Start", path: "/quickstart" },
			{ title: "Command Runners", path: "/command-runners" },
			{ title: "Core Configuration", path: "/configuration" },
		],
	},
	{
		title: "Architecture",
		items: [
			{ title: "Strategic Overview", path: "/architecture" },
			{ title: "Neural Vitals", path: "/vitals" },
			{ title: "Security Matrix", path: "/security" },
			{ title: "Volatility Memory", path: "/memory" },
		],
	},
	{
		title: "Command Registry",
		items: [
			{ title: "Core CLI Commands", path: "/cli-commands" },
			{ title: "Interactive Hub", path: "/interactive-commands" },
			{ title: "Tactical Profiles", path: "/profiles" },
			{ title: "Reconnaissance Subdomains", path: "/subdomain-fetch" },
		],
	},
	{
		title: "MCP Ecosystem",
		items: [
			{ title: "Sovereign MCP Servers", path: "/mcp-servers" },
			{ title: "Orchestrated Tools", path: "/builtin-tools" },
			{ title: "Neural Tool Bridges", path: "/tool-bridges" },
			{ title: "External MCP Bridges", path: "/custom-mcp" },
		],
	},
	{
		title: "Reference Data",
		items: [
			{ title: "Tactical Scripts", path: "/scripts" },
			{ title: "Industrial Stack", path: "/tech-stack" },
			{ title: "Mission Typography", path: "/typography" },
			{ title: "The Architect", path: "/creator" },
		],
	},
];

export interface CliCommand {
	name: string;
	aliases?: string[];
	category:
		| "Mission Core"
		| "Intelligence & Reporting"
		| "Asset Management"
		| "System & Maintenance";
	description: string;
	usage: string;
	args?: {
		name: string;
		type: string;
		required: boolean;
		description: string;
		default?: string;
	}[];
	examples: { command: string; description: string }[];
}

export const cliCommands: CliCommand[] = [
	{
		name: "scan",
		aliases: ["recon"],
		category: "Mission Core",
		description:
			"Initialize full-spectrum target acquisition and reconnaissance. Launches the agent with a 13-phase, 89-step methodology for comprehensive intelligence gathering and autonomous analysis.",
		usage: "myth scan <target> [--profile <name>]",
		args: [
			{
				name: "target",
				type: "string",
				required: true,
				description: "Target domain, IP, or URL for mission focus",
			},
			{
				name: "--profile",
				type: "string",
				required: false,
				description: "Reconnaissance methodology profile (stealth, quick, full, custom)",
				default: "full",
			},
		],
		examples: [
			{
				command: "myth scan example.com",
				description: "Standard reconnaissance on example.com",
			},
			{
				command: "myth scan 192.168.1.0/24 --profile stealth",
				description: "Stealth reconnaissance on a subnet",
			},
			{
				command: "myth recon target.org --profile quick",
				description: "Quick surface scan focused on rapid acquisition",
			},
		],
	},
	{
		name: "stealth",
		category: "Mission Core",
		description:
			"Launch immediate low-signature, passive-only reconnaissance. Uses only OSINT techniques, DNS analysis, and publicly available registries to minimize detection risk.",
		usage: "myth stealth <target>",
		args: [
			{
				name: "target",
				type: "string",
				required: true,
				description: "Target domain, IP, or URL",
			},
		],
		examples: [
			{
				command: "myth stealth target.com",
				description: "Passive-only recon with zero active probing or traffic",
			},
		],
	},
	{
		name: "osint",
		category: "Mission Core",
		description:
			"Launch specialized Open Source Intelligence operations. Aggregates data from 50+ public sources, domain registries, and leaked data archives without direct target interaction.",
		usage: "myth osint <target>",
		args: [
			{
				name: "target",
				type: "string",
				required: true,
				description: "Target domain, IP, or URL",
			},
		],
		examples: [
			{
				command: "myth osint company.com",
				description: "Deep OSINT mapping and intelligence synthesis",
			},
		],
	},
	{
		name: "vuln",
		category: "Mission Core",
		description:
			"Perform a deep, multi-vector vulnerability assessment. Leverages the absolute full MCP toolset to identify misconfigurations and known CVEs via the neural reasoning engine.",
		usage: "myth vuln <target>",
		args: [
			{
				name: "target",
				type: "string",
				required: true,
				description: "Target domain, IP, or URL",
			},
		],
		examples: [
			{
				command: "myth vuln 10.10.10.1",
				description: "Deep vulnerability assessment of a specific tactical node",
			},
		],
	},
	{
		name: "chat",
		category: "Mission Core",
		description:
			"Launch an interactive tactical chat session with the AI agent. This is the primary interface for real-time mission modulation and tool-calling interaction.",
		usage: "myth chat [--no-tui]",
		args: [
			{
				name: "--no-tui",
				type: "boolean",
				required: false,
				description: "Disable the Terminal User Interface in favor of a simple CLI",
			},
		],
		examples: [
			{ command: "myth chat", description: "Launch interactive neural TUI" },
			{
				command: "myth chat --no-tui",
				description: "Launch simple interactive talk-session",
			},
		],
	},
	{
		name: "tools",
		category: "Asset Management",
		description:
			"Catalog and display all synchronized mission assets and tools. Lists local binaries, external MCP tools, and native specialized utilities with category filtering.",
		usage: "myth tools [--category <cat>] [--search <query>]",
		args: [
			{
				name: "--category",
				type: "string",
				required: false,
				description: "Filter assets by military category (e.g., scanner, web)",
			},
			{
				name: "--search",
				type: "string",
				required: false,
				description: "Search assets by name or operational keyword",
			},
		],
		examples: [
			{
				command: "myth tools",
				description: "List all available tactical assets",
			},
			{
				command: "myth tools --search shodan",
				description: "Search for Shodan-integrated toolsets",
			},
			{
				command: "myth tools --category reconnaissance",
				description: "Filter by reconnaissance category",
			},
		],
	},
	{
		name: "target",
		category: "Mission Core",
		description:
			"Force rotate the mission focus to a new target or CIDR range. Maintains session history but resets the neural objective focus.",
		usage: "myth target <target> [--profile <name>]",
		args: [
			{
				name: "target",
				type: "string",
				required: true,
				description: "Target domain, IP, or URL",
			},
			{
				name: "--profile",
				type: "string",
				required: false,
				description: "Reconnection profile for the new objective",
				default: "quick",
			},
		],
		examples: [
			{
				command: "myth target node-03.local",
				description: "Rotate mission focus to a new internal target",
			},
		],
	},
	{
		name: "config",
		category: "System & Maintenance",
		description:
			"Retrieve and display the current mission configuration. Sensitive API credentials and keys are autonomously masked for security during display.",
		usage: "myth config",
		args: [],
		examples: [
			{
				command: "myth config",
				description: "Display current neural core configuration state",
			},
		],
	},
	{
		name: "profile",
		category: "System & Maintenance",
		description:
			"View or modulate tactical reconnaissance profiles. Allows for enabling/disabling specific methodology phases for precision operational control.",
		usage: "myth profile [<name>] [<action>] [<index>]",
		args: [
			{
				name: "name",
				type: "string",
				required: false,
				description: "Profile identifier (e.g., full, stealth, osint)",
			},
			{
				name: "action",
				type: "string",
				required: false,
				description: "Action to perform: enable or disable",
			},
			{
				name: "index",
				type: "string",
				required: false,
				description: "Phase indices targeting specific roadmap steps",
			},
		],
		examples: [
			{ command: "myth profile", description: "List all operational profiles" },
			{
				command: "myth profile full",
				description: "View full detail of the 'full' methodology",
			},
			{
				command: "myth profile elite disable 1,4",
				description: "Disable phases 1 and 4 in the 'elite' profile",
			},
		],
	},
	{
		name: "check",
		aliases: ["health"],
		category: "System & Maintenance",
		description:
			"Verify system health, sandbox status, and tool availability with an advanced diagnostic engine that checks for process isolation and neural link integrity.",
		usage: "myth check",
		args: [],
		examples: [
			{
				command: "myth check",
				description: "Run full mission-critical health diagnostics",
			},
			{ command: "myth health", description: "Same command using alias" },
		],
	},
	{
		name: "mcp",
		category: "Asset Management",
		description:
			"Manage Custom User MCP Servers. Supports listing, toggling, adding local/remote servers, and granular tool management for tactical expansion.",
		usage: "myth mcp <subcommand>",
		args: [],
		examples: [
			{
				command: "myth mcp list",
				description: "List all configured MCP tactical servers",
			},
			{
				command: "myth mcp toggle github on",
				description: "Enable the GitHub reconnaissance MCP server",
			},
			{
				command: "myth mcp add-local scan-engine npx -a arg1 env:API_KEY=xxx",
				description: "Add a local specialized MCP server",
			},
			{
				command: "myth mcp allow-tool github create_issue",
				description: "Allow a specific tool from a server registry",
			},
		],
	},
	{
		name: "vitals",
		aliases: ["status"],
		category: "Intelligence & Reporting",
		description:
			"Analyze neural pulses and session lifecycle telemetry. Shows core integrity, session tracking, target vector, and operational uptime.",
		usage: "myth vitals",
		args: [],
		examples: [
			{
				command: "myth vitals",
				description: "View real-time neural session vitals",
			},
		],
	},
	{
		name: "findings",
		category: "Intelligence & Reporting",
		description:
			"Aggregate and display all discovered tactical intelligence and vulnerabilities from the current mission session.",
		usage: "myth findings",
		args: [],
		examples: [
			{
				command: "myth findings",
				description: "Generate a list of all current mission findings",
			},
		],
	},
	{
		name: "graph",
		category: "Intelligence & Reporting",
		description:
			"Render the infrastructure relationship graph of target assets, showing relationships between targets, subdomains, and discoveries.",
		usage: "myth graph",
		args: [],
		examples: [
			{
				command: "myth graph",
				description: "Display an interactive infrastructure graph",
			},
		],
	},
	{
		name: "history",
		aliases: ["logs", "events"],
		category: "Intelligence & Reporting",
		description:
			"Aggregate tactical event logs and the full mission history from the active session for auditing or review.",
		usage: "myth history",
		args: [],
		examples: [
			{
				command: "myth history",
				description: "View the full sequence of mission events",
			},
		],
	},
	{
		name: "report",
		category: "Intelligence & Reporting",
		description:
			"Generate a comprehensive executive intelligence summary. Leverages the AI agent to analyze all collected telemetry into a strategic document.",
		usage: "myth report",
		args: [],
		examples: [
			{
				command: "myth report",
				description: "Generate a final executive mission report",
			},
		],
	},
	{
		name: "sync",
		category: "Asset Management",
		description:
			"Force a re-synchronization with local tool registries and hot-reloads the global configuration across all MCP bridges.",
		usage: "myth sync",
		args: [],
		examples: [
			{
				command: "myth sync",
				description: "Re-synchronize all core mission assets",
			},
		],
	},
	{
		name: "inspect",
		aliases: ["man", "info"],
		category: "Intelligence & Reporting",
		description:
			"Retrieve deep technical documentation for a specific tool, including JSON Schema definitions, required inputs, and neural usage notes.",
		usage: "myth inspect <name>",
		args: [
			{
				name: "name",
				type: "string",
				required: true,
				description: "Target tool or operational topic",
			},
		],
		examples: [
			{
				command: "myth inspect nmap",
				description: "View technical specs for the nmap bridge",
			},
		],
	},
	{
		name: "depth",
		category: "System & Maintenance",
		description:
			"Modulate the maximum neural iteration depth for the agent. This governs how many successive tool-calls the LLM can make per prompt.",
		usage: "myth depth <number>",
		args: [
			{
				name: "depth",
				type: "u32",
				required: true,
				description: "Max iteration rounds (tactical ceiling)",
			},
		],
		examples: [
			{
				command: "myth depth 50",
				description: "Set the neural iteration depth to 50",
			},
		],
	},
	{
		name: "burn",
		category: "System & Maintenance",
		description:
			"EMERGENCY PURGE: Immediate shredding of all session data and hard system shutdown. Destroys all volatile RAM-based intelligence.",
		usage: "myth burn",
		args: [],
		examples: [{ command: "myth burn", description: "Emergency mission self-destruct" }],
	},
	{
		name: "wipe",
		category: "System & Maintenance",
		description:
			"Wipe the current session memory, tactical contexts, and terminal scrollback history to reset to a clean operational state.",
		usage: "myth wipe",
		args: [],
		examples: [{ command: "myth wipe", description: "Reset session and purge context" }],
	},
	{
		name: "clear",
		category: "System & Maintenance",
		description:
			"Purge visual terminal buffers. Note: Session memory and tactical context remain untouched.",
		usage: "myth clear",
		args: [],
		examples: [{ command: "myth clear", description: "Clear current visual session" }],
	},
	{
		name: "usage",
		aliases: ["u"],
		category: "System & Maintenance",
		description:
			"Display the tactical usage documentation and command doctrine in the built-in operational manual viewer.",
		usage: "myth usage",
		args: [],
		examples: [
			{
				command: "myth usage",
				description: "View the official mission manual",
			},
		],
	},
	{
		name: "version",
		aliases: ["v"],
		category: "System & Maintenance",
		description: "Display the current neural core version and build telemetry.",
		usage: "myth version",
		args: [],
		examples: [{ command: "myth version", description: "Display build specifications" }],
	},
	{
		name: "subdomains",
		category: "Mission Core",
		description:
			"High-speed multi-source subdomain discovery engine. Utilizes an 18-phase quantum-grade pipeline for unparalleled enumeration depth.",
		usage: "myth subdomains <domain> [--active] [--recursive] [--ultra]",
		args: [
			{
				name: "domain",
				type: "string",
				required: true,
				description: "Target root domain",
			},
			{
				name: "--active",
				type: "boolean",
				required: false,
				description: "Activate active brute-force and permutations",
			},
			{
				name: "--recursive",
				type: "boolean",
				required: false,
				description: "Activate recursive discovery on found nodes",
			},
			{
				name: "--master",
				type: "boolean",
				required: false,
				description: "ULTRA MODE: Tor + Proxies + Deep Recursion + Massive DL",
			},
		],
		examples: [
			{
				command: "myth subdomains example.com --master",
				description: "Initialize ultra-robust sovereign discovery",
			},
		],
	},
	{
		name: "master",
		aliases: ["ultra"],
		category: "Mission Core",
		description:
			"ULTRA-ROBUST DISCOVERY: Provisions an automated mesh of Tor, Proxies, and Mega-wordlists for deep, anonymous discovery.",
		usage: "myth master <domain>",
		args: [
			{
				name: "domain",
				type: "string",
				required: true,
				description: "Target domain for ultra-deep recon",
			},
		],
		examples: [
			{
				command: "myth master target.org",
				description: "Launch anonymous ultra-deep discovery",
			},
		],
	},
	{
		name: "typography",
		aliases: ["fonts"],
		category: "System & Maintenance",
		description:
			"Manage tactical typography and terminal font synchronization. Supports 10 built-in high-performance assets, real-time fidelity audits, and Zero-Latency synchronization across modern terminal emulators.",
		usage: "myth typography <subcommand> [<args>]",
		args: [
			{
				name: "subcommand",
				type: "string",
				required: true,
				description: "Subcommand: list, set, or revert",
			},
		],
		examples: [
			{
				command: "myth typography list",
				description: "Display the tactical font asset registry",
			},
			{
				command: "myth typography set hack-nerd-font",
				description: "Synchronize terminal with Hack Nerd Font",
			},
			{
				command: "myth typography revert",
				description: "Restore terminal to its original OS state",
			},
		],
	},
	{
		name: "completions",
		category: "System & Maintenance",
		description:
			"Generate high-performance shell autocompletion tactical scripts for various shell systems (bash, zsh, etc.).",
		usage: "myth completions <shell>",
		args: [
			{
				name: "shell",
				type: "string",
				required: true,
				description: "Target shell architecture",
			},
		],
		examples: [
			{
				command: "myth completions zsh > ~/.zfunc/_myth",
				description: "Generate and install zsh completions",
			},
		],
	},
];

export interface McpServer {
	name: string;
	type: "local" | "remote" | "custom";
	description: string;
	command?: string;
	url?: string;
	transport: string;
	tools?: string[];
	envVars?: string[];
}

export const builtinMcpServers: McpServer[] = [
	{
		name: "filesystem",
		type: "local",
		description:
			"Read/write files and directories in the mission workspace. Provides sandboxed file operations.",
		command: "npx @modelcontextprotocol/server-filesystem",
		transport: "stdio",
		tools: [
			"read_file",
			"write_file",
			"list_directory",
			"create_directory",
			"move_file",
			"search_files",
			"get_file_info",
		],
	},
	{
		name: "lightpanda",
		type: "local",
		description:
			"PRIMARY High-performance browser engine built in Zig. 11x faster than Chromium with 9x less RAM. Zero-latency JS reconnaissance.",
		command: "lightpanda",
		transport: "stdio",
		tools: ["navigate", "click", "screenshot", "evaluate_js", "get_html", "get_text"],
	},
	{
		name: "sqlite",
		type: "local",
		description:
			"Execute SQL queries against SQLite databases. Useful for local database reconnaissance.",
		command: "npx @modelcontextprotocol/server-sqlite",
		transport: "stdio",
		tools: ["read_query", "write_query", "create_table", "list_tables", "describe_table"],
	},
	{
		name: "playwright",
		type: "local",
		description:
			"Legacy browser automation for specialized UI interactions. Used as a fallback for complex Chromium-only scenarios.",
		command: "npx @playwright/mcp@latest",
		transport: "stdio",
		tools: ["navigate", "click", "fill", "screenshot", "evaluate", "wait_for_selector"],
	},
	{
		name: "webfetch",
		type: "local",
		description:
			"Advanced HTTP fetching with Readability extraction. Converts web content to clean markdown.",
		command: "npx @aspect-build/mcp-webfetch@latest",
		transport: "stdio",
		tools: ["webfetch", "webfetch_batch"],
	},
	{
		name: "llm_researcher",
		type: "local",
		description:
			"Deep research assistant that performs multi-step web research with LLM-powered synthesis.",
		command: "npx @anthropic/mcp-researcher",
		transport: "stdio",
		tools: ["research", "summarize", "fact_check"],
	},
	{
		name: "open-websearch",
		type: "local",
		description:
			"Open web search integration for real-time internet querying. Provides structured search results from multiple engines.",
		command: "npx open-websearch-mcp@latest",
		transport: "stdio",
		tools: ["web_search"],
	},
	{
		name: "fetch",
		type: "remote",
		description: "Remote HTTP fetch service for URL content retrieval with header and body access.",
		url: "https://router.mcp.so/sse/fetch",
		transport: "sse",
		tools: ["fetch"],
	},
	{
		name: "github",
		type: "remote",
		description:
			"GitHub API integration for repository management, issue tracking, and code search.",
		url: "https://router.mcp.so/sse/github",
		transport: "sse",
		tools: ["search_repositories", "create_issue", "list_commits", "get_file_contents"],
		envVars: ["GITHUB_PERSONAL_ACCESS_TOKEN"],
	},
	{
		name: "exa",
		type: "remote",
		description:
			"Exa AI search engine for web intelligence. Provides semantic search with content extraction.",
		url: "https://router.mcp.so/sse/exa",
		transport: "sse",
		tools: ["search", "find_similar", "get_contents"],
		envVars: ["EXA_API_KEY"],
	},
	{
		name: "jina",
		type: "remote",
		description:
			"Jina AI reader for converting web pages to clean text. Excellent for parsing complex web content.",
		url: "https://router.mcp.so/sse/jina",
		transport: "sse",
		tools: ["read_url", "search_web"],
	},
	{
		name: "codegate",
		type: "custom",
		description:
			"CodeGate security proxy for AI-powered code analysis and vulnerability detection.",
		command: "npx codegate-mcp",
		transport: "stdio",
		tools: ["analyze_code", "scan_dependencies"],
	},
];

export interface BuiltinTool {
	name: string;
	category: string;
	description: string;
	parameters: {
		name: string;
		type: string;
		required: boolean;
		description: string;
	}[];
}

export const builtinTools: BuiltinTool[] = [
	{
		name: "generate_file",
		category: "Utility",
		description:
			"Generate a new file (or overwrite) with precise content control. Scoped to mission workspace.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Relative path in workspace",
			},
			{
				name: "content",
				type: "string",
				required: true,
				description: "Raw content to write",
			},
		],
	},
	{
		name: "append_to_file",
		category: "Utility",
		description: "Append content to an existing mission asset. Scoped to mission workspace.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Relative path in workspace",
			},
			{
				name: "content",
				type: "string",
				required: true,
				description: "Content to append",
			},
		],
	},
	{
		name: "search_memory",
		category: "Memory",
		description:
			"Search session memory using semantic retrieval. Recall past findings and tool outputs.",
		parameters: [
			{
				name: "query",
				type: "string",
				required: true,
				description: "Search query",
			},
			{
				name: "limit",
				type: "integer",
				required: false,
				description: "Max results (default: 5)",
			},
		],
	},
	{
		name: "report_phase_completion",
		category: "Mission",
		description: "Advance to the next mission phase. Formally status findings.",
		parameters: [
			{
				name: "phase",
				type: "string",
				required: true,
				description: "Completed phase number",
			},
			{
				name: "summary",
				type: "string",
				required: true,
				description: "Accomplishment summary",
			},
			{
				name: "next_steps",
				type: "array",
				required: false,
				description: "Strategic next moves",
			},
		],
	},
	{
		name: "browse",
		category: "Web",
		description: "Navigate to a URL and extract content. Supports authenticated sessions.",
		parameters: [
			{
				name: "url",
				type: "string",
				required: true,
				description: "Target URL",
			},
			{
				name: "session_name",
				type: "string",
				required: false,
				description: "Optional auth session identifier",
			},
		],
	},
	{
		name: "web_action",
		category: "Web",
		description: "Perform browser actions: click, type, screenshot. Supports headless automation.",
		parameters: [
			{
				name: "action",
				type: "string (click|type|screenshot|wait_for)",
				required: true,
				description: "Browser action to perform",
			},
			{
				name: "selector",
				type: "string",
				required: false,
				description: "CSS selector for the target element",
			},
			{
				name: "text",
				type: "string",
				required: false,
				description: "Text to type (for type action)",
			},
			{
				name: "url",
				type: "string",
				required: false,
				description: "URL for screenshot action",
			},
			{
				name: "output_path",
				type: "string",
				required: false,
				description: "Local screenshot save path",
			},
			{
				name: "timeout",
				type: "integer",
				required: false,
				description: "Wait timeout in seconds (default: 30)",
			},
			{
				name: "session_name",
				type: "string",
				required: false,
				description: "Auth session identifier",
			},
		],
	},
	{
		name: "web_login",
		category: "Web",
		description: "Automate form-based authentication to establish a session.",
		parameters: [
			{
				name: "url",
				type: "string",
				required: true,
				description: "Login page URL",
			},
			{
				name: "user_selector",
				type: "string",
				required: true,
				description: "CSS selector for username field",
			},
			{
				name: "pass_selector",
				type: "string",
				required: true,
				description: "CSS selector for password field",
			},
			{
				name: "user_value",
				type: "string",
				required: true,
				description: "Username value",
			},
			{
				name: "pass_value",
				type: "string",
				required: true,
				description: "Password value",
			},
			{
				name: "session_name",
				type: "string",
				required: false,
				description: "Session identifier for reuse",
			},
		],
	},
	{
		name: "generate_secure_asset",
		category: "Security",
		description: "Generate an AES-256-GCM-SIV encrypted mission asset with forensic metadata.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Relative path in workspace",
			},
			{
				name: "content",
				type: "string",
				required: false,
				description: "Raw content to encrypt",
			},
			{
				name: "key",
				type: "string",
				required: true,
				description: "Hex-encoded 32-byte encryption key",
			},
		],
	},
	{
		name: "generate_batch",
		category: "Utility",
		description: "Generate multiple assets in parallel using multi-core hybrid orchestration.",
		parameters: [
			{
				name: "files",
				type: "array",
				required: true,
				description: "Array of [path, content] pairs",
			},
		],
	},
	{
		name: "generate_secure_batch",
		category: "Security",
		description: "Generate multiple encrypted assets in parallel (Sovereign Tier Scale).",
		parameters: [
			{
				name: "assets",
				type: "array",
				required: true,
				description: "Array of [path, content, hex_key] triples",
			},
		],
	},
	{
		name: "generate_compressed_batch",
		category: "Utility",
		description: "Parallel multi-threaded compression (Zstd) of mission archives.",
		parameters: [
			{
				name: "files",
				type: "array",
				required: true,
				description: "Array of [path, content, level] triples",
			},
		],
	},
	{
		name: "patch_json",
		category: "Utility",
		description: "Apply atomic RFC 6902 structural patches to JSON datasets.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Target JSON file path",
			},
			{
				name: "patch",
				type: "object",
				required: true,
				description: "RFC 6902 patch operations",
			},
		],
	},
	{
		name: "read_mmap",
		category: "Utility",
		description: "Zero-copy Memory-Mapped reading of massive (1GB+) mission assets.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "File path to memory-map",
			},
		],
	},
	{
		name: "generate_payload",
		category: "Security",
		description:
			"Generate specialized security payloads (webshells, reverseshells) for specific targets.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Relative path in workspace",
			},
			{
				name: "payload_type",
				type: "string",
				required: true,
				description: "Type of payload (e.g., webshell, reverseshell)",
			},
		],
	},
	{
		name: "generate_payload_file",
		category: "Security",
		description: "Generate a standalone payload file with targeted formatting and headers.",
		parameters: [
			{
				name: "format",
				type: "string",
				required: true,
				description: "File format (e.g., php, py, exe)",
			},
			{
				name: "payload_type",
				type: "string",
				required: true,
				description: "Type of payload",
			},
		],
	},
	{
		name: "generate_with_metadata",
		category: "Utility",
		description: "Generate a file with custom mission metadata and forensic tracking.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Relative path in workspace",
			},
			{
				name: "format",
				type: "string",
				required: true,
				description: "Target format",
			},
			{
				name: "content",
				type: "string",
				required: false,
				description: "Optional raw content",
			},
			{
				name: "metadata",
				type: "object",
				required: true,
				description: "Custom key-value pairs",
			},
		],
	},
	{
		name: "generate_compressed",
		category: "Utility",
		description: "High-speed Zstd compression of a single mission asset.",
		parameters: [
			{
				name: "path",
				type: "string",
				required: true,
				description: "Target path",
			},
			{
				name: "content",
				type: "string",
				required: true,
				description: "Raw content to compress",
			},
			{
				name: "level",
				type: "integer",
				required: false,
				description: "Compression level 1-22 (default: 3)",
			},
		],
	},
	{
		name: "get_statistics",
		category: "Mission",
		description:
			"Retrieve real-time telemetry on file generation performance and asset cataloging.",
		parameters: [],
	},
	{
		name: "subdomain_fetch",
		category: "Recon",
		description:
			"Elite, 18-phase subdomain discovery engine. Combines 77+ passive sources with high-speed active brute-forcing, permutations, TLS SAN extraction, and web crawling. Support for proxies, Tor, and stealth profiles.",
		parameters: [
			{
				name: "domain",
				type: "string",
				required: true,
				description: "Target domain (e.g., example.com)",
			},
		],
	},
];

export interface ToolBridge {
	name: string;
	rigName: string;
	description: string;
	category: string;
}

export const toolBridges: ToolBridge[] = [
	{
		name: "ExecuteToolBridge",
		rigName: "execute_tool",
		description:
			"Execute any security tool inside the Bubblewrap sandbox. Resolves binary paths, dispatches to local/external/native tools, stores results in memory.",
		category: "Core Execution",
	},
	{
		name: "ExecuteBatchBridge",
		rigName: "execute_batch",
		description:
			"Execute multiple tools in PARALLEL (Swarm Mode). Great for surface mapping, port scanning multiple targets, or simultaneous enumeration.",
		category: "Core Execution",
	},
	{
		name: "DiscoverToolsBridge",
		rigName: "discover_tools",
		description:
			"Discover available tools by search query or category filter. Returns up to 200 tools with name, category, and path.",
		category: "Discovery",
	},
	{
		name: "GetToolHelpBridge",
		rigName: "get_tool_help",
		description:
			"Get the --help output for a specific tool binary, or retrieve JSON Schema from native registry.",
		category: "Discovery",
	},
	{
		name: "ReportPhaseCompletionBridge",
		rigName: "report_phase_completion",
		description:
			"Formally advance to the next reconnaissance phase. Tracks progress through the 13-phase methodology.",
		category: "Mission Control",
	},
	{
		name: "ReportFindingBridge",
		rigName: "report_finding",
		description:
			"Register a critical finding (vulnerability, leak, or asset) into the session ReconGraph with severity classification.",
		category: "Mission Control",
	},
	{
		name: "SearchMemoryBridge",
		rigName: "search_memory",
		description:
			"Semantic vector search across session history. Powered by Qdrant in-memory store with NIM embeddings.",
		category: "Memory",
	},
	{
		name: "ListResourcesBridge",
		rigName: "list_resources",
		description: "List available resources (data files, schemas, logs) from connected MCP servers.",
		category: "MCP Protocol",
	},
	{
		name: "ReadResourceBridge",
		rigName: "read_resource",
		description: "Read the contents of a specific resource from a connected MCP server.",
		category: "MCP Protocol",
	},
	{
		name: "ListPromptsBridge",
		rigName: "list_prompts",
		description: "List available prompt templates from connected MCP servers.",
		category: "MCP Protocol",
	},
	{
		name: "GetPromptBridge",
		rigName: "get_prompt",
		description: "Retrieve a specific prompt template with arguments from an MCP server.",
		category: "MCP Protocol",
	},
	{
		name: "GenerateFileBridge",
		rigName: "generate_file",
		description:
			"Delegates to the native FileGenerator for creating files with format support and metadata tracking.",
		category: "Native Utilities",
	},
	{
		name: "AppendToFileBridge",
		rigName: "append_to_file",
		description:
			"Delegates to the native FileGenerator for appending content to existing mission assets.",
		category: "Native Utilities",
	},
	{
		name: "BrowseBridge",
		rigName: "browse",
		description:
			"Delegates to the native WebAutomation engine for headless browser content extraction.",
		category: "Native Utilities",
	},
	{
		name: "WebActionBridge",
		rigName: "web_action",
		description:
			"Delegates to the native WebAutomation engine for click, type, screenshot, and wait_for actions.",
		category: "Native Utilities",
	},
	{
		name: "WebLoginBridge",
		rigName: "web_login",
		description:
			"Delegates to the native WebAutomation engine for automated form-based authentication.",
		category: "Native Utilities",
	},
];

export const reconPhases = [
	{
		phase: 0,
		name: "Organizational Mapping",
		description: "WHOIS, corporate registry, subsidiary detection, org chart intelligence",
	},
	{
		phase: 1,
		name: "DNS & Domain Intelligence",
		description:
			"Zone transfers, subdomain brute-force, DNS record analysis, registrar intelligence",
	},
	{
		phase: 2,
		name: "Network Perimeter Mapping",
		description: "Port scanning, service enumeration, firewall detection, CDN/WAF fingerprinting",
	},
	{
		phase: 3,
		name: "Web Application Fingerprinting",
		description: "Technology stack detection, CMS identification, framework analysis",
	},
	{
		phase: 4,
		name: "Authentication & Session Analysis",
		description: "Login flow analysis, session token inspection, auth bypass reconnaissance",
	},
	{
		phase: 5,
		name: "Content & Resource Discovery",
		description: "Directory brute-forcing, hidden endpoint detection, backup file hunting",
	},
	{
		phase: 6,
		name: "Cloud & Infrastructure Intelligence",
		description: "Cloud provider detection, S3 bucket enumeration, serverless function mapping",
	},
	{
		phase: 7,
		name: "API & Service Analysis",
		description: "REST/GraphQL endpoint discovery, API schema extraction, version detection",
	},
	{
		phase: 8,
		name: "Vulnerability Assessment",
		description: "Known CVE matching, misconfig detection, injection point identification",
	},
	{
		phase: 9,
		name: "Data Leakage & OSINT",
		description: "Credential leak databases, paste sites, code repositories, social media",
	},
	{
		phase: 10,
		name: "Supply Chain Analysis",
		description: "Third-party dependency audit, CDN analysis, embedded resource tracking",
	},
	{
		phase: 11,
		name: "Continuous Monitoring Setup",
		description: "Change detection, certificate expiry monitoring, DNS change alerts",
	},
	{
		phase: 12,
		name: "Reporting & Synthesis",
		description: "Executive summary generation, finding correlation, risk scoring",
	},
];

export interface InteractiveCommand {
	name: string;
	category: string;
	description: string;
	usage: string;
	cliEquiv: string;
	examples: { command: string; description: string }[];
}

export const interactiveCommands: InteractiveCommand[] = [
	{
		name: "scan",
		category: "Mission Core",
		description: "Initialize automated asset discovery with full 13-phase methodology.",
		usage: "/scan <target> [--profile <name>]",
		cliEquiv: "myth scan <target>",
		examples: [
			{
				command: "scan example.com",
				description: "Standard 13-phase discovery against example.com",
			},
			{
				command: "/scan 192.168.1.0/24 --profile quick",
				description: "Fast subnet sweep using the 'quick' recon profile",
			},
		],
	},
	{
		name: "recon",
		category: "Mission Core",
		description:
			"Agent-led deep interrogation mission (alias for scan). Provides high-fidelity intelligence gathering.",
		usage: "/recon <target>",
		cliEquiv: "myth recon <target>",
		examples: [
			{
				command: "recon target.com",
				description: "Initialize deep agent-led reconnaissance mission.",
			},
		],
	},
	{
		name: "target",
		category: "Mission Core",
		description: "Re-align mission focus to a new target, domain, or CIDR.",
		usage: "/target <target>",
		cliEquiv: "myth target <target>",
		examples: [
			{
				command: "/target internal.nexus.local",
				description: "Shift neural focus to an internal domain.",
			},
		],
	},
	{
		name: "depth",
		category: "Mission Core",
		description: "Modulate neural iteration and recursion depth for automated scanning.",
		usage: "/depth <1-100>",
		cliEquiv: "myth depth <n>",
		examples: [
			{
				command: "depth 5",
				description: "Increase recursion depth for aggressive discovery.",
			},
		],
	},
	{
		name: "subdomains",
		category: "Mission Core",
		description:
			"High-speed multi-source subdomain discovery engine. Use /subdomains for full flag reference.",
		usage: "/subdomains <domain> [flags]",
		cliEquiv: "myth subdomains <domain>",
		examples: [
			{
				command: "subdomains google.com --active --recursive",
				description: "Aggressive multi-depth subdomain extraction.",
			},
		],
	},
	{
		name: "master",
		category: "Mission Core",
		description: "ULTRA-ROBUST DISCOVERY: Tor + Proxies + Mega Wordlist + Deep Recursion.",
		usage: "/master <domain>",
		cliEquiv: "myth master <domain>",
		examples: [
			{
				command: "/master stealth-target.com",
				description: "Maximum-depth robust discovery via Tor network.",
			},
		],
	},
	{
		name: "stealth",
		category: "Precision Ops",
		description:
			"Zero-footprint, OSINT-only intelligence gathering targeting passive data sources.",
		usage: "/stealth <target>",
		cliEquiv: "myth stealth <target>",
		examples: [
			{
				command: "stealth corporate.com",
				description: "Passive intelligence gathering without touching target infrastructure.",
			},
		],
	},
	{
		name: "osint",
		category: "Precision Ops",
		description: "Specialized Open Source Intelligence mapping and retrieval.",
		usage: "/osint <target>",
		cliEquiv: "myth osint <target>",
		examples: [
			{
				command: "/osint user@target.com",
				description: "Retrieve public intelligence related to a specific identity.",
			},
		],
	},
	{
		name: "vuln",
		category: "Precision Ops",
		description: "Deep-vector vulnerability assessment engine for targeted security analysis.",
		usage: "/vuln <target>",
		cliEquiv: "myth vuln <target>",
		examples: [
			{
				command: "vuln 10.0.0.5",
				description: "Initialize vulnerability assessment on a specific host.",
			},
		],
	},
	{
		name: "findings",
		category: "Intelligence & Analytics",
		description: "Aggregate and view all discovered tactical intelligence for the current session.",
		usage: "/findings",
		cliEquiv: "myth findings",
		examples: [{ command: "/findings", description: "View all session findings." }],
	},
	{
		name: "graph",
		category: "Intelligence & Analytics",
		description: "Render infrastructure relationship topology and tactical connection graph.",
		usage: "/graph",
		cliEquiv: "myth graph",
		examples: [{ command: "graph", description: "Render connection topology." }],
	},
	{
		name: "history",
		category: "Intelligence & Analytics",
		description: "Retrieve tactical event logs and mission history. (Aliases: /logs, /events)",
		usage: "/history",
		cliEquiv: "myth logs",
		examples: [
			{
				command: "/history --limit 50",
				description: "View the last 50 mission events.",
			},
		],
	},
	{
		name: "report",
		category: "Intelligence & Analytics",
		description: "Generate a comprehensive executive mission summary via neural synthesis.",
		usage: "/report",
		cliEquiv: "myth report",
		examples: [
			{
				command: "report --format pdf",
				description: "Generate a PDF executive summary.",
			},
		],
	},
	{
		name: "vitals",
		category: "Intelligence & Analytics",
		description:
			"View neural pulses, session lifecycle telemetry, and resource usage. (Alias: /status)",
		usage: "/vitals",
		cliEquiv: "myth status",
		examples: [{ command: "/vitals", description: "View system telemetry." }],
	},
	{
		name: "tools",
		category: "Asset Registry",
		description: "Catalog and inspect all synchronized mission-ready tools.",
		usage: "/tools",
		cliEquiv: "myth tools",
		examples: [{ command: "tools", description: "List all active tools." }],
	},
	{
		name: "inspect",
		category: "Asset Registry",
		description:
			"Retrieve deep technical documentation for a specific asset. (Aliases: /man, /info)",
		usage: "/inspect <tool>",
		cliEquiv: "myth inspect <tool>",
		examples: [
			{
				command: "/inspect nmap",
				description: "View deep manual for the nmap tool bridge.",
			},
		],
	},
	{
		name: "mcp",
		category: "Asset Registry",
		description: "Manage and synchronize Model Context Protocol server assets.",
		usage: "/mcp [list | sync | toggle | add]",
		cliEquiv: "myth mcp",
		examples: [
			{
				command: "mcp list",
				description: "List all MCP servers and their PIDs.",
			},
		],
	},
	{
		name: "sync",
		category: "Asset Registry",
		description: "Force hot-plug re-synchronization of all neural links.",
		usage: "/sync",
		cliEquiv: "myth sync",
		examples: [
			{
				command: "/sync --force",
				description: "Force immediate re-sync of all assets.",
			},
		],
	},
	{
		name: "config",
		category: "System & Maintenance",
		description: "View or modulate mission configuration (API keys are masked).",
		usage: "/config",
		cliEquiv: "myth config",
		examples: [{ command: "config", description: "View current configuration." }],
	},
	{
		name: "check",
		category: "System & Maintenance",
		description: "Run comprehensive system health diagnostics and connectivity tests.",
		usage: "/check",
		cliEquiv: "myth check",
		examples: [{ command: "/check", description: "Diagnostic system check." }],
	},
	{
		name: "burn",
		category: "System & Maintenance",
		description: "EMERGENCY PURGE: Volatile buffer destruction and session shredding.",
		usage: "/burn",
		cliEquiv: "myth burn",
		examples: [
			{
				command: "burn --now",
				description: "Immediate emergency session termination.",
			},
		],
	},
];

export interface ReconProfile {
	name: string;
	description: string;
	phases: string;
	steps: number;
	premium?: boolean;
	example: string;
}

export const reconProfiles: ReconProfile[] = [
	{
		name: "quick",
		description: "Rapid surface-level scan covering organizational and DNS phases.",
		phases: "0-5",
		steps: 30,
		example: "myth scan target.com --profile quick",
	},
	{
		name: "full",
		description: "Comprehensive 13-phase methodology with all 89 operational steps.",
		phases: "0-12",
		steps: 89,
		premium: true,
		example: "myth scan target.com --profile full",
	},
	{
		name: "stealth",
		description: "Low-noise passive-only operations prioritizing OSINT and metadata.",
		phases: "0,1,9",
		steps: 15,
		example: "myth scan target.com --profile stealth",
	},
	{
		name: "vuln",
		description: "Vulnerability-focused assessment skipping initial passive discovery.",
		phases: "2,3,4,5,7,8",
		steps: 45,
		example: "myth scan target.com --profile vuln",
	},
	{
		name: "elite",
		description: "Maximum depth — all 13 phases enabled with extended neural recursion.",
		phases: "0-12",
		steps: 89,
		premium: true,
		example: "myth scan target.com --profile elite",
	},
];

export interface SubdomainFlag {
	flag: string;
	description: string;
	category: "Operational" | "Stealth" | "Networking";
	example: string;
}

export const subdomainFlags: SubdomainFlag[] = [
	{
		flag: "--active",
		description: "Enable active brute-force and permutation scanning.",
		category: "Operational",
		example: "myth subdomains example.com --active",
	},
	{
		flag: "--recursive",
		description: "Enable recursive discovery on found subdomains.",
		category: "Operational",
		example: "myth subdomains example.com --recursive",
	},
	{
		flag: "--only-alive",
		description: "Filter results to only show live subdomains (Default: true).",
		category: "Operational",
		example: "myth subdomains example.com --only-alive",
	},
	{
		flag: "--master",
		description: "ULTRA-ROBUST MODE: Tor + Proxies + Mega Wordlist + Deep Recursion.",
		category: "Stealth",
		example: "myth subdomains example.com --master",
	},
	{
		flag: "--stealth",
		description: "Reduce concurrency and add randomized delays for evasion.",
		category: "Stealth",
		example: "myth subdomains example.com --stealth",
	},
	{
		flag: "--tor",
		description: "Route all discovery traffic through the Tor network.",
		category: "Networking",
		example: "myth subdomains example.com --tor",
	},
	{
		flag: "--proxies-file",
		description: "Use a specific list of proxies from a provided file.",
		category: "Networking",
		example: "myth subdomains example.com --proxies-file proxies.txt",
	},
];

export interface SubdomainParam {
	name: string;
	type: string;
	required: boolean;
	def: string;
	desc: string;
	example: string;
}

export const subdomainParams: SubdomainParam[] = [
	{
		name: "domain",
		type: "string",
		required: true,
		def: "-",
		desc: "Target domain (e.g., example.com)",
		example: '"domain": "example.com"',
	},
	{
		name: "active",
		type: "boolean",
		required: false,
		def: "false",
		desc: "Enable active brute-force and permutation scanning",
		example: '"active": true',
	},
	{
		name: "recursive",
		type: "boolean",
		required: false,
		def: "false",
		desc: "Enable recursive discovery (scans sub-subdomains)",
		example: '"recursive": true',
	},
	{
		name: "only_alive",
		type: "boolean",
		required: false,
		def: "true",
		desc: "Only return subdomains that resolve to IPs",
		example: '"only_alive": true',
	},
	{
		name: "stealth",
		type: "boolean",
		required: false,
		def: "false",
		desc: "Stealth mode: reduces concurrency and adds randomized delays",
		example: '"stealth": true',
	},
	{
		name: "concurrency",
		type: "integer",
		required: false,
		def: "50",
		desc: "Maximum concurrent discovery tasks",
		example: '"concurrency": 100',
	},
	{
		name: "use_tor",
		type: "boolean",
		required: false,
		def: "false",
		desc: "Route all discovery traffic through the Tor network",
		example: '"use_tor": true',
	},
	{
		name: "wordlist_type",
		type: "enum",
		required: false,
		def: "medium",
		desc: "Built-in wordlist: none | small | medium | large | quick | deep | mega",
		example: '"wordlist_type": "mega"',
	},
];

export const discoveryPhases = [
	{
		name: "Passive Aggregation",
		desc: "100+ sources (sovereign zero-latency aggregation)",
	},
	{
		name: "Streaming Brute-force",
		desc: "2GB+ cloud-streamed high-entropy wordlists",
	},
	{
		name: "Wildcard Filtering",
		desc: "Zero-noise algorithmic DNS wildcard detection",
	},
	{
		name: "AltDNS Mutation",
		desc: "Dash-dot permutations and incremental logic",
	},
	{
		name: "Recursive Chaining",
		desc: "Multi-depth automated discovery pipelines",
	},
	{
		name: "Source Map Analysis",
		desc: "Extracting hidden operational logic from maps",
	},
	{
		name: "JS Variable Scraping",
		desc: "High-fidelity HTML/JS relationship mapping",
	},
	{
		name: "Hardened VHost Probing",
		desc: "SNI-aware sovereign discovery logic",
	},
	{
		name: "ENT Discovery",
		desc: "Empty Non-Terminal recursive resolution",
	},
	{
		name: "NSEC Zone Walking",
		desc: "DNSSEC chain dumping and mapping",
	},
	{
		name: "NSEC3 Chain Mapping",
		desc: "Advanced DNSSEC security verification",
	},
	{
		name: "AXFR Zone Transfer",
		desc: "Legacy zone transfer vulnerability auditing",
	},
	{
		name: "PTR Reverse Mapping",
		desc: "Recursive in-addr.arpa reverse sweeps",
	},
	{
		name: "CIDR Block Expansion",
		desc: "RDAP organizational sweeps and identification",
	},
	{
		name: "Neural Cloud Recon",
		desc: "AWS/Azure/GCP high-speed artifact probing",
	},
	{
		name: "Hidden Asset Crawler",
		desc: "Automated robots/security path analysis",
	},
	{
		name: "TLS SAN Extraction",
		desc: "Sovereign multi-port certificate parsing",
	},
	{
		name: "Quantum Logic Closure",
		desc: "Final 10k req/sec operational validation",
	},
];

export const typographyContent = {
	title: "Mission Typography Suite",
	subtitle: "Industrial visual fidelity and high-performance terminal aesthetics.",
	description:
		"The MYTH Typography Suite is a mission-critical synchronization engine designed for elite researchers. It bridges high-fidelity visualization with industrial-grade terminal aesthetics, ensuring zero-latency synchronization of tactical assets.",
	features: [
		{
			title: "Sovereign Sync",
			desc: "Direct-to-buffer synchronization utilizing ANSI RIS and terminal escape vectors.",
		},
		{
			title: "Tactical Provisioning",
			desc: "Integrated asset acquisition engine: if a requested tool is missing, the core triggers an automated installation cycle.",
		},
		{
			title: "Neural Safety",
			desc: "Universal 'Kill-Switch' restoration. Sessions are wrapped in panic-safe hooks to ensure original terminal state protection.",
		},
	],
	fonts: [
		{ name: "JetBrains Mono", id: "jet-brains-mono", style: "Nerdy / Professional" },
		{ name: "Hack Nerd Font", id: "hack-nerd-font", style: "Standard Tactical" },
		{ name: "Agave Nerd Font", id: "agave-nerd-font", style: "Ultra-Clean / Thin" },
		{ name: "Fira Code", id: "fira-code", style: "High-Readability Ligatures" },
		{ name: "MesloLGS NF", id: "meslo-nf", style: "Classic Powerlevel10k" },
		{ name: "CaskaydiaCove NF", id: "cascadia-nf", style: "Modern / Geometric" },
		{ name: "Inconsolata LGC", id: "inconsolata", style: "Monospace Excellence" },
		{ name: "UbuntuMono NF", id: "ubuntu-mono", style: "System Fluidity" },
		{ name: "VictorMono NF", id: "victor-mono", style: "Cursive Italics Focus" },
		{ name: "DroidSansMono", id: "droid-sans", style: "Retro-Tactical" },
	],
	commands: [
		{
			cmd: "myth typography list",
			desc: "Initialize a high-fidelity audit of the local tactical registry.",
		},
		{
			cmd: "myth typography set <id>",
			desc: "Synchronize the current terminal environment with a mission-critical asset.",
		},
		{
			cmd: "myth typography revert",
			desc: "Flush tactical typography and restore the global terminal default state.",
		},
	],
	technicalSpecs: {
		engine: "FontConfig / Direct ANSI",
		safety: "SIGINT/SIGTERM/SIGHUP Capture",
		recovery: "RIS (Reset to Initial State)",
		provisioning: "Zip-stream / SHA-256 Verified",
	},
};
