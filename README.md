# Time Tracker (`tt`)

A Rust-based CLI time tracking application that parses markdown files to generate time reports. Process time tracking data with filtering, reporting, and multiple output formats.

[![Tests](https://github.com/lhohan/simple-time-tracker/actions/workflows/rust.yml/badge.svg)](https://github.com/lhohan/simple-time-tracker/actions/workflows/rust.yml)
[![Coverage Status](https://coveralls.io/repos/github/lhohan/simple-time-tracker/badge.svg?branch=main)](https://coveralls.io/repos/github/lhohan/simple-time-tracker?branch=main)

## Installation

### Local Install
```bash
just install
```

### Build from Source
```bash
just build
```

## Usage

### Basic Usage
```bash
# Run with specific file and period
just run "./data.md" "this-week"

# Direct cargo run
cargo run -- -i "./data.md" --period "this-week"
```

### Features
- Parse markdown files for time entries
- Filter by time periods (this-week, last-week, etc.)
- Generate reports in multiple formats (text, markdown)
- Tag-based categorization and filtering
- Hierarchical time breakdown by calendar units (day, week, month, year)
- CLI-based interface with comprehensive options

### Time Breakdown Reports

View hierarchical breakdowns of tracked time by calendar units. The `--breakdown` flag requires either `--tags` or `--project` to filter entries.

#### Supported Units
- **day**: No hierarchy, lists all days with entries
- **week**: Hierarchical: weeks → days
- **month**: Hierarchical: months → weeks (→ days if needed)
- **year**: Hierarchical: years → months
- **auto**: Automatically selects a unit one level above the current period
  - Day period → Week breakdown
  - Week period → Month breakdown
  - Month period → Year breakdown
  - Year period → Year breakdown

#### Examples

```bash
# Show daily breakdown for specific tag (current day)
just run "./data.md" "today" --tags work --breakdown day

# Show weekly breakdown (weeks with days) for this week
just run "./data.md" "this-week" --tags work --breakdown week

# Show monthly breakdown (months with weeks) for a month
just run "./data.md" "this-month" --project myproject --breakdown month

# Show yearly breakdown (years with months) for a year
just run "./data.md" "2024" --tags work --breakdown year

# Automatic breakdown (one level above the period)
just run "./data.md" "this-week" --tags work --breakdown auto

# Markdown format output
just run "./data.md" "this-month" --tags work --breakdown week --format markdown
```

#### Features
- **Chronological ordering**: Entries appear in date order
- **Human-friendly labels**: Includes weekday names (e.g., "2025-01-15 (Wed)"), ISO week numbers ("2025-W03")
- **Zero-entry omission**: Days/weeks with no entries are excluded
- **Multiple formats**: Text (indented tree) and Markdown (heading hierarchy)
- **Tag filtering**: Works with `--tags` (OR semantics) or `--project` (first tag)

#### Output Format Examples

**Text format (week breakdown):**
```
2025-W03
  2025-01-15 (Wed)  3h 45m
  2025-01-16 (Thu)  2h 30m
  2025-01-17 (Fri)  1h 15m
```

**Markdown format:**
```
# Time Breakdown Report

## 2025-W03

- 2025-01-15 (Wed): 3h 45m
- 2025-01-16 (Thu): 2h 30m
- 2025-01-17 (Fri): 1h 15m
```

## Development Setup

### Prerequisites

- Rust toolchain
- Just (command runner)

Or, all batteries included dependencies management:
- Nix (for isolated dependency management, optional)
- direnv (optional but recommended for automatic environment loading)

### Claude Code Integration

This project includes project-scoped Serena MCP integration for enhanced Claude Code capabilities.

#### Usage
```bash
# Just enter the directory. direnv handles the rest.
# Then, simply run the claude command.
claude
```

Serena's tools will be automatically available within your Claude session.

#### Architecture
- **Zero-Configuration**: `direnv` automatically configures your shell. Just run `claude`.
- **No Global Installs**: All Python dependencies managed via Nix flake
- **Project Scoped**: Serena MCP only active when using this repo's configuration
- **Zero Pollution**: No changes to global Claude Code configuration
- **Reproducible**: Same environment across machines via Nix

### Files
- `scripts/serena-mcp` - Serena MCP server launcher (uses `nix develop -c uvx`)
- `.envrc` - Automatically configures the `claude` command with project-specific settings.
- `flake.nix` - Updated with Python 3.12 and uv dependencies

### Maintenance
- Serena updates automatically via `uvx --from git+https://github.com/oraios/serena`
- To pin a specific version, modify the git reference in `scripts/serena-mcp`
- If Serena changes its invocation pattern, update `scripts/serena-mcp`
