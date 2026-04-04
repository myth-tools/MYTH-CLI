# syntax=docker/dockerfile:1.4
# ==============================================================================
# MYTH-CLI : Mission Control Docker Image
# Engine: kalilinux/kali-rolling (Official SOTA)
# 
# Security Notice: 
# MYTH enforces absolute hardware/network sandboxing natively via Linux User 
# Namespaces (Bubblewrap). To grant the neural core necessary isolation 
# capacity, the container MUST be launched with kernel capabilities:
# 
# Usage: docker run -it --rm --privileged -v $(pwd):/mission myth-cli
# ==============================================================================

# ─── Stage 1: Hermetic Rust Compilation ───
# Always build with the latest stable Rust (1.94 does not exist; avoid pinning to nonexistent tags)
FROM rust:1-slim-bookworm AS builder

# Enforce sparse registry for ultra-fast CI/CD resolution
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV DEBIAN_FRONTEND=noninteractive

# Provision minimal host build suite
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myth
# Import strictly the repository context
COPY . .

# Compile neural core with aggressive optimizations & execute binary stripping
RUN cargo build --release --locked && \
    strip target/release/myth

# ─── Stage 2: Tactical Operational Environment ───
FROM kalilinux/kali-rolling AS runtime

# Professional OpenContainers (OCI) Metadata
LABEL org.opencontainers.image.title="MYTH CLI Node"
LABEL org.opencontainers.image.description="SOTA AI-Powered Reconnaissance Agent"
LABEL org.opencontainers.image.url="https://myth.work.gd"
LABEL org.opencontainers.image.source="https://github.com/myth-tools/MYTH-CLI"
LABEL org.opencontainers.image.vendor="MYTH Operations"
LABEL org.opencontainers.image.licenses="MIT"
LABEL maintainer="Chief Architect <shesher0llms@gmail.com>"
LABEL creator.name="Shesher Hasan"
LABEL creator.role="Chief Architect"
LABEL creator.contact="shesher0llms@gmail.com"
LABEL creator.organization="myth-tools"
LABEL creator.clearance="OPERATIVE-LEVEL-4 (SENIOR ARCHITECT)"
LABEL creator.license="MYTH-INSTITUTIONAL-COMMERCIAL-2026-BETA"

ENV DEBIAN_FRONTEND=noninteractive
ENV MYTH_CONTAINER_MODE=1
ENV MYTH_TELEMETRY=0
ENV MALLOC_ARENA_MAX=2

# Provision primary tactical host arsenal
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    bubblewrap \
    ca-certificates \
    libssl3 \
    zstd \
    tor \
    nmap \
    dnsutils \
    curl \
    git \
    && apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Provision Lightpanda (Elite Browser Engine)
RUN ARCH=$(uname -m) && \
    case $ARCH in \
        x86_64)  BINARY="lightpanda-x86_64-linux" ;; \
        aarch64) BINARY="lightpanda-aarch64-linux" ;; \
        *) echo "⚠ INFO: Lightpanda not yet available for $ARCH. Skipping."; exit 0 ;; \
    esac && \
    echo "Provisioning Lightpanda: $BINARY" && \
    curl -fsSL \
        --connect-timeout 30 \
        --retry 3 \
        --retry-delay 2 \
        "https://github.com/lightpanda-io/browser/releases/download/nightly/$BINARY" \
        -o /usr/local/bin/lightpanda && \
    # Validate download is a real binary (not an HTML error page)
    FILE_SIZE=$(stat -c%s /usr/local/bin/lightpanda 2>/dev/null || echo 0) && \
    if [ "$FILE_SIZE" -lt 1024 ]; then \
        echo "⚠ WARN: Lightpanda download appears invalid (${FILE_SIZE} bytes). Removing."; \
        rm -f /usr/local/bin/lightpanda; \
    else \
        chmod +x /usr/local/bin/lightpanda && \
        echo "✔ Lightpanda operational at /usr/local/bin/lightpanda"; \
    fi

# Map operational volume for persistent intelligence reports
RUN mkdir -p /mission
WORKDIR /mission
VOLUME ["/mission"]

# Transfer optimized neural core from hermetic builder stage
COPY --from=builder /usr/src/myth/target/release/myth /usr/local/bin/myth
RUN chmod +x /usr/local/bin/myth

# Enforce direct immutable core execution
ENTRYPOINT ["myth"]
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD ["myth", "--version"] || exit 1
CMD ["--help"]
