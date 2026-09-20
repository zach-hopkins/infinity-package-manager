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

The repository contains the Product A foundation:

- YAML package, manifest, and lockfile schemas
- static, forkable registry records with provenance
- a Rust registry reader and deterministic resolver prototype
- environment-aware dependency, exclusive-capability, phase, and `before`/`after` validation
- a fixture for the planned conservative EET stack

Schema 2 is the current public contract. It describes named game environments,
opaque release IDs, release-specific WeiDU component selectors, installer
language/inputs, materialization metadata, and an explicit execution graph.
The resolver reads legacy schema-1 manifests and registry records as a
migration convenience, but always emits schema-2 lockfiles.

Release text is not assumed to be SemVer. A plain value is an exact release
ID/display-version match; `id:<release-id>` is an explicit opaque ID; and a
range such as `>=8, <9` opts into SemVer projection when registry metadata
provides one. Lockfiles preserve the selected graph, environments, artifacts,
installer decisions, registry revision, and toolchain identity.

The `fetch` command downloads HTTPS ZIP artifacts into a content-addressed
cache, verifies SHA-256 values, tries declared mirrors, bounds downloads, and
safely extracts them. Extraction rejects unsafe, duplicate, and
case-colliding paths and revalidates cached extracted content. The next
milestone is still WeiDU execution; no GUI or semantic compiler is in scope.

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
