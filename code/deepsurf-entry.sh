#!/usr/bin/env bash
# code/deepsurf-entry.sh
# Build & register custom Rust toolchains at container start, set up AFL, coredumps, then exec the given command.
# Idempotent; controlled by env flags below.
set -Eeuo pipefail

log() { echo "[entry] $*"; }

# ----- Paths (match your image layout) -----
: "${HOME:=/root}"
: "${DEEPSURF_HOME:=$HOME/deepSURF}"
: "${CODE_DIR:=$DEEPSURF_HOME/code}"
: "${DATA_DIR:=$DEEPSURF_HOME/dataset}"
: "${RQS_DIR:=$DEEPSURF_HOME/rqs}"

# ----- Toggles -----
: "${BUILD_CUSTOM_RUST:=1}"   # 0 = skip stage2 build/link
: "${PIN_HOME_COMPAT:=1}"     # kept for completeness; currently commented-out below
: "${CLEAN_BUILD:=1}"         # 0 = keep prior partial build/

# AFL setup toggles
: "${SET_FUZZING_DEFAULT:=1}"     # make nightly-2024-06-13 default via alias rustc-fuzzing
: "${AFL_VERSION:=0.15.7}"        # afl crate version to pin in change-afl
: "${DO_AFL_SETUP:=1}"            # 0 = skip AFL steps

# Extra cargo tools
: "${DO_TOOLS_INSTALL:=1}"        # install rustfilt & rustdoc-md
: "${RUSTDOC_MD_TOOLCHAIN:=rustc-rustdoc-md}"

# Coredump setup
: "${ENABLE_COREDUMPS:=1}"        # set ulimit and kernel.core_pattern
: "${CORE_PATTERN:=core}"         # desired /proc/sys/kernel/core_pattern value

# Ensure rustup/cargo available in this shell
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env"
else
  export PATH="$HOME/.cargo/bin:$PATH"
fi

# Helper: install a nightly and link an alias
install_and_link() {
  local date="$1" alias="$2"
  if ! rustup toolchain list | grep -q "nightly-${date}"; then
    log "Installing nightly-${date}…"
    rustup toolchain install "nightly-${date}"
  fi
  local path="$HOME/.rustup/toolchains/nightly-${date}-x86_64-unknown-linux-gnu"
  if [[ -d "$path" ]] && ! rustup toolchain list | grep -q "^${alias}"; then
    rustup toolchain link "${alias}" "${path}"
    log "Linked toolchain: ${alias} -> nightly-${date}"
  fi
}

# -------------------------------
#   Custom Rust (stage2) build
# -------------------------------
if [[ "$BUILD_CUSTOM_RUST" = "1" ]]; then
  RUST_SRC="$CODE_DIR/rust"
  if [[ -d "$RUST_SRC" ]]; then
    log "Custom Rust source: $RUST_SRC"

    # (home crate pinning left commented-out per your note)

    # Clean previous partial build once
    if [[ "$CLEAN_BUILD" = "1" && -d "$RUST_SRC/build" && ! -x "$RUST_SRC/build/host/stage2/bin/rustc" ]]; then
      log "Cleaning previous partial build directory…"
      rm -rf "$RUST_SRC/build"
    fi

    cd "$RUST_SRC"

    # Build stage2 if not present
    if [[ ! -x "$RUST_SRC/build/host/stage2/bin/rustc" ]]; then
      log "Building Rust stage2 (./x build --stage 2)… this can take a while."
      ./x build --stage 2
    else
      log "Stage2 already present; skipping build."
    fi

    # Link toolchain: rustc_surf -> stage2
    if ! rustup toolchain list | grep -q '^rustc_surf'; then
      rustup toolchain link rustc_surf "$RUST_SRC/build/host/stage2" || true
      log "Linked toolchain: rustc_surf"
    fi

    # Move cargo from stage2-tools-bin → stage2/bin if needed
    HOST_TRIPLE="$(rustc -Vv 2>/dev/null | sed -n 's/^host: //p' || true)"
    [[ -z "$HOST_TRIPLE" ]] && HOST_TRIPLE="x86_64-unknown-linux-gnu"
    CARGO_SRC="$RUST_SRC/build/${HOST_TRIPLE}/stage2-tools-bin/cargo"
    CARGO_DST="$RUST_SRC/build/${HOST_TRIPLE}/stage2/bin/cargo"
    if [[ -f "$CARGO_SRC" && ! -f "$CARGO_DST" ]]; then
      mkdir -p "$(dirname "$CARGO_DST")"
      mv -f "$CARGO_SRC" "$CARGO_DST"
      log "Moved cargo into stage2/bin."
    fi

    # Install & link your three nightly toolchains (aliases)
    install_and_link "2025-01-27" "rustc-rustdoc-md"
    install_and_link "2024-06-13" "rustc-fuzzing"
    install_and_link "2024-02-01" "rustc-replay-crash"

    log "Toolchains ready:"
    rustup toolchain list
  else
    log "No custom Rust source at $RUST_SRC; skipping custom build."
  fi
else
  log "BUILD_CUSTOM_RUST=0 — skipping custom Rust build."
fi

