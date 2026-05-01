//! Lightpanda MCP Server — High-Performance Browser Automation.
//!
//! Lightpanda is a headless browser built from scratch in Zig, designed specifically
//! for AI agents. It offers 11x faster execution and 9x less memory consumption
//! than traditional Chromium-based solutions.
//!
//! This integration provides a robust bridge for JS-heavy reconnaissance and
//! autonomous web navigation.

use crate::config::{CustomMcpServer, LocalMcpConfig, McpTransport};

pub fn get_config() -> CustomMcpServer {
    CustomMcpServer::Local(LocalMcpConfig {
        enabled: true,
        command: "lightpanda".to_string(),
        args: vec![
            "mcp".to_string(),
            "--log-level".to_string(),
            "error".to_string(),
        ],
        env: crate::mcp_env! {
            "LIGHTPANDA_DISABLE_TELEMETRY" => "true"
        },
        description: Some(
            "Lightpanda — PRIMARY High-performance browser engine for ultra-fast, zero-latency web search and tactical reconnaissance. Always prioritized for information retrieval.".to_string(),
        ),
        working_dir: None,
        timeout: 300, // Elite-level timeout for deep scanning
        allowed_tools: vec![],
        transport: McpTransport::Stdio,
        install_script: Some(
            r#"if ! command -v lightpanda > /dev/null 2>&1; then
    echo "MYTH: Initializing Autonomous Provisioning Protocol..."
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)  BINARY="lightpanda-x86_64-linux" ;;
        aarch64) BINARY="lightpanda-aarch64-linux" ;;
        *) echo "MYTH: [ERROR] Unsupported architecture: $ARCH"; exit 1 ;;
    esac

    # Determine install directory: $PREFIX/bin on Termux, ~/.local/bin elsewhere
    if [ -n "${PREFIX:-}" ] && echo "${PREFIX}" | grep -q "com.termux"; then
        INSTALL_DIR="$PREFIX/bin"
    else
        INSTALL_DIR="$HOME/.local/bin"
    fi
    mkdir -p "$INSTALL_DIR"

    TEMP_FILE="${TMPDIR:-/tmp}/lightpanda_$$.tmp"
    URL="https://github.com/lightpanda-io/browser/releases/download/nightly/$BINARY"
    echo "MYTH: Synchronizing browser engine from $URL..."
    if curl -fsSL --connect-timeout 15 --max-time 120 "$URL" -o "$TEMP_FILE"; then
        chmod +x "$TEMP_FILE"
        mv "$TEMP_FILE" "$INSTALL_DIR/lightpanda"
        echo "MYTH: Engine lock established at $INSTALL_DIR/lightpanda"
        # Ensure it's reachable for the upcoming check
        export PATH="$INSTALL_DIR:$PATH"
    else
        rm -f "$TEMP_FILE" 2>/dev/null || true
        echo "MYTH: [ERROR] Synchronization failed. Network uplink unstable?"
        exit 1
    fi
fi"#.to_string(),
        ),
    })
}
