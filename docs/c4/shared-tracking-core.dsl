inputProcessing = component "parsing::process_input" {
    description "Reads markdown time-entry files, parses entries, and applies date/tag filters."
    technology "Rust (src/parsing/mod.rs, src/parsing/parser.rs, src/parsing/filter.rs)"
    tags "Shared Component"
}

trackingDomain = component "domain (core types)" {
    description "Defines `TimeEntry`, tags, periods/date ranges, and tracked-time domain types."
    technology "Rust (src/domain/mod.rs, src/domain/tags.rs, src/domain/dates/*)"
    tags "Shared Component"
}

reportBuilder = component "domain::reporting" {
    description "Builds overview/detail/breakdown/outcomes report models from tracked time."
    technology "Rust (src/domain/reporting.rs)"
    tags "Shared Component"
}

executionStatistics = component "cli::statistics" {
    description "Writes/reads JSONL execution records and aggregates flag-usage statistics."
    technology "Rust (src/cli/statistics.rs)"
    tags "Shared Component"
}

inputProcessing -> trackingDomain "Creates tracked time and parse errors with"
reportBuilder -> trackingDomain "Aggregates tracked time with"
