pub(crate) fn maybe_date_from_header(line: &str) -> Option<&str> {
    let mut words = line.split_whitespace();

    if matches!(words.next(), Some(first) if first.starts_with('#'))
        && matches!(words.next(), Some("TT"))
    {
        words.next()
    } else {
        None
    }
}
