use serde::{Deserialize, Deserializer};

fn empty_string_as_none<'de, D>(de: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    Ok(opt.filter(|s| !s.is_empty()))
}

#[derive(Debug, Deserialize)]
pub struct DashboardParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub period: Option<String>,
    pub limit: Option<bool>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub from: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub to: Option<String>,
}
