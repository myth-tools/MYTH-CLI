import { motion, type Variants } from "framer-motion";
import {
	Activity,
	Activity as ActivityIcon,
	Brain,
	Database,
	Globe,
	Layers,
	Lock,
	Rocket,
	Shield,
	Terminal,
	Zap,
} from "lucide-react";
import { Link } from "react-router-dom";
import { CopyButton } from "../components/CopyButton";
import SystemGraph from "../components/SystemGraph";
import { MYTH_NAME, MYTH_VERSION } from "../data/metadata";

const features = [
	{
		icon: <Shield className="w-5 h-5" />,
		title: "Sandboxed",
		desc: "Bubblewrap namespaces — host OS is read-only. Every tool call is isolated.",
	},
	{
		icon: <Database className="w-5 h-5" />,
		title: "Volatile",
		desc: "All data in RAM (tmpfs). Vanishes on session exit — zero forensic trace.",
	},
	{
		icon: <Zap className="w-5 h-5" />,
		title: "Ultra-Fast",
		desc: "Lightpanda Zig engine — 11× faster browser recon, 9× less RAM than Chrome.",
	},
	{
		icon: <Brain className="w-5 h-5" />,
		title: "AI-Driven",
		desc: "NVIDIA NIM + Rig.rs chains tools autonomously via 16 Rust tool bridges.",
	},
	{
		icon: <Globe className="w-5 h-5" />,
		title: "Universal",
		desc: "MCP protocol connects to 3,000+ Kali tools + any custom stdio/SSE server.",
	},
	{
		icon: <Lock className="w-5 h-5" />,
		title: "Secure",
		desc: "50+ blocked exploits, shell injection & path traversal guards built-in.",
	},
];

const comparisonRows = [
	{
		feature: "AI-Assisted Recon",
		myth: "✓ Fully autonomous — 13 phases, 89 steps",
		cursor: "~ Code-only suggestions",
		metasploit: "✗ Manual exploitation only",
	},
	{
		feature: "Kali Tool Integration",
		myth: "✓ 3,000+ tools via MCP bridges",
		cursor: "✗ None",
		metasploit: "✓ Built-in modules only",
	},
	{
		feature: "Zero Disk Trace",
		myth: "✓ Full RAM-only storage",
		cursor: "✗ Writes to disk",
		metasploit: "✗ Persistent database",
	},
	{
		feature: "Process Isolation",
		myth: "✓ Bubblewrap namespaces",
		cursor: "✗ Direct host access",
		metasploit: "✗ No sandboxing",
	},
	{
		feature: "Customizable via MCP",
		myth: "✓ Add any server at runtime",
		cursor: "~ Extensions only",
		metasploit: "✗ Fixed module system",
	},
	{
		feature: "LLM Reasoning Engine",
		myth: "✓ DeepSeek R1 + LLaMA 3.1 70B",
		cursor: "✓ GPT-4 / Claude",
		metasploit: "✗ None",
	},
	{
		feature: "Semantic Memory Recall",
		myth: "✓ Qdrant in-memory vectors",
		cursor: "~ Basic context window",
		metasploit: "✗ None",
	},
	{
		feature: "Binary Size",
		myth: "✓ ~8MB static binary",
		cursor: "✗ Electron (200MB+)",
		metasploit: "✗ Ruby runtime required",
	},
];

const quickLinks = [
	{
		icon: <Rocket className="w-5 h-5" />,
		title: "Quick Start",
		desc: "Install and launch your first recon mission",
		path: "/quickstart",
	},
	{
		icon: <Terminal className="w-5 h-5" />,
		title: "CLI Commands",
		desc: "All 27 myth subcommands with full parameter docs",
		path: "/cli-commands",
	},
	{
		icon: <Activity className="w-5 h-5" />,
		title: "Neural Vitals",
		desc: "Live telemetry and core diagnostics",
		path: "/vitals",
	},
	{
		icon: <Layers className="w-5 h-5" />,
		title: "Architecture",
		desc: "Interactive system graph and module breakdown",
		path: "/architecture",
	},
	{
		icon: <Globe className="w-5 h-5" />,
		title: "MCP Servers",
		desc: "11 factory-bundled MCP server registry",
		path: "/mcp-servers",
	},
	{
		icon: <Shield className="w-5 h-5" />,
		title: "Security Model",
		desc: "Zero-trust sandbox, Tor OPSEC, crypto stack",
		path: "/security",
	},
];

