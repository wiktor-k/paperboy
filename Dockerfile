FROM --platform=$BUILDPLATFORM rust:1.70 AS rust
ARG SOURCE_DATE_EPOCH
# cross-compile using clang/llvm: https://github.com/briansmith/ring/issues/1414#issuecomment-1055177218

RUN apt-get update && apt-get -y install musl-tools clang llvm

ENV CC_aarch64_unknown_linux_musl=clang
ENV AR_aarch64_unknown_linux_musl=llvm-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
      "linux/arm64") echo aarch64-unknown-linux-musl > /target ;; \
      "linux/amd64") echo x86_64-unknown-linux-musl > /target ;; \
      *) echo Unsupported architecture && exit 1 ;; \
    esac

RUN rustup target add $(cat /target)
RUN rustup component add rustfmt clippy

WORKDIR /app

# Cache dependencies: https://benjamincongdon.me/blog/2019/12/04/Fast-Rust-Docker-Builds-with-cargo-vendor/
COPY Cargo.toml Cargo.lock ./

RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY src ./src

# Check code quality in one step
RUN cargo fmt --all -- --check && \
    cargo clippy -- -D warnings && \
    cargo test

RUN cargo build --release --target $(cat /target)

RUN cp target/$(cat /target)/release/main .

RUN sha256sum main

FROM alpine:3.18 as alpine
ARG SOURCE_DATE_EPOCH
ENV \
    # Show full backtraces for crashes.
    RUST_BACKTRACE=full
RUN apk add --no-cache \
      tini ghostscript \
    && rm -rf /var/cache/* \
    && mkdir /var/cache/apk

COPY --from=rust /app/main ./app/

# See: https://github.com/moby/buildkit/blob/master/docs/build-repro.md
# Limit the timestamp upper bound to SOURCE_DATE_EPOCH.
# Workaround for https://github.com/moby/buildkit/issues/3180
RUN find /$( ls / | grep -E -v "^(dev|mnt|proc|sys)$" ) | xargs touch -d "@${SOURCE_DATE_EPOCH}" -h || true

# Squash the entire stage for resetting the whiteout timestamps.
# Workaround for https://github.com/moby/buildkit/issues/3168
FROM scratch
COPY --from=alpine / /

ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/app/main"]

EXPOSE 80
