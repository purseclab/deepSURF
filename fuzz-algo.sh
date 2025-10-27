#!/usr/bin/env bash
# rqs/fuzz.sh
# Launch AFL++ fuzzing for all harnesses generated in RQ2 algorithmica-0.1.8.
# Spawns one CmpLog *master* and one ASAN *follower* per harness using tmux.
#
# Usage:
#   ./fuzz.sh                      # run with defaults (60s)
#   FUZZING_TIME=300 ./fuzz.sh     # run for 5 minutes
#
# Expected harness layout per directory:
#   <harness_dir>/
#     bins/<target_name>_non_asan
#     bins/<target_name>_asan
#     input/           (seed dir; created if missing with a single '0' file)
#     len              (optional; if present, passed as -g <len> to cargo-afl)
#
# Notes:
# - Requires tmux and cargo-afl (installed by the container's entry script).
# - Session names are prefixed with "$SESSION_PREFIX" to avoid collisions.

set -Eeuo pipefail

# -------- Config (overridable via env) --------
CRATE_PATH="algorithmica-0.1.8"
CORPUS_ROOT="${CORPUS_ROOT:-$HOME/deepSURF/rqs/rq2/deepSURF/harnesses/$CRATE_PATH/deepSURF/fuzz/fuzzing_corpus/}"
LOG_FILE="${LOG_FILE:-$CORPUS_ROOT/fuzz.log}"
FUZZING_TIME="${FUZZING_TIME:-60}"         # seconds total
INIT_DELAY="${INIT_DELAY:-0.2}"            # delay between tmux session launches
SESSION_PREFIX="${SESSION_PREFIX:-rq2algo}" # prefix for tmux sessions we create
RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-rustc-fuzzing}"  # default set by entry script

# AFL env (recommended)
export AFL_I_DONT_CARE_ABOUT_MISSING_CRASHES=0
export AFL_SKIP_CPUFREQ=0

# -------- Preflight --------
if ! command -v tmux >/dev/null 2>&1; then
  echo "[fuzz] ERROR: tmux not found. Please install tmux." >&2
  exit 2
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "[fuzz] ERROR: cargo not found in PATH." >&2
  exit 2
fi
if ! cargo +"$RUSTUP_TOOLCHAIN" afl --help >/dev/null 2>&1; then
  echo "[fuzz] ERROR: cargo-afl subcommand not available for toolchain '$RUSTUP_TOOLCHAIN'." >&2
  echo "[fuzz] Hint: cargo install cargo-afl --version 0.15.7 --force" >&2
  exit 2
fi
if [[ ! -d "$CORPUS_ROOT" ]]; then
  echo "[fuzz] ERROR: corpus root not found: $CORPUS_ROOT" >&2
  exit 2
fi

mkdir -p "$(dirname "$LOG_FILE")"
echo "[fuzz] Starting fuzzing campaign in: $CORPUS_ROOT" | tee "$LOG_FILE"
echo "[fuzz] Total time: ${FUZZING_TIME}s" | tee -a "$LOG_FILE"
echo "[fuzz] Toolchain: +${RUSTUP_TOOLCHAIN}" | tee -a "$LOG_FILE"

