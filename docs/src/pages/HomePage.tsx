import { useEffect, useRef } from 'react';
import { Link } from 'react-router-dom';
import gsap from 'gsap';
import { Zap, Shield, Brain, Terminal, Database, Globe, Lock, Layers, Rocket, Activity } from 'lucide-react';
import { NAME, VERSION } from '../data/metadata';
import SystemGraph from '../components/SystemGraph';

const features = [
  { icon: <Shield className="w-5 h-5" />, title: 'Sandboxed', desc: 'Secure Bubblewrap namespaces — host OS is read-only isolation' },
  { icon: <Database className="w-5 h-5" />, title: 'Volatile', desc: 'All data stored in RAM (tmpfs) — vanishes on session exit' },
  { icon: <Zap className="w-5 h-5" />, title: 'Ultra-fast', desc: 'Native Rust binary, sub-2ms startup, minimal footprint' },
  { icon: <Brain className="w-5 h-5" />, title: 'AI-driven', desc: 'NVIDIA NIM + Rig.rs chains tools for autonomous reasoning' },
  { icon: <Globe className="w-5 h-5" />, title: 'Universal', desc: 'Connects to 3000+ Kali tools via integrated MCP bridges' },
  { icon: <Lock className="w-5 h-5" />, title: 'Secure', desc: '50+ blocked commands, shell injection & path traversal guards' },
];

const quickLinks = [
  { icon: <Rocket className="w-5 h-5" />, title: 'Quick Start', desc: 'Initialize and launch your first mission', path: '/quickstart' },
  { icon: <Terminal className="w-5 h-5" />, title: 'CLI Commands', desc: 'Deep technical command matrix', path: '/cli-commands' },
  { icon: <Activity className="w-5 h-5" />, title: 'Neural Vitals', desc: 'Live telemetry and core diagnostics', path: '/vitals' },
  { icon: <Layers className="w-5 h-5" />, title: 'Architecture', desc: 'Interactive system graph and module breakdown', path: '/architecture' },
];

