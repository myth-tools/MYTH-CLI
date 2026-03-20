import { PageHeader, CodeBlock } from '../components/Layout';

export default function InstallationPage() {
  const repoUrl = import.meta.env.VITE_REPO_URL || 'https://github.com/myth-tools/MYTH-CLI';
  const pagesUrl = import.meta.env.VITE_PAGES_URL || 'https://myth.work.gd';

  return (
    <div>
      <PageHeader
        title="Installation"
        description="Get MYTH installed on your Kali Linux system. Multiple installation methods are available."
        badge="Getting Started"
      />

      <h2 className="text-xl font-bold text-white mb-4">Prerequisites</h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-8">
        {[
          { name: 'Kali Linux', desc: 'Or any Debian-based distro' },
          { name: 'Kali Linux Recon Tools', desc: 'Pre-installed on Kali' },
          { name: 'Bubblewrap', desc: 'For sandbox isolation' },
          { name: 'NVIDIA NIM API Key', desc: 'Free from build.nvidia.com' },
        ].map((p) => (
          <div key={p.name} className="feature-card rounded-lg p-4">
            <h3 className="font-semibold text-white text-sm">{p.name}</h3>
            <p className="text-xs text-cyber-dim mt-1">{p.desc}</p>
          </div>
        ))}
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Tactical Deployment</h2>
      <p className="text-cyber-text/80 mb-4">Run the following one-liner to deploy MYTH on your Kali Linux system. This automatically configures your APT repository, signs the authority keys, and fulfills all tactical dependencies.</p>
      <CodeBlock lang="bash" title="install.sh" code={`curl -sSL ${pagesUrl}/install.sh | sudo bash`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">🚨 Mandatory Setup</h2>
      <p className="text-cyber-text/80 mb-3">Complete these steps immediately after installation to activate your operative:</p>
      
      <div className="space-y-6">
        <div>
          <h3 className="text-sm font-semibold text-white mb-2">1. Initialize Neural Link</h3>
          <p className="text-xs text-cyber-dim mb-2">Export your NVIDIA NIM API key to enable autonomous reasoning:</p>
          <CodeBlock lang="bash" code={`export NVIDIA_API_KEY="nvapi-xxxxxxxxxxxxx"`} />
        </div>

        <div>
          <h3 className="text-sm font-semibold text-white mb-2">2. Synchronize Assets</h3>
          <p className="text-xs text-cyber-dim mb-2">Sync the 3000+ Kali Recon tools and mission metadata:</p>
          <CodeBlock lang="bash" code={`myth sync`} />
        </div>

        <div>
          <h3 className="text-sm font-semibold text-white mb-2">3. Operational Health Check</h3>
          <p className="text-xs text-cyber-dim mb-2">Verify the sandbox, tools, and network connectivity:</p>
          <CodeBlock lang="bash" code={`myth check`} />
        </div>
      </div>

      <h2 className="text-xl font-bold text-white mb-4 mt-8">🛠️ For Developers (Build from Source)</h2>
      <p className="text-cyber-text/80 mb-3">If you intend to modify the core engine, you must install the Rust toolchain:</p>
      <CodeBlock lang="bash" code={`git clone ${repoUrl}.git
sudo apt install bubblewrap`} />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">Verify Installation</h2>
      <CodeBlock lang="bash" code={`myth --version
myth check`} />
    </div>
  );
}
