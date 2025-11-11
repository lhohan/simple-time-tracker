use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DashboardParams {
    pub period: Option<String>,
    pub limit: Option<bool>,
}
