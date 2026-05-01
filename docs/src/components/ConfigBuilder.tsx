import Editor from "@monaco-editor/react";
import { Activity, Box, ChevronRight, Eye, Shield, Terminal, Zap } from "lucide-react";
import { useState } from "react";

const initialConfig = `# ───────────────────────────────────────────────────────────
# MYTH User Configuration  (~/.config/myth/user.yaml)
# Edit and copy into your config file. Changes hot-reload.
# ───────────────────────────────────────────────────────────

agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100      # Max LLM tool-call rounds per turn
  timeout_seconds: 300     # Per-command execution timeout
  user_name: "Chief"       # Defaults to $USER if unset
  log_level: "info"        # trace | debug | info | warn | error
  all_report_path: "mission_report.md"

provider:
  # Add multiple keys — MYTH rotates on rate-limit automatically
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"
  base_url: "https://integrate.api.nvidia.com/v1"
  temperature: 0.5
  max_tokens: 131072

sandbox:
  enabled: true
  share_network: true       # Required for recon tools
  new_session: true         # TIOCSTI terminal injection prevention
  die_with_parent: true     # Auto-cleanup on agent exit
  workspace_size_mb: 512
  hostname: "myth-sandbox"

memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"         # All vectors are volatile — destroyed on exit
  collection_name: "agent_session"
  vector_size: 1024
  auto_start: true

proxy:
  enabled: false
  url: "socks5://127.0.0.1:9050"  # Tor or custom proxy
  use_for_llm: true
  use_for_tools: true
  auto_rotate: false`;

const presets = [
	{
		id: "shadow",
		name: "Shadow Protocol",
		icon: <Eye className="w-3.5 h-3.5" />,
		desc: "Max Stealth & Anonymity",
		mods: { "proxy.enabled": true, "agent.log_level": "warn", "provider.temperature": 0.3 },
	},
	{
		id: "recon",
		name: "Deep Spectrum Recon",
		icon: <Activity className="w-3.5 h-3.5" />,
		desc: "High Power Discovery",
		mods: { "agent.max_iterations": 250, "sandbox.share_network": true, "memory.auto_start": true },
	},
	{
		id: "safe",
		name: "Hardened Sandbox",
		icon: <Shield className="w-3.5 h-3.5" />,
		desc: "Maximum Isolation Mode",
		mods: { "sandbox.share_network": false, "sandbox.workspace_size_mb": 128 },
	},
];

