import { motion } from "framer-motion";
import type { ReactNode } from "react";

interface SkeletonProps {
	className?: string;
	variant?: "text" | "rect" | "circle" | "badge";
	delay?: number;
}

export const Skeleton = ({ className = "", variant = "text", delay = 0 }: SkeletonProps) => {
	const variants = {
		text: "h-4 w-full rounded",
		rect: "h-32 w-full rounded-xl",
		circle: "h-12 w-12 rounded-full",
		badge: "h-5 w-16 rounded-full",
	};
	return (
		<div
			className={`skeleton ${variants[variant]} ${className}`}
			style={{ animationDelay: `${delay}ms` }}
		/>
	);
};

// Tactical blueprint-style skeleton for page header
const PageHeaderSkeleton = () => (
	<div className="mb-10 space-y-5 relative">
		<div className="flex items-center gap-3">
			<Skeleton variant="badge" className="w-20 h-5" delay={0} />
			<div className="w-px h-3 bg-cyber-primary/20" />
			<div className="text-[8px] font-mono text-cyber-primary/40 uppercase tracking-[0.3em] animate-pulse">
				AUTH_READY
			</div>
		</div>
		<Skeleton className="w-3/4 h-10 rounded-lg" delay={100} />
		<div className="space-y-2">
			<Skeleton className="w-full h-4 opacity-60" delay={200} />
			<Skeleton className="w-5/6 h-4 opacity-40" delay={300} />
		</div>
		<div className="h-px w-full bg-gradient-to-r from-cyber-primary/40 via-cyber-border/20 to-transparent mt-8" />
	</div>
);

const CardGridSkeleton = ({ count = 4 }: { count?: number }) => (
	<div className="grid grid-cols-1 sm:grid-cols-2 gap-5 mb-10">
		{Array.from({ length: count }).map((_, i) => (
			<div
				// biome-ignore lint/suspicious/noArrayIndexKey: Static grid placeholder
				key={`card-sk-${i}`}
				className="glass-panel border-white/[0.03] rounded-2xl h-36 p-5 flex flex-col gap-4"
			>
				<div className="flex items-center gap-3">
					<Skeleton variant="circle" className="w-8 h-8 opacity-40" delay={i * 100} />
					<Skeleton className="w-24 h-4" delay={i * 100 + 50} />
				</div>
				<Skeleton className="w-full h-12 rounded-lg opacity-20" delay={i * 100 + 100} />
			</div>
		))}
	</div>
);

export const PageSkeleton = () => (
	<motion.div
		initial={{ opacity: 0 }}
		animate={{ opacity: 1 }}
		className="max-w-4xl mx-auto px-6 py-12 space-y-8 relative overflow-hidden"
	>
		{/* Background Tactical Grid Grid */}
		<div className="absolute inset-0 bg-[radial-gradient(#ffffff03_1px,transparent_1px)] [background-size:20px_20px] pointer-events-none opacity-20" />

		<PageHeaderSkeleton />

		{/* Tactical Stream Status */}
		<div className="flex items-center justify-between mb-8 border-b border-white/[0.03] pb-4">
			<div className="flex items-center gap-4">
				<div className="relative">
					<div className="w-2.5 h-2.5 rounded-full bg-cyber-primary/40 animate-ping absolute inset-0" />
					<div className="w-2.5 h-2.5 rounded-full bg-cyber-primary relative" />
				</div>
				<div className="flex flex-col">
					<span className="text-[10px] font-mono text-cyber-primary uppercase tracking-[0.2em] font-bold">
						Tactical_Feed_Active
					</span>
					<span className="text-[8px] font-mono text-cyber-dim uppercase tracking-widest mt-0.5">
						Buffer: Synchronizing Core Registry
					</span>
				</div>
			</div>
			<div className="hidden sm:flex gap-1.5">
				{[1, 2, 3, 4].map((i) => (
					<div
						key={i}
						className="w-1 h-3 bg-cyber-primary/20 rounded-full animate-pulse"
						style={{ animationDelay: `${i * 200}ms` }}
					/>
				))}
			</div>
		</div>

		<CardGridSkeleton count={4} />

		<div className="space-y-4">
			<Skeleton
				className="h-32 rounded-2xl border border-white/[0.03] bg-white/[0.01]"
				delay={500}
			/>
			<div className="grid grid-cols-3 gap-4">
				<Skeleton className="h-20 rounded-xl" delay={600} />
				<Skeleton className="h-20 rounded-xl" delay={700} />
				<Skeleton className="h-20 rounded-xl" delay={800} />
			</div>
		</div>

		{/* Bottom Scanning Beam */}
		<div className="absolute left-0 right-0 h-px bg-cyber-primary/20 top-0 animate-scanline-fast pointer-events-none" />
	</motion.div>
);

export const SidebarSkeleton = () => (
	<div className="p-6 space-y-8 h-full bg-black/20">
		<div className="space-y-4">
			<div className="flex items-center gap-4">
				<Skeleton variant="circle" className="w-10 h-10 border border-cyber-primary/20" />
				<div className="space-y-2">
					<Skeleton className="w-24 h-4" />
					<Skeleton className="w-16 h-2 opacity-30" />
				</div>
			</div>
			<div className="h-px bg-white/5 w-full" />
		</div>

		<div className="space-y-6">
			{[1, 2, 3, 4, 5].map((i) => (
				<div key={i} className="space-y-3">
					<Skeleton className="w-20 h-2 opacity-20" delay={i * 50} />
					<div className="space-y-1.5 pl-4">
						<Skeleton className="w-full h-8 rounded-lg opacity-10" delay={i * 50 + 20} />
						<Skeleton className="w-11/12 h-8 rounded-lg opacity-5" delay={i * 50 + 40} />
					</div>
				</div>
			))}
		</div>
	</div>
);

export const SkeletonWrapper = ({ children }: { children: ReactNode }) => (
	<div className="premium-shimmer">{children}</div>
);
