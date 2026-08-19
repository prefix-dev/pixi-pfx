# pixi-pfx

A CLI tool that wraps the [prefix.dev](https://prefix.dev) GraphQL API. Designed as a pixi plugin (`pixi pfx`), with structured JSON output optimized for AI agent consumption.

Uses [cynic](https://cynic-rs.dev/) for compile-time schema validation — if the API schema changes and a field is removed or renamed, the build fails immediately.

## Quick Start

```bash
# Build
pixi run build

# Or with cargo directly
cargo build --release

# Install
pixi run install
```

## Global installation

To use as a global installation use:

```bash
# from git
pixi global install --git https://github.com/tdejager/pixi-pfx.git
# With the repository checked out locally
pixi global install  --path .
```

## Usage

All commands output JSON to stdout. Success responses use `{"ok": true, "data": ...}`, errors use `{"ok": false, "error": {"code": "...", "message": "..."}}`.

### Authentication

By default, `pixi-pfx` uses credentials from rattler's authentication storage, including OAuth credentials created by `pixi auth login` or `rattler auth login`. OAuth access tokens are refreshed automatically. An explicit token takes precedence:

```bash
# Uses rattler OAuth/auth storage
pixi-pfx auth whoami

# Explicit token override
pixi-pfx --token <TOKEN> auth whoami
# or
export PREFIX_DEV_API_TOKEN=<TOKEN>
pixi-pfx auth whoami
```

### Channels

```bash
# Get channel details (no auth needed for public channels)
pixi-pfx channel get conda-forge

# List channels
pixi-pfx channel list --public --limit 10
pixi-pfx channel list --owner myuser --order-by size --direction desc
pixi-pfx channel list --search conda --limit 5

# Create / update / delete (requires auth)
pixi-pfx channel create my-channel --description "My channel" --public
pixi-pfx channel update my-channel --description "Updated"
pixi-pfx channel delete my-channel

# CEP-6 channel notices
pixi-pfx channel add-notice my-channel maintenance "Maintenance starts at 20:00 UTC" --level warning --expires-at 2026-08-20T22:00:00Z
pixi-pfx channel update-notice my-channel maintenance "Maintenance moved to 21:00 UTC" --level warning
pixi-pfx channel delete-notice my-channel maintenance

# Members
pixi-pfx channel add-member my-channel someuser contributor
pixi-pfx channel remove-member my-channel someuser

# OIDC publishers
pixi-pfx channel add-github-oidc my-channel --owner org --repo repo --workflow build.yml
pixi-pfx channel add-gitlab-oidc my-channel --namespace group --project proj --workflow .gitlab-ci.yml
pixi-pfx channel add-google-oidc my-channel --email sa@project.iam.gserviceaccount.com
pixi-pfx channel delete-oidc my-channel <publisher-id>

# Transfer ownership
pixi-pfx channel transfer my-channel new-owner
```

### Packages

```bash
# Search by name (similarity-based)
pixi-pfx package search numpy --limit 10

# Get package details with variants
pixi-pfx package get conda-forge numpy --variants-limit 5

# List with filtering
pixi-pfx package list --name-contains scipy --order-by name

# Find by matchspec
pixi-pfx package matchspec "numpy>=2.0" --channel conda-forge

# Get a specific variant
pixi-pfx package variant conda-forge numpy linux-64 numpy-2.0.0-py312h1234.conda

# List versions
pixi-pfx package versions conda-forge numpy --limit 10

# Yank / unyank (requires auth)
pixi-pfx package yank my-channel linux-64 pkg-1.0.conda --reason "broken build"
pixi-pfx package unyank my-channel linux-64 pkg-1.0.conda

# Copy pinned package files into a channel asynchronously (requires auth)
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy my-channel \
  --packages '[{"url":"https://prefix.dev/conda-forge/linux-64/pkg-1.0.conda","sha256":"<64-hex-sha256>"}]'
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy-status <job-id>
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package active-copy my-channel

# Batch delete variants (requires auth)
pixi-pfx package batch-delete my-channel --entries '[{"subdir":"linux-64","filename":"pkg-1.0.conda"}]'
```

### API Keys

```bash
pixi-pfx auth api-key list
pixi-pfx auth api-key create my-key --description "CI key" --expires-at 2025-12-31T00:00:00Z
pixi-pfx auth api-key revoke my-key
pixi-pfx auth api-key delete my-key
```

### Agent Discovery

The `describe` command outputs a JSON schema of all available commands, arguments, types, and descriptions:

```bash
# Full command tree
pixi-pfx describe

# Specific command
pixi-pfx describe channel get
```

## Schema Management

The `schema.graphql` file is committed to the repo so builds work offline and in CI. To update it from the live API:

```bash
pixi run sync-schema
```

This fetches the latest schema via GraphQL introspection. If the schema changed in a breaking way, the next `cargo build` will fail with compile errors pointing to the exact fields that drifted.

## Testing

```bash
# Unit tests (no network)
pixi run test

# Integration tests against live prefix.dev API
pixi run test-online
```

## Project Structure

```
src/
  main.rs              Entry point, JSON envelope output
  cli.rs               Clap CLI definitions
  client.rs            PrefixClient: sends cynic operations via reqwest
  error.rs             Error types with JSON serialization
  schema.rs            cynic schema module registration
  commands/
    auth.rs            Auth command handlers
    channel.rs         Channel command handlers
    package.rs         Package command handlers
    describe.rs        Walks clap Command tree to JSON
  queries/
    common.rs          Scalars, enums, input types, shared fragments
    auth.rs            Viewer, API key query/mutation fragments
    channel.rs         Channel query/mutation fragments
    package.rs         Package query/mutation fragments
schema.graphql         Compile-time schema reference
pixi.toml              Pixi project config
```
