# Time Tracker Web - Developer Onboarding Guide

Welcome to the Time Tracker web application! This guide will walk you through the web architecture, HTMX patterns, and how everything fits together. This is inherited code, so we'll focus on understanding what exists and how to maintain and extend it.

## Table of Contents
1. [Quick Start](#quick-start)
2. [Understanding HTMX](#understanding-htmx)
3. [Architecture Overview](#architecture-overview)
4. [Screen Components](#screen-components)
5. [Backend-Frontend Interaction](#backend-frontend-interaction)
6. [Adding New Features](#adding-new-features)
7. [Testing](#testing)
8. [Known Issues & Improvements](#known-issues--improvements)

---

## Quick Start

### Running the Web Server

```bash
# Simple run with default data path
just run-web path/to/time-entries.md

# With auto-reload during development
just web-w -i path/to/time-entries.md

# Manual cargo command (if you prefer)
cargo run --features web -- --web -i path/to/time-entries.md
```

The server will start on `http://localhost:3000` by default.

### Running Tests

```bash
# All web tests
just test-web

# Single test
cargo nextest run test_name

# With watch mode
just test-w
```

---

## Understanding HTMX

### What is HTMX?

HTMX is a library that lets you access modern browser features (AJAX, CSS Transitions, WebSockets) directly from HTML, without writing JavaScript. Instead of writing JS event handlers, you use HTML attributes to describe behavior.

**Traditional approach:**
```javascript
// JavaScript
document.getElementById('myButton').addEventListener('click', function() {
    fetch('/api/data')
        .then(response => response.text())
        .then(html => {
            document.getElementById('target').innerHTML = html;
        });
});
```

**HTMX approach:**
```html
<!-- Just HTML attributes -->
<button hx-get="/api/data" hx-target="#target">Load Data</button>
<div id="target"></div>
```

### Key HTMX Attributes

#### Core Attributes

**`hx-get`, `hx-post`, `hx-put`, `hx-delete`**
- Specifies which HTTP verb to use and the URL to request
- Example: `hx-get="/api/dashboard"` - fetches data from this endpoint

**`hx-target`**
- Specifies where the response HTML should be inserted
- Example: `hx-target="#projects"` - puts response into the element with id="projects"
- Can use CSS selectors: `#id`, `.class`, `closest div`, etc.

**`hx-trigger`**
- Specifies what event triggers the request
- Default: `click` for buttons, `submit` for forms
- Example: `hx-trigger="change"` - triggers on value change (for inputs)

**`hx-include`**
- Includes additional form data in the request
- Example: `hx-include="#filter-form"` - includes all inputs from the filter form

**`hx-swap`**
- Controls how response is swapped into the target
- Values: `innerHTML` (default), `outerHTML`, `beforebegin`, `afterend`, etc.

**`hx-swap-oob="true"` (Out-of-Band)**
- Updates multiple parts of the page with a single response
- The response includes elements with matching IDs that get swapped automatically
- This is a powerful pattern used throughout this app

### HTMX in Action - Example from Dashboard

Let's break down a real example from `templates/dashboard.html`:

```html
<button
    type="button"
    hx-get="/api/dashboard"
    hx-target="#projects"
    hx-include="#filter-form"
    onclick="document.getElementById('selected-period').value = 'today'">
    Today
</button>
```

**What happens when you click this button:**

1. JavaScript `onclick` sets the hidden input value to "today"
2. HTMX intercepts the click and makes GET request to `/api/dashboard`
3. HTMX includes all form fields from `#filter-form` as query parameters
4. Backend responds with HTML partial (not JSON!)
5. HTMX swaps response into the `#projects` div
6. Chart and summary get updated via out-of-band swaps (see below)

### Out-of-Band Swaps - A Key Pattern

This app uses a powerful HTMX feature called "out-of-band swaps" extensively.

**Example from** `templates/dashboard_combined_partial.html`:

```html
<!-- Main content (goes into hx-target) -->
<table>
    <tr><td>Project A</td><td>100 min</td></tr>
</table>

<!-- Out-of-band updates (automatically swap into matching IDs) -->
<div id="summary" hx-swap-oob="true">
    <h2>Total Time Tracked: <strong>5h 30m</strong></h2>
</div>

<div id="chart-pie" hx-swap-oob="true">
    <canvas id="projects-pie-chart"></canvas>
</div>
```

**What this means:**
- The table goes into the `hx-target` (wherever the request specified)
- The `#summary` div automatically updates the summary section
- The `#chart-pie` div automatically updates the chart
- All from ONE server response!

This is how filtering updates the projects list, summary, AND chart simultaneously.

---

## Architecture Overview

### Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| **Web Framework** | Axum v0.7 | Async HTTP server, routing |
| **Async Runtime** | Tokio | Handles async/await |
| **Templates** | Askama v0.12 | Type-safe compile-time HTML templates |
| **Frontend** | HTMX v1.9.10 | Dynamic interactions without JavaScript |
| **Styling** | PicoCSS v2 | Classless CSS framework |
| **Charts** | Chart.js v4.4.1 | Pie chart visualizations |

### Directory Structure

```
src/web/
├── mod.rs              # Module exports
├── server.rs           # Route definitions
├── handlers.rs         # Request handlers & templates
└── models.rs           # Query parameter models

templates/
├── base.html                          # Base layout (extends into pages)
├── dashboard.html                     # Main dashboard page
├── outcomes.html                      # Outcomes analysis page
├── flag_statistics.html               # CLI stats page
├── tag_detail.html                    # Project detail modal
├── dashboard_combined_partial.html    # Dashboard updates
├── outcomes_combined_partial.html     # Outcomes updates
└── chart_*.html                       # Chart partials

tests/web/
├── common.rs           # Test utilities & DSL
├── dashboard_tests.rs  # Dashboard tests
├── outcomes_tests.rs   # Outcomes tests
└── ...
```

### Request Flow

```
Browser
   ↓ (HTMX makes request)
Axum Router (server.rs)
   ↓ (routes to handler)
Handler Function (handlers.rs)
   ↓ (spawns blocking task)
Data Processing (domain layer)
   ↓ (returns data)
Template Rendering (Askama)
   ↓ (renders HTML)
HTTP Response
   ↓ (HTMX swaps into DOM)
Browser (updated!)
```

---

## Screen Components

### 1. Dashboard (`/` route)

**Purpose:** Main view showing time tracked by project with filtering

**Key Features:**
- Total time summary
- Project list with clickable links
- Period filters (Today, This Week, This Month, All Time)
- Custom date range picker
- Limit to top 90% toggle
- Pie chart visualization
- Tag detail drill-down

**Template:** `templates/dashboard.html`
**Handler:** `handlers::dashboard`
**Partial API:** `/api/dashboard` → `handlers::dashboard_partial`

**HTMX Patterns Used:**
```html
<!-- Period filter buttons -->
<button hx-get="/api/dashboard" hx-target="#projects" hx-include="#filter-form">
    Today
</button>

<!-- Date inputs with auto-update -->
<input type="date" name="from"
    hx-trigger="change"
    hx-get="/api/dashboard"
    hx-target="#projects"
    hx-include="#filter-form">

<!-- Clickable project links that load detail -->
<a href="#"
    hx-get="/api/tag/{{ project.description }}"
    hx-target="#detail"
    hx-include="#filter-form">
    {{ project.description }}
</a>
```

**Out-of-Band Updates:**
When you filter, the response updates THREE areas:
1. `#projects` - the main target (project list)
2. `#summary` - total time (via hx-swap-oob)
3. `#chart-pie` - pie chart (via hx-swap-oob)

### 2. Outcomes Page (`/outcomes` route)

**Purpose:** Shows time tracked by outcome (Done, Wip, Blocked, etc.)

**Key Features:**
- Similar to dashboard but groups by outcome instead of project
- Same filtering capabilities
- Separate pie chart for outcomes

**Template:** `templates/outcomes.html`
**Handler:** `handlers::outcomes_page`
**Partial API:** `/api/outcomes` → `handlers::outcomes_partial`

**Note:** This page duplicates a lot of dashboard code. This is a known area for improvement.

### 3. Flag Statistics Page (`/flag-statistics` route)

**Purpose:** Shows CLI usage statistics (which flags were used, success rates)

**Key Features:**
- Total/successful/failed execution counts
- Per-flag usage statistics
- Filter flags by include/exclude patterns

**Template:** `templates/flag_statistics.html`
**Handler:** `handlers::flag_statistics`
**Partial API:** `/api/flag-statistics` → `handlers::flag_statistics_partial`

**HTMX Pattern:**
```html
<!-- Form with auto-submit on change -->
<form hx-get="/api/flag-statistics" hx-target="#flags-table" hx-trigger="change">
    <input type="text" name="include" placeholder="Include flags...">
    <input type="text" name="exclude" placeholder="Exclude flags...">
</form>
```

### 4. Tag Detail Modal (`/api/tag/:tag_name` partial)

**Purpose:** Shows detailed breakdown of time entries for a specific tag/project

**Template:** `templates/tag_detail.html`
**Handler:** `handlers::tag_detail`

**HTMX Pattern:**
```html
<!-- Loaded dynamically into #detail div when project link clicked -->
<div id="detail"></div>
```

**Security Note:** Tag validation is performed in `handlers::is_valid_tag()` to prevent injection attacks.

---

## Backend-Frontend Interaction

### How Data Flows

#### 1. Initial Page Load

```
GET /
  ↓
handlers::dashboard()
  ↓ (reads data file)
  ↓ (processes all time entries)
  ↓ (renders full dashboard.html template)
  ↓
Returns complete HTML page
```

#### 2. Filter Interaction

```
User clicks "Today" button
  ↓ (onclick sets period=today)
  ↓ (hx-get triggered)
GET /api/dashboard?period=today
  ↓
handlers::dashboard_partial()
  ↓ (extracts query params via DashboardParams)
  ↓ (creates filter from period)
  ↓ (spawns blocking task for I/O)
  ↓ (processes filtered entries)
  ↓ (renders dashboard_combined_partial.html)
  ↓
Returns HTML with:
  - <table> (goes into hx-target)
  - <div id="summary" hx-swap-oob> (updates summary)
  - <div id="chart-pie" hx-swap-oob> (updates chart)
```

### Query Parameters

Defined in `src/web/models.rs`:

**DashboardParams:**
```rust
struct DashboardParams {
    period: Option<String>,    // "today", "this-week", "this-month"
    limit: Option<bool>,       // true = limit to top 90%
    from: Option<String>,      // Custom date range start (YYYY-MM-DD)
    to: Option<String>,        // Custom date range end (YYYY-MM-DD)
}
```

**FlagStatsParams:**
```rust
struct FlagStatsParams {
    include: Option<String>,   // Comma-separated flags to include
    exclude: Option<String>,   // Comma-separated flags to exclude
}
```

### Blocking I/O Pattern

The app uses `tokio::task::spawn_blocking` for file I/O operations:

```rust
pub async fn dashboard_partial(...) -> Result<Html<String>, WebError> {
    // Clone data needed for blocking task
    let data_path = state.data_path.clone();

    // Spawn blocking task (moves to thread pool)
    let tracking_result = tokio::task::spawn_blocking(move || {
        parsing::process_input(&data_path, filter.as_ref())
    })
    .await  // Wait for completion
    .map_err(|e| WebError::DataProcessingFailed(...))?
    .map_err(|e| WebError::DataProcessingFailed(...))?;

    // Continue with async work...
}
```

**Why?** File I/O is synchronous and would block the async runtime. Spawn_blocking moves it to a thread pool.

### Template Rendering with Askama

Askama compiles templates at build time for type safety:

```rust
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub total_time: String,
    pub projects: Vec<TimeTotal>,
}

// In handler
let template = DashboardTemplate {
    total_time: format_minutes(overview.total_minutes()),
    projects: overview.entries_time_totals().clone(),
};

let html = template.render()
    .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
Ok(Html(html))
```

**Template syntax** (in `dashboard.html`):
```html
<h2>Total: {{ total_time }}</h2>

{% for project in projects %}
    <tr>
        <td>{{ project.description }}</td>
        <td>{{ project.minutes }} min</td>
    </tr>
{% endfor %}
```

### Error Handling

Custom error type in `handlers.rs`:

```rust
pub enum WebError {
    DataProcessingFailed(String),
    TemplateRenderFailed(String),
    InvalidTag(String),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        // Logs error, returns user-friendly message
        (StatusCode::INTERNAL_SERVER_ERROR, Html("<p>Error message</p>"))
    }
}
```

---

## Adding New Features

### Example: Adding a New Filter Type

Let's say you want to add a "Last 7 Days" filter button.

#### Step 1: Update the Template

`templates/dashboard.html`:
```html
<button type="button"
    hx-get="/api/dashboard"
    hx-target="#projects"
    hx-include="#filter-form"
    onclick="document.getElementById('selected-period').value = 'last-7-days'">
    Last 7 Days
</button>
```

#### Step 2: Handle in Backend

The backend already handles arbitrary period strings through `PeriodRequested::from_str()`. If "last-7-days" is already supported there, you're done! If not:

`src/domain/period.rs` (or wherever PeriodRequested is defined):
```rust
impl PeriodRequested {
    pub fn from_str(s: &str, clock: &Clock) -> Result<Self> {
        match s {
            "today" => Ok(Self::Today),
            "this-week" => Ok(Self::ThisWeek),
            "this-month" => Ok(Self::ThisMonth),
            "last-7-days" => Ok(Self::Last7Days),  // Add this
            _ => Err(...)
        }
    }
}
```

#### Step 3: Test It

`tests/web/dashboard_tests.rs`:
```rust
#[tokio::test]
async fn dashboard_filters_by_last_7_days() {
    given_time_tracker_with_entries("...")
        .when_viewing_dashboard()
        .with_param("period", "last-7-days")
        .then_response_contains("Expected Project");
}
```

### Example: Adding a New Page

Let's say you want to add a "Weekly Summary" page.

#### Step 1: Create Route

`src/web/server.rs`:
```rust
pub fn create_router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/weekly-summary", get(handlers::weekly_summary))  // Add this
        .route("/api/weekly-summary", get(handlers::weekly_summary_partial))  // Add this
        // ...
}
```

#### Step 2: Create Handler

`src/web/handlers.rs`:
```rust
#[derive(Template)]
#[template(path = "weekly_summary.html")]
pub struct WeeklySummaryTemplate {
    pub weeks: Vec<WeekData>,
}

pub async fn weekly_summary(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, WebError> {
    // Similar to dashboard handler
    let template = WeeklySummaryTemplate {
        weeks: /* process data */,
    };

    let html = template.render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}

pub async fn weekly_summary_partial(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    // Partial update version
}
```

#### Step 3: Create Templates

`templates/weekly_summary.html`:
```html
{% extends "base.html" %}

{% block content %}
<h1>Weekly Summary</h1>

<div id="weeks-list">
    {% for week in weeks %}
        <div>{{ week.start_date }} - {{ week.end_date }}: {{ week.total }}</div>
    {% endfor %}
</div>
{% endblock %}
```

#### Step 4: Add Navigation

Update `templates/dashboard.html`:
```html
<nav>
    <a href="/outcomes">View Outcomes →</a>
    <a href="/flag-statistics">View Flag Statistics →</a>
    <a href="/weekly-summary">Weekly Summary →</a>  <!-- Add this -->
</nav>
```

#### Step 5: Write Tests

`tests/web/weekly_summary_tests.rs`:
```rust
#[tokio::test]
async fn weekly_summary_shows_current_week() {
    given_time_tracker_with_entries("...")
        .when_viewing_page("/weekly-summary")
        .then_response_contains("Current Week");
}
```

---

## Testing

### Test Structure

The web tests use a custom DSL (Domain-Specific Language) defined in `tests/web/common.rs`:

```rust
#[tokio::test]
async fn dashboard_shows_total_time() {
    given_time_tracker_with_entries("
        2024-01-01
        09:00-10:00 project-a
    ")
    .when_viewing_dashboard()
    .then_response_contains("60 min");
}
```

### WebApp Test DSL

**Given (Setup):**
```rust
given_time_tracker_with_entries("...markdown...")
    // Creates temp file, starts test server
```

**When (Action):**
```rust
.when_viewing_dashboard()
    // Makes GET request to /

.when_viewing_page("/outcomes")
    // Makes GET request to /outcomes

.with_param("period", "today")
    // Adds query parameter
```

**Then (Assertion):**
```rust
.then_response_contains("Expected Text")
    // Checks response body contains text

.then_response_is_ok()
    // Checks status 200

.then_json_contains(r#"{"key": "value"}"#)
    // Checks JSON response
```

### Running Specific Tests

```bash
# All web tests
cargo nextest run --features web

# Single test
cargo nextest run dashboard_shows_total_time

# With output
cargo nextest run dashboard_shows_total_time --no-capture
```

### Testing HTMX Interactions

Since HTMX is client-side, you primarily test the server responses:

```rust
#[tokio::test]
async fn filtering_by_period_updates_data() {
    given_time_tracker_with_entries("
        2024-01-01
        09:00-10:00 old-project

        2024-12-01
        09:00-10:00 new-project
    ")
    .when_viewing_page("/api/dashboard")
    .with_param("period", "this-month")
    .then_response_contains("new-project")
    .then_response_does_not_contain("old-project");
}
```

---

## Known Issues & Improvements

### Code Duplication

**Issue:** Dashboard and Outcomes pages share 80% of the same code.

**Current state:**
- `dashboard.html` and `outcomes.html` have nearly identical filter sections
- `dashboard_partial` and `outcomes_partial` have nearly identical logic

**Possible improvements:**
- Extract shared filter component into reusable partial
- Create shared handler logic for filtering
- Consider a single configurable page with view toggle

**Where to look:**
- `templates/dashboard.html` lines 50-78 (filters)
- `templates/outcomes.html` lines 49-77 (nearly identical)
- `handlers.rs` - compare `dashboard_partial` and `outcomes_partial`

### Inline JavaScript in Templates

**Issue:** Chart initialization and state management uses inline `<script>` tags.

**Current state:**
- Chart.js setup embedded in template files
- onclick handlers for setting filter values
- Global state in `window.projectsChart`

**Examples:**
- `templates/dashboard.html` lines 120-183
- `templates/dashboard.html` line 54: `onclick="document.getElementById('selected-period').value = 'today'"`

**Possible improvements:**
- Extract to separate JS file
- Use HTMX events for state management
- Consider Alpine.js for reactivity if complexity grows

### No Loading States

**Issue:** When filtering large datasets, there's no visual feedback during request.

**Current state:**
- User clicks filter → delay → content updates
- No spinner or loading indicator

**HTMX solution:**
```html
<!-- Add loading class during request -->
<div hx-get="/api/dashboard"
     hx-indicator="#loading-spinner">
    ...
</div>

<div id="loading-spinner" class="htmx-indicator">
    Loading...
</div>
```

**Where to add:**
- `templates/dashboard.html` filter buttons
- `templates/outcomes.html` filter buttons

### Filter State Not in URL

**Issue:** Filters are not reflected in URL, so:
- Can't bookmark filtered view
- Browser back button doesn't work as expected
- Can't share filtered URL

**Current state:**
- Period stored in hidden input field
- Date range in form inputs
- No URL updates

**HTMX solution:**
```html
<!-- Push state to URL -->
<button hx-get="/api/dashboard"
        hx-push-url="true"
        hx-target="#projects">
    Today
</button>
```

**Considerations:**
- Need to handle initial page load with URL params
- May need to coordinate with hidden input state

### Chart Updates Don't Handle Edge Cases

**Issue:** Chart initialization doesn't handle all scenarios gracefully.

**Current state:**
- Chart destroyed and recreated on each update (good)
- No handling for empty data sets (chart errors?)
- No error boundaries if Chart.js fails to load

**Where to improve:**
- `templates/dashboard_combined_partial.html` lines 39-101
- Add checks for empty datasets
- Add fallback if Chart.js CDN unavailable

### Limited Error Messages

**Issue:** Errors shown to users are generic.

**Current state:**
```rust
WebError::DataProcessingFailed(msg) => {
    (StatusCode::INTERNAL_SERVER_ERROR, "Error loading data".to_string())
}
```

**Possible improvements:**
- More specific user-facing messages
- Suggestions for common issues (file not found, parse errors)
- Styled error components instead of plain `<p>` tags

### No Pagination

**Issue:** Large datasets load all results at once.

**Current state:**
- All projects loaded and rendered
- Limit to 90% helps but not configurable
- No infinite scroll or pagination

**Considerations:**
- Time tracking data usually isn't that large
- May not be worth the complexity
- Consider lazy loading for tag detail view

---

## Development Tips

### 1. Use Auto-Reload for Fast Iteration

```bash
just web-w -i data/time-entries.md
```

This watches for changes and restarts the server automatically.

### 2. Check Your Changes in Multiple Browsers

HTMX is well-supported, but always test in:
- Chrome/Edge (Chromium)
- Firefox
- Safari (if on Mac)

### 3. Use Browser DevTools Network Tab

When debugging HTMX:
1. Open DevTools → Network tab
2. Click a filter button
3. Look for the XHR request to `/api/dashboard`
4. Check the response HTML
5. Verify it has the expected structure

### 4. Test with Different Data

Try edge cases:
- Empty data file
- Single entry
- Thousands of entries
- Entries with special characters in tags
- Date ranges with no data

### 5. Read HTMX Docs

The official HTMX docs are excellent:
- https://htmx.org/docs/
- https://htmx.org/examples/

Similar patterns used in this app:
- Click to Load (tag detail)
- Active Search (could be added for filtering)
- Infinite Scroll (could be added for pagination)

### 6. Use Clippy

Always run before committing:
```bash
just run-clippy
```

This project uses clippy as a functional programming guide.

### 7. Follow the TDD Approach

The project guidelines emphasize TDD:
1. Write failing test
2. Implement minimal code
3. Make it pass
4. Refactor

Example workflow:
```bash
# Write test in tests/web/dashboard_tests.rs
cargo nextest run new_test_name  # Fails

# Implement in handlers.rs
cargo nextest run new_test_name  # Passes

# Refactor if needed
just run-clippy
```

---

## Glossary

**Axum** - Async web framework built on Tokio, similar to Express.js (Node) or Flask (Python)

**HTMX** - Library for building dynamic UIs with HTML attributes instead of JavaScript

**Askama** - Template engine that compiles templates at build time (like Jinja2 or Handlebars)

**Tokio** - Async runtime for Rust, enables async/await (like asyncio in Python)

**Out-of-Band Swap** - HTMX feature that updates multiple page sections from one response

**Partial** - Template fragment rendered without full page layout (for AJAX updates)

**Spawn Blocking** - Moving synchronous I/O to thread pool to avoid blocking async runtime

**TDD** - Test-Driven Development (write tests first, then implement)

**Clippy** - Rust linter that suggests functional programming improvements

---

## Next Steps

Now that you understand the architecture:

1. **Explore the code** - Read through `handlers.rs` and the templates
2. **Run the app** - Try different filters and interactions
3. **Make a small change** - Add a new button or text field
4. **Write a test** - Use the test DSL to verify your change
5. **Read the domain layer** - Understanding the business logic helps

### Recommended Reading Order

1. `src/web/server.rs` - See all routes
2. `templates/base.html` - Understand the layout
3. `templates/dashboard.html` - See HTMX in action
4. `src/web/handlers.rs` - One handler at a time
5. `tests/web/dashboard_tests.rs` - See testing patterns

### Questions to Explore

- How would you add real-time updates (WebSocket)?
- Could you replace the onclick handlers with pure HTMX?
- How would you add user authentication?
- What if you wanted to persist filter preferences?

---

## Getting Help

- **HTMX Docs:** https://htmx.org/docs/
- **Axum Examples:** https://github.com/tokio-rs/axum/tree/main/examples
- **Askama Guide:** https://djc.github.io/askama/
- **Project Guidelines:** See `CLAUDE.md` in repo root

Welcome to the team! This codebase isn't perfect, but it's maintainable and testable. Focus on understanding the patterns, and you'll be productive quickly.
