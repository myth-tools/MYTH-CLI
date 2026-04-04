import {
	Activity,
	AlertCircle,
	BarChart3,
	Box,
	Check,
	CheckCircle2,
	ChevronDown,
	ChevronRight,
	ChevronUp,
	Copy,
	Cpu,
	Download,
	Globe,
	History,
	Info,
	Loader2,
	Monitor,
	Search,
	Shield,
	Smartphone,
	Terminal,
	Zap,
} from "lucide-react";
import React, { useEffect, useState } from "react";
import { PageHeader } from "../components/Layout";

interface VersionInfo {
	os: string;
	platform: string;
	version: string;
	arch: string;
	display_arch: string;
	size: number;
	size_human: string;
	date: number;
	date_human: string;
	description: string;
	maintainer: string;
	section: string;
	filename: string;
	path: string;
	sha256: string;
	verify_cmd: string;
	install_cmd: string;
}

const getPlatformIcon = (platform: string) => {
	if (platform.includes("Termux")) {
		return <Smartphone className="w-5 h-5" />;
	}
	if (platform.includes("Arch")) {
		return <Cpu className="w-5 h-5" />;
	}
	if (platform.includes("Debian")) {
		return <Monitor className="w-5 h-5" />;
	}
	if (platform.includes("Fedora")) {
		return <Shield className="w-5 h-5" />;
	}
	return <Globe className="w-5 h-5" />;
};

const getPlatformColor = (os: string, platform: string) => {
	if (platform.includes("Termux")) {
		return "from-cyber-primary/20 to-cyber-primary/5 border-cyber-primary/20 text-cyber-primary";
	}
	if (os === "debian") {
		return "from-cyber-error/20 to-cyber-error/5 border-cyber-error/20 text-cyber-error";
	}
	if (os === "fedora") {
		return "from-cyber-secondary/20 to-cyber-secondary/5 border-cyber-secondary/20 text-cyber-secondary";
	}
	if (os === "arch") {
		return "from-cyber-accent/20 to-cyber-accent/5 border-cyber-accent/20 text-cyber-accent";
	}
	return "from-white/10 to-white/5 border-white/20 text-white";
};

interface VersionCardProps {
	v: VersionInfo;
	latestTimestamp: number;
	filter: string;
	expandedId: string | null;
	setExpandedId: (id: string | null) => void;
	copiedId: string | null;
	copyToClipboard: (text: string, vId: string) => void;
	pagesUrl: string;
}

