import { Brain, ExternalLink, Github, Globe, Linkedin, Mail, Twitter, Zap } from "lucide-react";
import { PageHeader } from "../components/Layout";

export default function CreatorPage() {
	return (
		<div className="max-w-5xl mx-auto">
			<PageHeader
				title="Neural Architect"
				description="The visionary mind behind the MYTH Tactical Sovereign Discovery Project. Bridging the gap between human intuition and machine-speed reconnaissance."
				badge="Project Founder"
			/>

			<div className="grid grid-cols-1 lg:grid-cols-12 gap-12 mt-16">
				{/* Architect Avatar & Core Stats */}
				<div className="lg:col-span-5 space-y-8">
					<div className="relative group">
						<div className="absolute -inset-4 bg-gradient-to-r from-cyber-primary/20 via-cyber-secondary/20 to-cyber-accent/20 rounded-[2.5rem] blur-2xl opacity-50 group-hover:opacity-100 transition-opacity duration-700" />
						<div className="relative aspect-square rounded-[2rem] overflow-hidden border border-white/10 shadow-2xl">
							<img
								src="https://avatars.githubusercontent.com/u/144673841?v=4"
								alt="Shesher"
								className="w-full h-full object-cover grayscale hover:grayscale-0 transition-all duration-700 scale-105 group-hover:scale-100"
							/>
							<div className="absolute inset-0 bg-gradient-to-t from-black via-transparent to-transparent opacity-60" />
							<div className="absolute bottom-6 left-6 right-6">
								<div className="flex items-center gap-3">
									<div className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse shadow-[0_0_8px_#00ffa366]" />
									<span className="text-[10px] font-black text-white uppercase tracking-[0.3em]">
										Status: Online / Decrypting
									</span>
								</div>
							</div>
						</div>
					</div>

					<div className="glass-panel p-8 rounded-3xl border border-cyber-border/50 space-y-6 relative overflow-hidden">
						<div className="absolute top-0 left-0 w-1 h-full bg-cyber-primary/40" />
						<div className="flex items-center justify-between">
							<h3 className="text-xs font-black text-white uppercase tracking-widest">
								Core Identity
							</h3>
							<span className="text-[10px] font-mono text-cyber-primary px-2 py-0.5 bg-cyber-primary/10 rounded">
								ID: ARCH-01
							</span>
						</div>
						<div className="space-y-4">
							<div className="flex items-center justify-between py-3 border-b border-white/5">
								<span className="text-xs text-cyber-dim">Designation</span>
								<span className="text-xs text-white font-bold uppercase tracking-wider">
									Shesher
								</span>
							</div>
							<div className="flex items-center justify-between py-3 border-b border-white/5">
								<span className="text-xs text-cyber-dim">Neural Focus</span>
								<span className="text-xs text-cyber-primary font-bold uppercase tracking-wider">
									Agentic AI / Pen-Testing
								</span>
							</div>
							<div className="flex items-center justify-between py-3">
								<span className="text-xs text-cyber-dim">Location</span>
								<span className="text-xs text-white font-bold uppercase tracking-wider">
									Earth {"//"} Digital Space
								</span>
							</div>
						</div>
					</div>
				</div>

				{/* Detailed Dossier */}
				<div className="lg:col-span-7 space-y-10">
					<section className="space-y-6">
						<div className="flex items-center gap-3">
							<Brain className="w-6 h-6 text-cyber-primary" />
							<h2 className="text-2xl font-black text-white uppercase tracking-tighter">
								Visionary Dossier
							</h2>
						</div>
						<div className="prose prose-invert max-w-none">
							<p className="text-base text-cyber-text/90 leading-relaxed font-light">
								Shesher is a cutting-edge developer and security researcher specializing in the
								intersection of <span className="text-cyber-primary font-bold">Agentic AI</span> and
								<span className="text-cyber-secondary font-bold"> Cybersecurity</span>. With a deep
								passion for creating autonomous systems that think and act at machine-speed, he
								pioneers tools that empower researchers to scale their reconnaissance efforts beyond
								human limitations.
							</p>
							<p className="text-base text-cyber-text/90 leading-relaxed font-light mt-6">
								The{" "}
								<span className="text-white font-bold uppercase tracking-widest italic">Myth</span>{" "}
								project represents his latest evolution—a fusion of high-performance Rust
								engineering and advanced LLM orchestrations designed for the future of digital
								discovery.
							</p>
						</div>
					</section>

					<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
						<div className="glass-panel p-6 rounded-[2rem] border border-white/5 hover:border-cyber-primary/30 transition-colors group">
							<div className="w-10 h-10 rounded-xl bg-white/5 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
								<Mail className="w-5 h-5 text-cyber-primary" />
							</div>
							<h4 className="text-xs font-black text-white uppercase tracking-widest mb-2">
								Direct Link
							</h4>
							<p className="text-[11px] text-cyber-dim mb-4 leading-relaxed">
								For strategic partnerships or neural integration inquiries.
							</p>
							<a
								href="mailto:shesher0llms@gmail.com"
								className="text-xs text-cyber-primary font-bold flex items-center gap-2 hover:underline"
							>
								shesher0llms@gmail.com
								<ExternalLink className="w-3 h-3" />
							</a>
						</div>

						<div className="glass-panel p-6 rounded-[2rem] border border-white/5 hover:border-cyber-secondary/30 transition-colors group">
							<div className="w-10 h-10 rounded-xl bg-white/5 flex items-center justify-center mb-4 group-hover:scale-110 transition-transform">
								<Github className="w-5 h-5 text-cyber-secondary" />
							</div>
							<h4 className="text-xs font-black text-white uppercase tracking-widest mb-2">
								Mission Source
							</h4>
							<p className="text-[11px] text-cyber-dim mb-4 leading-relaxed">
								Audit the neural core and contribute to the evolution.
							</p>
							<a
								href="https://github.com/Shesher0"
								target="_blank"
								rel="noopener noreferrer"
								className="text-xs text-cyber-secondary font-bold flex items-center gap-2 hover:underline"
							>
								@Shesher0
								<ExternalLink className="w-3 h-3" />
							</a>
						</div>
					</div>

					{/* Tactical Connect Section */}
					<div className="pt-8 border-t border-white/10">
						<h4 className="text-[10px] font-black text-cyber-dim uppercase tracking-[0.4em] mb-8">
							Neural Networks / Protocols
						</h4>
						<div className="flex flex-wrap gap-4">
							{[
								{ icon: <Twitter />, label: "X / Twitter", id: "@Shesher_AI" },
								{ icon: <Globe />, label: "Personal Space", id: "shesher.ai" },
								{ icon: <Linkedin />, label: "LinkedIn", id: "Shesher" },
							].map((social) => (
								<button
									key={social.label}
									type="button"
									className="px-6 py-3 rounded-2xl bg-white/5 border border-white/10 hover:border-cyber-primary/40 hover:bg-cyber-primary/5 transition-all flex items-center gap-3 group outline-none cursor-pointer"
								>
									<div className="text-cyber-dim group-hover:text-cyber-primary transition-colors">
										{social.icon}
									</div>
									<div className="flex flex-col items-start">
										<span className="text-[9px] text-cyber-dim uppercase font-bold tracking-widest">
											{social.label}
										</span>
										<span className="text-xs text-white font-mono">{social.id}</span>
									</div>
								</button>
							))}
						</div>
					</div>
				</div>
			</div>

			{/* Architect Quote */}
			<div className="mt-24 relative p-12 rounded-[3rem] border border-cyber-primary/20 bg-cyber-primary/5 overflow-hidden">
				<div className="absolute top-0 right-0 p-8 opacity-[0.03]">
					<Brain className="w-64 h-64" />
				</div>
				<div className="relative max-w-3xl">
					<div className="text-cyber-primary mb-6">
						<Zap className="w-8 h-8 fill-current" />
					</div>
					<blockquote className="text-2xl md:text-3xl font-light text-white leading-tight tracking-tight italic">
						"In the age of machine intelligence, speed is the ultimate firewall. We don't just find
						vulnerabilities; we predict and neutralize them before they manifest."
					</blockquote>
					<div className="mt-8 flex items-center gap-4">
						<div className="h-px w-12 bg-cyber-primary" />
						<cite className="not-italic text-sm font-black uppercase tracking-[0.3em] text-cyber-primary">
							Shesher, lead architect
						</cite>
					</div>
				</div>
			</div>
		</div>
	);
}
