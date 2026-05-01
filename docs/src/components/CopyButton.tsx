import { AnimatePresence, motion } from "framer-motion";
import { Check, Copy } from "lucide-react";
import { useCallback, useState } from "react";

interface CopyButtonProps {
	text: string;
	className?: string;
}

export function CopyButton({ text, className = "" }: CopyButtonProps) {
	const [copied, setCopied] = useState(false);

	const handleCopy = useCallback(async () => {
		try {
			await navigator.clipboard.writeText(text);
			setCopied(true);
			setTimeout(() => setCopied(false), 2000);
		} catch (err) {
			console.error("Failed to copy tactical data:", err);
		}
	}, [text]);

	return (
		<button
			type="button"
			onClick={handleCopy}
			className={`group relative p-2 rounded-lg bg-white/5 border border-white/10 hover:border-cyber-primary/40 hover:bg-cyber-primary/5 transition-all duration-300 active:scale-95 ${className}`}
			aria-label={copied ? "Copied" : "Copy to clipboard"}
		>
			<AnimatePresence mode="wait" initial={false}>
				{copied ? (
					<motion.div
						key="check"
						initial={{ scale: 0.5, opacity: 0 }}
						animate={{ scale: 1, opacity: 1 }}
						exit={{ scale: 0.5, opacity: 0 }}
						transition={{ duration: 0.15 }}
						className="flex items-center gap-1.5"
					>
						<Check className="w-3.5 h-3.5 text-cyber-primary" />
						<span className="text-[10px] font-bold text-cyber-primary uppercase tracking-wider">
							Copied
						</span>
					</motion.div>
				) : (
					<motion.div
						key="copy"
						initial={{ opacity: 0 }}
						animate={{ opacity: 1 }}
						exit={{ opacity: 0 }}
						transition={{ duration: 0.15 }}
					>
						<Copy className="w-3.5 h-3.5 text-cyber-dim group-hover:text-cyber-primary transition-colors" />
					</motion.div>
				)}
			</AnimatePresence>

			{/* Premium Glow effect on hover */}
			<div className="absolute inset-0 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity bg-cyber-primary/5 blur-md -z-10" />
		</button>
	);
}
