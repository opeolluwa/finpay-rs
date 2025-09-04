FROM rust:1.86.0-slim AS build

RUN set -e && \
    apt-get update -q && \
    apt-get install -yq \
        libgcc-s1 \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        llvm-dev \
        cmake \
        git \
        zlib1g-dev \
        build-essential && \
    rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt

WORKDIR /app

COPY . .

RUN cargo build --release

# copy native deps for extractous lib https://crates.io/crates/extractous
RUN find target/release/build -name "libtika_native.so" -exec cp {} /usr/local/lib/ \;



FROM debian:bookworm-slim AS final

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    # apt-get install -y openjdk-17-jre-headless && \
    # apt-get install -y tesseract-ocr && \
    apt-get install -y wget && \
    apt-get install -y ca-certificates && update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

    
RUN echo "deb http://deb.debian.org/debian bullseye main contrib non-free" > /etc/apt/sources.list
RUN apt-get update
RUN apt-get install -y lame
RUN lame --version 


WORKDIR /app

# copy the app
COPY --from=build /app/target/release/aers ./aers 
# copy the models and other statics files 
COPY --from=build /app/crates/aers_email_client/templates ./templates
COPY --from=build /app/resources ./resources
#copy the migrations
COPY --from=build /app/migrations ./migrations
COPY --from=build /usr/local/lib/libtika_native.so /usr/local/lib


# Downlaod the espeak files 
RUN wget https://github.com/thewh1teagle/piper-rs/releases/download/espeak-ng-files/espeak-ng-data.tar.gz

#extract the downloaded files 
RUN tar xf espeak-ng-data.tar.gz

# Set environment variable for espeak-ng data directory
RUN export PIPER_ESPEAKNG_DATA_DIRECTORY=./espeak-ng-data

# Register /usr/local/lib for runtime
RUN echo "/usr/local/lib" >> /etc/ld.so.conf.d/tika.conf && ldconfig

# Or, if you prefer env var approach:
# ENV LD_LIBRARY_PATH=/usr/local/lib

EXPOSE 8080

CMD ["./aers"]