# -------------------------------
#   AFL setup & default toolchain
# -------------------------------
if [[ "$DO_AFL_SETUP" = "1" ]]; then
  # 1) make nightly-2024-06-13 default (use alias rustc-fuzzing if present)
  if [[ "$SET_FUZZING_DEFAULT" = "1" ]]; then
    FUZZ_DATE="2024-06-13"
    FUZZ_TC="nightly-${FUZZ_DATE}"
    FUZZ_PATH="$HOME/.rustup/toolchains/${FUZZ_TC}-x86_64-unknown-linux-gnu"
    if ! rustup toolchain list | grep -q "${FUZZ_TC}"; then
      log "Installing ${FUZZ_TC}…"
      rustup toolchain install "${FUZZ_TC}"
    fi
    if ! rustup toolchain list | grep -q '^rustc-fuzzing'; then
      rustup toolchain link rustc-fuzzing "${FUZZ_PATH}" || true
    fi
    rustup default rustc-fuzzing
    export RUSTUP_TOOLCHAIN=rustc-fuzzing
    log "Default toolchain set to: $(rustup show active-toolchain || echo rustc-fuzzing)"
  fi

  # 2) cargo afl config --plugins --force (pin cargo-afl to 0.15.7 and force reinstall)
  log "Installing cargo-afl 0.15.7 (forced)…"
  cargo install cargo-afl --version 0.15.7 --force --locked || true
  log "Running: cargo afl config --plugins --force --locked"
  cargo afl config --plugins --force || true

  # 3) cd  (home)
  cd "$HOME"

  # 4) cargo new change-afl
  if [[ ! -d change-afl ]]; then
    cargo new change-afl
  fi

  # 5) echo 'afl="=<AFL_VERSION>"' >> change-afl/Cargo.toml (ensure [dependencies])
  TOML="$HOME/change-afl/Cargo.toml"
  if ! grep -q '^\[dependencies\]' "$TOML"; then
    printf "\n[dependencies]\n" >> "$TOML"
  fi
  if ! grep -q '^afl\s*=' "$TOML"; then
    echo "afl = \"=${AFL_VERSION}\"" >> "$TOML"
    echo "home = \"=0.5.9\"" >> "$TOML"
  fi

  # 6) cd change-afl/ && cargo build
  pushd "$HOME/change-afl" >/dev/null
  cargo build
  popd >/dev/null

  # 7) Replace afl crate's lib.rs with code/fuzzer/src/lib.rs and rebuild
  SRC_OVERRIDE="$CODE_DIR/fuzzer/src/lib.rs"
  if [[ -f "$SRC_OVERRIDE" ]]; then
    CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"
    AFL_SRC_DIR="$(find "${CARGO_HOME_DIR}/registry/src" -maxdepth 3 -type d -name "afl-${AFL_VERSION}" 2>/dev/null | head -n 1 || true)"
    if [[ -n "$AFL_SRC_DIR" && -d "$AFL_SRC_DIR/src" ]]; then
      cp -f "${AFL_SRC_DIR}/src/lib.rs" "${AFL_SRC_DIR}/src/lib.rs.bak" || true
      cp -f "$SRC_OVERRIDE" "${AFL_SRC_DIR}/src/lib.rs"
      log "Patched ${AFL_SRC_DIR}/src/lib.rs with ${SRC_OVERRIDE}"
      pushd "$HOME/change-afl" >/dev/null
      cargo clean || true
      cargo build
      popd >/dev-null
    else
      log "WARNING: Could not locate afl-${AFL_VERSION} sources under ${CARGO_HOME_DIR}/registry/src (was the build successful?)."
    fi
  else
    log "WARNING: Override not found: ${SRC_OVERRIDE}"
  fi
else
  log "DO_AFL_SETUP=0 — skipping AFL steps."
fi

# -------------------------------
#   Extra cargo tools
# -------------------------------
if [[ "$DO_TOOLS_INSTALL" = "1" ]]; then
  if ! command -v rustfilt >/dev/null 2>&1; then
    log "Installing rustfilt…"
    cargo install rustfilt || true
  fi
  if ! command -v rustdoc-md >/dev/null 2>&1; then
    if rustup toolchain list | grep -q "^${RUSTDOC_MD_TOOLCHAIN}"; then
      log "Installing rustdoc-md with +${RUSTDOC_MD_TOOLCHAIN}…"
      cargo +${RUSTDOC_MD_TOOLCHAIN} install rustdoc-md || true
    else
      log "Installing rustdoc-md…"
      cargo install rustdoc-md || true
    fi
  fi
fi

# -------------------------------
#   Enable core dumps (persist via entry)
# -------------------------------
if [[ "$ENABLE_COREDUMPS" = "1" ]]; then
  ulimit -c unlimited || true
  CURRENT="$(cat /proc/sys/kernel/core_pattern 2>/dev/null || echo "")"
  if [[ "$CURRENT" != "$CORE_PATTERN" ]]; then
    if echo "$CORE_PATTERN" > /proc/sys/kernel/core_pattern 2>/dev/null; then
      log "Set kernel.core_pattern -> '$CORE_PATTERN'"
    elif command -v sysctl >/dev/null 2>&1 && sysctl -w kernel.core_pattern="$CORE_PATTERN" >/dev/null 2>&1; then
      log "Set kernel.core_pattern via sysctl -> '$CORE_PATTERN'"
    else
      log "WARNING: Could not set kernel.core_pattern (need extra privileges)."
      log "Hint: run with either:"
      log "  docker run --privileged --ulimit core=-1 -it --rm deepsurf"
      log "  docker run --cap-add=SYS_ADMIN --security-opt seccomp=unconfined --ulimit core=-1 -it --rm deepsurf"
    fi
  else
    log "kernel.core_pattern already '$CORE_PATTERN'"
  fi
fi

#Build LLVM Artifacts
cargo +rustc_surf afl config --build
cargo +rustc-replay-crash afl config --build
log "deepSURF is ready!"

# Hand off to the container's main command
exec "$@"
