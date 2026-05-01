import ConfigBuilder from "../components/ConfigBuilder";
import { CodeBlock, PageHeader } from "../components/Layout";

export default function ConfigurationPage() {
	return (
		<div>
			<PageHeader
				title="Configuration"
				description="MYTH uses a two-tier YAML configuration system. agent.yaml (compiled in) provides defaults; user.yaml (user-owned) overrides everything. Changes hot-reload without restarting."
				badge="Getting Started"
			/>

			{/* Interactive Config Builder */}
			<div className="mb-12">
				<h2 className="text-xl font-bold text-white mb-6">Interactive Config Builder</h2>
				<ConfigBuilder />
				<p className="text-xs text-cyber-dim mt-4">
					💡 Edit the YAML above then click{" "}
					<span className="text-cyber-primary font-bold">COPY CONFIG</span> — paste it into{" "}
					<code className="text-cyber-primary">~/.config/myth/user.yaml</code>. Changes apply
					immediately without restarting the agent.
				</p>
			</div>

			{/* Config Files */}
			<h2 className="text-xl font-bold text-white mb-4">Configuration Files</h2>
			<div className="grid grid-cols-1 sm:grid-cols-3 gap-3 mb-8">
				<div className="feature-card rounded-lg p-4">
					<h3 className="font-semibold text-white text-sm">config/agent.yaml</h3>
					<p className="text-xs text-cyber-dim mt-1">
						Internal defaults — compiled into the binary at build time. Never edit this directly.
					</p>
				</div>
				<div className="feature-card rounded-lg p-4">
					<h3 className="font-semibold text-white text-sm">~/.config/myth/user.yaml</h3>
					<p className="text-xs text-cyber-dim mt-1">
						Your overrides: API keys, models, sandbox, memory, TUI, proxy settings.
					</p>
				</div>
				<div className="feature-card rounded-lg p-4">
					<h3 className="font-semibold text-white text-sm">~/.config/myth/mcp.json</h3>
					<p className="text-xs text-cyber-dim mt-1">
						MCP server registry. Includes factory defaults + your custom additions.
					</p>
				</div>
			</div>

			{/* Agent Config */}
			<h2 className="text-xl font-bold text-white mb-4">Configuration Sections</h2>

			<h3 className="text-lg font-semibold text-white mb-3 mt-6">Agent</h3>
			<CodeBlock
				lang="yaml"
				title="agent section"
				code={`agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100     # Max tool-calling rounds per turn
  timeout_seconds: 300    # Per-command timeout
  user_name: "Chief"      # Defaults to $USER
  log_level: "info"       # trace, debug, info, warn, error
  all_report_path: "mission_report.md"`}
			/>

			{/* LLM — CORRECT SCHEMA */}
			<h3 className="text-lg font-semibold text-white mb-3 mt-6">LLM / NVIDIA NIM</h3>
			<CodeBlock
				lang="yaml"
				title="provider section"
				code={`provider:
  # Multiple keys supported — MYTH auto-rotates on rate-limit
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    # - "nvapi-yyyyyyyy"  # second key for rotation
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"
  base_url: "https://integrate.api.nvidia.com/v1"
  temperature: 0.5
  max_tokens: 131072`}
			/>

			<div className="glass-panel rounded-xl p-4 mb-6 border border-cyber-secondary/20">
				<p className="text-xs text-cyber-text/80">
					<span className="text-cyber-secondary font-bold">NVIDIA NIM API Key:</span> Get a free key
					at{" "}
					<a
						href="https://build.nvidia.com/"
						target="_blank"
						rel="noopener noreferrer"
						className="text-cyber-primary hover:underline"
					>
						build.nvidia.com
					</a>
					. No GPU required — all inference runs on NVIDIA's cloud.
				</p>
			</div>

			{/* Sandbox */}
			<h3 className="text-lg font-semibold text-white mb-3 mt-6">Sandbox</h3>
			<CodeBlock
				lang="yaml"
				title="sandbox section"
				code={`sandbox:
  enabled: true
  bwrap_path: "/usr/bin/bwrap"
  share_network: true       # Required for recon tools
  new_session: true         # TIOCSTI prevention
  die_with_parent: true
  read_only_paths: ["/usr", "/bin", "/lib", "/etc"]
  writable_tmpfs: ["/tmp", "/var/tmp", "/run"]
  workspace_size_mb: 512
  hostname: "myth-sandbox"`}
			/>

			{/* Memory */}
			<h3 className="text-lg font-semibold text-white mb-3 mt-6">Memory</h3>
			<CodeBlock
				lang="yaml"
				title="memory section"
				code={`memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"          # All data is volatile
  grpc_port: 6334
  http_port: 6333
  collection_name: "agent_session"
  vector_size: 1024          # NIM embedding dimension
  auto_start: true`}
			/>

			{/* TUI */}
			<h3 className="text-lg font-semibold text-white mb-3 mt-6">TUI</h3>
			<CodeBlock
				lang="yaml"
				title="tui section"
				code={`tui:
  enabled: true
  theme: "dark"
  show_tree_panel: true
  show_status_bar: true
  max_output_lines: 5000
  scroll_speed: 3
  colors:
    primary: "#00ff88"
    secondary: "#0088ff"
    accent: "#ff0055"
    background: "#0a0a0f"
    surface: "#1a1a2e"`}
			/>

			{/* Proxy / Tor */}
			<h3 className="text-lg font-semibold text-white mb-3 mt-6">Proxy / Tor</h3>
			<CodeBlock
				lang="yaml"
				title="proxy section"
				code={`proxy:
  enabled: false
  url: "socks5://127.0.0.1:9050"  # Tor or custom proxy
  use_for_llm: true
  use_for_tools: true
  auto_rotate: false               # Rotate Tor IP per request
  tor_control_port: 9051
  tor_control_password: ""`}
			/>

			{/* Hot-Reload */}
			<h2 className="text-xl font-bold text-white mb-4 mt-8">Hot-Reload</h2>
			<p className="text-cyber-text/80 mb-3">
				MYTH watches <code className="text-cyber-primary">user.yaml</code> and{" "}
				<code className="text-cyber-primary">mcp.json</code> via filesystem watchers. Changes apply
				without restarting the agent.
			</p>
			<CodeBlock lang="bash" code="myth sync   # Force manual re-sync" />
			<CodeBlock lang="bash" code="myth config # View all current settings (keys masked)" />
		</div>
	);
}
