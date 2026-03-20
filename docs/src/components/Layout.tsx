import { useState, useEffect, useCallback } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown, ChevronRight, Menu, X, Zap, Search, Terminal, Box, Server, Shield, BookOpen } from 'lucide-react';
import { sidebarNav, cliCommands, builtinMcpServers, builtinTools, toolBridges, reconPhases } from '../data/content';
import { NAME, VERSION, REPOSITORY_URL } from '../data/metadata';

// Site-wide Highlight Component
export const HighlightText = ({ text, highlight }: { text: string; highlight: string }) => {
  if (!highlight.trim()) return <>{text}</>;
  const parts = text.split(new RegExp(`(${highlight})`, 'gi'));
  return (
    <>
      {parts.map((part, i) => 
        part.toLowerCase() === highlight.toLowerCase() ? (
          <span key={i} className="bg-cyber-primary/30 text-cyber-primary font-bold rounded-sm px-[1px]">
            {part}
          </span>
        ) : (
          part
        )
      )}
    </>
  );
};


export function Sidebar({ mobile, onClose }: { mobile?: boolean; onClose?: () => void }) {
  const location = useLocation();
  const [expanded, setExpanded] = useState<Record<string, boolean>>(
    Object.fromEntries(sidebarNav.map((s) => [s.title, true]))
  );

  const toggle = (title: string) => setExpanded((p) => ({ ...p, [title]: !p[title] }));

  return (
    <nav className={`${mobile ? 'w-full' : 'w-64 min-w-64'} h-full glass-panel flex flex-col z-20`}>
      <div className="p-4 border-b border-cyber-border flex items-center gap-2">
        <Link to="/" className="flex items-center gap-3 group" onClick={onClose}>
          <div className="w-10 h-10 flex items-center justify-center relative premium-shimmer rounded-xl">
<svg viewBox="0 0 32 32" className="w-full h-full drop-shadow-[0_0_12px_rgba(0,255,163,0.5)] transition-all duration-700 group-hover:scale-110 group-hover:rotate-3">
              <defs>
                <linearGradient id="myth-premium-grad-ui" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stopColor="#00ffa3" />
                  <stop offset="50%" stopColor="#00d1ff" />
                  <stop offset="100%" stopColor="#7000ff" />
                </linearGradient>
              </defs>
              <path d="M16 2L29 9.5V22.5L16 30L3 22.5V9.5L16 2Z" stroke="url(#myth-premium-grad-ui)" strokeWidth="0.8" opacity="0.4" fill="url(#myth-premium-grad-ui)" fillOpacity="0.05" />
              <path d="M6 24V8L14 18L16 14L18 18L26 8V24" stroke="url(#myth-premium-grad-ui)" strokeWidth="4" strokeLinejoin="miter" strokeLinecap="square" />
            </svg>
          </div>
          <div className="flex flex-col">
            <span className="text-2xl glow-text-premium">{NAME}</span>
            <span className="text-[9px] mt-0.5 text-cyber-secondary/50 font-mono tracking-[0.3em] uppercase">Tactical Intelligence v{VERSION}</span>
          </div>
        </Link>
        {mobile && (
          <button onClick={onClose} className="ml-auto text-cyber-dim hover:text-white">
            <X className="w-5 h-5" />
          </button>
        )}
      </div>

      <div className="flex-1 overflow-y-auto py-2 px-2">
        {sidebarNav.map((section) => (
          <div key={section.title} className="mb-1">
            <button
              onClick={() => toggle(section.title)}
              className="w-full flex items-center gap-1.5 px-2 py-1.5 text-xs font-semibold uppercase tracking-wider text-cyber-dim hover:text-cyber-primary transition-colors"
            >
              {expanded[section.title] ? <ChevronDown className="w-3 h-3" /> : <ChevronRight className="w-3 h-3" />}
              {section.title}
            </button>
            <AnimatePresence initial={false}>
              {expanded[section.title] && (
                <motion.div
                  initial={{ height: 0, opacity: 0 }}
                  animate={{ height: 'auto', opacity: 1 }}
                  exit={{ height: 0, opacity: 0 }}
                  transition={{ duration: 0.2 }}
                  className="overflow-hidden"
                >
                  {section.items.map((item) => {
                    const isActive = location.pathname === item.path || (item.path !== '/' && location.pathname.startsWith(item.path));
                    return (
                      <Link
                        key={item.path}
                        to={item.path}
                        onClick={onClose}
                        className={`block pl-6 pr-2 py-1.5 text-sm rounded-md mx-1 mb-0.5 transition-all ${
                          isActive
                            ? 'sidebar-link-active font-medium'
                            : 'text-cyber-text/70 hover:text-white hover:bg-white/5'
                        }`}
                      >
                        {item.title}
                      </Link>
                    );
                  })}
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        ))}
      </div>

      <div className="p-3 border-t border-cyber-border">
        <a
          href={REPOSITORY_URL}
          target="_blank"
          rel="noopener noreferrer"
          className="text-xs text-cyber-dim hover:text-cyber-primary transition-colors flex items-center gap-1"
        >
          GitHub →
        </a>
      </div>
    </nav>
  );
}

export function Layout({ children }: { children: React.ReactNode }) {
  const [mobileOpen, setMobileOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const location = useLocation();
  const navigate = useNavigate();

  // Unified Search Index
  const searchIndex = [
    ...sidebarNav.flatMap(s => s.items.map(p => ({ title: p.title, path: p.path, desc: '', type: 'Page', icon: <BookOpen className="w-3.5 h-3.5" /> }))),
    ...cliCommands.map(c => ({ title: `myth ${c.name}`, path: `/cli-commands#${c.name}`, desc: c.description, type: 'Command', icon: <Terminal className="w-3.5 h-3.5" /> })),
    ...builtinTools.map(t => ({ title: t.name, path: '/builtin-tools', desc: t.description, type: 'Tool', icon: <Box className="w-3.5 h-3.5" /> })),
    ...builtinMcpServers.map(s => ({ title: s.name, path: '/mcp-servers', desc: s.description, type: 'Server', icon: <Server className="w-3.5 h-3.5" /> })),
    ...toolBridges.map(b => ({ title: b.name, path: '/tool-bridges', desc: b.description, type: 'Bridge', icon: <Zap className="w-3.5 h-3.5" /> })),
    ...reconPhases.map(p => ({ title: p.name, path: '/profiles', desc: p.description, type: 'Phase', icon: <Shield className="w-3.5 h-3.5" /> })),
  ];


  const filtered = searchQuery.trim()
    ? searchIndex.filter(item => 
        item.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
        (item.desc?.toLowerCase().includes(searchQuery.toLowerCase()))
      ).slice(0, 10)
    : [];

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === 'k' && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setSearchOpen(open => !open);
      }
      if (e.key === '/') {
        if (document.activeElement?.tagName === 'INPUT' || document.activeElement?.tagName === 'TEXTAREA') return;
        e.preventDefault();
        setSearchOpen(true);
      }
    };
    document.addEventListener('keydown', down);
    return () => document.removeEventListener('keydown', down);
  }, []);

  useEffect(() => {
    setSelectedIndex(0);
  }, [searchQuery]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex(i => (i + 1) % filtered.length);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex(i => (i - 1 + filtered.length) % filtered.length);
    } else if (e.key === 'Enter' && filtered[selectedIndex]) {
      e.preventDefault();
      navigate(filtered[selectedIndex].path);
      setSearchOpen(false);
      setSearchQuery('');
    } else if (e.key === 'Escape') {
      setSearchOpen(false);
    }
  }, [filtered, selectedIndex, navigate]);


  return (
    <div className="flex h-screen overflow-hidden">
      {/* Desktop sidebar */}
      <div className="sidebar-desktop hidden md:block">
        <Sidebar />
      </div>

      {/* Mobile sidebar overlay */}
      <AnimatePresence>
        {mobileOpen && (
          <>
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="fixed inset-0 bg-black/60 z-40 md:hidden"
              onClick={() => setMobileOpen(false)}
            />
            <motion.div
              initial={{ x: -280 }}
              animate={{ x: 0 }}
              exit={{ x: -280 }}
              transition={{ type: 'spring', damping: 25, stiffness: 300 }}
              className="fixed left-0 top-0 bottom-0 z-50 md:hidden"
            >
              <Sidebar mobile onClose={() => setMobileOpen(false)} />
            </motion.div>
          </>
        )}
      </AnimatePresence>

      {/* Main content area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Top bar */}
        <header className="h-14 border-b border-cyber-border/30 bg-cyber-bg/40 backdrop-blur-xl flex items-center px-4 gap-3 shrink-0 z-10 scanline">
          <button
            onClick={() => setMobileOpen(true)}
            className="md:hidden text-cyber-dim hover:text-white"
          >
            <Menu className="w-5 h-5" />
          </button>

          <div className="flex-1 flex items-center gap-3">
            <button
              onClick={() => setSearchOpen(!searchOpen)}
              className="flex items-center gap-2 px-3 py-1.5 text-sm text-cyber-dim bg-cyber-bg border border-cyber-border rounded-lg hover:border-cyber-primary/40 transition-colors w-full max-w-xs"
            >
              <Search className="w-3.5 h-3.5" />
              <span>Search docs…</span>
              <kbd className="ml-auto text-[10px] px-1.5 py-0.5 bg-cyber-surface rounded border border-cyber-border">⌘K</kbd>
            </button>
          </div>

          <a
            href={REPOSITORY_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="text-xs text-cyber-dim hover:text-cyber-primary transition-colors hidden sm:block"
          >
            GitHub
          </a>
        </header>

        {/* Search overlay */}
        <AnimatePresence>
          {searchOpen && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="fixed inset-0 bg-cyber-bg/80 backdrop-blur-md z-[100] flex items-start justify-center pt-24 px-4"
              onClick={() => setSearchOpen(false)}
            >
              <motion.div
                initial={{ y: -20, scale: 0.98, opacity: 0 }}
                animate={{ y: 0, scale: 1, opacity: 1 }}
                exit={{ y: -20, scale: 0.98, opacity: 0 }}
                className="bg-cyber-surface/90 border border-cyber-primary/20 rounded-2xl w-full max-w-2xl shadow-[0_0_50px_-12px_rgba(0,255,157,0.15)] overflow-hidden"
                onClick={(e) => e.stopPropagation()}
              >
                <div className="flex items-center px-5 border-b border-cyber-border/50 bg-white/5">
                  <Search className="w-5 h-5 text-cyber-primary animate-pulse" />
                  <input
                    autoFocus
                    value={searchQuery}
                    onChange={(e) => setSearchQuery(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Search anything: commands, tools, servers, methodology..."
                    className="flex-1 bg-transparent py-5 px-4 text-base text-white outline-none placeholder-cyber-dim font-light"
                  />
                  <div className="flex items-center gap-1.5 px-2 py-1 rounded bg-black/40 border border-cyber-border/50 text-[10px] text-cyber-dim font-mono">
                    ESC
                  </div>
                </div>

                <div className="max-h-[60vh] overflow-y-auto custom-scrollbar p-2">
                  {!searchQuery ? (
                    <div className="p-8 text-center">
                      <Zap className="w-8 h-8 text-cyber-dim/30 mx-auto mb-3" />
                      <p className="text-sm text-cyber-dim uppercase tracking-widest font-bold">Quantum Search Ready</p>
                      <p className="text-xs text-cyber-dim/60 mt-2">Search across all 3,000+ Kali tools and MYTH protocols</p>
                    </div>
                  ) : filtered.length === 0 ? (
                    <div className="p-8 text-center">
                      <p className="text-sm text-cyber-dim">No matching tactical data found for "<span className="text-cyber-primary">{searchQuery}</span>"</p>
                    </div>
                  ) : (
                    <div className="space-y-1">
                      {filtered.map((item, i) => (
                        <Link
                          key={`${item.type}-${item.title}-${i}`}
                          to={item.path}
                          onClick={() => { setSearchOpen(false); setSearchQuery(''); }}
                          className={`flex items-center gap-4 px-4 py-3 rounded-xl transition-all border ${
                            i === selectedIndex 
                              ? 'bg-cyber-primary/10 border-cyber-primary/30 translate-x-1' 
                              : 'bg-transparent border-transparent hover:bg-white/5'
                          }`}
                        >
                          <div className={`p-2 rounded-lg ${i === selectedIndex ? 'bg-cyber-primary/20 text-cyber-primary' : 'bg-cyber-surface text-cyber-dim'} transition-colors`}>
                            {item.icon}
                          </div>
                          <div className="flex-1 min-w-0">
                            <div className="flex items-center gap-2 mb-0.5">
                              <span className="text-sm font-bold text-white tracking-tight leading-none">
                                <HighlightText text={item.title} highlight={searchQuery} />
                              </span>
                              <span className="text-[10px] uppercase tracking-tighter px-1.5 py-0.5 rounded bg-black/40 text-cyber-dim border border-cyber-border font-bold">
                                {item.type}
                              </span>
                            </div>
                            {item.desc && (
                              <p className="text-xs text-cyber-dim font-light truncate">
                                <HighlightText text={item.desc} highlight={searchQuery} />
                              </p>
                            )}
                          </div>
                          {i === selectedIndex && (
                            <ChevronRight className="w-4 h-4 text-cyber-primary shrink-0" />
                          )}
                        </Link>
                      ))}
                    </div>
                  )}
                </div>

                <div className="px-5 py-3 border-t border-cyber-border/50 bg-black/20 flex items-center justify-between text-[10px] text-cyber-dim font-mono uppercase tracking-widest">
                  <div className="flex gap-4">
                    <span className="flex items-center gap-1"><span className="text-cyber-primary">↑↓</span> Navigate</span>
                    <span className="flex items-center gap-1"><span className="text-cyber-primary">Enter</span> Select</span>
                  </div>
                  <div>
                    {filtered.length} Results
                  </div>
                </div>
              </motion.div>
            </motion.div>
          )}

        </AnimatePresence>

        {/* Page content */}
        <main className="flex-1 overflow-y-auto">
          <motion.div
            key={location.pathname}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.25 }}
            className="max-w-4xl mx-auto px-6 py-10"
          >
            {children}
          </motion.div>
        </main>
      </div>
    </div>
  );
}

