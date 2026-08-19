# pixi-pfx

A command-line client for the [prefix.dev](https://prefix.dev) GraphQL API, shipped as a pixi plugin
(`pixi pfx`). Manage channels, packages and API keys from a terminal or from CI — readable tables by
default, and a stable JSON envelope behind `--json` for scripts and AI agents.

Queries are checked against the prefix.dev schema at compile time with
[cynic](https://cynic-rs.dev/), so a renamed or removed field breaks the build instead of shipping a
release that fails at runtime.

## Install

Every push to `main` builds `linux-64`, `osx-64`, `osx-arm64` and `win-64` packages and publishes
them to the `pixi-extensions` channel on beta.prefix.dev, so the quickest way in is:

```bash
pixi global install -c https://beta.prefix.dev/pixi-extensions pixi-pfx
```

To build from source instead:

```bash
# from git
pixi global install --git https://github.com/prefix-dev/pixi-pfx.git

# from a local checkout
pixi global install --path .

# plain cargo — binary lands in target/release/pixi-pfx
cargo build --release
```

## Authenticate

pixi-pfx reads rattler's authentication storage — the same credentials that `pixi auth login` and
`rattler auth login` write. If you are already logged in to prefix.dev there is nothing to set up,
and OAuth access tokens are refreshed automatically:

```console
$ pixi-pfx auth whoami
Logged in as: wolfv
```

To act with a specific API key instead — in CI, for example — pass `--token` or set
`PREFIX_DEV_API_TOKEN`. An explicit token always takes precedence over stored credentials:

```bash
export PREFIX_DEV_API_TOKEN=<TOKEN>
pixi-pfx auth whoami

# equivalent
pixi-pfx --token <TOKEN> auth whoami
```

Read-only queries against public channels need no credentials at all.

## Output

By default commands print a key/value block for a single object and a table for a list:

```console
$ pixi-pfx package search numpy --limit 3
NAME   CHANNEL               VERSION  PLATFORMS                       SUMMARY
numpy  conda-forge           2.5.2    linux-64, linux-aarch64, linu…  The fundamental package for scientific …
numpy  emscripten-forge-dev  2.4.4    emscripten-wasm32               The fundamental package for scientific …
numpy  emscripten-forge-3x   2.4.4    emscripten-wasm32               The fundamental package for scientific …
Page 1/1 (3 total)
```

Long cells are truncated to keep the table narrow, and paginated commands print a
`Page x/y (n total)` footer.

Add `--json` for a stable envelope — this is what scripts and agents should consume:

```jsonc
// success
{"ok": true, "data": { ... }}

// failure
{"ok": false, "error": {"code": "GRAPHQL_ERROR", "message": "...", "details": { ... }}}
```

Field names inside `data` are snake_case. `details` is only present for GraphQL errors. Successful
commands exit `0`, failures exit `1`; in table mode the error goes to stderr as
`error [CODE]: message`. Error codes are stable strings:

| Code | Meaning |
| --- | --- |
| `HTTP_ERROR` | The request to the endpoint failed. |
| `GRAPHQL_ERROR` | The API returned GraphQL errors, or the response could not be decoded. |
| `AUTH_STORAGE_ERROR` | rattler's credential storage could not be opened or refreshed. |
| `INVALID_ARGUMENT` | An argument was malformed, e.g. invalid JSON in `--packages`. |
| `JSON_ERROR` | Serializing or deserializing JSON failed. |

## Global options

| Option | Default | Description |
| --- | --- | --- |
| `--json` | off | Emit the JSON envelope instead of tables. |
| `--token <TOKEN>` | rattler auth storage | API token to authenticate with; also read from `PREFIX_DEV_API_TOKEN`. |
| `--endpoint <URL>` | `https://prefix.dev/api/graphql` | GraphQL endpoint. Point it at `https://beta.prefix.dev/api/graphql` to work against beta. |

All three are global and may appear anywhere on the command line.

## Channels

### Find and inspect

```bash
# details for one channel — public channels need no auth
pixi-pfx channel get conda-forge

# list channels
pixi-pfx channel list --limit 10
pixi-pfx channel list --owner myuser --order-by size --direction desc
pixi-pfx channel list --search conda --limit 5
```

`channel list` is paginated with `--limit` (default 25) and `--page` (0-indexed). `--order-by`
accepts `name`, `size`, `created-at`, `package-count`, `namespace` and `billing-owner`, combined with
`--direction asc|desc`. Note that `--search` *orders* results by name similarity rather than
filtering them, so the reported total stays the same.

### Create, update, delete

```bash
pixi-pfx channel create my-channel --description "My channel" --public
pixi-pfx channel update my-channel --description "Updated" --public false
pixi-pfx channel delete my-channel
```

On `create`, `--public` is a flag; on `update` it takes an explicit `true`/`false`, so you can turn
visibility off again. Both commands also accept `--logo <URL>`, the CEP-0042 relations
`--relation-base` / `--relation-overrides`, and `--allow-v3-uploads true|false` for packages that
need v3 repodata.

### Members and trusted publishers

```bash
# roles: owner, contributor, viewer
pixi-pfx channel add-member my-channel someuser contributor
pixi-pfx channel remove-member my-channel someuser

# OIDC publishers, for keyless uploads from CI
pixi-pfx channel add-github-oidc my-channel --owner org --repo repo --workflow build.yml
pixi-pfx channel add-gitlab-oidc my-channel --namespace group --project proj --workflow .gitlab-ci.yml
pixi-pfx channel add-google-oidc my-channel --email sa@project.iam.gserviceaccount.com
pixi-pfx channel delete-oidc my-channel <publisher-id>

# hand the channel to someone else
pixi-pfx channel transfer my-channel new-owner
```

The GitHub and GitLab publishers take an optional `--environment`; the Google publisher takes an
optional `--sub` constraint. `channel get` lists the configured publishers with their ids.

### Notices

[CEP-6](https://github.com/conda/ceps/blob/main/cep-0006.md) notices are shown on the channel page
and served to compatible conda clients:

```bash
pixi-pfx channel add-notice my-channel maintenance "Maintenance starts at 20:00 UTC" \
  --level warning --expires-at 2026-08-20T22:00:00Z
pixi-pfx channel update-notice my-channel maintenance "Maintenance moved to 21:00 UTC" --level warning
pixi-pfx channel delete-notice my-channel maintenance
```

The argument after the channel is the stable notice id. `--level` is `info` (default), `warning` or
`critical`, and `--expires-at` takes an RFC 3339 timestamp.

## Packages

```bash
# search by name, ordered by similarity
pixi-pfx package search numpy --limit 10

# package details, including variants
pixi-pfx package get conda-forge numpy --variants-limit 5

# list with a name filter
pixi-pfx package list --name-contains scipy --order-by name

# resolve a matchspec — --channel is required and may be repeated
pixi-pfx package matchspec "numpy>=2.0" --channel conda-forge

# a single variant, and all versions of a package
pixi-pfx package variant conda-forge numpy linux-64 numpy-2.0.0-py312h1234.conda
pixi-pfx package versions conda-forge numpy --limit 10
```

`package list` orders by `name`, `last-created-date` or `total-size`; `search`, `list` and `versions`
all take `--limit` / `--page`.

### Yank and delete

```bash
pixi-pfx package yank my-channel linux-64 pkg-1.0.conda --reason "broken build"
pixi-pfx package unyank my-channel linux-64 pkg-1.0.conda

pixi-pfx package batch-delete my-channel \
  --entries '[{"subdir":"linux-64","filename":"pkg-1.0.conda"}]'
```

Yanking keeps the file but hides it from solvers; `batch-delete` removes variants for good.

### Copy packages into a channel

Copying between channels resolves every matching source variant and submits it to the asynchronous
copy job. Multiple package names are accepted, and optional version/platform filters narrow the
selection:

```bash
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy-from-channel \
  destination-channel source-channel numpy scipy

pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy-from-channel \
  destination-channel source-channel numpy --version 2.3.0 --platform linux-64
```

Packages can also be snatched from arbitrary URLs. Each entry pins a `url` together with its
expected `sha256`, so the copy either reproduces exactly that file or fails:

```bash
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy my-channel \
  --packages '[{"url":"https://prefix.dev/conda-forge/linux-64/pkg-1.0.conda","sha256":"<64-hex-sha256>"}]'

# Follow a job by id, or ask what is currently running for a channel
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package copy-status <job-id>
pixi-pfx --endpoint https://beta.prefix.dev/api/graphql package active-copy my-channel
```

## API keys

```bash
pixi-pfx auth api-key list
pixi-pfx auth api-key create my-key --description "CI key" --expires-at 2026-12-31T00:00:00Z
pixi-pfx auth api-key revoke my-key
pixi-pfx auth api-key delete my-key
```

The key value is returned only on creation — store it right away. Revoking disables the key but
keeps the record; deleting removes it entirely.

## Scripts and agents

`describe` prints the command tree — subcommands, arguments, types and help text — as JSON, so a
script or an agent can discover the surface without scraping `--help`. It always emits JSON, with or
without `--json`:

```console
$ pixi-pfx describe channel get
{
  "args": [
    {
      "description": "Channel name",
      "name": "name",
      "required": true,
      "type": "string"
    }
  ],
  "description": "Get channel details by name"
}
```

Leave out the command path to get the whole tree, including the global flags:

```bash
pixi-pfx describe
```

Together with `--json` that makes pixi-pfx self-describing: discover the commands, run one, read the
result out of the envelope.

```bash
pixi-pfx --json channel get conda-forge | jq '.data.base_url'
```

## Development

```bash
# build
pixi run build          # cargo build --release
pixi run build-debug

# tests
pixi run test           # unit tests, no network
pixi run test-online    # integration tests against the live prefix.dev API

# regenerate COMMANDS.json from the clap tree
pixi run generate-docs
```

### Schema management

`schema.graphql` is committed so builds work offline and in CI. Refresh it from the live API with:

```bash
pixi run sync-schema
```

That fetches the schema by introspection. If something drifted in a breaking way, the next
`cargo build` fails with errors pointing at the exact fields that changed.

### Project structure

```
src/
  main.rs              Entry point: dispatch, output kind selection, exit codes
  cli.rs               Clap CLI definitions
  client.rs            PrefixClient: sends cynic operations via reqwest, resolves credentials
  format.rs            Table/key-value rendering and the JSON envelope
  error.rs             Error types, codes and JSON serialization
  schema.rs            cynic schema module registration
  commands/
    auth.rs            Auth and API key handlers
    channel.rs         Channel handlers
    package.rs         Package handlers
    describe.rs        Walks the clap Command tree to JSON
  queries/
    common.rs          Scalars, enums, input types, shared fragments
    auth.rs            Viewer and API key fragments
    channel.rs         Channel query/mutation fragments
    package.rs         Package query/mutation fragments
schema.graphql         Compile-time schema reference
pixi.toml              Pixi workspace, package and tasks
```
