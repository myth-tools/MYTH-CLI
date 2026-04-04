use crate::config::ProxyConfig;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{error, info, trace, warn};

// Global rate limiter for Tor NEWNYM (The Tor daemon enforces a ~10s internal rate limit).
// We track this to avoid spamming the daemon and causing command lag.
static LAST_NEWNYM_MS: AtomicU64 = AtomicU64::new(0);
const TOR_RATE_LIMIT_MS: u64 = 10000;

/// Sends the SIGNAL NEWNYM command to the Tor Control Port to request a new IP address.
/// Engineered for ultra-low latency and industry-grade resilience.
pub async fn rotate_ip_if_enabled(config: &ProxyConfig) {
    if !config.enabled || !config.auto_rotate {
        return;
    }

    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let last = LAST_NEWNYM_MS.load(Ordering::Acquire);

    // If we're within the cooling period, use the existing fresh circuit.
    if now_ms - last < TOR_RATE_LIMIT_MS {
        let remaining = TOR_RATE_LIMIT_MS - (now_ms - last);
        trace!(
            "Tor NEWNYM cooldown active ({}ms remaining). Reusing current dynamic circuit.",
            remaining
        );
        return;
    }

    let port = config.tor_control_port.unwrap_or(9051);
    let addr = format!("127.0.0.1:{}", port);

    // Extremely aggressive timeout (500ms) because Tor Control is local.
    // We should not block tactical tool executions if the daemon is sluggish.
    let stream_result =
        tokio::time::timeout(Duration::from_millis(500), TcpStream::connect(&addr)).await;

    let mut stream = match stream_result {
        Ok(Ok(s)) => s,
        Ok(Err(e)) => {
            warn!(error = %e, "Tor Control Port unreachable. Is the daemon running? Bypassing rotation.");
            return;
        }
        Err(_) => {
            warn!("Tor connection timeout. Bypassing rotation to maintain scan velocity.");
            return;
        }
    };

    // --- Authentication Phase ---
    let auth_cmd = if let Some(ref pass) = config.tor_control_password {
        format!("AUTHENTICATE \"{}\"\r\n", pass)
    } else {
        "AUTHENTICATE\r\n".to_string()
    };

    if let Err(e) = stream.write_all(auth_cmd.as_bytes()).await {
        error!(error = %e, "Tor AUTH packet transmission failed.");
        return;
    }

    let mut buf = [0u8; 128];
    match tokio::time::timeout(Duration::from_millis(300), stream.read(&mut buf)).await {
        Ok(Ok(n)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            if !response.starts_with("250") {
                warn!(response = %response.trim(), "Tor rejected authentication. Check your password.");
                return;
            }
        }
        _ => {
            warn!("Tor auth verification sequence timed out.");
            return;
        }
    }

    // --- NEWNYM Signal Transmission ---
    if let Err(e) = stream.write_all(b"SIGNAL NEWNYM\r\n").await {
        error!(error = %e, "Tor NEWNYM signal drop.");
        return;
    }

    match tokio::time::timeout(Duration::from_millis(300), stream.read(&mut buf)).await {
        Ok(Ok(n)) => {
            let response = String::from_utf8_lossy(&buf[..n]);
            if response.starts_with("250") {
                LAST_NEWNYM_MS.store(now_ms, Ordering::Release);
                // Print using primary theme color internally
                info!("⚡ TACTICAL IP ROTATED: Tor Circuit Reset Executed.");
            } else {
                warn!(response = %response.trim(), "Tor daemon failed to execute NEWNYM signal.");
                return;
            }
        }
        _ => {
            warn!("Tor circuit verification logic timed out.");
            return;
        }
    }

    // Sub-second stabilization delay. Tor processes NEWNYM asynchronously.
    // 350ms is highly optimal to allow circuit closure without severe pipeline blocking.
    tokio::time::sleep(Duration::from_millis(350)).await;
}
