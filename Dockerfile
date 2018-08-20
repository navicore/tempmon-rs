################################################################################
# Arguments
################################################################################
ARG rust_revision="1.28.0"

################################################################################
# Base image
################################################################################
FROM resin/armv7hf-debian as base

# use this in the arm6l vs arm7l vs arm8l issue below
RUN echo "arch is armv7hf"

ENV INITSYSTEM=on
ENV DEBIAN_FRONTEND=noninteractive

################################################################################
# Rust image
################################################################################

FROM base as rust

# Install build tools
RUN apt-get -q update && apt-get install -yq --no-install-recommends build-essential curl file pkg-config libssl-dev git

ENV PATH=/root/.cargo/bin:$PATH

RUN cp `which uname` /bin/uname-orig && echo '#!/bin/bash\nif [[ $1 == "-m" ]]; then if [[ "%%RESIN_MACHINE_NAME%%" == "raspberry-pi" ]]; then echo "armv6l"; else echo "armv7l"; fi; else /bin/uname-orig $@; fi;' > `which uname`

# Install specific version of Rust (see ARG)
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- -y --revision=${rust_revision}

################################################################################
# Builder
################################################################################

FROM rust as builder
ENV OPENSSL_DIR=/usr

# Build real app
COPY . /build/app
WORKDIR /build/app
RUN  cargo build --release

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
