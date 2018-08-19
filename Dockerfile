################################################################################
# Arguments
################################################################################
ARG rust_revision="1.28.0"

################################################################################
# Base image
################################################################################

FROM debian as base

ENV INITSYSTEM=on
ENV DEBIAN_FRONTEND=noninteractive

################################################################################
# Rust image
################################################################################

FROM base as rust

# Install build tools
RUN apt-get -q update && apt-get install -yq --no-install-recommends sudo ca-certificates build-essential curl file pkg-config libssl-dev openssl autoconf bison gzip libreadline-dev patch sed zlib1g-dev
ENV PATH=/root/.cargo/bin:$PATH

RUN mkdir -p /build/openssl && curl -s https://www.openssl.org/source/openssl-1.0.2l.tar.gz | tar -C /build/openssl -xzf - && \
    cd /build/openssl/openssl-1.0.2l && \
    ./Configure \
      --openssldir=/opt/openssl/openssl-1.0.2 \
      shared linux-x86_64 && \
    make && make install_sw

# Install specific version of Rust (see ARG)
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- -y --revision=${rust_revision}

################################################################################
# Builder
################################################################################

FROM rust as builder

ENV OPENSSL_INCLUDE_DIR=/opt/openssl/openssl-1.0.2/include
ENV OPENSSL_LIB_DIR=/opt/openssl/openssl-1.0.2/lib
ENV OPENSSL_STATIC=yes

COPY . /build/app

# Build real app
WORKDIR /build/app
RUN cargo build --release

################################################################################
# Final image
################################################################################

FROM base

# Copy binary from builder image
WORKDIR /app
COPY --from=builder /build/app/target/release/tempmon-rs .
COPY --from=builder /build/app/config config/

# Launch application
CMD ["/app/tempmon-rs"]
