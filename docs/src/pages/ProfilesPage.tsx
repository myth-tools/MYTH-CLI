import { PageHeader, CodeBlock } from '../components/Layout';
import { reconPhases } from '../data/content';

const profiles = [
  { name: 'quick', desc: 'Rapid surface-level scan covering phases 0-5', phases: '0-5', steps: 30 },
  { name: 'full', desc: 'Comprehensive 13-phase methodology with all 89 steps', phases: '0-12', steps: 89 },
  { name: 'stealth', desc: 'Low-noise passive-only operations prioritizing OSINT', phases: '0,1,9', steps: 15 },
  { name: 'vuln', desc: 'Vulnerability-focused assessment skipping passive phases', phases: '2,3,4,5,7,8', steps: 45 },
  { name: 'elite', desc: 'Maximum depth — all 13 phases enabled with extended depth', phases: '0-12', steps: 89 },
];

export default function ProfilesPage() {
  return (
    <div>
      <PageHeader title="Recon Profiles & Phases" description="MYTH uses a structured 13-phase, 89-step methodology. Profiles define which phases are active." badge="Commands" />

      <h2 className="text-xl font-bold text-white mb-4">Available Profiles</h2>
      <div className="space-y-3 mb-8">
        {profiles.map((p) => (
          <div key={p.name} className="feature-card rounded-lg p-4 flex items-center gap-4">
            <code className="text-cyber-primary font-bold text-base font-mono w-20 shrink-0">{p.name}</code>
            <div className="flex-1">
              <p className="text-sm text-cyber-text/80">{p.desc}</p>
              <div className="flex gap-4 mt-1">
                <span className="text-xs text-cyber-dim">Phases: <code className="text-cyber-primary">{p.phases}</code></span>
                <span className="text-xs text-cyber-dim">Steps: <code className="text-cyber-primary">{p.steps}</code></span>
              </div>
            </div>
          </div>
        ))}
      </div>

      <h2 className="text-xl font-bold text-white mb-4">Profile Management</h2>
      <CodeBlock lang="bash" title="View all profiles" code="myth profile" />
      <CodeBlock lang="bash" title="View specific profile" code="myth profile full" />
      <CodeBlock lang="bash" title="Disable phases" code="myth profile elite disable 3,4,5" />
      <CodeBlock lang="bash" title="Enable phases" code="myth profile elite enable 3,4,5" />

      <h2 className="text-xl font-bold text-white mb-4 mt-8">The 13 Phases</h2>
      <div className="space-y-2">
        {reconPhases.map((phase) => (
          <div key={phase.phase} className="flex gap-4 items-start p-3 rounded-lg border border-cyber-border hover:border-cyber-primary/30 transition-colors">
            <span className="text-cyber-primary font-mono text-sm font-bold shrink-0 w-8 text-center">{phase.phase}</span>
            <div>
              <h3 className="text-sm font-semibold text-white">{phase.name}</h3>
              <p className="text-xs text-cyber-dim mt-0.5">{phase.description}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