const VersionCard = React.memo(
	({
		v,
		latestTimestamp,
		filter,
		expandedId,
		setExpandedId,
		copiedId,
		copyToClipboard,
		pagesUrl,
	}: VersionCardProps) => {
		const currentId = `${v.os}-${v.version}-${v.arch}`;
		const isExpanded = expandedId === currentId;

		return (
			<div
				className={`feature-card border-l-4 transition-all hover:translate-x-2 ${
					v.date === latestTimestamp && filter === "all"
						? "border-cyber-primary ring-1 ring-cyber-primary/30 bg-cyber-primary/[0.03]"
						: "border-cyber-border bg-white/[0.01]"
				} rounded-[2rem] p-8 relative overflow-hidden group`}
			>
				{v.date === latestTimestamp && filter === "all" && (
					<div className="absolute top-6 right-8">
						<div className="flex items-center gap-2 bg-cyber-primary text-black px-4 py-1.5 rounded-full text-[10px] font-black uppercase tracking-widest shadow-[0_0_25px_#00ffa399] animate-pulse">
							<CheckCircle2 className="w-3.5 h-3.5" />
							PRIME STABLE
						</div>
					</div>
				)}

				<div className="flex flex-col xl:flex-row xl:items-center justify-between gap-8 relative z-10">
					<div className="flex-1">
						<div className="flex items-center gap-4 mb-4">
							<div className="h-12 w-12 rounded-xl bg-white/5 border border-white/5 flex items-center justify-center font-black italic text-cyber-primary shadow-inner group-hover:scale-110 transition-transform">
								{getPlatformIcon(v.platform)}
							</div>
							<div>
								<div className="flex items-center gap-3">
									<span className="text-3xl font-black text-white tracking-tighter">
										v{v.version}
									</span>
									<span
										className={`px-3 py-1 text-[9px] font-black rounded-lg uppercase tracking-widest border bg-gradient-to-br ${getPlatformColor(v.os, v.platform)}`}
									>
										{v.platform}
									</span>
								</div>
								<div className="flex flex-wrap items-center gap-4 mt-2">
									<p className="text-[10px] text-cyber-dim/80 font-black uppercase tracking-widest flex items-center gap-2">
										<span
											className={`w-1.5 h-1.5 rounded-full animate-pulse ${v.os === "debian" ? "bg-cyber-error" : v.os === "fedora" ? "bg-cyber-secondary" : "bg-cyber-primary"}`}
										/>
										{v.display_arch}
									</p>
									<p className="text-[10px] text-cyber-dim/60 font-mono">RELEASE: {v.date_human}</p>
								</div>
							</div>
						</div>

						<p className="text-sm text-cyber-text/80 mb-6 leading-relaxed border-l-2 border-white/10 pl-6 py-2 italic font-medium bg-white/[0.02] rounded-r-xl">
							{v.description || "No tactical manifest provided for this release cycle."}
						</p>

						<div className="grid grid-cols-1 sm:grid-cols-4 gap-6 pt-4 border-t border-white/5">
							<div className="flex items-center gap-3 group/stat">
								<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-primary transition-colors">
									<Box className="w-4 h-4" />
								</div>
								<div>
									<div className="text-[9px] text-cyber-dim uppercase font-black">Payload Size</div>
									<div className="text-xs font-mono text-white">{v.size_human}</div>
								</div>
							</div>
							<div className="flex items-center gap-3 group/stat">
								<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-secondary transition-colors">
									<Shield className="w-4 h-4" />
								</div>
								<div>
									<div className="text-[9px] text-cyber-dim uppercase font-black">Signatory</div>
									<div className="text-xs font-mono text-white truncate max-w-[100px]">
										{v.maintainer.split(" <")[0]}
									</div>
								</div>
							</div>
							<div className="flex items-center gap-3 group/stat">
								<div className="w-8 h-8 rounded-lg bg-white/5 flex items-center justify-center text-cyber-dim group-hover/stat:text-cyber-accent transition-colors">
									<Terminal className="w-4 h-4" />
								</div>
								<div>
									<div className="text-[9px] text-cyber-dim uppercase font-black">Suite</div>
									<div className="text-xs font-mono text-white uppercase">{v.section}</div>
								</div>
							</div>

							{/* Integrity HUD Toggle */}
							<button
								type="button"
								onClick={() => setExpandedId(isExpanded ? null : currentId)}
								className={`flex items-center justify-between px-4 py-2 rounded-xl border transition-all ${
									isExpanded
										? "bg-cyber-primary/10 border-cyber-primary/30 text-cyber-primary"
										: "bg-white/5 border-white/5 text-cyber-dim hover:border-white/20"
								}`}
							>
								<div className="flex items-center gap-2">
									<Zap className="w-3.5 h-3.5" />
									<span className="text-[10px] font-black uppercase">Inspect</span>
								</div>
								{isExpanded ? (
									<ChevronUp className="w-3.5 h-3.5" />
								) : (
									<ChevronDown className="w-3.5 h-3.5" />
								)}
							</button>
						</div>

						{/* Expandable Verification HUD */}
						{isExpanded && (
							<div className="mt-6 p-6 bg-black/40 rounded-3xl border border-cyber-primary/20 space-y-4 animate-in fade-in slide-in-from-top-2 duration-300">
								<div>
									<div className="text-[9px] text-cyber-primary uppercase font-black mb-2 flex items-center gap-2">
										<Shield className="w-3 h-3" /> SHA-256 Fingerprint
									</div>
									<div className="flex items-center gap-3 bg-white/5 p-3 rounded-xl border border-white/5 group/copy">
										<code className="text-[10px] font-mono text-white break-all flex-1">
											{v.sha256}
										</code>
										<button
											type="button"
											onClick={() => copyToClipboard(v.sha256, `sha-${currentId}`)}
											className="shrink-0 p-2 hover:bg-white/10 rounded-lg transition-colors"
										>
											{copiedId === `sha-${currentId}` ? (
												<Check className="w-4 h-4 text-cyber-primary" />
											) : (
												<Copy className="w-4 h-4 text-cyber-dim group-hover/copy:text-cyber-primary" />
											)}
										</button>
									</div>
								</div>

								<div>
									<div className="text-[9px] text-cyber-primary uppercase font-black mb-2 flex items-center gap-2">
										<Terminal className="w-3 h-3" /> Terminal Verification
									</div>
									<div className="flex items-center gap-3 bg-white/5 p-3 rounded-xl border border-white/5 group/copy">
										<code className="text-[10px] font-mono text-cyber-dim break-all flex-1">
											{v.verify_cmd}
										</code>
										<button
											type="button"
											onClick={() => copyToClipboard(v.verify_cmd, `cmd-${currentId}`)}
											className="shrink-0 p-2 hover:bg-white/10 rounded-lg transition-colors"
										>
											{copiedId === `cmd-${currentId}` ? (
												<Check className="w-4 h-4 text-cyber-primary" />
											) : (
												<Copy className="w-4 h-4 text-cyber-dim group-hover/copy:text-cyber-primary" />
											)}
										</button>
									</div>
								</div>

								<div>
									<div className="text-[9px] text-cyber-secondary uppercase font-black mb-2 flex items-center gap-2">
										<Zap className="w-3 h-3" /> Installation Command
									</div>
									<div className="flex items-center gap-3 bg-white/5 p-3 rounded-xl border border-white/5 group/copy">
										<code className="text-[10px] font-mono text-cyber-dim break-all flex-1">
											{v.install_cmd}
										</code>
										<button
											type="button"
											onClick={() => copyToClipboard(v.install_cmd, `inst-${currentId}`)}
											className="shrink-0 p-2 hover:bg-white/10 rounded-lg transition-colors"
										>
											{copiedId === `inst-${currentId}` ? (
												<Check className="w-4 h-4 text-cyber-secondary" />
											) : (
												<Copy className="w-4 h-4 text-cyber-dim group-hover/copy:text-cyber-secondary" />
											)}
										</button>
									</div>
								</div>
							</div>
						)}
					</div>

					<div className="shrink-0 flex flex-col gap-3">
						<button
							type="button"
							onClick={() => {
								window.open(`${pagesUrl}/${v.path}`, "_blank");
							}}
							className="w-full xl:w-56 px-8 py-3 text-[11px] font-black uppercase tracking-[0.2em] bg-cyber-primary text-black hover:bg-white transition-all rounded-xl flex items-center justify-center gap-3 group/btn shadow-lg shadow-cyber-primary/20"
						>
							<Download className="w-4 h-4" />
							Download Binary
						</button>
						<button
							type="button"
							onClick={() => {
								window.location.href = "#/installation";
							}}
							className="w-full xl:w-56 px-8 py-3 text-[11px] font-black uppercase tracking-[0.2em] bg-white/5 border border-white/10 hover:bg-white/10 text-white transition-all rounded-xl flex items-center justify-center gap-3 group/btn"
						>
							Deploy Instantly
							<ChevronRight className="w-4 h-4 group-hover/btn:translate-x-1 transition-transform" />
						</button>
					</div>
				</div>
			</div>
		);
	},
);
VersionCard.displayName = "VersionCard";

