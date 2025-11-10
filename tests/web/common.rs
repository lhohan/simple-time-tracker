#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_panics_doc)]

use assert_fs::prelude::*;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use chrono::NaiveDate;
use std::sync::Arc;
use tower::util::ServiceExt;

#[derive(Debug, Clone)]
enum InputSource {
    File { content: String },
}

impl InputSource {
    fn file(content: &str) -> Self {
        Self::File {
            content: content.to_string(),
        }
    }
}

pub struct WebApp;

impl WebApp {
    pub fn given() -> WebAppSpec {
        WebAppSpec::new()
    }
}

#[derive(Clone)]
pub struct WebAppSpec {
    input: Option<InputSource>,
    run_date: Option<NaiveDate>,
}

impl WebAppSpec {
    fn new() -> Self {
        Self {
            input: None,
            run_date: None,
        }
    }

    pub fn a_file_with_content(mut self, content: &str) -> Self {
        self.input = Some(InputSource::file(content));
        self
    }

    #[allow(dead_code)]
    pub fn at_date(mut self, date: &str) -> Self {
        let date =
            NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Invalid date format in test");
        self.run_date = Some(date);
        self
    }

    pub fn when_get(self, path: &str) -> RequestBuilder {
        RequestBuilder {
            spec: self,
            method: Method::GET,
            path: path.to_string(),
            query: None,
        }
    }
}

pub struct RequestBuilder {
    spec: WebAppSpec,
    method: Method,
    path: String,
    query: Option<String>,
}

impl RequestBuilder {
    #[allow(dead_code)]
    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    pub async fn execute(self) -> WebAppResult {
        let (temp_dir, _input_path) = if let Some(input) = self.spec.input {
            let temp =
                Arc::new(assert_fs::TempDir::new().expect("Failed to create temporary directory"));
            match input {
                InputSource::File { content } => {
                    let file = temp.child("test.md");
                    file.write_str(&content).expect("Failed to write test file");
                    (Some(temp.clone()), Some(file.path().to_path_buf()))
                }
            }
        } else {
            (None, None)
        };

        if let Some(run_date) = self.spec.run_date {
            let today = run_date.format("%Y-%m-%d").to_string();
            std::env::set_var("TT_TODAY", today);
        }

        let state = std::sync::Arc::new(time_tracker::web::AppState {
            data_path: _input_path,
        });
        let app = time_tracker::web::server::create_router_with_state(state);

        let uri = if let Some(query) = self.query {
            format!("{}?{}", self.path, query)
        } else {
            self.path.clone()
        };

        let request = Request::builder()
            .method(self.method)
            .uri(uri)
            .body(Body::empty())
            .expect("Failed to build request");

        let response = app
            .oneshot(request)
            .await
            .expect("Failed to execute request");

        std::env::remove_var("TT_TODAY");

        WebAppResult {
            status: response.status(),
            body: axum::body::to_bytes(response.into_body(), usize::MAX)
                .await
                .expect("Failed to read response body"),
            _temp_dir: temp_dir,
        }
    }

    pub async fn should_succeed(self) -> WebAppResult {
        let result = self.execute().await;
        assert!(
            result.status.is_success(),
            "Expected success status but got: {}",
            result.status
        );
        result
    }
}

pub struct WebAppResult {
    status: StatusCode,
    body: axum::body::Bytes,
    _temp_dir: Option<Arc<assert_fs::TempDir>>,
}

impl WebAppResult {
    pub fn expect_status(self, code: u16) -> Self {
        assert_eq!(
            self.status.as_u16(),
            code,
            "Expected status {} but got {}",
            code,
            self.status
        );
        self
    }

    pub fn expect_contains(self, text: &str) -> Self {
        let body_str = std::str::from_utf8(&self.body).expect("Invalid UTF-8 in response body");
        assert!(
            body_str.contains(text),
            "Expected body to contain '{}' but it didn't.\nBody: {}",
            text,
            body_str
        );
        self
    }

    #[allow(dead_code)]
    pub fn expect_not_contains(self, text: &str) -> Self {
        let body_str = std::str::from_utf8(&self.body).expect("Invalid UTF-8 in response body");
        assert!(
            !body_str.contains(text),
            "Expected body to not contain '{}' but it did.\nBody: {}",
            text,
            body_str
        );
        self
    }
}
