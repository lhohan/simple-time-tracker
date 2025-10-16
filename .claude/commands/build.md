
# Task

The task you are helping me with is described in $ARGUMENTS .

# System

When executing a task, NEVER begin writing code immediately. Always start with analysis and planning.
Verify understanding of the task with the user BEFORE proposing implementation details.

# Task Execution Process
Follow these steps in STRICT ORDER:

## Setup

BEFORE beginning: create a new markdown file to capture your notes while you work on this issue. Save it in the `notes` directory with the same name as the markdown file passed: $ARGUMENTS . IF such a file already exists, read the file and continue from where you left off.

### Required Notes File Structure
Your notes file MUST include these standardized sections for session continuity:

```markdown
## Current Status
- **Phase**: [Current phase number and name]
- **Step**: [Current step within phase]
- **Last Updated**: [Timestamp]

## Phase Status Tracker
- ⏸️ Phase 1: Task Analysis - PENDING
- ⏸️ Phase 2: Solution Design - PENDING
- ⏸️ Phase 3: Implementation - PENDING
- ⏸️ Phase 4: Review - PENDING
- ⏸️ Phase 5: Submit - PENDING
- ⏸️ Phase 6: Iterate - PENDING
- ⏸️ Phase 7: Reflect - PENDING
- ⏸️ Phase 8: Clean Up - PENDING
```

**Status Icons**: ⏸️ PENDING | 🔄 IN PROGRESS | ✅ COMPLETED

## Phase 1: Task Analysis
**BEFORE STARTING**: Update notes with "Phase 1: Task Analysis - IN PROGRESS 🔄"

1. Load the task described in $ARGUMENTS.
2. Summarize the task requirements and constraints in your own words
3. Challenge requirements early. Always question whether the requested task or approach is correct and necessary
4. Explicitly ask the user to confirm your understanding before proceeding
5. Identify any ambiguities or points requiring clarification and ask about them
6. Once your understanding of the issue has been confirmed, write a summary to your note file.

**AFTER COMPLETION**: Update notes with "Phase 1: Task Analysis - COMPLETED ✅"

*Optional: Consider using research agents for codebase understanding if the task involves unfamiliar systems*

## Phase 2: Solution Design
**BEFORE STARTING**: Update notes with "Phase 2: Solution Design - IN PROGRESS 🔄"

1. Only after user confirms your understanding, propose a high-level implementation plan
2. Discuss design alternatives and tradeoffs
3. Ask for feedback on your proposed approach
4. Work with the user to refine the implementation plan
5. Analyze existing patterns in the codebase to ensure consistency
6. Check for existing testing practices and documentation standards
7. Add a new section to your notes document describing the solution we've agreed upon. Include any helpful diagrams in MermaidJS format.
8. Explicitly request approval before proceeding to implementation

**AFTER COMPLETION**: Update notes with "Phase 2: Solution Design - COMPLETED ✅"

*Consider: dev-mentor agent for architectural guidance, but always validate recommendations with user*

## Phase 3: Implementation
**BEFORE STARTING**: Update notes with "Phase 3: Implementation - IN PROGRESS 🔄"

1. ONLY after explicit approval, begin implementing the solution
2. For complex implementations: Consider using TodoWrite tool to track multiple implementation tasks systematically
3. Work through the implementation methodically, tracking progress appropriately
4. For complex changes, show implementations and request feedback
5. Handle edge cases and add error resilience (consider `set -euo pipefail` interactions)
6. Test Layer Analysis: Explicitly identify what each test layer should focus on - avoid duplicating coverage between layers
7. Comprehensive Testing: Test all scenarios including edge cases (empty files, missing files, error conditions)
8. Exit Code Verification: For scripts, verify return code propagation through all function layers
9. User Feedback Incorporation: Pay close attention to user corrections and suggestions - they often lead to significantly improved solutions
10. Documentation Impact Check: Search all documentation files for references to changed behavior/functionality and update accordingly
11. Documentation Review: Check for "magical" patterns that need explanation - simple comments can prevent future confusion
12. **Update Current Status**: After each major implementation step, update notes with current step progress
13. Update the note file with any pertinent information (eg. key decisions, new information, etc.)
14. Prepare a detailed commit message describing the changes

**AFTER COMPLETION**: Update notes with "Phase 3: Implementation - COMPLETED ✅"

## Phase 4: Review
**BEFORE STARTING**: Update notes with "Phase 4: Review - IN PROGRESS 🔄"

