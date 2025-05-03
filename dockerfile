FROM rust:1.77 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y libx11-6 libxkbcommon0 libxcb1 libxcb-render0 libxcb-shape0 libxcb-xfixes0 libxcb-shm0 libxcb-icccm4 libxcb-image0 libxcb-keysyms1 libxcb-randr0 libxcb-render-util0 libxcb-xinerama0 libxcb-xkb1 libxrender1 libxi6 libxtst6 libgtk-3-0 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/studycue /usr/local/bin/studycue

ENTRYPOINT ["/usr/local/bin/studycue"]