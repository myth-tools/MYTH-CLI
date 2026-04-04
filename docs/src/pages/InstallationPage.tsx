import {
	Activity,
	AlertTriangle,
	Box,
	CheckCircle2,
	ChevronRight,
	Cpu,
	HardDrive,
	Info,
	Monitor,
	Network,
	Shield,
	Terminal,
	Zap,
} from "lucide-react";
import { useState } from "react";
import { CodeBlock, PageHeader } from "../components/Layout";

const methods = [
	{
		id: "oneliner",
		label: "One-Line Installer",
		icon: <Zap className="w-4 h-4" />,
		badge: "Recommended",
	},
	{
		id: "apt",
		label: "Native Linux",
		icon: <Network className="w-4 h-4" />,
		badge: "System Pkg",
	},
	{ id: "binary", label: "Pre-Built Binary", icon: <Box className="w-4 h-4" />, badge: "Fastest" },
	{
		id: "npm",
		label: "NPM / Node.js",
		icon: <Activity className="w-4 h-4" />,
		badge: "JavaScript",
	},
	{ id: "pypi", label: "PyPI / Python", icon: <Activity className="w-4 h-4" />, badge: "Python" },
	{
		id: "docker",
		label: "Docker Core",
		icon: <HardDrive className="w-4 h-4" />,
		badge: "Container",
	},
	{ id: "snap", label: "Snap Store", icon: <Monitor className="w-4 h-4" />, badge: "Ubuntu" },
	{ id: "cargo", label: "Cargo Build", icon: <Cpu className="w-4 h-4" />, badge: "From Source" },
	{ id: "nix", label: "Nix Hermetic", icon: <Shield className="w-4 h-4" />, badge: "Immutability" },
];

const compatibilityMatrix = [
	{
		os: "Kali Linux / Debian",
		arch: "amd64 / arm64",
		status: "NATIVE (APT)",
		color: "text-cyber-primary",
		note: "Primary target OS. Full tool access guaranteed via signed APT repo.",
	},
	{
		os: "Fedora / RHEL",
		arch: "amd64 / arm64",
		status: "STABLE (RPM)",
		color: "text-blue-400",
		note: "Native package via RPM repo. Full bwrap and network support.",
	},
	{
		os: "Arch Linux / Manjaro",
		arch: "amd64 / arm64",
		status: "STABLE (PACMAN)",
		color: "text-teal-400",
		note: "Native .pkg.tar.zst via Arch repository integration.",
	},
	{
		os: "Termux / Local Android",
		arch: "arm64",
		status: "MOBILE TACTICAL",
		color: "text-purple-400",
		note: "Fully optimized for mobile execution. Dynamic prefix support.",
	},
	{
		os: "WSL2 (Ubuntu/Kali)",
		arch: "amd64",
		status: "SUPPORTED",
		color: "text-cyber-accent",
		note: "Requires systemd enabled for bwrap namespace support.",
	},
];

