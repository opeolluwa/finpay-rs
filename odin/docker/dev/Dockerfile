FROM rust:1.90.0

ENV CARGO_TARGET_DIR=/cargo-target 

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq \
        libgcc-s1 \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        zlib1g-dev \
        llvm-dev \
        cmake \
        git \
        lame \
        build-essential && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

RUN apt-get install -y lame

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

RUN cargo build 

VOLUME ["/app", "/cargo-target", "/usr/local/cargo/registry"]
CMD ["cargo", "watch", \
     "--why", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-w", "migrations", \
     "-x", "check", \
     "-x", "run"]
