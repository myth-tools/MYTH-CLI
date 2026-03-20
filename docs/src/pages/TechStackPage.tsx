import { PageHeader } from '../components/Layout';
import { VERSION } from '../data/metadata';

const categories = [
  {
    name: 'Runtime & Language',
    deps: [
      { name: 'Rust', ver: 'Edition 2021', desc: 'Systems language for the entire CLI' },
      { name: 'Tokio', ver: '1.x', desc: 'Async runtime with full feature set' },
    ],
  },
  {
    name: 'AI & LLM',
    deps: [
      { name: 'Rig (rig-core)', ver: '0.32.0', desc: 'Agent framework for Rust — tool calling, streaming, prompt management' },
      { name: 'NVIDIA NIM', ver: '-', desc: 'OpenAI-compatible LLM inference API (DeepSeek V3, LLaMA 3.3)' },
    ],
  },
  {
    name: 'MCP Protocol',
    deps: [
      { name: 'rust-mcp-sdk', ver: '0.8.3', desc: 'MCP server & client implementation (stdio, SSE)' },
      { name: 'rust-mcp-schema', ver: '0.9.6', desc: 'MCP JSON-RPC schema types' },
    ],
  },
  {
    name: 'State Management',
    deps: [
      { name: 'petgraph', ver: '0.6', desc: 'Directed graph for ReconGraph (targets, findings, phases)' },
      { name: 'dashmap', ver: '6', desc: 'Concurrent hashmap for thread-safe state' },
    ],
  },
  {
    name: 'CLI & TUI',
    deps: [
      { name: 'clap', ver: '4', desc: 'Command-line argument parsing with derive macros' },
      { name: 'ratatui', ver: '0.30', desc: 'Terminal user interface framework' },
      { name: 'crossterm', ver: '0.29', desc: 'Cross-platform terminal manipulation' },
      { name: 'rustyline', ver: '17.0', desc: 'Interactive line editor with history and completion' },
    ],
  },
  {
    name: 'Web & HTTP',
    deps: [
      { name: 'reqwest', ver: '0.12', desc: 'HTTP client with streaming, compression, TLS' },
      { name: 'reqwest-eventsource', ver: '0.6.0', desc: 'SSE client for MCP remote servers' },
      { name: 'headless_chrome', ver: '1.0.21', desc: 'CDP-based headless browser automation' },
      { name: 'fantoccini', ver: '0.22.1', desc: 'WebDriver-based browser automation' },
      { name: 'scraper', ver: '0.25.0', desc: 'HTML parsing and CSS selector queries' },
    ],
  },
  {
    name: 'Serialization & Data',
    deps: [
      { name: 'serde', ver: '1', desc: 'Serialization framework' },
      { name: 'serde_json', ver: '1', desc: 'JSON parsing' },
      { name: 'serde_yaml', ver: '0.9', desc: 'YAML config parsing' },
      { name: 'schemars', ver: '1.2.1', desc: 'JSON Schema generation for tool definitions' },
    ],
  },
  {
    name: 'System & Security',
    deps: [
      { name: 'nix', ver: '0.29', desc: 'Unix system APIs (process, signal)' },
      { name: 'sha2', ver: '0.10.9', desc: 'SHA-256 hashing for integrity checks' },
      { name: 'uuid', ver: '1', desc: 'UUID v4 generation for session/finding IDs' },
      { name: 'notify', ver: '8.2.0', desc: 'Filesystem watcher for config hot-reload' },
    ],
  },
  {
    name: 'Observability',
    deps: [
      { name: 'tracing', ver: '0.1', desc: 'Structured logging framework' },
      { name: 'tracing-subscriber', ver: '0.3', desc: 'Log filtering and formatting' },
      { name: 'color-eyre', ver: '0.6', desc: 'Colorized error reporting with context' },
      { name: 'owo-colors', ver: '4.3.0', desc: 'Zero-allocation terminal colors' },
      { name: 'indicatif', ver: '0.18.4', desc: 'Progress bars and spinners' },
    ],
  },
];

export default function TechStackPage() {
  return (
    <div>
      <PageHeader title="Tech Stack" description="Complete dependency breakdown — every crate MYTH depends on and why." badge="Reference" />

      {categories.map((cat) => (
        <div key={cat.name} className="mb-8">
          <h2 className="text-lg font-bold text-white mb-3">{cat.name}</h2>
          <table className="w-full text-sm docs-table rounded-lg overflow-hidden">
            <thead><tr><th>Crate</th><th>Version</th><th>Purpose</th></tr></thead>
            <tbody>
              {cat.deps.map((d) => (
                <tr key={d.name}>
                  <td><code className="text-cyber-primary text-xs font-mono">{d.name}</code></td>
                  <td className="text-cyber-dim text-xs font-mono">{d.ver}</td>
                  <td className="text-cyber-text/70 text-xs">{d.desc}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ))}

      <h2 className="text-xl font-bold text-white mb-4">Build Profile</h2>
      <div className="feature-card rounded-lg p-4">
        <div className="grid grid-cols-2 gap-2 text-sm">
          <div><span className="text-cyber-dim">Optimization:</span> <code className="text-cyber-primary">opt-level = 3</code></div>
          <div><span className="text-cyber-dim">LTO:</span> <code className="text-cyber-primary">true</code></div>
          <div><span className="text-cyber-dim">Codegen Units:</span> <code className="text-cyber-primary">1</code></div>
          <div><span className="text-cyber-dim">Strip:</span> <code className="text-cyber-primary">true</code></div>
          <div><span className="text-cyber-dim">Panic:</span> <code className="text-cyber-primary">abort</code></div>
          <div><span className="text-cyber-dim">Version:</span> <code className="text-cyber-primary">{VERSION}</code></div>
          <div><span className="text-cyber-dim">Binary Size:</span> <code className="text-cyber-primary">~8MB</code></div>
        </div>
      </div>
    </div>
  );
}