export default function VersionsPage() {
	const [versions, setVersions] = useState<VersionInfo[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);
	const [filter, setFilter] = useState("all");
	const [searchQuery, setSearchQuery] = useState("");
	const [expandedId, setExpandedId] = useState<string | null>(null);
	const [copiedId, setCopiedId] = useState<string | null>(null);

	const pagesUrl = import.meta.env.VITE_PAGES_URL || "https://myth.work.gd";

	useEffect(() => {
		async function fetchVersions() {
			try {
				const response = await fetch("./versions.json");
				if (!response.ok) {
					throw new Error("Neural registry manifest unreachable.");
				}
				const data: VersionInfo[] = await response.json();

				const sorted = data.sort((a, b) => b.date - a.date);

				setVersions(sorted);
			} catch (err) {
				setError(err instanceof Error ? err.message : "Unknown technical error");
			} finally {
				setLoading(false);
			}
		}

		fetchVersions();
	}, []); // Removed pagesUrl to fix unnecessary renders

	const copyToClipboard = (text: string, vId: string) => {
		navigator.clipboard.writeText(text);
		setCopiedId(vId);
		setTimeout(() => setCopiedId(null), 2000);
	};

	// Industry-Grade Filtering & Grouping Engine
	const filteredVersions = React.useMemo(() => {
		const result = versions
			.filter((v) => (filter === "all" ? true : v.os === filter))
			.filter((v) =>
				searchQuery
					? v.version.toLowerCase().includes(searchQuery.toLowerCase()) ||
						v.platform.toLowerCase().includes(searchQuery.toLowerCase()) ||
						v.display_arch.toLowerCase().includes(searchQuery.toLowerCase())
					: true,
			);
		return result;
	}, [versions, filter, searchQuery]);

	// Dynamic "PRIME STABLE" Sentinel
	const latestTimestamp = React.useMemo(
		() => Math.max(...versions.map((v) => v.date), 0),
		[versions],
	);

	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Tactical Version Registry"
					description="Live mission history synchronized directly with the primary MYTH decentralized repository. Real-time telemetry on every release vector."
					badge="Mission History"
				/>

				{/* Version Lifecycle Matrix */}
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
					<div className="flex flex-col gap-6 mb-8">
						<div className="flex flex-col md:flex-row md:items-center justify-between gap-6">
							<div className="flex items-center gap-3 text-cyber-primary">
								<History className="w-5 h-5" />
								<h2 className="text-xl font-black text-white uppercase tracking-tighter">
									Availability Stream
								</h2>
							</div>

							{/* Search Bar */}
							<div className="relative group flex-1 max-w-md">
								<Search className="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-cyber-dim group-focus-within:text-cyber-primary transition-colors" />
								<input
									type="text"
									placeholder="Search Registry (Version, Platform, Arch)..."
									value={searchQuery}
									onChange={(e) => setSearchQuery(e.target.value)}
									className="w-full bg-white/5 border border-white/10 rounded-2xl py-3 pl-12 pr-4 text-xs text-white placeholder:text-cyber-dim focus:outline-none focus:border-cyber-primary/50 focus:ring-4 focus:ring-cyber-primary/5 transition-all"
								/>
							</div>

							<div className="flex flex-wrap items-center gap-2 bg-white/5 p-1.5 rounded-2xl border border-white/10">
								{["all", "debian", "fedora", "arch"].map((f) => (
									<button
										key={f}
										type="button"
										onClick={() => setFilter(f)}
										className={`px-4 py-1.5 rounded-xl text-[10px] font-black uppercase tracking-widest transition-all ${
											filter === f
												? "bg-cyber-primary text-black shadow-[0_0_15px_#00ffa366]"
												: "text-cyber-dim hover:text-white hover:bg-white/5"
										}`}
									>
										{f}
									</button>
								))}
							</div>
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
							{filteredVersions.map((v) => (
								<VersionCard
									key={`${v.os}-${v.version}-${v.arch}-${v.filename}`}
									v={v}
									latestTimestamp={latestTimestamp}
									filter={filter}
									expandedId={expandedId}
									setExpandedId={setExpandedId}
									copiedId={copiedId}
									copyToClipboard={copyToClipboard}
									pagesUrl={pagesUrl}
								/>
							))}
						</div>
					)}
				</div>
			</div>

			{/* Sidebar - Progress Checklist */}
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
							<div className="text-xs font-mono text-white">
								{versions[0]?.os.toUpperCase() || "N/A"}_{versions[0]?.arch.toUpperCase() || "N/A"}
							</div>
							<p className="text-[9px] text-cyber-dim/50 mt-1 italic">
								Updated:{" "}
								{versions[0]
									? new Date(versions[0].date * 1000).toLocaleDateString()
									: new Date().toLocaleDateString()}
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
