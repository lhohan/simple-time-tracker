use askama::Template;
use axum::extract::{Query, State};
use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use chrono::NaiveDate;
use std::path::PathBuf;
use std::sync::Arc;

use crate::domain::reporting::{OverviewReport, TimeTotal};
use crate::domain::time::Clock;
use crate::domain::PeriodRequested;
use crate::parsing;
use crate::parsing::filter::Filter;

use super::models::DashboardParams;

pub enum WebError {
    DataProcessingFailed(String),
    TemplateRenderFailed(String),
    InvalidTag(String),
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            WebError::DataProcessingFailed(msg) => {
                eprintln!("Data processing error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error loading data".to_string())
            }
            WebError::TemplateRenderFailed(msg) => {
                eprintln!("Template render error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error rendering page".to_string())
            }
            WebError::InvalidTag(tag) => {
                eprintln!("Invalid tag requested: {}", tag);
                (StatusCode::BAD_REQUEST, format!("Invalid tag: {}", tag))
            }
        };
        (status, Html(format!("<p>{}</p>", message))).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub data_path: Option<PathBuf>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub total_time: String,
    pub projects: Vec<TimeTotal>,
}

fn format_minutes(minutes: u32) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    if hours > 0 {
        if mins > 0 {
            format!("{hours}h {mins}m")
        } else {
            format!("{hours}h")
        }
    } else {
        format!("{mins}m")
    }
}

fn is_valid_tag(tag: &str) -> bool {
    !tag.is_empty()
        && tag.len() < 256
        && tag.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
}

pub async fn dashboard(State(state): State<Arc<AppState>>) -> Result<Html<String>, WebError> {
    let template = if let Some(data_path) = state.data_path.clone() {
        let tracking_result = tokio::task::spawn_blocking(move || {
            parsing::process_input(&data_path, None)
        })
        .await
        .map_err(|e| WebError::DataProcessingFailed(format!("Task failed: {}", e)))?
        .map_err(|e| WebError::DataProcessingFailed(e.to_string()))?;

        if let Some(time_entries) = tracking_result.time_entries {
            let overview = OverviewReport::overview(&time_entries, None, None);

            DashboardTemplate {
                total_time: format_minutes(overview.total_minutes()),
                projects: overview.entries_time_totals().clone(),
            }
        } else {
            DashboardTemplate {
                total_time: "0m".to_string(),
                projects: vec![],
            }
        }
    } else {
        DashboardTemplate {
            total_time: "8h 30m".to_string(),
            projects: vec![],
        }
    };

    let html = template
        .render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "projects_partial.html")]
pub struct ProjectsPartialTemplate {
    pub projects: Vec<TimeTotal>,
}

pub async fn dashboard_partial(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    let template = if let Some(data_path) = state.data_path.clone() {
        let clock = std::env::var("TT_TODAY")
            .ok()
            .and_then(|today_str| NaiveDate::parse_from_str(&today_str, "%Y-%m-%d").ok())
            .map(Clock::with_today)
            .unwrap_or_else(Clock::system);

        let period = params
            .period
            .as_ref()
            .and_then(|p| PeriodRequested::from_str(p, &clock).ok());

        let filter = period.as_ref().map(|p| Filter::DateRange(p.date_range()));

        let tracking_result = tokio::task::spawn_blocking(move || {
            parsing::process_input(&data_path, filter.as_ref())
        })
        .await
        .map_err(|e| WebError::DataProcessingFailed(format!("Task failed: {}", e)))?
        .map_err(|e| WebError::DataProcessingFailed(e.to_string()))?;

        if let Some(time_entries) = tracking_result.time_entries {
            let overview = OverviewReport::overview(&time_entries, None, period.as_ref());

            ProjectsPartialTemplate {
                projects: overview.entries_time_totals().clone(),
            }
        } else {
            ProjectsPartialTemplate {
                projects: vec![],
            }
        }
    } else {
        ProjectsPartialTemplate {
            projects: vec![],
        }
    };

    let html = template
        .render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "tag_detail.html")]
pub struct TagDetailTemplate {
    pub tag_name: String,
    pub entries: Vec<EntryDisplay>,
    pub total_minutes: u32,
}

pub struct EntryDisplay {
    pub description: String,
    pub duration: u32,
}

