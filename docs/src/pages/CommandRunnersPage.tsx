import { useState } from "react";
import { CodeBlock, PageHeader } from "../components/Layout";

const ecosystems = [
	{
		id: "apt",
		label: "Native Repositories",
		emoji: "🐧",
		badge: "Recommended",
		badgeColor: "bg-cyber-primary text-cyber-bg",
		desc: "The cleanest installation method for Linux (Kali, Debian, Fedora, Arch, Termux). Managed by your system package manager (apt/dnf/pacman).",
		sections: [
			{
				title: "Add Repository & Install",
				blocks: [
					{
						lang: "bash",
						title: "One-liner (auto-configures repo + key)",
						code: "curl -sSL https://myth.work.gd/install.sh | sudo bash",
					},
					{
						lang: "bash",
						title: "Native package manager install (after bootstrap)",
						code: "# Debian/Ubuntu/Kali\nsudo apt update && sudo apt install myth\n\n# Fedora/RHEL\nsudo dnf install myth\n\n# Arch Linux\nsudo pacman -Syu myth\n\n# Termux\npkg install myth",
					},
					{ lang: "bash", title: "Launch a mission", code: "myth scan <target>" },
				],
			},
		],
		note: "Includes all system dependencies. Fully managed by your native package manager.",
	},
	{
		id: "npm",
		label: "NPM / Bun / PNPM",
		emoji: "📦",
		badge: "JavaScript",
		badgeColor: "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
		desc: "MYTH ships as a native Rust binary wrapped in an NPM package. Works with all modern Node.js package managers.",
		note: "Package name: @myth-tools/myth — install as a global binary or run ephemerally.",
		sections: [
			{
				title: "A. Persistent Global Install",
				blocks: [
					{ lang: "bash", title: "npm", code: "npm install -g @myth-tools/myth" },
					{ lang: "bash", title: "bun", code: "bun add -g @myth-tools/myth" },
					{ lang: "bash", title: "pnpm", code: "pnpm add -g @myth-tools/myth" },
					{ lang: "bash", title: "yarn", code: "yarn global add @myth-tools/myth" },
					{ lang: "bash", title: "Run", code: "myth scan <target>" },
				],
			},
			{
				title: "B. Ephemeral (Zero Install — Cached Execution)",
				blocks: [
					{ lang: "bash", title: "npx", code: "npx @myth-tools/myth scan <target>" },
					{ lang: "bash", title: "bunx", code: "bunx @myth-tools/myth scan <target>" },
					{ lang: "bash", title: "pnpm dlx", code: "pnpm dlx @myth-tools/myth scan <target>" },
					{ lang: "bash", title: "yarn dlx", code: "yarn dlx @myth-tools/myth scan <target>" },
				],
			},
		],
	},
	{
		id: "python",
		label: "PyPI / UV / pipx",
		emoji: "🐍",
		badge: "Python",
		badgeColor: "bg-blue-500/20 text-blue-300 border-blue-500/30",
		desc: "Native Rust binary wrapped in a Python wheel. Zero Python runtime overhead — the wheel just downloads and places the binary.",
		note: "Package name: myth-cli (PyPI namespace). The binary is always called myth.",
		sections: [
			{
				title: "A. Persistent Global Install",
				blocks: [
					{ lang: "bash", title: "uv (recommended — fastest)", code: "uv tool install myth-cli" },
					{ lang: "bash", title: "pipx (isolated environment)", code: "pipx install myth-cli" },
					{ lang: "bash", title: "pip (legacy)", code: "pip install myth-cli" },
					{ lang: "bash", title: "Run", code: "myth scan <target>" },
				],
			},
			{
				title: "B. Ephemeral (Zero Install)",
				blocks: [
					{ lang: "bash", title: "uvx", code: "uvx --from myth-cli myth scan <target>" },
					{ lang: "bash", title: "pipx run", code: "pipx run --spec myth-cli myth scan <target>" },
				],
			},
		],
	},
	{
		id: "docker",
		label: "Docker / Podman",
		emoji: "🐳",
		badge: "Containers",
		badgeColor: "bg-cyan-500/20 text-cyan-300 border-cyan-500/30",
		desc: "Full Kali Linux container image with MYTH pre-installed. All data stored in container RAM — zero host disk writes.",
		note: "--privileged is required for Bubblewrap's kernel namespace mounting inside the container.",
		sections: [
			{
				title: "Container Execution",
				blocks: [
					{
						lang: "bash",
						title: "Docker",
						code: "docker run -it --rm --privileged ghcr.io/myth-tools/myth scan <target>",
					},
					{
						lang: "bash",
						title: "Podman (rootless)",
						code: "podman run -it --rm --privileged ghcr.io/myth-tools/myth scan <target>",
					},
					{
						lang: "bash",
						title: "Interactive mode",
						code: "docker run -it --rm --privileged ghcr.io/myth-tools/myth",
					},
					{
						lang: "bash",
						title: "With volume (persist reports)",
						code: "docker run -it --rm --privileged -v $(pwd)/reports:/reports ghcr.io/myth-tools/myth scan <target>",
					},
				],
			},
		],
	},
	{
		id: "snap",
		label: "Snap (Ubuntu Store)",
		emoji: "🔧",
		badge: "Canonical",
		badgeColor: "bg-orange-500/20 text-orange-300 border-orange-500/30",
		desc: "Native Ubuntu Store distribution via Canonical Snapcraft. Automatic updates and classic sandboxing.",
		note: "Requires classic confinement for Bubblewrap namespace support.",
		sections: [
			{
				title: "Snap Execution",
				blocks: [
					{ lang: "bash", title: "Install", code: "sudo snap install myth --classic" },
					{ lang: "bash", title: "Run", code: "myth scan <target>" },
					{ lang: "bash", title: "Alternative run", code: "snap run myth scan <target>" },
				],
			},
		],
	},
	{
		id: "cargo",
		label: "Cargo / Rust",
		emoji: "🦀",
		badge: "Developer",
		badgeColor: "bg-orange-600/20 text-orange-400 border-orange-600/30",
		desc: "Build from source or install pre-compiled binaries directly via the Rust toolchain. Ideal for contributors.",
		note: "cargo binstall downloads a pre-compiled binary without compilation overhead.",
		sections: [
			{
				title: "Rust Toolchain",
				blocks: [
					{
						lang: "bash",
						title: "Binary install (fastest — no compile)",
						code: "cargo binstall myth",
					},
					{
						lang: "bash",
						title: "Build from source (GitHub)",
						code: "cargo install myth --git https://github.com/myth-tools/MYTH-CLI",
					},
					{ lang: "bash", title: "Run in dev mode", code: "cargo run --release -- scan <target>" },
					{ lang: "bash", title: "Run installed binary", code: "myth scan <target>" },
				],
			},
		],
	},
	{
		id: "nix",
		label: "Nix Flake",
		emoji: "❄️",
		badge: "Reproducible",
		badgeColor: "bg-indigo-500/20 text-indigo-300 border-indigo-500/30",
		desc: "100% reproducible deployments. Automatically provisions all dependencies (tor, nmap, bwrap) from Nix store. Zero system state modification.",
		note: "nix run leaves zero permanent system trace — all deps garbage-collected after runtime.",
		sections: [
			{
				title: "Nix Execution",
				blocks: [
					{
						lang: "bash",
						title: "Instant ephemeral run",
						code: "nix run github:myth-tools/MYTH-CLI?dir=package_runners -- scan <target>",
					},
					{
						lang: "bash",
						title: "Persistent dev shell",
						code: "nix shell github:myth-tools/MYTH-CLI?dir=package_runners",
					},
					{ lang: "bash", title: "Then run myth", code: "myth scan <target>" },
				],
			},
		],
	},
];

