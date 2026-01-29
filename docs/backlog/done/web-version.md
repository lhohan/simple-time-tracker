# Time Tracker Web Dashboard - Implementation Prompt

> **Status: COMPLETED** (with modifications)
> The web dashboard has been implemented and integrated into the main `tt` binary using the `--web` flag instead of a separate binary. See `docs/web-dashboard.md` for current documentation.

## Overview
Create a clean, extensible web interface for the time-tracker project using **Axum + HTMX + Askama**.
The web server should:
- Reuse the existing domain, parsing, and reporting layers
- Read from the same data files as the CLI (ports & adapters pattern)
- Serve as a test of the hexagonal architecture's flexibility
- Start simple and visual, but leave room for advanced features

## Tech Stack

### Core Framework
- **Web Framework**: Axum (Rust async web framework)
- **Templating**: Askama (compile-time type-safe templates)
  - Crates: `askama`, `askama_axum`
  - All template errors caught at compile time
  - Templates compile to Rust code for zero runtime overhead
- **Interactivity**: HTMX for dynamic UI without full JavaScript framework
- **Visualization**: Chart.js via CDN for charts
- **Styling**: Pico CSS or Tailwind CSS (minimal, clean design)

### Dependencies to Add
```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
askama = "0.12"
askama_axum = "0.4"
tower = "0.4"
tower-http = { version = "0.5", features = ["fs", "trace"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Architecture

### Project Structure
Create a new `src/web/` adapter layer alongside the existing CLI adapter:

```
src/
├── main.rs              # CLI entry point (keep as-is)
├── lib.rs               # Core orchestration (keep as-is)
├── domain/              # Pure business logic (keep as-is)
├── parsing/             # Parsing adapter (keep as-is)
├── reporting/           # Reporting adapter (keep as-is)
├── cli/                 # CLI adapter (keep as-is)
└── web/                 # NEW: Web adapter
    ├── mod.rs           # Public API exports
    ├── server.rs        # Axum server setup & routing
    ├── handlers.rs      # HTTP request handlers
    ├── models.rs        # Request/response types
    └── templates/       # Askama templates
        ├── base.html    # Base layout with CSS
        ├── dashboard.html  # Main dashboard view
        └── components.html # Reusable HTMX partials
```

### Separation of Concerns
- **Web adapter**: HTTP layer only (routes, handlers, templates)
- **Domain layer**: Zero changes required (already pure)
- **Parsing adapter**: Reuse as-is for file reading
- **Reporting adapter**: Reuse domain report types

### Web Entry Point
Create a separate binary for the web server:

```rust
// src/bin/tt-web.rs
use time_tracker::web;

#[tokio::main]
async fn main() {
    web::run_server("127.0.0.1:3000").await;
}
```

## Testing Strategy

### Follow DSL Pattern from CLI Tests

The codebase has an excellent fluent DSL pattern documented in `/docs/UNIVERSAL_DSL_TESTING_GUIDE.md`. Apply the same pattern to web integration tests.

### Test Structure
```
tests/
├── acceptance/          # Existing CLI tests (keep)
└── web/                 # NEW: Web integration tests
    ├── mod.rs
    ├── common.rs        # Web test DSL implementation
    └── dashboard_tests.rs  # Acceptance tests for dashboard
```

### Web Test DSL Example

Mirror the CLI test pattern for HTTP requests:

```rust
// Given-When-Then pattern
WebApp::given()
    .a_file_with_content("## TT 2025-01-15\n- #project-alpha 2h 30m")
    .when_get("/")
    .should_succeed()
    .expect_status(200)
    .expect_contains("project-alpha")
    .expect_contains("2h 30m");

// HTMX endpoint testing
WebApp::given()
    .a_file_with_content("...")
    .when_get("/api/dashboard")
    .with_query("period=this-week")
    .should_succeed()
    .expect_contains_tag("project-alpha")
    .expect_duration("project-alpha", "2h 30m");

// Tag detail view
WebApp::given()
    .a_file_with_content("...")
    .when_get("/api/tag/project-alpha")
    .should_succeed()
    .expect_entry_count(3)
    .expect_entry_with_description("Implemented feature X");
```

### Test DSL Implementation Pattern

**Setup Phase** - `WebAppSpec`:
```rust
pub struct WebAppSpec {
    input: Option<InputSource>,  // Reuse from CLI tests
    run_date: Option<NaiveDate>,
}

impl WebAppSpec {
    // Setup methods
    pub fn a_file_with_content(mut self, content: &str) -> Self { /* ... */ }
    pub fn a_directory_with_files(mut self, files: &[(&str, &str)]) -> Self { /* ... */ }
    pub fn at_date(mut self, date: &str) -> Self { /* ... */ }

