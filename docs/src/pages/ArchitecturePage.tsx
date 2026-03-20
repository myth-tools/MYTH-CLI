import { PageHeader, CodeBlock } from '../components/Layout';
import SystemGraph from '../components/SystemGraph';

export default function ArchitecturePage() {
  return (
    <div>
      <PageHeader title="Architecture" description="How MYTH works internally — from CLI input to sandboxed tool execution." badge="Architecture" />

      <div className="mb-12">
        <h2 className="text-xl font-bold text-white mb-6 flex items-center gap-2">
          <span className="text-cyber-primary">01.</span> Interactive System Map
        </h2>
        <SystemGraph />
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Core Modules</h2>
      <div className="space-y-3 mb-8">
        {[
          { module: 'src/main.rs', desc: 'Entry point — parses CLI args via clap, loads config, dispatches to commands' },
          { module: 'src/cli.rs', desc: '27 CLI subcommands (scan, subdomains, master, completions, mcp, etc.) with argument parsing and dispatch logic' },
          { module: 'src/core/agent.rs', desc: 'ReconAgent — the main AI agent that builds Rig agents with 16 tool bridges, manages sessions, and handles streaming chat' },
          { module: 'src/core/commands.rs', desc: '30+ tactical commands with fuzzy matching, semantic tokenization, ghost suggestions, and dual slash/non-slash support' },
          { module: 'src/core/session.rs', desc: 'Session lifecycle management with volatile tmpfs workspaces and mission metadata persistence' },
          { module: 'src/core/recon_graph.rs', desc: 'ReconGraph state machine tracking phases, findings, targets, and severity classification' },
          { module: 'src/llm/', desc: 'NIM client (OpenAI-compatible), system/session prompts, and 16 Rig tool bridge implementations' },
          { module: 'src/mcp/', desc: 'MCP server (discover/execute/help), client manager with differential sync, tool discovery scanner, and schemas' },
          { module: 'src/sandbox/', desc: 'Bubblewrap sandbox with security policy (50+ blocked commands), per-command namespace isolation' },
          { module: 'src/memory/', desc: 'Qdrant-based in-memory vector store with NIM embeddings for semantic session recall' },
          { module: 'src/config/', desc: 'Two-tier YAML config with validation, hot-reload watcher, and MCP storage with factory defaults sync' },
          { module: 'src/builtin_tools/', desc: '19 native Rust utility tools (file gen, encryption, compression, web automation, payloads, memory search, recon)' },
          { module: 'src/builtin_mcp/', desc: '11 factory-default MCP server definitions (local, remote, custom)' },
          { module: 'src/tui/', desc: 'ratatui TUI with multi-panel layout, tool execution visualization, and real-time streaming' },
        ].map((m) => (
          <div key={m.module} className="flex gap-4 items-start p-3 rounded-lg border border-cyber-border hover:border-cyber-primary/30 transition-colors">
            <code className="text-cyber-primary text-xs font-mono whitespace-nowrap shrink-0 mt-0.5">{m.module}</code>
            <p className="text-sm text-cyber-text/80">{m.desc}</p>
          </div>
        ))}
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Data Flow</h2>
      <ol className="space-y-3 mb-8">
        {[
          'User enters a command or chat message via CLI or TUI',
          'ReconAgent adds the message to conversation history',
          'Rig.rs agent is built with 16 tool bridges attached',
          'LLM (NVIDIA NIM) decides which tools to call based on context',
          'Tool bridges dispatch to: local binaries (via sandbox), external MCP servers, or native Rust tools',
          'Results are streamed back via dual-mode telemetry (TUI events or CLI output)',
          'Tool outputs are automatically stored in Qdrant for semantic recall',
          'ReconGraph is updated with findings and phase progress',
          'LLM synthesizes results and responds to the user',
        ].map((step, i) => (
          <li key={i} className="flex gap-3">
            <span className="text-cyber-primary font-mono text-sm shrink-0">{i + 1}.</span>
            <span className="text-sm text-cyber-text/80">{step}</span>
          </li>
        ))}
      </ol>

      <h2 className="text-xl font-bold text-white mb-4">Project Structure</h2>
      <CodeBlock lang="text" title="Directory layout" code={`MYTH_CLI/
├── Cargo.toml                 # Dependencies & .deb packaging
├── config/
│   ├── agent.yaml             # Internal defaults (compiled in)
│   ├── user.yaml              # User overrides
│   └── mcp.json               # MCP server registry
├── src/
│   ├── main.rs                # Entry point & CLI
│   ├── cli.rs                 # CLI command definitions
│   ├── interactive.rs         # Interactive mode (TUI + CLI)
│   ├── stream.rs              # Streaming response consumer
│   ├── markdown_renderer.rs   # Terminal markdown renderer
│   ├── config/                # YAML config parsing & hot-reload
│   ├── core/                  # Agent brain & state machine
│   ├── llm/                   # NVIDIA NIM client & tool bridges
│   ├── mcp/                   # MCP server, clients & discovery
│   ├── sandbox/               # Bubblewrap & security policy
│   ├── memory/                # Qdrant in-memory storage
│   ├── builtin_tools/         # 7 native Rust utility tools
│   ├── builtin_mcp/           # 10 factory MCP server defs
│   ├── tui/                   # Terminal UI (ratatui)
│   └── ui/                    # CLI themes & formatting
├── scripts/
│   ├── install.sh             # One-line installer
│   ├── build_deb.sh           # .deb package builder
│   ├── init_repo.sh           # Repository initializer
│   ├── release_local.sh       # Local release builder
│   ├── uninstall.sh           # Clean removal script
│   ├── preinst                # Debian pre-install safety checks
│   ├── postinst               # Debian post-install symlinks
│   ├── postrm                 # Debian purge mechanics
│   └── conffiles              # Debian configuration tracking
└── README.md`} />
    </div>
  );
}
