import { PageHeader } from '../components/Layout';
import { builtinTools } from '../data/content';

export default function BuiltinToolsPage() {
  return (
    <div>
      <PageHeader title="Built-in Tools" description="19 native Rust utility tools that execute without the sandbox. These provide essential mission capabilities across Utility, Security, Web, Memory, Mission, and Recon categories." badge="MCP Ecosystem" />

      <div className="space-y-6">
        {builtinTools.map((tool) => (
          <div key={tool.name} className="feature-card rounded-xl p-5">
            <div className="flex items-center gap-3 mb-2">
              <h3 className="font-bold text-white font-mono">{tool.name}</h3>
              <span className="text-[10px] px-2 py-0.5 bg-cyber-secondary/15 text-cyber-secondary border border-cyber-secondary/30 rounded font-mono">{tool.category}</span>
            </div>
            <p className="text-sm text-cyber-text/80 mb-4">{tool.description}</p>
            <h4 className="text-xs font-semibold text-cyber-dim mb-2">Parameters</h4>
            <table className="w-full text-sm docs-table rounded-lg overflow-hidden">
              <thead><tr><th>Name</th><th>Type</th><th>Required</th><th>Description</th></tr></thead>
              <tbody>
                {tool.parameters.map((p) => (
                  <tr key={p.name}>
                    <td><code className="text-cyber-primary text-xs">{p.name}</code></td>
                    <td className="text-cyber-dim text-xs font-mono">{p.type}</td>
                    <td>{p.required ? <span className="text-cyber-error text-xs">Yes</span> : <span className="text-cyber-dim text-xs">No</span>}</td>
                    <td className="text-cyber-text/70 text-xs">{p.description}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ))}
      </div>
    </div>
  );
}