    // Transition to action phase
    pub fn when_get(self, path: &str) -> RequestBuilder { /* ... */ }
    pub fn when_post(self, path: &str) -> RequestBuilder { /* ... */ }
}
```

**Action Phase** - `RequestBuilder`:
```rust
pub struct RequestBuilder {
    spec: WebAppSpec,
    method: Method,
    path: String,
    query: Option<String>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn with_query(mut self, query: &str) -> Self { /* ... */ }
    pub fn with_json_body(mut self, json: serde_json::Value) -> Self { /* ... */ }

    // Execute and transition to assertion phase
    pub async fn execute(self) -> WebAppResult { /* ... */ }

    // Shorthand (auto-execute)
    pub fn should_succeed(self) -> WebAppResult { /* ... */ }
    pub fn should_fail(self) -> WebAppResult { /* ... */ }
}
```

**Assertion Phase** - `WebAppResult`:
```rust
pub struct WebAppResult {
    response: Response,
    _temp_resources: Vec<TempResource>,  // Keep files alive
}

impl WebAppResult {
    // Basic assertions
    pub fn expect_status(self, code: u16) -> Self { /* ... */ }
    pub fn expect_contains(self, text: &str) -> Self { /* ... */ }
    pub fn expect_not_contains(self, text: &str) -> Self { /* ... */ }

    // Domain-specific assertions
    pub fn expect_contains_tag(self, tag: &str) -> Self { /* ... */ }
    pub fn expect_duration(self, tag: &str, duration: &str) -> Self { /* ... */ }
    pub fn expect_entry_count(self, count: usize) -> Self { /* ... */ }
}
```

### Test Coverage
- **Acceptance tests** (primary): Full server + real data files
  - Dashboard renders with real time-tracking data
  - Date range filtering works end-to-end
  - Tag drill-down displays correct entries
- **Template tests** (secondary): Askama template rendering
  - Data transformations feed templates correctly
  - HTML structure is valid
- **Handler tests** (minimal): Request/response logic

## Phase 1: MVP Dashboard

### Goal
Prove the architecture works with minimal scope. Deliver value quickly.

### Features

#### 1. Summary View (Home Page)
- **Endpoint**: `GET /`
- **Display**:
  - Total time tracked (default: this-week)
  - Time distribution by tags/projects
  - List of projects with durations and percentages
- **Domain Reuse**: `OverviewReport::overview()`

#### 2. Date Range Filtering
- **UI**: Simple buttons (Today / This Week / This Month)
- **Interactivity**: HTMX updates dashboard without full reload
- **Endpoint**: `GET /api/dashboard?period={period}`
  - Returns partial HTML (just the project list)
- **Domain Reuse**: `PeriodRequested` from CLI adapter

#### 3. Tag Detail View
- **Trigger**: Click on tag name in dashboard
- **Endpoint**: `GET /api/tag/{tag_name}`
- **Display**:
  - Individual entries for that tag
  - Format: date, duration, description, outcome
- **Domain Reuse**: `TrackedTime::tasks_tracked_for()`

### Implementation Steps

#### Step 1: Setup (Hardcoded Data)
1. Add dependencies to `Cargo.toml`
2. Create `src/web/` directory structure
3. Create basic Axum server with single route
4. Create Askama template with hardcoded data
5. Verify server runs and renders HTML

#### Step 2: Domain Integration
1. Add `#[derive(Serialize)]` to domain types:
   - `TimeEntry`, `Tag`, `Outcome`
   - `OverviewReport`, `TimeTotal`
   - `TrackedTime`
2. Create handler that:
   - Reads from file (reuse `parsing::process_input`)
   - Generates report (reuse `OverviewReport::overview`)
   - Passes to Askama template
3. Test with real markdown file

#### Step 3: Test DSL
1. Create `tests/web/common.rs` with DSL types
2. Implement `WebAppSpec`, `RequestBuilder`, `WebAppResult`
3. Write first acceptance test (dashboard renders)
4. Verify test passes with real data

#### Step 4: HTMX Filtering
1. Add date filter buttons to template
2. Create `/api/dashboard` endpoint returning partial HTML
3. Wire HTMX attributes (`hx-get`, `hx-target`)
4. Test filtering works without reload

#### Step 5: Tag Detail View
1. Make tag names clickable in dashboard
2. Create `/api/tag/{name}` endpoint
3. Create detail view template (partial HTML)
4. HTMX swaps in detail view
5. Test drill-down functionality

