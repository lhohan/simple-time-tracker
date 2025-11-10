use askama::Template;
use axum::extract::State;
use axum::response::Html;
use std::path::PathBuf;
use std::sync::Arc;

use crate::domain::reporting::{OverviewReport, TimeTotal};
use crate::parsing;

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

pub async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let template = if let Some(ref data_path) = state.data_path {
        let tracking_result = parsing::process_input(data_path, None)
            .expect("Failed to process input");

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

    Html(template.render().expect("Failed to render template"))
}
