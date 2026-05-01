import ReactFlow, { Background, Controls, type Edge, MarkerType, type Node } from "reactflow";
import "reactflow/dist/style.css";
import { Brain, Database, FileJson, Search } from "lucide-react";
import { CyberNode } from "./CyberNode";

const nodeTypes = {
	cyber: CyberNode,
};

const initialNodes: Node[] = [
	{
		id: "output",
		type: "cyber",
		data: {
			label: "TOOL OUTPUTS",
			sublabel: "Input Data",
			details: "Findings / Logs\nTool Bridge Results",
			icon: <FileJson size={14} />,
			borderColor: "#00ff8866",
			glowColor: "#00ff8833",
		},
		position: { x: 250, y: 0 },
	},
	{
		id: "nim",
		type: "cyber",
		data: {
			label: "NVIDIA NIM API",
			sublabel: "Embedding Engine",
			details: "NV-Embed-QA\n1024-dim Vector",
			icon: <Brain size={14} />,
			borderColor: "#0088ff66",
			glowColor: "#0088ff33",
		},
		position: { x: 250, y: 150 },
	},
	{
		id: "qdrant",
		type: "cyber",
		data: {
			label: "QDRANT STORE",
			sublabel: "Vector DB",
			details: "In-Memory RAM Store\nVolatile Collection",
			icon: <Database size={14} />,
			borderColor: "#ff005566",
			glowColor: "#ff005533",
		},
		position: { x: 250, y: 300 },
	},
	{
		id: "search",
		type: "cyber",
		data: {
			label: "SEMANTIC SEARCH",
			sublabel: "Recall Logic",
			details: "Cosine Similarity\nContext Injection",
			icon: <Search size={14} />,
			borderColor: "#ffaa0066",
			glowColor: "#ffaa0033",
		},
		position: { x: 250, y: 450 },
	},
];

const initialEdges: Edge[] = [
	{
		id: "e1-2",
		source: "output",
		target: "nim",
		animated: true,
		markerEnd: { type: MarkerType.ArrowClosed, color: "#00ff88" },
		style: { stroke: "#00ff88", strokeWidth: 2 },
	},
	{
		id: "e2-3",
		source: "nim",
		target: "qdrant",
		animated: true,
		markerEnd: { type: MarkerType.ArrowClosed, color: "#0088ff" },
		style: { stroke: "#0088ff", strokeWidth: 2 },
	},
	{
		id: "e3-4",
		source: "qdrant",
		target: "search",
		animated: true,
		markerEnd: { type: MarkerType.ArrowClosed, color: "#ff0055" },
		style: { stroke: "#ff0055", strokeWidth: 2 },
	},
];

import { useEffect, useRef, useState } from "react";

export default function MemoryGraph() {
	const [isFullscreen, setIsFullscreen] = useState(false);
	const containerRef = useRef<HTMLDivElement>(null);

	const toggleFullscreen = () => {
		if (!containerRef.current) {
			return;
		}
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
		document.addEventListener("fullscreenchange", handleFsChange);
		return () => document.removeEventListener("fullscreenchange", handleFsChange);
	}, []);

	return (
		<div
			ref={containerRef}
			className={`${isFullscreen ? "fixed inset-0 z-[100] h-screen w-screen" : "h-[400px] w-full"} glass-panel rounded-2xl overflow-hidden relative border border-cyber-border/50 group transition-all duration-300 shadow-[0_0_50px_-12px_rgba(0,255,163,0.2)]`}
		>
			{/* Tactical HUD Overlays */}
			<div className="absolute inset-0 pointer-events-none z-20">
				<div className="absolute top-0 left-0 w-16 h-16 border-t-2 border-l-2 border-cyber-primary/20 rounded-tl-2xl translate-x-4 translate-y-4" />
				<div className="absolute top-0 right-0 w-16 h-16 border-t-2 border-r-2 border-cyber-primary/20 rounded-tr-2xl -translate-x-4 translate-y-4" />
				<div className="absolute bottom-0 left-0 w-16 h-16 border-b-2 border-l-2 border-cyber-primary/20 rounded-bl-2xl translate-x-4 -translate-y-4" />
				<div className="absolute bottom-0 right-0 w-16 h-16 border-b-2 border-r-2 border-cyber-primary/20 rounded-br-2xl -translate-x-4 -translate-y-4" />
			</div>

			<div className="absolute top-6 left-8 z-30 flex flex-col gap-1">
				<div className="flex items-center gap-3">
					<span className="px-3 py-1 bg-cyber-primary text-black text-[10px] uppercase tracking-[0.3em] font-black rounded-sm">
						VOLATILE_STORAGE_RADAR
					</span>
					<div className="flex gap-1">
						{[1, 2, 3].map((i) => (
							<div
								key={i}
								className="w-1 h-3 bg-cyber-primary/40 rounded-full animate-pulse"
								style={{ animationDelay: `${i * 150}ms` }}
							/>
						))}
					</div>
				</div>
				<button
					type="button"
					onClick={toggleFullscreen}
					className="text-[9px] text-cyber-dim font-mono ml-1 hover:text-white transition-colors text-left flex items-center gap-2 group/btn"
				>
					<span className="opacity-40 group-hover/btn:opacity-100 transition-opacity">
						{isFullscreen ? "[ DISCONNECT_RADAR ]" : "[ INITIALIZE_RADAR_SYNC ]"}
					</span>
				</button>
			</div>

			<ReactFlow
				nodes={initialNodes}
				edges={initialEdges}
				nodeTypes={nodeTypes}
				fitView
				className="bg-[#020406]"
				proOptions={{ hideAttribution: true }}
			>
				<Background color="#00ff88" gap={30} size={1} />
				<Controls
					showInteractive={false}
					className="!bg-black/80 !border-white/5 !fill-cyber-primary !m-6 !rounded-xl"
				/>
			</ReactFlow>

			<div className="absolute bottom-8 left-8 z-30 flex items-center gap-4 opacity-30 text-[8px] font-mono text-cyber-dim uppercase tracking-widest">
				<div className="flex items-center gap-2">
					<div className="w-2 h-2 rounded-full bg-cyber-primary" />
					<span>SWEEP_ACTIVE</span>
				</div>
				<div className="flex items-center gap-2">
					<div className="w-2 h-2 rounded-full bg-cyber-secondary" />
					<span>SYNC_READY</span>
				</div>
			</div>

			<div className="absolute inset-0 pointer-events-none bg-[radial-gradient(circle_at_center,transparent_0%,rgba(0,255,163,0.05)_100%)]" />
			<div className="absolute inset-0 pointer-events-none shadow-[inset_0_0_120px_rgba(0,0,0,0.6)]" />
		</div>
	);
}
