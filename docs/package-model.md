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

Component selectors can carry claim-level provenance too. This lets a record
say that a numeric/LABEL pair was mechanically observed while a capability,
ordering relationship, or compatibility claim is separately author-declared or
community-curated. Upstream integration is valuable maintenance information,
but it is not required for a release to gain strong IEPM evidence through exact
artifacts and repeatable tests. In the public status vocabulary a release may
become `Supported` without upstream integration; `Verified` is reserved for an
exact configuration receipt.

Manifests contain intent. Lockfiles contain the resolver's exact choice:

- named game environments, each with a versioned fingerprint profile
- registry revision
- IEPM version and, when selected, WeiDU version
- exact opaque release IDs, display versions, selected archive content, and
  release-specific WeiDU component selectors
- installer program, release-specific language index, typed installer inputs,
  and materialization choices
- environment-aware dependency edges and the resolved execution-node graph
- `execution_readiness` and causal blocking reasons when the graph is useful
  for analysis but not eligible for execution

Unknown metadata is omitted rather than invented. An omitted game fingerprint,
artifact, or WeiDU version makes the lockfile honest about what it cannot yet
reproduce; `iepm verify` will later turn those gaps into actionable checks.

Currently A4 prepares explicit `zip` artifacts. They are cached by SHA-256,
verified before use, may have fallback mirrors, and are extracted only after
rejecting unsafe, duplicate, case-colliding, and symbolic-link paths. An empty
`platforms` or `architectures` list denotes a portable artifact; a nonempty
list is an allow-list.

The registry can additionally record an `executable` artifact format so a
released Windows EXE has a URL, platform, and SHA-256 without being mislabeled
as a ZIP. It remains deliberately non-preparable and non-executable until a
release-specific materialization fixture exists; this is evidence retention,
not generic EXE support.
