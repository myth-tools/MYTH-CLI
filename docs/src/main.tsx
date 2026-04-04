import { StrictMode, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import { Activity, Shield, Terminal, Zap } from "lucide-react";
import App from "./App";

const bootSequence = [
	"INITIALIZING MYTH NEURAL CORE...",
	"ESTABLISHING SANDBOX ENCLAVE [BUBBLEWRAP]...",
	"MOUNTING VOLATILE MEMORY [RAM_ONLY]...",
	"SYNCHRONIZING RIG.RS ENGINE...",
	"CONNECTING NVIDIA NIM STRATEGY LAYER...",
	"ACTIVATING MISSION CONTROL HUD...",
	"READY.",
];

function NeuralBoot() {
	const [status, setStatus] = useState<string[]>([]);
	const [booted, setBooted] = useState(false);

	useEffect(() => {
		// Only run boot sequence on primary entrance
		const hasBooted = sessionStorage.getItem("myth_booted");
		if (hasBooted) {
			setBooted(true);
			return;
		}

		let current = 0;
		const interval = setInterval(() => {
			if (current < bootSequence.length) {
				setStatus((prev) => [...prev.slice(-3), bootSequence[current]]);
				current++;
			} else {
				clearInterval(interval);
				sessionStorage.setItem("myth_booted", "true");
				setTimeout(() => setBooted(true), 800);
			}
		}, 180);
		return () => {
			clearInterval(interval);
		};
	}, []);

	if (booted) {
		return null;
	}

	return (
		<div className="fixed inset-0 z-[9999] bg-[#050508] flex items-center justify-center font-mono overflow-hidden">
			{/* Grid Background */}
			<div className="absolute inset-0 bg-[url('/grid.svg')] opacity-[0.03] pointer-events-none" />

			<div className="relative w-full max-w-md p-10">
				<div className="flex items-center gap-5 mb-10">
					<div className="w-14 h-14 rounded-2xl bg-cyber-primary/10 border border-cyber-primary/30 flex items-center justify-center shadow-[0_0_40px_rgba(0,255,163,0.2)]">
						<Terminal className="w-7 h-7 text-cyber-primary" />
					</div>
					<div>
						<h1 className="text-2xl font-black text-white tracking-[0.4em] uppercase leading-none mb-1">
							MYTH_CORE
						</h1>
						<div className="text-[10px] text-cyber-primary/60 tracking-[0.2em] font-bold">
							STATION_ID: KALI_LINUX_X64
						</div>
					</div>
				</div>

				<div className="space-y-3 mb-10 h-24 flex flex-col justify-end">
					{status.map((s, i) => (
						<div
							// biome-ignore lint/suspicious/noArrayIndexKey: Linear sequence status
							key={`${s}-${i}`}
							className={`text-[11px] flex items-center gap-4 transition-all duration-300 ${i === status.length - 1 ? "text-cyber-primary translate-x-1" : "text-cyber-dim opacity-30"}`}
						>
							<span className="shrink-0 font-bold">{i === status.length - 1 ? "»" : "#"}</span>
							<span className="tracking-wider">{s}</span>
						</div>
					))}
				</div>

				<div className="relative h-1.5 bg-white/5 rounded-full overflow-hidden">
					<div className="absolute inset-y-0 left-0 bg-cyber-primary w-full origin-left animate-boot-progress shadow-[0_0_15px_rgba(0,255,163,0.5)]" />
				</div>

				<div className="mt-10 flex justify-between items-center opacity-30">
					<div className="flex items-center gap-2">
						<Shield className="w-3.5 h-3.5 text-cyber-primary" />
						<span className="text-[9px] font-black tracking-widest text-white uppercase">
							Secured
						</span>
					</div>
					<div className="flex items-center gap-2">
						<Activity className="w-3.5 h-3.5 text-cyber-secondary" />
						<span className="text-[9px] font-black tracking-widest text-white uppercase">
							Active
						</span>
					</div>
					<div className="flex items-center gap-2">
						<Zap className="w-3.5 h-3.5 text-cyber-accent" />
						<span className="text-[9px] font-black tracking-widest text-white uppercase">Pure</span>
					</div>
				</div>
			</div>
		</div>
	);
}

const rootElement = document.getElementById("root");
if (rootElement) {
	createRoot(rootElement).render(
		<StrictMode>
			<NeuralBoot />
			<App />
		</StrictMode>,
	);
}
