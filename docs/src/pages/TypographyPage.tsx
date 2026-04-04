import {
	Box,
	CheckCircle2,
	Info,
	Keyboard,
	Layers,
	Layout,
	Monitor,
	Settings,
	Terminal,
	Zap,
} from "lucide-react";
import { CodeBlock, PageHeader } from "../components/Layout";

const terminalRecommendations = [
	{
		name: "Kitty",
		icon: <Terminal />,
		status: "ELITE",
		color: "text-cyber-primary",
		note: "Best-in-class GPU acceleration and glyph support. Native image rendering.",
	},
	{
		name: "Alacritty",
		icon: <Zap />,
		status: "FASTEST",
		color: "text-cyber-secondary",
		note: "Pure performance. Rust-based, zero-latency rendering for high-speed missions.",
	},
	{
		name: "ITerm2 (macOS)",
		icon: <Monitor />,
		status: "STABLE",
		color: "text-blue-400",
		note: "Comprehensive feature set. Native support for complex лигатуры and Nerd Fonts.",
	},
	{
		name: "WezTerm",
		icon: <Layout />,
		status: "VERSATILE",
		color: "text-cyber-accent",
		note: "Lua-scriptable, cross-platform powerhouse. Excellent font fallback engine.",
	},
];

const glyphSupport = [
	{ category: "Nerd Font Icons", mapping: "0xEA00 - 0xEBFF", support: "99%", status: "MANDATORY" },
	{ category: "ANSI 256 Colors", mapping: "Standard xterm", support: "100%", status: "STABLE" },
	{
		category: "Ligatures (=>, !=)",
		mapping: "Font Dependent",
		support: "100%",
		status: "OPTIONAL",
	},
	{ category: "UTF-8 / Unicode", mapping: "Global Wide", support: "100%", status: "STABLE" },
];

