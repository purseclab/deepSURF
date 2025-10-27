# RQ1 — Bug-Finding Capability

deepSURF identified 42 memory safety bugs (including 12 previously-unknown) by fuzzing the harnesses that it automatically generated.

Bug numbering in this directory follows the table presented in the main repository [README](../../README.md).

---

## Directory Layout

- All proof-of-concepts (PoCs) are located under: `../rqs/rq1`.
- Each bug is stored in a directory named `bug#<id>`, e.g., `bug#3-toodee`.
- Each bug directory contains:
  - `harness/` — an auto-generated harness used to expose the bug.
  - `affected_code.txt` — the affected function or trait.
  - `crash_input` — a crashing input discovered by fuzzing.
  - `replay_crash.sh` — a script that builds the harness and replays the crash.
  - `bug_output.log` — the crashing output.
- To replay all bugs run: `../rqs/rq1/replay_all.sh`.

> **Note.** Each `bug_output.log` should show an ASan-detected memory safety violation (e.g., buffer overflow, double free, SEGV, use-after-free).  
> **Exception — Bug #29:** Reported to RustSec as [RUSTSEC-2019-0012](https://rustsec.org/advisories/RUSTSEC-2019-0012.html); the linked GitHub issue ([servo/rust-smallvec#149](https://github.com/servo/rust-smallvec/issues/149)) uses a manual PoC that triggers an “entered unreachable code” panic when the corruption is exercised.

---