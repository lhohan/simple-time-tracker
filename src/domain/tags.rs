use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Project(String),
    Context(String),
}

impl Tag {
    /// Creates a tag from a raw string representation.
    ///
    /// # Panics
    ///
    /// Panics if the string starts with "prj-" but the prefix cannot be stripped (should not happen in practice).
    #[must_use]
    pub fn from_raw(raw_tag: &str) -> Self {
        if raw_tag.starts_with("prj-") {
            Tag::Project(raw_tag.strip_prefix("prj-").unwrap().to_string())
        } else {
            Tag::Context(raw_tag.to_string())
        }
    }

    #[must_use]
    pub fn raw_value(&self) -> String {
        match self {
            Tag::Project(name) => format!("prj-{name}"),
            Tag::Context(name) => name.clone(),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_value())
    }
}

#[derive(Clone)]
pub enum TagFilter {
    Any(Vec<Tag>),
}

impl TagFilter {
    #[must_use]
    pub fn parse(input: Vec<String>) -> Self {
        let tags = input.into_iter().map(|tag| Tag::from_raw(&tag)).collect();
        TagFilter::Any(tags)
    }

    #[must_use]
    pub fn filter_tags(&self) -> Vec<Tag> {
        match self {
            TagFilter::Any(tags) => tags.clone(),
        }
    }

    #[must_use]
    pub fn tags(&self) -> Vec<Tag> {
        self.filter_tags()
    }
}
