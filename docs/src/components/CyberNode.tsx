import { Handle, Position } from "reactflow";

interface CyberNodeData {
	borderColor?: string;
	glowColor?: string;
	icon?: React.ReactNode;
	sublabel?: string;
	label: string;
	details?: string;
}

interface CyberNodeProps {
	data: CyberNodeData;
	selected: boolean;
}

export const CyberNode = ({ data, selected }: CyberNodeProps) => {
	const borderColor = data.borderColor || "rgba(0, 255, 136, 0.3)";
	const glowColor = data.glowColor || "rgba(0, 255, 136, 0.2)";

	return (
		<div
			className={`
      relative group transition-all duration-700
      ${selected ? "scale-110" : "scale-100"}
    `}
		>
			{/* High-Fidelity Tactical Glow */}
			<div
				className="absolute -inset-4 rounded-[2rem] blur-2xl opacity-0 group-hover:opacity-60 transition-opacity duration-700 pointer-events-none"
				style={{ background: `radial-gradient(circle, ${glowColor} 0%, transparent 70%)` }}
			/>

			<div
				className={`
          relative px-7 py-5 rounded-2xl border bg-black/90 backdrop-blur-3xl shadow-2xl min-w-[210px]
          transition-all duration-500 overflow-hidden
          ${selected ? "border-cyber-primary shadow-[0_0_30px_rgba(0,255,163,0.2)]" : ""}
        `}
				style={{ borderColor: selected ? undefined : borderColor }}
			>
				{/* Scanning Line Internal */}
				<div className="absolute inset-0 bg-gradient-to-b from-transparent via-white/[0.03] to-transparent h-1/2 w-full -translate-y-full group-hover:animate-scanline-fast pointer-events-none" />

				<div className="flex flex-col items-center gap-2 relative z-10">
					{data.icon && (
						<div className="text-cyber-primary mb-3 transform group-hover:scale-125 group-hover:rotate-6 transition-all duration-500 filter drop-shadow-[0_0_8px_rgba(0,255,163,0.5)]">
							{data.icon}
						</div>
					)}

					<div className="text-[10px] uppercase tracking-[0.4em] text-cyber-dim font-black opacity-60 group-hover:opacity-100 transition-opacity">
						{data.sublabel || "Component"}
					</div>

					<div className="text-sm font-black text-white tracking-widest uppercase text-center group-hover:text-cyber-primary transition-colors">
						{data.label}
					</div>

					{data.details && (
						<div className="mt-3 pt-3 border-t border-white/10 w-full flex flex-col gap-1.5">
							{data.details.split("\n").map((line: string) => (
								<div
									key={line}
									className="text-[9px] text-cyber-dim/80 font-mono text-center leading-tight tracking-tight"
								>
									{line}
								</div>
							))}
						</div>
					)}

					{/* Metadata Pulse Tag */}
					<div className="mt-4 flex items-center gap-1.5">
						<div className="w-1 h-1 rounded-full bg-cyber-primary animate-ping" />
						<div className="text-[7px] font-mono text-cyber-primary/40 uppercase tracking-widest">
							Stream: Synchronized
						</div>
					</div>
				</div>

				<Handle
					type="target"
					position={Position.Top}
					className="!bg-cyber-primary !border-cyber-bg !w-2 !h-2 !static !mx-auto !mt-[-26px] !opacity-0 group-hover:!opacity-100 transition-opacity"
				/>
				<Handle
					type="source"
					position={Position.Bottom}
					className="!bg-cyber-primary !border-cyber-bg !w-2 !h-2 !static !mx-auto !mb-[-26px] !opacity-0 group-hover:!opacity-100 transition-opacity"
				/>
			</div>

			{/* Corner Accents */}
			<div className="absolute top-0 left-0 w-2 h-2 border-t border-l border-white/20 rounded-tl-sm pointer-events-none" />
			<div className="absolute top-0 right-0 w-2 h-2 border-t border-r border-white/20 rounded-tr-sm pointer-events-none" />
			<div className="absolute bottom-0 left-0 w-2 h-2 border-b border-l border-white/20 rounded-bl-sm pointer-events-none" />
			<div className="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-white/20 rounded-br-sm pointer-events-none" />
		</div>
	);
};
