# Architecture

IEPM models an installation as a resolved sequence of state transformations.
Order can change meaning, so the resolver must make phase and order explicit.

```text
manifest → registry → resolver → lockfile → verified artifacts → WeiDU execution
```

The registry is static YAML in Git: versioned, forkable, and independent of a
server. A client resolves against a named registry revision; old lockfiles stay
reproducible even if future releases become unverified.

EET is a platform transform rather than an ordinary content package. The
initial phase vocabulary is `preprocess`, `bgee`, `eet-import`, `eet`,
`eet-end`, and `post-eet-end`.

Capability conflicts are evaluated for selected components, rather than treating
entire packages as mutually exclusive. For example, independent Tactics Remix
and SCS content could coexist even when two selected AI components both provide
the exclusive `mage-ai` capability.
