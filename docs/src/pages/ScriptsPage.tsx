import { PageHeader, CodeBlock } from '../components/Layout';

const scripts = [
  {
    name: 'install.sh',
    desc: 'The official one-liner installer. It automatically configures the APT repository, installs tactical dependencies like bubblewrap, and falls back to a neural source build if the repository is unreachable.',
    code: 'curl -sSL https://myth.work.gd/install.sh | sudo bash',
  },
  {
    name: 'build_deb.sh',
    desc: 'Builds a .deb package using cargo-deb. Includes user.yaml config, README, and creates the myth + agent symlinks.',
    code: 'bash scripts/build_deb.sh',
  },
  {
    name: 'init_repo.sh',
    desc: 'Initializes a local APT repository structure for self-hosted package distribution. Signs with GPG and generates Release files.',
    code: 'bash scripts/init_repo.sh',
  },
  {
    name: 'release_local.sh',
    desc: 'Combines build_deb.sh + init_repo.sh for a complete local release workflow. Builds, packages, signs, and publishes.',
    code: 'bash scripts/release_local.sh',
  },
  {
    name: 'uninstall.sh',
    desc: 'Dedicated uninstaller for source-based deployments. Safely removes binaries, symlinks, and configuration from the system.',
    code: 'sudo bash scripts/uninstall.sh',
  },
  {
    name: 'Maintainer Scripts (preinst, postinst, postrm)',
    desc: 'Standard Debian package lifecycle hooks. Handles safe pre-flight checks, symlinking (/usr/bin/agent), and deep purge operations.',
    code: 'cat scripts/postinst',
  },
  {
    name: 'conffiles',
    desc: 'Debian packaging directive to protect user overrides (/etc/myth/user.yaml) during apt upgrades.',
    code: 'cat scripts/conffiles',
  },
];

export default function ScriptsPage() {
  return (
    <div>
      <PageHeader title="Scripts" description="Helper scripts for installation, packaging, and release management." badge="Reference" />

      <div className="space-y-8">
        {scripts.map((s) => (
          <div key={s.name}>
            <h2 className="text-lg font-bold text-white font-mono mb-2">{s.name}</h2>
            <p className="text-sm text-cyber-text/80 mb-3">{s.desc}</p>
            <CodeBlock lang="bash" title="Run" code={s.code} />
          </div>
        ))}
      </div>
    </div>
  );
}
