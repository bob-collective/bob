# BOB Gateway Documentation

Developer documentation for BOB Gateway - an intent-based bridge enabling Bitcoin users to access DeFi protocols with a single Bitcoin transaction.

## Development

**Prerequisites**:
- Node.js version 19 or higher
- A docs repository with a `docs.json` file

### Install the Mintlify CLI

Install the [Mintlify CLI](https://www.npmjs.com/package/mint) to preview your documentation changes locally:

```bash
npm i -g mint
```

### Preview locally

Run the following command at the root of your documentation, where your `docs.json` is located:

```bash
mint dev
```

View your local preview at `http://localhost:3000`.

### Custom ports

By default, Mintlify uses port 3000. You can customize the port using the `--port` flag:

```bash
mint dev --port 3333
```

### Update CLI

To ensure your local preview aligns with production, update the CLI:

```bash
mint update
```

### Validate links

To identify any broken links in your documentation:

```bash
mint broken-links
```

## Publishing changes

Install our GitHub app from your [dashboard](https://dashboard.mintlify.com/settings/organization/github-app) to propagate changes from your repo to your deployment. Changes are deployed to production automatically after pushing to the default branch.

## Troubleshooting

- **Dev environment isn't running**: Run `mint update` to ensure you have the most recent version of the CLI.
- **Page loads as a 404**: Make sure you are running in a folder with a valid `docs.json`.
- **Error with "sharp" module on darwin-arm64**: Remove the CLI (`npm remove -g mint`), upgrade to Node v19+, and reinstall (`npm i -g mint`).
- **Unknown error**: Delete the `~/.mintlify` folder and run `mint dev` again.

## Resources

- [Mintlify documentation](https://mintlify.com/docs)
- [CLI changelog](https://www.npmjs.com/package/mintlify?activeTab=versions)
