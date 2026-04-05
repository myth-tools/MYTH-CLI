#!/usr/bin/env node
"use strict";
/**
 * MYTH CLI — Native Execution Link (Node.js Proxy)
 * 
 * This advanced interceptor allows the MYTH neural core to be deployed dynamically 
 * via `npx` without sacrificing the strict, system-level sandboxing (Bubblewrap) 
 * required for offensive security operations.
 */

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

// Stylized output handlers respecting TTY
const isTTY = process.stdout.isTTY;
const format = {
    red: (str) => isTTY ? `\x1b[31m${str}\x1b[0m` : str,
    green: (str) => isTTY ? `\x1b[32m${str}\x1b[0m` : str,
    yellow: (str) => isTTY ? `\x1b[33m${str}\x1b[0m` : str,
    cyan: (str) => isTTY ? `\x1b[36m${str}\x1b[0m` : str,
    bold: (str) => isTTY ? `\x1b[1m${str}\x1b[0m` : str,
    dim: (str) => isTTY ? `\x1b[2m${str}\x1b[0m` : str,
};

// ─── 0. System Constraint Validation ───
if (os.platform() !== 'linux') {
    console.error(format.red(format.bold(`✘ [FATAL ARCHITECTURE ERROR]`)));
    console.error(format.red(`MYTH is a highly specialized offensive security operative.`));
    console.error(format.yellow(`It enforces strict sandboxing via Linux user namespaces (Bubblewrap).`));
    console.error(`Current OS: ${os.platform()} is NOT supported. Deploy on Kali Linux or Ubuntu.\n`);
    process.exit(1);
}

/**
 * Perform a robust resolution of the NATIVE neural core binary.
 * This filters out Node.js symlinks to prevent infinite proxy loops.
 * @param {string} cmd Binary to locate
 * @returns {string|null} Resolved path or null
 */
const getCommandPath = (cmd) => {
    try {
        // FAST-PATH: Probe standard tactical locations directly (Zero-Latency)
        const fastPaths = ['/usr/bin/myth', '/usr/local/bin/myth'];
        for (const fp of fastPaths) {
            if (fs.existsSync(fp)) {
                const stats = fs.statSync(fp);
                if (stats.isFile()) return fp;
            }
        }

        // SAFE FALLBACK: Use `which` (avoids shell injection from whereis)
        try {
            const whichOut = require('child_process')
                .execSync(`which ${JSON.stringify(cmd)} 2>/dev/null`, { stdio: 'pipe' })
                .toString().trim();
            if (whichOut && fs.existsSync(whichOut)) {
                const stats = fs.statSync(whichOut);
                if (stats.isFile()) return whichOut;
            }
        } catch (_) {}

        // PATH SCAN: Walk every directory in $PATH manually
        const pathDirs = (process.env.PATH || '').split(':').filter(Boolean);
        for (const dir of pathDirs) {
            const candidate = path.join(dir, cmd);
            // Exclude npm/nvm/bun/node paths to avoid proxy loop
            const isNodeScript = candidate.includes('node_modules') ||
                                 candidate.includes('.npm') ||
                                 candidate.includes('.nvm') ||
                                 candidate.includes('.bun');
            if (!isNodeScript && fs.existsSync(candidate)) {
                const stats = fs.statSync(candidate);
                if (stats.isFile()) return candidate;
            }
        }
        return null;
    } catch {
        return null;
    }
};

/**
 * ─── 1. Core Deployment Orchestrator ───
 * Elevates privileges securely and invokes the native bash implementation.
 */
