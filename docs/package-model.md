# Package model

Package IDs are lowercase ASCII kebab-case and immutable once published.
Aliases preserve an exact historical identity after a rename. Lineage records
forks, predecessors, and continuations but never substitutes one package for
another during resolution. A release has an opaque
`release_id`, a human display version, and an optional SemVer projection; none
of those is assumed to equal a Git tag, TP2 `VERSION`, or artifact hash.

Each registry release records content-addressed artifact locations and mirrors,
materialization instructions, supported games, claim-level provenance,
relationships, installers, and components. Components retain a stable IEPM ID
and may map per release to a TP2 path, `LABEL`, numeric fallback, and
subcomponent group. `provenance` names source strength: `declared`, `derived`,
`verified`, `community`, or `unverified`; behavioral and compatibility claims
must not inherit that status merely because structural data did.

Manifests contain intent. Lockfiles contain the resolver's exact choice:

- named game environments, each with a versioned fingerprint profile
- registry revision
- IEPM version and, when selected, WeiDU version
- exact opaque release IDs, display versions, selected archive content, and
  release-specific WeiDU component selectors
- language, typed installer inputs, and materialization choices
- environment-aware dependency edges and the resolved execution-node graph
- `execution_readiness` and causal blocking reasons when the graph is useful
  for analysis but not eligible for execution

Unknown metadata is omitted rather than invented. An omitted game fingerprint,
artifact, or WeiDU version makes the lockfile honest about what it cannot yet
reproduce; `iepm verify` will later turn those gaps into actionable checks.

Currently A4 supports explicit `zip` artifacts. They are cached by SHA-256,
verified before use, may have fallback mirrors, and are extracted only after
rejecting unsafe, duplicate, case-colliding, and symbolic-link paths. An empty
`platforms` or `architectures` list denotes a portable artifact; a nonempty
list is an allow-list.
