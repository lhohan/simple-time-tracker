# Product Requirements Document & Development Roadmap: Time Tracking Application

## 1. Introduction

This document outlines the requirements and development plan for a simple, text-based time-tracking application. The application will analyze markdown files to track time spent on various projects and generate reports. The target audience is individuals seeking a straightforward method for time tracking and reporting.  The primary goal is to provide accurate and flexible time-tracking capabilities with minimal user overhead.


## 2. Product Overview

This application reads markdown files, parses time entries associated with project identifiers, and generates reports summarizing time allocations across different projects and time periods.  It prioritizes a command-line interface (CLI) for initial usage, with a potential web interface as a future enhancement.


## 3. Features

### 3.1 Input:
* **File Reading:** The application will read `.txt` and `.md` files.  It will recursively process files within a specified input directory. Error messages will indicate the file and line number where parsing errors occur.
* **Markdown Parsing:**  Time entries are specified in markdown using a heading of any level starting with `TT` followed by the date YYYY-MM-DD.  Entries are indicated by top-level bullet points (`-`). Each entry includes at least one project identifier (#project) and a time value (e.g., `20m`, `1h`, `4p` for minutes, hours, and pomodoros (1 pomodoro = 30 minutes), respectively). Multiple project identifiers (separated by spaces) are allowed.  Projects can be nested implicitly by including multiple identifiers in a single entry (left-to-right order implies sub-project). (Addresses Tasks 001, 002).  See examples below.
* **Time Units:** Minutes (m), hours (h), pomodoros (p).
### 3.2 Processing:
* The application will extract data from the marked down files, including timestamps, project identifiers, and descriptions, validating data integrity as it’s processed.
* **Data Storage:**  For this initial version, data will be processed directly without explicit persistent storage to keep the implementation simpler. A future enhancement *could* include using a SQLite database for improved performance and data management.

### 3.3 Output:
* **Report Generation:** The application will generate reports summarizing time spent on projects for daily, weekly, monthly periods, and custom date ranges (Task 008).  Reports clearly present a breakdown of time per project.
* **Error reporting** The application will report warnings for detected TT headings that may contain wrong dates and time entries that may be wrong. Opposed to silently ignoring we prefer to help the user identify wrongly entered data.
* **Report Format:** Reports will be generated as plain text files to maintain simplicity.  Each report will summarize total time spent on each project within the selected period.  The output will be human-readable and straightforward. (Decision made for initial release, other formats may be added later)
* **Web Server:** A simple web server for viewing reports will be considered in a later phase (as per the PRD).

### 3.4 User Interface:
* **Command-Line Interface (CLI):** The application will be primarily controlled via a command-line interface, allowing users to specify the input directory, report type, and other parameters.  This is prioritized for the initial version.
* **Web Interface:** A web interface is considered a future enhancement (as per the PRD).

### 3.5 Configuration:
* **Project Identifiers:**  Users will not need to explicitly define project identifiers before use, however, consistency in project names across files is important for accurate reporting.
* **Time Intervals:** Users will be able to select daily, weekly, monthly, or specify a custom date range for reporting. (Addresses Task 008)
* **Input:** CLI parameter to specify the file or the directory containing the markdown files.

## 4. Suggestions for Input Format

### 4.1. Basic Markdown Example

```markdown
## TT 2025-01-15

- #journaling  20m
- #prj-time-tracking #rust 4p
- #sport 1h Cycle
- #sport 30m Get ready
```

Time tracking entries are marked with markdown heading (any level deep) starting with TT followed by a date.
Example above contained entries for Jan 15th 2025.

Each new entry starts with a top-level bullet item (starts with '-').
In each entry:
- Contains a time:
    - 20m : 'm' stands for minutes
    - 1h : 'h' stands for hours
    - 4p : 'p' stands for pomodoros, 1 pomodoro will be translated to 30 minutes
    - There can be multiple times per entry
- The '#' indicates the project or key the time should be recorded under.

### 4.2 Multiple project identifiers

An entry for a sub-project 'sub-proj' below main project 'proj'. Further nesting is possible.

```
- #sub-proj #proj 5h My task
```


## 4. Development Roadmap

### Phase 1: Core Functionality (MVP):
* Implement core markdown parsing focusing on the core time-tracking elements.
* Implement daily, weekly, and monthly reporting.
* Develop a robust CLI.
* Implement CI/CD (Task 000).  Nix will be used for package management and reproducible builds.  Just will be used for task automation.
### Phase 2: Enhancements and Refinements:
* Implement support for handling and reporting outcomes/goals (Task 001). This functionality will provide context and higher level view of time allocation and progress on projects.
* Refine support for multiple projects and sub-projects (Task 002).  Improve clarity and support for nesting projects.
* Add support for extended and full-day entries (Task 003).  Allow entries that specify the entire working day was spent on a project.
* Address areas for improvement after project filtering (Task 004) which includes support for subprojects and multiple filters.
* Extend support for custom date range selection (Task 008).
* Implement working-day length support allowing users to specify their typical working hours (Task 007).
*  Improve error handling and reporting for easier debugging.
### Phase 3: Optional Features:
* Develop a user-friendly web interface.
* Explore the use of generated specs as input for LLMs (Task 006).
* Implement persistent data storage (using SQLite or other suitable database).


## 5. Open Issues and Decisions

* Decision made:  No persistent data storage for MVP.  SQLite will be considered later if performance issues arise.
* Decision made: Output report format is plain text for the MVP.  Other formats (CSV, JSON) can be considered in future enhancements.
* Further clarification is needed on the precise nature of the "outcomes/goals" feature (Task 001).


## 6. Open Tasks

### Task 000: Set up CI/CD (Integrated into Phase 1 of Development Roadmap):
  * Use nix set-up in CI/CD pipeline
  * Report and set treshholds for clippy warnings

### Task 001: Add support for outcomes or goals (Integrated into Phase 2 of Development Roadmap):**
    * Allow users to associate tasks with broader outcomes or goals.
    * Explore the use of "domains" as a hierarchical structure above projects.

### Task 002: Support for Multiple Projects and Activities

#### Status: Design Phase Complete, Ready for Implementation

#### Design Decisions:
1. **Time Entry Model**
   - Distinguish between Project-Based and Activity-Based entries
   - Support multiple tags with left-to-right significance
   - Maintain flexible context tagging

2. **Tag Relationships**
   - Primary tag (leftmost) determines entry type
   - Supporting tags provide context/categorization
   - Project tags use 'prj-' prefix

3. **Validation Rules**
   - Enforce project naming conventions
   - Require at least one tag per entry
   - Validate time format specifications

#### Implementation Tasks:
1. [ ] Core Data Structures
   - Implement TimeEntry enum and supporting types
   - Add validation rules
   - Create tag relationship handling

2. [ ] Parsing
   - Implement markdown parser for new format
   - Add validation during parsing
   - Handle multiple tag scenarios

3. [ ] Reporting
   - Implement flexible tag-based querying
   - Add project vs. activity filtering
   - Support hierarchical time summaries

4. [ ] Testing
   - Unit tests for all tag patterns
   - Integration tests with sample markdown
   - Edge case validation

#### Validation Criteria:
- Can process entries with multiple tags
- Correctly interprets project vs. activity entries
- Generates accurate reports across tag combinations
- Maintains backward compatibility with existing logs


### Task 003: Support extended and full-day entries (Integrated into Phase 2 of Development Roadmap):**
    * Implement support for specifying full or extended day entries for projects.

### Task 004: Areas for improvement after project filtering (Integrated into Phase 2 of Development Roadmap):**
    * Enhance support for sub-projects (#dev #rust) in filtering.
    * Add support for multiple project filters simultaneously.
    * Refactor reporting logic to accommodate hierarchical project reporting.
    * Extract formatting logic into a separate module.
    * Improve unit testing for edge cases.
    * Consider the builder pattern for Report construction.

### Task 006: Experiment with tests outputting specs... (Integrated into Phase 2 of Development Roadmap):**
    * Explore the feasibility of tests outputting specifications that can serve as input for LLMs.

### Task 007: Add length-of-working days support (Integrated into Phase 2 of Development Roadmap):**
    * Allow users to specify their typical working day length for better efficiency calculations including time spent on misc tasks.

### Task 008: Implement reporting for between dates, specific months, etc (Integrated into Section 3.3 Output and Phase 1 & 2 of the Development Roadmap):**
    * Implement reporting for specified periods (last week, last month) in addition to daily/weekly/monthly reports and custom date ranges.

### Task 009: Active and finished project support.

Why?: To get an overview of finished projects and active projects. To get a sense of how long projects are running (Could) but mostly listing achievements.

- Projects are defined as: have an end-date.

### Task 010: Get overview per week of top activitities/projects worked on to get an high-level feel of what I worked on

### Task 011: Log files containing time entries under --verbose flag, useful when entries are read from a directory and need to find where entry was defined
