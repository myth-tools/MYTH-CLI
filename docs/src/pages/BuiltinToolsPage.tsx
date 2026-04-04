import { useState } from "react";
import { CodeBlock, PageHeader } from "../components/Layout";
import { builtinTools } from "../data/content";

const categoryColors: Record<string, string> = {
	Utility: "bg-cyber-secondary/10 text-cyber-secondary border-cyber-secondary/30",
	Security: "bg-cyber-error/10 text-cyber-error border-cyber-error/30",
	Web: "bg-blue-500/10 text-blue-300 border-blue-500/30",
	Memory: "bg-purple-500/10 text-purple-300 border-purple-500/30",
	Mission: "bg-cyber-primary/10 text-cyber-primary border-cyber-primary/30",
	Recon: "bg-cyber-warning/10 text-cyber-warning border-cyber-warning/30",
};

const categories = ["All", ...new Set(builtinTools.map((t) => t.category))];

const exampleCalls: Record<string, string> = {
	generate_file: `{ "tool": "generate_file", "args": { "path": "report.md", "content": "# Scan Results\\n..." } }`,
	search_memory: `{ "tool": "search_memory", "args": { "query": "open ports on target", "limit": 5 } }`,
	subdomain_fetch: `{ "tool": "subdomain_fetch", "args": { "domain": "example.com" } }`,
	browse: `{ "tool": "browse", "args": { "url": "https://target.com/login", "session_name": "auth_session" } }`,
	generate_secure_asset: `{ "tool": "generate_secure_asset", "args": { "path": "exfil.enc", "content": "sensitive", "key": "abcdef0123..." } }`,
};

export default function BuiltinToolsPage() {
	const [activeCategory, setActiveCategory] = useState("All");
	const [searchQuery, setSearchQuery] = useState("");

	const filtered = builtinTools.filter((t) => {
		const matchCat = activeCategory === "All" || t.category === activeCategory;
		const term = searchQuery.toLowerCase().trim();
		const matchSearch =
			!term || t.name.toLowerCase().includes(term) || t.description.toLowerCase().includes(term);
		return matchCat && matchSearch;
	});

	return (
		<div>
			<PageHeader
				title="Built-in Tools"
				description="19 native Rust utility tools that execute directly without the Bubblewrap sandbox. These provide essential mission capabilities — file operations, cryptographic security, web automation, semantic memory access, and elite subdomain discovery."
				badge="MCP Ecosystem"
			/>

			{/* Why Built-In vs MCP */}
			<div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-10">
				<div className="glass-panel rounded-xl p-5 border border-cyber-primary/20">
					<h3 className="text-sm font-bold text-cyber-primary mb-2 uppercase tracking-wider">
						Built-in Tools
					</h3>
					<ul className="space-y-1.5 text-xs text-cyber-text/80">
						<li className="flex items-start gap-2">
							<span className="text-cyber-primary mt-0.5">✓</span> Native Rust — zero subprocess
							overhead
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-primary mt-0.5">✓</span> Run OUTSIDE the sandbox (direct
							host access)
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-primary mt-0.5">✓</span> Always available — no
							installation required
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-primary mt-0.5">✓</span> AES-256-GCM, Zstd, mmap —
							enterprise-grade
						</li>
					</ul>
				</div>
				<div className="glass-panel rounded-xl p-5 border border-cyber-border/30">
					<h3 className="text-sm font-bold text-cyber-dim mb-2 uppercase tracking-wider">
						MCP External Tools (comparison)
					</h3>
					<ul className="space-y-1.5 text-xs text-cyber-text/60">
						<li className="flex items-start gap-2">
							<span className="text-cyber-dim mt-0.5">→</span> Run via stdio/SSE protocol
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-dim mt-0.5">→</span> Sandboxed inside Bubblewrap namespace
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-dim mt-0.5">→</span> Require separate installation
						</li>
						<li className="flex items-start gap-2">
							<span className="text-cyber-dim mt-0.5">→</span> Provide access to 3,000+ Kali
							binaries
						</li>
					</ul>
				</div>
			</div>

			{/* Category + Search filters */}
			<div className="flex flex-wrap items-center gap-3 mb-6">
				<div className="flex flex-wrap gap-2">
					{categories.map((cat) => (
						<button
							key={cat}
							type="button"
							onClick={() => setActiveCategory(cat)}
							className={`text-[11px] px-3 py-1.5 rounded-lg border font-mono font-bold uppercase tracking-wider transition-all ${
								activeCategory === cat
									? "bg-cyber-primary text-cyber-bg border-cyber-primary"
									: "bg-white/5 text-cyber-dim border-cyber-border hover:border-cyber-primary/40"
							}`}
						>
							{cat}
						</button>
					))}
				</div>
				<div className="ml-auto relative">
					<input
						id="builtin-tools-search"
						name="builtin-tools-query"
						aria-label="Search builtin tools"
						type="text"
						placeholder="Search tools..."
						value={searchQuery}
						onChange={(e) => setSearchQuery(e.target.value)}
						className="pl-3 pr-20 py-1.5 text-xs bg-cyber-bg border border-cyber-border rounded-lg text-white placeholder-cyber-dim focus:outline-none focus:border-cyber-primary transition-all"
					/>
					<span className="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] font-mono text-cyber-primary/60">
						{filtered.length} tools
					</span>
				</div>
			</div>

			<div className="space-y-6">
				{filtered.map((tool) => (
					<div key={tool.name} id={tool.name} className="feature-card rounded-xl p-5 scroll-mt-24">
						<div className="flex items-center gap-3 mb-2 flex-wrap">
							<h3 className="font-bold text-white font-mono text-base">{tool.name}</h3>
							<span
								className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold uppercase tracking-wider ${categoryColors[tool.category] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
							>
								{tool.category}
							</span>
						</div>
						<p className="text-sm text-cyber-text/80 mb-4 leading-relaxed">{tool.description}</p>

						{tool.parameters.length > 0 ? (
							<div className="table-container mb-4">
								<table className="w-full text-xs docs-table rounded-lg overflow-hidden">
									<thead>
										<tr>
											<th>Parameter</th>
											<th>Type</th>
											<th>Required</th>
											<th>Description</th>
										</tr>
									</thead>
									<tbody>
										{tool.parameters.map((p) => (
											<tr key={p.name}>
												<td>
													<code className="text-cyber-primary text-xs">{p.name}</code>
												</td>
												<td className="text-cyber-dim text-xs font-mono">{p.type}</td>
												<td>
													{p.required ? (
														<span className="text-cyber-error text-xs font-bold">YES</span>
													) : (
														<span className="text-cyber-dim text-xs opacity-40">no</span>
													)}
												</td>
												<td className="text-cyber-text/70 text-xs">{p.description}</td>
											</tr>
										))}
									</tbody>
								</table>
							</div>
						) : (
							<p className="text-xs text-cyber-dim italic mb-4">No parameters required.</p>
						)}

						{exampleCalls[tool.name] && (
							<CodeBlock lang="json" title="Example LLM Tool Call" code={exampleCalls[tool.name]} />
						)}
					</div>
				))}
			</div>
		</div>
	);
}
