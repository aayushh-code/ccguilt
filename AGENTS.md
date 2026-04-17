# AGENTS.md

## Commands

```bash
cargo fmt --check          # check formatting (CI runs this first)
cargo clippy -- -D warnings # lint — warnings are hard errors in CI
cargo test                  # run tests
cargo build --release       # release build (LTO + stripped)
```

CI order: `fmt → clippy → test → build`. Default branch is `master`.

## Project

ccguilt — Rust CLI that reads `~/.claude` session data and calculates environmental impact (energy, CO2, water) from token usage with satirical commentary.

## Architecture

Three data paths (selected at runtime):
- **SQLite-backed incremental** (default in deep scan): parses JSONL into `ccguilt.db`, re-ingests only changed files. Falls back to direct JSONL parse on DB error.
- **Direct JSONL** (`--no-db` or fallback): parallel-parses all JSONL under `~/.claude/projects/` via rayon, dedup by message ID (last line wins).
- **Fast mode** (`--fast`): reads `~/.claude/stats-cache.json`. No session/project detail.

Data flow: `CLI → config file merge → discover data dir → parse tokens → aggregate by period/group → calculate cost + impact → render output`

## Module layout (src/)

| Module | Purpose |
|--------|---------|
| `main.rs` | Entry point, arg routing, early-exit branches |
| `cli.rs` | clap args, Period/GroupBy enums |
| `models.rs` | TokenRecord, ModelTier, UsageBucket, CostSummary, ImpactSummary, GuiltLevel |
| `config.rs` | Energy profiles, pricing, environmental constants |
| `config_file.rs` | Loads `ccguilt.toml` user config, merges with CLI |
| `runtime.rs` | RuntimeConfig — merged CLI + config file settings |
| `aggregate.rs` | Buckets records by period/project/model; fast-path converters |
| `dateparse.rs` | Natural date parsing (`--since 7d`, `--since monday`, `--diff last-week`) |
| `sort_filter.rs` | Sort buckets, apply min-co2/min-cost/top-N filters |
| `calc/` | `cost.rs` (USD), `impact.rs` (environmental metrics + guilt level) |
| `data/` | `discovery.rs` (find ~/.claude), `jsonl.rs` (parallel parse), `cache.rs` (stats-cache.json), `db.rs` (SQLite incremental) |
| `display/` | `table.rs` (comfy-table), `json.rs`, `csv.rs`, `html.rs`, `markdown.rs`, `chart.rs`, `heatmap.rs`, `diff.rs`, `compare.rs`, `guilt.rs`, `session_detail.rs`, `offset.rs`, `mascot.rs`, `token_breakdown.rs` |
| `interactive/` | TUI mode (`-i`): `state.rs`, `render.rs` |
| `achievements.rs` | Hall of Shame achievement system |
| `recommend.rs` | Model cost/CO2 optimization tips |
| `forecast.rs` | Usage forecasting |
| `watch.rs` | `--watch` interval re-run |
| `completions.rs` | Shell completion setup |
| `update.rs` | Self-update (`--increase-guilt`) |

## Key design details

- `rusqlite` uses `bundled` feature — no system SQLite needed
- Cache read tokens: 0.10x energy multiplier; cache creation: 1.0x
- PUE 1.2 multiplier on all energy calcs
- ModelTier extracted from model name substring ("opus"/"sonnet"/"haiku"); unknown → Sonnet defaults
- JSONL filters to `type="assistant"` only, skips synthetic models and zero-token entries
- `IndexMap` preserves model insertion order
- Respects `NO_COLOR` env var; `--no-color` flag also available
