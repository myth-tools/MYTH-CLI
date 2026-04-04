import { CodeBlock, PageHeader } from "../components/Layout";
import { reconPhases, reconProfiles } from "../data/content";

const profileIncludedPhases: Record<string, number[]> = {
	quick: [0, 1, 2, 3, 4, 5],
	full: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
	stealth: [0, 1, 9],
	vuln: [2, 3, 4, 5, 7, 8],
	elite: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
};

const profileThemes: Record<string, string> = {
	quick: "border-cyber-secondary/30 bg-cyber-secondary/5",
	full: "border-cyber-primary/30 bg-cyber-primary/5",
	stealth: "border-purple-500/30 bg-purple-500/5",
	vuln: "border-cyber-error/30 bg-cyber-error/5",
	elite: "border-cyber-warning/30 bg-cyber-warning/5",
};

const profileTextColors: Record<string, string> = {
	quick: "text-cyber-secondary",
	full: "text-cyber-primary",
	stealth: "text-purple-400",
	vuln: "text-cyber-error",
	elite: "text-cyber-warning",
};

export default function ProfilesPage() {
	return (
		<div className="flex flex-col lg:flex-row gap-8">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Recon Profiles & Phases"
					description="MYTH's 13-phase, 89-step methodology broken into 5 mission templates. Each profile activates a different subset of phases for targeted operational control."
					badge="Mission Templates"
				/>

				{/* Profile Cards */}
				<div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4 mb-12">
					{reconProfiles.map((p) => (
						<div
							key={p.name}
							className={`feature-card rounded-xl p-5 border ${profileThemes[p.name] ?? "border-cyber-border/40"} hover-glow transition-all group`}
						>
							<div className="flex items-center justify-between mb-3">
								<code
									className={`text-lg font-black font-mono tracking-tighter uppercase ${profileTextColors[p.name] ?? "text-cyber-primary"}`}
								>
									{p.name}
								</code>
								{p.premium && (
									<span className="text-[9px] px-2 py-0.5 bg-cyber-primary text-black font-bold rounded uppercase tracking-widest animate-pulse">
										Elite Tier
									</span>
								)}
							</div>
							<p className="text-xs text-cyber-text/80 mb-5 leading-relaxed">{p.description}</p>
							<div className="grid grid-cols-2 gap-2 mb-4">
								<div className="bg-black/40 rounded-lg p-2 border border-white/5 text-center">
									<div className="text-[9px] text-cyber-dim uppercase font-bold tracking-widest mb-0.5">
										Phases
									</div>
									<div className={`text-sm font-mono font-bold ${profileTextColors[p.name]}`}>
										{p.phases}
									</div>
								</div>
								<div className="bg-black/40 rounded-lg p-2 border border-white/5 text-center">
									<div className="text-[9px] text-cyber-dim uppercase font-bold tracking-widest mb-0.5">
										Steps
									</div>
									<div className={`text-sm font-mono font-bold ${profileTextColors[p.name]}`}>
										{p.steps}
									</div>
								</div>
							</div>
							<div className="border-t border-white/5 pt-3">
								<code
									className={`block text-[10px] font-mono bg-black/30 p-2 rounded border border-white/5 ${profileTextColors[p.name] ?? "text-cyber-primary"}/80 group-hover:border-${p.name === "quick" ? "cyber-secondary" : "cyber-primary"}/30 transition-colors break-all`}
								>
									{p.example}
								</code>
							</div>
						</div>
					))}
				</div>

				{/* Profile Comparison Matrix */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-6 flex items-center gap-3">
					<span className="text-cyber-primary font-mono">0x01</span>
					Phase Coverage Matrix
				</h2>
				<div className="table-container mb-12">
					<table className="w-full text-xs docs-table">
						<thead>
							<tr>
								<th className="text-left py-3 px-3 min-w-[140px]">Phase</th>
								{reconProfiles.map((p) => (
									<th key={p.name} className={`text-center py-3 px-2 ${profileTextColors[p.name]}`}>
										{p.name}
									</th>
								))}
							</tr>
						</thead>
						<tbody className="divide-y divide-cyber-border/20">
							{reconPhases.map((phase, i) => (
								<tr key={phase.phase} className="hover:bg-white/[0.02] transition-colors">
									<td className="py-2.5 px-3">
										<span className="text-cyber-dim font-mono mr-2">
											{phase.phase.toString().padStart(2, "0")}
										</span>
										<span className="text-cyber-text/80">{phase.name}</span>
									</td>
									{reconProfiles.map((p) => {
										const included = profileIncludedPhases[p.name]?.includes(i);
										return (
											<td key={p.name} className="py-2.5 px-2 text-center">
												{included ? (
													<span className={`font-bold ${profileTextColors[p.name]}`}>✓</span>
												) : (
													<span className="text-cyber-dim/30">–</span>
												)}
											</td>
										);
									})}
								</tr>
							))}
						</tbody>
					</table>
				</div>

				{/* Methodology Control */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-6 flex items-center gap-3">
					<span className="text-cyber-primary font-mono">0x02</span>
					Methodology Control
				</h2>
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-12">
					<CodeBlock lang="bash" title="List all profiles" code="myth profile" />
					<CodeBlock lang="bash" title="Inspect a profile" code="myth profile full" />
					<CodeBlock
						lang="bash"
						title="Disable specific phases"
						code="myth profile elite disable 3,4,5"
					/>
					<CodeBlock lang="bash" title="Re-enable phases" code="myth profile elite enable 3,4,5" />
					<CodeBlock
						lang="bash"
						title="Run with a profile"
						code="myth scan target.com --profile stealth"
					/>
					<CodeBlock
						lang="bash"
						title="Interactive override"
						code="scan target.com --profile vuln"
					/>
				</div>

				{/* 13 phases */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono">0x03</span>
					The 13-Phase Methodology
				</h2>
				<div className="space-y-3">
					{reconPhases.map((phase) => (
						<div
							key={phase.phase}
							className="glass-panel p-4 rounded-xl border border-cyber-border/30 hover:border-cyber-primary/40 transition-all flex gap-5 items-start group"
						>
							<div className="w-10 h-10 shrink-0 flex items-center justify-center rounded-lg bg-white/5 border border-white/10 group-hover:border-cyber-primary/30 transition-all">
								<span className="text-cyber-primary font-black font-mono text-base">
									{phase.phase.toString().padStart(2, "0")}
								</span>
							</div>
							<div className="flex-1 min-w-0">
								<h3 className="text-sm font-bold text-white mb-1 group-hover:text-cyber-primary transition-colors">
									{phase.name}
								</h3>
								<p className="text-xs text-cyber-dim leading-relaxed">{phase.description}</p>
							</div>
							<div className="hidden sm:flex shrink-0 flex-wrap gap-1 max-w-[180px]">
								{reconProfiles
									.filter((p) => profileIncludedPhases[p.name]?.includes(phase.phase))
									.map((p) => (
										<span
											key={p.name}
											className={`text-[8px] px-1.5 py-0.5 rounded font-mono font-bold border ${profileThemes[p.name]?.replace("bg-", "bg-").replace("/5", "/20") ?? ""} ${profileTextColors[p.name]}`}
										>
											{p.name}
										</span>
									))}
							</div>
						</div>
					))}
				</div>
			</div>

			{/* Sidebar */}
			<div className="hidden lg:block w-64 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-2xl p-5 border border-cyber-border/50">
					<div className="flex items-center gap-2 mb-4 text-cyber-primary">
						<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
						<h4 className="text-[10px] font-black uppercase tracking-[0.2em]">Template Index</h4>
					</div>
					<nav className="space-y-1">
						{reconProfiles.map((p) => (
							<div
								key={p.name}
								className="group flex items-center justify-between p-2 rounded-lg hover:bg-cyber-primary/5 transition-all cursor-default"
							>
								<span
									className={`text-[11px] transition-colors font-bold ${profileTextColors[p.name]}`}
								>
									{p.name}
								</span>
								<div className="text-right">
									<div className="text-[9px] font-mono text-cyber-dim/50">{p.steps} steps</div>
									<div className="text-[9px] font-mono text-cyber-dim/30">ph:{p.phases}</div>
								</div>
							</div>
						))}
					</nav>
					<div className="mt-6 pt-5 border-t border-cyber-border/30">
						<div className="bg-cyber-primary/10 rounded-lg p-3 border border-cyber-primary/20">
							<p className="text-[10px] text-cyber-primary font-bold leading-relaxed">
								METHODOLOGY: RIGOROUS
							</p>
							<p className="text-[9px] text-cyber-primary/60 mt-1 leading-relaxed">
								89 atomic steps across 13 operational sectors.
							</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	);
}
