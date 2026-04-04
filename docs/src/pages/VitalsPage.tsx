import {
	Area,
	AreaChart,
	Bar,
	BarChart,
	CartesianGrid,
	Cell,
	ResponsiveContainer,
	Tooltip,
	XAxis,
	YAxis,
} from "recharts";
import { PageHeader } from "../components/Layout";

const latencyData = [
	{ time: "0s", ms: 120 },
	{ time: "5s", ms: 450 },
	{ time: "10s", ms: 210 },
	{ time: "15s", ms: 380 },
	{ time: "20s", ms: 140 },
	{ time: "25s", ms: 290 },
	{ time: "30s", ms: 180 },
];

const recallData = [
	{ phase: "0", accuracy: 98 },
	{ phase: "2", accuracy: 95 },
	{ phase: "4", accuracy: 99 },
	{ phase: "6", accuracy: 92 },
	{ phase: "8", accuracy: 96 },
	{ phase: "10", accuracy: 97 },
	{ phase: "12", accuracy: 94 },
];

const coreMetrics = [
	{
		label: "Session Uptime",
		value: "48.2h",
		color: "text-cyber-primary",
		desc: "Time since last myth init",
	},
	{
		label: "Memory Usage",
		value: "142MB",
		color: "text-cyber-secondary",
		desc: "Agent + Qdrant in-process RSS",
	},
	{
		label: "Tool Success Rate",
		value: "99.8%",
		color: "text-cyber-primary",
		desc: "Non-timeout tool exits / total calls",
	},
	{
		label: "Browser Status",
		value: "VERIFIED",
		color: "text-cyber-accent",
		desc: "Lightpanda engine detected and operational",
	},
	{
		label: "MCP Servers",
		value: "11 / 11",
		color: "text-cyber-success",
		desc: "Active MCP connections",
	},
	{
		label: "Vector Entries",
		value: "2,847",
		color: "text-purple-400",
		desc: "In-memory Qdrant records (session)",
	},
	{
		label: "LLM Model",
		value: "DeepSeek R1",
		color: "text-cyber-secondary",
		desc: "Active NVIDIA NIM inference model",
	},
	{
		label: "Sandbox",
		value: "ACTIVE",
		color: "text-cyber-primary",
		desc: "Bubblewrap namespace isolation enabled",
	},
];

