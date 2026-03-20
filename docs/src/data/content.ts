export const sidebarNav = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Introduction', path: '/' },
      { title: 'Installation', path: '/installation' },
      { title: 'Quick Start', path: '/quickstart' },
      { title: 'Configuration', path: '/configuration' },
    ],
  },
  {
    title: 'Architecture',
    items: [
      { title: 'Overview', path: '/architecture' },
      { title: 'Neural Vitals', path: '/vitals' },
      { title: 'Security Model', path: '/security' },
      { title: 'Memory System', path: '/memory' },
    ],
  },
  {
    title: 'Commands',
    items: [
      { title: 'CLI Commands', path: '/cli-commands' },
      { title: 'Interactive Commands', path: '/interactive-commands' },
      { title: 'Recon Profiles', path: '/profiles' },
      { title: 'Subdomain Fetcher', path: '/subdomain-fetch' },
    ],
  },
  {
    title: 'MCP Ecosystem',
    items: [
      { title: 'Built-in MCP Servers', path: '/mcp-servers' },
      { title: 'Built-in Tools', path: '/builtin-tools' },
      { title: 'LLM Tool Bridges', path: '/tool-bridges' },
      { title: 'Custom MCP Servers', path: '/custom-mcp' },
    ],
  },
  {
    title: 'Reference',
    items: [
      { title: 'Scripts', path: '/scripts' },
      { title: 'Tech Stack', path: '/tech-stack' },
    ],
  },
];

export interface CliCommand {
  name: string;
  aliases?: string[];
  description: string;
  usage: string;
  args?: { name: string; type: string; required: boolean; description: string; default?: string }[];
  examples: { command: string; description: string }[];
}

