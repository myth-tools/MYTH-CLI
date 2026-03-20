import { PageHeader, CodeBlock } from '../components/Layout';
import { builtinMcpServers } from '../data/content';

export default function McpServersPage() {
  const local = builtinMcpServers.filter((s) => s.type === 'local');
  const remote = builtinMcpServers.filter((s) => s.type === 'remote');
  const custom = builtinMcpServers.filter((s) => s.type === 'custom');

  const ServerCard = ({ server }: { server: typeof builtinMcpServers[0] }) => (
    <div className="feature-card rounded-xl p-5 mb-4">
      <div className="flex items-center gap-3 mb-2">
        <h3 className="font-bold text-white">{server.name}</h3>
        <span className={`text-[10px] px-2 py-0.5 rounded font-mono ${
          server.type === 'local' ? 'bg-cyber-success/15 text-cyber-success border border-cyber-success/30' :
          server.type === 'remote' ? 'bg-cyber-secondary/15 text-cyber-secondary border border-cyber-secondary/30' :
          'bg-cyber-warning/15 text-cyber-warning border border-cyber-warning/30'
        }`}>
          {server.type}
        </span>
        <span className="text-[10px] px-2 py-0.5 rounded font-mono bg-cyber-dim/15 text-cyber-dim border border-cyber-dim/30">
          {server.transport}
        </span>
      </div>
      <p className="text-sm text-cyber-text/80 mb-3">{server.description}</p>
      {server.command && (
        <div className="mb-3">
          <span className="text-xs text-cyber-dim">Command: </span>
          <code className="text-xs text-cyber-primary">{server.command}</code>
        </div>
      )}
      {server.url && (
        <div className="mb-3">
          <span className="text-xs text-cyber-dim">URL: </span>
          <code className="text-xs text-cyber-primary">{server.url}</code>
        </div>
      )}
      {server.tools && server.tools.length > 0 && (
        <div className="mb-2">
          <span className="text-xs text-cyber-dim block mb-1">Available Tools:</span>
          <div className="flex flex-wrap gap-1.5">
            {server.tools.map((t) => (
              <code key={t} className="text-xs px-2 py-0.5 bg-cyber-primary/10 text-cyber-primary/80 rounded">{t}</code>
            ))}
          </div>
        </div>
      )}
      {server.envVars && server.envVars.length > 0 && (
        <div>
          <span className="text-xs text-cyber-dim block mb-1">Required Environment Variables:</span>
          <div className="flex flex-wrap gap-1.5">
            {server.envVars.map((e) => (
              <code key={e} className="text-xs px-2 py-0.5 bg-cyber-warning/10 text-cyber-warning/80 rounded">{e}</code>
            ))}
          </div>
        </div>
      )}
    </div>
  );

  return (
    <div>
      <PageHeader title="Built-in MCP Servers" description="MYTH ships with 11 pre-configured MCP servers for comprehensive reconnaissance. Factory defaults are automatically synced to mcp.json." badge="MCP Ecosystem" />

      <h2 className="text-xl font-bold text-white mb-4">Local Tactical Assets ({local.length})</h2>
      <p className="text-sm text-cyber-text/80 mb-4">Local servers run as child processes via stdio transport. They are launched on-demand and auto-managed by the agent.</p>
      {local.map((s) => <ServerCard key={s.name} server={s} />)}

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Remote Intelligence Assets ({remote.length})</h2>
      <p className="text-sm text-cyber-text/80 mb-4">Remote servers connect via SSE (Server-Sent Events) to cloud-hosted MCP endpoints.</p>
      {remote.map((s) => <ServerCard key={s.name} server={s} />)}

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Custom Security Assets ({custom.length})</h2>
      <p className="text-sm text-cyber-text/80 mb-4">Specialized security-focused MCP servers for code analysis and vulnerability detection.</p>
      {custom.map((s) => <ServerCard key={s.name} server={s} />)}

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Factory Defaults Sync</h2>
      <p className="text-sm text-cyber-text/80 mb-3">MYTH automatically syncs factory defaults to <code className="text-cyber-primary">~/.config/myth/mcp.json</code> on startup. This means:</p>
      <ul className="space-y-2 mb-6">
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary">•</span>New built-in servers are added automatically on update</li>
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary">•</span>Core commands/args/descriptions are kept in sync with source code</li>
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary">•</span>User settings (enabled state, env vars, allowed_tools) are preserved</li>
      </ul>
      <CodeBlock lang="bash" title="Force re-sync" code="myth mcp sync" />
    </div>
  );
}
