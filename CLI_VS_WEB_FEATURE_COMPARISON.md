# CLI vs Web Feature Comparison

This document compares the features available in the CLI and web interfaces of the Time Tracker application, identifying what's missing in the web interface.

## Summary

The web interface currently implements a subset of CLI features focused on basic viewing and period filtering. Missing are advanced filtering options (tags, project, exclusions), the breakdown reporting feature, and task detail views.

## Feature Matrix

| Feature | CLI | Web | Notes |
|---------|-----|-----|-------|
| **Input/Output** |
| Input file specification | ✅ `-i, --input` | ✅ Startup only | Web: Set at server start via CLI flag |
| Verbose output | ✅ `-v, --verbose` | ❌ | Web: N/A - logs go to console |
| Output format (text/markdown) | ✅ `--format` | ❌ | Web: HTML only |
| **Filtering** |
| Period: today | ✅ `--period today` | ✅ Button | |
| Period: this-week | ✅ `--period this-week` | ✅ Button | |
| Period: last-week | ✅ `--period last-week` | ❌ | |
| Period: this-month | ✅ `--period this-month` | ✅ Button | |
| Period: last-month | ✅ `--period last-month` | ❌ | |
| Period: month-n notation | ✅ `--period m-3` | ❌ | CLI: m-n, month-n for relative months |
| Period: year | ✅ `--period 2024` | ❌ | |
| Custom date range | ✅ `--from` | ✅ Date inputs | Web: Requires both from and to |
| From date only | ✅ `--from` | ❌ | Web: Requires both dates |
| Project filter | ✅ `--project` | ❌ | CLI: Filter by single project (first tag) |
| Tags filter | ✅ `--tags` | ❌ | CLI: OR semantics, comma-separated |
| Exclude tags | ✅ `--exclude-tags` | ❌ | CLI: Comma-separated exclusion list |
| **Display Options** |
| Limit to top 90% | ✅ `--limit` | ✅ Checkbox | |
| Show project details/tasks | ✅ `--details` | ❌ | CLI: Shows task descriptions per project |
| Breakdown with task details | ✅ `--breakdown --details` | ❌ | CLI: Tasks aggregated by first tag at each level |
| **Reporting** |
| Overview/summary | ✅ | ✅ | Total time + projects list |
| Time breakdown | ✅ `--breakdown` | ❌ | CLI: Hierarchical by day/week/month/year |
| Breakdown: day | ✅ `-b day` | ❌ | |
| Breakdown: week | ✅ `-b week` | ❌ | |
| Breakdown: month | ✅ `-b month` | ❌ | |
| Breakdown: year | ✅ `-b year` | ❌ | |
| Breakdown: auto | ✅ `-b auto` | ❌ | CLI: Auto-selects based on period |
| Tag/project drill-down | ❌ | ✅ | Web: Click to see individual entries |
| Outcomes analysis | ✅ | ✅ | Web: Separate page |
| **Visualization** |
| Text table output | ✅ | ❌ | |
| Markdown output | ✅ | ❌ | |
| HTML table display | ❌ | ✅ | |
| Pie charts | ❌ | ✅ | Web: Chart.js visualizations |
| **Server** |
| Web server mode | ✅ `--web` | N/A | CLI launches web server |
| Port configuration | ✅ `-p, --port` | N/A | CLI: Default 3000 |
| Host configuration | ✅ `--host` | N/A | CLI: Default 127.0.0.1 |

## Missing Features in Web Interface

### Priority 1: Core Filtering Features

These features are fundamental for focused analysis and are heavily used in CLI workflows.

#### 1.1 Tags Filter (`--tags`)

**CLI Usage:**
```bash
tt -i data.md --period this-week --tags work,coding
```

**Description:** Filter to show only entries with specified tags (OR semantics). Allows focusing on specific categories of work.

**Use Case:** "Show me all time spent on work or coding tasks this week"

**Implementation Notes:**
- Web needs multi-select tag picker or comma-separated input
- Should integrate with existing period filters
- Updates both dashboard and outcomes pages

---

#### 1.2 Project Filter (`--project`)

**CLI Usage:**
```bash
tt -i data.md --period this-month --project myproject
```

**Description:** Filter by single project (uses first tag as project identifier). Stricter than tags filter.

**Use Case:** "Show me all time entries for myproject this month"

**Implementation Notes:**
- Dropdown or autocomplete of available projects
- Mutually exclusive with tags filter
- Shows only entries where first tag matches project

---

#### 1.3 Exclude Tags (`--exclude-tags`)

**CLI Usage:**
```bash
tt -i data.md --period this-week --exclude-tags meetings,admin
```

**Description:** Exclude entries with specified tags from results.

**Use Case:** "Show me all work except meetings and admin tasks"

**Implementation Notes:**
- Can combine with tags/project filters
- Multi-select or comma-separated input
- Applies filter at parsing level

---

### Priority 2: Time Breakdown Reporting

This is a significant CLI feature with no web equivalent. Provides hierarchical time analysis by calendar units.

#### 2.1 Breakdown by Day

**CLI Usage:**
```bash
tt -i data.md --period this-week --tags work --breakdown day
```

**Output Example:**
```
2025-01-15 (Wed)  3h 45m
2025-01-16 (Thu)  2h 30m
2025-01-17 (Fri)  1h 15m
```

**Description:** Lists all days with entries, showing total time per day.

**Use Case:** "Show me daily totals for work tasks this week"

---

#### 2.2 Breakdown by Week

**CLI Usage:**
```bash
tt -i data.md --period this-month --tags work --breakdown week
```

**Output Example:**
```
2025-W03
  2025-01-15 (Wed)  3h 45m
  2025-01-16 (Thu)  2h 30m
  2025-01-17 (Fri)  1h 15m
2025-W04
  2025-01-22 (Mon)  4h 00m
```

