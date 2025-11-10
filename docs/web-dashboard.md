# Web Dashboard Documentation

The Time Tracker web dashboard provides an interactive browser-based interface for viewing and analyzing your time tracking data. Built with Axum, HTMX, and Askama, it demonstrates the flexibility of the hexagonal architecture by adding a new adapter without modifying the core domain logic.

## Quick Start

### Running the Web Server

```bash
# Using just (recommended)
just web

# Using cargo directly
cargo run --bin tt-web

# With auto-reload during development
just web-w
```

The server will start at **http://127.0.0.1:3000**

### Building for Production

```bash
# Build optimized binary
just build-web

# Run the optimized binary
./target/release/tt-web
```

## Configuration

### Data File Path

By default, the web dashboard looks for time tracking data in the same location as the CLI. Currently, the server starts with no default data path (displays hardcoded example data).

To configure a data file path, you'll need to modify `src/bin/tt-web.rs` to pass an `AppState` with your data path:

```rust
use time_tracker::web::{self, AppState};
use std::sync::Arc;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        data_path: Some(PathBuf::from("./data/time-entries.md")),
    });

    let app = web::server::create_router_with_state(state);

    // ... rest of server setup
}
```

### Server Configuration

**Port and Address:**
Default: `127.0.0.1:3000`

To change the port, modify `src/bin/tt-web.rs`:

```rust
web::run_server("127.0.0.1:8080").await; // Custom port
```

**Environment Variables:**
- `TT_TODAY` - Override current date for testing (format: YYYY-MM-DD)

## Features

### Dashboard View

The main dashboard (`/`) displays:

- **Total Time Tracked** - Aggregate time across all entries
- **Time by Project** - Table showing:
  - Project/tag name (clickable for drill-down)
  - Duration in minutes
  - Percentage of total time
- **Period Filters** - Interactive buttons for:
  - Today
  - This Week
  - This Month
  - All Time

### Interactive Filtering

Click any filter button to update the project list dynamically via HTMX:

- Filters are applied at the parsing layer (same logic as CLI)
- Projects list updates without full page reload
- Uses the same `PeriodRequested` and `Filter` types as CLI

**Implementation:**
- Endpoint: `GET /api/dashboard?period={period}`
- Returns: HTML partial (projects table only)
- HTMX swaps content in `#projects` div

### Tag Detail View

Click any project name to see individual time entries:

- Shows all entries tagged with that project
- Displays entry description and duration
- Calculates total time for the tag
- Opens below the projects table

**Implementation:**
- Endpoint: `GET /api/tag/{tag_name}`
- Returns: HTML partial (entry detail table)
- HTMX loads into `#detail` div

## File Format

The web dashboard reads the same markdown format as the CLI:

```markdown
## TT 2025-01-15
- #project-alpha 2h 30m Building dashboard
- #project-beta 1h 15m Code review
- #project-alpha #frontend 45m UI polish

## TT 2025-01-16
- #project-beta 3h Testing and bug fixes
```

**Format Requirements:**
- Date headers: `## TT YYYY-MM-DD`
- Time entries: `- #tag duration description`
- Tags: Start with `#`, can have multiple tags per entry
- Duration: Hours (`h`), minutes (`m`), or pomodoros (`p`)

## Architecture

### Hexagonal Architecture

The web dashboard demonstrates the ports & adapters pattern:

```
┌─────────────────────────────────────────┐
│           Web Adapter Layer             │
│  (HTTP handlers, templates, routing)    │
└─────────────────┬───────────────────────┘
                  │
                  ├→ Uses
                  │
┌─────────────────▼───────────────────────┐
│         Domain Layer (Pure Logic)       │
│  - TrackedTime, TimeEntry, Outcome      │
│  - OverviewReport, DetailReport         │
│  - PeriodRequested, DateRange           │
└─────────────────┬───────────────────────┘
                  │
                  ├→ Uses
                  │
┌─────────────────▼───────────────────────┐
│        Parsing Adapter Layer            │
│   (File I/O, markdown parsing)          │
└─────────────────────────────────────────┘
```

**Key Principles:**
- Domain layer has zero HTTP dependencies
- Same domain logic powers both CLI and web
- Both adapters share parsing and reporting layers
- Only `Serialize` derives added for web (presentation concern)

### Tech Stack

**Backend:**
- **Axum** - Async web framework
- **Tokio** - Async runtime
- **Askama** - Type-safe compile-time templates
- **Tower** - Service middleware

**Frontend:**
- **HTMX** - Dynamic interactions without JavaScript
- **Pico CSS** - Minimal, semantic CSS framework
- **Chart.js** - (Future) For visualizations

### Request Flow

**Main Dashboard:**
1. Browser requests `GET /`
2. `dashboard()` handler reads data file via `parsing::process_input()`
3. Domain creates `OverviewReport` with aggregated data
4. Askama renders `dashboard.html` with data
5. HTML+CSS+HTMX returned to browser

**Filtered Dashboard:**
1. User clicks "This Week" button
2. HTMX sends `GET /api/dashboard?period=this-week`
3. Handler parses period string with `PeriodRequested::from_str()`
4. Creates `Filter::DateRange()` and passes to parsing
5. Parsing layer filters entries by date
6. OverviewReport aggregates filtered data
7. Askama renders `projects_partial.html`
8. HTMX swaps HTML into `#projects` div

