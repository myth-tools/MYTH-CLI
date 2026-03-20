import { useState } from 'react';
import Editor from '@monaco-editor/react';

const initialConfig = `agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100
  timeout_seconds: 300
  user_name: "Chief"
  log_level: "info"
  all_report_path: "mission_report.md"

llm:
  provider: "nvidia-nim"
  base_url: "https://integrate.api.nvidia.com/v1"
  model: "deepseek-ai/deepseek-v3.1"
  temperature: 0.1
  max_tokens: 8192

sandbox:
  enabled: true
  share_network: true
  workspace_size_mb: 512
  hostname: "myth-sandbox"`;

export default function ConfigBuilder() {
  const [yaml, setYaml] = useState(initialConfig);

  return (
    <div className="glass-panel rounded-2xl overflow-hidden border border-cyber-border/50">
      <div className="bg-cyber-surface2 px-4 py-2 border-b border-cyber-border flex items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="flex gap-1.5">
            <div className="w-2.5 h-2.5 rounded-full bg-[#ff5f56]" />
            <div className="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]" />
            <div className="w-2.5 h-2.5 rounded-full bg-[#27c93f]" />
          </div>
          <span className="text-[10px] text-cyber-dim font-mono ml-4 uppercase tracking-widest">Interactive_Config_Engine</span>
        </div>
        <button 
          onClick={() => navigator.clipboard.writeText(yaml)}
          className="text-[10px] px-2 py-1 bg-cyber-primary/10 text-cyber-primary border border-cyber-primary/20 rounded hover:bg-cyber-primary/20 transition-all font-bold"
        >
          COPY OVERRIDES
        </button>
      </div>
      <div className="h-[400px]">
        <Editor
          height="100%"
          defaultLanguage="yaml"
          theme="vs-dark"
          value={yaml}
          onChange={(v) => setYaml(v || '')}
          options={{
            minimap: { enabled: false },
            fontSize: 12,
            fontFamily: "'JetBrains Mono', monospace",
            lineHeight: 20,
            padding: { top: 16, bottom: 16 },
            scrollBeyondLastLine: false,
            automaticLayout: true,
            backgroundColor: '#030305'
          }}
          beforeMount={(monaco) => {
            monaco.editor.defineTheme('cyber-theme', {
              base: 'vs-dark',
              inherit: true,
              rules: [
                { token: 'type', foreground: '00ff88' },
                { token: 'string', foreground: 'e0e0e0' },
                { token: 'keyword', foreground: '0088ff' },
                { token: 'number', foreground: 'ff0055' },
                { token: 'comment', foreground: '666680', fontStyle: 'italic' },
              ],
              colors: {
                'editor.background': '#0a0a0f',
                'editor.lineHighlightBackground': '#111118',
              }
            });
          }}
        />
      </div>
    </div>
  );
}
