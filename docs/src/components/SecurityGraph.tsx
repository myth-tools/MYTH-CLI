import ReactFlow, { 
  Background, 
  Controls, 
  type Node,
  type Edge,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { CyberNode } from './CyberNode';
import { Shield, Lock, Eye, HardDrive, AlertTriangle } from 'lucide-react';

const nodeTypes = {
  cyber: CyberNode,
};

const initialNodes: Node[] = [
  {
    id: 'policy',
    type: 'cyber',
    data: { 
      label: 'SECURITY POLICY', 
      sublabel: 'Layer 1', 
      details: '50+ Blocked Commands\nInjection Guards',
      icon: <Lock size={14} />,
      borderColor: '#ff444466',
      glowColor: '#ff444433'
    },
    position: { x: 250, y: 0 },
  },
  {
    id: 'sandbox',
    type: 'cyber',
    data: { 
      label: 'BWRAP SANDBOX', 
      sublabel: 'Layer 2', 
      details: 'Namespace Isolation\nKernel Hardening',
      icon: <Shield size={14} />,
      borderColor: '#00ff8866',
      glowColor: '#00ff8833'
    },
    position: { x: 250, y: 150 },
  },
  {
    id: 'mounts',
    type: 'cyber',
    data: { 
      label: 'MOUNT POLICY', 
      sublabel: 'Layer 3', 
      details: 'Read-only Host FS\ntmpfs Writable Dirs',
      icon: <HardDrive size={14} />,
      borderColor: '#0088ff66',
      glowColor: '#0088ff33'
    },
    position: { x: 250, y: 300 },
  },
  {
    id: 'volatile',
    type: 'cyber',
    data: { 
      label: 'VOLATILE STORAGE', 
      sublabel: 'Layer 4', 
      details: 'RAM-only Operation\nZero Persistence',
      icon: <Eye size={14} />,
      borderColor: '#ff005566',
      glowColor: '#ff005533'
    },
    position: { x: 250, y: 450 },
  },
  {
    id: 'audit',
    type: 'cyber',
    data: { 
      label: 'AUDIT ENGINE', 
      sublabel: 'Layer 5', 
      details: 'Real-time Verdicts\nViolation Cleanup',
      icon: <AlertTriangle size={14} />,
      borderColor: '#ffaa0066',
      glowColor: '#ffaa0033'
    },
    position: { x: 250, y: 600 },
  }
];

const initialEdges: Edge[] = [
  { id: 'e1-2', source: 'policy', target: 'sandbox', animated: true, style: { stroke: '#ff4444', strokeWidth: 2 } },
  { id: 'e2-3', source: 'sandbox', target: 'mounts', animated: true, style: { stroke: '#00ff88', strokeWidth: 2 } },
  { id: 'e3-4', source: 'mounts', target: 'volatile', animated: true, style: { stroke: '#0088ff', strokeWidth: 2 } },
  { id: 'e4-5', source: 'volatile', target: 'audit', animated: true, style: { stroke: '#ff0055', strokeWidth: 2 } },
];

import { useState, useEffect, useRef } from 'react';

export default function SecurityGraph() {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const toggleFullscreen = () => {
    if (!containerRef.current) return;
    if (!document.fullscreenElement) {
      containerRef.current.requestFullscreen();
      setIsFullscreen(true);
    } else {
      document.exitFullscreen();
      setIsFullscreen(false);
    }
  };

  useEffect(() => {
    const handleFsChange = () => setIsFullscreen(!!document.fullscreenElement);
    document.addEventListener('fullscreenchange', handleFsChange);
    return () => document.removeEventListener('fullscreenchange', handleFsChange);
  }, []);

  return (
    <div 
      ref={containerRef}
      className={`${isFullscreen ? 'fixed inset-0 z-[100] h-screen w-screen' : 'h-[450px] w-full'} glass-panel rounded-2xl overflow-hidden relative border border-cyber-border/50 group transition-all duration-300`}
    >
      <div className="absolute top-4 left-4 z-10 flex flex-col gap-1">
        <span className="px-2 py-1 bg-cyber-error/10 text-cyber-error text-[9px] uppercase tracking-widest font-black border border-cyber-error/20 rounded-md">
          DEFENSE_MATRIX v4.0
        </span>
        <button 
          onClick={toggleFullscreen}
          className="text-[8px] text-cyber-dim font-mono animate-pulse ml-1 hover:text-cyber-error transition-colors text-left"
        >
          {isFullscreen ? '[ EXIT_FULLSCREEN ]' : '[ ACTIVATE_FULLSCREEN ]'}
        </button>
      </div>
      
      <ReactFlow
        nodes={initialNodes}
        edges={initialEdges}
        nodeTypes={nodeTypes}
        fitView
        className="bg-cyber-bg/40"
        proOptions={{ hideAttribution: true }}
      >
        <Background color="#1a1a2e" gap={24} size={1} />
        <Controls showInteractive={false} className="!bg-black/80 !border-white/5 !fill-cyber-error" />
      </ReactFlow>
      <div className="absolute inset-0 pointer-events-none shadow-[inset_0_0_100px_rgba(0,0,0,0.5)]" />
    </div>
  );
}