export const cliCommands: CliCommand[] = [
  {
    name: 'scan',
    aliases: ['recon'],
    description: 'Initialize full-spectrum target acquisition and reconnaissance. Launches the agent with a 13-phase, 89-step methodology for comprehensive intelligence gathering.',
    usage: 'myth scan <target> [--profile <name>]',
    args: [
      { name: 'target', type: 'string', required: true, description: 'Target domain, IP, or URL for mission focus' },
      { name: '--profile', type: 'string', required: false, description: 'Reconnaissance methodology profile', default: 'full' },
    ],
    examples: [
      { command: 'myth scan example.com', description: 'Full recon on example.com' },
      { command: 'myth scan 192.168.1.0/24 --profile stealth', description: 'Stealth recon on a subnet' },
      { command: 'myth recon target.org --profile quick', description: 'Quick surface scan using alias' },
    ],
  },
  {
    name: 'stealth',
    description: 'Launch immediate low-signature, passive-only reconnaissance. Uses only OSINT techniques to minimize detection by IDS/IPS.',
    usage: 'myth stealth <target>',
    args: [
      { name: 'target', type: 'string', required: true, description: 'Target domain, IP, or URL' },
    ],
    examples: [
      { command: 'myth stealth target.com', description: 'Passive-only recon with zero active probing' },
    ],
  },
  {
    name: 'osint',
    description: 'Launch specialized Open Source Intelligence operations. Aggregates data from public records, domain registries, and cloud-leak databases without touching target infrastructure.',
    usage: 'myth osint <target>',
    args: [
      { name: 'target', type: 'string', required: true, description: 'Target domain, IP, or URL' },
    ],
    examples: [
      { command: 'myth osint company.com', description: 'OSINT-focused intelligence mapping' },
    ],
  },
  {
    name: 'vuln',
    description: 'Perform a deep, multi-vector vulnerability assessment. Leverages the full MCP toolset to identify exploitable misconfigurations, known CVEs, and logic flaws.',
    usage: 'myth vuln <target>',
    args: [
      { name: 'target', type: 'string', required: true, description: 'Target domain, IP, or URL' },
    ],
    examples: [
      { command: 'myth vuln 10.0.0.1', description: 'Deep vulnerability assessment of a specific IP' },
    ],
  },
  {
    name: 'chat',
    description: 'Launch an interactive tactical chat session with the AI agent. Opens the neural TUI by default, or a simple CLI with --no-tui.',
    usage: 'myth chat',
    args: [],
    examples: [
      { command: 'myth chat', description: 'Launch interactive TUI session' },
      { command: 'myth --no-tui chat', description: 'Launch interactive CLI session (no TUI)' },
    ],
  },
  {
    name: 'tools',
    description: 'Catalog and display all synchronized mission assets/tools. Lists local binaries, external MCP tools, and native utilities.',
    usage: 'myth tools [--category <cat>] [--search <query>]',
    args: [
      { name: '--category', type: 'string', required: false, description: 'Filter assets by technical category' },
      { name: '--search', type: 'string', required: false, description: 'Search assets by name or keyword' },
    ],
    examples: [
      { command: 'myth tools', description: 'List all available tools' },
      { command: 'myth tools --search nmap', description: 'Search for nmap-related tools' },
      { command: 'myth tools --category scanner', description: 'Filter by scanner category' },
    ],
  },
  {
    name: 'target',
    description: 'Force rotate the mission focus to a new target or CIDR range.',
    usage: 'myth target <target> [--profile <name>]',
    args: [
      { name: 'target', type: 'string', required: true, description: 'Target domain, IP, or URL' },
      { name: '--profile', type: 'string', required: false, description: 'Recon profile', default: 'quick' },
    ],
    examples: [
      { command: 'myth target newdomain.com', description: 'Switch mission focus to a new target' },
    ],
  },
  {
    name: 'config',
    description: 'Retrieve the current mission configuration metadata. API keys are masked for safe display.',
    usage: 'myth config',
    args: [],
    examples: [
      { command: 'myth config', description: 'Display current agent configuration' },
    ],
  },
  {
    name: 'profile',
    description: 'View or modulate tactical reconnaissance profiles and their phases. Supports enabling/disabling individual phases.',
    usage: 'myth profile [<name>] [<action>] [<index>]',
    args: [
      { name: 'name', type: 'string', required: false, description: 'Profile name (quick, full, stealth, etc.)' },
      { name: 'action', type: 'string', required: false, description: 'Action: enable or disable' },
      { name: 'index', type: 'string', required: false, description: 'Phase indices (comma-separated)' },
    ],
    examples: [
      { command: 'myth profile', description: 'List all available profiles' },
      { command: 'myth profile full', description: 'View full profile with all phases' },
      { command: 'myth profile elite disable 3,4,5', description: 'Disable phases 3, 4, 5 in elite profile' },
    ],
  },
  {
    name: 'check',
    aliases: ['health'],
    description: 'Verify system health, sandbox status, and tool availability with an advanced diagnostic engine.',
    usage: 'myth check',
    args: [],
    examples: [
      { command: 'myth check', description: 'Run full system health diagnostics' },
      { command: 'myth health', description: 'Same command using alias' },
    ],
  },
  {
    name: 'mcp',
    description: 'Manage Custom User MCP Servers. Supports listing, toggling, adding local/remote servers, and tool management.',
    usage: 'myth mcp <subcommand>',
    args: [],
    examples: [
      { command: 'myth mcp list', description: 'List all configured MCP servers' },
      { command: 'myth mcp toggle github on', description: 'Enable the GitHub MCP server' },
      { command: 'myth mcp add-local my-tool npx -a arg1 env:API_KEY=abc', description: 'Add a local MCP server' },
      { command: 'myth mcp add-remote my-api https://api.example.com/sse', description: 'Add a remote SSE MCP server' },
      { command: 'myth mcp tools github', description: 'List tools for the GitHub MCP server' },
      { command: 'myth mcp sync', description: 'Force re-sync factory defaults to mcp.json' },
      { command: 'myth mcp remove old-server', description: 'Remove an MCP server' },
      { command: 'myth mcp allow-tool github create_issue', description: 'Allow a specific tool' },
      { command: 'myth mcp block-tool github delete_repo', description: 'Block a specific tool' },
    ],
  },
  {
    name: 'vitals',
    aliases: ['status'],
    description: 'Analyze neural pulses and session lifecycle metadata. Shows core integrity, session ID, target vector, profile, and uptime.',
    usage: 'myth vitals',
    args: [],
    examples: [
      { command: 'myth vitals', description: 'View session vitals and telemetry' },
    ],
  },
  {
    name: 'findings',
    description: 'Aggregate and display all discovered tactical intelligence from the current session.',
    usage: 'myth findings',
    args: [],
    examples: [
      { command: 'myth findings', description: 'List all critical findings' },
    ],
  },
  {
    name: 'graph',
    description: 'Render the infrastructure relationship graph of target assets showing targets, findings, and relationships.',
    usage: 'myth graph',
    args: [],
    examples: [
      { command: 'myth graph', description: 'Display infrastructure graph' },
    ],
  },
  {
    name: 'history',
    aliases: ['logs', 'events'],
    description: 'Aggregate tactical event logs and mission history from the current session.',
    usage: 'myth history',
    args: [],
    examples: [
      { command: 'myth history', description: 'View tactical event log' },
    ],
  },
  {
    name: 'report',
    description: 'Generate a comprehensive executive intelligence summary by leveraging the AI agent to analyze all collected data.',
    usage: 'myth report',
    args: [],
    examples: [
      { command: 'myth report', description: 'Generate executive summary report' },
    ],
  },
  {
    name: 'sync',
    description: 'Force a re-synchronization with local tool registries and hot-reload configuration.',
    usage: 'myth sync',
    args: [],
    examples: [
      { command: 'myth sync', description: 'Re-sync tools and configuration' },
    ],
  },
  {
    name: 'inspect',
    aliases: ['man', 'info'],
    description: 'Retrieve deep technical documentation for a specific tool or asset, including JSON Schema parameter definitions.',
    usage: 'myth inspect <name>',
    args: [
      { name: 'name', type: 'string', required: true, description: 'Tool or topic name' },
    ],
    examples: [
      { command: 'myth inspect nmap', description: 'View nmap documentation' },
      { command: 'myth inspect generate_file', description: 'View native file generation tool docs' },
    ],
  },
  {
    name: 'depth',
    description: 'Modulate the maximum neural iteration depth for the agent. Controls how many tool-calling rounds the LLM can perform.',
    usage: 'myth depth <number>',
    args: [
      { name: 'depth', type: 'u32', required: true, description: 'Iteration count (e.g., 1-100)' },
    ],
    examples: [
      { command: 'myth depth 50', description: 'Set max iterations to 50' },
    ],
  },
  {
    name: 'burn',
    description: 'EMERGENCY: Immediate shred of all data and system shutdown. Destroys all session data, memory, and volatile storage.',
    usage: 'myth burn',
    args: [],
    examples: [
      { command: 'myth burn', description: 'Emergency purge all data' },
    ],
  },
  {
    name: 'wipe',
    description: 'Wipe the current session memory, tactical context, and terminal scrollback. Resets agent to clean state.',
    usage: 'myth wipe',
    args: [],
    examples: [
      { command: 'myth wipe', description: 'Clear session memory and reset' },
    ],
  },
  {
    name: 'clear',
    description: 'Purge visual buffers only. Session memory remains intact.',
    usage: 'myth clear',
    args: [],
    examples: [
      { command: 'myth clear', description: 'Clear the terminal screen' },
    ],
  },
  {
    name: 'usage',
    aliases: ['u'],
    description: 'Display the tactical usage documentation and command doctrine. Opens the built-in manual.',
    usage: 'myth usage',
    args: [],
    examples: [
      { command: 'myth usage', description: 'View tactical manual' },
    ],
  },
  {
    name: 'version',
    aliases: ['v'],
    description: 'Display the current neural core version and build telemetry.',
    usage: 'myth version',
    args: [],
    examples: [
      { command: 'myth version', description: 'Display version info' },
    ],
  },
  {
    name: 'subdomains',
    description: 'High-speed multi-source subdomain discovery engine. Utilizes an 18-phase quantum-grade pipeline for unparalleled enumeration.',
    usage: 'myth subdomains <domain> [--active] [--recursive] [--ultra]',
    args: [
      { name: 'domain', type: 'string', required: true, description: 'Target domain' },
      { name: '--active', type: 'boolean', required: false, description: 'Enable active brute-force and permutations' },
      { name: '--recursive', type: 'boolean', required: false, description: 'Enable recursive discovery on found subdomains' },
      { name: '--master', type: 'boolean', required: false, description: 'ULTRA-ROBUST MODE: Tor + Proxies + Mega Wordlist + Deep Recursion (Alias: --ultra)' },
    ],
    examples: [
      { command: 'myth subdomains example.com', description: 'Run standard subdomain discovery' },
      { command: 'myth subdomains example.com --master', description: 'Run in ultra-robust sovereign mode' },
    ],
  },
  {
    name: 'master',
    aliases: ['ultra'],
    description: 'ULTRA-ROBUST DISCOVERY: Auto-configures Tor, Proxies, Mega-wordlists, and recursive deep-scanning.',
    usage: 'myth master <domain>',
    args: [
      { name: 'domain', type: 'string', required: true, description: 'Target domain' },
    ],
    examples: [
      { command: 'myth master example.com', description: 'Run ultra-robust subdomain discovery' },
    ],
  },
  {
    name: 'completions',
    description: 'Generate high-performance shell autocompletion tactical scripts.',
    usage: 'myth completions <shell>',
    args: [
      { name: 'shell', type: 'string', required: true, description: 'Target shell environment (bash, zsh, fish, powershell, elvish)' },
    ],
    examples: [
      { command: 'myth completions bash > ~/.local/share/bash-completion/completions/myth', description: 'Generate bash completions' },
    ],
  },
];

