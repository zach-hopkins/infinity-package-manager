# IEPM agent context

This repository is building a package-management control plane above WeiDU.
Treat the public schemas, registry vocabulary, and lockfile semantics as
long-lived APIs. Do not make a schema change solely because it is cleaner in
code; favor small additions that preserve old lockfile meaning.

## Non-negotiable boundaries

- EET is a multi-environment transformation: model its source and target game
  roots explicitly. A phase is a barrier only within one environment; graph
  edges express real ordering and cross-environment dependencies.
- A stable IEPM component ID is user intent. Its TP2, `LABEL`, numeric fallback,
  and subcomponent mapping are release-specific implementation details.
- Release IDs are opaque. Display version, SemVer projection, Git tag, TP2
  version, artifact, and repository state are distinct facts.
- SHA-256 identifies artifact content. URLs are locations and may disappear;
  mirrors must not change content identity.
- `aliases` may canonicalize an exact historical package identity. `lineage`
  is informational only: never silently replace an abandoned package with a
  fork or continuation.
- Unknown compatibility is unverified, not incompatible. Do not turn parser
  evidence or inherited structural metadata into verified behavioral claims.
- A resolved lockfile may be `analysis-only`. A5 execution must require an
  `executable` preflight with verified artifact, installer, component selector,
  and required portable inputs.
- Lockfiles must not contain machine-specific absolute paths or secrets. Bind
  named environments to local filesystem paths outside the portable lockfile.

## Scope discipline

- Keep code concrete Rust structs and functions. Do not introduce generic
  frameworks, trait hierarchies, or a relationship DSL without a demonstrated
  real-mod case that the current primitives cannot represent.
- Add relationship conditions only when a popular, documented mod requires
  them. `requires`, `optional`, `recommends`, `conflicts`, `before`, and `after`
  are intentionally the small initial vocabulary.
- Preserve the distinction between artifact acquisition and package
  materialization. A4 prepares verified content; A5 owns game-tree mutation.
- A5 planning/preflight is non-mutating. Do not add real execution until a
  trusted fixture validates the plan against a manual install, and never
  execute against a user's only game installation.

Read [LIVING_CONTEXT.md](LIVING_CONTEXT.md) before changing schemas, resolver
behavior, registry semantics, artifact handling, or the A5 execution boundary.
