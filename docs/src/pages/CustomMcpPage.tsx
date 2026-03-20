import { PageHeader, CodeBlock } from '../components/Layout';

export default function CustomMcpPage() {
  return (
    <div>
      <PageHeader title="Custom MCP Servers" description="Extend MYTH's capabilities by integrating your own MCP servers." badge="MCP Ecosystem" />

      <h2 className="text-xl font-bold text-white mb-4">Adding a Local Server</h2>
      <p className="text-sm text-cyber-text/80 mb-3">Local servers run as child processes with stdio transport:</p>
      <CodeBlock lang="bash" title="Interactive mode" code={`/mcp add-local my-tool npx my-mcp-server -a --verbose env:API_KEY=abc123 desc:My_custom_tool`} />
      <CodeBlock lang="bash" title="CLI mode" code={`myth mcp add-local my-tool npx my-mcp-server -a --verbose env:API_KEY=abc123 desc:My_custom_tool`} />

      <h3 className="text-lg font-semibold text-white mb-3 mt-6">Syntax Breakdown</h3>
      <table className="w-full text-sm docs-table rounded-lg overflow-hidden mb-8">
        <thead><tr><th>Part</th><th>Description</th></tr></thead>
        <tbody>
          <tr><td><code className="text-cyber-primary text-xs">my-tool</code></td><td className="text-cyber-text/70 text-xs">Server identifier name</td></tr>
          <tr><td><code className="text-cyber-primary text-xs">npx</code></td><td className="text-cyber-text/70 text-xs">Base command to run</td></tr>
          <tr><td><code className="text-cyber-primary text-xs">my-mcp-server -a --verbose</code></td><td className="text-cyber-text/70 text-xs">Arguments (everything after command until env:/desc:)</td></tr>
          <tr><td><code className="text-cyber-primary text-xs">env:KEY=VALUE</code></td><td className="text-cyber-text/70 text-xs">Environment variables (multiple allowed)</td></tr>
          <tr><td><code className="text-cyber-primary text-xs">desc:My_custom_tool</code></td><td className="text-cyber-text/70 text-xs">Description (underscores = spaces)</td></tr>
        </tbody>
      </table>

      <h2 className="text-xl font-bold text-white mb-4">Adding a Remote Server</h2>
      <p className="text-sm text-cyber-text/80 mb-3">Remote servers connect via SSE (Server-Sent Events):</p>
      <CodeBlock lang="bash" code={`/mcp add-remote my-api https://api.example.com/sse`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Managing Servers</h2>
      <CodeBlock lang="bash" title="List all servers" code="/mcp list" />
      <CodeBlock lang="bash" title="Toggle on/off" code={`/mcp toggle my-tool on
/mcp toggle my-tool off`} />
      <CodeBlock lang="bash" title="View tools for a server" code="/mcp tools my-tool" />
      <CodeBlock lang="bash" title="Remove a server" code="/mcp remove my-tool" />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Tool Allow/Block Lists</h2>
      <p className="text-sm text-cyber-text/80 mb-3">Control which tools the AI agent can use:</p>
      <CodeBlock lang="bash" code={`# Allow a specific tool
/mcp allow-tool github create_issue

# Block a dangerous tool
/mcp block-tool github delete_repo`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">mcp.json Structure</h2>
      <CodeBlock lang="json" title="~/.config/myth/mcp.json" code={`{
  "mcpServers": {
    "my-tool": {
      "command": "npx",
      "args": ["my-mcp-server", "-a", "--verbose"],
      "env": {
        "API_KEY": "abc123"
      },
      "description": "My custom tool",
      "enabled": true,
      "is_user_managed": true,
      "allowed_tools": ["tool_a"],
      "blocked_tools": ["tool_b"]
    },
    "my-api": {
      "url": "https://api.example.com/sse",
      "transport": "sse",
      "description": "My remote API",
      "enabled": true,
      "is_user_managed": true
    }
  }
}`} />
    </div>
  );
}
