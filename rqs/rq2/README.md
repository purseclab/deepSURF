# RQ2 — Comparison with Other Rust Fuzzing Tools

deepSURF outperforms state-of-the-art Rust fuzzing tools in detecting memory corruption vulnerabilities by supporting complex sequences of APIs with diverse and complex argument types, enabling it to detect 26 memory safety bugs that other tools fail to uncover.

The bugs found by deepSURF on the ERASan dataset correspond to bug#1–bug#26 in the table presented in the main repository [README](../../README.md).

---

## Directory Layout

- Tool-specific harnesses:
  - `../rqs/rq2/<TOOL>/`, where `<TOOL>` ∈ `{deepSURF, RPG, RUG, RULF}`.
- _URAPI Coverage_ statistics (per crate):
  - deepSURF: `../rqs/rq2/deepSURF/harnesses/<crate>/deepSURF/deepsurf_stats.txt`.
  - RPG, RUG, RULF: `../rqs/rq2/{RPG|RUG|RULF}/urapi_coverage.txt`.
- Complete list of identified _URAPIs_ per crate: `../rqs/rq2/erasan_urapis.txt`.

> **Note:** _URAPI Coverage_ for RUG, RPG, and RULF was computed manually since these tools do not report this metric.
---