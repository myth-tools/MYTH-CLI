import {
	Activity,
	Brain,
	ChevronRight,
	GitCompare,
	Info,
	Layers,
	Search,
	Shield,
	Terminal,
	Zap,
} from "lucide-react";
import { useState } from "react";
import { HighlightText, PageHeader } from "../components/Layout";
import { interactiveCommands } from "../data/content";

export default function InteractiveCommandsPage() {
	const [filter, setFilter] = useState("");

	const categories = [
		"Mission Core",
		"Precision Ops",
		"Intelligence & Analytics",
		"Asset Registry",
		"System & Maintenance",
	] as const;

	const filtered = interactiveCommands.filter((c) => {
		const term = filter.toLowerCase().trim();
		if (!term) {
			return true;
		}

		return (
			c.name.toLowerCase().includes(term) ||
			c.description.toLowerCase().includes(term) ||
			c.category.toLowerCase().includes(term) ||
			c.usage.toLowerCase().includes(term)
		);
	});

	const grouped = categories.reduce(
		(acc, cat) => {
			const commands = filtered.filter((c) => c.category === cat);
			if (commands.length > 0) {
				acc[cat] = commands;
			}
			return acc;
		},
		{} as Record<string, typeof interactiveCommands>,
	);

	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Interactive Neural Commands"
					description="Tactical slash commands available inside the Myth TUI and interactive CLI sessions. Unified neural mapping between command-line parity and AI-assisted missions."
					badge="Cyber Interface"
				/>

				{/* NEW: Operational Workflow Visual Section */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-6">
						<Layers className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Operational Neural Loop
						</h2>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-3 gap-6">
						{[
							{
								step: "01. Intake",
								icon: <Activity className="w-4 h-4" />,
								title: "Input Parsing",
								desc: "The system detects if you entered a /slash command, a verified alias, or natural language intent.",
								color: "border-cyber-primary/30 text-cyber-primary bg-cyber-primary/5",
							},
							{
								step: "02. Synthesis",
								icon: <Brain className="w-4 h-4" />,
								title: "Neural Mapping",
								desc: "Commands are translated into specific tool calls. Natural language is sent to the NVIDIA NIM core for reasoning.",
								color: "border-cyber-secondary/30 text-cyber-secondary bg-cyber-secondary/5",
							},
							{
								step: "03. Execution",
								icon: <Zap className="w-4 h-4" />,
								title: "Tactical Action",
								desc: "The sandboxed execution layer dispatches local binaries or remote MCP servers and streams results to the TUI.",
								color: "border-cyber-accent/30 text-cyber-accent bg-cyber-accent/5",
							},
						].map((w) => (
							<div
								key={w.step}
								className={`p-6 rounded-2xl border ${w.color} relative overflow-hidden group hover:scale-[1.02] transition-all`}
							>
								<div className="text-[10px] font-black uppercase tracking-[0.2em] mb-4 opacity-60">
									{w.step}
								</div>
								<div className="flex items-start gap-3 mb-3">
									<div className="w-8 h-8 rounded-lg bg-black/20 flex items-center justify-center border border-white/5">
										{w.icon}
									</div>
									<h3 className="font-bold text-white uppercase text-xs tracking-widest mt-1.5">
										{w.title}
									</h3>
								</div>
								<p className="text-[11px] text-cyber-text/70 leading-relaxed">{w.desc}</p>
								<div className="absolute top-0 right-0 p-2 opacity-[0.03] group-hover:opacity-[0.08] transition-opacity">
									<Activity className="w-16 h-16" />
								</div>
							</div>
						))}
					</div>
				</div>

				{/* NEW: CLI vs Interactive Matrix */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-6">
						<GitCompare className="w-5 h-5 text-cyber-secondary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Execution Parity Matrix
						</h2>
					</div>
					<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50">
						<table className="w-full text-left text-xs docs-table border-none">
							<thead className="bg-white/5">
								<tr>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
										Context Feature
									</th>
									<th className="py-4 px-6 text-white uppercase tracking-widest text-[10px]">
										Standard CLI
									</th>
									<th className="py-4 px-6 text-cyber-primary uppercase tracking-widest text-[10px]">
										Neural Interface (Interactive)
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/20">
								{[
									["Tool Parity", "Full (via subcommands)", "Full (via slash commands)"],
									[
										"AI Assistance",
										"Optional (via myth chat prompt)",
										"Always Active (Natural Language)",
									],
									[
										"Visual Output",
										"Stdout / Piped Log",
										"Real-time TUI Matrix / Interactive Charts",
									],
									[
										"Context Memory",
										"Persistence Volume Needed",
										"In-Memory Semantic Recall (Active)",
									],
									["State Control", "Restart per mission", "Modify mission state in-flight"],
									[
										"Interactive Help",
										"myth <cmd> --help",
										"Ghost Suggest / Real-time manual lookup",
									],
								].map(([feat, cli, neuro]) => (
									<tr key={feat} className="hover:bg-white/[0.02] transition-colors">
										<td className="py-4 px-6 font-bold text-cyber-text/80">{feat}</td>
										<td className="py-4 px-6 text-cyber-dim italic">{cli}</td>
										<td className="py-4 px-6 text-cyber-primary font-bold">{neuro}</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>

				<div className="mb-10 relative max-w-2xl">
					<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
						<Search className="h-4 w-4 text-cyber-dim" />
					</div>
					<input
						id="interactive-command-search"
						name="interactive-command-query"
						aria-label="Search interactive commands"
						type="text"
						placeholder="Search interactive commands, categories, or usage patterns..."
						value={filter}
						onChange={(e) => setFilter(e.target.value)}
						className="w-full pl-10 pr-24 py-3 text-sm bg-cyber-bg border border-cyber-border rounded-2xl text-white placeholder-cyber-dim focus:outline-none focus:border-cyber-primary transition-all shadow-md"
					/>
					<div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
						<span className="text-[10px] font-mono text-cyber-primary px-2 py-0.5 bg-cyber-primary/10 rounded uppercase">
							{filtered.length} tactical units
						</span>
					</div>
				</div>

				{/* Commands Grouped */}
				<div className="space-y-20">
					{categories.map((category) => {
						const cmds = grouped[category];
						if (!cmds) {
							return null;
						}

						return (
							<div
								key={category}
								id={category.replace(/ /g, "-").toLowerCase()}
								className="scroll-mt-24"
							>
								<div className="flex items-center gap-4 mb-8">
									<h2 className="text-xl font-black text-white uppercase tracking-wider italic">
										{category}
									</h2>
									<div className="h-[2px] flex-1 bg-gradient-to-r from-cyber-primary/40 to-transparent" />
									<span className="text-[10px] font-mono text-cyber-dim uppercase tracking-widest">
										{cmds.length} vectors
									</span>
								</div>

								<div className="grid grid-cols-1 gap-6">
									{cmds.map((c) => (
										<div
											key={c.name}
											className="feature-card p-6 rounded-2xl border border-cyber-border/40 hover:border-cyber-primary/40 transition-all group/card relative overflow-hidden"
										>
											<div className="absolute -top-12 -right-12 w-24 h-24 bg-cyber-primary/5 rounded-full blur-2xl group-hover/card:bg-cyber-primary/10 transition-all pointer-events-none" />

											<div className="flex flex-wrap items-center justify-between gap-4 mb-4">
												<div className="flex items-center gap-4">
													<div className="px-3 py-1 bg-cyber-primary/10 border border-cyber-primary/20 rounded-lg shadow-sm">
														<code className="text-lg font-bold text-cyber-primary font-mono group-hover/card:text-white transition-colors">
															/<HighlightText text={c.name} highlight={filter} />
														</code>
													</div>
													<div className="h-5 w-px bg-cyber-border/50" />
													<code className="text-[11px] text-cyber-dim font-mono opacity-80 bg-white/5 px-2 py-0.5 rounded">
														CLI Parity: myth {c.cliEquiv.replace("myth ", "")}
													</code>
												</div>
												<span className="text-[10px] px-2.5 py-1 bg-white/5 text-cyber-dim border border-white/10 rounded-full uppercase font-bold tracking-widest">
													{c.category}
												</span>
											</div>

											<p className="text-sm text-cyber-text/80 mb-6 leading-relaxed max-w-3xl">
												<HighlightText text={c.description} highlight={filter} />
											</p>

											<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
												{/* Input Definition */}
												<div className="bg-black/40 rounded-xl p-5 border border-white/5 group-hover/card:border-cyber-primary/15 transition-colors">
													<div className="flex items-center gap-2 mb-3 text-[10px] text-cyber-dim font-bold uppercase tracking-widest">
														<Terminal className="w-3.5 h-3.5 text-cyber-primary" />
														Mission Input Syntax
													</div>
													<div className="bg-cyber-bg/60 p-3 rounded-lg border border-white/5 font-mono text-xs text-cyber-secondary overflow-x-auto whitespace-pre">
														{c.usage}
													</div>
												</div>

												{/* Examples */}
												<div className="space-y-4">
													<div className="text-[10px] text-cyber-dim/60 uppercase font-bold tracking-widest ml-1 flex items-center gap-2">
														<Zap className="w-3.5 h-3.5 text-cyber-primary" />
														Tactical Scenarios
													</div>
													<div className="space-y-3">
														{c.examples.map((ex) => (
															<div key={ex.command} className="group/ex">
																<div className="bg-white/5 hover:bg-cyber-primary/5 p-3 rounded-xl border border-transparent hover:border-cyber-primary/10 transition-all">
																	<div className="flex items-center gap-2 mb-2">
																		<ChevronRight className="w-3 h-3 text-cyber-primary" />
																		<code className="text-[11px] font-mono text-cyber-primary font-bold">
																			/{ex.command}
																		</code>
																	</div>
																	<p className="text-[10px] text-cyber-dim/80 leading-relaxed italic border-l border-cyber-primary/20 pl-3 group-hover/ex:text-cyber-text/80 transition-colors">
																		{ex.description}
																	</p>
																</div>
															</div>
														))}
													</div>
												</div>
											</div>
										</div>
									))}
								</div>
							</div>
						);
					})}
				</div>

				{/* NEW: Neural Core Pro-Tips */}
				<div className="mt-24 pt-12 border-t border-cyber-border/30">
					<div className="flex items-center gap-3 mb-8">
						<Brain className="w-6 h-6 text-cyber-primary" />
						<h2 className="text-2xl font-black text-white uppercase tracking-tighter">
							Neural Core Integration
						</h2>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
						<div className="glass-panel p-6 rounded-2xl border border-cyber-primary/20 bg-cyber-primary/5">
							<h3 className="text-sm font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
								<Info className="w-4 h-4 text-cyber-primary" />
								Talk to the AI
							</h3>
							<p className="text-xs text-cyber-text/80 leading-relaxed mb-4">
								You don't always need slash commands. The neural interface understands natural
								language intent and can plan multi-step missions autonomously.
							</p>
							<div className="space-y-2">
								<div className="p-3 bg-black/40 rounded-xl border border-white/5">
									<p className="text-[9px] text-cyber-dim mb-1 italic">Prompt Example:</p>
									<p className="text-xs text-white">
										"Check if target.com has any open S3 buckets and summarize findings."
									</p>
								</div>
								<div className="p-3 bg-black/40 rounded-xl border border-white/5">
									<p className="text-[9px] text-cyber-dim mb-1 italic">Prompt Example:</p>
									<p className="text-xs text-white">
										"Perform a stealth scan of the subnet 10.0.5.0/24 and report back."
									</p>
								</div>
							</div>
						</div>

						<div className="glass-panel p-6 rounded-2xl border border-cyber-secondary/20 bg-cyber-secondary/5">
							<h3 className="text-sm font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
								<Layers className="w-4 h-4 text-cyber-secondary" />
								Dynamic Control
							</h3>
							<ul className="space-y-3">
								<li className="flex gap-3 items-start">
									<div className="w-1.5 h-1.5 rounded-full bg-cyber-secondary mt-1 shrink-0" />
									<p className="text-xs text-cyber-text/80 leading-relaxed">
										Toggle MCP servers on-the-fly during a mission using{" "}
										<code className="text-cyber-secondary">/mcp toggle</code>.
									</p>
								</li>
								<li className="flex gap-3 items-start">
									<div className="w-1.5 h-1.5 rounded-full bg-cyber-secondary mt-1 shrink-0" />
									<p className="text-xs text-cyber-text/80 leading-relaxed">
										Update neural recursion depth real-time via{" "}
										<code className="text-cyber-secondary">/depth</code> to expand focus.
									</p>
								</li>
								<li className="flex gap-3 items-start">
									<div className="w-1.5 h-1.5 rounded-full bg-cyber-secondary mt-1 shrink-0" />
									<p className="text-xs text-cyber-text/80 leading-relaxed">
										Call <code className="text-cyber-secondary">/sync</code> to hot-reload tools
										without dropping your session.
									</p>
								</li>
							</ul>
						</div>
					</div>
				</div>
			</div>

			{/* Sidebar Quick-Jump */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-6 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-transparent via-cyber-primary/40 to-transparent" />
					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Shield className="w-4 h-4" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Interface Index</h4>
					</div>
					<nav className="space-y-2">
						{categories.map((cat) => {
							const id = cat.replace(/ /g, "-").toLowerCase();
							const count = filtered.filter((c) => c.category === cat).length;
							if (count === 0) {
								return null;
							}

							return (
								<button
									key={cat}
									type="button"
									onClick={(e) => {
										e.preventDefault();
										document.getElementById(id)?.scrollIntoView({ behavior: "smooth" });
									}}
									className="w-full group flex items-center justify-between p-3 rounded-2xl hover:bg-cyber-primary/5 border border-transparent hover:border-cyber-primary/20 transition-all outline-none cursor-pointer"
								>
									<span className="text-[11px] text-cyber-dim group-hover:text-white transition-colors font-bold uppercase tracking-wider">
										{cat}
									</span>
									<span className="text-[10px] font-mono text-cyber-primary/40 group-hover:text-cyber-primary transition-colors bg-cyber-primary/5 px-2 py-0.5 rounded border border-cyber-primary/10">
										{count}
									</span>
								</button>
							);
						})}
					</nav>
					<div className="mt-12 pt-8 border-t border-cyber-border/20">
						<div className="p-4 bg-black/40 rounded-2xl border border-cyber-primary/10">
							<div className="flex items-center gap-2 mb-2">
								<Activity className="w-3.5 h-3.5 text-cyber-primary animate-pulse" />
								<span className="text-[10px] text-cyber-dim font-black uppercase tracking-widest">
									Interface Telemetry
								</span>
							</div>
							<p className="text-[10px] text-cyber-text/60 leading-relaxed italic">
								All interactive vectors currently{" "}
								<span className="text-cyber-primary font-bold">STABLE</span> and mapped to mission
								assets.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
