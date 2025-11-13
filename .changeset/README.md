# Changesets

This directory contains the changeset configuration for managing versions and publishing the `@gobob/tokenlist` and `@gobob/bob-sdk` packages.

## How to use

### Creating a changeset

When you make changes that should trigger a version bump, create a changeset:

```bash
npm run changeset
```

This will prompt you to:
1. Select which packages have changed
2. Choose the type of change (major/minor/patch)
3. Write a summary of the changes

The changeset will be saved as a markdown file in this directory.

### Versioning packages

When you're ready to create new versions:

```bash
npm run changeset:version
```

This will:
1. Update package versions based on changesets
2. Update the SDK's dependency on tokenlist if needed
3. Delete consumed changeset files
4. Update CHANGELOG files (if enabled)

### Publishing packages

After versioning, publish the packages:

```bash
npm run changeset:publish
```

This will:
1. Build and publish tokenlist first
2. Build and publish the SDK (which depends on the new tokenlist version)

## Linked packages

The tokenlist and SDK are configured as linked packages. This means when tokenlist gets a version bump, the SDK will automatically get a patch bump to use the new tokenlist version.

## Important notes

- Always publish tokenlist before SDK since SDK depends on it
- The `linked` configuration ensures version consistency
- Commit your changesets to git before running version/publish
