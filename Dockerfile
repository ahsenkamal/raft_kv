FROM rust:1.92
WORKDIR /raft_kv

COPY . .

RUN cargo build

ENTRYPOINT ["cargo", "run", "--bin", "node", "--"]
