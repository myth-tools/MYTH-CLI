import { useState } from "react";
import { CodeBlock, PageHeader } from "../components/Layout";
import { builtinMcpServers } from "../data/content";

const typeColors: Record<string, string> = {
	local: "bg-cyber-primary/10 text-cyber-primary border-cyber-primary/30",
	remote: "bg-cyber-secondary/10 text-cyber-secondary border-cyber-secondary/30",
	custom: "bg-cyber-warning/10 text-cyber-warning border-cyber-warning/30",
};

const transportColors: Record<string, string> = {
	stdio: "bg-cyber-success/10 text-cyber-success border-cyber-success/30",
	sse: "bg-blue-400/10 text-blue-300 border-blue-400/30",
};

const types = ["All", "local", "remote", "custom"];

export default function McpServersPage() {
	const [activeType, setActiveType] = useState("All");

	const filtered = builtinMcpServers.filter((s) => activeType === "All" || s.type === activeType);

	return (
		<div>
			<PageHeader
				title="Built-in MCP Servers"
				description={`${builtinMcpServers.length} factory-default MCP servers bundled with MYTH. Managed via \`myth sync\` and configured in ~/.config/myth/mcp.json. Mix of local (stdio) and remote (SSE) servers for maximum coverage.`}
				badge="MCP Ecosystem"
			/>

			{/* Factory-Default Sync */}
			<div className="glass-panel rounded-xl p-5 mb-8 border border-cyber-primary/20">
				<h2 className="text-sm font-bold text-cyber-primary mb-3 uppercase tracking-wider flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
					Factory-Default Synchronization
				</h2>
				<p className="text-sm text-cyber-text/80 mb-4 leading-relaxed">
					On first run, MYTH performs a <strong className="text-white">differential sync</strong> —
					comparing the bundled factory MCP registry against the user's{" "}
					<code className="text-cyber-primary">~/.config/myth/mcp.json</code>. New factory defaults
					are added; user customizations are preserved. Run{" "}
					<code className="text-cyber-primary">myth sync</code> at any time to force re-sync.
				</p>
				<div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
					<CodeBlock lang="bash" title="List servers" code="myth mcp list" />
					<CodeBlock lang="bash" title="Enable/disable" code="myth mcp toggle lightpanda off" />
					<CodeBlock lang="bash" title="Force sync" code="myth sync" />
				</div>
			</div>

			{/* Type legend + filter */}
			<div className="flex flex-wrap items-center gap-3 mb-6">
				<div className="flex flex-wrap gap-2">
					{types.map((t) => (
						<button
							key={t}
							type="button"
							onClick={() => setActiveType(t)}
							className={`text-[11px] px-3 py-1.5 rounded-lg border font-mono font-bold uppercase tracking-wider transition-all ${
								activeType === t
									? "bg-cyber-primary text-cyber-bg border-cyber-primary"
									: "bg-white/5 text-cyber-dim border-cyber-border hover:border-cyber-primary/40"
							}`}
						>
							{t}
						</button>
					))}
				</div>
				<span className="text-[10px] font-mono text-cyber-dim ml-auto">
					{filtered.length} servers
				</span>
			</div>

			{/* Server cards */}
			<div className="space-y-5">
				{filtered.map((server) => (
					<div
						key={server.name}
						id={server.name}
						className="feature-card rounded-xl p-5 scroll-mt-24"
					>
						<div className="flex flex-wrap items-center gap-2 mb-3">
							<h3 className="font-bold text-white font-mono text-base">{server.name}</h3>
							<span
								className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold uppercase ${typeColors[server.type] ?? ""}`}
							>
								{server.type}
							</span>
							<span
								className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold uppercase ${transportColors[server.transport] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
							>
								{server.transport}
							</span>
						</div>

						<p className="text-sm text-cyber-text/80 mb-4 leading-relaxed">{server.description}</p>

						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							{/* Connection info */}
							<div>
								{server.command && (
									<CodeBlock lang="bash" title="Command (stdio)" code={server.command} />
								)}
								{server.url && <CodeBlock lang="bash" title="Endpoint (SSE)" code={server.url} />}
								{server.envVars && server.envVars.length > 0 && (
									<div className="mt-3">
										<p className="text-[10px] font-mono text-cyber-warning/70 uppercase tracking-widest mb-1">
											Required Env Vars
										</p>
										<div className="flex flex-wrap gap-2">
											{server.envVars.map((ev) => (
												<code
													key={ev}
													className="text-[11px] px-2 py-0.5 bg-cyber-warning/10 text-cyber-warning border border-cyber-warning/30 rounded font-mono"
												>
													{ev}
												</code>
											))}
										</div>
									</div>
								)}
							</div>

							{/* Tools list */}
							{server.tools && server.tools.length > 0 && (
								<div>
									<p className="text-[10px] font-mono text-cyber-dim uppercase tracking-widest mb-2">
										Available Tools ({server.tools.length})
									</p>
									<div className="flex flex-wrap gap-1.5">
										{server.tools.map((tool) => (
											<code
												key={tool}
												className="text-[10px] px-2 py-0.5 bg-cyber-primary/5 text-cyber-primary border border-cyber-primary/20 rounded font-mono"
											>
												{tool}
											</code>
										))}
									</div>
								</div>
							)}
						</div>
					</div>
				))}
			</div>

			{/* Env var setup guide */}
			<div className="mt-12 glass-panel rounded-xl p-6 border border-cyber-secondary/20">
				<h2 className="text-base font-bold text-cyber-secondary mb-4 uppercase tracking-wider">
					Setting Up Remote Server Credentials
				</h2>
				<p className="text-sm text-cyber-text/80 mb-4">
					Remote servers that require API keys (GitHub, Exa, Jina) read credentials from environment
					variables. Set them before launching MYTH:
				</p>
				<CodeBlock
					lang="bash"
					title="Export credentials"
					code={`# Add to ~/.bashrc, ~/.zshrc, or set in user.yaml env block
export GITHUB_PERSONAL_ACCESS_TOKEN="ghp_xxxxxxxxxxxxx"
export EXA_API_KEY="exa-xxxxxxxxxxxxxxxxx"

# Then launch MYTH — credentials are auto-injected into MCP processes
myth scan target.com`}
				/>
				<p className="text-xs text-cyber-dim mt-4 italic">
					💡 Credentials are never logged or stored by MYTH. They live only in process environment
					for the session lifetime.
				</p>
			</div>
		</div>
	);
}
