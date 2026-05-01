import { CodeBlock, PageHeader } from "../components/Layout";
import MemoryGraph from "../components/MemoryGraph";

const specRows = [
	{ prop: "Backend", value: "Qdrant (embedded in-process)" },
	{ prop: "Mode", value: "in-memory (volatile)" },
	{ prop: "Embedding Model", value: "NV-Embed-QA via NVIDIA NIM" },
	{ prop: "Vector Dimensions", value: "1024" },
	{ prop: "Distance Metric", value: "Cosine Similarity" },
	{ prop: "Max Entries per Session", value: "Unlimited (RAM-bound)" },
	{ prop: "Auto-store on Tool Exec", value: "Yes — every stdout/stderr" },
	{ prop: "Search Latency", value: "< 5ms (in-process)" },
	{ prop: "GRPC Port", value: "6334 (internal only)" },
	{ prop: "HTTP Port", value: "6333 (internal only)" },
	{ prop: "Persistence to Disk", value: "Never" },
	{ prop: "Destruction Trigger", value: "Session exit / myth burn" },
];

const entryTypes = [
	{
		type: "ToolOutput",
		trigger: "Automatic — every sandbox tool execution",
		content: "stdout, stderr, exit code, tool name, timestamp",
		example: "nmap scan results, gobuster paths, nikto output",
	},
	{
		type: "Finding",
		trigger: "Explicit — agent calls report_finding bridge",
		content: "severity, title, description, target, evidence",
		example: "SQLi vulnerability found, open S3 bucket, exposed admin panel",
	},
	{
		type: "UserInput",
		trigger: "Automatic — every user message",
		content: "raw message text, timestamp",
		example: "Scan example.com for SQL injection",
	},
	{
		type: "AgentResponse",
		trigger: "Automatic — every LLM response",
		content: "synthesis text, tool calls made, conclusions",
		example: "Analysis: found 3 open ports, 1 critical CVE...",
	},
];

