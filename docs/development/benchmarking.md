<!-- SPDX-License-Identifier: BSD-2-Clause -->
# Benchmarking

```sh
pnpm --silent bench plan smoke
pnpm --silent bench run --plan smoke
pnpm --silent bench run --plan local
```

Plans in `bench/plans/` define lengths, distributions and sample counts. Names and display identities live in `bench/catalog.toml`. The Rust harness generates deterministic weighted inputs, validates recovery against direct accumulation before timing, then alternates direct-a/direct-b/recovery and recovery/direct-b/direct-a. Each result is checked after the timer stops. Each JSONL row contains input/result hashes, output support and elapsed nanoseconds; the CLI emits a metadata row with Git head, dirty state, Rust compiler, CPU and plan. `pnpm --silent` keeps package-manager banners out of JSONL.

The `overlap` distribution has linear output support, `wide` has the same overlap across a much larger span, and `distinct` gives each pair its own sum. Length is the number of terms in each input. The initial 4096-term limit prevents accidental unbounded pair enumeration; it is a harness resource limit, not a library limit. Smoke CI checks exact equality and harness execution only.

## Claim requirements

For a performance claim, use a quiet machine with enough free RAM, fixed affinity, recorded governor/turbo settings and no swapping or competing load. On Linux, `taskset -c CPU pnpm --silent bench run --plan local` passes affinity to the child harness. Repeat in fresh processes, use identical plans and hashes, and retain raw rows under one task-local `target/scratch/` directory. Record compiler flags, dependency versions, host details, thread count and exact commit/dirty state.

Compute direct-a/direct-b ratios for each case and report their range alongside recovery/direct ratios. Do not call differences within uncontrolled variation a speedup. Investigate slowdowns outside the controls. The current harness does not automatically enforce placement, collect counters or compare Git revisions. Repeated revision measurements must use isolated checkouts and alternating process order with matching hashes. Local screens do not replace dedicated placement-controlled acceptance runs.

No speed claim accompanies the initial repository. The smoke job is not a performance regression threshold. Raw profiles, JSONL and large datasets stay out of Git. Attach release evidence to a reviewed release rather than committing it as source. Future continuous reporting must match the current head and read the actual benchmark check results, not only a tracking workflow.
