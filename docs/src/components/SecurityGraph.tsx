import ReactFlow, { Background, Controls, type Edge, type Node } from "reactflow";
import "reactflow/dist/style.css";
import { AlertTriangle, Eye, HardDrive, Lock, Shield } from "lucide-react";
import { CyberNode } from "./CyberNode";

const nodeTypes = {
	cyber: CyberNode,
};

const initialNodes: Node[] = [
	{
		id: "policy",
		type: "cyber",
		data: {
			label: "SECURITY POLICY",
			sublabel: "Layer 1",
			details: "50+ Blocked Commands\nInjection Guards",
			icon: <Lock size={14} />,
			borderColor: "#ff444466",
			glowColor: "#ff444433",
		},
		position: { x: 250, y: 0 },
	},
	{
		id: "sandbox",
		type: "cyber",
		data: {
			label: "BWRAP SANDBOX",
			sublabel: "Layer 2",
			details: "Namespace Isolation\nKernel Hardening",
			icon: <Shield size={14} />,
			borderColor: "#00ff8866",
			glowColor: "#00ff8833",
		},
		position: { x: 250, y: 150 },
	},
	{
		id: "mounts",
		type: "cyber",
		data: {
			label: "MOUNT POLICY",
			sublabel: "Layer 3",
			details: "Read-only Host FS\ntmpfs Writable Dirs",
			icon: <HardDrive size={14} />,
			borderColor: "#0088ff66",
			glowColor: "#0088ff33",
		},
		position: { x: 250, y: 300 },
	},
	{
		id: "volatile",
		type: "cyber",
		data: {
			label: "VOLATILE STORAGE",
			sublabel: "Layer 4",
			details: "RAM-only Operation\nZero Persistence",
			icon: <Eye size={14} />,
			borderColor: "#ff005566",
			glowColor: "#ff005533",
		},
		position: { x: 250, y: 450 },
	},
	{
		id: "audit",
		type: "cyber",
		data: {
			label: "AUDIT ENGINE",
			sublabel: "Layer 5",
			details: "Real-time Verdicts\nViolation Cleanup",
			icon: <AlertTriangle size={14} />,
			borderColor: "#ffaa0066",
			glowColor: "#ffaa0033",
		},
		position: { x: 250, y: 600 },
	},
];

const initialEdges: Edge[] = [
	{
		id: "e1-2",
		source: "policy",
		target: "sandbox",
		animated: true,
		style: { stroke: "#ff4444", strokeWidth: 2 },
	},
	{
		id: "e2-3",
		source: "sandbox",
		target: "mounts",
		animated: true,
		style: { stroke: "#00ff88", strokeWidth: 2 },
	},
	{
		id: "e3-4",
		source: "mounts",
		target: "volatile",
		animated: true,
		style: { stroke: "#0088ff", strokeWidth: 2 },
	},
	{
		id: "e4-5",
		source: "volatile",
		target: "audit",
		animated: true,
		style: { stroke: "#ff0055", strokeWidth: 2 },
	},
];

import { useEffect, useRef, useState } from "react";

export default function SecurityGraph() {
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
			className={`${isFullscreen ? "fixed inset-0 z-[100] h-screen w-screen" : "h-[450px] w-full"} glass-panel rounded-2xl overflow-hidden relative border border-cyber-border/50 group transition-all duration-500 shadow-[0_0_50px_-12px_rgba(0,0,0,0.8)]`}
		>
			{/* Tactical HUD Overlays */}
			<div className="absolute inset-0 pointer-events-none z-20 overflow-hidden">
				<div className="absolute top-0 left-0 w-20 h-20 border-t-2 border-l-2 border-cyber-error/20 rounded-tl-3xl translate-x-4 translate-y-4" />
				<div className="absolute top-0 right-0 w-20 h-20 border-t-2 border-r-2 border-cyber-error/20 rounded-tr-3xl -translate-x-4 translate-y-4" />
				<div className="absolute bottom-0 left-0 w-20 h-20 border-b-2 border-l-2 border-cyber-error/20 rounded-bl-3xl translate-x-4 -translate-y-4" />
				<div className="absolute bottom-0 right-0 w-20 h-20 border-b-2 border-r-2 border-cyber-error/20 rounded-br-3xl -translate-x-4 -translate-y-4" />

				{/* Side Telemetry */}
				<div className="absolute left-6 top-1/2 -translate-y-1/2 flex flex-col gap-6 items-center opacity-30">
					{[1, 2, 3, 4].map((i) => (
						<div
							key={i}
							className="w-1 h-8 bg-cyber-error/40 rounded-full animate-pulse"
							style={{ animationDelay: `${i * 200}ms` }}
						/>
					))}
				</div>
				<div className="absolute right-8 top-1/2 -translate-y-1/2 flex flex-col gap-1 text-[8px] font-mono text-cyber-error/40 tracking-tighter uppercase whitespace-nowrap rotate-90">
					<span>Mission_Critical_Auth</span>
					<span>Latency.0ms</span>
					<span>Buffer.Active</span>
				</div>
			</div>

			<div className="absolute top-6 left-8 z-30 flex flex-col gap-1.5">
				<div className="flex items-center gap-3">
					<span className="px-3 py-1 bg-cyber-error text-black text-[10px] uppercase tracking-[0.3em] font-black rounded-sm shadow-[0_0_15px_rgba(255,68,68,0.4)]">
						TACTICAL_DEFENSE_MATRIX
					</span>
					<span className="text-[10px] font-mono text-cyber-error animate-pulse font-bold tracking-widest uppercase">
						Live_Feed
					</span>
				</div>
				<button
					type="button"
					onClick={toggleFullscreen}
					className="text-[9px] text-cyber-dim font-mono ml-1 hover:text-white transition-colors text-left flex items-center gap-2 group/btn"
				>
					<span className="opacity-40 group-hover/btn:opacity-100 transition-opacity">
						{isFullscreen ? "[ STOP_VO_LINK ]" : "[ ACTIVATE_TACTICAL_LINK ]"}
					</span>
				</button>
			</div>

			<div className="absolute bottom-6 right-8 z-30 flex flex-col items-end gap-1 opacity-40">
				<div className="text-[12px] font-black text-white italic">MYTH_CORE</div>
				<div className="text-[8px] font-mono text-cyber-dim">SYSTEM_UPTIME: 100%</div>
			</div>

			<ReactFlow
				nodes={initialNodes}
				edges={initialEdges}
				nodeTypes={nodeTypes}
				fitView
				className="bg-[#050508]"
				proOptions={{ hideAttribution: true }}
			>
				<Background color="#1a1a2e" gap={28} size={1.2} />
				<Controls
					showInteractive={false}
					className="!bg-black/80 !border-white/5 !fill-cyber-error !m-6 !rounded-xl !p-1"
				/>
			</ReactFlow>

			{/* Bottom Scanline Beam */}
			<div className="absolute inset-0 pointer-events-none bg-gradient-to-b from-transparent via-cyber-error/[0.03] to-transparent h-20 w-full animate-scanline-fast z-10" />
			<div className="absolute inset-0 pointer-events-none shadow-[inset_0_0_150px_rgba(0,0,0,0.7)]" />
		</div>
	);
}
