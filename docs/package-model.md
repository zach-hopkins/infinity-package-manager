# Package model

Package IDs are lowercase ASCII kebab-case and immutable once published. A
release is the unit of artifact identity and compatibility.

Each registry release records its version, artifact URL/hash, supported games,
provenance, install phase, ordering edges, dependencies, and components.
`provenance` names the source strength: `declared`, `derived`, `verified`,
`community`, or `unverified`.

Manifests contain intent. Lockfiles contain the resolver's exact choice:

- game target, version, and supplied fingerprint
- registry revision
- IEPM version and, when selected, WeiDU version
- exact package versions and symbolic components
- artifact URL, SHA-256, archive format, and optional platform restrictions
  when the registry has verified artifact data
- direct dependency edges, including their constraints and required components
- resolved install order

Unknown metadata is omitted rather than invented. An omitted game fingerprint,
artifact, or WeiDU version makes the lockfile honest about what it cannot yet
reproduce; `iepm verify` will later turn those gaps into actionable checks.

Currently A4 supports explicit `zip` artifacts. They are cached by SHA-256,
verified before use, and extracted only after rejecting unsafe archive paths.
An empty `platforms` list denotes a portable artifact; a nonempty list is an
allow-list for the intended host platforms.
