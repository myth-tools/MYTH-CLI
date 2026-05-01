import { Activity, Brain, Info, Layers, Search, Shield, Terminal, Zap } from "lucide-react";
import { useState } from "react";
import { CodeBlock, HighlightText, PageHeader } from "../components/Layout";
import { cliCommands } from "../data/content";

const categories = [
	{
		id: "mission-core",
		label: "Mission Core",
		icon: <Zap className="w-4 h-4" />,
		desc: "The main tactical tools used for active/passive discovery.",
	},
	{
		id: "intelligence-&-reporting",
		label: "Intelligence & Reporting",
		icon: <Activity className="w-4 h-4" />,
		desc: "Commands for synthesizing data, generating reports, and viewing findings.",
	},
	{
		id: "asset-management",
		label: "Asset Management",
		icon: <Shield className="w-4 h-4" />,
		desc: "Synchronize and inspect local tools and remote MCP servers.",
	},
	{
		id: "system-&-maintenance",
		label: "System & Maintenance",
		icon: <Terminal className="w-4 h-4" />,
		desc: "Core diagnostic, configuration, and emergency decommissioning tools.",
	},
] as const;

export default function CliCommandsPage() {
	const [filter, setFilter] = useState("");

	const filtered = cliCommands.filter((c) => {
		const term = filter.toLowerCase().trim();
		if (!term) {
			return true;
		}

		const matchName = c.name.toLowerCase().includes(term);
		const matchDesc = c.description.toLowerCase().includes(term);
		const matchCategory = c.category.toLowerCase().includes(term);
		const matchAlias = c.aliases?.some((a) => a.toLowerCase().includes(term)) ?? false;
		const matchUsage = c.usage.toLowerCase().includes(term);
		const matchArgs =
			c.args?.some(
				(a) => a.name.toLowerCase().includes(term) || a.description.toLowerCase().includes(term),
			) ?? false;

		return matchName || matchDesc || matchCategory || matchAlias || matchUsage || matchArgs;
	});

	const grouped = categories.reduce(
		(acc, cat) => {
			const commands = filtered.filter((c) => c.category === cat.label);
			if (commands.length > 0) {
				acc[cat.label] = commands;
			}
			return acc;
		},
		{} as Record<string, typeof cliCommands>,
	);

	const scrollToCategory = (id: string) => {
		document.getElementById(id)?.scrollIntoView({ behavior: "smooth" });
	};

	return (
		<div className="flex flex-col lg:flex-row gap-8">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="CLI Commands"
					description="Complete reference for all 27 myth CLI subcommands. Grouped by operational mission sectors for maximum tactical efficiency."
					badge="Commands Registry"
				/>

				{/* NEW: Category Navigation Cards */}
				<div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4 mb-10">
					{categories.map((cat) => (
						<button
							key={cat.id}
							type="button"
							onClick={() => scrollToCategory(cat.id)}
							className="feature-card rounded-xl p-4 text-left border border-cyber-border/40 hover:border-cyber-primary/50 transition-all group"
						>
							<div className="flex items-center gap-3 mb-2">
								<div className="w-8 h-8 rounded-lg bg-cyber-primary/10 border border-cyber-primary/20 flex items-center justify-center text-cyber-primary group-hover:scale-110 transition-transform">
									{cat.icon}
								</div>
								<h3 className="text-[11px] font-bold text-white uppercase tracking-wider">
									{cat.label}
								</h3>
							</div>
							<p className="text-[10px] text-cyber-dim leading-relaxed">{cat.desc}</p>
						</button>
					))}
				</div>

				<div className="mb-10 relative max-w-2xl">
					<div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
						<Search className="h-5 w-5 text-cyber-dim" />
					</div>
					<input
						id="cli-command-search"
						name="cli-command-query"
						aria-label="Search CLI commands"
						type="text"
						placeholder="Search any command, flag, or operational category..."
						value={filter}
						onChange={(e) => setFilter(e.target.value)}
						className="w-full pl-10 pr-24 py-3 text-sm bg-cyber-bg border border-cyber-border rounded-xl text-white placeholder-cyber-dim focus:outline-none focus:border-cyber-primary focus:ring-1 focus:ring-cyber-primary/50 transition-all shadow-sm"
					/>
					<div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
						<span className="text-xs font-mono text-cyber-primary px-2 py-1 bg-cyber-primary/10 rounded">
							{filtered.length} tactical units
						</span>
					</div>
				</div>

				{filtered.length === 0 && (
					<div className="text-center py-12 border border-dashed border-cyber-border rounded-xl bg-cyber-dark/30">
						<h3 className="text-lg font-medium text-white mb-1">
							Search query returned zero results
						</h3>
						<p className="text-sm text-cyber-dim mb-4">
							Ensure mission parameters match existing command registry units.
						</p>
						<button
							type="button"
							onClick={() => setFilter("")}
							className="px-4 py-2 bg-cyber-primary/10 text-cyber-primary border border-cyber-primary/30 rounded-lg hover:bg-cyber-primary/20 transition-colors text-sm font-medium"
						>
							Reset Intelligence Filter
						</button>
					</div>
				)}

				<div className="space-y-20">
					{categories.map((cat) => {
						const cmds = grouped[cat.label];
						if (!cmds) {
							return null;
						}

						return (
							<div key={cat.id} id={cat.id} className="scroll-mt-24">
								<div className="flex items-center gap-4 mb-10">
									<div className="flex items-center gap-3">
										<div className="w-10 h-10 rounded-xl bg-cyber-primary/10 border border-cyber-primary/20 flex items-center justify-center text-cyber-primary">
											{cat.icon}
										</div>
										<h2 className="text-2xl font-black text-white uppercase tracking-tighter">
											{cat.label}
										</h2>
									</div>
									<div className="h-px flex-1 bg-gradient-to-r from-cyber-primary/30 to-transparent" />
									<span className="text-xs font-mono text-cyber-dim uppercase tracking-widest">
										{cmds.length} commands
									</span>
								</div>

								<div className="space-y-16">
									{cmds.map((cmd) => (
										<div key={cmd.name} id={cmd.name} className="scroll-mt-24">
											<div className="flex flex-wrap items-center gap-3 mb-4">
												<div className="px-3 py-1 bg-black/40 border border-cyber-border rounded-lg flex items-center gap-3">
													<code className="text-lg font-bold text-white font-mono flex items-center gap-2">
														<span className="text-cyber-dim opacity-50">$</span>
														myth <HighlightText text={cmd.name} highlight={filter} />
													</code>
												</div>
												{cmd.aliases?.map((a) => (
													<span
														key={a}
														className="text-[10px] px-2 py-1 bg-cyber-secondary/10 text-cyber-secondary border border-cyber-secondary/20 rounded font-mono uppercase tracking-widest"
													>
														Alias: <HighlightText text={a} highlight={filter} />
													</span>
												))}
												<div className="ml-auto flex items-center gap-2">
													<span className="text-[10px] px-2 py-1 bg-cyber-primary/5 text-cyber-primary/40 border border-cyber-primary/10 rounded font-mono uppercase tracking-widest">
														Sector: {cmd.category}
													</span>
												</div>
											</div>

											<p className="text-sm text-cyber-text/80 leading-relaxed mb-6 max-w-4xl">
												<HighlightText text={cmd.description} highlight={filter} />
											</p>

											<div className="grid grid-cols-1 gap-8">
												<div className="relative group">
													<div className="absolute -inset-0.5 bg-gradient-to-r from-cyber-primary/20 to-transparent rounded-xl opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none" />
													<CodeBlock lang="bash" title="Operational Usage" code={cmd.usage} />
												</div>

												{cmd.args && cmd.args.length > 0 && (
													<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50">
														<div className="bg-white/5 px-5 py-3 border-b border-cyber-border/50 flex items-center justify-between">
															<div className="flex items-center gap-2">
																<div className="w-1.5 h-1.5 rounded-full bg-cyber-primary" />
																<span className="text-[10px] font-bold text-cyber-dim uppercase tracking-widest">
																	Command Parameters
																</span>
															</div>
															<span className="text-[10px] font-mono text-cyber-primary/70 bg-cyber-primary/5 px-2 py-0.5 rounded border border-cyber-primary/10">
																{cmd.args.length} Inputs
															</span>
														</div>
														<div className="table-container">
															<table className="w-full text-xs docs-table border-none">
																<thead>
																	<tr>
																		<th className="text-left py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
																			Flag / Param
																		</th>
																		<th className="text-left py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
																			Data Type
																		</th>
																		<th className="text-center py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
																			Priority
																		</th>
																		<th className="text-left py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
																			Operational Context
																		</th>
																	</tr>
																</thead>
																<tbody className="divide-y divide-cyber-border/30">
																	{cmd.args.map((a) => (
																		<tr
																			key={a.name}
																			className="hover:bg-white/[0.04] transition-colors"
																		>
																			<td className="py-4 px-6 font-mono text-cyber-primary font-bold text-sm">
																				<HighlightText text={a.name} highlight={filter} />
																			</td>
																			<td className="py-4 px-6">
																				<span className="text-cyber-dim font-mono bg-white/5 px-2 py-1 rounded text-[10px] border border-white/5">
																					{a.type}
																				</span>
																			</td>
																			<td className="py-4 px-6 text-center">
																				{a.required ? (
																					<span className="text-[10px] font-bold text-cyber-error bg-cyber-error/10 px-2 py-0.5 rounded border border-cyber-error/20">
																						CRITICAL
																					</span>
																				) : (
																					<span className="text-[10px] text-cyber-dim/50 font-medium">
																						OPTIONAL
																					</span>
																				)}
																			</td>
																			<td className="py-4 px-6 text-cyber-text/80 leading-relaxed">
																				<HighlightText text={a.description} highlight={filter} />
																				{a.default && (
																					<div className="text-[11px] text-cyber-secondary mt-1 flex items-center gap-1.5">
																						<Info className="w-3 h-3 opacity-60" />
																						<span>
																							Default Vector:{" "}
																							<code className="font-mono bg-cyber-secondary/5 px-1 rounded">
																								{a.default}
																							</code>
																						</span>
																					</div>
																				)}
																			</td>
																		</tr>
																	))}
																</tbody>
															</table>
														</div>
													</div>
												)}

												{/* NEW: Operational Scenarios with shell tips */}
												<div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
													<div className="lg:col-span-2 space-y-4">
														<div className="flex items-center gap-2 mb-2 ml-1">
															<Zap className="w-4 h-4 text-cyber-primary" />
															<h4 className="text-[11px] font-bold text-white uppercase tracking-[0.2em]">
																Operational Scenarios
															</h4>
														</div>
														<div className="space-y-4">
															{cmd.examples.map((ex) => (
																<div key={ex.command} className="group relative">
																	<div className="absolute left-0 top-0 bottom-0 w-1 bg-cyber-primary/30 rounded-full group-hover:bg-cyber-primary transition-colors" />
																	<div className="pl-5">
																		<div className="bg-black/60 rounded-xl border border-white/5 overflow-hidden group-hover:border-cyber-primary/20 transition-all shadow-sm">
																			<div className="bg-white/5 px-3 py-1 flex items-center justify-between border-b border-white/5">
																				<span className="text-[9px] font-mono text-cyber-dim uppercase tracking-widest">
																					scenario execution
																				</span>
																				<Terminal className="w-3 h-3 text-cyber-dim opacity-40" />
																			</div>
																			<div className="p-4">
																				<code className="text-[13px] font-mono text-cyber-primary break-all">
																					{ex.command}
																				</code>
																				<p className="text-[11px] text-cyber-dim mt-3 leading-relaxed border-t border-white/5 pt-3">
																					{ex.description}
																				</p>
																			</div>
																		</div>
																	</div>
																</div>
															))}
														</div>
													</div>

													{/* NEW: Shell Integration Tips */}
													<div className="lg:col-span-1">
														<div className="sticky top-28">
															<div className="flex items-center gap-2 mb-4 ml-1">
																<Info className="w-4 h-4 text-cyber-secondary" />
																<h4 className="text-[11px] font-bold text-cyber-secondary uppercase tracking-[0.2em]">
																	Shell Context
																</h4>
															</div>
															<div className="glass-panel p-5 rounded-2xl border border-cyber-secondary/20 bg-cyber-secondary/5 space-y-4">
																<div className="space-y-2">
																	<div className="text-[10px] font-bold text-cyber-secondary uppercase">
																		Piping Support
																	</div>
																	<p className="text-[10px] text-cyber-text/60 leading-relaxed italic border-l border-cyber-secondary/20 pl-3">
																		Results are optimized for <code>grep</code> and <code>awk</code>{" "}
																		parsing. Outputs can be piped directly into further discovery
																		units.
																	</p>
																</div>
																<div className="space-y-2">
																	<div className="text-[10px] font-bold text-cyber-secondary uppercase">
																		Automation Hint
																	</div>
																	<p className="text-[10px] text-cyber-text/60 leading-relaxed italic border-l border-cyber-secondary/20 pl-3">
																		Use <code>--no-tui</code> flag for headless script integration
																		or CI pipeline execution.
																	</p>
																</div>
																<div className="pt-2">
																	<div className="p-2 bg-black/40 rounded border border-white/5 text-[9px] font-mono text-cyber-dim">
																		$ myth {cmd.name} ... | tee out.txt
																	</div>
																</div>
															</div>
														</div>
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

			{/* Categorical Quick-Jump Sidebar */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-6 border border-cyber-border/50 shadow-xl overflow-hidden relative">
					<div className="absolute top-0 right-0 w-24 h-24 bg-cyber-primary/5 rounded-full blur-3xl pointer-events-none" />

					<div className="flex items-center gap-3 mb-8">
						<div className="w-2 h-2 rounded-full bg-cyber-primary shadow-[0_0_8px_#00ffa3] animate-pulse" />
						<h4 className="text-[11px] font-black text-white uppercase tracking-[0.3em]">
							Registry Index
						</h4>
					</div>

					<nav className="space-y-2">
						{categories.map((cat) => {
							const count = filtered.filter((c) => c.category === cat.label).length;
							if (count === 0) {
								return null;
							}

							return (
								<button
									key={cat.id}
									type="button"
									onClick={() => scrollToCategory(cat.id)}
									className="w-full group flex flex-col p-3 rounded-xl hover:bg-white/5 border border-transparent hover:border-white/5 transition-all outline-none cursor-pointer text-left"
								>
									<div className="flex items-center justify-between mb-1">
										<span className="text-[11px] text-white/50 group-hover:text-cyber-primary transition-colors font-bold uppercase tracking-wider">
											{cat.label}
										</span>
										<span className="text-[10px] font-mono text-cyber-primary/40 group-hover:text-cyber-primary transition-colors">
											{count.toString().padStart(2, "0")}
										</span>
									</div>
									<div className="flex gap-1">
										{Array.from({ length: Math.min(count, 8) }).map((_, i) => (
											<div
												// biome-ignore lint/suspicious/noArrayIndexKey: Static dot matrix for counts
												key={`status-dot-${cat.id}-${i}`}
												className="h-0.5 flex-1 bg-cyber-primary/20 group-hover:bg-cyber-primary/40 transition-colors"
											/>
										))}
									</div>
								</button>
							);
						})}
					</nav>

					<div className="mt-12 pt-8 border-t border-cyber-border/30">
						<div className="flex items-center justify-between mb-2">
							<span className="text-[10px] text-cyber-dim uppercase font-bold tracking-widest">
								Total Ops Pulse
							</span>
							<span className="text-xs font-mono text-cyber-primary font-bold">
								{cliCommands.length}
							</span>
						</div>
						<div className="h-1 w-full bg-white/5 rounded-full overflow-hidden">
							<div className="h-full bg-cyber-primary w-[100%] shadow-[0_0_8px_#00ffa3]" />
						</div>
						<p className="text-[10px] text-cyber-dim/60 leading-relaxed italic mt-4">
							Registry fully synchronized with operative standards.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
