import {
	Activity,
	AlertCircle,
	BarChart3,
	Box,
	CheckCircle2,
	ChevronRight,
	History,
	Info,
	Loader2,
	Shield,
	Terminal,
	Zap,
} from "lucide-react";
import { useEffect, useState } from "react";
import { PageHeader } from "../components/Layout";

interface VersionInfo {
	version: string;
	architecture: string;
	size: string;
	description: string;
	maintainer: string;
	section: string;
}

export default function VersionsPage() {
	const [versions, setVersions] = useState<VersionInfo[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);

	const pagesUrl = import.meta.env.VITE_PAGES_URL || "https://myth.work.gd";

	useEffect(() => {
		async function fetchVersions() {
			try {
				const response = await fetch(`${pagesUrl}/dists/stable/main/binary-amd64/Packages`);
				if (!response.ok) {
					throw new Error("Neural registry unreachable. Repository may be offline.");
				}
				const text = await response.text();

				const packageBlocks = text.split("\n\n");
				const parsedVersions: VersionInfo[] = packageBlocks
					.map((block) => {
						const lines = block.split("\n");
						const getField = (name: string) => {
							const line = lines.find((l) => l.startsWith(`${name}:`));
							return line ? line.split(": ")[1] : "";
						};

						return {
							version: getField("Version"),
							architecture: getField("Architecture"),
							size: getField("Size"),
							description: getField("Description"),
							maintainer: getField("Maintainer"),
							section: getField("Section"),
						};
					})
					.filter((v) => v.version);

				const sorted = parsedVersions.sort((a, b) => {
					return b.version.localeCompare(a.version, undefined, {
						numeric: true,
						sensitivity: "base",
					});
				});

				setVersions(sorted);
			} catch (err) {
				setError(err instanceof Error ? err.message : "Unknown technical error");
			} finally {
				setLoading(false);
			}
		}

		fetchVersions();
	}, []);

	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Tactical Version Registry"
					description="Live mission history synchronized directly with the primary MYTH decentralized repository. Real-time telemetry on every release vector."
					badge="Mission History"
				/>

				{/* NEW: Version Lifecycle Matrix */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-8">
						<BarChart3 className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Operational Stability Lifecycle
						</h2>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-3 gap-6">
						{[
							{
								label: "Stable Channel",
								status: "NOMINAL",
								desc: "Hardened releases. Optimized for mission-critical operations and long-running reconnaissance.",
								color: "border-cyber-primary/30 text-cyber-primary",
							},
							{
								label: "Rolling Channel",
								status: "ACTIVE",
								desc: "Live-synced with Kali Linux rolling updates. Features the latest tactical primitives.",
								color: "border-cyber-secondary/30 text-cyber-secondary",
							},
							{
								label: "Deep-Bleed",
								status: "UNSTABLE",
								desc: "Experimental builds. Pure research and development. Not recommended for live field ops.",
								color: "border-cyber-error/20 text-cyber-error",
							},
						].map((s) => (
							<div
								key={s.label}
								className={`p-6 rounded-3xl border ${s.color} bg-black/40 group relative overflow-hidden backdrop-blur-sm`}
							>
								<div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
									<Activity className="w-16 h-16" />
								</div>
								<div className="text-[10px] font-black uppercase tracking-[0.2em] mb-4 opacity-80">
									{s.label}
								</div>
								<div className="text-[11px] font-black italic tracking-tighter mb-3 flex items-center gap-2">
									<div className="w-1.5 h-1.5 rounded-full bg-current shadow-[0_0_8px_currentColor]" />
									{s.status}
								</div>
								<p className="text-[11px] text-cyber-dim leading-relaxed">{s.desc}</p>
							</div>
						))}
					</div>
				</div>

				<div className="space-y-6">
					<div className="flex items-center justify-between mb-4">
						<div className="flex items-center gap-3 text-cyber-primary">
							<History className="w-5 h-5" />
							<h2 className="text-xl font-black text-white uppercase tracking-tighter">
								Availability Stream
							</h2>
						</div>
						<div className="text-[10px] font-mono text-cyber-dim uppercase tracking-widest bg-white/5 px-3 py-1 rounded-full border border-white/10">
							{versions.length} ACTIVE DEPLOYMENTS
						</div>
					</div>

					{loading ? (
						<div className="glass-panel py-32 rounded-[2.5rem] border border-cyber-border/40 flex flex-col items-center justify-center gap-6 shadow-2xl">
							<div className="relative">
								<Loader2 className="w-16 h-16 text-cyber-primary animate-spin" />
								<Zap className="w-6 h-6 text-cyber-primary absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 animate-pulse" />
							</div>
							<div className="space-y-1 text-center">
								<p className="text-white font-black italic tracking-[0.2em] text-sm">
									SYNCHRONIZING REGISTRY...
								</p>
								<p className="text-[10px] text-cyber-dim font-mono uppercase">
									Interrogating Universal Linux Manifest
								</p>
							</div>
						</div>
					) : error ? (
						<div className="glass-panel p-10 rounded-[2.5rem] border border-cyber-error/30 bg-cyber-error/5 text-center">
							<AlertCircle className="w-12 h-12 text-cyber-error mx-auto mb-4" />
							<h3 className="text-lg font-bold text-white mb-2 uppercase tracking-widest">
								Registry Error
							</h3>
							<p className="text-sm text-cyber-error/80 mb-6 font-mono">{error}</p>
							<button
								type="button"
								onClick={() => window.location.reload()}
								className="px-6 py-2 bg-cyber-error text-black font-black uppercase text-xs tracking-widest rounded-lg shadow-lg hover:bg-white transition-all"
							>
								Retry Handshake
							</button>
						</div>
					) : (
						<div className="grid gap-6">
							{versions.map((v, i) => (
								<div
									key={v.version}
									className={`feature-card border-l-4 transition-all hover:translate-x-2 ${
										i === 0
											? "border-cyber-primary ring-1 ring-cyber-primary/30 bg-cyber-primary/[0.03]"
											: "border-cyber-border bg-white/[0.01]"
									} rounded-[2rem] p-8 relative overflow-hidden group`}
								>
									{i === 0 && (
										<div className="absolute top-6 right-8">
											<div className="flex items-center gap-2 bg-cyber-primary text-black px-4 py-1 rounded-full text-[10px] font-black uppercase tracking-widest shadow-[0_0_15px_#00ffa366]">
												<CheckCircle2 className="w-3.5 h-3.5" />
												PRIMARY STABLE
											</div>
										</div>
									)}

									<div className="flex flex-col xl:flex-row xl:items-center justify-between gap-8 relative z-10">
										<div className="flex-1">
											<div className="flex items-center gap-4 mb-4">
												<div className="h-10 w-10 rounded-xl bg-white/5 border border-white/5 flex items-center justify-center font-black italic text-cyber-primary text-sm shadow-inner group-hover:scale-110 transition-transform">
													V{i + 1}
												</div>
												<div>
													<div className="flex items-center gap-3">
														<span className="text-3xl font-black text-white tracking-tighter">
															v{v.version}
														</span>
														<span className="px-2 py-0.5 text-[9px] font-black bg-white/10 text-cyber-dim rounded uppercase tracking-widest border border-white/5">
															{v.architecture}
														</span>
													</div>
													<p className="text-xs text-cyber-dim/60 font-mono mt-1">
														ID: BUILD-{v.version.replace(".", "-")}-RECON
													</p>
												</div>
											</div>

											<p className="text-sm text-cyber-text/80 mb-6 leading-relaxed border-l-2 border-white/5 pl-4 py-1 italic">
												{v.description || "No tactical manifest provided for this release cycle."}
											</p>

											<div className="grid grid-cols-1 sm:grid-cols-3 gap-6 pt-4 border-t border-white/5">
												<div className="flex items-center gap-3 group/stat">
													<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-primary transition-colors">
														<Box className="w-4 h-4" />
													</div>
													<div>
														<div className="text-[9px] text-cyber-dim uppercase font-black">
															Binary Size
														</div>
														<div className="text-xs font-mono text-white">
															{(Number.parseInt(v.size, 10) / 1024 / 1024).toFixed(2)} MB
														</div>
													</div>
												</div>
												<div className="flex items-center gap-3 group/stat">
													<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-secondary transition-colors">
														<Shield className="w-4 h-4" />
													</div>
													<div>
														<div className="text-[9px] text-cyber-dim uppercase font-black">
															Maintainer
														</div>
														<div className="text-xs font-mono text-white truncate max-w-[120px]">
															{v.maintainer.split(" <")[0]}
														</div>
													</div>
												</div>
												<div className="flex items-center gap-3 group/stat">
													<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-accent transition-colors">
														<Terminal className="w-4 h-4" />
													</div>
													<div>
														<div className="text-[9px] text-cyber-dim uppercase font-black">
															Mission Suite
														</div>
														<div className="text-xs font-mono text-white uppercase">
															{v.section}
														</div>
													</div>
												</div>
											</div>
										</div>

										<div className="shrink-0">
											<button
												type="button"
												onClick={() => {
													window.location.href = "#/installation";
												}}
												className="w-full xl:w-auto px-8 py-3 text-[11px] font-black uppercase tracking-[0.2em] bg-white/5 border border-white/10 hover:bg-cyber-primary hover:text-black hover:border-cyber-primary transition-all rounded-xl flex items-center justify-center gap-3 group/btn shadow-xl ring-0 hover:ring-8 ring-cyber-primary/5"
											>
												Initialize Deployment
												<ChevronRight className="w-4 h-4 group-hover/btn:translate-x-1 transition-transform" />
											</button>
										</div>
									</div>
								</div>
							))}
						</div>
					)}
				</div>
			</div>

			{/* Sidebar - NEW: Progress Checklist */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-[2.5rem] p-8 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 right-0 w-24 h-24 bg-cyber-primary/5 rounded-full blur-3xl" />
					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Info className="w-5 h-5" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Registry Pulse</h4>
					</div>

					<div className="space-y-6 mb-10">
						<div className="p-4 bg-white/5 rounded-2xl border border-white/5 group hover:border-cyber-primary/20 transition-all cursor-default">
							<div className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-1.5">
								Registry Integrity
							</div>
							<div className="text-sm font-mono text-white font-bold flex items-center gap-2">
								<div className="w-1.5 h-1.5 rounded-full bg-cyber-primary animate-pulse" />
								STABLE-SYNC
							</div>
						</div>

						<div className="p-4 bg-white/5 rounded-2xl border border-white/5 group hover:border-cyber-secondary/20 transition-all cursor-default">
							<div className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-1.5">
								Last Manifest
							</div>
							<div className="text-xs font-mono text-white">AMD64_STABLE</div>
							<p className="text-[9px] text-cyber-dim/50 mt-1 italic italic">
								Updated: {new Date().toLocaleDateString()}
							</p>
						</div>
					</div>

					<div className="p-6 bg-cyber-primary/5 rounded-3xl border border-cyber-primary/20">
						<h5 className="text-[10px] font-black text-cyber-primary uppercase tracking-[0.2em] mb-3">
							Verification Note
						</h5>
						<p className="text-[10px] text-cyber-primary/70 leading-relaxed italic">
							Every release is SHA-256 fingerprinted and GPG signed. The native registries ensure
							immutable delivery of these primitives across all Linux families.
						</p>
					</div>

					<div className="mt-12 pt-8 border-t border-cyber-border/20 text-center">
						<p className="text-[10px] text-cyber-dim/50 italic leading-relaxed">
							Decentralized version registry operating at{" "}
							<span className="text-white">v{versions.length}.0</span> parity.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
