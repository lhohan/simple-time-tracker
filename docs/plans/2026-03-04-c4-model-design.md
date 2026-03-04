# Time Tracker C4 Model Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add a strict C4 model in Structurizr DSL that documents the Time Tracker system context and container view.

**Architecture:** Model `Time Tracker` as a single software system with two application containers (`CLI Application` and `Web Dashboard`) plus one internal datastore container (`Run Statistics Log`). Represent the user-managed markdown input as an external file-based software system so the system context remains strict while the container view still shows the data dependency.

**Tech Stack:** Structurizr DSL, C4 model terminology, repository documentation in `docs/`

---

### Task 1: Lock the model scope

**Files:**
- Modify: `docs/plans/2026-03-04-c4-model-design.md`
- Create: `docs/c4/time-tracker.dsl`

**Step 1: Record the approved scope**

Capture the agreed boundaries:
- One software system: `Time Tracker`
- Two runtime containers: `CLI Application`, `Web Dashboard`
- One internal datastore container: `Run Statistics Log`
- One external dependency: `Time Entry Markdown Files`

**Step 2: Keep the model strict**

Do not model shared Rust modules as containers.
Reserve parsing/domain/reporting for a later component view on a specific container.

### Task 2: Write the Structurizr DSL

**Files:**
- Create: `docs/c4/time-tracker.dsl`

**Step 1: Define the model**

Create:
- `User` as a person
- `Time Entry Markdown Files` as an external software system
- `Time Tracker` as the software system in scope
- `CLI Application`, `Web Dashboard`, and `Run Statistics Log` inside `Time Tracker`

**Step 2: Define relationships**

Show:
- `User` runs the CLI application
- `User` uses the web dashboard
- Both application containers read time entry markdown files
- CLI app appends execution records to the run statistics log
- Web dashboard reads flag statistics from and appends execution records to the run statistics log

**Step 3: Define views**

Add:
- a system context view for `Time Tracker`
- a container view for `Time Tracker`
- automatic left-to-right layout

### Task 3: Verify the deliverable

**Files:**
- Test: `docs/c4/time-tracker.dsl`

**Step 1: Inspect the DSL**

Run a text check to confirm the requested views and names are present.

**Step 2: Attempt tool validation if available**

If Structurizr tooling is available locally, run a validation command against the DSL workspace.
If not available, report that the DSL was inspected but not tool-validated.
