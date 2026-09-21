# Infinity Package Manager

**IEPM** is a standards-first package manager for Infinity Engine mods. It keeps
[WeiDU](https://www.weidu.org/) as the execution backend while adding a modern,
reproducible control layer above it.

> WeiDU solved: “How do I safely patch Baldur's Gate?”
>
> IEPM aims to solve: “How does an ecosystem of independently authored patches
> behave like a modern package repository?”

## The four questions

| Layer | Question | Purpose |
| --- | --- | --- |
| Manifest | What does the player want? | Small, shareable human intent. |
| Registry | What releases and components exist? | Versioned ecosystem knowledge, evidence, phases, dependencies, and conflicts. |
| Lockfile | What exact solution was chosen? | A reproducible resolved build. |
| Semantic model | What do components mean? | A later enhancement for safe merging and useful conflict explanations. |

Product A is deliberately limited to package management: stable IDs, releases,
artifacts, compatibility, dependency resolution, capabilities, ordering, and
reproducible lockfiles. It does not need to understand arbitrary WeiDU programs.

The repository's authoritative project memory is
[LIVING_CONTEXT.md](LIVING_CONTEXT.md). It applies equally to human
contributors and coding agents: schema evolution must be evidence-led, EET
remains multi-environment, lineage never silently substitutes a fork, and only
an explicitly executable lockfile may reach A5 mutation.

## Current status

The repository contains the Product A foundation and first CLI workflow:

- YAML package, manifest, and lockfile schemas
- static, forkable registry records with provenance
- a Rust registry reader and deterministic resolver prototype
- environment-aware dependency, exclusive-capability, phase, and `before`/`after` validation
- a fixture for the planned conservative EET stack
- search, preview-first manifest add, portable lockfile verification, and managed full-copy workspace commands

Manifest/package records use schema 2; replayable lockfiles use schema 3. The
current contracts describe named game environments, opaque release IDs,
release-specific WeiDU component selectors, installer language/inputs,
preflight baselines, materialization metadata, explicit execution graphs, and
A5 readiness.
The resolver reads legacy schema-1 manifests and registry records as a
migration convenience, but always emits schema-3 lockfiles.

Release text is not assumed to be SemVer. A plain value is an exact release
ID/display-version match; `id:<release-id>` is an explicit opaque ID; and a
range such as `>=8, <9` opts into SemVer projection when registry metadata
provides one. Lockfiles preserve the selected graph, environments, artifacts,
installer decisions, registry revision, and toolchain identity.

The `fetch` command downloads HTTPS ZIP artifacts into a content-addressed
cache, verifies SHA-256 values, tries declared mirrors, bounds downloads, and
safely extracts them. Extraction rejects unsafe, duplicate, and
case-colliding paths and revalidates cached extracted content. A6 also offers
inert TP2 structural inspection and release-drift review for registry curators;
it never executes TP2 source or updates YAML automatically. See
[the A6 boundary](docs/a6-tp2-ingestion.md).

The first A5 slice is available as a non-mutating preflight:

```text
cargo run -p iepm -- plan --lockfile modpack.lock.json
```

It accepts only schema-3 `executable` lockfiles and renders the intended
materialization and WeiDU actions. It never fetches, starts a process, or
changes a game tree.

The narrow A5 executor is available for a fresh **managed** disposable copy
only. Use `iepm snapshot` and `iepm workspace` to create the full-copy
workspaces first; `execute` (also available as `install`) rejects source
snapshots, sealed builds, and unmanaged paths. It requires every named
environment to be bound at runtime (not recorded in the lockfile), an explicit
`--confirm-disposable`, a shared WeiDU binary, and a log directory. It verifies
and materializes artifacts before executing in
graph order, verifies the lockfile's core-layout fingerprint before mutation,
and retains a command/stdout/stderr receipt plus final workspace fingerprints.
An existing run-state marker is never resumed in place: use a new controlled
copy and log directory after interruption. Every WeiDU launcher—bundled or
shared—requires both its numeric mod language and the game's locale; IEPM
renders those before unattended flags and component selection.
`--allow-weidu-warnings` is an explicit review decision: it accepts only
WeiDU exit code 3 after the requested components are confirmed in `WeiDU.log`.

The smallest executable EET route fixture is available at
`examples/eet-minimal/modpack.yaml`. It uses a BG2EE target workspace with
`after_eet_import: eet`: preparation and EET import run against BG2EE, while
EET_End runs against the transformed EET result. Its fingerprints were measured
from the exact clean Steam 2.6.6 copies used for the evidence run, so they are
not universal game fingerprints. All WeiDU routes also require a game `locale` such
as `en_US`; `plan` renders that locale, the bound workspace, and the
noninteractive WeiDU flags separately from each package's numeric WeiDU
language index.

An environment may declare an ordered `baseline` of already-installed WeiDU
log entries. `plan` renders that as a verify-only preflight and refuses to
schedule a matching component for installation. EET's copied
`WeiDU-BGEE.log` remains source-history provenance, not a second target plan;
see [the EET execution evidence](docs/a5-eet-evidence.md) and [bundled launcher
evidence](docs/a5-hidden-gameplay-options-evidence.md).

For example, a manifest can select a version range and symbolic component IDs:

```yaml
mods:
  - package: tactics-remix
    environment: eet-target
    version: ">=8.0, <9.0"
    components: [tactical-encounters]
```

Use `version: "8.2"` for an exact opaque upstream version. SemVer range syntax
uses [SemVer requirement syntax](https://docs.rs/semver/latest/semver/struct.VersionReq.html)
only when the range contains an explicit range operator.

Pass `--weidu-version` to record the exact executor expected for a resolved
installation. If the registry has not yet established an artifact or game
fingerprint, IEPM leaves it absent instead of fabricating a claim.

To prepare the resolved artifacts for a lockfile:

```text
cargo run -p iepm -- fetch --lockfile modpack.lock.json --cache .iepm-cache
```

Artifact URLs must use HTTPS. An artifact is identified by SHA-256, may list
mirrors, and may be restricted by platform and CPU architecture. ZIP extraction
rejects unsafe paths and enforces an 8 GiB uncompressed-size limit.

To execute an already-resolved minimal route, use paths created by `iepm workspace`:

```text
cargo run -p iepm -- execute --lockfile modpack.lock.json --cache .iepm-cache \
  --weidu C:\\tools\\weidu.exe --workspace bgee-source=C:\\testing\\bgee-source \
  --workspace eet-target=C:\\testing\\bg2ee-target --log-dir C:\\testing\\iepm-logs \
  --confirm-disposable
```

To inspect a candidate workspace's fingerprint without mutating it:

```text
cargo run -p iepm -- fingerprint --workspace C:\\testing\\bgee-source --locale en_US
```

The CLI workflow, including `search`, preview-first `add`, non-mutating
`verify`, full-copy `snapshot`/`workspace`, `install`, and `seal`, is described
in [the A7 workflow guide](docs/a7-cli-workflow.md). IEPM deliberately does not
yet reuse prefix checkpoints: a safe cache key needs more evidence than the
core-layout fingerprint.

To prepare curator-review evidence from a known local TP2 without running it:

```text
cargo run -p iepm -- inspect-tp2 --tp2 C:\\mods\\Example\\setup-Example.tp2
cargo run -p iepm -- review-tp2 --registry registry --package example \
  --release-id example-release --tp2 C:\\mods\\Example\\setup-Example.tp2
```

`match` confirms only version/language/component-selector agreement. It does
not verify compatibility, installer behavior, or a release's artifact.

## Try it

With Rust 1.85 or newer installed:

```text
cargo run -p iepm -- resolve \
  --registry registry \
  --manifest examples/eet-balanced/modpack.yaml \
  --output modpack.lock.json
```

The resulting `modpack.lock.json` is the authoritative resolved build;
`WeiDU.log` will eventually be an execution receipt, not the lockfile.

## Repository layout

```text
crates/       Rust reference implementation
docs/         Architecture and data-model decisions
schemas/      Language-independent contracts
registry/     Static package records
examples/     Reproducible manifest fixtures
tests/        Resolver fixture coverage
```

See [ROADMAP.md](ROADMAP.md), [docs/architecture.md](docs/architecture.md), and
[CONTRIBUTING.md](CONTRIBUTING.md).
