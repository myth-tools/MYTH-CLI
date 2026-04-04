import ReactFlow, { Background, Controls, type Edge, MarkerType, type Node } from "reactflow";
import "reactflow/dist/style.css";
import { Brain, Cpu, Database, Layers, Terminal } from "lucide-react";
import { CyberNode } from "./CyberNode";

const nodeTypes = {
	cyber: CyberNode,
};

const initialNodes: Node[] = [
	{
		id: "user",
		type: "cyber",
		data: {
			label: "USER INTERFACE",
			sublabel: "Term-Ops",
			details: "Dual Mode: TUI / CLI\nFuzzy Dispatcher",
			icon: <Terminal size={14} />,
			borderColor: "#0088ff66",
			glowColor: "#0088ff33",
		},
		position: { x: 250, y: 0 },
	},
	{
		id: "core",
		type: "cyber",
		data: {
			label: "NEURAL CORE",
			sublabel: "Agent Brain",
			details: "Rig.rs Engine\nNIM Strategy Layer",
			icon: <Cpu size={14} />,
			borderColor: "#00ff8866",
			glowColor: "#00ff8833",
		},
		position: { x: 250, y: 150 },
	},
	{
		id: "llm",
		type: "cyber",
		data: {
			label: "NVIDIA NIM",
			sublabel: "Intelligence",
			details: "Meta-Llama 3.1 70B\nSemantic Reasoning",
			icon: <Brain size={14} />,
			borderColor: "#ff005566",
			glowColor: "#ff005533",
		},
		position: { x: 0, y: 300 },
	},
	{
		id: "memory",
		type: "cyber",
		data: {
			label: "QDRANT STORE",
			sublabel: "Volatile Memory",
			details: "Vector Embeddings\nCosine Similarity",
			icon: <Database size={14} />,
			borderColor: "#ffaa0066",
			glowColor: "#ffaa0033",
		},
		position: { x: 500, y: 300 },
	},
	{
		id: "mcp",
		type: "cyber",
		data: {
			label: "MCP BRIDGES",
			sublabel: "Interconnect",
			details: "Tool Bridges (16)\nServer Discovery",
			icon: <Layers size={14} />,
			borderColor: "#0088ff66",
			glowColor: "#0088ff33",
		},
		position: { x: 250, y: 450 },
	},
];

const initialEdges: Edge[] = [
	{
		id: "e1-2",
		source: "user",
		target: "core",
		animated: true,
		markerEnd: { type: MarkerType.ArrowClosed, color: "#0088ff" },
		style: { stroke: "#0088ff", strokeWidth: 2 },
	},
	{
		id: "e2-3",
		source: "core",
		target: "llm",
		markerEnd: { type: MarkerType.ArrowClosed, color: "#00ff88" },
		style: { stroke: "#00ff88", strokeWidth: 2 },
		label: "REASONING",
		labelStyle: { fill: "#fff", fontSize: 8, fontWeight: 900 },
	},
	{
		id: "e3-4",
		source: "llm",
		target: "core",
		markerEnd: { type: MarkerType.ArrowClosed, color: "#ff0055" },
		style: { stroke: "#ff0055", strokeWidth: 2, strokeDasharray: "5,5" },
		label: "ACTIONS",
		labelStyle: { fill: "#fff", fontSize: 8, fontWeight: 900 },
	},
	{
		id: "e2- memory",
		source: "core",
		target: "memory",
		animated: true,
		markerEnd: { type: MarkerType.ArrowClosed, color: "#ffaa00" },
		style: { stroke: "#ffaa00", strokeWidth: 2 },
		label: "STORE/RECALL",
		labelStyle: { fill: "#fff", fontSize: 8, fontWeight: 900 },
	},
	{
		id: "e2-5",
		source: "core",
		target: "mcp",
		markerEnd: { type: MarkerType.ArrowClosed, color: "#0088ff" },
		style: { stroke: "#0088ff", strokeWidth: 2 },
	},
];

import { useEffect, useRef, useState } from "react";

