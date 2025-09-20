
# Taks

The task you are helping me with is described in $ARGUMENTS .

# System

When executing a task, NEVER begin writing code immediately. Always start with analysis and planning.
Verify understanding of the task with the user BEFORE proposing implementation details.

# Task Execution Process
Follow these steps in STRICT ORDER:

## Setup

BEFORE beginning: create a new markdown file to capture your notes while you work on this issue. Save it in the `docs/notes` directory with the same name as the markdown file passed: $ARGUMENTS . IF such a file already exists, read the file and continue from where you left off.

### Required Notes File Structure
Your notes file MUST include these standardized sections for session continuity:

```markdown
## Current Status
- **Phase**: [Current phase number and name]
- **Step**: [Current step within phase]
- **Last Updated**: [Timestamp]

```

## Developement process

### TDD with Primitive Whole

Follow this progression for robust development:

1. **Red**: Write simplest possible end-to-end test that fails
2. **Green**: Implement minimal solution that passes
3. **Refactor**: Extract reusable patterns while keeping tests green
4. **Iterate**: Add complexity only when needed for next feature

### Integration with Project Workflow

1. **Start with acceptance test** using CLI interface
2. **Use TempDir for file operations**
3. **Commit after each working iteration**
4. **Add unit tests only for complex business logic on stable interfaces or APIs**

### Writing tests using a domain oriented testing DSL to describe behaviour

- Write tests using a domain oriented testing DSL to describe behaviour
- Write executable specifications: the tests read like specifications and can be executed directly against the codebase to verify the behaviour

## Critical Reminders

- **User approval gates**: Never proceed with major decisions without user confirmation - prevents overengineering and scope creep
- **Coverage monitoring**: Establish baseline, monitor throughout, verify maintenance - enables safe large-scale refactoring
- **Incremental implementation**: Small, verifiable steps with continuous validation - prevents catastrophic failures
- **Agent recommendation validation**: Question agent advice against actual requirements - user domain knowledge trumps agent suggestions
- **CLAUDE.md Compliance**: Always check CLAUDE.md for project-specific constraints (e.g., no Claude references in commits)
- **Documentation Consistency**: Search and update ALL documentation that might reference changed functionality
- **Test Quality**: When working with tests, prioritize atomic readability over code reuse - inline test data is preferable to constants for clarity
