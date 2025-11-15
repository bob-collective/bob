# Changesets

This directory contains [Changesets](https://github.com/changesets/changesets) for managing versioning and releases of `@gobob/tokenlist`.

## How to use Changesets

### Adding a changeset

When you make changes that should trigger a release, create a changeset:

```bash
npm run changeset
```

This will prompt you to:
1. Select which packages are affected (if using a monorepo)
2. Choose the type of change (major, minor, or patch)
3. Write a summary of the changes

A new file will be created in `.changeset/` with a unique name.

### Releasing

The release process is automated via GitHub Actions:

1. **On Pull Requests**: The workflow checks if a changeset file exists. If not, it will comment on the PR asking for one.

2. **On Push to Master**: The workflow will:
   - Version the package based on changesets
   - Update the changelog
   - Create a release PR (if there are changesets to release)
   - Publish to npm (if the release PR is merged)

### Manual release (if needed)

If you need to manually release:

```bash
# Version packages (updates version in package.json and creates changelog)
npm run version

# Build and publish to npm
npm run release
```

## Best Practices

- Always create a changeset for user-facing changes
- Use clear, descriptive summaries in changesets
- Review the generated changelog before merging release PRs
- Don't commit changeset files directly to master - they should come from PRs

