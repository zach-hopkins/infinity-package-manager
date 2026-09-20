# Contributing

IEPM is designed to survive without a heroic central maintainer. Prefer small,
auditable changes to the static registry and preserve old lockfile behavior.

## Registry rules

- Treat releases as distinct compatibility facts; never generalize a claim from
  a package to all releases.
- Record `provenance` for compatibility and ordering claims.
- Unknown new game or mod releases are **unverified**, not implicitly supported.
- Use stable symbolic component labels when available; numeric WeiDU component
  IDs are execution details.
- Model EET, EET_End, and DLC Merger as platform/infrastructure phases.

Run `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, and
`cargo test --workspace` before opening a pull request.