export default function SystemGraph() {
	const [isFullscreen, setIsFullscreen] = useState(false);
	const containerRef = useRef<HTMLDivElement>(null);

	const toggleFullscreen = () => {
		if (!containerRef.current) {
			return;
		}
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
		document.addEventListener("fullscreenchange", handleFsChange);
		return () => document.removeEventListener("fullscreenchange", handleFsChange);
	}, []);

	return (
		<div
			ref={containerRef}
			className={`${isFullscreen ? "fixed inset-0 z-[100] h-screen w-screen" : "h-[500px] w-full"} glass-panel rounded-3xl overflow-hidden relative border border-white/5 group transition-all duration-500 shadow-[0_30px_60px_-12px_rgba(0,0,0,0.5)]`}
		>
			{/* Elite HUD Overlays */}
			<div className="absolute inset-0 pointer-events-none z-20">
				{/* Corner Brackets */}
				<div className="absolute top-8 left-8 w-12 h-12 border-t border-l border-cyber-primary/30" />
				<div className="absolute top-8 right-8 w-12 h-12 border-t border-r border-cyber-primary/30" />
				<div className="absolute bottom-8 left-8 w-12 h-12 border-b border-l border-cyber-primary/30" />
				<div className="absolute bottom-8 right-8 w-12 h-12 border-b border-r border-cyber-primary/30" />

				{/* HUD Text Lines */}
				<div className="absolute top-1/2 left-8 -translate-y-1/2 flex flex-col gap-2 text-[7px] font-mono text-cyber-primary/30 uppercase tracking-[0.2em]">
					<span>CORE_LINK: ONLINE</span>
					<span>NIM_BUFF: OK</span>
					<span>MCP_LOAD: 0.12ms</span>
				</div>
				<div className="absolute top-1/2 right-8 -translate-y-1/2 flex flex-col gap-2 text-[7px] font-mono text-cyber-primary/30 uppercase tracking-[0.2em] items-end">
					<span>SNDBX: ENFORCED</span>
					<span>RAM_MOD: VOLATILE</span>
					<span>TRGT_LOCK: ACTIVE</span>
				</div>
			</div>

			<div className="absolute top-8 left-10 z-30 flex flex-col gap-2">
				<div className="flex items-center gap-4">
					<div className="flex items-center justify-center w-5 h-5 rounded-lg bg-cyber-primary/10 border border-cyber-primary/30 shadow-[0_0_15px_rgba(0,255,163,0.2)]">
						<Brain className="w-3 h-3 text-cyber-primary animate-pulse" />
					</div>
					<h3 className="text-sm font-black text-white tracking-[0.3em] uppercase">
						Neural_Orchestration_Grid
					</h3>
				</div>
				<button
					type="button"
					onClick={toggleFullscreen}
					className="text-[9px] text-cyber-dim font-mono ml-9 hover:text-cyber-primary transition-all flex items-center gap-2 group/btn"
				>
					<span className="opacity-40 group-hover/btn:opacity-100 transition-opacity">
						{isFullscreen ? "[ DECOUPLE_HUD ]" : "[ PROJECT_COMMAND_HUD ]"}
					</span>
				</button>
			</div>

			<div className="absolute bottom-10 left-1/2 -translate-x-1/2 z-30 flex gap-10 opacity-20 hover:opacity-50 transition-opacity duration-700">
				<div className="flex flex-col items-center gap-1">
					<div className="h-1 w-20 bg-gradient-to-r from-transparent via-cyber-primary to-transparent" />
					<span className="text-[7px] font-mono text-cyber-primary tracking-widest uppercase">
						Sync_Active
					</span>
				</div>
				<div className="flex flex-col items-center gap-1 text-cyber-error">
					<div className="h-1 w-20 bg-gradient-to-r from-transparent via-cyber-error to-transparent" />
					<span className="text-[7px] font-mono tracking-widest uppercase">Encryp_High</span>
				</div>
			</div>

			<ReactFlow
				nodes={initialNodes}
				edges={initialEdges}
				nodeTypes={nodeTypes}
				fitView
				className="bg-[#050608]"
				proOptions={{ hideAttribution: true }}
			>
				<Background color="#111" gap={40} size={1} />
				<Controls
					showInteractive={false}
					className="!bg-black/60 !border-white/5 !fill-cyber-primary !m-10 !rounded-2xl !p-2 !backdrop-blur-xl"
				/>
			</ReactFlow>

			{/* Scanning Beam Overlay */}
			<div className="absolute inset-0 pointer-events-none bg-gradient-to-r from-transparent via-cyber-primary/[0.02] to-transparent w-40 h-full animate-scanline-horizontal z-10" />
			<div className="absolute inset-0 pointer-events-none shadow-[inset_0_0_200px_rgba(0,0,0,0.8)]" />
		</div>
	);
}