**Tag Detail:**
1. User clicks project name
2. HTMX sends `GET /api/tag/project-alpha`
3. Handler uses `TrackedTime::tasks_tracked_for()` from domain
4. Gets individual entries for that tag
5. Askama renders `tag_detail.html`
6. HTMX loads HTML into `#detail` div

## Development

### Project Structure

```
src/
├── bin/
│   └── tt-web.rs           # Web server entry point
├── web/
│   ├── mod.rs              # Module exports
│   ├── server.rs           # Axum server & routing
│   ├── handlers.rs         # HTTP request handlers
│   └── models.rs           # Request/response types
templates/
├── base.html               # Base layout with CSS/HTMX
├── dashboard.html          # Main dashboard view
├── projects_partial.html   # Project list (HTMX partial)
└── tag_detail.html         # Tag detail view (HTMX partial)
tests/web/
├── common.rs               # Web test DSL
├── dashboard_tests.rs      # Dashboard acceptance tests
└── tag_detail_tests.rs     # Detail view tests
```

### Running Tests

```bash
# Run all tests (CLI + web)
just test

# Run only web tests
just test-web

# Run with coverage
just test-coverage
```

**Test DSL Pattern:**

The web tests follow the same fluent DSL pattern as CLI tests:

```rust
WebApp::given()
    .a_file_with_content("## TT 2025-01-15\n- #project 2h Work")
    .at_date("2025-01-15")
    .when_get("/api/dashboard")
    .with_query("period=today")
    .should_succeed()
    .await
    .expect_status(200)
    .expect_contains("project")
    .expect_contains("120 min");
```

### Adding New Endpoints

1. **Create Handler** in `src/web/handlers.rs`:
```rust
pub async fn my_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    // Use domain logic
    let data = /* ... */;

    // Render template
    let template = MyTemplate { data };
    Html(template.render().expect("Failed to render"))
}
```

2. **Create Template** in `templates/my_template.html`:
```html
<h2>{{ title }}</h2>
<div>{{ content }}</div>
```

3. **Add Route** in `src/web/server.rs`:
```rust
pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/my-path", get(handlers::my_handler))
        .with_state(state)
}
```

4. **Write Test** in `tests/web/`:
```rust
#[tokio::test]
async fn my_endpoint_should_work() {
    WebApp::given()
        .when_get("/my-path")
        .should_succeed()
        .await
        .expect_contains("expected content");
}
```

### Code Quality

**Pre-commit Checks:**
```bash
# Format code
just fmt

# Run linter
just run-clippy

# Run tests
just test
```

**Standards:**
- All handlers must have tests
- Templates compile at build time (type-safe)
- No comments unless explicitly needed
- Functional programming style (immutable transformations)
- Error handling with Result types

## Troubleshooting

### Server Won't Start

**Port Already in Use:**
```
Error: Address already in use (os error 48)
```
Solution: Change port in `src/bin/tt-web.rs` or kill process on port 3000

**Data File Not Found:**
The server will show hardcoded example data if no data path is configured. Configure `AppState` with your data file path.

### Tests Failing

**TT_TODAY Not Respected:**
Ensure handlers read `std::env::var("TT_TODAY")` and create `Clock::with_today()` for deterministic behavior.

**Template Compilation Errors:**
Askama validates templates at compile time. Check:
- Template syntax is correct
- All variables in template are provided by handler
- Template files are in `templates/` directory

### Performance

**Slow Parsing:**
- Consider caching parsed data in `AppState`
- Use period filters to reduce data processing
- Profile with `cargo flamegraph --bin tt-web`

**High Memory Usage:**
- Large markdown files are parsed on every request
- Future: Implement file watching and cache invalidation
- Future: Add database backend for frequently accessed data

## Future Enhancements

### Planned Features

**Phase 3 - Visualizations:**
- Chart.js time distribution graphs
- Bar chart: time per project
- Pie chart: percentage breakdown
- Time trends over weeks/months

**Phase 4 - Advanced Features:**
- Outcome analysis display
- Entry editing interface
- File upload capability
- CSV/JSON export
- Multiple file support

**Configuration:**
- Config file support (TOML/YAML)
- Environment variable configuration
- CLI flags for server startup
- Data directory watching

**Authentication:**
- User accounts (future consideration)
- Session management
- Access control

## API Reference

### Endpoints

**`GET /`**
- Returns: Full dashboard HTML
- Query params: None
- Content-Type: text/html

**`GET /api/dashboard`**
- Returns: Projects list HTML partial
- Query params:
  - `period` (optional): today, this-week, this-month, last-week, etc.
- Content-Type: text/html
- HTMX: Updates `#projects` div

**`GET /api/tag/:tag_name`**
- Returns: Tag detail HTML partial
- Path params:
  - `tag_name`: URL-encoded tag name
- Content-Type: text/html
- HTMX: Updates `#detail` div

### Response Formats

All endpoints return HTML (server-side rendering). No JSON API currently.

For programmatic access, use the CLI with `--format markdown` or implement a JSON formatter.

## Contributing

When adding web features:

1. Follow TDD - Write failing test first
2. Implement minimal code to pass
3. Refactor for clarity
4. Ensure clippy clean
5. Update this documentation

See `docs/testing-guidelines.md` for testing standards.

## References

- [Main README](../README.md) - Project overview
- [Testing Guide](./UNIVERSAL_DSL_TESTING_GUIDE.md) - DSL patterns
- [CLAUDE.md](../CLAUDE.md) - Development guidelines
- [Axum Documentation](https://docs.rs/axum)
- [Askama Documentation](https://docs.rs/askama)
- [HTMX Documentation](https://htmx.org)
