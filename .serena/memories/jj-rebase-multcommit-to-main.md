# Rebasing Multi-Commit Branches onto Main in Jujutsu

## Step-by-Step Process

### 1. Check Your Current State
```bash
jj log --no-pager  # Verify branch exists and commits are visible
```

### 2. Duplicate the Branch (if commits are pushed/immutable)
```bash
jj duplicate -r <bookmark-name>
# Output shows new bookmark created, e.g., "abc123def456"
```
**Note:** If commits are unpushed, skip this step.

### 3. Rebase onto Main
```bash
jj rebase -b <bookmark-name> -d main
# For duplicated commits, use the new bookmark ID from step 2
```
This moves all commits in the branch onto `main` as a linear stack.

### 4. Verify All Changes Made It
```bash
# Compare the full diff between old and new location
jj diff -r <old-bookmark> -c main
# Should be empty if all changes are present

# Or inspect specific commits
jj show main  # See the new commits on main
```

### 5. Clean Up Orphaned Commits
```bash
# Abandon the old orphaned branch
jj abandon <old-bookmark>

# Delete the old bookmark if it exists
jj bookmark delete <old-bookmark-name>
```

### 6. Update Main Bookmark and Push
```bash
jj bookmark set main -r @-  # Update main bookmark to new commit
jj git push
```

---

## GitHub Considerations

**Critical:** If this branch had an open pull request on GitHub:
1. The old commit hashes are tracked by the PR
2. Your new commits have different hashes
3. **GitHub will NOT auto-detect the merge** — the PR remains open
4. **You must manually close the PR** on GitHub with a note like: "Changes merged to main locally via rebase. PR superseded."

**Better for GitHub workflows:** Use GitHub's UI to merge PRs (squash/rebase) instead. This keeps GitHub's PR system in sync automatically.

---

## Single-Line Reference
**Pushed commits:** `jj duplicate -r <branch> && jj rebase -b <new-id> -d main && jj abandon <old-branch>`
