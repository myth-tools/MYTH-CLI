import {
	AlertTriangle,
	ChevronRight,
	Eye,
	FileJson,
	History,
	Info,
	Layers,
	Lock,
	ShieldAlert,
	ShieldCheck,
	Zap,
} from "lucide-react";
import { CodeBlock, PageHeader } from "../components/Layout";
import SecurityGraph from "../components/SecurityGraph";

const sandboxProps = [
	"Read-only host filesystem — no writes to /usr, /bin, /etc, /home",
	"Per-command PID/IPC namespace isolation — processes cannot see each other",
	"tmpfs writable directories (/tmp, /var/tmp, /run) — auto-cleared on exit",
	"Mission workspace bound at /workspace — scoped to current session",
	"/proc and /dev mounted for tool compatibility",
	"TIOCSTI terminal escape prevention (--new-session flag)",
	"--die-with-parent ensures automatic subprocess cleanup",
	"Custom hostname (myth-sandbox) for network fingerprint control",
	"Random User-Agent injection per request for anti-bot evasion",
	"Optional proxychains integration for full traffic anonymization",
];

const auditTrailExample = {
	timestamp: "2026-04-02T14:24:12Z",
	session_id: "MYTH-88x2-ALPHA",
	event: "TOOL_DISPATCH",
	tool: "nmap",
	args: ["-sS", "-Pn", "target.com"],
	sandbox_verdict: "ALLOWED",
	isolation_layer: "bwrap",
	integrity_hash: "sha256:7e8d9c...",
	operator_override: false,
};