export interface McpServer {
  name: string;
  type: 'local' | 'remote' | 'custom';
  description: string;
  command?: string;
  url?: string;
  transport: string;
  tools?: string[];
  envVars?: string[];
}

export const builtinMcpServers: McpServer[] = [
  {
    name: 'filesystem',
    type: 'local',
    description: 'Read/write files and directories in the mission workspace. Provides sandboxed file operations.',
    command: 'npx @modelcontextprotocol/server-filesystem',
    transport: 'stdio',
    tools: ['read_file', 'write_file', 'list_directory', 'create_directory', 'move_file', 'search_files', 'get_file_info'],
  },
  {
    name: 'sqlite',
    type: 'local',
    description: 'Execute SQL queries against SQLite databases. Useful for local database reconnaissance.',
    command: 'npx @modelcontextprotocol/server-sqlite',
    transport: 'stdio',
    tools: ['read_query', 'write_query', 'create_table', 'list_tables', 'describe_table'],
  },
  {
    name: 'playwright',
    type: 'local',
    description: 'Browser automation for web application testing. Supports headless Chrome/Chromium for UI interaction.',
    command: 'npx @playwright/mcp@latest',
    transport: 'stdio',
    tools: ['navigate', 'click', 'fill', 'screenshot', 'evaluate', 'wait_for_selector'],
  },
  {
    name: 'webfetch',
    type: 'local',
    description: 'Advanced HTTP fetching with Readability extraction. Converts web content to clean markdown.',
    command: 'npx @aspect-build/mcp-webfetch@latest',
    transport: 'stdio',
    tools: ['webfetch', 'webfetch_batch'],
  },
  {
    name: 'llm_researcher',
    type: 'local',
    description: 'Deep research assistant that performs multi-step web research with LLM-powered synthesis.',
    command: 'npx @anthropic/mcp-researcher',
    transport: 'stdio',
    tools: ['research', 'summarize', 'fact_check'],
  },
  {
    name: 'open-websearch',
    type: 'local',
    description: 'Open web search integration for real-time internet querying. Provides structured search results from multiple engines.',
    command: 'npx open-websearch-mcp@latest',
    transport: 'stdio',
    tools: ['web_search'],
  },
  {
    name: 'fetch',
    type: 'remote',
    description: 'Remote HTTP fetch service for URL content retrieval with header and body access.',
    url: 'https://router.mcp.so/sse/fetch',
    transport: 'sse',
    tools: ['fetch'],
  },
  {
    name: 'github',
    type: 'remote',
    description: 'GitHub API integration for repository management, issue tracking, and code search.',
    url: 'https://router.mcp.so/sse/github',
    transport: 'sse',
    tools: ['search_repositories', 'create_issue', 'list_commits', 'get_file_contents'],
    envVars: ['GITHUB_PERSONAL_ACCESS_TOKEN'],
  },
  {
    name: 'exa',
    type: 'remote',
    description: 'Exa AI search engine for web intelligence. Provides semantic search with content extraction.',
    url: 'https://router.mcp.so/sse/exa',
    transport: 'sse',
    tools: ['search', 'find_similar', 'get_contents'],
    envVars: ['EXA_API_KEY'],
  },
  {
    name: 'jina',
    type: 'remote',
    description: 'Jina AI reader for converting web pages to clean text. Excellent for parsing complex web content.',
    url: 'https://router.mcp.so/sse/jina',
    transport: 'sse',
    tools: ['read_url', 'search_web'],
  },
  {
    name: 'codegate',
    type: 'custom',
    description: 'CodeGate security proxy for AI-powered code analysis and vulnerability detection.',
    command: 'npx codegate-mcp',
    transport: 'stdio',
    tools: ['analyze_code', 'scan_dependencies'],
  },
];

