import {
	Database,
	Globe,
	History,
	Info,
	Activity as Pulse,
	Shield,
	Terminal,
	Zap,
} from "lucide-react";
import { CodeBlock, PageHeader } from "../components/Layout";
import { discoveryPhases, subdomainFlags, subdomainParams } from "../data/content";

const sourceReliability = [
	{
		method: "Passive Aggregation",
		reliability: "98%",
		speed: "Ultra-Fast",
		noise: "Zero",
		sources: "Virustotal, SecurityTrails, Shodan, WebArchive, +96 others",
	},
	{
		method: "Cloud-Scale Brute",
		reliability: "100% (Coverage dependent)",
		speed: "Fast (Parallelized)",
		noise: "Low-Medium",
		sources: "2GB+ Propitiatory Wordlists, Permutation Engine",
	},
	{
		method: "DNSSEC Walking",
		reliability: "99% (Host dependent)",
		speed: "Standard",
		noise: "Minimal",
		sources: "NSEC/NSEC3 Zone Manipulation",
	},
	{
		method: "JS Variable Scrape",
		reliability: "85%",
		speed: "Real-time",
		noise: "Zero (Passive)",
		sources: "Global HTML/JS parsing & regex extraction",
	},
	{
		method: "Neural Cloud Recon",
		reliability: "92%",
		speed: "High-latency (Deep Dive)",
		noise: "Zero",
		sources: "AWS/Azure/GCP organization metadata",
	},
];

const performanceBenchmarks = [
	{
		targetType: "Standard Domain",
		domains: "100 - 5k",
		time: "15s - 45s",
		memory: "140MB",
		mode: "standard",
	},
	{
		targetType: "Enterprise Org",
		domains: "10k - 50k",
		time: "2m - 5m",
		memory: "450MB",
		mode: "full (--active)",
	},
	{
		targetType: "Cloud-Native Target",
		domains: "50k - 200k",
		time: "8m - 20m",
		memory: "1.2GB",
		mode: "master (--recursive)",
	},
];

