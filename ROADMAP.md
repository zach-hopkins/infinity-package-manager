# Roadmap

## Product A — modern package manager

- [x] A0: schema-2 manifest/package contracts, schema-3 replay lockfile, stable IDs/aliases, environment graph vocabulary, and claim provenance
- [x] A1: static registry reader and package query surface
- [x] A2 (initial): deterministic graph validation and ordering
- [x] A2: opaque release IDs, optional SemVer ranges, candidate search, and release-specific symbolic component selection
- [x] A3: environment-aware lockfile, installer inputs, executable component mappings, and execution graph
- [x] A4: content-addressed artifact variants/mirrors, hardened cache, extraction, and SHA-256 verification
- [x] A5 (first slice): schema-3 executable preflight and non-mutating `iepm plan`
- [x] A5 (evidence slice): real EET source-history/baseline and shared-WeiDU command-route fixture
- [ ] A5: validate plan against a trusted EET fixture, then ordered WeiDU execution in disposable environment workspaces
- [ ] A6: TP2-assisted registry ingestion and release-drift review
- [ ] A7: user-facing search/add/verify CLI polish

Product B (semantic analysis) and Product C (semantic merge/compiler) remain
future enhancements. They must not block Product A.
