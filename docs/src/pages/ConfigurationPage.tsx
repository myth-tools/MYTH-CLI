import { PageHeader, CodeBlock } from '../components/Layout';
import ConfigBuilder from '../components/ConfigBuilder';

export default function ConfigurationPage() {
  return (
    <div>
      <PageHeader title="Configuration" description="MYTH uses a two-tier YAML configuration system with hot-reload support." badge="Getting Started" />

      <div className="mb-12">
        <h2 className="text-xl font-bold text-white mb-6">Interactive Config Builder</h2>
        <ConfigBuilder />
        <p className="text-xs text-cyber-dim mt-4">💡 Edit the YAML above to customize your mission parameters, then copy it to your <code className="text-cyber-primary">user.yaml</code> file.</p>
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Configuration Files</h2>
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 mb-8">
        <div className="feature-card rounded-lg p-4">
          <h3 className="font-semibold text-white text-sm">config/agent.yaml</h3>
          <p className="text-xs text-cyber-dim mt-1">Internal defaults embedded at compile time</p>
        </div>
        <div className="feature-card rounded-lg p-4">
          <h3 className="font-semibold text-white text-sm">~/.config/myth/user.yaml</h3>
          <p className="text-xs text-cyber-dim mt-1">User overrides (API keys, profiles, tuning)</p>
        </div>
        <div className="feature-card rounded-lg p-4">
          <h3 className="font-semibold text-white text-sm">~/.config/myth/mcp.json</h3>
          <p className="text-xs text-cyber-dim mt-1">MCP server registry with factory defaults sync</p>
        </div>
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Configuration Sections</h2>

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">Agent Config</h3>
      <CodeBlock lang="yaml" title="agent section" code={`agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100     # Max tool-calling rounds per turn
  timeout_seconds: 300    # Per-command timeout
  user_name: "Chief"      # Defaults to $USER
  log_level: "info"       # trace, debug, info, warn, error
  all_report_path: "mission_report.md"`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">LLM / NVIDIA NIM</h3>
      <CodeBlock lang="yaml" title="llm section" code={`llm:
  provider: "nvidia-nim"
  base_url: "https://integrate.api.nvidia.com/v1"
  nvidia_nim_api_key: ["nvapi-xxx"]  # Supports multiple keys for rotation
  model: "deepseek-ai/deepseek-v3"
  temperature: 0.1
  max_tokens: 8192
  top_p: 0.9
  fallback_model: "meta/llama-3.3-70b-instruct"  # Auto-rotates on failure`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">Sandbox</h3>
      <CodeBlock lang="yaml" title="sandbox section" code={`sandbox:
  enabled: true
  bwrap_path: "/usr/bin/bwrap"
  share_network: true       # Required for recon tools
  new_session: true         # TIOCSTI prevention
  die_with_parent: true
  read_only_paths: ["/usr", "/bin", "/lib", "/etc"]
  writable_tmpfs: ["/tmp", "/var/tmp", "/run"]
  workspace_size_mb: 512
  hostname: "myth-sandbox"`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">Memory</h3>
      <CodeBlock lang="yaml" title="memory section" code={`memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"          # All data is volatile
  grpc_port: 6334
  http_port: 6333
  collection_name: "agent_session"
  vector_size: 1024          # NIM embedding dimension
  auto_start: true`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">TUI</h3>
      <CodeBlock lang="yaml" title="tui section" code={`tui:
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
    surface: "#1a1a2e"`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">Proxy / Tor</h3>
      <CodeBlock lang="yaml" title="proxy section" code={`proxy:
  enabled: false
  url: "socks5://127.0.0.1:9050"  # Tor or custom proxy
  use_for_llm: true
  use_for_tools: true
  auto_rotate: false         # Rotate Tor IP per request
  tor_control_port: 9051
  tor_control_password: ""`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Hot-Reload</h2>
      <p className="text-cyber-text/80 mb-3">MYTH supports live configuration hot-reload via filesystem watchers. Changes to <code className="text-cyber-primary">user.yaml</code> and <code className="text-cyber-primary">mcp.json</code> are automatically detected and applied without restarting the agent.</p>
      <p className="text-cyber-text/80">Use <code className="text-cyber-primary">myth sync</code> or <code className="text-cyber-primary">/sync</code> to force a manual re-sync.</p>
    </div>
  );
}