export function CodeBlock({ code, lang = 'bash', title }: { code: string; lang?: string; title?: string }) {
  const [copied, setCopied] = useState(false);

  const copy = () => {
    navigator.clipboard.writeText(code).catch((err) => {
      console.warn('Clipboard copy failed:', err);
    });
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="code-block my-4">
      {title && (
        <div className="code-header">
          <span>{title}</span>
          <button onClick={copy} className="text-cyber-dim hover:text-cyber-primary transition-colors text-xs">
            {copied ? '✓ Copied' : 'Copy'}
          </button>
        </div>
      )}
      {!title && (
        <div className="code-header">
          <span>{lang}</span>
          <button onClick={copy} className="text-cyber-dim hover:text-cyber-primary transition-colors text-xs">
            {copied ? '✓ Copied' : 'Copy'}
          </button>
        </div>
      )}
      <pre className="text-cyber-text/90"><code>{code}</code></pre>
    </div>
  );
}

export function PageHeader({ title, description, badge }: { title: string; description: string; badge?: string }) {
  return (
    <div className="mb-8">
      <div className="flex items-center gap-3 mb-2">
        {badge && (
          <span className="px-2 py-0.5 text-[10px] font-mono font-bold uppercase tracking-wider bg-cyber-primary/15 text-cyber-primary border border-cyber-primary/30 rounded">
            {badge}
          </span>
        )}
      </div>
      <h1 className="text-3xl font-bold text-white mb-3">{title}</h1>
      <p className="text-cyber-dim text-lg leading-relaxed">{description}</p>
      <div className="h-px bg-gradient-to-r from-cyber-primary/40 via-cyber-border to-transparent mt-6" />
    </div>
  );
}

export function InfoCard({ title, children, icon }: { title: string; children: React.ReactNode; icon?: React.ReactNode }) {
  return (
    <div className="glass-panel rounded-xl p-5 mb-4 relative overflow-hidden group hover:neon-border transition-all duration-500">
      <div className="absolute top-0 right-0 w-16 h-16 bg-cyber-primary/5 blur-2xl group-hover:bg-cyber-primary/10 transition-colors" />
      <div className="flex items-center gap-2 mb-3">
        {icon && <span className="text-cyber-primary animate-pulse">{icon}</span>}
        <h3 className="font-semibold text-white tracking-tight uppercase text-xs">{title}</h3>
      </div>
      <div className="text-sm text-cyber-text/70 leading-relaxed">{children}</div>
    </div>
  );
}