**Description:** Hierarchical view: weeks → days. Shows ISO week numbers.

**Use Case:** "Show me weekly breakdown of work time this month"

---

#### 2.3 Breakdown by Month

**CLI Usage:**
```bash
tt -i data.md --period 2024 --project myproject --breakdown month
```

**Description:** Hierarchical view: months → weeks (→ days if needed).

**Use Case:** "Show me monthly breakdown of project time across the year"

---

#### 2.4 Breakdown by Year

**CLI Usage:**
```bash
tt -i data.md --tags work --breakdown year
```

**Description:** Hierarchical view: years → months.

**Use Case:** "Show me yearly trends for work time"

---

#### 2.5 Breakdown Auto Mode

**CLI Usage:**
```bash
tt -i data.md --period this-week --tags work --breakdown auto
```

**Description:** Automatically selects breakdown unit one level above period:
- Day period → Week breakdown
- Week period → Month breakdown
- Month period → Year breakdown
- Year period → Year breakdown

**Use Case:** "Show me contextual time breakdown based on my period selection"

**Implementation Notes:**
- Could be new page: "/breakdown" or tab in dashboard
- Requires tree/hierarchical display component
- Should support both text and markdown export
- Needs tags/project filter to work (same as CLI)

---

### Priority 3: Extended Period Options

Additional period filter options available in CLI.

#### 3.1 Last Week

**CLI Usage:**
```bash
tt -i data.md --period last-week
# or short form:
tt -i data.md --period lw
```

**Description:** Filter to previous week.

**Implementation Notes:** Add button next to "This Week"

---

#### 3.2 Last Month

**CLI Usage:**
```bash
tt -i data.md --period last-month
# or short form:
tt -i data.md --period lm
```

**Description:** Filter to previous month.

**Implementation Notes:** Add button next to "This Month"

---

#### 3.3 Relative Month Notation

**CLI Usage:**
```bash
tt -i data.md --period m-3   # 3 months ago
tt -i data.md --period month-6
```

**Description:** Filter to specific month N months ago.

**Implementation Notes:**
- Dropdown or number input
- Label: "Months ago: [ 1 ▼]" or slider

---

#### 3.4 Year Filter

**CLI Usage:**
```bash
tt -i data.md --period 2024
```

**Description:** Filter to entire year.

**Implementation Notes:**
- Year picker or dropdown of available years
- Auto-detect years from data

---

### Priority 4: Detail Views

#### 4.1 Project Details View (`--details`)

**CLI Usage:**
```bash
tt -i data.md --period this-week --tags work --details
```

**Description:** Shows individual task descriptions grouped by project, not just aggregated time totals.

**Use Case:** "Show me what specific tasks I worked on for each project"

**Current Web Behavior:** Web has tag drill-down (click project to see entries), but no equivalent to the CLI's `--details` flag for formatted task lists.

**Implementation Notes:**
- May already be partially satisfied by tag detail view
- Could enhance tag detail view to match CLI details format
- Consider toggle between summary and detail modes

---

### Priority 5: Output Format Options

#### 5.1 Export as Markdown

**CLI Usage:**
```bash
tt -i data.md --period this-month --format markdown > report.md
```

**Description:** Export report in markdown format for documentation.

**Implementation Notes:**
- Add "Export" dropdown with Markdown/Text options
- Generate file download with formatted output
- Should work for both overview and breakdown reports

---

#### 5.2 Export as Text

**CLI Usage:**
```bash
tt -i data.md --period this-month --format text > report.txt
```

**Description:** Export report in plain text format.

**Implementation Notes:** Same as markdown export

---

### Priority 6: Date Range Flexibility

#### 6.1 From-Date-Only Filter

**CLI Usage:**
```bash
tt -i data.md --from 2025-01-01
```

**Description:** Show all entries from specified date onward (no end date).

**Current Web Behavior:** Web requires both from and to dates.

**Implementation Notes:**
- Make "to" date optional
- Default to "today" if to date empty
- Update validation logic

---

## Implementation Recommendations

### Phase 1: Essential Filters (High Impact)
1. Tags filter with multi-select
2. Exclude tags filter
3. Last week / last month buttons
4. Project filter dropdown

**Rationale:** These are most commonly used for focused analysis and already have backend support.

---

### Phase 2: Time Breakdown (High Value)
1. New "/breakdown" page or dashboard tab
2. Day breakdown view
3. Week breakdown view (hierarchical)
4. Auto breakdown mode
5. Month/year breakdown (future)

**Rationale:** Major CLI feature with no web equivalent. Provides powerful time analysis capabilities.

---

### Phase 3: Extended Periods & Export
1. Relative month notation (m-n)
2. Year filter
3. Markdown/text export buttons
4. From-date-only support

**Rationale:** Enhances flexibility and allows documentation workflows.

---

### Phase 4: Refinements
1. Enhanced detail view formatting
2. Additional chart types for breakdown data
3. Keyboard shortcuts for common filters
4. Filter presets/bookmarks

**Rationale:** Polish and user experience improvements.

---

## Notes

- **Web-only features:** Interactive drill-down, pie charts, HTMX updates
- **CLI-only features:** Verbose mode, text/markdown output formats
- **Validation requirements:** CLI requires `--breakdown` to have `--tags` or `--project` specified
- **Architecture:** All filtering logic exists in domain layer, web just needs UI + handler wiring

## References

- CLI Args: `src/cli/mod.rs`
- Web Handlers: `src/web/handlers.rs`
- Web Models: `src/web/models.rs`
- Templates: `templates/dashboard.html`, `templates/outcomes.html`
- Help output: `cargo run --release -- --help`
