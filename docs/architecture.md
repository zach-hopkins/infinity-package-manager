# Architecture

IEPM models an installation as an environment-aware graph of state
transformations. Order can change meaning, so the resolver makes graph edges
explicit; phases are conventional hard barriers within one environment.

```text
manifest → registry → resolver → lockfile → verified artifacts → WeiDU execution
```

The registry is static YAML in Git: versioned, forkable, and independent of a
server. A client resolves against a named registry revision; old lockfiles stay
reproducible even if future releases become unverified.

Resolution and execution are intentionally separate states. A resolver may
produce an `analysis-only` lockfile for an unverified or incomplete graph, but
A5 must preflight and reject it before filesystem mutation. Portable lockfiles
refer to named environments rather than local absolute game paths.

EET is a platform transform rather than an ordinary content package. A schema-2
plan names at least a BGEE/SoD source environment and a BG2EE target workspace.
When the target declares `after_eet_import: eet`, packages through the
`eet-import` phase execute against BG2EE; later phases execute against the
transformed EET workspace with the same portable environment name. This allows
pre-EET packages such as EE Fixpack to retain their true BG2EE compatibility
without pretending they can run after EET. EET import can depend on
source-environment work such as DLC Merger while native EET packages execute
in the transformed target. The phase vocabulary is `preprocess`, `bgee`,
`eet-import`, `eet`, `eet-end`, and `post-eet-end`; it is not a substitute for
graph edges or cross-environment inputs.

Capability conflicts are evaluated for selected components, rather than treating
entire packages as mutually exclusive. For example, independent Tactics Remix
and SCS content could coexist even when two selected AI components both provide
the exclusive `mage-ai` capability.

Aliases are exact identity normalization. Fork, predecessor, and continuation
relationships are informational lineage, not automatic package replacement.
See the [living project context](../LIVING_CONTEXT.md) for the project-wide
growth policy and current A5 boundary.
