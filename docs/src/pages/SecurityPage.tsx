import { PageHeader, CodeBlock } from '../components/Layout';
import SecurityGraph from '../components/SecurityGraph';


export default function SecurityPage() {
  return (
    <div>
      <PageHeader title="Security Model" description="MYTH's multi-layered defense architecture ensures safe reconnaissance operations." badge="Architecture" />

      <div className="mb-12">
        <h3 className="text-sm font-bold text-cyber-error mb-6 uppercase tracking-widest flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-cyber-error animate-pulse" />
          Interactive Defense Layers
        </h3>
        <SecurityGraph />
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Defense Matrix</h2>
      <table className="w-full text-sm docs-table rounded-lg overflow-hidden mb-8">
        <thead><tr><th>Layer</th><th>Protection</th></tr></thead>
        <tbody>
          <tr><td className="font-semibold text-white">1. Security Policy</td><td className="text-cyber-text/80">Unrestricted by default (no hardcoded blocklist), with argument validation and audit logging.</td></tr>
          <tr><td className="font-semibold text-white">2. Bubblewrap Sandbox</td><td className="text-cyber-text/80">Mount/PID/IPC namespace isolation, read-only host filesystem, tmpfs writable dirs</td></tr>
          <tr><td className="font-semibold text-white">3. Network Sharing</td><td className="text-cyber-text/80">Network is shared (required for recon) but host filesystem is invisible</td></tr>
          <tr><td className="font-semibold text-white">4. Volatile Storage</td><td className="text-cyber-text/80">All data on tmpfs — RAM only, zero disk persistence</td></tr>
          <tr><td className="font-semibold text-white">5. Audit Logging</td><td className="text-cyber-text/80">Every command checked and logged with timestamp and verdict</td></tr>
        </tbody>
      </table>

      <h2 className="text-xl font-bold text-white mb-4">Tactical Freedom</h2>
      <p className="text-cyber-text/80 mb-8 leading-relaxed">MYTH is designed as an unrestricted red-team agent. Unlike most AI agents, it does NOT enforce a hardcoded command blocklist. The operator has full sovereign control over which tools the agent can execute, while the isolation layers ensure the host system remains shielded.</p>

      <h2 className="text-xl font-bold text-white mb-4">Bubblewrap Sandbox Details</h2>
      <p className="text-cyber-text/80 mb-3">Every tool invocation is wrapped in a bubblewrap namespace with the following properties:</p>
      <ul className="space-y-2 mb-6">
        {[
          'Read-only host filesystem — no writes to /usr, /bin, /etc',
          'Per-command PID/IPC namespace isolation',
          'tmpfs writable directories (/tmp, /var/tmp, /run)',
          'Mission workspace bound at /workspace',
          '/proc and /dev mounted for tool compatibility',
          'TIOCSTI terminal escape prevention (--new-session)',
          '--die-with-parent ensures child cleanup',
          'Custom hostname (myth-sandbox) for fingerprint control',
          'Random User-Agent injection for anti-bot evasion',
          'Optional proxychains integration for traffic anonymization',
        ].map((item, i) => (
          <li key={i} className="flex gap-2 text-sm text-cyber-text/80">
            <span className="text-cyber-primary shrink-0">•</span>{item}
          </li>
        ))}
      </ul>

      <h2 className="text-xl font-bold text-white mb-4">Sandbox Configuration</h2>
      <CodeBlock lang="yaml" title="sandbox config" code={`sandbox:
  enabled: true              # Set to false at your own risk
  bwrap_path: "/usr/bin/bwrap"
  share_network: true        # Required for recon tools
  new_session: true          # TIOCSTI prevention
  die_with_parent: true      # Auto-cleanup
  read_only_paths:
    - "/usr"
    - "/bin"
    - "/lib"
    - "/etc"
  writable_tmpfs:
    - "/tmp"
    - "/var/tmp"
    - "/run"
  workspace_size_mb: 512
  hostname: "myth-sandbox"`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Output Safety</h2>
      <ul className="space-y-2 mb-6">
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary shrink-0">•</span>Stdout capped at 2MB per command execution</li>
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary shrink-0">•</span>Stderr capped at 512KB per command execution</li>
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary shrink-0">•</span>Configurable per-command timeout (default: 300s)</li>
        <li className="flex gap-2 text-sm text-cyber-text/80"><span className="text-cyber-primary shrink-0">•</span>Process auto-killed on timeout with proper cleanup</li>
      </ul>
    </div>
  );
}
