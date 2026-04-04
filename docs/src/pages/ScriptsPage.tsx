import { CodeBlock, PageHeader } from "../components/Layout";

const scripts = [
	{
		name: "install.sh",
		category: "Distribution",
		desc: "Official one-line installer. Configures the native signed repository (APT, RPM, or Arch), installs system dependencies (bubblewrap), provisions the Lightpanda engine, and guides first-run setup. Auto-detects architecture and OS family.",
		code: "curl -sSL https://myth.work.gd/install.sh | sudo bash",
	},
	{
		name: "uninstall.sh",
		category: "Distribution",
		desc: "Full decommission script. Removes binary, agent symlink, repository configurations, signing keys, and session data. Proactive zero-trace cleanup for all supported Linux distributions.",
		code: "sudo bash scripts/uninstall.sh",
	},
	{
		name: "build_deb.sh",
		category: "Packaging",
		desc: "Builds a signed .deb package using cargo-deb. Embeds user.yaml config, README.md, and man page. Installs the binary at /usr/bin/myth and creates the /usr/bin/agent symlink. Enforces --locked for reproducible builds.",
		code: "bash scripts/build_deb.sh",
	},
	{
		name: "init_repo.sh",
		category: "Packaging",
		desc: "Initializes a fully signed multi-distro repository structure. Generates APT Package manifests, RPM repodata, and Arch database signatures. Orchestrates the end-to-end industrial metadata registry.",
		code: "bash scripts/init_repo.sh",
	},
	{
		name: "release_local.sh",
		category: "Release",
		desc: "Full local release orchestrator. Runs build_deb.sh + init_repo.sh in sequence for a complete end-to-end build → package → sign → publish workflow. Validates all artifacts before proceeding.",
		code: "bash scripts/release_local.sh",
	},
	{
		name: "cross_build.sh",
		category: "Release",
		desc: "Cross-compilation script for building myth binaries for multiple architectures (x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu) using cargo cross. Used in CI/CD for GitHub Actions matrix builds.",
		code: "bash scripts/cross_build.sh",
	},
	{
		name: "verify.sh",
		category: "Quality",
		desc: "Pre-release verification suite. Runs cargo test --locked, cargo clippy (zero-warnings policy), cargo fmt --check, and build validation. Exits non-zero on any failure — used as a gate in CI before release.",
		code: "bash scripts/verify.sh",
	},
	{
		name: "test.sh",
		category: "Quality",
		desc: "Runs the full test suite with cargo test --locked. Includes unit tests, integration tests, and sandbox policy verification. Captures output and reports pass/fail per module.",
		code: "bash scripts/test.sh",
	},
	{
		name: "bootstrap.sh",
		category: "Distribution",
		desc: "Minimal repository bootstrapper. Adds signing keys and registry sources to the system, enabling native package manager installs (apt/dnf/pacman) without the full installer overhead.",
		code: "bash scripts/bootstrap.sh",
	},
	{
		name: "setup_gpg.sh",
		category: "Security",
		desc: "Generates and exports a dedicated GPG signing key for MYTH package signing. Creates the GPG key pair, exports the public key to myth.gpg for distribution, and configures the signing keyring for cargo-deb.",
		code: "bash scripts/setup_gpg.sh",
	},
	{
		name: "sign_release.sh",
		category: "Security",
		desc: "Signs all release artifacts (binary, .deb, SHA256SUMS) with the GPG signing key. Generates detached .asc signature files alongside each artifact for cryptographic verification by users.",
		code: "bash scripts/sign_release.sh",
	},
	{
		name: "completions.sh",
		category: "Shell",
		desc: "Generates shell completion scripts for bash, zsh, and fish using myth completions <shell>. Places completions in the correct system directories (/etc/bash_completion.d/, /usr/share/zsh/vendor-completions/, etc.).",
		code: "bash scripts/completions.sh",
	},
	{
		name: "preinst",
		category: "Linux Lifecycle",
		desc: "Universal pre-install safety hook. Validates architecture and environment compatibility (Debian, RPM, Arch, Termux) before the package installs.",
		code: "cat scripts/preinst",
	},
	{
		name: "postinst",
		category: "Linux Lifecycle",
		desc: "Universal post-install configuration hook. Creates the agent symlink, provisions runtime assets (Lightpanda), and initializes the mission-critical environment on any Linux OS.",
		code: "cat scripts/postinst",
	},
	{
		name: "postrm",
		category: "Linux Lifecycle",
		desc: "Universal post-remove/purge hook. Deep-cleans agent symlinks, session memory, and configuration roots across all platform variants for a zero-trace decommissioning.",
		code: "cat scripts/postrm",
	},
	{
		name: "conffiles",
		category: "Linux Packaging",
		desc: "Universal configuration registry. Marks user.yaml for persistence across updates, synchronized to dpkg conffiles, RPM %config, and Arch backup=() arrays.",
		code: "cat scripts/conffiles",
	},
	{
		name: "changelog.sh",
		category: "Release",
		desc: "Auto-generates a universal changelog entry by parsing git logic. Formats it for inclusion in the linux/changelog registry used by all distribution families.",
		code: "bash scripts/changelog.sh",
	},
];

const categoryColors: Record<string, string> = {
	Distribution: "bg-cyber-primary/10 text-cyber-primary border-cyber-primary/30",
	Packaging: "bg-cyber-secondary/10 text-cyber-secondary border-cyber-secondary/30",
	Release: "bg-purple-500/10 text-purple-300 border-purple-500/30",
	Quality: "bg-cyber-success/10 text-cyber-success border-cyber-success/30",
	Security: "bg-cyber-error/10 text-cyber-error border-cyber-error/30",
	Shell: "bg-cyber-warning/10 text-cyber-warning border-cyber-warning/30",
	"Linux Lifecycle": "bg-blue-500/10 text-blue-300 border-blue-500/30",
	"Linux Packaging": "bg-blue-400/10 text-blue-200 border-blue-400/30",
};

const categories = [...new Set(scripts.map((s) => s.category))];

export default function ScriptsPage() {
	return (
		<div>
			<PageHeader
				title="Scripts Reference"
				description={`Complete reference for all ${scripts.length} automation scripts — installation, packaging, release, security, and Linux lifecycle hooks.`}
				badge="Reference"
			/>

			{/* Category legend */}
			<div className="flex flex-wrap gap-2 mb-8">
				{categories.map((cat) => (
					<span
						key={cat}
						className={`text-[10px] px-2 py-1 rounded border font-mono font-bold uppercase tracking-wider ${categoryColors[cat] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
					>
						{cat}
					</span>
				))}
			</div>

			<div className="space-y-8">
				{scripts.map((s) => (
					<div key={s.name} className="feature-card rounded-xl p-5">
						<div className="flex items-center gap-3 mb-2 flex-wrap">
							<h2 className="text-lg font-bold text-white font-mono">{s.name}</h2>
							<span
								className={`text-[10px] px-2 py-0.5 rounded border font-mono font-bold uppercase tracking-wider ${categoryColors[s.category] ?? "bg-white/5 text-cyber-dim border-white/10"}`}
							>
								{s.category}
							</span>
						</div>
						<p className="text-sm text-cyber-text/80 mb-3 leading-relaxed">{s.desc}</p>
						<CodeBlock lang="bash" title="Run" code={s.code} />
					</div>
				))}
			</div>
		</div>
	);
}