# Gather harness dirs
mapfile -t HARNESS_DIRS < <(find "$CORPUS_ROOT" -mindepth 1 -maxdepth 1 -type d | sort)
if [[ ${#HARNESS_DIRS[@]} -eq 0 ]]; then
  echo "[fuzz] No harness directories found under $CORPUS_ROOT" | tee -a "$LOG_FILE"
  exit 0
fi

# Sanitize a name into a safe tmux session id
safe_name() {
  echo "$1" | tr -c '[:alnum:]_-' '_'
}

# Launch fuzzers
idx=1
STARTED=()
for harness_dir in "${HARNESS_DIRS[@]}"; do
  base="$(basename "$harness_dir")"
  # Keep behavior consistent with your example: strip _deepseek-r1_turn*
  target_name="${base%%_deepseek-r1_turn*}"
  bins_dir="$harness_dir/bins"
  bin_cmplog="$bins_dir/${target_name}_non_asan"
  bin_asan="$bins_dir/${target_name}_asan"
  in_dir="$harness_dir/input"
  out_dir="$harness_dir/out"
  len_file="$harness_dir/len"

  # Validate binaries exist
  missing=0
  [[ -f "$bin_cmplog" ]] || { echo "[fuzz] WARN: missing $bin_cmplog" | tee -a "$LOG_FILE"; missing=1; }
  [[ -f "$bin_asan"   ]] || { echo "[fuzz] WARN: missing $bin_asan"   | tee -a "$LOG_FILE"; missing=1; }
  if [[ "$missing" -eq 1 ]]; then
    continue
  fi

  # Ensure binaries are executable (your request)
  if [[ ! -x "$bin_cmplog" ]]; then
    chmod +x "$bin_cmplog" 2>/dev/null || true
  fi
  if [[ ! -x "$bin_asan" ]]; then
    chmod +x "$bin_asan" 2>/dev/null || true
  fi
  # Re-check executability
  if [[ ! -x "$bin_cmplog" || ! -x "$bin_asan" ]]; then
    echo "[fuzz] WARN: binaries not executable after chmod for $base; skipping." | tee -a "$LOG_FILE"
    continue
  fi

  # Seed directory
  mkdir -p "$in_dir"
  [[ -f "$in_dir/input0" ]] || echo "0" > "$in_dir/input0"

  # Clean previous output
  rm -rf "$out_dir"

  # Optional -g flag from len file
  g_flag=()
  if [[ -f "$len_file" ]]; then
    fuzzer_slice_len="$(cat "$len_file" || true)"
    if [[ -n "$fuzzer_slice_len" ]]; then
      g_flag=(-g "$fuzzer_slice_len")
    fi
  fi
  # Flatten -g for command string
  GFLAG_STR=""
  if [[ ${#g_flag[@]} -gt 0 ]]; then
    GFLAG_STR="-g ${g_flag[1]}"
  fi

  # Session names
  tag="$(safe_name "${SESSION_PREFIX}_${idx}_${target_name}")"
  sess_cmplog="${tag}_cmplog"
  sess_asan="${tag}_asan"

  # CmpLog master (non-ASAN)
  tmux new-session -d -s "$sess_cmplog" \
    bash -lc "exec cargo +${RUSTUP_TOOLCHAIN} afl fuzz ${GFLAG_STR} \
      -i '$in_dir' -o '$out_dir' -M 'fuzz_${idx}_cmplog' \
      '$bin_cmplog'"

  sleep "$INIT_DELAY"

  # ASAN follower
  tmux new-session -d -s "$sess_asan" \
    bash -lc "exec cargo +${RUSTUP_TOOLCHAIN} afl fuzz ${GFLAG_STR} \
      -i '$in_dir' -o '$out_dir' -S 'fuzz_${idx}_asan' -c - \
      '$bin_asan'"

  STARTED+=("$sess_cmplog" "$sess_asan")
  echo "[fuzz] Started $sess_cmplog and $sess_asan for $base" | tee -a "$LOG_FILE"
  idx=$((idx + 1))
done

if [[ ${#STARTED[@]} -eq 0 ]]; then
  echo "[fuzz] No sessions started (nothing to fuzz?)." | tee -a "$LOG_FILE"
  exit 0
fi

# Wait for the requested time
echo "[fuzz] $((${#STARTED[@]})) tmux sessions running. Sleeping ${FUZZING_TIME}s…" | tee -a "$LOG_FILE"
sleep "$FUZZING_TIME"

# Tear down sessions we created
for s in "${STARTED[@]}"; do
  tmux kill-session -t "$s" 2>/dev/null || true
done

echo "[fuzz] Fuzzing campaign complete." | tee -a "$LOG_FILE"
echo "[fuzz] Outputs: per-harness 'out/' directories under each harness folder." | tee -a "$LOG_FILE"
