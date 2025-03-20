use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    Project(String),
    Context(String),
}

impl Tag {
    pub fn from_raw(raw_tag: &str) -> Self {
        if raw_tag.starts_with("prj-") {
            Tag::Project(raw_tag.strip_prefix("prj-").unwrap().to_string())
        } else {
            Tag::Context(raw_tag.to_string())
        }
    }

    pub fn is_project(&self) -> bool {
        matches!(self, Tag::Project(_))
    }

    pub fn is_actvity(&self) -> bool {
        matches!(self, Tag::Context(_))
    }

    pub fn name(&self) -> &str {
        match self {
            Tag::Project(name) => name,
            Tag::Context(name) => name,
        }
    }

    pub fn raw_value(&self) -> String {
        match self {
            Tag::Project(name) => format!("prj-{}", name),
            Tag::Context(name) => name.clone(),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_value())
    }
}

pub enum TagFilter {
    ShowOnlyContainingAny(Vec<Tag>),
}

impl TagFilter {
    pub fn parse(input: Vec<String>) -> Self {
        let tags = input.into_iter().map(|tag| Tag::from_raw(&tag)).collect();
        TagFilter::ShowOnlyContainingAny(tags)
    }

    pub fn filter_tags(&self) -> Vec<Tag> {
        match self {
            TagFilter::ShowOnlyContainingAny(tags) => tags.clone(),
        }
    }

    pub fn project(&self) -> Option<Tag> {
        match self {
            TagFilter::ShowOnlyContainingAny(tags) => tags.first().cloned(),
        }
    }
}
