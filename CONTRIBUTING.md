# Contributing

IEPM is designed to survive without a heroic central maintainer. Prefer small,
auditable changes to the static registry and preserve old lockfile behavior.
Read [the living project context](LIVING_CONTEXT.md) before changing schemas,
resolver behavior, registry vocabulary, artifact handling, or A5.

## Registry rules

- Treat releases as distinct compatibility facts; never generalize a claim from
  a package to all releases.
- Record `provenance` for compatibility and ordering claims.
- Unknown new game or mod releases are **unverified**, not implicitly supported.
- Use stable symbolic component labels when available; numeric WeiDU component
  IDs are execution details.
- Model EET, EET_End, and DLC Merger as platform/infrastructure phases.
- Canonicalize exact aliases only. Record forks and continuations as
  informational lineage; never silently substitute them for a requested mod.
- Add a new relationship condition or capability convention only with a real,
  documented ecosystem case and a regression fixture.
- Keep local game paths and secrets out of manifests and lockfiles. Bind named
  environments to machine paths outside the portable build description.

## Execution rules

- `analysis-only` lockfiles may be resolved and inspected but must not execute.
- A5 must first produce a non-mutating plan/preflight, then operate only in an
  IEPM-controlled disposable build workspace.
- Keep acquisition, materialization, and game-tree mutation as separate steps.

Run `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, and
`cargo test --workspace` before opening a pull request.
