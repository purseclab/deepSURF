# Memory Corruption Vulnerabilities Found by deepSURF

This table lists memory-safety bugs identified by **deepSURF** on the _SURFBench_ dataset.

*Columns:* **Dataset**, **No**, **RustSec ID** (if assigned), **Crate**, **Affected Function/Trait**, **Bug Type**.  
*Bug type codes:* DF = double free, HBOF = heap buffer overflow, SBOF = stack buffer overflow, UAF = use-after-free, MEMCRP = other memory corruption violations such as arbitrary memory access and dropping of uninitialized memory.

*The previously-unknown bugs discovered by deepSURF are marked with `*`.*

---

### Table

| Dataset | No | RustSec ID        | Crate        | Affected Function/Trait                | Bug Type |
|:-------:|---:|:------------------|:-------------|:---------------------------------------|:--------:|
| ERASan  |  1 | RUSTSEC-2021-0053 | algorithmica | `merge_sort::merge`                    |   DF     |
| ERASan  |  2* | RUSTSEC-2025-0062| toodee       | `TooDee::remove_col`                   |  HBOF    |
| ERASan  |  3 | RUSTSEC-2021-0028 | toodee       | `TooDee::insert_row`                   |  HBOF    |
| ERASan  |  4 | RUSTSEC-2021-0028 | toodee       | `TooDee::insert_row`                   |   DF     |
| ERASan  |  5 | —                  | toodee       | `TooDee::insert_col`                   |  HBOF    |
| ERASan  |  6 | RUSTSEC-2021-0033 | stack_dst    | `StackA::push_cloned`                  |   DF     |
| ERASan  |  7 | —                  | stack_dst    | `StackA::push_stable`                  |   DF     |
| ERASan  |  8 | RUSTSEC-2021-0047 | slice-deque  | `SliceDeque::drain_filter`             |   DF     |
| ERASan  |  9* | RUSTSEC-2025-0044 | slice-deque  | `SliceRingBuffer::insert`              |   DF     |
| ERASan  | 10* | RUSTSEC-2025-0044 | slice-deque  | `slice_ring_buffer::IntoIter::clone`   |   DF     |
| ERASan  | 11* | RUSTSEC-2025-0044 | slice-deque  | `SliceRingBuffer::extend_from_slice`   |   DF     |
| ERASan  | 12* | RUSTSEC-2025-0044 | slice-deque  | `SliceRingBuffer::shrink_to_fit`       |   DF     |
| ERASan  | 13 | RUSTSEC-2021-0048 | stackvector  | `StackVec::extend`                     |  SBOF    |
| ERASan  | 14 | RUSTSEC-2021-0042 | insert_many  | `Insert_many::Vec::insert_many`        |  HBOF    |
| ERASan  | 15 | RUSTSEC-2019-0009 | smallvec-0.6.6 | `SmallVec::grow`                      |   DF     |
| ERASan  | 16 | RUSTSEC-2021-0003 | smallvec-1.6.0 | `SmallVec::insert_many`               |  HBOF    |
| ERASan  | 17 | RUSTSEC-2020-0039 | simple-slab  | `Slab::index`                          | MEMCRP   |
| ERASan  | 18 | RUSTSEC-2020-0039 | simple-slab  | `Slab::remove`                         |  HBOF    |
| ERASan  | 19 | RUSTSEC-2020-0038 | ordnung      | `CompactVec::remove`                   |   DF     |
| ERASan  | 20* | —                  | ordnung      | `CompactVec::remove`                   |   UAF    |
| ERASan  | 21 | RUSTSEC-2020-0005 | cbox         | `CBox::<T>::new`                       | MEMCRP   |
| ERASan  | 22 | RUSTSEC-2021-0018 | qwutils      | `VecExt::insert_slice_clone`           |   DF     |
| ERASan  | 23 | RUSTSEC-2021-0039 | endian_trait | `slices::Endian::from_be`              |   DF     |
| ERASan  | 24 | RUSTSEC-2020-0167 | pnet_packet  | `MutablepV4Packet::set_payload`        |  HBOF    |
| ERASan  | 25 | RUSTSEC-2021-0094 | rdiff        | `BlockHashes::diff_and_update`         |  HBOF    |
| ERASan  | 26 | RUSTSEC-2021-0049 | through      | `through::through`                     |   DF     |
| RustSan | 27 | RUSTSEC-2018-0003 | smallvec-0.6.1 | `SmallVec::insert_many`               |   DF     |
| RustSan | 28 | RUSTSEC-2018-0008 | slice-deque-0.1.15 | `SliceDeque::move_head_unchecked`  | MEMCRP   |
| RustSan | 29 | RUSTSEC-2019-0012 | smallvec-0.6.3 | `SmallVec::grow`                      | MEMCRP   |
| RustSan | 30 | RUSTSEC-2020-0041 | sized-chunks | `Chunk::insert_from`                   |   DF     |
| RustSan | 31 | RUSTSEC-2020-0041 | sized-chunks | `Chunk::insert_from`                   |  SBOF    |
| RustSan | 32 | RUSTSEC-2020-0006 | bumpalo-3.2.0 | `bumpalo::realloc`                    | HBOF |
| RustSan | 33 | RUSTSEC-2020-0047 | array-queue  | `array_queue::ArrayQueue::pop_back`    | MEMCRP |
| RustSan | 34* | RUSTSEC-2025-0044| array-queue  | `array_queue::ArrayQueue::push_front`  | MEMCRP |
| RustSan | 35* | RUSTSEC-2025-0049 | scratchpad   | `scratchpad::Tracking`                 |  HBOF    |
| RustSan | 36* | RUSTSEC-2025-0053 | arenavec     | `arenavec::common::SliceVec::split_off` |   DF    |
| RustSan | 37* | RUSTSEC-2025-0053 | arenavec     | `arenavec::common::allocate_inner`     |  HBOF    |
| RustSan | 38* | RUSTSEC-2025-0053 | arenavec     | `arenavec::common::AllocHandle`        | MEMCRP   |
| RustSan | 39 | RUSTSEC-2021-0052 | id-map       | `IdMap::clone_from`                    | DF  |
| RustSan | 40 | RUSTSEC-2021-0052 | id-map       | `IdMap::get_or_insert_with`            | DF       |
| RustSan | 41* | RUSTSEC-2025-0050 | id-map       | `IdMap::from_iter`                     | MEMCRP   |
| CrabTree| 42 | —                  | leapfrog     | `HashMap::insert`                      | MEMCRP |
| **TOTAL** | **42** |  —  | — | — | — |