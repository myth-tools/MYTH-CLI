import { CodeBlock, PageHeader } from "../components/Layout";
import SystemGraph from "../components/SystemGraph";

const coreModules = [
	{
		module: "src/main.rs",
		desc: "Entry point — parses CLI args via clap, loads two-tier YAML config, initialises telemetry, and dispatches to the command handler.",
	},
	{
		module: "src/cli.rs",
		desc: "27 CLI subcommands (scan, subdomains, master, stealth, osint, vuln, tools, chat, config, profile, check, mcp, vitals, findings, graph, history, report, sync, burn, wipe, clear, depth, inspect, usage, typography, completions, version) with full argument parsing.",
	},
	{
		module: "src/core/agent.rs",
		desc: "ReconAgent — the main AI orchestrator. Builds Rig.rs agents with 16 tool bridges, manages conversation history, handles streaming chat in both TUI and CLI modes, and drives the ReconGraph state machine.",
	},
	{
		module: "src/core/commands.rs",
		desc: "30+ tactical interactive commands with fuzzy matching (Jaro-Winkler), semantic tokenization, ghost-suggestion hints, and dual slash/no-slash dispatch.",
	},
	{
		module: "src/core/session.rs",
		desc: "Session lifecycle management — creates volatile tmpfs workspaces, persists mission metadata, and handles graceful cleanup on exit.",
	},
	{
		module: "src/core/recon_graph.rs",
		desc: "ReconGraph state machine — directed petgraph tracking 13 phases, 89 atomic steps, findings with severity, targets, and inter-asset relationships.",
	},
	{
		module: "src/llm/",
		desc: "NVIDIA NIM client (OpenAI-compatible), system/session prompts, streaming response consumer, 16 Rig.rs tool bridge implementations, and markdown terminal renderer.",
	},
	{
		module: "src/mcp/",
		desc: "MCP server (discover/execute/help over stdio), client manager with differential factory-defaults sync, tool discovery scanner, JSON-RPC schema types.",
	},
	{
		module: "src/sandbox/",
		desc: "Bubblewrap sandbox with security policy (50+ blocked command patterns), per-command namespace isolation, output caps (2MB stdout / 512KB stderr), and timeout enforcement.",
	},
	{
		module: "src/memory/",
		desc: "Qdrant in-memory vector store with NVIDIA NIM embeddings (1024-dim NV-Embed-QA). Auto-stores all tool outputs/findings. Cosine similarity semantic recall injected into LLM context.",
	},
	{
		module: "src/config/",
		desc: "Two-tier YAML config (agent.yaml compiled-in + user.yaml user-owned) with hot-reload filesystem watcher, validation, and factory-defaults sync for mcp.json.",
	},
	{
		module: "src/builtin_tools/",
		desc: "19 native Rust utility tools executed without sandbox: file generation, AES encryption, compression, web automation (Lightpanda/headless Chrome/WebDriver), payload generation, memory search, stealth recon, and more.",
	},
	{
		module: "src/builtin_mcp/",
		desc: "12 factory-default MCP server definitions — local (stdio), remote (SSE), and custom security-focused servers bundled with MYTH.",
	},
	{
		module: "src/tui/",
		desc: "ratatui TUI with multi-panel layout (chat / tool-execution visualization / recon-graph / findings), real-time streaming renderer, and keyboard event loop.",
	},
	{
		module: "src/interactive.rs",
		desc: "Interactive mode router — dual-mode dispatcher between the full TUI and the simpler CLI (--no-tui) readline loop, sharing the same command engine.",
	},
];

const dataFlowSteps = [
	"User enters a command or natural-language message via CLI or TUI",
	"ReconAgent adds the message to conversation history",
	"Rig.rs agent is built with 16 tool bridges attached, context injected from Qdrant",
	"LLM (NVIDIA NIM — DeepSeek R1 / LLaMA 3.1 70B) decides which tools to call",
	"Tool bridges dispatch to: local binaries via Bubblewrap sandbox, external MCP servers (stdio/SSE), or native Rust built-in tools",
	"Tool stdout/stderr are streamed back via dual-mode telemetry (TUI events or CLI output)",
	"Every tool output is automatically embedded (NV-Embed-QA) and stored in Qdrant for semantic recall",
	"ReconGraph is updated — phase progress, new findings, severity classification",
	"LLM synthesizes all results and streams the response back to the user",
	"Session ends → all RAM/tmpfs data destroyed, zero disk trace remains",
];

