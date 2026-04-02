# syntax=docker/dockerfile:1.7

# ── Builder ────────────────────────────────────────────────────────────────────
FROM rust:1.88.0-bookworm AS builder

# System packages needed at build time:
#   pkg-config + libssl-dev  - OpenSSL linkage for sqlx / reqwest
#   clang                    - tree-sitter crates compile C grammars via cc crate
#   liblzma-dev              - ort-sys uses lzma-rust2 which may need liblzma
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    clang \
    liblzma-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy everything and build
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build -p remembrall-server --release

# ── Runtime ────────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# libssl3        - OpenSSL runtime (sqlx TLS)
# libstdc++6     - C++ stdlib (ONNX Runtime references)
# ca-certificates - TLS certificate bundle (HTTPS for model download)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl3 \
    libstdc++6 \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN useradd --uid 1001 --create-home --shell /sbin/nologin remembrall

COPY --from=builder --chown=remembrall:remembrall \
    /build/target/release/remembrall /usr/local/bin/remembrall

USER remembrall
WORKDIR /home/remembrall

ENTRYPOINT ["/usr/local/bin/remembrall"]
# Default: no args = MCP server over stdio
CMD []
