# RQ3 — Ablation Study

deepSURF performs best when combining static analysis with LLM integration, as each component contributes uniquely and synergistically to its effectiveness.

---

## Directory Layout

- Each subdirectory corresponds to one configuration (as in Table 5 of the paper):  
`../rqs/rq3/<CONFIG>`, where `<CONFIG>` ∈ `{deepSURF, deepSURF-static, deepSURF-llm, deepSURF-no-skip, deepSURF-skip-all, deepSURF-o3, deepSURF-sonnet4, deepSURF-gpt5}`.

- List of all unsafe source code lines:  
`<CRATE_NAME>_unsafe_lines.log`, where `<CRATE_NAME>` ∈ `{simple-slab-0.3.2, smallvec-1.6.0, toodee-0.2.4}`.
    > Excluding blank/comment lines and code gated by features not enabled by default.

### Per-configuration contents

- Harnesses generated: `harnesses/<CRATE_NAME>`.

- PoCs of the detected bugs: `detected_bugs/<CRATE_NAME>`.
    > Same structure as `../rqs/rq1`.

- Unsafe code coverage metrics (under `unsafe_coverage/<CRATE_NAME>`):
    - `total_covered_lines_in_unsafe_regions.log` — all source lines hit during fuzzing in files that contain unsafe code.
    - `unsafe_covered_lines.log` — the subset of hits that fall inside unsafe blocks.
    - `covered_code_lines.log` — general code coverage statistics. 
---