### Non-Goals (Phase 2+)
- Outcome analysis (separate phase)
- Historical trends/charts (separate phase)
- Data export (separate phase)
- Authentication (separate phase)
- Multi-user support (separate phase)
- Database storage (use files only)
- Mobile-first design (start desktop)

## Phase 2: Visualization (Future)

### Charts
- Chart.js for time distribution
- Bar chart: time per project
- Pie chart: percentage breakdown
- Pass data as JSON to template

### Example
```rust
#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    projects: Vec<TimeTotal>,
    chart_data: ChartData,  // JSON for Chart.js
}
```

## Phase 3: Advanced Features (Future)

- Breakdown by day/week/month (reuse `BreakdownReport`)
- Outcome tracking visualization
- Time trends over weeks/months
- Export to CSV/JSON
- Entry editing interface
- File upload capability

## Implementation Guidelines

### Development Approach
Follow the project's TDD methodology:
1. **Write failing test first** (Red)
2. **Implement minimal code** to pass (Green)
3. **Refactor** for clarity (Refactor)
4. **Commit** after each passing test

### Incremental Steps
For multi-step features:
1. Make smallest possible change within a category
2. Run tests immediately (`just test`)
3. Commit if tests pass, debug if they fail
4. Move to next category only after current is complete

### Code Style Requirements
- **No comments** unless explicitly needed
- **Functional approach**: Immutable transformations
- **Types as guardrails**: Encode rules in types
- **Error handling**: Use `Result`, avoid panics
- **Clippy clean**: Run `just run-clippy` after changes

### Domain Integrity
- **NEVER modify domain types for web concerns**
- Add `#[derive(Serialize)]` only (presentation concern)
- Keep HTTP logic in `web/` adapter only
- Reuse existing validation/error handling

## Askama Template Examples

### Base Template (`templates/base.html`)
```html
<!DOCTYPE html>
<html>
<head>
    <title>Time Tracker</title>
    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css">
    <script src="https://unpkg.com/htmx.org@1.9.10"></script>
</head>
<body>
    <main class="container">
        {% block content %}{% endblock %}
    </main>
</body>
</html>
```

### Dashboard Template (`templates/dashboard.html`)
```html
{% extends "base.html" %}

{% block content %}
<h1>Time Tracker Dashboard</h1>

<div id="filters">
    <button hx-get="/api/dashboard?period=today" hx-target="#report">Today</button>
    <button hx-get="/api/dashboard?period=this-week" hx-target="#report">This Week</button>
    <button hx-get="/api/dashboard?period=this-month" hx-target="#report">This Month</button>
</div>

<div id="report">
    {% include "components.html" %}
</div>
{% endblock %}
```

### Components Template (`templates/components.html`)
```html
<table>
    <thead>
        <tr>
            <th>Project</th>
            <th>Duration</th>
            <th>Percentage</th>
        </tr>
    </thead>
    <tbody>
        {% for project in projects %}
        <tr>
            <td>
                <a href="#" hx-get="/api/tag/{{ project.key }}" hx-target="#detail">
                    {{ project.key }}
                </a>
            </td>
            <td>{{ project.formatted_duration }}</td>
            <td>{{ project.percentage }}%</td>
        </tr>
        {% endfor %}
    </tbody>
</table>

<div id="detail"></div>
```

## Handler Implementation Examples

### Dashboard Handler
```rust
use axum::{extract::Query, response::Html};
use askama::Template;

#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate {
    projects: Vec<TimeTotal>,
    total_minutes: u32,
}

pub async fn dashboard(
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    // 1. Determine data file path (config or default)
    let data_path = Path::new("data/time-entries.md");

    // 2. Parse input (reuse parsing adapter)
    let tracking_result = parsing::process_input(data_path, None)?;

    // 3. Generate report (reuse domain)
    let time_entries = tracking_result.time_entries.unwrap();
    let period = params.period.unwrap_or("this-week".to_string());
    let period_requested = PeriodRequested::parse(&period)?;

    let overview = OverviewReport::overview(
        &time_entries,
        None,  // No limit
        Some(&period_requested),
    );

    // 4. Render template
    let template = DashboardTemplate {
        projects: overview.entries_total_time,
        total_minutes: overview.total_minutes,
    };

    Ok(Html(template.render()?))
}
```

### HTMX Partial Endpoint
```rust
#[derive(Template)]
#[template(path = "components.html")]
struct ProjectListPartial {
    projects: Vec<TimeTotal>,
}

pub async fn dashboard_partial(
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    // Same logic as above, but return only the partial
    // ...

    let template = ProjectListPartial {
        projects: overview.entries_total_time,
    };

    Ok(Html(template.render()?))
}
```

