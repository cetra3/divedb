FROM ubuntu:20.04 as builder

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        libssl-dev \
        pkg-config \
        curl

ENV PATH=/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --default-toolchain stable

COPY watermark.png .
COPY divedb_core divedb_core
COPY divedb_macro divedb_macro

COPY Cargo.toml .
COPY Cargo.lock .
COPY templates templates
COPY src src

RUN \
  --mount=type=cache,target=/root/.cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  --mount=type=cache,target=/target \
  cargo build --release && cp target/release/divedb /divedb


FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /divedb /usr/local/bin/divedb

EXPOSE 3333

CMD ["divedb"]