async function executeDeployment() {
    console.log(`\n` + format.cyan(format.bold(`⚡ TACTICAL PROXY: MYTH Neural Core Offline`)));
    console.log(format.dim(`MYTH operates complex Linux sandboxes and requires host-level dependencies (Bubblewrap, Nmap, Tor).\nTo orchestrate these tools autonomously, we must first arm your environment natively.`));
    console.log(format.yellow(`\n⚠  SECURITY NOTICE: You will securely be prompted for your 'sudo' password.`));
    console.log(format.dim(`   Executing: 'curl -fsSL https://myth.work.gd/install.sh | sudo bash'\n`));

    return new Promise((resolve, reject) => {
        // Spawn remote bash installer directly with strict pipefail catching
        const proc = spawn('bash', ['-c', 'set -euo pipefail; curl -fsSL https://myth.work.gd/install.sh | sudo bash'], { 
            stdio: 'inherit'
        });

        // Register the child so top-level signal handlers can propagate to it during install
        _childProc = proc;

        proc.on('error', (err) => {
            console.error(format.red(`\n✘ [SPAWN ERROR] Failed to initialize deployment: ${err.message}`));
            reject(err);
        });

        proc.on('close', (code) => {
            if (code !== 0) {
                console.error(format.red(format.bold(`\n✘ [DEPLOYMENT FAILED] System configuration aborted.`)));
                process.exit(code || 1);
            }
            resolve();
        });
    });
}

/**
 * ─── 2. Transparent Telemetry Forwarder ───
 * Binds standard I/O and propagates OS-level signals identically to a native run.
 */
async function forwardTelemetry(binaryPath, args) {
    return new Promise((resolve, reject) => {
        const proc = spawn(binaryPath, args, {
            stdio: 'inherit',
            env: process.env
        });

        // Register the child so top-level signal handlers (SIGINT, SIGTERM, etc.)
        // can actually propagate to it. Without this assignment _childProc is always
        // null and signals are silently dropped by the handlers registered at startup.
        _childProc = proc;

        proc.on('error', (err) => {
            console.error(format.red(`\n✘ [CORE CRASH] Failed to invoke neural core: ${err.message}`));
            reject(err);
        });

        proc.on('close', (code, signal) => {
            // code is null when process was killed by a signal
            // Return 128 + signal number (POSIX convention) or 128 as generic
            if (code !== null) {
                resolve(code);
            } else {
                // Map signal name to number where possible
                const sigMap = { SIGINT: 2, SIGTERM: 15, SIGHUP: 1, SIGQUIT: 3 };
                const sigNum = (signal && sigMap[signal]) ? sigMap[signal] : 1;
                resolve(128 + sigNum);
            }
        });
    });
}

// ─── Phase Zero: Entry Point Orchestration ───

// Industry-Grade Signal Propagation (top-level, registered ONCE)
// Prevents MaxListenersExceededWarning from repeated forwardTelemetry calls
let _childProc = null;
const _signals = ['SIGINT', 'SIGTERM', 'SIGHUP', 'SIGQUIT', 'SIGUSR1', 'SIGUSR2'];
_signals.forEach((signal) => {
    process.on(signal, () => {
        if (_childProc && !_childProc.killed) {
            _childProc.kill(signal);
        }
    });
});

(async () => {
    try {
        const rawArgs = process.argv.slice(2);
        const finalArgs = rawArgs.length === 0 ? ['chat'] : rawArgs;
        
        let mythPath = getCommandPath('myth');

        // Bootstrap logic - Only show headers/logs if we are actually installing
        if (!mythPath) {
            await executeDeployment();
            
            mythPath = getCommandPath('myth');
            if (!mythPath) {
                console.error(format.red(format.bold(`\n✘ [FATAL] MYTH footprint vanished. PATH resolution failed after successful install.`)));
                console.error(format.yellow(`Try reloading your shell: "source ~/.bashrc" and execute again.`));
                process.exit(1);
            }
            
            console.log(format.green(format.bold(`\n✔ MYTH NEURAL CORE SYNCHRONIZED.`)));
            console.log(format.dim(`The 'myth' command is now natively available on your system.\n`));
        }

        // Forward raw telemetry or default to 'chat' mission
        const code = await forwardTelemetry(mythPath, finalArgs);
        process.exit(code);
    } catch (e) {
        console.error(format.red(`\n✘ [PROXY FAILURE] An unrecoverable exception occurred in the Node interceptor.`));
        console.error(format.dim(e.stack));
        process.exit(1);
    }
})();