export interface BuiltinTool {
  name: string;
  category: string;
  description: string;
  parameters: { name: string; type: string; required: boolean; description: string }[];
}

export const builtinTools: BuiltinTool[] = [
  {
    name: 'generate_file',
    category: 'Utility',
    description: 'Generate a new file (or overwrite) with precise content control. Scoped to mission workspace.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Relative path in workspace' },
      { name: 'content', type: 'string', required: true, description: 'Raw content to write' },
    ],
  },
  {
    name: 'append_to_file',
    category: 'Utility',
    description: 'Append content to an existing mission asset. Scoped to mission workspace.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Relative path in workspace' },
      { name: 'content', type: 'string', required: true, description: 'Content to append' },
    ],
  },
  {
    name: 'search_memory',
    category: 'Memory',
    description: 'Search session memory using semantic retrieval. Recall past findings and tool outputs.',
    parameters: [
      { name: 'query', type: 'string', required: true, description: 'Search query' },
      { name: 'limit', type: 'integer', required: false, description: 'Max results (default: 5)' },
    ],
  },
  {
    name: 'report_phase_completion',
    category: 'Mission',
    description: 'Advance to the next mission phase. Formally status findings.',
    parameters: [
      { name: 'phase', type: 'string', required: true, description: 'Completed phase number' },
      { name: 'summary', type: 'string', required: true, description: 'Accomplishment summary' },
      { name: 'next_steps', type: 'array', required: false, description: 'Strategic next moves' },
    ],
  },
  {
    name: 'browse',
    category: 'Web',
    description: 'Navigate to a URL and extract content. Supports authenticated sessions.',
    parameters: [
      { name: 'url', type: 'string', required: true, description: 'Target URL' },
      { name: 'session_name', type: 'string', required: false, description: 'Optional auth session identifier' },
    ],
  },
  {
    name: 'web_action',
    category: 'Web',
    description: 'Perform browser actions: click, type, screenshot. Supports headless automation.',
    parameters: [
      { name: 'action', type: 'string (click|type|screenshot|wait_for)', required: true, description: 'Browser action to perform' },
      { name: 'selector', type: 'string', required: false, description: 'CSS selector for the target element' },
      { name: 'text', type: 'string', required: false, description: 'Text to type (for type action)' },
      { name: 'url', type: 'string', required: false, description: 'URL for screenshot action' },
      { name: 'output_path', type: 'string', required: false, description: 'Local screenshot save path' },
      { name: 'timeout', type: 'integer', required: false, description: 'Wait timeout in seconds (default: 30)' },
      { name: 'session_name', type: 'string', required: false, description: 'Auth session identifier' },
    ],
  },
  {
    name: 'web_login',
    category: 'Web',
    description: 'Automate form-based authentication to establish a session.',
    parameters: [
      { name: 'url', type: 'string', required: true, description: 'Login page URL' },
      { name: 'user_selector', type: 'string', required: true, description: 'CSS selector for username field' },
      { name: 'pass_selector', type: 'string', required: true, description: 'CSS selector for password field' },
      { name: 'user_value', type: 'string', required: true, description: 'Username value' },
      { name: 'pass_value', type: 'string', required: true, description: 'Password value' },
      { name: 'session_name', type: 'string', required: false, description: 'Session identifier for reuse' },
    ],
  },
  {
    name: 'generate_secure_asset',
    category: 'Security',
    description: 'Generate an AES-256-GCM-SIV encrypted mission asset with forensic metadata.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Relative path in workspace' },
      { name: 'content', type: 'string', required: false, description: 'Raw content to encrypt' },
      { name: 'key', type: 'string', required: true, description: 'Hex-encoded 32-byte encryption key' },
    ],
  },
  {
    name: 'generate_batch',
    category: 'Utility',
    description: 'Generate multiple assets in parallel using multi-core hybrid orchestration.',
    parameters: [
      { name: 'files', type: 'array', required: true, description: 'Array of [path, content] pairs' },
    ],
  },
  {
    name: 'generate_secure_batch',
    category: 'Security',
    description: 'Generate multiple encrypted assets in parallel (Sovereign Tier Scale).',
    parameters: [
      { name: 'assets', type: 'array', required: true, description: 'Array of [path, content, hex_key] triples' },
    ],
  },
  {
    name: 'generate_compressed_batch',
    category: 'Utility',
    description: 'Parallel multi-threaded compression (Zstd) of mission archives.',
    parameters: [
      { name: 'files', type: 'array', required: true, description: 'Array of [path, content, level] triples' },
    ],
  },
  {
    name: 'patch_json',
    category: 'Utility',
    description: 'Apply atomic RFC 6902 structural patches to JSON datasets.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Target JSON file path' },
      { name: 'patch', type: 'object', required: true, description: 'RFC 6902 patch operations' },
    ],
  },
  {
    name: 'read_mmap',
    category: 'Utility',
    description: 'Zero-copy Memory-Mapped reading of massive (1GB+) mission assets.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'File path to memory-map' },
    ],
  },
  {
    name: 'generate_payload',
    category: 'Security',
    description: 'Generate specialized security payloads (webshells, reverseshells) for specific targets.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Relative path in workspace' },
      { name: 'payload_type', type: 'string', required: true, description: 'Type of payload (e.g., webshell, reverseshell)' },
    ],
  },
  {
    name: 'generate_payload_file',
    category: 'Security',
    description: 'Generate a standalone payload file with targeted formatting and headers.',
    parameters: [
      { name: 'format', type: 'string', required: true, description: 'File format (e.g., php, py, exe)' },
      { name: 'payload_type', type: 'string', required: true, description: 'Type of payload' },
    ],
  },
  {
    name: 'generate_with_metadata',
    category: 'Utility',
    description: 'Generate a file with custom mission metadata and forensic tracking.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Relative path in workspace' },
      { name: 'format', type: 'string', required: true, description: 'Target format' },
      { name: 'content', type: 'string', required: false, description: 'Optional raw content' },
      { name: 'metadata', type: 'object', required: true, description: 'Custom key-value pairs' },
    ],
  },
  {
    name: 'generate_compressed',
    category: 'Utility',
    description: 'High-speed Zstd compression of a single mission asset.',
    parameters: [
      { name: 'path', type: 'string', required: true, description: 'Target path' },
      { name: 'content', type: 'string', required: true, description: 'Raw content to compress' },
      { name: 'level', type: 'integer', required: false, description: 'Compression level 1-22 (default: 3)' },
    ],
  },
  {
    name: 'get_statistics',
    category: 'Mission',
    description: 'Retrieve real-time telemetry on file generation performance and asset cataloging.',
    parameters: [],
  },
  {
    name: 'subdomain_fetch',
    category: 'Recon',
    description: 'Elite, 18-phase subdomain discovery engine. Combines 77+ passive sources with high-speed active brute-forcing, permutations, TLS SAN extraction, and web crawling. Support for proxies, Tor, and stealth profiles.',
    parameters: [
      { name: 'domain', type: 'string', required: true, description: 'Target domain (e.g., example.com)' },
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
  { name: 'ExecuteToolBridge', rigName: 'execute_tool', description: 'Execute any security tool inside the Bubblewrap sandbox. Resolves binary paths, dispatches to local/external/native tools, stores results in memory.', category: 'Core Execution' },
  { name: 'ExecuteBatchBridge', rigName: 'execute_batch', description: 'Execute multiple tools in PARALLEL (Swarm Mode). Great for surface mapping, port scanning multiple targets, or simultaneous enumeration.', category: 'Core Execution' },
  { name: 'DiscoverToolsBridge', rigName: 'discover_tools', description: 'Discover available tools by search query or category filter. Returns up to 200 tools with name, category, and path.', category: 'Discovery' },
  { name: 'GetToolHelpBridge', rigName: 'get_tool_help', description: 'Get the --help output for a specific tool binary, or retrieve JSON Schema from native registry.', category: 'Discovery' },
  { name: 'ReportPhaseCompletionBridge', rigName: 'report_phase_completion', description: 'Formally advance to the next reconnaissance phase. Tracks progress through the 13-phase methodology.', category: 'Mission Control' },
  { name: 'ReportFindingBridge', rigName: 'report_finding', description: 'Register a critical finding (vulnerability, leak, or asset) into the session ReconGraph with severity classification.', category: 'Mission Control' },
  { name: 'SearchMemoryBridge', rigName: 'search_memory', description: 'Semantic vector search across session history. Powered by Qdrant in-memory store with NIM embeddings.', category: 'Memory' },
  { name: 'ListResourcesBridge', rigName: 'list_resources', description: 'List available resources (data files, schemas, logs) from connected MCP servers.', category: 'MCP Protocol' },
  { name: 'ReadResourceBridge', rigName: 'read_resource', description: 'Read the contents of a specific resource from a connected MCP server.', category: 'MCP Protocol' },
  { name: 'ListPromptsBridge', rigName: 'list_prompts', description: 'List available prompt templates from connected MCP servers.', category: 'MCP Protocol' },
  { name: 'GetPromptBridge', rigName: 'get_prompt', description: 'Retrieve a specific prompt template with arguments from an MCP server.', category: 'MCP Protocol' },
  { name: 'GenerateFileBridge', rigName: 'generate_file', description: 'Delegates to the native FileGenerator for creating files with format support and metadata tracking.', category: 'Native Utilities' },
  { name: 'AppendToFileBridge', rigName: 'append_to_file', description: 'Delegates to the native FileGenerator for appending content to existing mission assets.', category: 'Native Utilities' },
  { name: 'BrowseBridge', rigName: 'browse', description: 'Delegates to the native WebAutomation engine for headless browser content extraction.', category: 'Native Utilities' },
  { name: 'WebActionBridge', rigName: 'web_action', description: 'Delegates to the native WebAutomation engine for click, type, screenshot, and wait_for actions.', category: 'Native Utilities' },
  { name: 'WebLoginBridge', rigName: 'web_login', description: 'Delegates to the native WebAutomation engine for automated form-based authentication.', category: 'Native Utilities' },
];

export const reconPhases = [
  { phase: 0, name: 'Organizational Mapping', description: 'WHOIS, corporate registry, subsidiary detection, org chart intelligence' },
  { phase: 1, name: 'DNS & Domain Intelligence', description: 'Zone transfers, subdomain brute-force, DNS record analysis, registrar intelligence' },
  { phase: 2, name: 'Network Perimeter Mapping', description: 'Port scanning, service enumeration, firewall detection, CDN/WAF fingerprinting' },
  { phase: 3, name: 'Web Application Fingerprinting', description: 'Technology stack detection, CMS identification, framework analysis' },
  { phase: 4, name: 'Authentication & Session Analysis', description: 'Login flow analysis, session token inspection, auth bypass reconnaissance' },
  { phase: 5, name: 'Content & Resource Discovery', description: 'Directory brute-forcing, hidden endpoint detection, backup file hunting' },
  { phase: 6, name: 'Cloud & Infrastructure Intelligence', description: 'Cloud provider detection, S3 bucket enumeration, serverless function mapping' },
  { phase: 7, name: 'API & Service Analysis', description: 'REST/GraphQL endpoint discovery, API schema extraction, version detection' },
  { phase: 8, name: 'Vulnerability Assessment', description: 'Known CVE matching, misconfig detection, injection point identification' },
  { phase: 9, name: 'Data Leakage & OSINT', description: 'Credential leak databases, paste sites, code repositories, social media' },
  { phase: 10, name: 'Supply Chain Analysis', description: 'Third-party dependency audit, CDN analysis, embedded resource tracking' },
  { phase: 11, name: 'Continuous Monitoring Setup', description: 'Change detection, certificate expiry monitoring, DNS change alerts' },
  { phase: 12, name: 'Reporting & Synthesis', description: 'Executive summary generation, finding correlation, risk scoring' },
];
