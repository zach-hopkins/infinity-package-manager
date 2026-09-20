# Roadmap

## Product A — modern package manager

- [x] A0: schema-2 contracts, stable IDs/aliases, environment graph vocabulary, and claim provenance
- [x] A1: static registry reader and package query surface
- [x] A2 (initial): deterministic graph validation and ordering
- [x] A2: opaque release IDs, optional SemVer ranges, candidate search, and release-specific symbolic component selection
- [x] A3: environment-aware lockfile, installer inputs, executable component mappings, and execution graph
- [x] A4: content-addressed artifact variants/mirrors, hardened cache, extraction, and SHA-256 verification
- [ ] A5: dry-run then ordered WeiDU execution in disposable environment workspaces
- [ ] A6: TP2-assisted registry ingestion and release-drift review
- [ ] A7: user-facing search/add/verify CLI polish

Product B (semantic analysis) and Product C (semantic merge/compiler) remain
future enhancements. They must not block Product A.
