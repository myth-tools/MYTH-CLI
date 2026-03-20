import { PageHeader, CodeBlock } from '../components/Layout';

const cliFlags = [
  { flag: '--active', desc: 'Enable active brute-force and permutation scanning' },
  { flag: '--recursive', desc: 'Enable recursive discovery on found subdomains' },
  { flag: '--only-alive', desc: 'Filter results to only show live subdomains (Default: true)' },
  { flag: '--master / --ultra', desc: 'ULTRA-ROBUST MODE: Tor + Proxies + Mega Wordlist + Deep Recursion' },
];

const toolParams = [
  { name: 'domain', type: 'string', required: true, def: '-', desc: 'Target domain (e.g., example.com)' },
  { name: 'active', type: 'boolean', required: false, def: 'false', desc: 'Enable active brute-force and permutation scanning' },
  { name: 'recursive', type: 'boolean', required: false, def: 'false', desc: 'Enable recursive discovery (scans sub-subdomains)' },
  { name: 'only_alive', type: 'boolean', required: false, def: 'true', desc: 'Only return subdomains that resolve to IPs' },
  { name: 'stealth', type: 'boolean', required: false, def: 'false', desc: 'Stealth mode: reduces concurrency and adds randomized delays' },
  { name: 'quiet', type: 'boolean', required: false, def: 'false', desc: 'Suppress all progress/stats output' },
  { name: 'json', type: 'boolean', required: false, def: 'false', desc: 'Output results in JSONL format' },
  { name: 'concurrency', type: 'integer', required: false, def: '50', desc: 'Maximum concurrent discovery tasks' },
  { name: 'timeout', type: 'integer', required: false, def: '3600', desc: 'Global timeout in seconds' },
  { name: 'retries', type: 'integer', required: false, def: '5', desc: 'Number of retries on failure' },
  { name: 'min_delay_ms', type: 'integer', required: false, def: '50', desc: 'Minimum delay between requests (ms)' },
  { name: 'max_delay_ms', type: 'integer', required: false, def: '2000', desc: 'Maximum delay between requests (ms)' },
  { name: 'use_proxies', type: 'boolean', required: false, def: 'false', desc: 'Use rotating proxies for all discovery phases' },
  { name: 'proxies_file', type: 'string', required: false, def: '-', desc: 'Use a specific list of proxies from file' },
  { name: 'use_tor', type: 'boolean', required: false, def: 'false', desc: 'Route all discovery traffic through the Tor network' },
  { name: 'tor_address', type: 'string', required: false, def: '127.0.0.1:9050', desc: 'Custom Tor SOCKS5 address' },
  { name: 'disable_ua_rotation', type: 'boolean', required: false, def: 'false', desc: 'Disable User-Agent rotation (120+ agents)' },
  { name: 'respect_robots', type: 'boolean', required: false, def: 'true', desc: 'Follow robots.txt exclusion rules' },
  { name: 'disable_captcha_avoidance', type: 'boolean', required: false, def: 'false', desc: 'Disable built-in CAPTCHA detection/bypass' },
  { name: 'custom_resolvers', type: 'array', required: false, def: '-', desc: 'Custom DNS servers' },
  { name: 'resolvers_file', type: 'string', required: false, def: '-', desc: 'Load DNS servers from a specific file' },
  { name: 'disable_resolver_rotation', type: 'boolean', required: false, def: 'false', desc: 'Disable rotation of DNS servers' },
  { name: 'disable_wildcard_filter', type: 'boolean', required: false, def: 'false', desc: 'Disable intelligent wildcard DNS detection' },
  { name: 'depth', type: 'integer', required: false, def: '1', desc: 'Initial depth of subdomain gathering' },
  { name: 'recursive_depth', type: 'integer', required: false, def: '3', desc: 'Maximum depth for recursive discovery (up to 5)' },
  { name: 'max_pages', type: 'integer', required: false, def: '50000', desc: 'Max pages to crawl during web scraping' },
  { name: 'max_crawl_depth', type: 'integer', required: false, def: '3', desc: 'Maximum depth for web crawler' },
  { name: 'disable_checkpoints', type: 'boolean', required: false, def: 'false', desc: 'Disable session saving and resumption' },
  { name: 'checkpoint_dir', type: 'string', required: false, def: '.subdomain_fetch_checkpoints', desc: 'Directory to store session checkpoints' },
  { name: 'wordlist_type', type: 'enum', required: false, def: 'medium', desc: 'Built-in wordlist: none | small | medium | large | quick | deep | mega' },
  { name: 'disable_proxy_test', type: 'boolean', required: false, def: 'false', desc: 'Skip testing proxies for connectivity before use' },
  { name: 'custom_wordlists', type: 'array', required: false, def: '-', desc: 'Custom wordlist files for brute-forcing' },
];

