# Universal CLI/Server Test DSL Design Guide

This guide demonstrates how to build fluent, behavior-driven test DSLs that make acceptance tests read like executable specifications. The patterns work for both CLI applications and server applications.

## Core Architecture Pattern

All effective test DSLs follow a three-phase structure:

1. **Setup Phase**: Configure the system under test
2. **Action Phase**: Execute the operation being tested  
3. **Assertion Phase**: Verify the results

The key insight is using fluent method chaining to make this read naturally while maintaining type safety and resource management.

`★ Insight ─────────────────────────────────────`
The fluent pattern allows tests to be self-documenting while the builder pattern ensures all necessary setup occurs before execution. Each phase transitions through distinct types, preventing misuse and providing IDE auto-completion guidance.
`─────────────────────────────────────────────────`

## Universal Entry Point Pattern

Every DSL needs a clear starting point that indicates intent:

<example>
**CLI Application:**
```rust
Cmd::given()           // "Given this command setup..."
    .verbose_flag()
    .a_file_with_content("data")
    .when_run()        // "When I run it..."
    .should_succeed()  // "Then it should..."
```

**HTTP Server:**
```rust
BlogServer::new()           // "Given this server setup..."
    .add_file("post.md", "content")
    .get("/endpoint")       // "When I make this request..."
    .expect_status_code(200) // "Then it should..."
    .execute()
```

</example>

## Setup Phase Patterns

### Command Specification Builder
Track all configuration that affects system behavior:

<example>
```rust
#[derive(Clone, Default)]
pub struct CommandSpec {
    args: Vec<String>,              // CLI flags and options
    input: Option<InputSource>,     // Test data
    environment: HashMap<String, String>, // Environment setup
    working_dir: Option<PathBuf>,   // Execution context
}

impl CommandSpec {
    // Feature flags
    pub fn verbose_flag(mut self) -> Self { /* add --verbose */ }
    pub fn debug_mode(mut self) -> Self { /* add --debug */ }
    
    // Configuration
    pub fn with_config_file(mut self, content: &str) -> Self { /* setup config */ }
    pub fn with_timeout(mut self, seconds: u64) -> Self { /* add --timeout */ }
    
    // Test data
    pub fn with_input_file(mut self, content: &str) -> Self { /* create temp file */ }
    pub fn with_directory(mut self, files: &[(&str, &str)]) -> Self { /* create file tree */ }
    
    // Environment
    pub fn with_env_var(mut self, key: &str, value: &str) -> Self { /* set env */ }
    pub fn at_time(mut self, timestamp: &str) -> Self { /* mock time */ }
}
```
</example>

### Server Specification Builder  
Focus on application state and request configuration:

<example>
```rust
pub struct ServerSpec {
    files: Vec<FileSpec>,           // Server content/data
    config: Option<String>,         // Application configuration
    request: Option<RequestSpec>,   // HTTP request details
}

impl ServerSpec {
    // Data setup
    pub fn with_file(path: &str, content: &str) -> Self { /* add file */ }
    pub fn add_file(mut self, path: &str, content: &str) -> Self { /* chain files */ }
    
    // Configuration
    pub fn with_config(mut self, yaml: &str) -> Self { /* app config */ }
    pub fn with_database(mut self, schema: &str) -> Self { /* db setup */ }
    
    // Request specification
    pub fn get(self, path: &str) -> RequestBuilder { /* HTTP GET */ }
    pub fn post(self, path: &str) -> RequestBuilder { /* HTTP POST */ }
    pub fn with_headers(mut self, headers: &[(&str, &str)]) -> Self { /* request headers */ }
}
```
</example>

## Input Source Management

### Universal Input Types
Define abstractions that work across domains:

<example>
```rust
#[derive(Debug, Clone)]
pub enum InputSource {
    File { path: PathBuf, content: String },
    Directory { files: Vec<InputSource> },
    Stdin { content: String },
    ExistingPath { path: PathBuf },
    Network { mock_responses: Vec<(String, String)> },
}

impl InputSource {
    fn file(content: &str) -> Self { /* default file name */ }
    fn named_file(name: &str, content: &str) -> Self { /* specific name */ }
    fn directory(files: Vec<InputSource>) -> Self { /* file tree */ }
    fn database_with_data(schema: &str, data: &[(&str, &str)]) -> Self { /* test data */ }
}
```
</example>