export default function TypographyPage() {
	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Neural Typography & Visuals"
					description="MYTH utilizes advanced Nerd Font mapping and ligature support to deliver a high-speed, data-rich TUI experience. Modern terminal standards for modern operatives."
					badge="Visual Core"
				/>

				{/* NEW: Terminal Emulator Recommendation Guide */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-8">
						<Monitor className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Operational Terminal Guide
						</h2>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-2 gap-4">
						{terminalRecommendations.map((t) => (
							<div
								key={t.name}
								className="feature-card p-6 rounded-2xl border border-white/5 bg-black/40 group hover:border-cyber-primary/20 transition-all flex gap-5 items-start"
							>
								<div
									className={`w-12 h-12 rounded-xl bg-white/5 flex items-center justify-center border border-white/5 shrink-0 group-hover:scale-110 transition-transform ${t.color}`}
								>
									{t.icon}
								</div>
								<div>
									<div className="flex items-center gap-3 mb-1.5">
										<h3 className="text-base font-bold text-white uppercase tracking-wider">
											{t.name}
										</h3>
										<span
											className={`text-[9px] font-black uppercase px-2 py-0.5 rounded border border-current/20 bg-current/5 ${t.color}`}
										>
											{t.status}
										</span>
									</div>
									<p className="text-[11px] text-cyber-dim leading-relaxed italic">{t.note}</p>
								</div>
							</div>
						))}
					</div>
					<div className="mt-4 p-4 bg-cyber-primary/5 border border-cyber-primary/20 rounded-2xl flex items-center gap-3">
						<Info className="w-5 h-5 text-cyber-primary shrink-0" />
						<p className="text-[10px] text-cyber-primary/70 leading-relaxed italic">
							Pro Operative Tip: Ensure your terminal's{" "}
							<code className="bg-cyber-primary/10 px-1 rounded">Font Smoothing</code> is enabled
							for pixel-perfect glyph alignment.
						</p>
					</div>
				</div>

				{/* Glyph Matrix */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x01</span>
					Glyph & Protocol Matrix
				</h2>
				<div className="glass-panel overflow-hidden rounded-2xl border border-cyber-border/40 shadow-xl mb-16">
					<table className="w-full text-left text-xs docs-table border-none">
						<thead className="bg-white/5 border-b border-cyber-border/30">
							<tr>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
									Category
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
									Neural Mapping
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
									Engine Support
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[10px]">
									Priority
								</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-cyber-border/20">
							{glyphSupport.map((g) => (
								<tr key={g.category} className="hover:bg-white/[0.02] transition-colors group">
									<td className="py-5 px-6 font-bold text-white text-sm group-hover:text-cyber-primary transition-colors">
										{g.category}
									</td>
									<td className="py-5 px-6 text-cyber-dim font-mono">{g.mapping}</td>
									<td className="py-5 px-6 text-cyber-primary font-black font-mono">{g.support}</td>
									<td className="py-5 px-6">
										<span
											className={`text-[10px] font-black px-2 py-0.5 rounded-full border ${
												g.status === "MANDATORY"
													? "border-cyber-primary/40 text-cyber-primary bg-cyber-primary/5"
													: "border-white/10 text-cyber-dim bg-white/5"
											}`}
										>
											{g.status}
										</span>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>

				{/* Configuration */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x02</span>
					Tactical Configuration
				</h2>
				<div className="space-y-6 mb-16">
					<p className="text-sm text-cyber-text/80 leading-relaxed max-w-3xl">
						Typography settings are managed in your{" "}
						<code className="text-cyber-primary">user.yaml</code>. Use these flags to modulate the
						visual density of the neural interface.
					</p>
					<CodeBlock
						lang="yaml"
						title="Typography Modulations (config/user.yaml)"
						code={`tui:
  theme: "cyber-elite"       # Global visual profile
  nerd_fonts: true          # Use high-density glyph mapping
  ligatures: true           # Enable functional ligature rendering
  unicode_borders: true     # Use UTF-8 for box-drawing accuracy
  color_depth: "256bit"     # ANSI true-color output
  font_size: 11             # Optimal operative scaling`}
					/>
				</div>

				{/* Ligatures Box */}
				<div className="feature-card rounded-3xl p-8 border border-cyber-secondary/30 bg-cyber-secondary/[0.01] mb-12 relative overflow-hidden group shadow-lg">
					<div className="absolute -top-12 -right-12 w-32 h-32 bg-cyber-secondary/5 rounded-full blur-2xl group-hover:bg-cyber-secondary/10 transition-all pointer-events-none" />
					<div className="flex items-center gap-3 mb-5">
						<Keyboard className="w-5 h-5 text-cyber-secondary" />
						<h3 className="text-lg font-black text-white uppercase tracking-widest italic">
							Neural Ligatures
						</h3>
					</div>
					<p className="text-sm text-cyber-text/80 mb-8 leading-relaxed max-w-3xl border-l border-cyber-secondary/20 pl-4">
						MYTH is optimized for functional ligatures. Modern programming fonts like{" "}
						<strong className="text-white">Fira Code</strong> or{" "}
						<strong className="text-white">JetBrains Mono</strong> transform standard operators into
						single, high-speed tactical glyphs.
					</p>
					<div className="grid grid-cols-2 md:grid-cols-4 gap-4">
						{[
							{ src: "->", res: "→" },
							{ src: "!=", res: "≠" },
							{ src: "=>", res: "⇒" },
							{ src: "===", res: "≡" },
						].map((l) => (
							<div
								key={l.src}
								className="p-4 bg-black/40 rounded-xl border border-white/5 text-center group"
							>
								<div className="text-[10px] text-cyber-dim mb-1 font-mono">{l.src}</div>
								<div className="text-2xl font-mono text-cyber-secondary group-hover:scale-125 transition-transform">
									{l.res}
								</div>
							</div>
						))}
					</div>
				</div>
			</div>

			{/* Sidebar - Visual Health */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-7 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 right-0 w-full h-1 bg-gradient-to-r from-transparent via-cyber-primary/40 to-transparent" />
					<div className="flex items-center gap-3 mb-8 text-cyber-primary">
						<Layout className="w-5 h-5" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Visual Integrity</h4>
					</div>

					<div className="space-y-4 mb-10">
						{[
							{
								label: "Font Sync",
								value: "NOMINAL",
								icon: <CheckCircle2 className="w-4 h-4 text-cyber-primary" />,
							},
							{
								label: "Color Depth",
								value: "TRUECOLOR",
								icon: <Layers className="w-4 h-4 text-cyber-secondary" />,
							},
							{
								label: "Rendering",
								value: "GPU-ACCEL",
								icon: <Zap className="w-4 h-4 text-cyber-accent" />,
							},
							{
								label: "Glyph Alignment",
								value: "PERFECT",
								icon: <Box className="w-4 h-4 text-white" />,
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

					<div className="p-5 bg-cyber-primary/10 rounded-2xl border border-cyber-primary/20">
						<div className={`flex items-center gap-2 mb-2 text-cyber-primary flex-wrap`}>
							<Settings className="w-3.5 h-3.5" />
							<span className="text-[10px] font-black uppercase tracking-widest">
								Display Alert
							</span>
						</div>
						<p className="text-[10px] text-cyber-primary/70 leading-relaxed italic">
							Missing icons? Run{" "}
							<code className="bg-cyber-primary/10 px-1 rounded">myth check</code> to verify
							font-patch status.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