export default function ArchitecturePage() {
	return (
		<div>
			<PageHeader
				title="Architecture"
				description="How MYTH works internally — from CLI input to sandboxed tool execution to AI synthesis. A 5-layer system: CLI → Neural Core → LLM → Tool Bridges → Sandbox."
				badge="Architecture"
			/>

			{/* Interactive Graph */}
			<div className="mb-12">
				<h2 className="text-xl font-bold text-white mb-4 flex items-center gap-2">
					<span className="text-cyber-primary">01.</span> Interactive System Map
				</h2>
				<p className="text-sm text-cyber-text/80 mb-4">
					Click and drag nodes. Use the fullscreen button for the full interactive neural map.
				</p>
				<SystemGraph />
			</div>

			{/* Core Modules */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">02.</span> Core Modules
			</h2>
			<div className="space-y-2 mb-10">
				{coreModules.map((m) => (
					<div
						key={m.module}
						className="flex gap-4 items-start p-4 rounded-lg border border-cyber-border hover:border-cyber-primary/30 transition-colors group"
					>
						<code className="text-cyber-primary text-xs font-mono break-all shrink-0 mt-0.5 min-w-[160px]">
							{m.module}
						</code>
						<p className="text-sm text-cyber-text/80 leading-relaxed">{m.desc}</p>
					</div>
				))}
			</div>

			{/* Data Flow */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">03.</span> Request Data Flow
			</h2>
			<ol className="space-y-3 mb-10">
				{dataFlowSteps.map((step, i) => (
					<li key={step} className="flex gap-3 items-start">
						<span className="text-cyber-primary font-mono text-sm shrink-0 w-6 text-right">
							{i + 1}.
						</span>
						<span className="text-sm text-cyber-text/80 leading-relaxed">{step}</span>
					</li>
				))}
			</ol>

			{/* Project Structure */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">04.</span> Project Structure
			</h2>
			<CodeBlock
				lang="text"
				title="Directory layout"
				code={`MYTH_CLI/
├── Cargo.toml                  # Rust dependencies & .deb packaging metadata
├── config/
│   ├── agent.yaml              # Internal defaults (compiled into binary)
│   ├── user.yaml               # User overrides (API keys, model, sandbox)
│   └── mcp.json                # MCP server registry (factory defaults + custom)
├── src/
│   ├── main.rs                 # Entry point & global CLI dispatch
│   ├── cli.rs                  # 27 CLI subcommand definitions (clap)
│   ├── interactive.rs          # Dual-mode interactive router (TUI / CLI)
│   ├── stream.rs               # Streaming LLM response consumer
│   ├── markdown_renderer.rs    # Terminal markdown renderer (colored output)
│   ├── config/                 # YAML config parsing, validation & hot-reload
│   ├── core/                   # Agent brain, commands, session, ReconGraph
│   ├── llm/                    # NIM client, prompts, 16 tool bridges
│   ├── mcp/                    # MCP server, client manager, discovery
│   ├── sandbox/                # Bubblewrap isolation & security policy
│   ├── memory/                 # Qdrant vector store & NIM embeddings
│   ├── builtin_tools/          # 19 native Rust utility tools
│   ├── builtin_mcp/            # 12 factory MCP server definitions
│   ├── tui/                    # ratatui terminal UI
│   └── ui/                     # CLI themes, colors & formatting
├── scripts/                    # 17 automation scripts (install, release, etc.)
├── completions/                # Shell completions (bash, zsh, fish)
├── linux/                      # Universal Linux packaging control files
├── package_runners/            # NPM, PyPI, Docker, Snap, Nix runners
├── docs/                       # Documentation website (React/Vite)
└── README.md`}
			/>

			{/* Key Design Decisions */}
			<h2 className="text-xl font-bold text-white mb-4 mt-8">
				<span className="text-cyber-primary">05.</span> Key Design Decisions
			</h2>
			<div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-8">
				{[
					{
						title: "Static Binary",
						desc: "MYTH ships as a fully static, stripped Rust binary (~8MB). Zero runtime dependencies — no Python, Node, or interpreter required on the target system.",
					},
					{
						title: "Volatile by Default",
						desc: "All mission data lives in RAM (tmpfs). When the session ends, everything is destroyed. No forensic trace. myth burn triggers instant SIGKILL cleanup.",
					},
					{
						title: "Zero-Trust Sandbox",
						desc: "Every tool call goes through Bubblewrap namespace isolation. The host filesystem is read-only. Processes cannot see each other. TIOCSTI injection is blocked.",
					},
					{
						title: "Open Protocol — MCP",
						desc: "The Model Context Protocol (MCP) makes MYTH infinitely extensible. Any stdio or SSE server can be added at runtime without recompiling or restarting.",
					},
				].map((d) => (
					<div key={d.title} className="feature-card rounded-xl p-5">
						<h3 className="font-bold text-cyber-primary text-sm mb-2 uppercase tracking-wider">
							{d.title}
						</h3>
						<p className="text-xs text-cyber-dim leading-relaxed">{d.desc}</p>
					</div>
				))}
			</div>
		</div>
	);
}