### Resource Lifetime Management
Ensure test resources live through execution:

<example>
```rust
struct ExecutionContext {
    command: Command,                    // CLI command or server instance
    _temp_resources: Vec<TempResource>, // Keep resources alive
    environment: Environment,           // Test environment state
}

enum TempResource {
    Directory(Arc<TempDir>),      // Temporary filesystem
    Server(Arc<TestServer>),      // Running server instance  
    Database(Arc<TestDatabase>),  // Test database connection
}

impl ExecutionContext {
    fn execute(self) -> TestResult {
        // Setup environment
        self.apply_environment();
        
        // Execute action
        let output = self.run_action();
        
        // Cleanup (automatic via Drop traits)
        TestResult::new(output, self._temp_resources)
    }
}
```
</example>

## Action Phase Patterns

### CLI Execution
Transform specification into actual command execution:

<example>
```rust
impl CommandSpec {
    pub fn when_run(self) -> CommandResult {
        // Create temporary files from InputSource
        let (temp_dir, input_path) = self.setup_filesystem();
        
        // Build command
        let mut command = Command::cargo_bin("my-app").unwrap();
        command.arg("--input").arg(input_path);
        command.args(self.args);
        
        // Set environment
        for (key, value) in self.environment {
            command.env(key, value);
        }
        
        // Execute and capture output
        let output = command.assert();
        
        CommandResult { 
            output, 
            _temp_dir: Some(temp_dir) 
        }
    }
}
```
</example>

### Server Request Execution
Transform specification into HTTP requests:

<example>
```rust
impl ServerSpec {
    pub fn get(self, path: &str) -> RequestBuilder {
        RequestBuilder {
            server_spec: self,
            method: HttpMethod::Get,
            path: path.to_string(),
            assertions: Vec::new(),
        }
    }
}

impl RequestBuilder {
    pub async fn execute(self) -> TestResult {
        // Start server with test data
        let server = self.server_spec.start_test_server().await;
        
        // Make HTTP request
        let url = format!("http://{}{}", server.addr(), self.path);
        let response = reqwest::Client::new()
            .request(self.method.into(), url)
            .send()
            .await
            .unwrap();
        
        // Apply assertions
        let test_response = TestResponse::from(response).await;
        for assertion in self.assertions {
            assertion(&test_response);
        }
        
        // Server cleanup handled by Drop
    }
}
```
</example>

## Assertion Phase Patterns

### Basic Result Assertions
Provide clear success/failure validation:

<example>
```rust
impl TestResult {
    // Basic outcome validation
    pub fn should_succeed(self) -> Self { /* exit code 0 or HTTP 2xx */ }
    pub fn should_fail(self) -> Self { /* non-zero exit or HTTP error */ }
    pub fn should_timeout(self) -> Self { /* operation timeout */ }
    
    // Output validation
    pub fn expect_output(self, text: &str) -> Self { /* stdout/response contains */ }
    pub fn expect_error(self, text: &str) -> Self { /* stderr contains */ }
    pub fn expect_no_output(self) -> Self { /* empty output */ }
    
    // Pattern matching
    pub fn expect_output_matches(self, pattern: &str) -> Self { /* regex match */ }
    pub fn expect_json_field(self, path: &str, value: &str) -> Self { /* JSON validation */ }
}
```
</example>

### Domain-Specific Assertions
Create assertions that match your application's vocabulary:

<example>
**Time Tracking Application:**
```rust
impl CommandResult {
    pub fn expect_project(self, name: &str) -> ProjectAssertion { /* project in report */ }
    pub fn expect_task_with_duration(self, task: &str, duration: &str) -> Self { /* specific timing */ }
    pub fn expect_warning_at_line(self, line: usize, message: &str) -> Self { /* parse warnings */ }
}

pub struct ProjectAssertion {
    pub fn taking(self, duration: &str) -> Self { /* project duration */ }
    pub fn with_percentage(self, percent: &str) -> Self { /* time percentage */ }
    pub fn validate(self) -> CommandResult { /* finalize checks */ }
}
```