export default function CommandRunnersPage() {
	const [active, setActive] = useState("apt");
	const current = ecosystems.find((e) => e.id === active)!;

	return (
		<div>
			<PageHeader
				title="Universal Command Runners"
				description="Execute MYTH across 7 ecosystems — APT, NPM, PyPI, Docker, Snap, Cargo, and Nix. One binary. Every platform. The executable is always myth regardless of how you install it."
				badge="Deployment"
			/>

			{/* Namespace notice */}
			<div className="mb-8 border-l-4 border-cyber-warning pl-4 py-3 bg-cyber-warning/5 rounded-r-lg">
				<p className="text-cyber-warning font-bold text-sm mb-1">⚠️ Package Name Notice</p>
				<p className="text-xs text-cyber-text/80 leading-relaxed">
					Due to name collisions, the Python package is{" "}
					<code className="text-cyber-primary">myth-cli</code> and the NPM package is{" "}
					<code className="text-cyber-primary">@myth-tools/myth</code>. The resulting executable is
					always <code className="text-cyber-primary">myth</code> regardless of how you install it.
				</p>
			</div>

			{/* Ecosystem tabs */}
			<div className="flex flex-wrap gap-2 mb-6">
				{ecosystems.map((eco) => (
					<button
						key={eco.id}
						type="button"
						onClick={() => setActive(eco.id)}
						className={`flex items-center gap-1.5 text-[11px] px-3 py-1.5 rounded-lg border font-mono font-bold transition-all ${
							active === eco.id
								? "bg-cyber-primary text-cyber-bg border-cyber-primary"
								: "bg-white/5 text-cyber-dim border-cyber-border hover:border-cyber-primary/40"
						}`}
					>
						<span>{eco.emoji}</span>
						{eco.label}
					</button>
				))}
			</div>

			{/* Active ecosystem panel */}
			<div className="glass-panel rounded-2xl p-6 border border-cyber-border/50">
				<div className="flex flex-wrap items-center gap-3 mb-4">
					<h2 className="text-xl font-bold text-white">
						{current.emoji} {current.label}
					</h2>
					<span
						className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold uppercase tracking-wider ${current.badgeColor}`}
					>
						{current.badge}
					</span>
				</div>

				<p className="text-sm text-cyber-text/80 mb-2 leading-relaxed">{current.desc}</p>

				{current.note && (
					<p className="text-xs text-cyber-warning/80 italic mb-6 bg-cyber-warning/5 px-3 py-2 rounded border border-cyber-warning/20">
						💡 {current.note}
					</p>
				)}

				{current.sections.map((sec) => (
					<div key={sec.title} className="mb-6">
						{current.sections.length > 1 && (
							<h3 className="text-sm font-bold text-cyber-primary mb-3 uppercase tracking-wider">
								{sec.title}
							</h3>
						)}
						<div className="space-y-3">
							{sec.blocks.map((b) => (
								<CodeBlock key={b.title} lang={b.lang} title={b.title} code={b.code} />
							))}
						</div>
					</div>
				))}
			</div>

			{/* Pro tip */}
			<div className="mt-8 feature-card rounded-xl p-5 border-cyber-primary/30">
				<h3 className="font-semibold text-cyber-primary mb-2 flex items-center gap-2">
					⚡ Pro-Tip: Autonomous AI Mode
				</h3>
				<p className="text-sm text-cyber-text/80 leading-relaxed">
					Running <code className="text-cyber-primary">myth</code> or{" "}
					<code className="text-cyber-primary">npx @myth-tools/myth</code> with no arguments
					immediately launches an interactive AI terminal session with the full TUI. No subcommand
					required.
				</p>
			</div>
		</div>
	);
}
