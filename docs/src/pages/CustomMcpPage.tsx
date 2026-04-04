import { CodeBlock, PageHeader } from "../components/Layout";

const subcommands = [
	{
		cmd: "myth mcp list",
		slash: "/mcp list",
		desc: "List all configured MCP servers, their transport type, enabled status, and process ID.",
	},
	{
		cmd: "myth mcp toggle <name> on|off",
		slash: "/mcp toggle <name> on",
		desc: "Enable or disable a specific server. The agent instantly reconnects or disconnects.",
	},
	{
		cmd: "myth mcp tools <name>",
		slash: "/mcp tools <name>",
		desc: "Discover all tools exposed by a specific server via MCP tool discovery.",
	},
	{
		cmd: "myth mcp add-local <name> <cmd> [args] [env:KEY=VAL] [desc:text]",
		slash: "/mcp add-local <name> <cmd> ...",
		desc: "Register a new local stdio MCP server. Environment variables injected as env:KEY=VALUE.",
	},
	{
		cmd: "myth mcp add-remote <name> <url>",
		slash: "/mcp add-remote <name> <url>",
		desc: "Register a remote SSE MCP server endpoint.",
	},
	{
		cmd: "myth mcp remove <name>",
		slash: "/mcp remove <name>",
		desc: "Permanently remove a user-managed MCP server from the registry.",
	},
	{
		cmd: "myth mcp allow-tool <server> <tool>",
		slash: "/mcp allow-tool <server> <tool>",
		desc: "Add a tool to the allowlist for a specific server. The AI can now call it.",
	},
	{
		cmd: "myth mcp block-tool <server> <tool>",
		slash: "/mcp block-tool <server> <tool>",
		desc: "Add a tool to the blocklist. The AI will never call this tool even if discovered.",
	},
];

const realWorldExamples = [
	{
		title: "Add a Self-Hosted Shodan MCP Server",
		code: `# Install Shodan MCP server package
pip install shodan-mcp

# Register with MYTH (pass API key as env var)
myth mcp add-local shodan python -m shodan_mcp env:SHODAN_API_KEY=your_key desc:Shodan_search_engine

# The AI agent can now use Shodan tools autonomously
myth scan target.com`,
	},
	{
		title: "Add a Remote Security Intelligence SSE Server",
		code: `# Register a remote SSE server (no local install needed)
myth mcp add-remote threat-intel https://security-api.example.com/sse

# Toggle on
myth mcp toggle threat-intel on

# Verify it's active
myth mcp list`,
	},
	{
		title: "Add Custom Port Scanner via Docker MCP",
		code: `# Any MCP server can be run via Docker
myth mcp add-local portscan docker run --rm -i \\
  custom-scanner-mcp desc:Custom_port_scanner

# Restrict to only the tools you want
myth mcp allow-tool portscan scan_ports
myth mcp block-tool portscan upload_results`,
	},
	{
		title: "Add Nuclei Template Runner",
		code: `# Add a local nuclei MCP wrapper
myth mcp add-local nuclei npx @custom/nuclei-mcp desc:Nuclei_template_runner

# Verify tools are discoverable
myth mcp tools nuclei

# Now ask the agent to use it
myth scan target.com  # Agent will autonomously call nuclei templates`,
	},
];

