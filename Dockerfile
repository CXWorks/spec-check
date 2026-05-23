FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=Etc/UTC

# ===== 基础依赖 =====
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    wget \
    ca-certificates \
    pkg-config \
    libssl-dev \
    libpoppler-dev \
    poppler-utils \
    python3 \
    python3-pip \
    python3-venv \
    unzip \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*

# ===== 安装 Rust（Verus 需要）=====
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# ===== 安装 Python 依赖（RAG 检索） =====
WORKDIR /tmp
COPY prompt_engineering/requirements.txt /tmp/requirements.txt
RUN python3 -m pip install --no-cache-dir --upgrade pip setuptools wheel
RUN python3 -m pip install --no-cache-dir -r /tmp/requirements.txt

# ===== 安装 Verus =====
WORKDIR /opt
RUN git clone https://github.com/verus-lang/verus.git
WORKDIR /opt/verus

# 固定版本
RUN git reset --hard bec74a67d9281a4f51a7e1855760c5d16d8f63ff

# 安装 Z3
WORKDIR /opt/verus/source
RUN ./tools/get-z3.sh

# 构建 Verus
RUN bash -c "source ../tools/activate && vargo build --release"

# ===== 环境变量 =====
ENV VERUS_ROOT=/opt/verus
ENV PATH="/opt/verus/source/target-verus/release:${PATH}"

# ===== 拷贝你的项目 =====
WORKDIR /app
COPY . /app

# ===== 默认启动 =====
CMD ["/bin/bash"]
