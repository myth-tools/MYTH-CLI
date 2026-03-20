import { PageHeader } from '../components/Layout';
import { 
  AreaChart, Area, BarChart, Bar, Cell, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer 
} from 'recharts';

const latencyData = [
  { time: '0s', ms: 120 },
  { time: '5s', ms: 450 },
  { time: '10s', ms: 210 },
  { time: '15s', ms: 380 },
  { time: '20s', ms: 140 },
  { time: '25s', ms: 290 },
  { time: '30s', ms: 180 },
];

const recallData = [
  { phase: '0', accuracy: 98 },
  { phase: '2', accuracy: 95 },
  { phase: '4', accuracy: 99 },
  { phase: '6', accuracy: 92 },
  { phase: '8', accuracy: 96 },
  { phase: '10', accuracy: 97 },
  { phase: '12', accuracy: 94 },
];

export default function VitalsPage() {
  return (
    <div className="space-y-8 pb-12">
      <PageHeader title="Neural Vitals" description="Real-time telemetry and performance clusters of the MYTH agentic core." badge="Telemetry" />

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass-panel rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 left-0 w-1 h-full bg-cyber-primary" />
          <h3 className="text-sm font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-cyber-primary animate-pulse" />
            Neural Latency (ms)
          </h3>
          <div className="h-[250px] w-full">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={latencyData}>
                <defs>
                  <linearGradient id="colorMs" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#00ff88" stopOpacity={0.3}/>
                    <stop offset="95%" stopColor="#00ff88" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#2a2a3e" vertical={false} />
                <XAxis dataKey="time" stroke="#666680" fontSize={10} axisLine={false} tickLine={false} />
                <YAxis stroke="#666680" fontSize={10} axisLine={false} tickLine={false} />
                <Tooltip 
                  contentStyle={{ background: '#0d0d14', border: '1px solid #2a2a3e', borderRadius: '8px' }}
                  itemStyle={{ color: '#00ff88' }}
                />
                <Area type="monotone" dataKey="ms" stroke="#00ff88" fillOpacity={1} fill="url(#colorMs)" strokeWidth={2} />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        <div className="glass-panel rounded-2xl p-6 relative overflow-hidden">
          <div className="absolute top-0 left-0 w-1 h-full bg-cyber-secondary" />
          <h3 className="text-sm font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-cyber-secondary animate-pulse" />
            Semantic Recall Accuracy (%)
          </h3>
          <div className="h-[250px] w-full">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={recallData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#2a2a3e" vertical={false} />
                <XAxis dataKey="phase" stroke="#666680" fontSize={10} axisLine={false} tickLine={false} />
                <YAxis stroke="#666680" fontSize={10} axisLine={false} tickLine={false} domain={[80, 100]} />
                <Tooltip 
                  contentStyle={{ background: '#0d0d14', border: '1px solid #2a2a3e', borderRadius: '8px' }}
                />
                <Bar dataKey="accuracy" radius={[4, 4, 0, 0]}>
                  {recallData.map((_, index) => (
                    <Cell key={`cell-${index}`} fill={index % 2 === 0 ? '#0088ff' : '#00ff88'} />
                  ))}
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      <div className="glass-panel rounded-2xl p-6">
        <h3 className="text-sm font-bold text-white mb-4 uppercase tracking-widest">Core Integrity Metrics</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {[
            { label: 'Uptime', value: '48.2h', color: 'text-cyber-primary' },
            { label: 'Mem Usage', value: '142MB', color: 'text-cyber-secondary' },
            { label: 'Tool Success', value: '99.8%', color: 'text-cyber-primary' },
            { label: 'Sandbox Load', value: '0.12', color: 'text-cyber-accent' },
          ].map((m) => (
            <div key={m.label} className="p-4 rounded-xl bg-white/5 border border-white/5">
              <p className="text-[10px] text-cyber-dim uppercase font-bold mb-1">{m.label}</p>
              <p className={`text-xl font-bold font-mono ${m.color}`}>{m.value}</p>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