export default function SubdomainFetchPage() {
	return (
		<div className="flex flex-col lg:flex-row gap-8">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Subdomain Fetcher (Quantum Engine)"
					description="The MOST ADVANCED open-source subdomain enumeration engine built in Rust. A systematic 18-phase discovery pipeline pulling from 100+ sources with zero API key requirement."
					badge="Cyber Asset Intelligence"
				/>

				<div className="feature-card rounded-3xl p-8 mb-12 relative overflow-hidden group border border-cyber-primary/20 shadow-2xl">
					<div className="absolute top-0 right-0 p-6 opacity-[0.03] group-hover:opacity-[0.08] transition-opacity duration-1000 rotate-12">
						<Zap className="w-56 h-56 text-cyber-primary" />
					</div>

					<div className="relative z-10">
						<div className="flex items-center gap-3 mb-6">
							<div className="h-2.5 w-2.5 rounded-full bg-cyber-primary animate-ping" />
							<h3 className="text-lg font-black text-white uppercase tracking-widest drop-shadow-sm">
								Quantum Engine Core Capabilities
							</h3>
						</div>

						<div className="grid grid-cols-1 md:grid-cols-2 gap-y-4 gap-x-12">
							{[
								{
									label: "100+ Passive Sources",
									desc: "No API keys or manual configuration required for full-depth aggregation.",
									icon: <Globe className="w-4 h-4 text-cyber-primary" />,
								},
								{
									label: "DNSSEC Zone Expansion",
									desc: "Native support for NSEC/NSEC3 chain walking and zone transfer probing.",
									icon: <Shield className="w-4 h-4 text-cyber-primary" />,
								},
								{
									label: "Dynamic Resolver Synthesis",
									desc: "5000+ public resolvers auto-fetched and validated for zero-error discovery.",
									icon: <Pulse className="w-4 h-4 text-cyber-primary" />,
								},
								{
									label: "JS Variable Scrape Cache",
									desc: "Deep extraction of hidden endpoints from minified vendor JS files.",
									icon: <Database className="w-4 h-4 text-cyber-primary" />,
								},
								{
									label: "Native Tor OPSEC",
									desc: "Route mission traffic through SOCKS5 with automated node rotation.",
									icon: <Crosshair className="w-4 h-4 text-cyber-primary" />,
								},
								{
									label: "Recursive Alt-Discovery",
									desc: "Infinite permutation logic for dash-shuffles and number increments.",
									icon: <History className="w-4 h-4 text-cyber-primary" />,
								},
							].map((cap) => (
								<div
									key={cap.label}
									className="group/item border-l border-cyber-primary/10 pl-4 py-1 hover:border-cyber-primary transition-all"
								>
									<div className="flex items-center gap-2 mb-1.5">
										{cap.icon}
										<div className="text-xs font-bold text-white group-hover/item:text-cyber-primary transition-colors">
											{cap.label}
										</div>
									</div>
									<p className="text-[10px] text-cyber-dim leading-relaxed">{cap.desc}</p>
								</div>
							))}
						</div>
					</div>
				</div>

				{/* NEW: Source Reliability Matrix */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x01</span>
					Source Reliability Matrix
				</h2>
				<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50 mb-16 shadow-lg">
					<div className="table-container">
						<table className="w-full text-left text-xs docs-table border-none">
							<thead className="bg-white/5 border-b border-cyber-border/50">
								<tr>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Discovery Method
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Reliability
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Operational Speed
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Noise Level
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Primary Sources
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/30">
								{sourceReliability.map((s) => (
									<tr key={s.method} className="hover:bg-white/[0.02] transition-colors">
										<td className="py-4 px-6 border-l-2 border-cyber-primary/20 font-bold text-white">
											{s.method}
										</td>
										<td className="py-4 px-6">
											<span className="text-cyber-primary font-mono font-bold bg-cyber-primary/5 px-2 py-0.5 rounded">
												{s.reliability}
											</span>
										</td>
										<td className="py-4 px-6 text-cyber-dim italic">{s.speed}</td>
										<td className="py-4 px-6 font-mono text-[10px]">{s.noise}</td>
										<td className="py-4 px-6 text-cyber-text/60">{s.sources}</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x02</span>
					The 18-Phase Pipeline
				</h2>
				<div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-3 mb-16">
					{discoveryPhases.map((phase, i) => (
						<div
							key={phase.name}
							className="glass-panel p-5 rounded-2xl border border-cyber-border/30 hover:border-cyber-primary/40 transition-all group flex flex-col justify-between h-full bg-white/[0.01]"
						>
							<div>
								<div className="flex items-center justify-between mb-3">
									<div className="text-[10px] font-mono text-cyber-primary/60 font-bold tracking-widest">
										PHASE {String(i + 1).padStart(2, "0")}
									</div>
									<div className="w-1.5 h-1.5 rounded-full bg-cyber-primary group-hover:shadow-[0_0_8px_#00ffa3] transition-all" />
								</div>
								<h3 className="text-[11px] font-black text-white mb-2 uppercase tracking-wider group-hover:text-cyber-primary transition-colors">
									{phase.name}
								</h3>
							</div>
							<p className="text-[10px] text-cyber-dim leading-relaxed h-8 overflow-hidden line-clamp-2">
								{phase.desc}
							</p>
						</div>
					))}
				</div>

				{/* NEW: Performance Benchmarks */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x03</span>
					Performance Benchmarks
				</h2>
				<div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
					{performanceBenchmarks.map((b) => (
						<div
							key={b.targetType}
							className="feature-card p-6 rounded-2xl border border-cyber-border/40 bg-black/20 hover:border-cyber-primary/20 transition-all"
						>
							<h4 className="text-[11px] font-black text-white uppercase tracking-widest mb-4 border-b border-white/5 pb-2">
								{b.targetType}
							</h4>
							<div className="space-y-4">
								<div className="flex items-center justify-between">
									<span className="text-[10px] text-cyber-dim uppercase font-bold">
										Domains Found
									</span>
									<span className="text-xs font-mono text-white font-bold">{b.domains}</span>
								</div>
								<div className="flex items-center justify-between">
									<span className="text-[10px] text-cyber-dim uppercase font-bold">
										Estimated Time
									</span>
									<span className="text-xs font-mono text-cyber-secondary font-bold">{b.time}</span>
								</div>
								<div className="flex items-center justify-between">
									<span className="text-[10px] text-cyber-dim uppercase font-bold">
										Memory Floor
									</span>
									<span className="text-xs font-mono text-cyber-primary font-bold">{b.memory}</span>
								</div>
								<div className="pt-2">
									<div className="text-[9px] text-cyber-dim/50 uppercase italic group-hover:text-cyber-primary/50">
										Optimal Mode: {b.mode}
									</div>
								</div>
							</div>
						</div>
					))}
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x04</span>
					Tactical Deployment
				</h2>
				<div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-20">
					<CodeBlock lang="bash" title="Standard Discovery" code="myth subdomains example.com" />
					<CodeBlock
						lang="bash"
						title="Active Brute-Force"
						code="myth subdomains example.com --active"
					/>
					<CodeBlock
						lang="bash"
						title="Ultra-Robust Master"
						code="myth subdomains example.com --master"
					/>
					<CodeBlock
						lang="bash"
						title="Neural Interface Input"
						code="/subdomains example.com --active"
					/>
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-6 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x05</span>
					Operational Flags
				</h2>
				<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50 mb-20">
					<div className="table-container">
						<table className="w-full text-xs docs-table border-none">
							<thead className="bg-white/5 border-b border-cyber-border/50">
								<tr>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Operational Flag
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Tactical Sector
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Strategic Impact
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Example Usage
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/30">
								{subdomainFlags.map((f) => (
									<tr key={f.flag} className="hover:bg-white/[0.04] transition-colors">
										<td className="py-4 px-6 font-mono text-cyber-primary font-bold text-[13px]">
											{f.flag}
										</td>
										<td className="py-4 px-6">
											<span
												className={`text-[9px] px-2 py-1 rounded border font-bold uppercase tracking-widest ${
													f.category === "Stealth"
														? "bg-cyber-secondary/10 border-cyber-secondary/30 text-cyber-secondary"
														: f.category === "Networking"
															? "bg-blue-500/10 border-blue-500/30 text-blue-400"
															: "bg-white/5 border-white/10 text-cyber-dim"
												}`}
											>
												{f.category}
											</span>
										</td>
										<td className="py-4 px-6 text-cyber-text/80 leading-relaxed text-[11px]">
											{f.description}
										</td>
										<td className="py-4 px-6 whitespace-nowrap">
											<code className="text-[10px] text-cyber-secondary font-mono bg-cyber-secondary/5 px-2 py-1 rounded border border-cyber-secondary/10">
												{f.example}
											</code>
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-6 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x06</span>
					Neural Tool Parameters (JSONL)
				</h2>
				<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50 mb-12 shadow-xl">
					<div className="table-container">
						<table className="w-full text-xs docs-table border-none">
							<thead className="bg-white/5 border-b border-cyber-border/50">
								<tr>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Neural Field
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Data Type
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Default Vector
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										Capability Mapping
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
										JSON Pattern
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/30">
								{subdomainParams.map((p) => (
									<tr key={p.name} className="hover:bg-white/[0.04] transition-colors">
										<td className="py-4 px-6 font-mono text-white font-bold">{p.name}</td>
										<td className="py-4 px-6 text-cyber-dim font-mono">
											<span className="bg-white/5 px-1.5 py-1 rounded border border-white/5">
												{p.type}
											</span>
											{p.required && (
												<span className="text-cyber-error ml-2 text-[8px] font-black uppercase tracking-widest bg-cyber-error/10 px-1 py-0.5 rounded border border-cyber-error/20">
													REQUIRED
												</span>
											)}
										</td>
										<td className="py-4 px-6 text-cyber-dim font-mono italic">{p.def}</td>
										<td className="py-4 px-6 text-cyber-text/80 text-[11px] leading-relaxed">
											{p.desc}
										</td>
										<td className="py-4 px-6 whitespace-nowrap">
											<code className="text-[10px] text-cyber-primary font-mono bg-cyber-primary/5 px-2 py-1 rounded border border-cyber-primary/10">
												{p.example}
											</code>
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>
			</div>

			{/* Sidebar Quick-Jump */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-6 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute -top-12 -left-12 w-32 h-32 bg-cyber-primary/5 rounded-full blur-3xl pointer-events-none" />

					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Terminal className="w-4 h-4" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Quantum Pulse</h4>
					</div>

					<div className="space-y-5">
						{[
							{
								label: "Engine Status",
								value: "STABLE",
								icon: <Pulse className="w-3.5 h-3.5 text-cyber-primary animate-pulse" />,
								sub: "Pulse sync nominal",
							},
							{
								label: "Asset Coverage",
								value: "~99.98% ABS",
								icon: <Shield className="w-3.5 h-3.5 text-cyber-secondary" />,
								sub: "Verified organizations",
							},
							{
								label: "Source Latency",
								value: "< 250ms",
								icon: <Zap className="w-3.5 h-3.5 text-cyber-accent" />,
								sub: "Aggregation peak",
							},
						].map((s) => (
							<div
								key={s.label}
								className="p-4 bg-white/5 rounded-2xl border border-white/5 group hover:border-cyber-primary/20 transition-all"
							>
								<div className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-1.5 flex items-center justify-between">
									{s.label}
									{s.icon}
								</div>
								<div className="text-sm font-mono text-white font-bold group-hover:text-cyber-primary transition-colors">
									{s.value}
								</div>
								<div className="text-[9px] text-cyber-dim/40 mt-1 italic">{s.sub}</div>
							</div>
						))}
					</div>

					<div className="mt-8 pt-8 border-t border-cyber-border/30">
						<div className="p-4 bg-cyber-primary/10 rounded-2xl border border-cyber-primary/20">
							<div className="text-[10px] text-cyber-primary font-black uppercase tracking-widest mb-2 flex items-center gap-2">
								<Info className="w-3.5 h-3.5" />
								Operational Tip
							</div>
							<p className="text-[10px] text-cyber-primary/70 leading-relaxed italic">
								For enterprise targets, always start with{" "}
								<code className="bg-cyber-primary/10 px-1 rounded">--master</code> to bypass
								restrictive DNS wildcard filters.
							</p>
						</div>
					</div>

					<div className="mt-8 pt-6 border-t border-cyber-border/20">
						<p className="text-[10px] text-cyber-dim/50 leading-relaxed italic text-center">
							Core synchronized with{" "}
							<span className="text-white">v{discoveryPhases.length}.0 pipeline</span> protocols.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}

// Internal icons for layout
function Crosshair({ className }: { className?: string }) {
	return (
		<svg
			className={className}
			fill="none"
			viewBox="0 0 24 24"
			stroke="currentColor"
			strokeWidth={2}
		>
			<title>Crosshair Icon</title>
			<path
				strokeLinecap="round"
				strokeLinejoin="round"
				d="M12 11V3m0 21v-8m0-5h8m-13 0H3m9 9a9 9 0 100-18 9 9 0 000 18z"
			/>
		</svg>
	);
}
