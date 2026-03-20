import { PageHeader } from '../components/Layout';
import { toolBridges } from '../data/content';

export default function ToolBridgesPage() {
  const categories = [...new Set(toolBridges.map((b) => b.category))];

  return (
    <div>
      <PageHeader title="LLM Tool Bridges" description="16 Rig.rs tool bridges that connect the AI agent to execution capabilities. The LLM calls these tools to perform actions." badge="MCP Ecosystem" />

      <div className="feature-card rounded-xl p-4 mb-8">
        <h3 className="text-sm font-semibold text-cyber-primary mb-2">How Tool Bridges Work</h3>
        <p className="text-xs text-cyber-text/80 mb-2">Tool bridges implement Rig's <code className="text-cyber-primary">Tool</code> trait. Each bridge has:</p>
        <ul className="space-y-1 text-xs text-cyber-text/80">
          <li>• A <code className="text-cyber-primary">definition()</code> method that returns a JSON Schema for the LLM</li>
          <li>• A <code className="text-cyber-primary">call()</code> method that executes the action and returns results</li>
          <li>• Dual-mode telemetry (TUI events + CLI output) for real-time feedback</li>
          <li>• Automatic semantic memory storage for later recall</li>
        </ul>
      </div>

      {categories.map((cat) => (
        <div key={cat} className="mb-8">
          <h2 className="text-lg font-bold text-white mb-4">{cat}</h2>
          <div className="space-y-3">
            {toolBridges.filter((b) => b.category === cat).map((bridge) => (
              <div key={bridge.name} className="feature-card rounded-lg p-4">
                <div className="flex items-center gap-3 mb-1">
                  <code className="text-sm font-bold text-white font-mono">{bridge.rigName}</code>
                  <span className="text-[10px] text-cyber-dim font-mono">({bridge.name})</span>
                </div>
                <p className="text-sm text-cyber-text/70">{bridge.description}</p>
              </div>
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}
