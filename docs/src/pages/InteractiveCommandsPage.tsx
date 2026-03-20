import { PageHeader, CodeBlock } from '../components/Layout';

const commands = [
  { group: 'Reconnaissance & Targeting', cmds: [
    { cmd: '/scan <target>', cli: 'myth scan', desc: 'Initialize automated asset discovery with full 13-phase methodology' },
    { cmd: '/recon <target>', cli: 'myth recon', desc: 'Agent-led deep interrogation mission (alias for scan)' },
    { cmd: '/target <target>', cli: 'myth target', desc: 'Re-align mission focus to a new target/CIDR' },
    { cmd: '/depth <1-100>', cli: 'myth depth', desc: 'Modulate neural iteration/recursion depth' },
    { cmd: '/subdomains <target>', cli: 'myth subdomains', desc: 'High-speed multi-source subdomain discovery engine (Alias: /subdomains help for flags)' },
    { cmd: '/master <target>', cli: 'myth master', desc: 'ULTRA-ROBUST DISCOVERY: Tor + Proxies + Mega Wordlist + Deep Recursion (Alias: /ultra)' },
  ]},
  { group: 'Precision Tactical Operations', cmds: [
    { cmd: '/stealth <target>', cli: 'myth stealth', desc: 'Zero-footprint, OSINT-only intelligence gathering' },
    { cmd: '/osint <target>', cli: 'myth osint', desc: 'Specialized Open Source data mapping' },
    { cmd: '/vuln <target>', cli: 'myth vuln', desc: 'Deep-vector vulnerability assessment engine' },
  ]},
  { group: 'Intelligence Analytics', cmds: [
    { cmd: '/findings', cli: 'myth findings', desc: 'Aggregate discovered tactical intelligence' },
    { cmd: '/graph', cli: 'myth graph', desc: 'Render infrastructure relationship topology' },
    { cmd: '/history', cli: 'myth history', desc: 'Retrieve tactical event logs & mission history (Aliases: /logs, /events)' },
    { cmd: '/report', cli: 'myth report', desc: 'Generate comprehensive executive summary via AI' },
    { cmd: '/vitals', cli: 'myth vitals', desc: 'View neural pulses & session lifecycle telemetry (Alias: /status)' },
  ]},
  { group: 'Asset Registry & MCP', cmds: [
    { cmd: '/tools', cli: 'myth tools', desc: 'Catalog all synchronized mission tools' },
    { cmd: '/inspect <tool>', cli: 'myth inspect', desc: 'Retrieve deep technical documentation (Aliases: /man, /info)' },
    { cmd: '/mcp list', cli: 'myth mcp list', desc: 'Display all MCP servers with status & PIDs' },
    { cmd: '/mcp toggle <n> <on|off>', cli: 'myth mcp toggle', desc: 'Enable/Disable specific MCP servers' },
    { cmd: '/mcp add-local <args>', cli: 'myth mcp add-local', desc: 'Integrate new local MCP capabilities' },
    { cmd: '/mcp add-remote <args>', cli: 'myth mcp add-remote', desc: 'Add remote SSE MCP server' },
    { cmd: '/mcp sync', cli: 'myth mcp sync', desc: 'Force re-sync factory defaults' },
    { cmd: '/mcp remove <name>', cli: 'myth mcp remove', desc: 'Remove an MCP server' },
    { cmd: '/sync', cli: 'myth sync', desc: 'Force hot-plug re-sync of neural links' },
  ]},
  { group: 'System & Emergency', cmds: [
    { cmd: '/profile <name>', cli: 'myth profile', desc: 'Switch between mission recon profiles' },
    { cmd: '/config', cli: 'myth config', desc: 'View mission configuration (keys masked)' },
    { cmd: '/check', cli: 'myth check', desc: 'Run system health diagnostics (Alias: /health)' },
    { cmd: '/help', cli: 'myth usage', desc: 'Display tactical usage documentation (Aliases: /h, /?)' },
    { cmd: '/usage', cli: 'myth usage', desc: 'Display platform tactical methodology manual (Alias: /u)' },
    { cmd: '/version', cli: 'myth version', desc: 'Display neural core version telemetry (Alias: /v, /version)' },
    { cmd: '/clear', cli: 'myth clear', desc: 'Purge visual buffers (Memory remains, Alias: /cls)' },
    { cmd: '/wipe', cli: 'myth wipe', desc: 'Clear session memory and tactical context' },
    { cmd: '/quit', cli: '', desc: 'Initiate secure session termination (Aliases: /exit, /q)' },
    { cmd: '/burn', cli: 'myth burn', desc: 'EMERGENCY: Volatile buffer destruction' },
  ]},
];

export default function InteractiveCommandsPage() {
  return (
    <div>
      <PageHeader title="Interactive Commands" description="Commands available inside TUI and interactive CLI sessions. All commands work with or without a leading slash." badge="Commands" />

      <div className="feature-card rounded-xl p-4 mb-8">
        <h3 className="text-sm font-semibold text-cyber-primary mb-2">💡 Command Parity</h3>
        <p className="text-xs text-cyber-text/80">All commands work identically in TUI and CLI modes. You can type <code className="text-cyber-primary">scan target.com</code> or <code className="text-cyber-primary">/scan target.com</code> — both are recognized. Non-command text is sent as chat to the AI agent.</p>
      </div>

      <div className="feature-card rounded-xl p-4 mb-8">
        <h3 className="text-sm font-semibold text-cyber-primary mb-2">✨ Smart Features</h3>
        <ul className="space-y-1">
          <li className="text-xs text-cyber-text/80">• <strong>Fuzzy Matching:</strong> Mistyped <code className="text-cyber-primary">/scna</code>? MYTH suggests <code className="text-cyber-primary">/scan</code></li>
          <li className="text-xs text-cyber-text/80">• <strong>Ghost Suggestions:</strong> Real-time autocomplete hints as you type</li>
          <li className="text-xs text-cyber-text/80">• <strong>Argument Completion:</strong> Profile names and targets are suggested contextually</li>
          <li className="text-xs text-cyber-text/80">• <strong>Semantic Highlighting:</strong> Commands, flags, and targets are color-coded</li>
        </ul>
      </div>

      {commands.map((group) => (
        <div key={group.group} className="mb-8">
          <h2 className="text-lg font-bold text-white mb-3">{group.group}</h2>
          <table className="w-full text-sm docs-table rounded-lg overflow-hidden">
            <thead><tr><th>Command</th><th>CLI Equiv.</th><th>Description</th></tr></thead>
            <tbody>
              {group.cmds.map((c) => (
                <tr key={c.cmd}>
                  <td><code className="text-cyber-primary text-xs font-mono">{c.cmd}</code></td>
                  <td><code className="text-cyber-dim text-xs font-mono">{c.cli}</code></td>
                  <td className="text-cyber-text/70 text-xs">{c.desc}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      ))}

      <h2 className="text-xl font-bold text-white mb-4">MCP Management in Interactive Mode</h2>
      <CodeBlock lang="bash" title="Adding a local MCP server" code={`/mcp add-local my-tool npx env:API_KEY=abc123 desc:My_custom_tool`} />
      <CodeBlock lang="bash" title="Adding a remote SSE server" code={`/mcp add-remote my-api https://api.example.com/sse`} />
      <CodeBlock lang="bash" title="Tool allow/block" code={`/mcp allow-tool github create_issue
/mcp block-tool github delete_repo`} />
    </div>
  );
}
