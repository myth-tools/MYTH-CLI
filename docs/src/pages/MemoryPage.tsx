import { PageHeader, CodeBlock } from '../components/Layout';
import MemoryGraph from '../components/MemoryGraph';

export default function MemoryPage() {
  return (
    <div>
      <PageHeader title="Memory System" description="Qdrant-based in-memory vector store with NVIDIA NIM embeddings for semantic session recall." badge="Architecture" />

      <h2 className="text-xl font-bold text-white mb-4">Overview</h2>
      <p className="text-cyber-text/80 mb-6">MYTH uses an in-memory vector database (Qdrant) to automatically store and semantically search tool outputs, findings, and session context. All data is stored in RAM and destroyed on exit — no persistence to disk.</p>

      <div className="mb-12">
        <h3 className="text-sm font-bold text-cyber-primary mb-6 uppercase tracking-widest flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
          Interactive Memory Architecture
        </h3>
        <MemoryGraph />
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Entry Types</h2>
      <table className="w-full text-sm docs-table rounded-lg overflow-hidden mb-8">
        <thead><tr><th>Type</th><th>Description</th></tr></thead>
        <tbody>
          <tr><td className="font-semibold text-white">ToolOutput</td><td className="text-cyber-text/80">Auto-stored from every tool execution — stdout, stderr, exit code</td></tr>
          <tr><td className="font-semibold text-white">Finding</td><td className="text-cyber-text/80">Manually stored by the AI agent when reporting discoveries</td></tr>
          <tr><td className="font-semibold text-white">UserInput</td><td className="text-cyber-text/80">User messages for context recall</td></tr>
          <tr><td className="font-semibold text-white">AgentResponse</td><td className="text-cyber-text/80">Agent analysis and synthesis results</td></tr>
        </tbody>
      </table>

      <h2 className="text-xl font-bold text-white mb-4">Searching Memory</h2>
      <p className="text-cyber-text/80 mb-3">The AI agent can search memory using the <code className="text-cyber-primary">search_memory</code> tool bridge:</p>
      <CodeBlock lang="json" title="Tool call example" code={`{
  "tool": "search_memory",
  "args": {
    "query": "open ports on target",
    "limit": 5
  }
}`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Configuration</h2>
      <CodeBlock lang="yaml" title="memory configuration" code={`memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"
  grpc_port: 6334
  http_port: 6333
  collection_name: "agent_session"
  vector_size: 1024
  auto_start: true`} />

      <div className="mt-6 feature-card rounded-xl p-4 border-cyber-primary/20">
        <h3 className="text-sm font-semibold text-cyber-primary mb-2">🔒 Volatile by Design</h3>
        <p className="text-xs text-cyber-text/80">The in-memory store is intentionally ephemeral. No data touches the disk. When the session ends, all vectors and entries are destroyed. This is a core security property of MYTH.</p>
      </div>
    </div>
  );
}
