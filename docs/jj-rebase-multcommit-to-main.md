# Rebasing Multi-Commit Branches onto Main in Jujutsu

Note: The process below also works for rebasing a single commit onto main. Just replace whereever you see `<bookmark-name>` with `<ref-id>`.

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
# If the original had a bookmark, delete the bookmark
jj bookmark delete <old-bookmark-name>

# If the original revision was pushed/immutable:
# You CANNOT abandon it. It will remain visible in jj log output.
# There is no way to remove or hide it from the repository.
```
**Important Limitation:** Orphaned immutable (pushed) commits cannot be abandoned, hidden, or removed. They remain permanently visible in `jj log` output. This is a significant downside of rebasing locally when commits are already pushed to origin.

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
**Pushed commits:** `jj duplicate -r <branch> && jj rebase -b <new-id> -d main && jj bookmark delete <old-bookmark>`

**Caveat:** Immutable orphaned commits will remain visible in `jj log` forever. For this reason, **prefer merging through GitHub's UI** when possible to avoid littering your history with orphaned commits.
