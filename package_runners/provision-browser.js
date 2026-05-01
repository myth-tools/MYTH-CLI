#!/usr/bin/env node
"use strict";

const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');

/**
 * MYTH CLI — Elite Browser Provisioning Logic (NPM Vector)
 *
 * Automatically retrieves the Lightpanda browser engine during `npm install`.
 * Features: Concurrency Locking, Redirect Loop Guard, Download Timeout,
 *           Atomic Deploy, Progress Tracking, and Guaranteed Lock Release.
 */

const MAX_REDIRECTS = 10;
const DOWNLOAD_TIMEOUT_MS = 90_000; // 90 seconds

async function main() {
    if (os.platform() !== 'linux') return;

    const home = os.homedir();
    const binDir = path.join(home, '.local', 'bin');
    const dest = path.join(binDir, 'lightpanda');
    const lockDir = path.join(binDir, '.lightpanda.lock');

    if (fs.existsSync(dest)) {
        console.log("⚡ MYTH: Browser engine is already synchronized and operational.");
        return;
    }

    // ─── Directory Setup ───
    if (!fs.existsSync(binDir)) fs.mkdirSync(binDir, { recursive: true });

    // ─── Concurrency Lock (Industry Standard) ───
    let lockAcquired = false;
    for (let attempt = 0; attempt < 30; attempt++) {
        try {
            fs.mkdirSync(lockDir);
            lockAcquired = true;
            break;
        } catch (e) {
            process.stdout.write(`\r⚠ MYTH: Another provisioning engine is active. Waiting... [${attempt + 1}/30]`);
            await new Promise(r => setTimeout(r, 2000));
        }
    }
    if (!lockAcquired) {
        console.error(`\n✘ MYTH: Failed to acquire system lock after 60 seconds. Aborting.`);
        process.exit(1);
    }
    console.log(""); // Newline after lock wait

    // ─── Always release lock on exit ───
    const releaseLock = () => {
        try { fs.rmSync(lockDir, { recursive: true, force: true }); } catch (e) {}
    };

    try {
        const arch = os.arch();
        const binary = arch === 'x64'
            ? "lightpanda-x86_64-linux"
            : arch === 'arm64'
                ? "lightpanda-aarch64-linux"
                : null;

        if (!binary) {
            console.warn(`⚠ MYTH: Architecture '${arch}' not supported for Lightpanda. Skipping.`);
            return;
        }

        const url = `https://github.com/lightpanda-io/browser/releases/download/nightly/${binary}`;
        console.log(`⚡ MYTH: Initiating Level-1 Autonomous Browser Engine Provisioning...`);
        console.log(`   Source: ${url}`);

        // Use PID-unique tempfile to avoid collisions on parallel installs
        const tempFile = path.join(os.tmpdir(), `lightpanda-${process.pid}-${Date.now()}.tmp`);

        await download(url, tempFile);

        // Validate download is non-empty before deploying
        const stats = fs.statSync(tempFile);
        if (stats.size < 1024) {
            fs.unlinkSync(tempFile);
            throw new Error(`Downloaded file is suspiciously small (${stats.size} bytes). Aborting.`);
        }

        // Atomic deploy: chmod then rename
        fs.chmodSync(tempFile, '755');
        fs.renameSync(tempFile, dest);

        console.log(`✔ MYTH: Browser engine deployed at: ${dest}`);

        // Advertise PATH addition if needed
        if (!(process.env.PATH || '').includes(binDir)) {
            console.log(`\n💡 MYTH: Add to your shell profile to activate: export PATH="$PATH:${binDir}"`);
        }
    } finally {
        releaseLock();
    }
}

/**
 * Downloads a URL to a local file with:
 * - Redirect following (up to MAX_REDIRECTS)
 * - Connection timeout
 * - Progress display (with or without Content-Length)
 * @param {string} url
 * @param {string} destFile
 * @param {number} redirectCount
 */
function download(url, destFile, redirectCount = 0) {
    return new Promise((resolve, reject) => {
        if (redirectCount > MAX_REDIRECTS) {
            return reject(new Error(`Too many redirects (>${MAX_REDIRECTS}). Aborting download.`));
        }

        const file = fs.createWriteStream(destFile);
        let resolved = false;

        const cleanup = (err) => {
            if (!resolved) {
                resolved = true;
                file.destroy();
                try { fs.unlinkSync(destFile); } catch (e) {}
                reject(err);
            }
        };

        const req = https.get(url, { timeout: DOWNLOAD_TIMEOUT_MS }, (response) => {
            // Handle redirects
            if (response.statusCode === 301 || response.statusCode === 302 || response.statusCode === 307 || response.statusCode === 308) {
                // Consume and release the response socket before redirecting
                response.resume();
                file.close(() => {
                    try { fs.unlinkSync(destFile); } catch (e) {}
                });
                download(response.headers.location, destFile, redirectCount + 1)
                    .then(resolve)
                    .catch(reject);
                return;
            }

            if (response.statusCode !== 200) {
                response.resume();
                return cleanup(new Error(`HTTP ${response.statusCode}: Engine synchronization failed.`));
            }

            const totalSize = parseInt(response.headers['content-length'] || '0', 10);
            let downloadedSize = 0;

            response.on('data', (chunk) => {
                downloadedSize += chunk.length;
                const mb = (downloadedSize / 1024 / 1024).toFixed(1);
                if (totalSize > 0) {
                    const percent = ((downloadedSize / totalSize) * 100).toFixed(1);
                    const totalMb = (totalSize / 1024 / 1024).toFixed(1);
                    process.stdout.write(`\r   📡 Downloading: ${percent}% [${mb}/${totalMb} MB]    `);
                } else {
                    process.stdout.write(`\r   📡 Downloading: ${mb} MB received...    `);
                }
            });

            response.on('error', cleanup);
            response.pipe(file);

            file.on('finish', () => {
                file.close(() => {
                    if (!resolved) {
                        resolved = true;
                        process.stdout.write('\n');
                        console.log(`✔ MYTH: Browser engine download complete.`);
                        resolve();
                    }
                });
            });

            file.on('error', cleanup);
        });

        req.on('timeout', () => {
            req.destroy(new Error(`Download timed out after ${DOWNLOAD_TIMEOUT_MS / 1000}s.`));
        });

        req.on('error', cleanup);
    });
}

main().catch(e => {
    console.error(`\n✘ MYTH: Browser provisioning failed: ${e.message}`);
    process.exit(1);
});