export default function CustomMcpPage() {
	return (
		<div>
			<PageHeader
				title="Custom MCP Servers"
				description="Extend MYTH with any MCP-compatible server — local (stdio) or remote (SSE). Stored in ~/.config/myth/mcp.json and hot-reloaded without restarting the agent."
				badge="MCP Ecosystem"
			/>

			{/* What is a custom MCP server */}
			<div className="glass-panel rounded-xl p-5 mb-10 border border-cyber-primary/20">
				<h2 className="text-sm font-bold text-cyber-primary mb-3 uppercase tracking-wider flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
					What is a Custom MCP Server?
				</h2>
				<p className="text-sm text-cyber-text/80 leading-relaxed mb-3">
					Any program that speaks the <strong className="text-white">Model Context Protocol</strong>{" "}
					can be registered as a custom server. This includes: npm packages, Python scripts, Docker
					containers, Go binaries, or any program that reads JSON-RPC from stdin and writes to
					stdout.
				</p>
				<p className="text-sm text-cyber-text/80 leading-relaxed">
					Once registered, the AI agent{" "}
					<strong className="text-white">automatically discovers</strong> all tools exposed by the
					server and can call them autonomously during missions — no manual integration required.
				</p>
			</div>

			{/* Adding a Local Server */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">01.</span> Add a Local Server (stdio)
			</h2>
			<p className="text-sm text-cyber-text/80 mb-3">
				Local servers run as child processes with stdio transport:
			</p>
			<CodeBlock
				lang="bash"
				title="CLI"
				code="myth mcp add-local <name> <cmd> [args...] [env:KEY=VALUE] [desc:your_description]"
			/>
			<CodeBlock
				lang="bash"
				title="Interactive session"
				code="/mcp add-local my-tool npx my-mcp-server -a --verbose env:API_KEY=abc123 desc:My_custom_tool"
			/>

			{/* Syntax breakdown */}
			<h3 className="text-base font-semibold text-white mb-3 mt-6">Syntax Breakdown</h3>
			<div className="table-container mb-10">
				<table className="w-full text-sm docs-table rounded-lg overflow-hidden">
					<thead>
						<tr>
							<th>Token</th>
							<th>Example</th>
							<th>Description</th>
						</tr>
					</thead>
					<tbody>
						{[
							["<name>", "my-tool", "Server identifier — used in all mcp subcommands"],
							["<cmd>", "npx", "The base executable to run"],
							["[args...]", "my-mcp-server -a --verbose", "Arguments passed after the command"],
							[
								"env:KEY=VALUE",
								"env:API_KEY=abc123",
								"Inject env vars into the server process (repeatable)",
							],
							[
								"desc:text",
								"desc:My_custom_tool",
								"Description shown in mcp list (underscores = spaces)",
							],
						].map(([token, example, desc]) => (
							<tr key={token as string}>
								<td>
									<code className="text-cyber-primary text-xs">{token}</code>
								</td>
								<td className="text-cyber-dim text-xs font-mono">{example}</td>
								<td className="text-cyber-text/70 text-xs">{desc}</td>
							</tr>
						))}
					</tbody>
				</table>
			</div>

			{/* Adding a Remote Server */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">02.</span> Add a Remote Server (SSE)
			</h2>
			<p className="text-sm text-cyber-text/80 mb-3">
				Remote servers connect via Server-Sent Events (SSE). No local process spawned:
			</p>
			<CodeBlock lang="bash" title="CLI" code="myth mcp add-remote <name> <url>" />
			<CodeBlock
				lang="bash"
				title="Interactive session"
				code="/mcp add-remote my-api https://api.example.com/sse"
			/>

			{/* Full Subcommand Reference */}
			<h2 className="text-xl font-bold text-white mb-4 mt-10">
				<span className="text-cyber-primary">03.</span> Full MCP Subcommand Reference
			</h2>
			<div className="space-y-3 mb-10">
				{subcommands.map((s) => (
					<div key={s.cmd} className="feature-card rounded-xl p-4">
						<div className="flex flex-wrap items-center gap-2 mb-2">
							<code className="text-cyber-primary text-xs font-mono font-bold">{s.cmd}</code>
							<code className="text-cyber-dim text-xs font-mono opacity-60">or {s.slash}</code>
						</div>
						<p className="text-xs text-cyber-text/70 leading-relaxed">{s.desc}</p>
					</div>
				))}
			</div>

			{/* mcp.json Schema */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">04.</span> mcp.json Schema
			</h2>
			<CodeBlock
				lang="json"
				title="~/.config/myth/mcp.json"
				code={`{
  "mcpServers": {
    "my-local-tool": {
      "command": "npx",
      "args": ["my-mcp-server", "-a", "--verbose"],
      "env": { "API_KEY": "abc123" },
      "description": "My custom local tool",
      "enabled": true,
      "is_user_managed": true,    // preserved during myth sync
      "allowed_tools": ["tool_a", "tool_b"],
      "blocked_tools": ["tool_c"]
    },
    "my-remote-api": {
      "url": "https://api.example.com/sse",
      "transport": "sse",
      "description": "Remote security API",
      "enabled": true,
      "is_user_managed": true
    }
  }
}`}
			/>
			<p className="text-xs text-cyber-dim mt-2 italic mb-10">
				💡 <code className="text-cyber-primary">is_user_managed: true</code> prevents{" "}
				<code className="text-cyber-primary">myth sync</code> from overwriting your custom servers
				when applying factory defaults.
			</p>

			{/* Real-World Examples */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">05.</span> Real-World Integration Examples
			</h2>
			<div className="space-y-6">
				{realWorldExamples.map((ex) => (
					<div key={ex.title} className="feature-card rounded-xl p-5">
						<h3 className="font-bold text-white text-sm mb-3">{ex.title}</h3>
						<CodeBlock lang="bash" code={ex.code} />
					</div>
				))}
			</div>
		</div>
	);
}
