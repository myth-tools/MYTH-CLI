import{a as e}from"./rolldown-runtime-COnpUsM8.js";import{i as t,r as n}from"./framer-motion-BkDZnqVO.js";import{P as r,W as i,Y as a,Z as o,a as s,c,t as l}from"./lucide-Cjrn-RS_.js";import{r as u,t as d}from"./index-DRJ1kQkt.js";import{t as f}from"./monaco-B3QCl-SO.js";var p=e(t(),1),m=n(),h=`# ───────────────────────────────────────────────────────────
# MYTH User Configuration  (~/.config/myth/user.yaml)
# Edit and copy into your config file. Changes hot-reload.
# ───────────────────────────────────────────────────────────

agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100      # Max LLM tool-call rounds per turn
  timeout_seconds: 300     # Per-command execution timeout
  user_name: "Chief"       # Defaults to $USER if unset
  log_level: "info"        # trace | debug | info | warn | error
  all_report_path: "mission_report.md"

provider:
  # Add multiple keys — MYTH rotates on rate-limit automatically
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"
  base_url: "https://integrate.api.nvidia.com/v1"
  temperature: 0.5
  max_tokens: 131072

sandbox:
  enabled: true
  share_network: true       # Required for recon tools
  new_session: true         # TIOCSTI terminal injection prevention
  die_with_parent: true     # Auto-cleanup on agent exit
  workspace_size_mb: 512
  hostname: "myth-sandbox"

memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"         # All vectors are volatile — destroyed on exit
  collection_name: "agent_session"
  vector_size: 1024
  auto_start: true

proxy:
  enabled: false
  url: "socks5://127.0.0.1:9050"  # Tor or custom proxy
  use_for_llm: true
  use_for_tools: true
  auto_rotate: false`,g=[{id:`shadow`,name:`Shadow Protocol`,icon:(0,m.jsx)(r,{className:`w-3.5 h-3.5`}),desc:`Max Stealth & Anonymity`,mods:{"proxy.enabled":!0,"agent.log_level":`warn`,"provider.temperature":.3}},{id:`recon`,name:`Deep Spectrum Recon`,icon:(0,m.jsx)(o,{className:`w-3.5 h-3.5`}),desc:`High Power Discovery`,mods:{"agent.max_iterations":250,"sandbox.share_network":!0,"memory.auto_start":!0}},{id:`safe`,name:`Hardened Sandbox`,icon:(0,m.jsx)(c,{className:`w-3.5 h-3.5`}),desc:`Maximum Isolation Mode`,mods:{"sandbox.share_network":!1,"sandbox.workspace_size_mb":128}}];function _(){let[e,t]=(0,p.useState)(h),[n,r]=(0,p.useState)(!1),[o,c]=(0,p.useState)(null),u=()=>{navigator.clipboard.writeText(e),r(!0),setTimeout(()=>r(!1),2e3)},d=e=>{c(e.id);let n=h;for(let[t,r]of Object.entries(e.mods)){let e=RegExp(`${t.split(`.`).pop()}: .*`,`g`);n=n.replace(e,`${t.split(`.`).pop()}: ${r}`)}t(n)};return(0,m.jsxs)(`div`,{className:`flex flex-col lg:flex-row gap-6`,children:[(0,m.jsxs)(`div`,{className:`w-full lg:w-72 shrink-0 space-y-3`,children:[(0,m.jsxs)(`div`,{className:`text-[10px] font-black text-cyber-dim uppercase tracking-[0.3em] mb-4 flex items-center gap-2 px-2`,children:[(0,m.jsx)(a,{className:`w-3 h-3`}),`Tactical Loadouts`]}),g.map(e=>(0,m.jsxs)(`button`,{type:`button`,onClick:()=>d(e),className:`w-full p-4 rounded-2xl border text-left transition-all ${o===e.id?`bg-cyber-primary/10 border-cyber-primary/40 shadow-[0_0_20px_-5px_rgba(0,255,163,0.2)]`:`bg-white/[0.02] border-white/5 hover:bg-white/[0.05] hover:border-white/10`}`,children:[(0,m.jsxs)(`div`,{className:`flex items-center gap-3 mb-2`,children:[(0,m.jsx)(`div`,{className:`p-2 rounded-lg ${o===e.id?`bg-cyber-primary/20 text-cyber-primary`:`bg-black/40 text-cyber-dim`}`,children:e.icon}),(0,m.jsx)(`div`,{className:`font-bold text-sm text-white`,children:e.name})]}),(0,m.jsx)(`p`,{className:`text-[10px] text-cyber-dim leading-relaxed uppercase tracking-tighter`,children:e.desc})]},e.id)),(0,m.jsxs)(`div`,{className:`p-4 rounded-2xl bg-cyber-accent/5 border border-cyber-accent/10 mt-6 hidden lg:block`,children:[(0,m.jsxs)(`div`,{className:`flex items-center gap-2 mb-2 text-cyber-accent`,children:[(0,m.jsx)(l,{className:`w-3.5 h-3.5`}),(0,m.jsx)(`span`,{className:`text-[10px] font-bold uppercase tracking-widest`,children:`Auto-Inject`})]}),(0,m.jsx)(`p`,{className:`text-[10px] text-cyber-dim/80 leading-normal`,children:`All presets are dynamically hashed and verified before being applied to the core engine.`})]})]}),(0,m.jsxs)(`div`,{className:`flex-1 glass-panel rounded-3xl overflow-hidden border border-white/5 shadow-2xl relative`,children:[(0,m.jsxs)(`div`,{className:`bg-cyber-surface2 px-5 py-3 border-b border-white/5 flex items-center justify-between`,children:[(0,m.jsxs)(`div`,{className:`flex items-center gap-3`,children:[(0,m.jsxs)(`div`,{className:`flex gap-1.5 opacity-40`,children:[(0,m.jsx)(`div`,{className:`w-2.5 h-2.5 rounded-full bg-[#ff5f56]`}),(0,m.jsx)(`div`,{className:`w-2.5 h-2.5 rounded-full bg-[#ffbd2e]`}),(0,m.jsx)(`div`,{className:`w-2.5 h-2.5 rounded-full bg-[#27c93f]`})]}),(0,m.jsx)(`div`,{className:`w-px h-3 bg-white/10 mx-2`}),(0,m.jsxs)(`div`,{className:`flex items-center gap-2 text-cyber-dim font-mono text-[10px] uppercase tracking-widest`,children:[(0,m.jsx)(s,{className:`w-3 h-3`}),`user.yaml`]})]}),(0,m.jsxs)(`button`,{type:`button`,onClick:u,className:`flex items-center gap-2 text-[10px] px-4 py-1.5 rounded-full font-black tracking-widest transition-all ${n?`bg-cyber-success/20 text-cyber-success border border-cyber-success/30 shadow-[0_0_15px_rgba(0,255,163,0.2)]`:`bg-cyber-primary/10 text-cyber-primary border border-cyber-primary/20 hover:bg-cyber-primary/20`}`,children:[n?`IDENTIFIED & COPIED`:`EXPORT CONFIG`,(0,m.jsx)(i,{className:`w-3 h-3`})]})]}),(0,m.jsx)(`div`,{className:`h-[520px]`,children:(0,m.jsx)(f,{height:`100%`,defaultLanguage:`yaml`,theme:`cyber-theme`,value:e,onChange:e=>t(e||``),options:{minimap:{enabled:!1},fontSize:13,fontFamily:`'JetBrains Mono', monospace`,lineHeight:22,padding:{top:20,bottom:20},scrollBeyondLastLine:!1,automaticLayout:!0,wordWrap:`on`,folding:!0,lineNumbers:`on`,renderLineHighlight:`line`,scrollbar:{verticalScrollbarSize:4,horizontalScrollbarSize:4}},beforeMount:e=>{e.editor.defineTheme(`cyber-theme`,{base:`vs-dark`,inherit:!0,rules:[{token:`type`,foreground:`00ffa3`},{token:`string`,foreground:`9090b0`},{token:`keyword`,foreground:`00d1ff`},{token:`number`,foreground:`ffcc00`},{token:`comment`,foreground:`505060`,fontStyle:`italic`}],colors:{"editor.background":`#0a0a0f`,"editor.lineHighlightBackground":`#151520`,"editorLineNumber.foreground":`#404060`,"editorLineNumber.activeForeground":`#00ffa3`,"editor.selectionBackground":`#00ffa325`}})}})}),(0,m.jsx)(`div`,{className:`absolute inset-0 pointer-events-none opacity-[0.03] bg-[url('/noise.svg')] mix-blend-overlay`})]})]})}function v(){return(0,m.jsxs)(`div`,{children:[(0,m.jsx)(u,{title:`Configuration`,description:`MYTH uses a two-tier YAML configuration system. agent.yaml (compiled in) provides defaults; user.yaml (user-owned) overrides everything. Changes hot-reload without restarting.`,badge:`Getting Started`}),(0,m.jsxs)(`div`,{className:`mb-12`,children:[(0,m.jsx)(`h2`,{className:`text-xl font-bold text-white mb-6`,children:`Interactive Config Builder`}),(0,m.jsx)(_,{}),(0,m.jsxs)(`p`,{className:`text-xs text-cyber-dim mt-4`,children:[`💡 Edit the YAML above then click`,` `,(0,m.jsx)(`span`,{className:`text-cyber-primary font-bold`,children:`COPY CONFIG`}),` — paste it into`,` `,(0,m.jsx)(`code`,{className:`text-cyber-primary`,children:`~/.config/myth/user.yaml`}),`. Changes apply immediately without restarting the agent.`]})]}),(0,m.jsx)(`h2`,{className:`text-xl font-bold text-white mb-4`,children:`Configuration Files`}),(0,m.jsxs)(`div`,{className:`grid grid-cols-1 sm:grid-cols-3 gap-3 mb-8`,children:[(0,m.jsxs)(`div`,{className:`feature-card rounded-lg p-4`,children:[(0,m.jsx)(`h3`,{className:`font-semibold text-white text-sm`,children:`config/agent.yaml`}),(0,m.jsx)(`p`,{className:`text-xs text-cyber-dim mt-1`,children:`Internal defaults — compiled into the binary at build time. Never edit this directly.`})]}),(0,m.jsxs)(`div`,{className:`feature-card rounded-lg p-4`,children:[(0,m.jsx)(`h3`,{className:`font-semibold text-white text-sm`,children:`~/.config/myth/user.yaml`}),(0,m.jsx)(`p`,{className:`text-xs text-cyber-dim mt-1`,children:`Your overrides: API keys, models, sandbox, memory, TUI, proxy settings.`})]}),(0,m.jsxs)(`div`,{className:`feature-card rounded-lg p-4`,children:[(0,m.jsx)(`h3`,{className:`font-semibold text-white text-sm`,children:`~/.config/myth/mcp.json`}),(0,m.jsx)(`p`,{className:`text-xs text-cyber-dim mt-1`,children:`MCP server registry. Includes factory defaults + your custom additions.`})]})]}),(0,m.jsx)(`h2`,{className:`text-xl font-bold text-white mb-4`,children:`Configuration Sections`}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`Agent`}),(0,m.jsx)(d,{lang:`yaml`,title:`agent section`,code:`agent:
  name: "MYTH"
  version: "0.1.0"
  max_iterations: 100     # Max tool-calling rounds per turn
  timeout_seconds: 300    # Per-command timeout
  user_name: "Chief"      # Defaults to $USER
  log_level: "info"       # trace, debug, info, warn, error
  all_report_path: "mission_report.md"`}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`LLM / NVIDIA NIM`}),(0,m.jsx)(d,{lang:`yaml`,title:`provider section`,code:`provider:
  # Multiple keys supported — MYTH auto-rotates on rate-limit
  api_keys:
    - "nvapi-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    # - "nvapi-yyyyyyyy"  # second key for rotation
  model: "deepseek-ai/deepseek-r1"
  fallback_model: "nvidia/llama-3.1-nemotron-70b-instruct"
  base_url: "https://integrate.api.nvidia.com/v1"
  temperature: 0.5
  max_tokens: 131072`}),(0,m.jsx)(`div`,{className:`glass-panel rounded-xl p-4 mb-6 border border-cyber-secondary/20`,children:(0,m.jsxs)(`p`,{className:`text-xs text-cyber-text/80`,children:[(0,m.jsx)(`span`,{className:`text-cyber-secondary font-bold`,children:`NVIDIA NIM API Key:`}),` Get a free key at`,` `,(0,m.jsx)(`a`,{href:`https://build.nvidia.com/`,target:`_blank`,rel:`noopener noreferrer`,className:`text-cyber-primary hover:underline`,children:`build.nvidia.com`}),`. No GPU required — all inference runs on NVIDIA's cloud.`]})}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`Sandbox`}),(0,m.jsx)(d,{lang:`yaml`,title:`sandbox section`,code:`sandbox:
  enabled: true
  bwrap_path: "/usr/bin/bwrap"
  share_network: true       # Required for recon tools
  new_session: true         # TIOCSTI prevention
  die_with_parent: true
  read_only_paths: ["/usr", "/bin", "/lib", "/etc"]
  writable_tmpfs: ["/tmp", "/var/tmp", "/run"]
  workspace_size_mb: 512
  hostname: "myth-sandbox"`}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`Memory`}),(0,m.jsx)(d,{lang:`yaml`,title:`memory section`,code:`memory:
  enabled: true
  backend: "qdrant"
  mode: "in-memory"          # All data is volatile
  grpc_port: 6334
  http_port: 6333
  collection_name: "agent_session"
  vector_size: 1024          # NIM embedding dimension
  auto_start: true`}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`TUI`}),(0,m.jsx)(d,{lang:`yaml`,title:`tui section`,code:`tui:
  enabled: true
  theme: "dark"
  show_tree_panel: true
  show_status_bar: true
  max_output_lines: 5000
  scroll_speed: 3
  colors:
    primary: "#00ff88"
    secondary: "#0088ff"
    accent: "#ff0055"
    background: "#0a0a0f"
    surface: "#1a1a2e"`}),(0,m.jsx)(`h3`,{className:`text-lg font-semibold text-white mb-3 mt-6`,children:`Proxy / Tor`}),(0,m.jsx)(d,{lang:`yaml`,title:`proxy section`,code:`proxy:
  enabled: false
  url: "socks5://127.0.0.1:9050"  # Tor or custom proxy
  use_for_llm: true
  use_for_tools: true
  auto_rotate: false               # Rotate Tor IP per request
  tor_control_port: 9051
  tor_control_password: ""`}),(0,m.jsx)(`h2`,{className:`text-xl font-bold text-white mb-4 mt-8`,children:`Hot-Reload`}),(0,m.jsxs)(`p`,{className:`text-cyber-text/80 mb-3`,children:[`MYTH watches `,(0,m.jsx)(`code`,{className:`text-cyber-primary`,children:`user.yaml`}),` and`,` `,(0,m.jsx)(`code`,{className:`text-cyber-primary`,children:`mcp.json`}),` via filesystem watchers. Changes apply without restarting the agent.`]}),(0,m.jsx)(d,{lang:`bash`,code:`myth sync   # Force manual re-sync`}),(0,m.jsx)(d,{lang:`bash`,code:`myth config # View all current settings (keys masked)`})]})}export{v as default};