# 使用 nightly 镜像以支持 1.88+ 的依赖需求
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app
# 安装编译所需的依赖
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
# 即使有 Cargo.lock 也会尝试编译，Nightly 可以处理 Edition 2024
RUN cargo build --release

# 阶段 2: 运行阶段
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# 注意：确保二进制文件名与你的 Cargo.toml 中的 name 一致
COPY --from=builder /app/target/release/cpa-card-backend .
COPY --from=builder /app/init.sql .

EXPOSE 3000
CMD ["./cpa-card-backend"]
