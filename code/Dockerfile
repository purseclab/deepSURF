# syntax=docker/dockerfile:1
FROM ubuntu:22.04 AS base

ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC LC_ALL=C.UTF-8 LANG=C.UTF-8
SHELL ["/bin/bash", "-lc"]

# --- packages ---
RUN apt-get update -y && \
    apt-get -y install --no-install-recommends \
      build-essential clang cmake make automake meson ninja-build bison flex \
      pkg-config libtool libtool-bin \
      curl wget xz-utils bzip2 gnupg lsb-release software-properties-common \
      sudo dialog tmux strace psmisc \
      git jupp nano bash-completion less vim joe \
      ssh apt-transport-https \
      python3 python3-dev python3-pip python-is-python3 \
      libssl-dev libglib2.0-dev \
      unzip && \
    rm -rf /var/lib/apt/lists/*

# --- LLVM 18 ---
RUN apt-get update -y && \
    apt-get -y install --no-install-recommends wget gnupg lsb-release && \
    wget -q -O /tmp/llvm.sh https://apt.llvm.org/llvm.sh && \
    chmod +x /tmp/llvm.sh && \
    /tmp/llvm.sh 18 && \
    rm -f /tmp/llvm.sh && \
    rm -rf /var/lib/apt/lists/*

# Make clang-18 default (optional)
RUN update-alternatives --install /usr/bin/clang   clang   /usr/bin/clang-18   100 && \
    update-alternatives --install /usr/bin/clang++ clang++ /usr/bin/clang++-18 100 && \
    update-alternatives --install /usr/bin/cc      cc      /usr/bin/clang-18   100 && \
    update-alternatives --install /usr/bin/c++     c++     /usr/bin/clang++-18 100

# --- Rust exactly as requested (no extras) ---
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && rustc --version && cargo --version

# Paths inside the image
ENV HOME=/root \
    DEEPSURF_HOME=/root/deepSURF \
    CODE_DIR=/root/deepSURF/code \
    DATA_DIR=/root/deepSURF/dataset \
    RQS_DIR=/root/deepSURF/rqs

# -------------------
#   FINAL (LOCAL ONLY)
# -------------------
FROM base AS final

# Create dirs
RUN mkdir -p "${CODE_DIR}" "${DATA_DIR}" "${RQS_DIR}"

# Copy the already-extracted code (no archives)
COPY code/fuzzer/            ${CODE_DIR}/fuzzer/
COPY code/global_data/       ${CODE_DIR}/global_data/
COPY code/harness_generator/ ${CODE_DIR}/harness_generator/
COPY code/rust/              ${CODE_DIR}/rust/

# Copy files from build context
COPY dataset/       ${DATA_DIR}/
COPY rqs/           ${RQS_DIR}/
COPY fuzz-algo.sh   ${DEEPSURF_HOME}/
COPY generate_harnesses.sh  ${DEEPSURF_HOME}/
COPY LICENSE                ${DEEPSURF_HOME}/
COPY README.md              ${DEEPSURF_HOME}/

# Ensure any scripts are executable
RUN find "${RQS_DIR}" -type f \( -name 'replay_crash.sh' -o -name 'replay_all' \) -exec chmod +x {} + || true
RUN chmod +x ${DEEPSURF_HOME}/fuzz-algo.sh ${DEEPSURF_HOME}/generate_harnesses.sh

# Entry script (adjust source path here if your script moved)
# If your entry script now lives at ./deepsurf-entry.sh (repo root), change the COPY accordingly.
COPY code/deepsurf-entry.sh /usr/local/bin/deepsurf-entry.sh
RUN chmod +x /usr/local/bin/deepsurf-entry.sh

ENTRYPOINT ["/usr/local/bin/deepsurf-entry.sh"]
CMD ["bash"]