const phases = [
  '100+ PASSIVE SOURCES (premium zero-cost aggregation)',
  'ULTRA-FAST STREAMING DNS BRUTE-FORCE (2GB+ cloud-streamed wordlists)',
  'INTELLIGENT WILDCARD FILTERING (zero-noise logic)',
  'ALTDNS MUTATION ENGINE (dash-dot swaps, number increments, chaining)',
  'RECURSIVE DISCOVERY (multi-depth chaining logic)',
  'JS SOURCE MAP ANALYSIS (extracting hidden developer paths)',
  'WEB CRAWLING & SCRAPING (recursive HTML/JS analysis)',
  'HARDENED VHOST DISCOVERY (HTTP/HTTPS + SNI-aware probing)',
  'ENT (EMPTY NON-TERMINAL) DISCOVERY',
  'DNSSEC NSEC WALKING (dumping zone chains)',
  'DNSSEC NSEC3 CHAIN MAPPING',
  'AXFR (ZONE TRANSFER) DISCOVERY',
  'PTR (REVERSE DNS) SCANNING',
  'ORGANIZATION CIDR EXPANSION (RDAP-based block sweeps)',
  'CLOUD PROVIDER RECON (AWS/Azure/GCP/Firebase probing)',
  'HIDDEN ASSET SCRAPING (robots, sitemaps, security.txt)',
  'QUANTUM TLS SAN EXTRACTION (Multi-Port certificate handshakes)',
  'QUANTUM BRUTE-FORCE CLOSURE (final 10,000 req/sec keyword pass)',
];

export default function SubdomainFetchPage() {
  return (
    <div>
      <PageHeader 
        title="Subdomain Fetcher (Quantum Engine)" 
        description="The MOST ADVANCED free subdomain enumeration tool ever created in Rust. Achieves ~100% absolute coverage through a systematic 18-phase discovery pipeline. NO API KEYS REQUIRED." 
        badge="Built-in Tool" 
      />

      <div className="feature-card rounded-xl p-4 mb-8">
        <h3 className="text-sm font-semibold text-cyber-primary mb-2">⚡ Quantum-Grade Capabilities</h3>
        <ul className="space-y-1">
          <li className="text-xs text-cyber-text/80">• <strong>100+ passive sources</strong> — zero API keys required</li>
          <li className="text-xs text-cyber-text/80">• <strong>5000+ fresh DNS resolvers</strong> fetched dynamically at runtime</li>
          <li className="text-xs text-cyber-text/80">• <strong>50+ working proxies</strong> auto-sourced from multiple providers</li>
          <li className="text-xs text-cyber-text/80">• <strong>120+ user agents</strong> rotated for stealth</li>
          <li className="text-xs text-cyber-text/80">• <strong>Tor support</strong> for full traffic anonymization</li>
          <li className="text-xs text-cyber-text/80">• <strong>Session checkpointing</strong> for resumable scans</li>
        </ul>
      </div>

      <h2 className="text-xl font-bold text-white mb-4">The 18-Phase Pipeline</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-2 mb-8">
        {phases.map((phase, i) => (
          <div key={i} className="bg-cyber-dark/30 border border-cyber-dim/20 rounded p-2 flex items-start gap-2">
            <span className="text-cyber-primary font-mono text-xs">{String(i + 1).padStart(2, '0')}.</span>
            <span className="text-cyber-text/80 text-xs">{phase}</span>
          </div>
        ))}
      </div>

      <h2 className="text-xl font-bold text-white mb-4">CLI Usage</h2>
      <CodeBlock lang="bash" title="Standard Passive Discovery" code={`myth subdomains example.com`} />
      <CodeBlock lang="bash" title="Active Brute-force + Permutations" code={`myth subdomains example.com --active`} />
      <CodeBlock lang="bash" title="Recursive Sub-Subdomain Scan" code={`myth subdomains example.com --active --recursive`} />
      <CodeBlock lang="bash" title="Ultra-Robust / Master Mode" code={`# Auto-configures Tor, Proxies, Mega-wordlists, deep recursion
myth subdomains example.com --master
# Or use the standalone alias
myth master example.com`} />
      <CodeBlock lang="bash" title="Stealth Mode via Tor" code={`myth subdomains example.com --master`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-4">Interactive TUI Usage</h2>
      <CodeBlock lang="bash" title="Inside TUI session" code={`/subdomains example.com
/subdomains example.com --master
/master example.com`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-4">CLI Flags</h2>
      <table className="w-full text-sm docs-table rounded-lg overflow-hidden mb-8">
        <thead><tr><th>Flag</th><th>Description</th></tr></thead>
        <tbody>
          {cliFlags.map((f) => (
            <tr key={f.flag}>
              <td><code className="text-cyber-primary text-xs font-mono whitespace-nowrap">{f.flag}</code></td>
              <td className="text-cyber-text/70 text-xs">{f.desc}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <h2 className="text-xl font-bold text-white mb-4">Complete Tool Parameter Reference (JSON Schema)</h2>
      <p className="text-sm text-cyber-text/80 mb-4">When invoked as a native tool by the AI agent via <code className="text-cyber-primary">subdomain_fetch</code>, these are all available parameters:</p>
      <table className="w-full text-sm docs-table rounded-lg overflow-hidden">
        <thead><tr><th>Parameter</th><th>Type</th><th>Required</th><th>Default</th><th>Description</th></tr></thead>
        <tbody>
          {toolParams.map((p) => (
            <tr key={p.name}>
              <td><code className="text-cyber-primary text-xs font-mono whitespace-nowrap">{p.name}</code></td>
              <td className="text-cyber-dim text-xs font-mono">{p.type}</td>
              <td>{p.required ? <span className="text-cyber-error text-xs">Yes</span> : <span className="text-cyber-dim text-xs">No</span>}</td>
              <td className="text-cyber-dim text-xs font-mono">{p.def}</td>
              <td className="text-cyber-text/70 text-xs">{p.desc}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
