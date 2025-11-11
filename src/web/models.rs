use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DashboardParams {
    pub period: Option<String>,
    pub limit: Option<bool>,
    pub from: Option<String>,
    pub to: Option<String>,
}
