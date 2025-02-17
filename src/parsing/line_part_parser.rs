use super::time_parser::parse_time;
use crate::domain::ParseError;

pub(crate) enum LinePart<'a> {
    Time(u32),
    Project(String),
    DescriptionPart(&'a str),
}

pub(crate) fn parse_part(part: &str) -> Result<LinePart, ParseError> {
    if part.starts_with('#') {
        let project = LinePart::Project(
            part.strip_prefix("#")
                .expect("project should have had '#' prefix")
                .to_string(),
        );
        Ok(project)
    } else {
        match parse_time(part) {
            Ok(Some(minutes)) => Ok(LinePart::Time(minutes)),
            Ok(None) => Ok(LinePart::DescriptionPart(part)),
            Err(err) => Err(err),
        }
    }
}
