# 阶段 1: 构建阶段
FROM rust:1.75-slim as builder

WORKDIR /app
# 安装编译所需的依赖
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

# 阶段 2: 运行阶段
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# 从构建阶段拷贝二进制文件
COPY --from=builder /app/target/release/cpa-card-backend .
# 拷贝 init.sql 用于初始化（如果你的后端逻辑会自动执行它）
COPY --from=builder /app/init.sql .

EXPOSE 3000
CMD ["./cpa-card-backend"]