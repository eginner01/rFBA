# ==================================================
# FastAPI Best Architecture - Rust Docker 镜像
# ==================================================
# 多阶段构建，优化镜像大小
# 
# 构建命令:
# docker build -t fba-rust:latest .
#
# 运行命令:
# docker run -p 8000:8000 --env-file .env fba-rust:latest
# ==================================================

# ==================================================
# 阶段 1: 构建阶段
# ==================================================
FROM rust:1.75-slim as builder

# 设置工作目录
WORKDIR /app

# 安装编译依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 配置文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src
COPY plugins ./plugins
COPY sql ./sql

# 构建发布版本
RUN cargo build --release

# ==================================================
# 阶段 2: 运行阶段
# ==================================================
FROM debian:bookworm-slim

# 设置工作目录
WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/fastapi_best_architecture_rust /app/fba-rust

# 复制必要的文件
COPY sql ./sql
COPY .env.example ./.env.example

# 创建数据目录
RUN mkdir -p /app/data /app/logs

# 设置环境变量
ENV RUST_LOG=info
ENV DATABASE_NAME=/app/data/fba.db

# 暴露端口
EXPOSE 8000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/api/v1/health || exit 1

# 启动应用
CMD ["/app/fba-rust"]
