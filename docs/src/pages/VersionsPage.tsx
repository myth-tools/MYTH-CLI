import { useState, useEffect } from 'react';
import { PageHeader, InfoCard } from '../components/Layout';
import { Shield, Box, History, Loader2, AlertCircle } from 'lucide-react';

interface VersionInfo {
  version: string;
  architecture: string;
  size: string;
  description: string;
  maintainer: string;
  section: string;
}

export default function VersionsPage() {
  const [versions, setVersions] = useState<VersionInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const pagesUrl = import.meta.env.VITE_PAGES_URL || 'https://myth.work.gd';

  useEffect(() => {
    async function fetchVersions() {
      try {
        // Fetch the APT Packages manifest
        const response = await fetch(`${pagesUrl}/dists/stable/main/binary-amd64/Packages`);
        if (!response.ok) throw new Error('Neurological registry unreachable. Repository may be offline.');
        const text = await response.text();
        
        // Parse the Debian-style control format
        const packageBlocks = text.split('\n\n');
        const parsedVersions: VersionInfo[] = packageBlocks
          .map(block => {
            const lines = block.split('\n');
            const getField = (name: string) => {
              const line = lines.find(l => l.startsWith(`${name}:`));
              return line ? line.split(': ')[1] : '';
            };
            
            return {
              version: getField('Version'),
              architecture: getField('Architecture'),
              size: getField('Size'),
              description: getField('Description'),
              maintainer: getField('Maintainer'),
              section: getField('Section'),
            };
          })
          .filter(v => v.version);
          
        // Sort by version (descending)
        const sorted = parsedVersions.sort((a, b) => {
          return b.version.localeCompare(a.version, undefined, { numeric: true, sensitivity: 'base' });
        });
        
        setVersions(sorted);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Unknown technical error');
      } finally {
        setLoading(false);
      }
    }

    fetchVersions();
  }, [pagesUrl]);

  return (
    <div className="space-y-8">
      <PageHeader
        title="Tactical Version Registry"
        description="Live version history synchronized directly with the primary MYTH repository."
        badge="Mission History"
      />

      {loading ? (
        <div className="flex flex-col items-center justify-center py-20 gap-4">
          <Loader2 className="w-10 h-10 text-cyber-primary animate-spin" />
          <p className="text-cyber-dim font-mono text-sm animate-pulse">SYNCHRONIZING REGISTRY...</p>
        </div>
      ) : error ? (
        <InfoCard title="Registry Error" icon={<AlertCircle className="w-5 h-5 text-red-500" />}>
          <p className="text-red-400 mb-2">{error}</p>
          <p className="text-xs text-cyber-dim italic">Check your network uplink or verify if the gateway at {pagesUrl} is active.</p>
        </InfoCard>
      ) : (
        <div className="space-y-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <History className="w-5 h-5 text-cyber-primary" />
              <h2 className="text-xl font-bold text-white uppercase tracking-tighter">Availability Stream</h2>
            </div>
            <span className="text-xs text-cyber-dim font-mono">{versions.length} ACTIVE RELEASES</span>
          </div>

          <div className="grid gap-4">
            {versions.map((v, i) => (
              <div 
                key={`${v.version}-${i}`} 
                className={`glass-panel border-l-4 transition-all hover:translate-x-1 ${
                  i === 0 ? 'border-cyber-primary ring-1 ring-cyber-primary/20 bg-cyber-primary/5' : 'border-cyber-border'
                } rounded-lg p-5 relative overflow-hidden group`}
              >
                {i === 0 && (
                  <div className="absolute top-0 right-0 p-2">
                    <span className="px-2 py-0.5 text-[10px] bg-cyber-primary text-black font-bold rounded uppercase">Latest Stable</span>
                  </div>
                )}
                
                <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
                  <div>
                    <div className="flex items-center gap-3 mb-1">
                      <span className="text-2xl font-bold text-white">v{v.version}</span>
                      <span className="px-2 py-0.5 text-[10px] border border-cyber-border text-cyber-dim rounded uppercase font-mono">{v.architecture}</span>
                    </div>
                    <p className="text-sm text-cyber-text/80 mb-3">{v.description}</p>
                    
                    <div className="flex flex-wrap gap-4 text-[11px] font-mono text-cyber-dim">
                      <div className="flex items-center gap-1">
                        <Box className="w-3 h-3" />
                        <span>SIZE: {(parseInt(v.size) / 1024 / 1024).toFixed(2)} MB</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <Shield className="w-3 h-3" />
                        <span>MAINTAINER: {v.maintainer}</span>
                      </div>
                      <div className="flex items-center gap-1">
                        <Loader2 className="w-3 h-3" />
                        <span>SUITE: {v.section}</span>
                      </div>
                    </div>
                  </div>
                  
                  <div className="flex items-center gap-2">
                    <button 
                      onClick={() => window.location.href = '#/installation'}
                      className="px-4 py-2 text-xs font-bold uppercase tracking-widest bg-white/5 border border-white/10 hover:bg-cyber-primary hover:text-black transition-all rounded-md"
                    >
                      Install Protocol
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>

          <p className="text-center text-xs text-cyber-dim italic mt-10">
            Version data is parsed in real-time from the Debian Packages manifest. 
            Cryptographic signatures are verified by the internal package manager on deployment.
          </p>
        </div>
      )}
    </div>
  );
}
