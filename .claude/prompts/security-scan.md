# Security & Privacy Scan

Perform a comprehensive security and privacy audit of this repository.

## Scan Categories

### 1. Secrets & Credentials
Search for:
- API keys, tokens, passwords (patterns: `sk-`, `ghp_`, `xox[baprs]-`, etc.)
- Private keys and certificates (`.pem`, `.key`, `.p12`, `.pfx`, etc.)
- Database credentials and connection strings
- OAuth secrets and JWT tokens
- Environment variables with sensitive data

### 2. Personal Data
Search for:
- Email addresses (especially non-generic domains)
- Phone numbers, addresses, SSNs
- Names in non-code contexts (comments, logs, test data)
- User-specific paths or identifiers
- PII in configuration files

### 3. Configuration Files
Examine:
- `.env*` files and their `.gitignore` status
- Configuration files (`.config`, `.yml`, `.json`)
- Docker/container configurations
- CI/CD pipeline secrets

### 4. Git History
Check:
- Whether sensitive files are properly gitignored
- Commit messages for exposed credentials
- Author information (especially noreply patterns)
- Files that were committed then gitignored

### 5. Code Patterns
Look for:
- Hardcoded credentials in source code
- Debug/development endpoints left active
- Commented-out sensitive code
- TODO/FIXME comments revealing security issues

## Output Format

Provide:
1. **Executive Summary**: Risk level (Clean/Low/Medium/High/Critical)
2. **Critical Findings**: Any exposed secrets with file:line references
3. **Warnings**: Configuration issues or potential risks
4. **Clean Areas**: What's properly secured
5. **Recommendations**: Actionable next steps

## Scan Methodology

Use parallel tool execution:
- `Grep` for pattern matching (secrets, emails, tokens)
- `Glob` for finding sensitive file types
- `Read` for examining configuration files
- `Bash` for git history analysis when needed

Focus on actionable findings - distinguish between:
- **Active threats**: Real exposed credentials
- **False positives**: Documentation examples, test mocks
- **Hygiene issues**: Proper gitignore usage

Keep the scan focused and avoid overwhelming output.