export default function MemoryPage() {
	return (
		<div>
			<PageHeader
				title="Memory System"
				description="MYTH's autonomous semantic memory — a Qdrant in-memory vector database powered by NVIDIA NIM embeddings. Every tool output is auto-stored and semantically searchable. Zero persistence. Destroyed on exit."
				badge="Architecture"
			/>

			{/* Overview */}
			<div className="glass-panel rounded-xl p-6 mb-10 border border-cyber-primary/20">
				<h2 className="text-base font-bold text-cyber-primary mb-3 uppercase tracking-wider flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
					How It Works
				</h2>
				<p className="text-sm text-cyber-text/80 leading-relaxed mb-3">
					Every time the AI agent executes a tool, MYTH automatically embeds the output using the{" "}
					<strong className="text-white">NV-Embed-QA</strong> model via NVIDIA NIM (1024-dimensional
					vectors) and stores it in a local Qdrant instance running entirely in RAM.
				</p>
				<p className="text-sm text-cyber-text/80 leading-relaxed">
					Before each new LLM prompt, MYTH performs a semantic similarity search against the session
					history and injects the top-k most relevant results directly into the context window. This
					gives the agent perfect recall of past findings — even across hundreds of tool calls —
					without ever exceeding the context limit.
				</p>
			</div>

			{/* Interactive Graph */}
			<div className="mb-12">
				<h2 className="text-xl font-bold text-white mb-4 flex items-center gap-2">
					<span className="text-cyber-primary">01.</span> Interactive Memory Architecture
				</h2>
				<MemoryGraph />
			</div>

			{/* Technical Specs */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">02.</span> Technical Specifications
			</h2>
			<div className="table-container mb-10">
				<table className="w-full text-xs docs-table">
					<thead>
						<tr>
							<th className="text-left py-3 px-4">Property</th>
							<th className="text-left py-3 px-4">Value</th>
						</tr>
					</thead>
					<tbody className="divide-y divide-cyber-border/30">
						{specRows.map((r) => (
							<tr key={r.prop} className="hover:bg-white/[0.02] transition-colors">
								<td className="py-3 px-4 font-mono text-cyber-primary font-bold text-xs">
									{r.prop}
								</td>
								<td className="py-3 px-4 text-cyber-text/80 text-xs font-mono">{r.value}</td>
							</tr>
						))}
					</tbody>
				</table>
			</div>

			{/* Entry Types */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">03.</span> Memory Entry Types
			</h2>
			<div className="space-y-4 mb-10">
				{entryTypes.map((e) => (
					<div key={e.type} className="feature-card rounded-xl p-5">
						<div className="flex flex-wrap items-center gap-3 mb-2">
							<span className="font-bold text-white font-mono">{e.type}</span>
							<span className="text-[10px] px-2 py-0.5 bg-cyber-secondary/10 text-cyber-secondary border border-cyber-secondary/30 rounded font-mono">
								{e.trigger}
							</span>
						</div>
						<p className="text-xs text-cyber-dim mb-1">
							<span className="text-cyber-text/60 font-bold">Content: </span>
							{e.content}
						</p>
						<p className="text-xs text-cyber-dim italic">
							<span className="text-cyber-primary/60">Example: </span>
							{e.example}
						</p>
					</div>
				))}
			</div>

			{/* Context Injection */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">04.</span> Context Injection Flow
			</h2>
			<div className="space-y-3 mb-8">
				{[
					"User sends message or tool returns output",
					"MYTH embeds the text with NV-Embed-QA (1024-dim vector) via NVIDIA NIM",
					"Vector is upserted into Qdrant in-memory collection",
					"Before next LLM prompt: cosine similarity search retrieves top-5 relevant entries",
					"Retrieved entries are prepended to the system prompt as [RECALLED CONTEXT]",
					"LLM receives full historic context without re-processing the entire chat history",
				].map((step, i) => (
					<div key={step} className="flex items-start gap-3">
						<span className="text-cyber-primary font-mono text-sm shrink-0 w-6 text-right">
							{i + 1}.
						</span>
						<span className="text-sm text-cyber-text/80 leading-relaxed">{step}</span>
					</div>
				))}
			</div>

			{/* Search API */}
			<h2 className="text-xl font-bold text-white mb-4">
				<span className="text-cyber-primary">05.</span> Semantic Search API
			</h2>
			<p className="text-sm text-cyber-text/80 mb-4">
				The agent can explicitly search memory via the{" "}
				<code className="text-cyber-primary">search_memory</code> tool bridge:
			</p>
			<CodeBlock
				lang="json"
				title="Tool call — semantic memory search"
				code={`{
  "tool": "search_memory",
  "args": {
    "query": "open ports found on 192.168.1.1",
    "limit": 5
  }
}

// Returns: top-5 cosine-similar entries with metadata
// [ { "type": "ToolOutput", "score": 0.97, "content": "nmap: 22/tcp open ssh..." }, ... ]`}
			/>

			{/* Config */}
			<h2 className="text-xl font-bold text-white mb-4 mt-8">
				<span className="text-cyber-primary">06.</span> Configuration
			</h2>
			<CodeBlock
				lang="yaml"
				title="~/.config/myth/user.yaml — memory section"
				code={`memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"      # No disk writes — ever
  grpc_port: 6334
  http_port: 6333
  collection_name: "agent_session"
  vector_size: 1024      # NV-Embed-QA output dimension
  auto_start: true       # Auto-launch Qdrant on agent start`}
			/>

			{/* Volatile guarantee */}
			<div className="mt-6 glass-panel rounded-xl p-5 border border-cyber-primary/20">
				<h3 className="text-sm font-bold text-cyber-primary mb-2 flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary" />
					Zero-Trace Guarantee
				</h3>
				<p className="text-xs text-cyber-text/80 leading-relaxed">
					The Qdrant instance is initialized at session start with{" "}
					<code className="text-cyber-primary">mode: in-memory</code>. All vectors and payload data
					live exclusively in RAM. When the MYTH session ends (normal exit,{" "}
					<code className="text-cyber-primary">myth burn</code>, or SIGKILL), the entire in-memory
					store is destroyed by the OS. No indexes, no WAL files, no residual data on disk. This is
					a non-negotiable security property.
				</p>
			</div>
		</div>
	);
}
