import { CodeBlock, PageHeader } from "../components/Layout";
import { toolBridges } from "../data/content";

const categoryColors: Record<string, string> = {
	"Core Execution": "bg-cyber-primary/10 text-cyber-primary border-cyber-primary/30",
	Discovery: "bg-cyber-secondary/10 text-cyber-secondary border-cyber-secondary/30",
	"Mission Control": "bg-cyber-warning/10 text-cyber-warning border-cyber-warning/30",
	Memory: "bg-purple-500/10 text-purple-300 border-purple-500/30",
	"MCP Protocol": "bg-blue-500/10 text-blue-300 border-blue-500/30",
	"Native Utilities": "bg-cyber-success/10 text-cyber-success border-cyber-success/30",
};

const toolCallExamples: Record<string, { desc: string; json: string }> = {
	execute_tool: {
		desc: "Execute nmap in the Bubblewrap sandbox",
		json: `{
  "tool": "execute_tool",
  "args": {
    "tool_name": "nmap",
    "args": ["-sV", "-p", "80,443,8080", "example.com"],
    "timeout": 60
  }
}`,
	},
	execute_batch: {
		desc: "Run nmap + gobuster in parallel (Swarm Mode)",
		json: `{
  "tool": "execute_batch",
  "args": {
    "commands": [
      { "tool": "nmap", "args": ["-sV", "example.com"] },
      { "tool": "gobuster", "args": ["dir", "-u", "https://example.com", "-w", "/usr/share/wordlists/common.txt"] }
    ]
  }
}`,
	},
	report_finding: {
		desc: "Register a critical vulnerability into ReconGraph",
		json: `{
  "tool": "report_finding",
  "args": {
    "title": "SQL Injection in /login",
    "severity": "critical",
    "description": "login endpoint is vulnerable to UNION-based SQLi",
    "target": "https://example.com/login",
    "evidence": "' OR 1=1-- returned 200 with admin data"
  }
}`,
	},
	search_memory: {
		desc: "Recall past tool outputs with semantic search",
		json: `{
  "tool": "search_memory",
  "args": {
    "query": "open ports found on target",
    "limit": 5
  }
}`,
	},
};

const categories = [...new Set(toolBridges.map((b) => b.category))];

export default function ToolBridgesPage() {
	return (
		<div>
			<PageHeader
				title="LLM Tool Bridges"
				description="16 Rig.rs Tool implementations that connect the NVIDIA NIM AI agent to MYTH's execution layer. The LLM autonomously decides which bridges to call based on mission requirements."
				badge="MCP Ecosystem"
			/>

			{/* How They Work */}
			<div className="glass-panel rounded-xl p-6 mb-10 border border-cyber-primary/20">
				<h2 className="text-base font-bold text-cyber-primary mb-4 uppercase tracking-wider flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
					How the Rig.rs Tool Trait Works
				</h2>
				<div className="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-4">
					{[
						{
							step: "1. definition()",
							desc: "Returns a JSON Schema to the LLM describing input parameters. The LLM uses this schema to know how to call the tool correctly.",
						},
						{
							step: "2. call(args)",
							desc: "Executes the bridge logic — dispatching to sandbox, MCP server, or native Rust engine — and returns structured results.",
						},
						{
							step: "3. telemetry",
							desc: "Dual-mode output: TUI events stream to the panel in real-time, CLI mode prints directly to stdout. All outputs auto-stored in semantic memory.",
						},
					].map((s) => (
						<div key={s.step} className="bg-white/5 rounded-lg p-4 border border-white/10">
							<p className="text-xs font-bold text-cyber-primary font-mono mb-2">{s.step}</p>
							<p className="text-xs text-cyber-dim leading-relaxed">{s.desc}</p>
						</div>
					))}
				</div>
				<CodeBlock
					lang="rust"
					title="Simplified Rig.rs Tool trait (conceptual)"
					code={`// Each tool bridge implements this trait
trait Tool {
    fn definition(&self) -> serde_json::Value; // JSON Schema for LLM
    async fn call(&self, args: Value) -> Result<String>; // execution logic
}

// The ReconAgent is built with all 16 bridges attached
let agent = openai_client
    .agent(model)
    .tool(ExecuteToolBridge::new(...))
    .tool(ExecuteBatchBridge::new(...))
    .tool(SearchMemoryBridge::new(...))
    // ... 13 more
    .build();`}
				/>
			</div>

			{/* Tool Call Examples */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">01.</span> LLM Tool Call Examples
			</h2>
			<div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-10">
				{Object.entries(toolCallExamples).map(([name, ex]) => (
					<div key={name} className="feature-card rounded-xl p-4">
						<p className="text-[10px] font-mono text-cyber-dim uppercase tracking-wider mb-1">
							{ex.desc}
						</p>
						<CodeBlock lang="json" title={`myth → LLM → ${name}`} code={ex.json} />
					</div>
				))}
			</div>

			{/* Bridge Registry */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">02.</span> Bridge Registry
			</h2>
			{categories.map((cat) => (
				<div key={cat} className="mb-8">
					<div className="flex items-center gap-3 mb-4">
						<h3 className="text-base font-bold text-white uppercase tracking-wider">{cat}</h3>
						<span
							className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold ${categoryColors[cat] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
						>
							{toolBridges.filter((b) => b.category === cat).length} bridges
						</span>
						<div className="h-px flex-1 bg-gradient-to-r from-cyber-border/50 to-transparent" />
					</div>
					<div className="space-y-3">
						{toolBridges
							.filter((b) => b.category === cat)
							.map((bridge) => (
								<div
									key={bridge.name}
									className="feature-card rounded-xl p-5 flex flex-col sm:flex-row sm:items-start gap-4"
								>
									<div className="shrink-0 min-w-[200px]">
										<code className="text-sm font-bold text-white font-mono block mb-1">
											{bridge.rigName}
										</code>
										<span className="text-[10px] text-cyber-dim font-mono opacity-60">
											struct: {bridge.name}
										</span>
									</div>
									<p className="text-sm text-cyber-text/70 leading-relaxed">{bridge.description}</p>
									<span
										className={`shrink-0 text-[9px] px-2 py-0.5 rounded border font-mono font-bold self-start ${categoryColors[bridge.category] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
									>
										{bridge.category}
									</span>
								</div>
							))}
					</div>
				</div>
			))}
		</div>
	);
}
