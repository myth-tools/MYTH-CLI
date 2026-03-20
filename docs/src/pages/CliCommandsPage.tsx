import { PageHeader, CodeBlock, HighlightText } from '../components/Layout';
import { cliCommands } from '../data/content';
import { useState } from 'react';


export default function CliCommandsPage() {
  const [filter, setFilter] = useState('');
  
  const filtered = cliCommands.filter((c) => {
    const term = filter.toLowerCase().trim();
    if (!term) return true;

    // Deep search across all fields for maximum robustness
    const matchName = c.name.toLowerCase().includes(term);
    const matchDesc = c.description.toLowerCase().includes(term);
    const matchAlias = c.aliases?.some(a => a.toLowerCase().includes(term)) ?? false;
    const matchUsage = c.usage.toLowerCase().includes(term);
    const matchArgs = c.args?.some(a => 
      a.name.toLowerCase().includes(term) || 
      a.description.toLowerCase().includes(term)
    ) ?? false;

    return matchName || matchDesc || matchAlias || matchUsage || matchArgs;
  });

  return (
    <div>
      <PageHeader title="CLI Commands" description="Complete reference for all myth CLI subcommands. These are invoked directly from your shell." badge="Commands" />

      <div className="mb-8 relative max-w-2xl">
        <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg className="h-5 w-5 text-cyber-dim" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </div>
        <input
          type="text"
          placeholder="Search commands, aliases, arguments, or descriptions..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          className="w-full pl-10 pr-24 py-3 text-sm bg-cyber-bg border border-cyber-border rounded-xl text-white placeholder-cyber-dim focus:outline-none focus:border-cyber-primary focus:ring-1 focus:ring-cyber-primary/50 transition-all shadow-sm"
        />
        {filter && (
          <button 
            onClick={() => setFilter('')}
            className="absolute inset-y-0 right-16 px-2 flex items-center text-cyber-dim hover:text-white transition-colors"
          >
            <svg className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
              <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
            </svg>
          </button>
        )}
        <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
          <span className="text-xs font-mono text-cyber-primary px-2 py-1 bg-cyber-primary/10 rounded">
            {filtered.length} {filtered.length === 1 ? 'result' : 'results'}
          </span>
        </div>
      </div>

      {filtered.length === 0 && (
        <div className="text-center py-12 border border-dashed border-cyber-border rounded-xl bg-cyber-dark/30">
          <svg className="mx-auto h-12 w-12 text-cyber-dim mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1} d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <h3 className="text-lg font-medium text-white mb-1">No matching commands found</h3>
          <p className="text-sm text-cyber-dim">Try adjusting your search query, or search for specific flags like "--json".</p>
          <button onClick={() => setFilter('')} className="mt-4 px-4 py-2 bg-cyber-primary/10 text-cyber-primary border border-cyber-primary/30 rounded-lg hover:bg-cyber-primary/20 transition-colors text-sm font-medium">
            Clear Search
          </button>
        </div>
      )}

      <div className="space-y-8">
        {filtered.map((cmd) => (
          <div key={cmd.name} id={cmd.name} className="scroll-mt-20">
            <div className="flex items-center gap-3 mb-2">
              <h2 className="text-lg font-bold text-white font-mono">
                myth <HighlightText text={cmd.name} highlight={filter} />
              </h2>
              {cmd.aliases?.map((a) => (
                <span key={a} className="text-xs px-2 py-0.5 bg-cyber-secondary/15 text-cyber-secondary border border-cyber-secondary/30 rounded font-mono">
                  alias: <HighlightText text={a} highlight={filter} />
                </span>
              ))}
            </div>
            <p className="text-sm text-cyber-text/80 mb-3">
              <HighlightText text={cmd.description} highlight={filter} />
            </p>

            <CodeBlock lang="bash" title="Usage" code={cmd.usage} />

            {cmd.args && cmd.args.length > 0 && (
              <div className="mb-3">
                <h4 className="text-sm font-semibold text-cyber-dim mb-2">Arguments</h4>
                <table className="w-full text-sm docs-table rounded-lg overflow-hidden">
                  <thead><tr><th>Name</th><th>Type</th><th>Required</th><th>Description</th></tr></thead>
                  <tbody>
                    {cmd.args.map((a) => (
                      <tr key={a.name}>
                        <td><code className="text-cyber-primary text-xs"><HighlightText text={a.name} highlight={filter} /></code></td>
                        <td className="text-cyber-dim text-xs">{a.type}</td>
                        <td>{a.required ? <span className="text-cyber-error text-xs">Yes</span> : <span className="text-cyber-dim text-xs">No</span>}</td>
                        <td className="text-cyber-text/70 text-xs">
                          <HighlightText text={a.description} highlight={filter} />
                          {a.default ? ` (default: ${a.default})` : ''}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}

            <h4 className="text-sm font-semibold text-cyber-dim mb-2">Examples</h4>
            {cmd.examples.map((ex, i) => (
              <div key={i} className="mb-2">
                <CodeBlock lang="bash" code={ex.command} />
                <p className="text-xs text-cyber-dim -mt-2 ml-1">{ex.description}</p>
              </div>
            ))}

            <div className="h-px bg-cyber-border mt-6" />
          </div>
        ))}
      </div>
    </div>
  );
}
