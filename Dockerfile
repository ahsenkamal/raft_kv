FROM rust:1.92
WORKDIR /raft_kv

COPY . .

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--bin"]
