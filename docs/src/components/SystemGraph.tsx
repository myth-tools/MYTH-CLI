import ReactFlow, { 
  Background, 
  Controls, 
  MarkerType,
  type Node,
  type Edge,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { CyberNode } from './CyberNode';
import { Terminal, Cpu, Database, Layers, Brain } from 'lucide-react';

const nodeTypes = {
  cyber: CyberNode,
};

const initialNodes: Node[] = [
  { 
    id: 'user', 
    type: 'cyber',
    data: { 
      label: 'USER INTERFACE', 
      sublabel: 'Term-Ops', 
      details: 'Dual Mode: TUI / CLI\nFuzzy Dispatcher',
      icon: <Terminal size={14} />,
      borderColor: '#0088ff66',
      glowColor: '#0088ff33'
    }, 
    position: { x: 250, y: 0 }
  },
  { 
    id: 'core', 
    type: 'cyber',
    data: { 
      label: 'NEURAL CORE', 
      sublabel: 'Agent Brain', 
      details: 'Rig.rs Engine\nNIM Strategy Layer',
      icon: <Cpu size={14} />,
      borderColor: '#00ff8866',
      glowColor: '#00ff8833'
    }, 
    position: { x: 250, y: 150 }
  },
  { 
    id: 'llm', 
    type: 'cyber',
    data: { 
      label: 'NVIDIA NIM', 
      sublabel: 'Intelligence', 
      details: 'Meta-Llama 3.1 70B\nSemantic Reasoning',
      icon: <Brain size={14} />,
      borderColor: '#ff005566',
      glowColor: '#ff005533'
    }, 
    position: { x: 0, y: 300 }
  },
  { 
    id: 'memory', 
    type: 'cyber',
    data: { 
      label: 'QDRANT STORE', 
      sublabel: 'Volatile Memory', 
      details: 'Vector Embeddings\nCosine Similarity',
      icon: <Database size={14} />,
      borderColor: '#ffaa0066',
      glowColor: '#ffaa0033'
    }, 
    position: { x: 500, y: 300 }
  },
  { 
    id: 'mcp', 
    type: 'cyber',
    data: { 
      label: 'MCP BRIDGES', 
      sublabel: 'Interconnect', 
      details: 'Tool Bridges (16)\nServer Discovery',
      icon: <Layers size={14} />,
      borderColor: '#0088ff66',
      glowColor: '#0088ff33'
    }, 
    position: { x: 250, y: 450 }
  },
];

const initialEdges: Edge[] = [
  { 
    id: 'e1-2', source: 'user', target: 'core', animated: true, 
    markerEnd: { type: MarkerType.ArrowClosed, color: '#0088ff' }, 
    style: { stroke: '#0088ff', strokeWidth: 2 } 
  },
  { 
    id: 'e2-3', source: 'core', target: 'llm', 
    markerEnd: { type: MarkerType.ArrowClosed, color: '#00ff88' }, 
    style: { stroke: '#00ff88', strokeWidth: 2 },
    label: 'REASONING',
    labelStyle: { fill: '#fff', fontSize: 8, fontWeight: 900 }
  },
  { 
    id: 'e3-4', source: 'llm', target: 'core', 
    markerEnd: { type: MarkerType.ArrowClosed, color: '#ff0055' }, 
    style: { stroke: '#ff0055', strokeWidth: 2, strokeDasharray: '5,5' },
    label: 'ACTIONS',
    labelStyle: { fill: '#fff', fontSize: 8, fontWeight: 900 }
  },
  { 
    id: 'e2- memory', source: 'core', target: 'memory', animated: true, 
    markerEnd: { type: MarkerType.ArrowClosed, color: '#ffaa00' }, 
    style: { stroke: '#ffaa00', strokeWidth: 2 },
    label: 'STORE/RECALL',
    labelStyle: { fill: '#fff', fontSize: 8, fontWeight: 900 }
  },
  { 
    id: 'e2-5', source: 'core', target: 'mcp', 
    markerEnd: { type: MarkerType.ArrowClosed, color: '#0088ff' }, 
    style: { stroke: '#0088ff', strokeWidth: 2 } 
  },
];

import { useState, useEffect, useRef } from 'react';

export default function SystemGraph() {
  const [isFullscreen, setIsFullscreen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const toggleFullscreen = () => {
    if (!containerRef.current) return;
    if (!document.fullscreenElement) {
      containerRef.current.requestFullscreen().catch((err) => {
        console.error(`Error attempting to enable full-screen mode: ${err.message}`);
      });
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
        <span className="px-2 py-1 bg-cyber-primary/10 text-cyber-primary text-[9px] uppercase tracking-widest font-black border border-cyber-primary/20 rounded-md">
          NEURAL INTERCONNECT v4.0
        </span>
        <button 
          onClick={toggleFullscreen}
          className="text-[8px] text-cyber-dim font-mono animate-pulse ml-1 hover:text-cyber-primary transition-colors text-left"
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
        <Controls showInteractive={false} className="!bg-black/80 !border-white/5 !fill-cyber-primary" />
      </ReactFlow>

      {/* Edge vignette */}
      <div className="absolute inset-0 pointer-events-none shadow-[inset_0_0_100px_rgba(0,0,0,0.5)]" />
    </div>
  );
}
