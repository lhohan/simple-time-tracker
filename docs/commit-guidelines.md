# Commit guidelines

## Quality Standards

- Only commit related changes

- Subject line should complete the sentence: "If applied, this commit will..."
  - Be concise
- Focus on WHAT and WHY, not HOW
- Use present tense, imperative mood ("add", "fix", "update", not "added", "fixed", "updated")
- Omit obvious details that can be inferred from the diff
- **Match verbosity to impact**: Small focused changes deserve concise messages; complex architectural changes warrant more detail
- Never reference Claude, Claude Code, or AI assistance in the commit message or as author

- Use conventional commit format (type(scope): description)
- No period at the end of the subject line
- Separate subject from body with a blank line
- Keep the subject line under 80 characters when possible
- Wrap body at 100 characters

## Conventional Commit Types ('type'):

- **feat**: new feature
- **fix**: bug fix
- **docs**: documentation changes
- **style**: formatting, missing semicolons, etc.
- **refactor**: code change that neither fixes a bug nor adds a feature
- **test**: adding or updating tests
- **chore**: maintenance tasks, dependency updates, changes to AI agents

## Message Structure:
```
type(scope): short description

[optional body explaining what and why]

[optional footer with issue references]
```

### Proportional Detail Guidelines:

**Small/focused changes** (single function, config tweak, simple bug fix):
- Concise subject line usually sufficient
- Example: `fix: handle empty input files correctly`

**Medium changes** (feature addition, refactoring multiple files):
- Subject + brief body explaining rationale
- Example: `feat: add user authentication` + body explaining approach

**Large/architectural changes** (major refactoring, new subsystems):
- Detailed body explaining design decisions and trade-offs
- Consider referencing ADRs for complex rationale