pub async fn tag_detail(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(tag_name): axum::extract::Path<String>,
) -> Result<Html<String>, WebError> {
    use crate::domain::tags::Tag;

    if !is_valid_tag(&tag_name) {
        return Err(WebError::InvalidTag(tag_name));
    }

    let template = if let Some(data_path) = state.data_path.clone() {
        let tag_name_clone = tag_name.clone();
        let tracking_result = tokio::task::spawn_blocking(move || {
            parsing::process_input(&data_path, None)
        })
        .await
        .map_err(|e| WebError::DataProcessingFailed(format!("Task failed: {}", e)))?
        .map_err(|e| WebError::DataProcessingFailed(e.to_string()))?;

        if let Some(time_entries) = tracking_result.time_entries {
            let tag = Tag::from_raw(&tag_name_clone);
            let detail_report = time_entries.tasks_tracked_for(&[tag]);

            let entries: Vec<EntryDisplay> = if !detail_report.summaries().is_empty() {
                detail_report.summaries()[0]
                    .task_summaries()
                    .iter()
                    .map(|summary| EntryDisplay {
                        description: summary.description.clone(),
                        duration: summary.minutes,
                    })
                    .collect()
            } else {
                vec![]
            };

            TagDetailTemplate {
                tag_name: tag_name.clone(),
                entries,
                total_minutes: detail_report.total_minutes(),
            }
        } else {
            TagDetailTemplate {
                tag_name: tag_name.clone(),
                entries: vec![],
                total_minutes: 0,
            }
        }
    } else {
        TagDetailTemplate {
            tag_name: tag_name.clone(),
            entries: vec![],
            total_minutes: 0,
        }
    };

    let html = template
        .render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}

#[derive(Template)]
#[template(path = "chart_projects_bar.html")]
pub struct ChartProjectsBarTemplate {
    pub projects: Vec<TimeTotal>,
}

pub async fn chart_projects_bar(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    let template = if let Some(data_path) = state.data_path.clone() {
        let clock = std::env::var("TT_TODAY")
            .ok()
            .and_then(|today_str| NaiveDate::parse_from_str(&today_str, "%Y-%m-%d").ok())
            .map(Clock::with_today)
            .unwrap_or_else(Clock::system);

        let period = params
            .period
            .as_ref()
            .and_then(|p| PeriodRequested::from_str(p, &clock).ok());

        let filter = period.as_ref().map(|p| Filter::DateRange(p.date_range()));

        let tracking_result = tokio::task::spawn_blocking(move || {
            parsing::process_input(&data_path, filter.as_ref())
        })
        .await
        .map_err(|e| WebError::DataProcessingFailed(format!("Task failed: {}", e)))?
        .map_err(|e| WebError::DataProcessingFailed(e.to_string()))?;

        if let Some(time_entries) = tracking_result.time_entries {
            let overview = OverviewReport::overview(&time_entries, None, period.as_ref());

            ChartProjectsBarTemplate {
                projects: overview.entries_time_totals().clone(),
            }
        } else {
            ChartProjectsBarTemplate {
                projects: vec![],
            }
        }
    } else {
        ChartProjectsBarTemplate {
            projects: vec![],
        }
    };

    let html = template
        .render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}


#[derive(Template)]
#[template(path = "chart_projects_pie.html")]
pub struct ChartProjectsPieTemplate {
    pub projects: Vec<TimeTotal>,
}

pub async fn chart_projects_pie(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DashboardParams>,
) -> Result<Html<String>, WebError> {
    let template = if let Some(data_path) = state.data_path.clone() {
        let clock = std::env::var("TT_TODAY")
            .ok()
            .and_then(|today_str| NaiveDate::parse_from_str(&today_str, "%Y-%m-%d").ok())
            .map(Clock::with_today)
            .unwrap_or_else(Clock::system);

        let period = params
            .period
            .as_ref()
            .and_then(|p| PeriodRequested::from_str(p, &clock).ok());

        let filter = period.as_ref().map(|p| Filter::DateRange(p.date_range()));

        let tracking_result = tokio::task::spawn_blocking(move || {
            parsing::process_input(&data_path, filter.as_ref())
        })
        .await
        .map_err(|e| WebError::DataProcessingFailed(format!("Task failed: {}", e)))?
        .map_err(|e| WebError::DataProcessingFailed(e.to_string()))?;

        if let Some(time_entries) = tracking_result.time_entries {
            let overview = OverviewReport::overview(&time_entries, None, period.as_ref());

            ChartProjectsPieTemplate {
                projects: overview.entries_time_totals().clone(),
            }
        } else {
            ChartProjectsPieTemplate {
                projects: vec![],
            }
        }
    } else {
        ChartProjectsPieTemplate {
            projects: vec![],
        }
    };

    let html = template
        .render()
        .map_err(|e| WebError::TemplateRenderFailed(e.to_string()))?;
    Ok(Html(html))
}