export default function SecurityPage() {
	return (
		<div className="flex flex-col lg:flex-row gap-10">
			{/* Main Content */}
			<div className="flex-1 min-w-0">
				<PageHeader
					title="Security Architecture"
					description="MYTH's 4-layer defense-in-depth model ensures absolute operational isolation. Offensive power, operator shielded via native kernel namespaces."
					badge="Defense Core"
				/>

				{/* Interactive Graph */}
				<div className="mb-14">
					<div className="flex items-center gap-3 mb-8">
						<Layers className="w-5 h-5 text-cyber-primary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Sovereign Defense Layers
						</h2>
					</div>
					<div className="glass-panel p-6 rounded-3xl border border-cyber-border/40 shadow-2xl relative overflow-hidden">
						<SecurityGraph />
						<div className="mt-8 grid grid-cols-1 md:grid-cols-4 gap-4">
							{[
								{ label: "L1: Sandbox", status: "ENABLED", color: "text-cyber-primary" },
								{ label: "L2: OPSEC", status: "PASSIVE", color: "text-cyber-secondary" },
								{ label: "L3: Logic", status: "ACTIVE", color: "text-cyber-accent" },
								{ label: "L4: Forensics", status: "WAITING", color: "text-cyber-error" },
							].map((s) => (
								<div
									key={s.label}
									className="p-3 bg-black/40 rounded-xl border border-white/5 text-center"
								>
									<div className="text-[9px] text-cyber-dim uppercase font-bold tracking-widest mb-1.5">
										{s.label}
									</div>
									<div
										className={`text-[10px] font-mono font-bold ${s.color} flex items-center justify-center gap-2`}
									>
										<div
											className={`w-1.5 h-1.5 rounded-full bg-current ${s.status === "ENABLED" ? "animate-pulse shadow-[0_0_8px_currentColor]" : ""}`}
										/>
										{s.status}
									</div>
								</div>
							))}
						</div>
					</div>
				</div>

				{/* NEW: Sequence Flow - Protocol Zero */}
				<div className="mb-16">
					<div className="flex items-center gap-3 mb-8">
						<History className="w-5 h-5 text-cyber-error" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Protocol Zero: Forensic Elimination
						</h2>
					</div>
					<div className="grid grid-cols-1 md:grid-cols-5 gap-4 relative">
						{[
							{
								step: "Trigger",
								act: "myth burn",
								icon: <ShieldAlert />,
								color: "border-cyber-error/40 text-cyber-error",
							},
							{
								step: "Signal",
								act: "SIGKILL ALL",
								icon: <Zap />,
								color: "border-white/10 text-white",
							},
							{
								step: "Wipe",
								act: "tmpfs purge",
								icon: <Info />,
								color: "border-white/10 text-white",
							},
							{
								step: "Obscure",
								act: "Scroll Clear",
								icon: <Eye />,
								color: "border-white/10 text-white",
							},
							{
								step: "Exit",
								act: "Proc Termination",
								icon: <Lock />,
								color: "border-cyber-primary/40 text-cyber-primary",
							},
						].map((s, i) => (
							<div
								key={s.step}
								className={`p-5 rounded-2xl border ${s.color} flex flex-col items-center text-center bg-white/[0.02] relative group`}
							>
								<div className="w-10 h-10 rounded-xl bg-black/40 border border-white/5 flex items-center justify-center mb-3 group-hover:scale-110 transition-transform">
									{s.icon}
								</div>
								<div className="text-[9px] text-cyber-dim uppercase font-black tracking-widest mb-1">
									{s.step}
								</div>
								<div className="text-[10px] font-mono font-bold">{s.act}</div>
								{i < 4 && (
									<div className="hidden md:block absolute -right-4 top-1/2 -translate-y-1/2 z-10">
										<ChevronRight className="w-4 h-4 text-white/20" />
									</div>
								)}
							</div>
						))}
					</div>
				</div>

				{/* Defense Matrix */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x01</span>
					Multi-Layer Infrastructure Matrix
				</h2>
				<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50 mb-16 shadow-lg">
					<table className="w-full text-left text-xs docs-table border-none">
						<thead className="bg-white/5 border-b border-cyber-border/50">
							<tr>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Defense Layer
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Native Mechanism
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Operator Protection
								</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-cyber-border/20">
							{[
								{
									layer: "L1: Sandbox",
									mech: "Bubblewrap (bwrap)",
									prot: "Mount/PID/IPC namespace isolation. Host system is invisible to subprocesses. Mandatory read-only paths.",
								},
								{
									layer: "L2: OPSEC",
									mech: "Tor SOCKS5 Proxy",
									prot: "Full source IP anonymization and DNS-over-Tor for all passive discovery traffic. No host DNS leaks.",
								},
								{
									layer: "L3: Hardening",
									mech: "Recursive Validation",
									prot: "Input sanitization engine prevents shell injection and path traversal in AI-generated commands.",
								},
								{
									layer: "L4: Forensics",
									mech: "Protocol Zero",
									prot: "Immediate RAM-only wipe of mission workspace on termination. Zero disk artifacts left on host.",
								},
							].map((d) => (
								<tr key={d.layer} className="hover:bg-cyber-primary/[0.02] transition-colors group">
									<td className="py-5 px-6 font-black text-white text-sm group-hover:text-cyber-primary transition-colors">
										{d.layer}
									</td>
									<td className="py-5 px-6 text-cyber-primary font-mono text-xs">{d.mech}</td>
									<td className="py-5 px-6 text-cyber-text/80 leading-relaxed text-[11px]">
										{d.prot}
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>

				{/* Bubblewrap Details */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 mt-10 flex items-center gap-3">
					<span className="text-cyber-primary font-mono italic">0x02</span>
					Sandbox Hardening Specifications
				</h2>
				<div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-10">
					{sandboxProps.map((item) => (
						<div
							key={item}
							className="flex gap-4 items-center bg-white/[0.02] border border-white/5 p-4 rounded-xl hover:border-cyber-primary/20 transition-all"
						>
							<ShieldCheck className="w-5 h-5 text-cyber-primary shrink-0 opacity-60" />
							<span className="text-xs text-cyber-text/80 leading-relaxed">{item}</span>
						</div>
					))}
				</div>

				<div className="mb-16">
					<CodeBlock
						lang="yaml"
						title="Sandbox Hardening Reference (config/user.yaml)"
						code={`sandbox:
  enabled: true              # Mandatory for operational safety
  bwrap_path: "/usr/bin/bwrap"
  share_network: true        # Required for network reconnaissance
  new_session: true          # TIOCSTI escape prevention
  die_with_parent: true      # Orphaned process cleanup
  read_only_paths:           # Immutable host assets
    - "/usr"
    - "/bin"
    - "/lib"
    - "/etc"
  writable_tmpfs:            # Volatile mission cache
    - "/tmp"
    - "/var/tmp"
    - "/run"
  workspace_size_mb: 512     # Memory-bound mission scope
  hostname: "myth-sandbox"   # Identifier modulation`}
					/>
				</div>

				{/* NEW: JSON Audit Trail */}
				<div className="mb-16">
					<div className="flex items-center gap-3 mb-6">
						<FileJson className="w-5 h-5 text-cyber-secondary" />
						<h2 className="text-xl font-black text-white uppercase tracking-tighter">
							Operational Audit Logging
						</h2>
					</div>
					<p className="text-sm text-cyber-text/80 leading-relaxed mb-6">
						Every tool dispatch is recorded in the session audit log including the SHA256 hash of
						the binary and the exact sandboxed arguments used.
					</p>
					<CodeBlock
						lang="json"
						title="Sample Audit Entry (myth-audit.log)"
						code={JSON.stringify(auditTrailExample, null, 2)}
					/>
				</div>

				{/* Cryptographic Stack */}
				<h2 className="text-xl font-black text-white uppercase tracking-tighter mb-8 mt-10">
					<span className="text-cyber-primary font-mono italic">0x03</span>
					Cryptographic Integrity Stack
				</h2>
				<div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50 mb-16 shadow-lg">
					<table className="w-full text-left text-xs docs-table border-none">
						<thead className="bg-white/5 border-b border-cyber-border/50">
							<tr>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Algorithm
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Category
								</th>
								<th className="py-4 px-6 text-cyber-dim uppercase tracking-widest text-[9px]">
									Tactical Use Case
								</th>
							</tr>
						</thead>
						<tbody className="divide-y divide-cyber-border/20">
							{[
								{
									alg: "AES-GCM-SIV",
									cat: "AEAD",
									use: "Standard disk-encryption for persistent mission intel. Resists nonce-misuse.",
								},
								{
									alg: "ChaCha20-Poly1305",
									cat: "AEAD Stream",
									use: "High-performance data encryption for mobile/LTE environments.",
								},
								{
									alg: "SHA-256",
									cat: "Hash",
									use: "Binary integrity validation (Lightpanda) and audit logging artifact tracking.",
								},
							].map((c) => (
								<tr key={c.alg} className="hover:bg-cyber-accent/[0.02] transition-colors">
									<td className="py-4 px-6 font-mono text-cyber-accent font-bold text-sm tracking-tight">
										{c.alg}
									</td>
									<td className="py-4 px-6 text-cyber-dim uppercase font-bold text-[10px] tracking-widest">
										{c.cat}
									</td>
									<td className="py-4 px-6 text-cyber-text/80 italic">{c.use}</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>

				{/* Protocol Zero - Feature Card */}
				<div className="feature-card rounded-3xl p-8 border border-cyber-error/30 bg-cyber-error/[0.01] mb-12 relative overflow-hidden group">
					<div className="absolute -top-12 -right-12 w-32 h-32 bg-cyber-error/5 rounded-full blur-2xl group-hover:bg-cyber-error/10 transition-all pointer-events-none" />
					<div className="flex items-center gap-3 mb-5">
						<div className="w-3 h-3 rounded-full bg-cyber-error animate-pulse" />
						<h3 className="text-lg font-black text-white uppercase tracking-widest">
							Emergency Decommissioning
						</h3>
					</div>
					<p className="text-sm text-cyber-text/80 mb-8 leading-relaxed max-w-3xl border-l border-cyber-error/20 pl-4">
						Triggering <code className="text-cyber-error font-bold">myth burn</code> sends an
						immediate <code className="text-cyber-error">SIGKILL</code> to all operative children,
						destroys the tmpfs workspace, and purges the TUI memory. This is the hardware-equivalent
						of an instant disk wipe.
					</p>
					<div className="flex flex-col sm:flex-row items-center gap-4">
						<CodeBlock lang="bash" code="myth burn" />
						<div className="flex items-center gap-2 text-cyber-error font-mono text-[10px] uppercase font-bold whitespace-nowrap bg-cyber-error/10 px-3 py-1 rounded-full border border-cyber-error/20 shadow-sm">
							<AlertTriangle className="w-3 h-3" />
							Irreversible Termination
						</div>
					</div>
				</div>
			</div>

			{/* Sidebar Quick-Jump */}
			<div className="hidden lg:block w-72 shrink-0 h-fit sticky top-24">
				<div className="glass-panel rounded-3xl p-7 border border-cyber-border/50 shadow-2xl relative overflow-hidden">
					<div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-transparent via-cyber-error/40 to-transparent" />
					<div className="flex items-center gap-3 mb-8 text-cyber-error">
						<ShieldAlert className="w-5 h-5" />
						<h4 className="text-[11px] font-black uppercase tracking-[0.3em]">Defense Guard</h4>
					</div>

					<div className="space-y-4 mb-10">
						{[
							{
								label: "Shielding",
								value: "NOMINAL",
								icon: <ShieldCheck className="w-4 h-4 text-cyber-primary" />,
							},
							{
								label: "Traceability",
								value: "FULL AUDIT",
								icon: <FileJson className="w-4 h-4 text-cyber-secondary" />,
							},
							{
								label: "Volatilitiy",
								value: "RAM-ONLY",
								icon: <Zap className="w-4 h-4 text-cyber-accent" />,
							},
							{
								label: "Termination",
								value: "P-ZERO SET",
								icon: <AlertTriangle className="w-4 h-4 text-cyber-error" />,
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
						<div className="flex items-center gap-2 mb-2 text-cyber-primary">
							<Info className="w-3.5 h-3.5" />
							<span className="text-[10px] font-black uppercase tracking-widest">
								Sandboxing Note
							</span>
						</div>
						<p className="text-[10px] text-cyber-primary/60 leading-relaxed italic">
							MYTH never runs untrusted code on the host root. Every execution is an isolated event.
						</p>
					</div>
				</div>
			</div>
		</div>
	);
}
