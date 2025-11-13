# Publishing Workflow with Changesets

This guide explains how to version and publish the `@gobob/tokenlist` and `@gobob/bob-sdk` packages.

## Setup

The repository uses npm workspaces and Changesets for version management. The packages are linked:
- `@gobob/tokenlist` - Token list data and utilities
- `@gobob/bob-sdk` - BOB SDK (depends on tokenlist)

## Step 1: Make Changes

Work on your features in the `tokenlist/` or `sdk/` directories as usual.

## Step 2: Create a Changeset

When you're ready to document your changes:

```bash
npm run changeset
```

You'll be prompted to:
1. **Select packages**: Choose which packages changed (tokenlist, sdk, or both)
2. **Choose bump type**:
   - `major` - Breaking changes
   - `minor` - New features (backwards compatible)
   - `patch` - Bug fixes
3. **Write summary**: Describe what changed

This creates a markdown file in `.changeset/` that tracks your changes.

### Example Changeset Flow

```bash
$ npm run changeset
🦋  Which packages would you like to include?
◉ @gobob/tokenlist
◯ @gobob/bob-sdk

🦋  What kind of change is this for @gobob/tokenlist?
◯ major
◉ minor
◯ patch

🦋  Please enter a summary for this change:
Added new token configurations for Layer 2 chains
```

## Step 3: Commit Changesets

```bash
git add .changeset/
git commit -m "chore: add changeset for token list updates"
```

## Step 4: Version Packages (Before Release)

When ready to publish, update versions:

```bash
npm run changeset:version
```

This will:
- Update `version` in package.json files
- Update SDK's dependency on tokenlist if tokenlist changed
- Generate/update CHANGELOG files
- Delete consumed changeset files

**Important**: Review the version changes and commit them:

```bash
git add .
git commit -m "chore: version packages"
```

## Step 5: Build Packages

```bash
npm run build
```

This builds both packages in the correct order (tokenlist first, then SDK).

## Step 6: Publish to npm

```bash
npm run changeset:publish
```

This will:
1. Build both packages
2. Publish tokenlist to npm
3. Publish SDK to npm (with updated tokenlist dependency)
4. Create git tags for the releases

## Step 7: Push to GitHub

```bash
git push --follow-tags
```

## Important Notes

### Dependency Management

The SDK depends on tokenlist. When tokenlist version changes:
- If using `file:../tokenlist`: Update manually or use changeset version
- After publishing, consider updating SDK to use published version like `^1.0.0`

### Linked Packages

In `.changeset/config.json`, the packages are configured as linked:
```json
"linked": [["@gobob/tokenlist", "@gobob/bob-sdk"]]
```

This means when tokenlist gets a version bump, SDK automatically gets a patch bump.

### Publishing Order

Always publish in this order:
1. tokenlist (no dependencies)
2. SDK (depends on tokenlist)

The `changeset publish` command handles this automatically.

## Common Scenarios

### Scenario 1: Only Tokenlist Changed

```bash
npm run changeset  # Select only tokenlist
git add .changeset && git commit
npm run changeset:version
npm run changeset:publish
```

Result: Tokenlist gets new version, SDK gets patch bump (due to linked config)

### Scenario 2: Only SDK Changed

```bash
npm run changeset  # Select only SDK
git add .changeset && git commit
npm run changeset:version
npm run changeset:publish
```

Result: Only SDK gets new version

### Scenario 3: Both Changed

```bash
npm run changeset  # Select both packages
git add .changeset && git commit
npm run changeset:version
npm run changeset:publish
```

Result: Both get appropriate version bumps

## Troubleshooting

### Build Fails

Ensure tokenlist builds before SDK:
```bash
cd tokenlist && npm run build
cd ../sdk && npm run build
```

### Wrong Dependency Version

After publishing tokenlist, update SDK's package.json:
```json
"dependencies": {
  "@gobob/tokenlist": "^1.0.0"  // Use published version, not file:
}
```

Then create another changeset and publish SDK again.

## CI/CD Integration

Consider adding GitHub Actions:
1. Auto-create PR when changesets are added
2. Auto-publish when PR is merged to main
3. Create GitHub releases with changelog

See: https://github.com/changesets/action