export default function HomePage() {
  const heroRef = useRef<HTMLDivElement>(null);
  const titleRef = useRef<HTMLHeadingElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      gsap.from(".hero-content > *", {
        y: 40,
        opacity: 0,
        duration: 0.8,
        stagger: 0.1,
        ease: "power4.out",
        delay: 0.2
      });

      gsap.to(".title-glow", {
        opacity: 0.8,
        duration: 2,
        repeat: -1,
        yoyo: true,
        ease: "sine.inOut"
      });
    }, heroRef);

    return () => ctx.revert();
  }, []);

  return (
    <div className="-mx-6 -mt-10 overflow-hidden" ref={heroRef}>
      {/* Hero Section */}
      <section className="relative hero-gradient px-6 pt-24 pb-20 text-center border-b border-cyber-border/20 overflow-hidden scanline">
        <div className="absolute inset-0 bg-[url('https://grainy-gradients.vercel.app/noise.svg')] opacity-20 pointer-events-none" />
        <div className="hero-content relative z-10 max-w-4xl mx-auto">
          <div className="inline-flex items-center gap-2 px-3 py-1 mb-8 rounded-full bg-cyber-primary/10 border border-cyber-primary/20 text-[10px] text-cyber-primary font-bold uppercase tracking-[0.2em] animate-pulse">
            <Activity className="w-3 h-3" /> SYSTEM STATUS: OPERATIONAL V{VERSION.toUpperCase()}
          </div>
          
          <h1 className="text-6xl md:text-8xl font-bold text-white mb-6 tracking-tighter relative group" ref={titleRef}>
            <span className="relative z-10">{NAME}</span>
            <span className="absolute inset-0 text-cyber-primary/20 blur-2xl title-glow opacity-0">{NAME}</span>
            <span className="text-cyber-primary text-4xl md:text-6xl align-top ml-2 animate-pulse opacity-70">-</span>
          </h1>

          <p className="text-lg md:text-xl text-cyber-text/60 max-w-2xl mx-auto mb-10 leading-relaxed font-light">
            An ultra-fast, sandboxed, volatile AI agent leveraging <span className="text-cyber-primary font-medium">3000+ Kali tools</span> via MCP. <br className="hidden md:block" />
            <span className="text-white/80">Powered by NVIDIA NIM & Rig.rs. All tactical data vanishes on exit.</span>
          </p>

          <div className="flex justify-center gap-5 flex-wrap">
            <Link
              to="/installation"
              className="px-8 py-3 bg-cyber-primary text-cyber-bg font-bold rounded-xl hover:bg-cyber-primary/90 transition-all hover:scale-[1.02] shadow-lg shadow-cyber-primary/20"
            >
              INITIALIZE AGENT
            </Link>
            <Link
              to="/architecture"
              className="px-8 py-3 glass-panel text-white font-bold rounded-xl hover:bg-white/5 transition-all border border-white/10"
            >
              TECHNICAL DEEP DIVE
            </Link>
          </div>
        </div>
      </section>

      {/* Core Properties */}
      <section className="px-6 py-20 bg-cyber-bg">
        <div className="max-w-5xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-xs font-bold text-cyber-primary uppercase tracking-[0.3em] mb-3">Core Intelligence</h2>
            <p className="text-3xl font-bold text-white">Advanced System Properties</p>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
            {features.map((f) => (
              <div key={f.title} className="glass-panel rounded-2xl p-6 group hover:neon-border transition-all duration-500">
                <div className="w-10 h-10 rounded-lg bg-cyber-primary/10 flex items-center justify-center text-cyber-primary mb-5 group-hover:scale-110 transition-transform">
                  {f.icon}
                </div>
                <h3 className="font-bold text-white mb-2 uppercase text-xs tracking-widest">{f.title}</h3>
                <p className="text-sm text-cyber-dim leading-relaxed">{f.desc}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      {/* Architecture Preview */}
      <section className="px-6 py-20 bg-cyber-surface/30 border-y border-cyber-border/20">
        <div className="max-w-5xl mx-auto">
          <div className="grid grid-cols-1 lg:grid-cols-5 gap-12 items-center">
            <div className="lg:col-span-2">
              <h2 className="text-xs font-bold text-cyber-secondary uppercase tracking-[0.3em] mb-3">Architecture</h2>
              <p className="text-3xl font-bold text-white mb-6">Neural Interconnects</p>
              <p className="text-sm text-cyber-dim leading-relaxed mb-8">
                The MYTH ecosystem is a distributed agentic network. Using Rig.rs as the neural backbone and NVIDIA NIM for high-fidelity reasoning, the agent dispatches tasks through sandboxed MCP bridges across 3000+ local tools and remote services.
              </p>
              <div className="space-y-4">
                {['Direct NIM Integration', 'Sandboxed Execution', 'In-Memory Recall'].map(tag => (
                  <div key={tag} className="flex items-center gap-3 text-xs text-cyber-text/80 font-mono">
                    <div className="w-1.5 h-1.5 rounded-full bg-cyber-primary" />
                    {tag}
                  </div>
                ))}
              </div>
            </div>
            <div className="lg:col-span-3">
              <SystemGraph />
            </div>
          </div>
        </div>
      </section>

      {/* Documentation Matrix */}
      <section className="px-6 py-20">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-16">
            <h2 className="text-xs font-bold text-cyber-accent uppercase tracking-[0.3em] mb-3">Portal</h2>
            <p className="text-3xl font-bold text-white">Documentation Matrix</p>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
            {quickLinks.map((l) => (
              <Link key={l.path} to={l.path} className="glass-panel rounded-2xl p-6 group flex items-start gap-4 hover:bg-white/5 transition-all">
                <div className="w-12 h-12 rounded-xl bg-white/5 flex items-center justify-center text-cyber-primary group-hover:bg-cyber-primary/20 group-hover:text-cyber-bg transition-all">
                  {l.icon}
                </div>
                <div>
                  <h3 className="font-bold text-white mb-1 group-hover:text-cyber-primary transition-colors">{l.title}</h3>
                  <p className="text-xs text-cyber-dim leading-normal">{l.desc}</p>
                </div>
              </Link>
            ))}
          </div>
        </div>
      </section>

      <footer className="px-6 py-12 border-t border-cyber-border/30 text-center bg-black/40">
        <p className="text-[10px] font-mono text-cyber-dim uppercase tracking-widest mb-4">
          Licensed under MIT — Neural Core v{VERSION}
        </p>
        <p className="text-[10px] text-cyber-accent/80 font-bold uppercase tracking-widest">
          ⚠️ Authorization Required: Never scan targets without written permission.
        </p>
      </footer>
    </div>
  );
}
