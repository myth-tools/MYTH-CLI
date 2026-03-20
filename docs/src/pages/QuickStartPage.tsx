import { PageHeader, CodeBlock } from '../components/Layout';

export default function QuickStartPage() {
  return (
    <div>
      <PageHeader
        title="Quick Start"
        description="Start using MYTH in minutes. Configure your API key and launch your first reconnaissance mission."
        badge="Getting Started"
      />

      <h2 className="text-xl font-bold text-white mb-4">1. Set Your API Key</h2>
      <p className="text-cyber-text/80 mb-3">MYTH requires an NVIDIA NIM API key. Get one free at <a href="https://build.nvidia.com/" target="_blank" className="text-cyber-primary hover:underline">build.nvidia.com</a>.</p>
      <CodeBlock lang="bash" title="Environment variable" code='export NVIDIA_API_KEY="nvapi-xxxxxxxxxxxxx"' />
      <p className="text-cyber-text/80 mb-3">Or add it to your config file:</p>
      <CodeBlock lang="yaml" title="~/.config/myth/user.yaml" code={`llm:
  provider: "nvidia-nim"
  base_url: "https://integrate.api.nvidia.com/v1"
  nvidia_nim_api_key: ["nvapi-xxxxxxxxxxxxx"]
  model: "deepseek-ai/deepseek-v3"`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">2. Verify System Health</h2>
      <CodeBlock lang="bash" code="myth check" />
      <p className="text-cyber-text/80 mb-3">This runs a comprehensive diagnostic check of sandbox status, network, MCP servers, and tool availability.</p>

      <h2 className="text-xl font-bold text-white mb-4 mt-8">3. Launch Your First Scan</h2>
      <CodeBlock lang="bash" title="Full reconnaissance" code="myth scan example.com" />
      <p className="text-cyber-text/80 mb-3">This will initialize the AI agent, synchronize tools, and begin the 13-phase reconnaissance methodology against the target.</p>

      <h2 className="text-xl font-bold text-white mb-4 mt-8">4. Interactive Chat Mode</h2>
      <CodeBlock lang="bash" title="TUI mode (default)" code="myth chat" />
      <CodeBlock lang="bash" title="Simple CLI mode" code="myth --no-tui chat" />
      <p className="text-cyber-text/80 mb-3">In interactive mode you can chat directly with the AI agent. Use commands like <code className="text-cyber-primary">/scan</code>, <code className="text-cyber-primary">/tools</code>, and <code className="text-cyber-primary">/help</code> inside the session.</p>

      <h2 className="text-xl font-bold text-white mb-4 mt-8">5. List Available Tools</h2>
      <CodeBlock lang="bash" code="myth tools" />
      <p className="text-cyber-text/80 mb-3">Shows all synchronized local binaries, external MCP tools, and native utilities.</p>

      <h2 className="text-xl font-bold text-white mb-4 mt-8">6. View Configuration</h2>
      <CodeBlock lang="bash" code="myth config" />
      <p className="text-cyber-text/80">API keys are automatically masked. Shows all settings including LLM, sandbox, memory, TUI, and profiles.</p>

      <div className="mt-10 feature-card rounded-xl p-5 border-cyber-warning/30">
        <h3 className="font-semibold text-cyber-warning mb-2">⚠️ Important</h3>
        <p className="text-sm text-cyber-text/80">Only scan targets you have explicit written authorization to test. Unauthorized scanning is illegal.</p>
      </div>
    </div>
  );
}
