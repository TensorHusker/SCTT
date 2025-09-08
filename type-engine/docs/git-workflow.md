# Git Workflow Best Practices for SCTT Development

## Branch Strategy

### Branch Types
- `main` - Production-ready code
- `feature/*` - New features
- `fix/*` - Bug fixes  
- `docs/*` - Documentation updates
- `refactor/*` - Code refactoring
- `test/*` - Test improvements

### Branch Naming Convention
```
<type>/<ticket-number>-<short-description>
```

Examples:
- `feature/sctt-type-engine`
- `fix/42-path-composition-bug`
- `docs/improve-readme`

## Commit Best Practices

### Atomic Commits
Each commit should:
- Do ONE thing
- Be self-contained
- Pass all tests
- Be reversible without breaking other changes

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting (no code change)
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance

#### Examples:
```
feat(type-engine): Add bidirectional type checking

Implements Algorithm W-style bidirectional type checking
with synthesis and checking modes. This enables better
type inference and error messages.

Closes #123
```

```
fix(kan): Correct composition in Pi types

The previous implementation didn't properly handle
dependent types during composition. Now correctly
substitutes when composing under Pi.
```

## Git Commands Cheat Sheet

### Starting Work
```bash
git checkout main
git pull origin main
git checkout -b feature/my-feature
```

### During Development
```bash
# Stage specific files (not everything!)
git add <specific-files>

# Check what you're committing
git diff --staged

# Commit with message
git commit -m "feat: Add new feature"

# Or for longer messages
git commit  # Opens editor
```

### Keeping Up to Date
```bash
# Fetch latest changes
git fetch origin

# Rebase on main (preferred over merge)
git rebase origin/main

# If conflicts occur
git status  # See conflicts
# Fix conflicts in files
git add <fixed-files>
git rebase --continue
```

### Pushing Changes
```bash
# First push of new branch
git push -u origin feature/my-feature

# Subsequent pushes
git push

# After rebase (only on your own branch!)
git push --force-with-lease
```

## Common Scenarios

### Scenario 1: Made changes on wrong branch
```bash
git stash
git checkout correct-branch
git stash pop
```

### Scenario 2: Need to split a commit
```bash
git reset HEAD^  # Undo last commit, keep changes
git add -p  # Interactively stage parts
git commit -m "First logical change"
git add .
git commit -m "Second logical change"
```

### Scenario 3: Forgot something in last commit
```bash
git add forgotten-file
git commit --amend --no-edit
```

### Scenario 4: Wrong commit message
```bash
git commit --amend -m "New message"
```

## Code Review Checklist

Before requesting review:
- [ ] All tests pass
- [ ] Code follows style guide
- [ ] Commits are atomic and well-described
- [ ] Branch is up-to-date with main
- [ ] Documentation is updated
- [ ] No debugging code left

## Advanced Tips

### Interactive Rebase
Clean up commit history before merging:
```bash
git rebase -i HEAD~3  # Rebase last 3 commits
```

Options:
- `pick` - Keep commit
- `reword` - Change message
- `squash` - Combine with previous
- `fixup` - Combine, discard message
- `drop` - Remove commit

### Bisect for Bug Finding
```bash
git bisect start
git bisect bad  # Current version is bad
git bisect good <commit>  # Known good commit
# Git checks out middle commit
# Test and mark as good/bad
git bisect good/bad
# Repeat until bug commit found
git bisect reset
```

### Stash Management
```bash
git stash save "WIP: Feature X"
git stash list
git stash apply stash@{1}
git stash drop stash@{1}
git stash pop  # Apply and drop
```

## Golden Rules

1. **Never force push to main**
2. **Always pull before starting work**
3. **Commit early, commit often** (but keep them logical)
4. **Write meaningful commit messages**
5. **Review your own PR first**
6. **Keep commits atomic**
7. **Rebase over merge for cleaner history**
8. **Test before committing**

## SCTT-Specific Guidelines

For type theory code:
- Commit type definitions separately from implementations
- Keep soundness-critical changes isolated
- Document mathematical properties in commit messages
- Reference papers/theorems when implementing algorithms