const steps = [
	{
		n: "01",
		title: "Install",
		desc: "Universal installer for Kali, Debian, Fedora, Arch, and Termux. Auto-detects architecture.",
		code: "curl -sSL https://myth.work.gd/install.sh | sudo bash",
	},
	{
		n: "02",
		title: "Configure",
		desc: "Add your free NVIDIA NIM API key to user.yaml. No GPU required.",
		code: "echo 'provider:\\n  api_keys:\\n    - nvapi-xxx' >> ~/.config/myth/user.yaml",
	},
	{
		n: "03",
		title: "Engage",
		desc: "Launch a full 13-phase autonomous recon mission. The agent takes it from there.",
		code: "myth scan target.com",
	},
];

const containerVariants: Variants = {
	hidden: { opacity: 0 },
	visible: {
		opacity: 1,
		transition: {
			staggerChildren: 0.1,
			delayChildren: 0.2,
		},
	},
};

const itemVariants: Variants = {
	hidden: { y: 20, opacity: 0 },
	visible: {
		y: 0,
		opacity: 1,
		transition: {
			duration: 0.8,
			ease: [0.2, 0, 0, 1],
		},
	},
};

export default function HomePage() {
	return (
		<div className="-mx-6 -mt-10 overflow-hidden">
			{/* ── Hero section — Stabilized with Motion ── */}
			<section className="relative hero-gradient px-6 pt-24 pb-20 text-center border-b border-cyber-border/20 overflow-hidden scanline">
				<div className="absolute inset-0 bg-[url('/noise.svg')] opacity-20 pointer-events-none" />
				<motion.div
					className="relative z-10 max-w-4xl mx-auto"
					initial="hidden"
					animate="visible"
					variants={containerVariants}
				>
					<motion.div
						variants={itemVariants}
						className="inline-flex items-center gap-2 px-3 py-1 mb-8 rounded-full bg-cyber-primary/10 border border-cyber-primary/20 text-[10px] text-cyber-primary font-bold uppercase tracking-[0.2em]"
					>
						<ActivityIcon className="w-3 h-3 animate-pulse" /> SYSTEM STATUS: OPERATIONAL V
						{MYTH_VERSION.toUpperCase()}
					</motion.div>

					<motion.h1
						variants={itemVariants}
						className="text-6xl md:text-8xl font-bold text-white mb-6 tracking-tighter relative group"
					>
						<span className="relative z-10">{MYTH_NAME}</span>

						<motion.span
							animate={{ opacity: [0, 0.6, 0] }}
							transition={{ duration: 3, repeat: Infinity, ease: "easeInOut" }}
							className="absolute inset-0 text-cyber-primary/20 blur-2xl opacity-0"
						>
							{MYTH_NAME}
						</motion.span>

						<span className="text-cyber-primary text-4xl md:text-6xl align-top ml-2 animate-pulse opacity-70">
							-
						</span>
					</motion.h1>

					<motion.p
						variants={itemVariants}
						className="text-lg md:text-xl text-cyber-text/60 max-w-2xl mx-auto mb-4 leading-relaxed font-light"
					>
						An ultra-fast, sandboxed, volatile AI-driven reconnaissance agent.
					</motion.p>
					<motion.p
						variants={itemVariants}
						className="text-base text-white/80 max-w-2xl mx-auto mb-10 leading-relaxed"
					>
						Connects to{" "}
						<span className="text-cyber-primary font-semibold">3,000+ Kali Linux tools</span> via
						Model Context Protocol (MCP). Powered by{" "}
						<span className="text-cyber-secondary font-semibold">NVIDIA NIM</span> and{" "}
						<span className="text-cyber-accent font-semibold">Rig.rs</span>. Built with Rust for
						elite performance. All tactical data vanishes on exit.
					</motion.p>

					<motion.div variants={itemVariants} className="flex justify-center gap-5 flex-wrap mb-14">
						<Link
							to="/installation"
							className="px-8 py-3 bg-cyber-primary text-cyber-bg font-bold rounded-xl hover:bg-cyber-primary/90 transition-all hover:scale-[1.02] shadow-lg shadow-cyber-primary/20"
						>
							INITIALIZE AGENT
						</Link>
						<Link
							to="/architecture"
							className="px-8 py-3 glass-panel text-white font-bold rounded-xl hover:bg-white/5 transition-all border border-white/10"
						>
							TECHNICAL DEEP DIVE
						</Link>
					</motion.div>

					{/* Stats bar */}
					<motion.div
						variants={itemVariants}
						className="grid grid-cols-2 sm:grid-cols-4 gap-4 max-w-3xl mx-auto"
					>
						{[
							{ value: "3,000+", label: "Kali Tools" },
							{ value: "13", label: "Recon Phases" },
							{ value: "89", label: "Methodology Steps" },
							{ value: "7", label: "Package Ecosystems" },
						].map((s) => (
							<div
								key={s.label}
								className="glass-panel rounded-xl p-4 text-center border border-cyber-primary/10"
							>
								<div className="text-2xl font-black text-cyber-primary glow-text mb-1">
									{s.value}
								</div>
								<div className="text-[10px] text-cyber-dim uppercase tracking-widest font-mono">
									{s.label}
								</div>
							</div>
						))}
					</motion.div>
				</motion.div>
			</section>

			{/* ── Remaining sections — Simplified stability ── */}
			<section className="px-6 py-20 bg-cyber-bg border-b border-cyber-border/20">
				<div className="max-w-5xl mx-auto">
					<div className="text-center mb-14">
						<h2 className="text-xs font-bold text-cyber-primary uppercase tracking-[0.3em] mb-3">
							Deploy in Minutes
						</h2>
						<p className="text-3xl font-bold text-white">Three Steps to Operational</p>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-3 gap-8">
						{steps.map((step) => (
							<div key={step.n} className="relative group/step">
								<div className="glass-panel rounded-2xl p-6 border border-cyber-border/30 hover:border-cyber-primary/30 transition-all flex flex-col h-full">
									<div className="flex items-center justify-between mb-6">
										<div className="px-2 py-1 rounded bg-cyber-primary/10 border border-cyber-primary/20 text-[10px] font-mono font-bold text-cyber-primary tracking-tighter">
											STEP_[{step.n}]
										</div>
										<div className="h-px flex-1 bg-gradient-to-r from-cyber-primary/20 to-transparent ml-4" />
									</div>

									<h3 className="text-xl font-black text-white mb-3 group-hover/step:text-cyber-primary transition-colors">
										{step.title}
									</h3>
									<p className="text-sm text-cyber-dim leading-relaxed mb-8 flex-1">{step.desc}</p>

									<div className="relative group/code">
										<div className="bg-black/60 rounded-xl p-4 pr-12 border border-cyber-border/50 overflow-hidden font-mono">
											<code className="text-[11px] text-cyber-primary whitespace-pre break-all">
												{step.code}
											</code>
										</div>
										<div className="absolute top-2 right-2 opacity-0 group-hover/code:opacity-100 transition-opacity">
											<CopyButton text={step.code} />
										</div>
									</div>
								</div>
							</div>
						))}
					</div>
				</div>
			</section>

			<section className="px-6 py-20 bg-cyber-bg border-b border-cyber-border/20">
				<div className="max-w-5xl mx-auto">
					<div className="text-center mb-14">
						<h2 className="text-xs font-bold text-cyber-primary uppercase tracking-[0.3em] mb-3">
							Core Intelligence
						</h2>
						<p className="text-3xl font-bold text-white">Advanced System Properties</p>
					</div>
					<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
						{features.map((f) => (
							<div
								key={f.title}
								className="glass-panel rounded-2xl p-6 group hover-glow transition-all duration-500"
							>
								<div className="w-10 h-10 rounded-lg bg-cyber-primary/10 flex items-center justify-center text-cyber-primary mb-5 group-hover:scale-110 transition-transform">
									{f.icon}
								</div>
								<h3 className="font-bold text-white mb-2 uppercase text-xs tracking-widest">
									{f.title}
								</h3>
								<p className="text-sm text-cyber-dim leading-relaxed">{f.desc}</p>
							</div>
						))}
					</div>
				</div>
			</section>

			<section className="px-6 py-20 bg-cyber-surface/30 border-b border-cyber-border/20">
				<div className="max-w-5xl mx-auto">
					<div className="text-center mb-14">
						<h2 className="text-xs font-bold text-cyber-accent uppercase tracking-[0.3em] mb-3">
							Competitive Edge
						</h2>
						<p className="text-3xl font-bold text-white">Why MYTH?</p>
						<p className="text-sm text-cyber-dim mt-3 max-w-xl mx-auto">
							MYTH is not a code editor plugin or an exploitation framework. It's a purpose-built
							autonomous recon agent for security professionals.
						</p>
					</div>
					<div className="table-container">
						<table className="w-full text-sm docs-table rounded-xl overflow-hidden">
							<thead>
								<tr>
									<th className="text-left py-3 px-4">Feature</th>
									<th className="text-left py-3 px-4 text-cyber-primary">MYTH</th>
									<th className="text-left py-3 px-4 text-cyber-dim">Cursor / Copilot</th>
									<th className="text-left py-3 px-4 text-cyber-dim">Metasploit</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-cyber-border/30">
								{comparisonRows.map((row) => (
									<tr key={row.feature} className="hover:bg-white/[0.02] transition-colors">
										<td className="py-3 px-4 font-medium text-cyber-text/80 text-xs">
											{row.feature}
										</td>
										<td className="py-3 px-4 text-xs text-cyber-primary font-medium">{row.myth}</td>
										<td className="py-3 px-4 text-xs text-cyber-dim">{row.cursor}</td>
										<td className="py-3 px-4 text-xs text-cyber-dim">{row.metasploit}</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>
			</section>

			<section className="px-6 py-20 bg-cyber-bg border-b border-cyber-border/20">
				<div className="max-w-5xl mx-auto">
					<div className="grid grid-cols-1 lg:grid-cols-5 gap-12 items-center">
						<div className="lg:col-span-2">
							<h2 className="text-xs font-bold text-cyber-secondary uppercase tracking-[0.3em] mb-3">
								Architecture
							</h2>
							<p className="text-3xl font-bold text-white mb-6">Neural Interconnects</p>
							<p className="text-sm text-cyber-dim leading-relaxed mb-8">
								MYTH is a distributed agentic network. Rig.rs serves as the neural backbone and
								NVIDIA NIM provides reasoning. The agent dispatches tasks through sandboxed MCP
								bridges across 3,000+ local tools and remote services.
							</p>
							<div className="space-y-3">
								{[
									"Direct NVIDIA NIM Integration (DeepSeek R1)",
									"Bubblewrap Sandboxed Execution",
									"In-Memory Semantic Recall (Qdrant)",
									"16 Rust Tool Bridges",
									"13-Phase Recon Methodology",
								].map((tag) => (
									<div
										key={tag}
										className="flex items-center gap-3 text-xs text-cyber-text/80 font-mono"
									>
										<div className="w-1.5 h-1.5 rounded-full bg-cyber-primary shrink-0" />
										{tag}
									</div>
								))}
							</div>
						</div>
						<div className="lg:col-span-3">
							<SystemGraph />
						</div>
					</div>
				</div>
			</section>

			<section className="px-6 py-20 bg-cyber-surface/20 border-b border-cyber-border/20">
				<div className="max-w-5xl mx-auto">
					<div className="text-center mb-14">
						<h2 className="text-xs font-bold text-cyber-accent uppercase tracking-[0.3em] mb-3">
							Portal
						</h2>
						<p className="text-3xl font-bold text-white">Documentation Matrix</p>
					</div>
					<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-5">
						{quickLinks.map((l) => (
							<Link
								key={l.path}
								to={l.path}
								className="glass-panel rounded-2xl p-6 group flex items-start gap-4 hover-glow transition-all clickable-card"
							>
								<div className="w-12 h-12 rounded-xl bg-white/5 flex items-center justify-center text-cyber-primary group-hover:bg-cyber-primary/20 group-hover:text-cyber-bg transition-all shrink-0">
									{l.icon}
								</div>
								<div>
									<h3 className="font-bold text-white mb-1 group-hover:text-cyber-primary transition-colors">
										{l.title}
									</h3>
									<p className="text-xs text-cyber-dim leading-normal">{l.desc}</p>
								</div>
							</Link>
						))}
					</div>
				</div>
			</section>

			<footer className="px-6 py-12 border-t border-cyber-border/30 text-center bg-black/40">
				<p className="text-[10px] font-mono text-cyber-dim uppercase tracking-widest mb-4">
					Licensed under MIT — Neural Core v{MYTH_VERSION}
				</p>
				<p className="text-[10px] text-cyber-accent/80 font-bold uppercase tracking-widest">
					⚠️ Authorization Required: Never scan targets without written permission.
				</p>
			</footer>
		</div>
	);
}
