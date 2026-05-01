import {
	Activity,
	CheckCircle2,
	ChevronRight,
	Clock,
	Info,
	Key,
	Monitor,
	Rocket,
	Target,
	Terminal,
} from "lucide-react";
import { CodeBlock, PageHeader } from "../components/Layout";

export default function QuickStartPage() {
	const pagesUrl = import.meta.env.VITE_PAGES_URL || "https://myth.work.gd";

	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Strategic Quick-Start"
					description="Deploy and activate MYTH in under 300 seconds. A streamlined sequence for immediate operational readiness."
					badge="Mission Ready"
				/>

				{/* NEW: Visual "First 5 Minutes" Timeline */}
				<div className="mb-16">
					<div className="flex items-center gap-3 mb-10">
						<Clock className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							First 300 Seconds: Operational Timeline
						</h2>
					</div>

					<div className="relative">
						{/* Vertical line for mobile, Horizontal for desktop handled by flex/relative layout */}
						<div className="absolute left-6 top-0 bottom-0 w-px bg-cyber-border/30 md:left-0 md:top-6 md:h-px md:w-full md:bg-gradient-to-r md:from-cyber-primary/60 md:to-transparent" />

						<div className="grid grid-cols-1 md:grid-cols-4 gap-8 relative z-10">
							{[
								{
									time: "T+60s",
									label: "Deployment",
									desc: "Native kernel execution established via one-line bootstrap.",
									status: "COMPLETED",
								},
								{
									time: "T+180s",
									label: "Neural Link",
									desc: "NVIDIA NIM core authenticated and in-memory recall active.",
									status: "PENDING",
								},
								{
									time: "T+240s",
									label: "Arsenal Sync",
									desc: "3,000+ tactical vectors indexed and verified for dispatch.",
									status: "WAITING",
								},
								{
									time: "T+300s",
									label: "Mission Launch",
									desc: "Full 13-phase methodology ready for target intake.",
									status: "WAITING",
								},
							].map((t, i) => (
								<div key={t.label} className="flex md:flex-col gap-6 md:gap-4 pl-12 md:pl-0">
									<div
										className={`w-12 h-12 rounded-full border-4 flex items-center justify-center shrink-0 z-20 ${
											i === 0
												? "bg-cyber-primary border-cyber-primary/20 text-black shadow-[0_0_15px_#00ffa366]"
												: "bg-cyber-bg border-cyber-border text-cyber-dim"
										}`}
									>
										<CheckpointIcon index={i} />
									</div>
									<div>
										<div
											className={`text-[10px] font-black uppercase tracking-[0.2em] mb-1 ${i === 0 ? "text-cyber-primary" : "text-cyber-dim"}`}
										>
											{t.time}
										</div>
										<div className="text-[12px] font-bold text-white mb-2">{t.label}</div>
										<p className="text-[10px] text-cyber-dim leading-relaxed max-w-[180px]">
											{t.desc}
										</p>
									</div>
								</div>
							))}
						</div>
					</div>
				</div>

				<div className="space-y-16">
					{/* Step 1: Install */}
					<section className="scroll-mt-24">
						<div className="flex items-center gap-4 mb-6">
							<div className="w-10 h-10 rounded-xl bg-cyber-primary/10 border border-cyber-primary/20 flex items-center justify-center text-cyber-primary font-black font-mono">
								01
							</div>
							<h3 className="text-xl font-bold text-white flex items-center gap-3 uppercase tracking-wider">
								Infrastructure Deployment
							</h3>
						</div>
						<p className="text-sm text-cyber-text/80 mb-6 leading-relaxed max-w-3xl border-l-2 border-cyber-primary/20 pl-6">
							Execute the autonomous bootstrap vector. This script verifies your kernel
							architecture, establishes the native signed package registry, and provisions the
							Lightpanda browser engine.
						</p>
						<CodeBlock
							lang="bash"
							title="Primary Deployment Vector"
							code={`curl -sSL ${pagesUrl}/install.sh | sudo bash`}
						/>
						<div className="mt-4 flex items-center gap-2 text-[10px] text-cyber-dim">
							<div className="h-3 w-3 text-cyber-primary">
								<Info className="w-full h-full" />
							</div>
							<span>
								Standard deployment targets:{" "}
								<code className="text-cyber-primary">/usr/bin/myth</code> &{" "}
								<code className="text-cyber-primary">L1 Native Registry</code>
							</span>
						</div>
					</section>

					{/* Step 2: API Keys */}
					<section className="scroll-mt-24">
						<div className="flex items-center gap-4 mb-6">
							<div className="w-10 h-10 rounded-xl bg-cyber-secondary/10 border border-cyber-secondary/20 flex items-center justify-center text-cyber-secondary font-black font-mono">
								02
							</div>
							<h3 className="text-xl font-bold text-white flex items-center gap-3 uppercase tracking-wider">
								Neural Core Authentication
							</h3>
						</div>
						<p className="text-sm text-cyber-text/80 mb-6 leading-relaxed max-w-3xl border-l-2 border-cyber-secondary/20 pl-6">
							MYTH utilizes the NVIDIA NIM architecture for high-speed local/cloud reasoning.
							Register your cryptographic keys in the operative configuration.
						</p>
						<div className="space-y-4">
							<CodeBlock
								lang="bash"
								title="Session Vector"
								code='export NVIDIA_API_KEY="nvapi-xxxxxxxxxxxxxxxxxxxx"'
							/>
							<div className="relative group">
								<div className="absolute -inset-1 bg-cyber-secondary/5 rounded-2xl group-hover:bg-cyber-secondary/10 transition-all pointer-events-none" />
								<CodeBlock
									lang="yaml"
									title="Permanent Registry (~/.config/myth/user.yaml)"
									code={`provider:
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxx"  # Primary Vector
    - "nvapi-yyyyyyyyyyyyyyyy"  # Redundant Vector (Auto-Rotation)
  model: "deepseek-ai/deepseek-r1"
  temperature: 0.5`}
								/>
							</div>
						</div>
					</section>

					{/* Step 3: Initialization */}
					<section className="scroll-mt-24">
						<div className="flex items-center gap-4 mb-6">
							<div className="w-10 h-10 rounded-xl bg-cyber-accent/10 border border-cyber-accent/20 flex items-center justify-center text-cyber-accent font-black font-mono">
								03
							</div>
							<h3 className="text-xl font-bold text-white flex items-center gap-3 uppercase tracking-wider">
								Tactical Ingest
							</h3>
						</div>
						<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
							<div className="feature-card p-6 rounded-2xl border border-white/5 bg-black/40">
								<p className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-3">
									Phase A: Arsenal Sync
								</p>
								<CodeBlock lang="bash" code="myth sync" />
								<p className="text-[10px] text-cyber-text/60 mt-3 leading-relaxed">
									Indexes local Kali binaries and remote MCP tools into a unified neural schema.
								</p>
							</div>
							<div className="feature-card p-6 rounded-2xl border border-white/5 bg-black/40">
								<p className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-3">
									Phase B: Integrity Check
								</p>
								<CodeBlock lang="bash" code="myth check" />
								<p className="text-[10px] text-cyber-text/60 mt-3 leading-relaxed">
									Validates sandbox namespaces, network throughput, and AI reasoning response.
								</p>
							</div>
						</div>
					</section>

					{/* Step 4: Mission */}
					<section className="scroll-mt-24">
						<div className="flex items-center gap-4 mb-6">
							<div className="w-10 h-10 rounded-xl bg-white/5 border border-white/10 flex items-center justify-center text-white font-black font-mono shadow-2xl">
								04
							</div>
							<h3 className="text-xl font-bold text-white flex items-center gap-3 uppercase tracking-wider">
								First Mission Launch
							</h3>
						</div>
						<div className="p-1 bg-gradient-to-r from-cyber-primary/40 to-transparent rounded-2xl mb-6">
							<div className="bg-black/80 p-8 rounded-[15px] border border-white/5">
								<CodeBlock
									lang="bash"
									title="Subdomain Intelligence Gathering"
									code="myth subdomains example.com --active"
								/>
								<div className="mt-8 grid grid-cols-1 sm:grid-cols-2 gap-8 outline-none">
									<div className="space-y-4">
										<div className="flex items-center gap-2">
											<Monitor className="w-4 h-4 text-cyber-primary" />
											<span className="text-[11px] font-bold text-white uppercase tracking-widest">
												TUI Interface
											</span>
										</div>
										<p className="text-[10px] text-cyber-dim leading-relaxed">
											Real-time status boards, asset progression charts, and streaming neural
											intent.
										</p>
									</div>
									<div className="space-y-4">
										<div className="flex items-center gap-2">
											<Terminal className="w-4 h-4 text-cyber-secondary" />
											<span className="text-[11px] font-bold text-white uppercase tracking-widest">
												CLI Parity
											</span>
										</div>
										<p className="text-[10px] text-cyber-dim leading-relaxed">
											Standardized UNIX-compliant output for scripting and headless automation.
										</p>
									</div>
								</div>
							</div>
						</div>
					</section>
				</div>
			</div>

			{/* Sidebar - NEW: Progress Checklist */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-7 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 right-0 w-24 h-24 bg-cyber-primary/5 rounded-full blur-3xl" />
					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Target className="w-5 h-5" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Onboarding Pulse</h4>
					</div>

					<div className="space-y-5 mb-10">
						{[
							{ label: "Deployment Vector", done: true },
							{ label: "NVIDIA Key Active", done: false },
							{ label: "Tool Synchronization", done: false },
							{ label: "System Diagnostic", done: false },
							{ label: "Neural Interface Ready", done: false },
						].map((item) => (
							<div key={item.label} className="flex items-center justify-between group">
								<span
									className={`text-[11px] font-bold tracking-tight transition-colors ${item.done ? "text-cyber-primary" : "text-cyber-dim group-hover:text-white"}`}
								>
									{item.label}
								</span>
								<div
									className={`w-5 h-5 rounded-md border flex items-center justify-center transition-all ${
										item.done
											? "bg-cyber-primary border-cyber-primary text-black"
											: "border-white/10 group-hover:border-white/30"
									}`}
								>
									{item.done && <CheckCircle2 className="w-3.5 h-3.5" />}
								</div>
							</div>
						))}
					</div>

					<div className="p-6 bg-black/40 rounded-2xl border border-white/5 relative group">
						<div className="text-[10px] text-cyber-dim uppercase font-black tracking-widest mb-3">
							Mission Ready?
						</div>
						<button
							type="button"
							className="w-full py-4 bg-cyber-primary text-black font-black uppercase text-[11px] tracking-widest rounded-xl hover:bg-white transition-all shadow-[0_0_20px_-5px_#00ffa366] flex items-center justify-center gap-2"
						>
							<Rocket className="w-4 h-4" />
							Launch Terminal
						</button>
						<p className="text-[9px] text-cyber-dim/50 mt-4 text-center italic group-hover:text-cyber-primary/40 transition-colors">
							Requires manual key registration first.
						</p>
					</div>

					<div className="mt-8 pt-8 border-t border-cyber-border/20">
						<button
							type="button"
							className="w-full flex items-center justify-between text-[11px] text-white/40 hover:text-cyber-primary transition-colors uppercase font-bold tracking-widest"
						>
							Full Manual Reference
							<ChevronRight className="w-4 h-4" />
						</button>
					</div>
				</div>
			</div>
		</div>
	);
}

// Checkpoint helper
function CheckpointIcon({ index }: { index: number }) {
	const icons = [
		<Monitor className="w-full h-full" key="m" />,
		<Key className="w-full h-full" key="k" />,
		<Activity className="w-full h-full" key="a" />,
		<Rocket className="w-full h-full" key="r" />,
	];
	return <div className="w-5 h-5">{icons[index]}</div>;
}