export default function InstallationPage() {
	const pagesUrl = import.meta.env.VITE_PAGES_URL || "https://myth.work.gd";
	const [active, setActive] = useState("oneliner");

	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Deployment & Activation"
					description="Deploy MYTH across 9 secure package ecosystems. Choose your vector — the binary identity is always `myth` regardless of the transport."
					badge="Mission Setup"
				/>

				{/* NEW: OS Compatibility Matrix */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-8">
						<Monitor className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Operational Compatibility Matrix
						</h2>
					</div>
					<div className="glass-panel rounded-3xl overflow-hidden border border-cyber-border/40 shadow-xl bg-black/20">
						<table className="w-full text-left text-xs docs-table border-none">
							<thead className="bg-white/5 border-b border-cyber-border/30">
								<tr>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
										Host Ecosystem
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
										Arch Map
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
										Uptime Status
									</th>
									<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
										Mission Notes
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/20">
								{compatibilityMatrix.map((c) => (
									<tr key={c.os} className="hover:bg-white/[0.02] transition-colors">
										<td className="py-5 px-6 font-bold text-white text-sm">{c.os}</td>
										<td className="py-5 px-6 text-cyber-dim font-mono">{c.arch}</td>
										<td className="py-5 px-6">
											<div
												className={`flex items-center gap-2 font-black italic tracking-tighter ${c.color}`}
											>
												<div className="w-1.5 h-1.5 rounded-full bg-current shadow-[0_0_8px_currentColor] animate-pulse" />
												{c.status}
											</div>
										</td>
										<td className="py-5 px-6 text-cyber-text/60 italic leading-relaxed text-[11px]">
											{c.note}
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x01</span>
					Deployment Vectors
				</h2>

				{/* Method tabs Grid */}
				<div className="grid grid-cols-2 sm:grid-cols-3 xl:grid-cols-5 gap-3 mb-8">
					{methods.map((m) => (
						<button
							key={m.id}
							type="button"
							onClick={() => setActive(m.id)}
							className={`p-4 rounded-2xl flex flex-col items-center gap-2 border transition-all group scale-100 hover:scale-[1.03] ${
								active === m.id
									? "bg-cyber-primary/10 border-cyber-primary/50 text-cyber-primary shadow-[0_0_20px_-5px_rgba(0,255,163,0.15)]"
									: "bg-white/[0.02] border-white/5 text-cyber-dim hover:border-cyber-primary/20"
							}`}
						>
							<div
								className={`w-8 h-8 rounded-xl flex items-center justify-center border transition-colors ${
									active === m.id
										? "bg-cyber-primary/20 border-cyber-primary/20"
										: "bg-black/40 border-white/5 group-hover:border-cyber-primary/30"
								}`}
							>
								{m.icon}
							</div>
							<div className="text-[10px] font-black uppercase text-center tracking-tight leading-tight">
								{m.label}
							</div>
						</button>
					))}
				</div>

				{/* Active Method Container */}
				<div className="glass-panel overflow-hidden rounded-3xl border border-cyber-border/40 mb-16 shadow-2xl relative">
					<div className="absolute top-0 right-0 p-8 opacity-[0.03] pointer-events-none">
						<Terminal className="w-48 h-48" />
					</div>

					<div className="p-8 relative z-10">
						<div className="flex items-center gap-4 mb-6">
							<div className="px-4 py-1.5 bg-cyber-primary text-black font-black uppercase text-[10px] tracking-[0.2em] rounded-full shadow-[0_0_15px_#00ffa366]">
								ACTIVE VECTOR: {methods.find((m) => m.id === active)?.label.toUpperCase()}
							</div>
							<div className="h-px flex-1 bg-gradient-to-r from-cyber-primary/30 to-transparent" />
						</div>

						{active === "oneliner" && (
							<div className="space-y-6">
								<p className="text-sm text-cyber-text/80 leading-relaxed max-w-3xl">
									The high-speed autonomous bootstrap. Verifies system integrity, signs GPG keys,
									enables the production APT registry, and interacts with the operative during final
									configuration.
								</p>
								<div className="grid gap-4">
									<CodeBlock
										lang="bash"
										title="Standard Deployment (Stable)"
										code={`curl -sSL ${pagesUrl}/install.sh | sudo bash`}
									/>
									<CodeBlock
										lang="bash"
										title="Specific Hardware Build"
										code={`curl -sSL ${pagesUrl}/install.sh | sudo VERSION=0.1.0 ARCH=arm64 bash`}
									/>
								</div>
								<div className="flex items-center gap-3 p-4 bg-cyber-primary/5 border border-cyber-primary/20 rounded-2xl">
									<Info className="w-5 h-5 text-cyber-primary shrink-0" />
									<p className="text-[11px] text-cyber-primary/80 italic leading-relaxed">
										Our script automatically executes an architecture check to ensure binary parity
										for your chipset (Intel, AMD, or ARM).
									</p>
								</div>
							</div>
						)}

						{active === "apt" && (
							<div className="space-y-6 text-sm">
								<p className="text-cyber-text/80 leading-relaxed font-bold italic">
									Industrial-grade native deployment. All tactical updates arrive directly via your
									system's primary package manager (APT, DNF, Pacman, or Pkg).
								</p>
								<div className="space-y-8">
									{/* APT Section */}
									<div className="space-y-4">
										<div className="flex items-center gap-2 text-cyber-primary">
											<ChevronRight className="w-4 h-4" />
											<span className="font-black uppercase tracking-widest text-[10px]">
												Vector: Debian / Ubuntu / Kali (APT)
											</span>
										</div>
										<CodeBlock
											lang="bash"
											title="Registry & Install"
											code={`curl -fsSL ${pagesUrl}/myth.gpg | sudo gpg --dearmor -o /etc/apt/keyrings/myth.gpg\necho "deb [signed-by=/etc/apt/keyrings/myth.gpg] ${pagesUrl} stable main" | sudo tee /etc/apt/sources.list.d/myth.list\nsudo apt update && sudo apt install myth`}
										/>
									</div>

									{/* DNF Section */}
									<div className="space-y-4 pt-4 border-t border-white/5">
										<div className="flex items-center gap-2 text-blue-400">
											<ChevronRight className="w-4 h-4" />
											<span className="font-black uppercase tracking-widest text-[10px]">
												Vector: Fedora / RHEL (DNF)
											</span>
										</div>
										<CodeBlock
											lang="bash"
											title="Registry & Install"
											code={`sudo tee /etc/yum.repos.d/myth.repo << REPOEOF\n[myth]\nname=MYTH Official Repository\nbaseurl=${pagesUrl}/rpm\nenabled=1\ngpgcheck=1\nrepo_gpgcheck=1\ngpgkey=${pagesUrl}/myth.gpg\ntype=rpm\nREPOEOF\n\nsudo dnf install myth`}
										/>
									</div>

									{/* Pacman Section */}
									<div className="space-y-4 pt-4 border-t border-white/5">
										<div className="flex items-center gap-2 text-teal-400">
											<ChevronRight className="w-4 h-4" />
											<span className="font-black uppercase tracking-widest text-[10px]">
												Vector: Arch Linux (Pacman)
											</span>
										</div>
										<CodeBlock
											lang="bash"
											title="Registry & Install"
											code={`sudo tee -a /etc/pacman.conf << ARCHEOF\n\n[myth]\nSigLevel = PackageOptional\nServer = ${pagesUrl}/arch\nARCHEOF\n\nsudo pacman -Syu myth`}
										/>
									</div>

									{/* Termux Section */}
									<div className="space-y-4 pt-4 border-t border-white/5">
										<div className="flex items-center gap-2 text-purple-400">
											<ChevronRight className="w-4 h-4" />
											<span className="font-black uppercase tracking-widest text-[10px]">
												Vector: Android (Termux / Pkg)
											</span>
										</div>
										<CodeBlock lang="bash" title="Direct Activation" code={`pkg install myth`} />
									</div>
								</div>
							</div>
						)}

						{active === "binary" && (
							<div className="space-y-6">
								<p className="text-sm text-cyber-text/80 leading-relaxed">
									Direct binary injection. Bypasses package managers for instant portability on
									air-gapped or ephemeral systems.
								</p>
								<CodeBlock
									lang="bash"
									title="Manual Binary Pull"
									code={`curl -fsSL "https://github.com/myth-tools/MYTH-CLI/releases/latest/download/myth-$(uname -m)-unknown-linux-gnu" -o myth && chmod +x myth && sudo mv myth /usr/local/bin/`}
								/>
							</div>
						)}

						{active === "npm" && (
							<div className="space-y-6">
								<p className="text-sm text-cyber-text/80 leading-relaxed font-mono">
									Channel: @myth-tools/myth
								</p>
								<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
									<CodeBlock
										lang="bash"
										title="Static Global"
										code="npm install -g @myth-tools/myth"
									/>
									<CodeBlock
										lang="bash"
										title="High-Speed Run"
										code="npx @myth-tools/myth scan <target>"
									/>
								</div>
							</div>
						)}

						{/* pypi, docker, snap, cargo, nix */}
						{["pypi", "docker", "snap", "cargo", "nix"].includes(active) && (
							<div className="py-20 text-center">
								<Activity className="w-12 h-12 text-cyber-primary/20 mx-auto mb-4 animate-pulse" />
								<p className="text-xs text-cyber-dim font-mono uppercase tracking-[0.3em]">
									Sector detail optimized for {active.toUpperCase()}
								</p>
								<p className="text-[10px] text-cyber-dim/50 mt-2 italic">
									Standardized CLI deployment paths active for this vector.
								</p>
								<div className="mt-8">
									<button
										type="button"
										onClick={() => setActive("oneliner")}
										className="text-cyber-primary text-[10px] font-bold uppercase tracking-widest border-b border-cyber-primary/30 hover:border-cyber-primary transition-all"
									>
										View Primary Vector →
									</button>
								</div>
							</div>
						)}
					</div>
				</div>

				{/* NEW: Troubleshooting / FAQ Section */}
				<div className="mb-16">
					<div className="flex items-center gap-3 mb-8">
						<AlertTriangle className="w-5 h-5 text-cyber-secondary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Common Deployment Hurdles
						</h2>
					</div>
					<div className="space-y-4">
						{[
							{
								q: "Execution Denied: bwrap error?",
								a: "MYTH requires unprivileged user namespaces. Enable systemd-sysctl or run in privileged containers for Docker/WSL2.",
							},
							{
								q: "GPG Key Verification Failure?",
								a: "Ensure you have the 'gnupg' package installed and are pulling from myth.work.gd. Use the one-liner for auto-sync.",
							},
							{
								q: "Tor Connectivity Latency?",
								a: "Native Tor requires the local 'tor' service to be active. Run 'sudo systemctl start tor' for L2 OPSEC support.",
							},
							{
								q: "Command 'myth' not found after install?",
								a: "Add /usr/local/bin or your package manager's bin path to your $PATH environment variable.",
							},
						].map((faq) => (
							<div
								key={faq.q}
								className="feature-card p-6 rounded-2xl border border-white/5 bg-white/[0.01] hover:bg-white/[0.03] transition-all"
							>
								<h3 className="text-sm font-bold text-cyber-secondary mb-2 flex items-start gap-3">
									<ChevronRight className="w-4 h-4 mt-0.5 shrink-0" />
									{faq.q}
								</h3>
								<p className="text-xs text-cyber-text/70 leading-relaxed pl-7 border-l border-white/5">
									{faq.a}
								</p>
							</div>
						))}
					</div>
				</div>

				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x02</span>
					Post-Deployment Sequence
				</h2>
				<div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-16">
					{[
						{
							step: "01. Authentication",
							cmd: "export NVIDIA_API_KEY=...",
							desc: "Initialize your AI reasoning vector.",
						},
						{
							step: "02. Arsenal Sync",
							cmd: "myth sync",
							desc: "Index 3,000+ local & remote tactical tools.",
						},
						{
							step: "03. Health Check",
							cmd: "myth check",
							desc: "Validate sandbox & connectivity integrity.",
						},
					].map((s) => (
						<div
							key={s.step}
							className="p-1 bg-gradient-to-br from-cyber-primary/20 to-transparent rounded-2xl"
						>
							<div className="bg-black/60 p-5 rounded-[15px] h-full border border-white/5">
								<div className="text-[10px] font-black text-cyber-primary uppercase tracking-widest mb-3">
									{s.step}
								</div>
								<code className="text-xs font-mono text-white block bg-black/40 p-2 rounded mb-3 border border-white/5 truncate">
									{s.cmd}
								</code>
								<p className="text-[10px] text-cyber-dim leading-relaxed">{s.desc}</p>
							</div>
						</div>
					))}
				</div>
			</div>

			{/* Sidebar */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-7 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 right-0 w-24 h-24 bg-cyber-primary/5 rounded-full blur-3xl" />
					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Shield className="w-5 h-5" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">
							Operative Integrity
						</h4>
					</div>

					<div className="space-y-4 mb-10">
						{[
							{
								label: "Signed Pkg",
								value: "VERIFIED",
								icon: <CheckCircle2 className="w-4 h-4 text-cyber-primary" />,
							},
							{
								label: "Binary Hash",
								value: "SHA-256 SUM",
								icon: <Activity className="w-4 h-4 text-cyber-secondary" />,
							},
							{
								label: "Sandbox Cap",
								value: "L1 SECURE",
								icon: <Shield className="w-4 h-4 text-cyber-accent" />,
							},
							{
								label: "Updates",
								value: "RECURSIVE",
								icon: <Monitor className="w-4 h-4 text-white" />,
							},
						].map((s) => (
							<div
								key={s.label}
								className="p-4 bg-white/5 rounded-2xl border border-white/5 flex items-center justify-between group hover:border-white/10 transition-all"
							>
								<div className="flex items-center gap-3">
									{s.icon}
									<span className="text-[10px] text-cyber-dim uppercase font-bold tracking-widest">
										{s.label}
									</span>
								</div>
								<span className="text-[10px] font-mono text-white font-bold">{s.value}</span>
							</div>
						))}
					</div>

					<div className="p-5 bg-cyber-secondary/10 rounded-2xl border border-cyber-secondary/20">
						<div className="flex items-center gap-2 mb-2 text-cyber-secondary">
							<AlertTriangle className="w-3.5 h-3.5" />
							<span className="text-[10px] font-black uppercase tracking-widest">
								Caution: Root
							</span>
						</div>
						<p className="text-[10px] text-cyber-secondary/70 leading-relaxed italic">
							Never run MYTH as root/sudo unless the installer explicitly requests it.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
