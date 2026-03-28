# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

ccguilt is a Rust CLI tool that tracks environmental impact of Claude Code usage. It reads Claude Code's local session data (`~/.claude`), calculates energy/CO2/water metrics from token usage, and displays results with satirical guilt commentary.

## Build & Run

```bash
cargo build                  # debug build
cargo build --release        # optimized release build (LTO + stripped)
cargo clippy                 # lint
cargo test                   # run tests (none yet)
cargo run -- --help          # show CLI usage
cargo run -- daily           # daily breakdown (deep scan)
cargo run -- weekly --fast   # weekly from stats-cache.json
cargo run -- --json total    # JSON output, all-time total
```

## Architecture

Two data ingestion paths:
- **Deep scan** (default): Parses all JSONL files under `~/.claude/projects/` in parallel via rayon. Deduplicates streaming messages by message ID (last line wins). Supports session-level breakdown.
- **Fast mode** (`--fast`): Reads `~/.claude/stats-cache.json` for pre-aggregated totals. No session-level detail.

Data flow: `CLI parse → discover data dir → parse tokens → aggregate by period → calculate cost + impact → render output`

### Module layout

- **`src/cli.rs`** — clap-derived CLI args and period enum (Daily/Weekly/Monthly/Session/Total)
- **`src/models.rs`** — Core types: `TokenRecord`, `ModelTier` (Opus/Sonnet/Haiku/Unknown), `TokenSummary`, `UsageBucket`, `CostSummary`, `ImpactSummary`, `GuiltLevel`
- **`src/config.rs`** — All constants: energy profiles (Wh/token per model), API pricing, environmental factors (CO2/kWh, PUE, water/kWh, tree baselines). Sources cited inline (Jegham 2025, Luccioni 2023, Li 2023, EPA eGRID 2024)
- **`src/aggregate.rs`** — Buckets token records by time period; `fast_path_total()`/`fast_path_daily()` convert cache data
- **`src/calc/`** — `cost.rs` computes USD from token counts + pricing; `impact.rs` computes environmental metrics + guilt level
- **`src/data/`** — `discovery.rs` finds `~/.claude` and JSONL files; `jsonl.rs` parallel-parses with rayon + dedup; `cache.rs` reads stats-cache.json
- **`src/display/`** — `table.rs` renders comfy-table; `json.rs` for structured output; `guilt.rs` has comparisons, tree progress bar, quotes

### Key design details

- Cache read tokens use 0.10x energy multiplier (memory lookup vs inference); cache creation uses 1.0x
- PUE 1.2 multiplier applied to all energy calculations for data center overhead
- `ModelTier` extracted from model name string (contains "opus"/"sonnet"/"haiku"); unknown defaults to Sonnet pricing/energy
- JSONL parsing filters to `type="assistant"` messages only, skips synthetic models and zero-token entries
- `IndexMap` preserves model insertion order in aggregated output
