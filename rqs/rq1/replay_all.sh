#!/usr/bin/env bash
# rq1/replay_all.sh
# Run replay_crash.sh under bug subdirectories inside rq1/.
# Each run logs stdout/stderr into bug_output.log inside the bug dir.

set -euo pipefail

RQ_DIR="."
TARGET="${1:-all}"

run_one() {
  local bug_dir="$1"
  local script="$bug_dir/replay_crash.sh"
  local logfile="$bug_dir/bug_output.log"

  if [[ -x "$script" ]]; then
    echo "=== Running $script (logging to $logfile) ==="
    (
      cd "$bug_dir"
      # write logs relative to the bug dir
      ./replay_crash.sh >bug_output.log 2>&1
    ) || echo ">>> $bug_dir CRASH REPLAYED"
  else
    echo "Skipping $bug_dir (no replay_crash.sh)"
  fi
}

if [[ "$TARGET" == "all" ]]; then
  for bug_dir in "$RQ_DIR"/bug#*; do
    if [[ -d "$bug_dir" ]]; then
      run_one "$bug_dir"
    fi
  done
else
  bug_dir="$RQ_DIR/$TARGET"
  if [[ -d "$bug_dir" ]]; then
    run_one "$bug_dir"
  else
    echo "Error: Bug directory '$bug_dir' not found."
    exit 1
  fi
fi