**Blog Engine Application:**
```rust
impl RequestBuilder {
    pub fn expect_status_code(self, code: u16) -> Self { /* HTTP status */ }
    pub fn expect_body_contains(self, text: &str) -> Self { /* response content */ }
    pub fn expect_contains_in_order(self, items: &[&str]) -> Self { /* content ordering */ }
    pub fn expect_header(self, name: &str, value: &str) -> Self { /* response headers */ }
}
```

</example>

## Complex Assertion Patterns

### Chained Validations
For outputs with multiple related items:

<example>
```rust
// Time tracking: Multiple projects in one report
Cmd::given()
    .a_file_with_content("project data")
    .when_run()
    .should_succeed()
    .expect_project("proj-1").taking("2h 30m").with_percentage("60")
    .expect_project("proj-2").taking("1h 15m").with_percentage("40")
    .validate()
    .expect_no_warnings();

// Blog: Multiple posts in specific order
BlogServer::new()
    .add_file("posts/new.md", "title: New Post\ndate: 2023-02-01")
    .add_file("posts/old.md", "title: Old Post\ndate: 2023-01-01")
    .get("/")
    .expect_contains_in_order(&["New Post", "Old Post"])
    .expect_body_contains("2 posts found")
    .execute();
```
</example>

### Error Scenario Testing
Design for failure cases with rich diagnostics:

<example>
```rust
// CLI: Invalid input handling
Cmd::given()
    .a_file_with_content("invalid: [malformed")
    .when_run()
    .should_fail()
    .expect_error("Parse error")
    .expect_error("line 1")
    .expect_warning_at_line(1, "Unclosed bracket");

// Server: 404 handling with context
BlogServer::new()
    .get("/nonexistent-post")
    .expect_status_code(404)
    .expect_body_contains("Post not found")
    .expect_header("Content-Type", "text/html")
    .execute();
```
</example>

## Implementation Strategy

### 1. Start with Core Types
Define your main specification and result types first:

<example>
```rust
// Your entry point
pub struct YourApp;
impl YourApp {
    pub fn given() -> AppSpec { AppSpec::new() }
}

// Your specification builder
pub struct AppSpec { /* fields for your domain */ }

// Your result type  
pub struct AppResult { /* output and resources */ }
```
</example>

### 2. Add Basic Builders
Implement setup methods for your domain:

<example>
```rust
impl AppSpec {
    // Configuration
    pub fn with_config(mut self, config: &str) -> Self { /* setup */ }
    
    // Input data
    pub fn with_test_data(mut self, data: &str) -> Self { /* test input */ }
    
    // Execution
    pub fn when_executed(self) -> AppResult { /* run and capture */ }
}
```
</example>

### 3. Build Basic Assertions
Start with fundamental validations:

<example>
```rust
impl AppResult {
    pub fn should_succeed(self) -> Self { /* basic success */ }
    pub fn expect_output(self, text: &str) -> Self { /* output validation */ }
}
```
</example>

### 4. Add Domain Assertions
Create assertions specific to your application:

<example>
```rust
impl AppResult {
    // Match your app's domain concepts
    pub fn expect_user_created(self, username: &str) -> Self { /* domain-specific */ }
    pub fn expect_calculation_result(self, expected: f64) -> Self { /* domain-specific */ }
}
```
</example>

`★ Insight ─────────────────────────────────────`
The progression from basic to domain-specific assertions mirrors how you understand your application. Start with simple success/failure, then add output validation, finally create assertions that speak your application's language. This evolutionary approach keeps tests maintainable as requirements change.
`─────────────────────────────────────────────────`

## Testing Your DSL

Once built, test the DSL itself to ensure it's robust:

<example>
```rust
#[test]
fn dsl_should_handle_basic_success_case() {
    YourApp::given()
        .with_valid_input()
        .when_executed()
        .should_succeed()
        .expect_output("Success");
}

#[test]
fn dsl_should_validate_error_conditions() {
    YourApp::given()
        .with_invalid_input()
        .when_executed()
        .should_fail()
        .expect_error("Invalid input");
}

#[test]
fn dsl_should_chain_multiple_assertions() {
    YourApp::given()
        .with_multi_item_input()
        .when_executed()
        .should_succeed()
        .expect_item("first").with_status("active")
        .expect_item("second").with_status("pending")
        .validate();
}
```
</example>

This universal pattern adapts to any domain while maintaining readability, type safety, and proper resource management. The key is identifying your application's core concepts and building fluent APIs around them.