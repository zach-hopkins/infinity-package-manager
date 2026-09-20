# Package model

Package IDs are lowercase ASCII kebab-case and immutable once published. A
release is the unit of artifact identity and compatibility.

Each registry release records its version, artifact URL/hash, supported games,
provenance, install phase, ordering edges, dependencies, and components.
`provenance` names the source strength: `declared`, `derived`, `verified`,
`community`, or `unverified`.

Manifests contain intent. Lockfiles contain the resolver's exact choice,
including registry revision and resolved order. The schema permits adding game
fingerprints and toolchain information before execution is implemented.