### Tag Detail Handler
```rust
#[derive(Template)]
#[template(path = "tag_detail.html")]
struct TagDetailTemplate {
    tag: String,
    entries: Vec<TimeEntry>,
    total_minutes: u32,
}

pub async fn tag_detail(
    Path(tag_name): Path<String>,
) -> Result<Html<String>, WebError> {
    // 1. Parse input
    let data_path = Path::new("data/time-entries.md");
    let tracking_result = parsing::process_input(data_path, None)?;

    // 2. Filter by tag (reuse domain)
    let time_entries = tracking_result.time_entries.unwrap();
    let tag = Tag::Context(tag_name.clone());
    let detail_report = time_entries.tasks_tracked_for(&[tag]);

    // 3. Render template
    let template = TagDetailTemplate {
        tag: tag_name,
        entries: detail_report.summaries[0].entries.clone(),
        total_minutes: detail_report.total_minutes,
    };

    Ok(Html(template.render()?))
}
```

## Success Criteria

### Functional
✅ Web server runs on localhost:3000
✅ Dashboard displays real time-tracking data from markdown files
✅ Date filtering (Today/This Week/This Month) works via HTMX
✅ Clicking tags shows detail view with individual entries
✅ All domain logic reused without modification
✅ Same data files work for both CLI and web interface

### Technical
✅ All tests pass (`just test`)
✅ Web integration tests follow DSL pattern
✅ Clippy clean (`just run-clippy`)
✅ Askama templates type-check at compile time
✅ No unsafe code
✅ Error handling preserves sources

### Architectural
✅ Web adapter is separate from domain
✅ Zero changes to existing domain types (except `Serialize`)
✅ Hexagonal architecture proven flexible
✅ Tests demonstrate ports & adapters pattern works

## Codebase Context

### Existing Architecture Strengths

The current codebase exhibits excellent separation of concerns:

1. **Domain Layer** (`src/domain/`): Pure business logic
   - `TimeEntry`: Core entity with parsing logic
   - `TrackedTime`: Aggregated time entries
   - `OverviewReport`, `DetailReport`, `BreakdownReport`: Report types
   - All functions are pure (no I/O, no side effects)

2. **Parsing Adapter** (`src/parsing/`): Input processing
   - Reads markdown files from filesystem
   - Supports single file or directory traversal
   - Applies filters during parsing
   - Returns `TimeTrackingResult` to domain

3. **Reporting Adapter** (`src/reporting/`): Output formatting
   - `Formatter` trait for multiple output formats
   - `TextFormatter`, `MarkdownFormatter` implementations
   - Easy to add `JsonFormatter` or `HtmlFormatter`

4. **CLI Adapter** (`src/cli/`): Command-line interface
   - Argument parsing with `clap`
   - Validation logic
   - Orchestrates domain operations

### Key Reusable Components

**For Web Interface:**
- ✅ `parsing::process_input()` - Read and parse markdown files
- ✅ `OverviewReport::overview()` - Generate summary reports
- ✅ `TrackedTime::tasks_tracked_for()` - Detail views by tag
- ✅ `BreakdownReport::from_tracked_time()` - Time breakdowns
- ✅ `PeriodRequested` - Date range handling
- ✅ `Filter` - Tag/date filtering logic

### Test DSL Pattern

The codebase has a sophisticated fluent test DSL (`tests/acceptance/common.rs`, ~546 lines):

**Pattern**: Given-When-Then with method chaining
```rust
Cmd::given()              // Setup
    .a_file_with_content("...")
    .tags_filter(&["project"])
    .when_run()           // Execute
    .should_succeed()     // Assert
    .expect_project("name")
        .taking("2h 30m")
        .with_percentage("60")
    .validate();
```

**Key Types**:
- `CommandSpec`: Setup phase builder
- `CommandResult`: Assertion phase
- `ProjectAssertion`: Chained domain assertions
- `InputSource`: File/directory abstraction

Apply this exact pattern to web tests for consistency.

## References

- **Architecture Guide**: `/docs/UNIVERSAL_DSL_TESTING_GUIDE.md`
- **Project README**: `/README.md`
- **Justfile**: Development commands (`just test`, `just run-clippy`)
- **Test Examples**: `/tests/acceptance/` (general.rs, tags.rs, periods.rs)

## Notes

- This is a **hobby project** but should maintain production-quality standards
- Emphasis on **clean architecture** and **testability**
- Start simple, avoid over-engineering
- Prioritize **reusability** of domain logic
- Keep **CLI and web separate** but sharing core library