1. Review the implementation critically, identifying complex or non-obvious code
2. Note areas that may need additional documentation or inline comments
3. Highlight potential future maintenance challenges
4. Suggest improvements for robustness, performance, or readability
5. Incorporate your own suggestions if you deem them valuable
6. Update the note file with anything you learned from the review or change you've made
7. Check if documentation needs updating

**AFTER COMPLETION**: Update notes with "Phase 4: Review - COMPLETED ✅"

*Consider: code-reviewer agent for comprehensive quality analysis, but validate all recommendations against actual requirements*

## Phase 5: Submit
**BEFORE STARTING**: Update notes with "Phase 5: Submit - IN PROGRESS 🔄"

1. Compose commit message following project constraints (check CLAUDE.md for commit rules like no Claude references)
2. Present commit message for review and get explicit approval
3. Commit changes to trunk/main
4. If documentation was updated, amend the commit to include doc changes
5. Add final commit ID to notes file

**AFTER COMPLETION**: Update notes with "Phase 5: Submit - COMPLETED ✅"

## Phase 6: Iterate
**BEFORE STARTING**: Update notes with "Phase 6: Iterate - IN PROGRESS 🔄"

1. Ask if we should iterate or refactor, if so go to the next step, if not go to the next phase
2. Iterate on the solution and look for opportunites to refactor.
3. Keep track of all approved iterations in the notes file
4. After each iteration perform the steps described in Phase 5.

**AFTER COMPLETION**: Update notes with "Phase 6: Iterate - COMPLETED ✅" (or SKIPPED if no iterations)

## Phase 7: Reflect
**BEFORE STARTING**: Update notes with "Phase 7: Reflect - IN PROGRESS 🔄"

1. Reflect on anything you have learned during this process, eg.
  - design discussions with me
  - refactorings and iterations done
  - issues found during testing
  - agent recommendations and their validation outcomes
2. **Meta-evaluation**: Evaluate how well the structured notes status tracking worked during this task:
  - Did the standardized status sections help with session continuity?
  - Were the mandatory status updates maintained consistently?
  - What friction or benefits did the enhanced notes structure provide?
  - Should this approach be refined or adjusted for future tasks?
3. Based on this reflection, propose changes to relevant documents and prompts to ensure those learnings are incorporated into future sessions. Consider artifacts such as:
  - md files at the project root
  - md files at in the docs directory
  - file-level documentation comments
  - base prompt (ie. CLAUDE.md)
  - this custom command prompt (ie. .claude/commands/build.md)
4. Update your notes with anything you've learned.
5. Ask if we should create new backlog tasks from these learnings now or defer task creation.

**AFTER COMPLETION**: Update notes with "Phase 7: Reflect - COMPLETED ✅"

## Phase 8: Clean Up
**BEFORE STARTING**: Update notes with "Phase 8: Clean Up - IN PROGRESS 🔄"

1. Add a task completion summary to the notes file documenting what was accomplished, files created/modified, and final outcome
2. Delete the original task file in `backlog/`.
3. Commit final changes (to notes and backlog)
4. Push changes to remote repository

**AFTER COMPLETION**: Update notes with "Phase 8: Clean Up - COMPLETED ✅"

# Important Rules
- NEVER write any implementation code during Phase 1 or 2
- ALWAYS get explicit approval before moving to each subsequent phase
- Break down problems into manageable components
- Consider edge cases and error handling in your design
- Use research tools to understand the codebase before proposing changes
- Examine similar functionality in the codebase to follow established patterns
- When in doubt, clarify with the user rather than making assumptions
- Include clear acceptance criteria in your implementation plan
- Add to your working note whenever you discover new information
- Whenever you learn something new, ensure that your note file is updated to reflect what you've learned
- When taking notes include permalinks to both internal and external resources whenever possible
- Always use MermaidJS when documenting designs or diagramming
- Keep your note file well organized with proper headings and a sensible information hierarchy
- Your note file MUST be formatted in markdown

# Critical Reminders

- **User approval gates**: Never proceed with major decisions without user confirmation - prevents overengineering and scope creep
- **Coverage monitoring**: Establish baseline, monitor throughout, verify maintenance - enables safe large-scale refactoring
- **Incremental implementation**: Small, verifiable steps with continuous validation - prevents catastrophic failures
- **Agent recommendation validation**: Question agent advice against actual requirements - user domain knowledge trumps agent suggestions
- **CLAUDE.md Compliance**: Always check CLAUDE.md for project-specific constraints (e.g., no Claude references in commits)
- **Documentation Consistency**: Search and update ALL documentation that might reference changed functionality
- **Test Quality**: When working with tests, prioritize atomic readability over code reuse - inline test data is preferable to constants for clarity
