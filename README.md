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

## Current status

The repository contains the Product A foundation:

- YAML package, manifest, and lockfile schemas
- static, forkable registry records with provenance
- a Rust registry reader and deterministic resolver prototype
- dependency, exclusive-capability, phase, and `before`/`after` validation
- a fixture for the planned conservative EET stack

The resolver supports SemVer requirements for package releases and stable,
symbolic component selection. Its lockfiles preserve the resolved graph,
registry revision, game data, artifacts when known, and toolchain identity.
The `fetch` command downloads HTTPS ZIP artifacts into a content-addressed
cache, verifies SHA-256 values, and safely extracts them. The next milestone is
WeiDU execution. No GUI or semantic compiler is in scope yet.

For example, a manifest can select a version range and symbolic component IDs:

```yaml
mods:
  - package: tactics-remix
    version: ">=8.0, <9.0"
    components: [tactical-encounters]
```

The optional `version` field uses [SemVer requirement syntax](https://docs.rs/semver/latest/semver/struct.VersionReq.html). Use `=8.2` for an exact release.

Pass `--weidu-version` to record the exact executor expected for a resolved
installation. If the registry has not yet established an artifact or game
fingerprint, IEPM leaves it absent instead of fabricating a claim.

To prepare the resolved artifacts for a lockfile:

```text
cargo run -p iepm -- fetch --lockfile modpack.lock.json --cache .iepm-cache
```

Artifact URLs must use HTTPS. ZIP extraction rejects unsafe paths and enforces
an 8 GiB uncompressed-size limit; registries may restrict an artifact to
`windows`, `linux`, or `macos`.

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