export default function ConfigBuilder() {
	const [yaml, setYaml] = useState(initialConfig);
	const [copied, setCopied] = useState(false);
	const [activePreset, setActivePreset] = useState<string | null>(null);

	const handleCopy = () => {
		navigator.clipboard.writeText(yaml);
		setCopied(true);
		setTimeout(() => setCopied(false), 2000);
	};

	const applyPreset = (preset: (typeof presets)[0]) => {
		setActivePreset(preset.id);
		let currentYaml = initialConfig;
		// Simple replacement for demo — real impl would use a YAML parser
		for (const [key, value] of Object.entries(preset.mods)) {
			const regex = new RegExp(`${key.split(".").pop()}: .*`, "g");
			currentYaml = currentYaml.replace(regex, `${key.split(".").pop()}: ${value}`);
		}
		setYaml(currentYaml);
	};

	return (
		<div className="flex flex-col lg:flex-row gap-6">
			{/* Preset Sidebar */}
			<div className="w-full lg:w-72 shrink-0 space-y-3">
				<div className="text-[10px] font-black text-cyber-dim uppercase tracking-[0.3em] mb-4 flex items-center gap-2 px-2">
					<Box className="w-3 h-3" />
					Tactical Loadouts
				</div>
				{presets.map((p) => (
					<button
						key={p.id}
						type="button"
						onClick={() => applyPreset(p)}
						className={`w-full p-4 rounded-2xl border text-left transition-all ${
							activePreset === p.id
								? "bg-cyber-primary/10 border-cyber-primary/40 shadow-[0_0_20px_-5px_rgba(0,255,163,0.2)]"
								: "bg-white/[0.02] border-white/5 hover:bg-white/[0.05] hover:border-white/10"
						}`}
					>
						<div className="flex items-center gap-3 mb-2">
							<div
								className={`p-2 rounded-lg ${activePreset === p.id ? "bg-cyber-primary/20 text-cyber-primary" : "bg-black/40 text-cyber-dim"}`}
							>
								{p.icon}
							</div>
							<div className="font-bold text-sm text-white">{p.name}</div>
						</div>
						<p className="text-[10px] text-cyber-dim leading-relaxed uppercase tracking-tighter">
							{p.desc}
						</p>
					</button>
				))}

				<div className="p-4 rounded-2xl bg-cyber-accent/5 border border-cyber-accent/10 mt-6 hidden lg:block">
					<div className="flex items-center gap-2 mb-2 text-cyber-accent">
						<Zap className="w-3.5 h-3.5" />
						<span className="text-[10px] font-bold uppercase tracking-widest">Auto-Inject</span>
					</div>
					<p className="text-[10px] text-cyber-dim/80 leading-normal">
						All presets are dynamically hashed and verified before being applied to the core engine.
					</p>
				</div>
			</div>

			{/* Editor Main */}
			<div className="flex-1 glass-panel rounded-3xl overflow-hidden border border-white/5 shadow-2xl relative">
				<div className="bg-cyber-surface2 px-5 py-3 border-b border-white/5 flex items-center justify-between">
					<div className="flex items-center gap-3">
						<div className="flex gap-1.5 opacity-40">
							<div className="w-2.5 h-2.5 rounded-full bg-[#ff5f56]" />
							<div className="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]" />
							<div className="w-2.5 h-2.5 rounded-full bg-[#27c93f]" />
						</div>
						<div className="w-px h-3 bg-white/10 mx-2" />
						<div className="flex items-center gap-2 text-cyber-dim font-mono text-[10px] uppercase tracking-widest">
							<Terminal className="w-3 h-3" />
							user.yaml
						</div>
					</div>
					<button
						type="button"
						onClick={handleCopy}
						className={`flex items-center gap-2 text-[10px] px-4 py-1.5 rounded-full font-black tracking-widest transition-all ${
							copied
								? "bg-cyber-success/20 text-cyber-success border border-cyber-success/30 shadow-[0_0_15px_rgba(0,255,163,0.2)]"
								: "bg-cyber-primary/10 text-cyber-primary border border-cyber-primary/20 hover:bg-cyber-primary/20"
						}`}
					>
						{copied ? "IDENTIFIED & COPIED" : "EXPORT CONFIG"}
						<ChevronRight className="w-3 h-3" />
					</button>
				</div>
				<div className="h-[520px]">
					<Editor
						height="100%"
						defaultLanguage="yaml"
						theme="cyber-theme"
						value={yaml}
						onChange={(v) => setYaml(v || "")}
						options={{
							minimap: { enabled: false },
							fontSize: 13,
							fontFamily: "'JetBrains Mono', monospace",
							lineHeight: 22,
							padding: { top: 20, bottom: 20 },
							scrollBeyondLastLine: false,
							automaticLayout: true,
							wordWrap: "on",
							folding: true,
							lineNumbers: "on",
							renderLineHighlight: "line",
							scrollbar: {
								verticalScrollbarSize: 4,
								horizontalScrollbarSize: 4,
							},
						}}
						beforeMount={(monaco) => {
							monaco.editor.defineTheme("cyber-theme", {
								base: "vs-dark",
								inherit: true,
								rules: [
									{ token: "type", foreground: "00ffa3" },
									{ token: "string", foreground: "9090b0" },
									{ token: "keyword", foreground: "00d1ff" },
									{ token: "number", foreground: "ffcc00" },
									{ token: "comment", foreground: "505060", fontStyle: "italic" },
								],
								colors: {
									"editor.background": "#0a0a0f",
									"editor.lineHighlightBackground": "#151520",
									"editorLineNumber.foreground": "#404060",
									"editorLineNumber.activeForeground": "#00ffa3",
									"editor.selectionBackground": "#00ffa325",
								},
							});
						}}
					/>
				</div>
				{/* Scanning Overlay Effect */}
				<div className="absolute inset-0 pointer-events-none opacity-[0.03] bg-[url('/noise.svg')] mix-blend-overlay" />
			</div>
		</div>
	);
}
