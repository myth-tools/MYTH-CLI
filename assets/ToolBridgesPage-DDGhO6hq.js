import{r as e}from"./framer-motion-BkDZnqVO.js";import{h as t,r as n,t as r}from"./index-DRJ1kQkt.js";var i=e(),a={"Core Execution":`bg-cyber-primary/10 text-cyber-primary border-cyber-primary/30`,Discovery:`bg-cyber-secondary/10 text-cyber-secondary border-cyber-secondary/30`,"Mission Control":`bg-cyber-warning/10 text-cyber-warning border-cyber-warning/30`,Memory:`bg-purple-500/10 text-purple-300 border-purple-500/30`,"MCP Protocol":`bg-blue-500/10 text-blue-300 border-blue-500/30`,"Native Utilities":`bg-cyber-success/10 text-cyber-success border-cyber-success/30`},o={execute_tool:{desc:`Execute nmap in the Bubblewrap sandbox`,json:`{
  "tool": "execute_tool",
  "args": {
    "tool_name": "nmap",
    "args": ["-sV", "-p", "80,443,8080", "example.com"],
    "timeout": 60
  }
}`},execute_batch:{desc:`Run nmap + gobuster in parallel (Swarm Mode)`,json:`{
  "tool": "execute_batch",
  "args": {
    "commands": [
      { "tool": "nmap", "args": ["-sV", "example.com"] },
      { "tool": "gobuster", "args": ["dir", "-u", "https://example.com", "-w", "/usr/share/wordlists/common.txt"] }
    ]
  }
}`},report_finding:{desc:`Register a critical vulnerability into ReconGraph`,json:`{
  "tool": "report_finding",
  "args": {
    "title": "SQL Injection in /login",
    "severity": "critical",
    "description": "login endpoint is vulnerable to UNION-based SQLi",
    "target": "https://example.com/login",
    "evidence": "' OR 1=1-- returned 200 with admin data"
  }
}`},search_memory:{desc:`Recall past tool outputs with semantic search`,json:`{
  "tool": "search_memory",
  "args": {
    "query": "open ports found on target",
    "limit": 5
  }
}`}},s=[...new Set(t.map(e=>e.category))];function c(){return(0,i.jsxs)(`div`,{children:[(0,i.jsx)(n,{title:`LLM Tool Bridges`,description:`16 Rig.rs Tool implementations that connect the NVIDIA NIM AI agent to MYTH's execution layer. The LLM autonomously decides which bridges to call based on mission requirements.`,badge:`MCP Ecosystem`}),(0,i.jsxs)(`div`,{className:`glass-panel rounded-xl p-6 mb-10 border border-cyber-primary/20`,children:[(0,i.jsxs)(`h2`,{className:`text-base font-bold text-cyber-primary mb-4 uppercase tracking-wider flex items-center gap-2`,children:[(0,i.jsx)(`span`,{className:`w-2 h-2 rounded-full bg-cyber-primary animate-pulse`}),`How the Rig.rs Tool Trait Works`]}),(0,i.jsx)(`div`,{className:`grid grid-cols-1 sm:grid-cols-3 gap-4 mb-4`,children:[{step:`1. definition()`,desc:`Returns a JSON Schema to the LLM describing input parameters. The LLM uses this schema to know how to call the tool correctly.`},{step:`2. call(args)`,desc:`Executes the bridge logic — dispatching to sandbox, MCP server, or native Rust engine — and returns structured results.`},{step:`3. telemetry`,desc:`Dual-mode output: TUI events stream to the panel in real-time, CLI mode prints directly to stdout. All outputs auto-stored in semantic memory.`}].map(e=>(0,i.jsxs)(`div`,{className:`bg-white/5 rounded-lg p-4 border border-white/10`,children:[(0,i.jsx)(`p`,{className:`text-xs font-bold text-cyber-primary font-mono mb-2`,children:e.step}),(0,i.jsx)(`p`,{className:`text-xs text-cyber-dim leading-relaxed`,children:e.desc})]},e.step))}),(0,i.jsx)(r,{lang:`rust`,title:`Simplified Rig.rs Tool trait (conceptual)`,code:`// Each tool bridge implements this trait
trait Tool {
    fn definition(&self) -> serde_json::Value; // JSON Schema for LLM
    async fn call(&self, args: Value) -> Result<String>; // execution logic
}

// The ReconAgent is built with all 16 bridges attached
let agent = openai_client
    .agent(model)
    .tool(ExecuteToolBridge::new(...))
    .tool(ExecuteBatchBridge::new(...))
    .tool(SearchMemoryBridge::new(...))
    // ... 13 more
    .build();`})]}),(0,i.jsxs)(`h2`,{className:`text-xl font-bold text-white mb-4`,children:[(0,i.jsx)(`span`,{className:`text-cyber-primary`,children:`01.`}),` LLM Tool Call Examples`]}),(0,i.jsx)(`div`,{className:`grid grid-cols-1 md:grid-cols-2 gap-4 mb-10`,children:Object.entries(o).map(([e,t])=>(0,i.jsxs)(`div`,{className:`feature-card rounded-xl p-4`,children:[(0,i.jsx)(`p`,{className:`text-[10px] font-mono text-cyber-dim uppercase tracking-wider mb-1`,children:t.desc}),(0,i.jsx)(r,{lang:`json`,title:`myth → LLM → ${e}`,code:t.json})]},e))}),(0,i.jsxs)(`h2`,{className:`text-xl font-bold text-white mb-4`,children:[(0,i.jsx)(`span`,{className:`text-cyber-primary`,children:`02.`}),` Bridge Registry`]}),s.map(e=>(0,i.jsxs)(`div`,{className:`mb-8`,children:[(0,i.jsxs)(`div`,{className:`flex items-center gap-3 mb-4`,children:[(0,i.jsx)(`h3`,{className:`text-base font-bold text-white uppercase tracking-wider`,children:e}),(0,i.jsxs)(`span`,{className:`text-[10px] px-2 py-0.5 rounded border font-mono font-bold ${a[e]??`bg-white/5 text-cyber-dim border-white/10`}`,children:[t.filter(t=>t.category===e).length,` bridges`]}),(0,i.jsx)(`div`,{className:`h-px flex-1 bg-gradient-to-r from-cyber-border/50 to-transparent`})]}),(0,i.jsx)(`div`,{className:`space-y-3`,children:t.filter(t=>t.category===e).map(e=>(0,i.jsxs)(`div`,{className:`feature-card rounded-xl p-5 flex flex-col sm:flex-row sm:items-start gap-4`,children:[(0,i.jsxs)(`div`,{className:`shrink-0 min-w-[200px]`,children:[(0,i.jsx)(`code`,{className:`text-sm font-bold text-white font-mono block mb-1`,children:e.rigName}),(0,i.jsxs)(`span`,{className:`text-[10px] text-cyber-dim font-mono opacity-60`,children:[`struct: `,e.name]})]}),(0,i.jsx)(`p`,{className:`text-sm text-cyber-text/70 leading-relaxed`,children:e.description}),(0,i.jsx)(`span`,{className:`shrink-0 text-[9px] px-2 py-0.5 rounded border font-mono font-bold self-start ${a[e.category]??`bg-white/5 text-cyber-dim border-white/10`}`,children:e.category})]},e.name))})]},e))]})}export{c as default};