export default function VitalsPage() {
	return (
		<div className="space-y-8 pb-12">
			<PageHeader
				title="Neural Vitals"
				description="Real-time telemetry and performance metrics of the MYTH agentic core. Data shown is representative of a typical active session."
				badge="Telemetry"
			/>

			{/* Note about representative data */}
			<div className="glass-panel rounded-xl p-4 border border-cyber-warning/20 text-xs text-cyber-warning/80">
				<span className="font-bold text-cyber-warning">📊 Note:</span> Charts display representative
				data from a benchmark session. Live metrics are visible inside the TUI via{" "}
				<code className="text-cyber-primary">myth vitals</code> or during an active session.
			</div>

			{/* Charts */}
			<div className="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div className="glass-panel rounded-2xl p-6 relative overflow-hidden">
					<div className="absolute top-0 left-0 w-1 h-full bg-cyber-primary" />
					<h3 className="text-sm font-bold text-white mb-2 uppercase tracking-widest flex items-center gap-2">
						<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
						LLM Round-Trip Latency (ms)
					</h3>
					<p className="text-[10px] text-cyber-dim mb-4">
						Time from tool-call dispatch to response receipt — includes NIM inference + network
					</p>
					<div className="h-[220px] w-full">
						<ResponsiveContainer width="100%" height="100%">
							<AreaChart data={latencyData}>
								<defs>
									<linearGradient id="colorMs" x1="0" y1="0" x2="0" y2="1">
										<stop offset="5%" stopColor="#00ff88" stopOpacity={0.3} />
										<stop offset="95%" stopColor="#00ff88" stopOpacity={0} />
									</linearGradient>
								</defs>
								<CartesianGrid strokeDasharray="3 3" stroke="#2a2a3e" vertical={false} />
								<XAxis
									dataKey="time"
									stroke="#666680"
									fontSize={10}
									axisLine={false}
									tickLine={false}
								/>
								<YAxis stroke="#666680" fontSize={10} axisLine={false} tickLine={false} unit="ms" />
								<Tooltip
									contentStyle={{
										background: "#0d0d14",
										border: "1px solid #2a2a3e",
										borderRadius: "8px",
									}}
									itemStyle={{ color: "#00ff88" }}
								/>
								<Area
									type="monotone"
									dataKey="ms"
									stroke="#00ff88"
									fillOpacity={1}
									fill="url(#colorMs)"
									strokeWidth={2}
								/>
							</AreaChart>
						</ResponsiveContainer>
					</div>
				</div>

				<div className="glass-panel rounded-2xl p-6 relative overflow-hidden">
					<div className="absolute top-0 left-0 w-1 h-full bg-cyber-secondary" />
					<h3 className="text-sm font-bold text-white mb-2 uppercase tracking-widest flex items-center gap-2">
						<span className="w-2 h-2 rounded-full bg-cyber-secondary animate-pulse" />
						Semantic Memory Recall Accuracy (%)
					</h3>
					<p className="text-[10px] text-cyber-dim mb-4">
						Cosine similarity score of recalled memories vs. exact match — per recon phase
					</p>
					<div className="h-[220px] w-full">
						<ResponsiveContainer width="100%" height="100%">
							<BarChart data={recallData}>
								<CartesianGrid strokeDasharray="3 3" stroke="#2a2a3e" vertical={false} />
								<XAxis
									dataKey="phase"
									stroke="#666680"
									fontSize={10}
									axisLine={false}
									tickLine={false}
									label={{
										value: "Phase",
										position: "insideBottom",
										offset: -2,
										fill: "#666680",
										fontSize: 10,
									}}
								/>
								<YAxis
									stroke="#666680"
									fontSize={10}
									axisLine={false}
									tickLine={false}
									domain={[80, 100]}
									unit="%"
								/>
								<Tooltip
									contentStyle={{
										background: "#0d0d14",
										border: "1px solid #2a2a3e",
										borderRadius: "8px",
									}}
								/>
								<Bar dataKey="accuracy" radius={[4, 4, 0, 0]}>
									{recallData.map((item, index) => (
										<Cell
											key={`cell-${item.phase}`}
											fill={index % 2 === 0 ? "#0088ff" : "#00ff88"}
										/>
									))}
								</Bar>
							</BarChart>
						</ResponsiveContainer>
					</div>
				</div>
			</div>

			{/* Core Integrity Metrics — 8 tiles */}
			<div className="glass-panel rounded-2xl p-6">
				<h3 className="text-sm font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
					<span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
					Core Integrity Metrics
				</h3>
				<div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
					{coreMetrics.map((m) => (
						<div
							key={m.label}
							className="p-4 rounded-xl bg-white/5 border border-white/5 group hover:border-cyber-primary/20 transition-colors"
						>
							<p className="text-[10px] text-cyber-dim uppercase font-bold tracking-wider mb-1">
								{m.label}
							</p>
							<p className={`text-lg font-bold font-mono ${m.color} mb-1`}>{m.value}</p>
							<p className="text-[9px] text-cyber-dim/60 leading-relaxed">{m.desc}</p>
						</div>
					))}
				</div>
			</div>

			{/* CLI access */}
			<div className="glass-panel rounded-xl p-5 border border-cyber-border/30">
				<h3 className="text-sm font-bold text-white mb-3 uppercase tracking-wider">
					Access Live Vitals
				</h3>
				<div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
					<div>
						<p className="text-xs text-cyber-dim mb-2">From CLI:</p>
						<code className="text-sm font-mono text-cyber-primary">myth vitals</code>
					</div>
					<div>
						<p className="text-xs text-cyber-dim mb-2">From interactive session:</p>
						<code className="text-sm font-mono text-cyber-primary">/vitals</code> or{" "}
						<code className="text-sm font-mono text-cyber-secondary">myth status</code>
					</div>
				</div>
			</div>
		</div>
	);
}
