workspace "Time Tracker" "C4 model for the Time Tracker Rust project." {
    !identifiers hierarchical

    model {
        user = person "User" "Tracks time and reviews reports with the CLI and web dashboard."

        timeEntryFiles = softwareSystem "Time Entry Markdown Files" "User-managed markdown files that store tracked time entries."

        timeTracker = softwareSystem "Time Tracker" "Rust application for parsing markdown time entries and presenting reports via CLI and web interfaces." {
            cli = container "CLI Application" "Parses markdown time entries and prints reports in text or markdown formats." "Rust + Clap" {
                cliWorkflow = component "main::run_cli + cli::Args" {
                    description "Parses CLI flags, resolves filters/periods, and orchestrates report generation."
                    technology "Rust (src/main.rs, src/cli/mod.rs)"
                }

                cliOutputFormatter = component "reporting::format::{text, markdown}" {
                    description "Formats report models as terminal text or markdown output."
                    technology "Rust (src/reporting/format/text.rs, src/reporting/format/markdown.rs)"
                }

                group "Shared Tracking Core" {
                    !include shared-tracking-core.dsl
                }
            }

            web = container "Web Dashboard" "Serves interactive dashboards, tag detail views, outcomes pages, and health endpoints over HTTP." "Rust + Axum + Askama" {
                httpHandlers = component "web::handlers" {
                    description "Accepts HTTP requests, resolves dashboard parameters, and coordinates dashboard, outcomes, tag detail, and health responses."
                    technology "Rust + Axum (src/web/handlers.rs)"
                }

                templateRenderer = component "Askama templates (templates/*.html)" {
                    description "Renders HTML pages and partials for dashboard and outcomes views."
                    technology "Askama + HTML (templates/*.html)"
                }

                group "Shared Tracking Core" {
                    !include shared-tracking-core.dsl
                }
            }

            runStatisticsLog = container "Run Statistics Log" "Stores JSON Lines records about application executions and flag usage." "JSONL file" {
                tags "Database"
            }
        }

        user -> timeTracker.cli "Runs reports with"
        user -> timeTracker.web "Views dashboards and flag statistics in"
        user -> timeTracker.cli.cliWorkflow "Runs reports with"
        user -> timeTracker.web.httpHandlers "Uses via browser"

        timeTracker.cli -> timeEntryFiles "Reads time entry markdown files from"
        timeTracker.web -> timeEntryFiles "Reads time entry markdown files from"

        timeTracker.cli -> timeTracker.runStatisticsLog "Appends execution records to"
        timeTracker.web -> timeTracker.runStatisticsLog "Reads flag statistics from and appends execution records to"

        timeTracker.cli.cliWorkflow -> timeTracker.cli.inputProcessing "Loads filtered entries via"
        timeTracker.cli.cliWorkflow -> timeTracker.cli.reportBuilder "Builds report models with"
        timeTracker.cli.cliWorkflow -> timeTracker.cli.cliOutputFormatter "Writes terminal output with"
        timeTracker.cli.cliWorkflow -> timeTracker.cli.executionStatistics "Records execution metadata via"
        timeTracker.cli.cliOutputFormatter -> timeTracker.cli.reportBuilder "Formats report models from"
        timeTracker.cli.inputProcessing -> timeEntryFiles "Reads markdown entries from"
        timeTracker.cli.executionStatistics -> timeTracker.runStatisticsLog "Appends JSONL records to"

        timeTracker.web.httpHandlers -> timeTracker.web.inputProcessing "Loads filtered entries via"
        timeTracker.web.httpHandlers -> timeTracker.web.reportBuilder "Builds dashboard and outcomes models with"
        timeTracker.web.httpHandlers -> timeTracker.web.templateRenderer "Renders HTML with"
        timeTracker.web.httpHandlers -> timeTracker.web.executionStatistics "Reads flag usage summaries via"
        timeTracker.web.inputProcessing -> timeEntryFiles "Reads markdown entries from"
        timeTracker.web.executionStatistics -> timeTracker.runStatisticsLog "Reads JSONL records from"
    }

    views {
        systemContext timeTracker "SystemContext" {
            title "Time Tracker - System Context"
            include *
            autoLayout lr
        }

        container timeTracker "Containers" {
            title "Time Tracker - Containers"
            include *
            autoLayout lr
        }

        component timeTracker.cli "CliComponents" {
            title "Time Tracker - CLI Application Components"
            include *
            autoLayout lr
        }

        component timeTracker.web "WebComponents" {
            title "Time Tracker - Web Dashboard Components"
            include *
            autoLayout lr
        }

        styles {
            element "Database" {
                shape Cylinder
            }

            element "Shared Component" {
                background #f4f1de
                color #111111
            }
        }
    }

    configuration {
        scope softwaresystem
    }
}
