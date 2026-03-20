import { Handle, Position } from 'reactflow';

export const CyberNode = ({ data, selected }: any) => {
  const borderColor = data.borderColor || 'rgba(0, 255, 136, 0.3)';
  const glowColor = data.glowColor || 'rgba(0, 255, 136, 0.2)';

  return (
    <div className={`
      relative group transition-all duration-500
      ${selected ? 'scale-110' : 'scale-100'}
    `}>
      {/* Dynamic Glow */}
      <div 
        className="absolute -inset-2 rounded-2xl blur-xl opacity-0 group-hover:opacity-40 transition-opacity duration-500"
        style={{ background: glowColor }}
      />
      
      <div 
        className="relative px-6 py-4 rounded-xl border bg-black/80 backdrop-blur-xl shadow-2xl min-w-[190px]"
        style={{ borderColor: borderColor }}
      >

        <div className="flex flex-col items-center gap-1.5">
          {data.icon && (
            <div className="text-cyber-primary mb-2 transform group-hover:scale-110 transition-transform duration-300">
              {data.icon}
            </div>
          )}
          
          <div className="text-[9px] uppercase tracking-[0.2em] text-cyber-dim font-black">
            {data.sublabel || 'Component'}
          </div>
          
          <div className="text-xs font-bold text-white tracking-wider uppercase text-center">
            {data.label}
          </div>

          {data.details && (
            <div className="mt-2 pt-2 border-t border-white/5 w-full flex flex-col gap-1">
              {data.details.split('\n').map((line: string, i: number) => (
                <div key={i} className="text-[8px] text-cyber-dim/60 font-mono text-center leading-tight">
                  {line}
                </div>
              ))}
            </div>
          )}
        </div>
        
        <Handle 
          type="target" 
          position={Position.Top} 
          className="!bg-cyber-primary !border-cyber-bg !w-1.5 !h-1.5 !static !mx-auto !mt-[-21px]" 
        />
        <Handle 
          type="source" 
          position={Position.Bottom} 
          className="!bg-cyber-primary !border-cyber-bg !w-1.5 !h-1.5 !static !mx-auto !mb-[-21px]" 
        />
      </div>
    </div>
  );
};
