# Roadmap

## Product A — modern package manager

- [x] A0: schema-2 manifest/package contracts, schema-3 replay lockfile, stable IDs/aliases, environment graph vocabulary, and claim provenance
- [x] A1: static registry reader and package query surface
- [x] A2 (initial): deterministic graph validation and ordering
- [x] A2: opaque release IDs, optional SemVer ranges, candidate search, and release-specific symbolic component selection
- [x] A3: environment-aware lockfile, installer inputs, executable component mappings, and execution graph
- [x] A4: content-addressed artifact variants/mirrors, hardened cache, extraction, and SHA-256 verification
- [x] A5 (first slice): schema-3 executable preflight and non-mutating `iepm plan`
- [x] A5 (evidence slice): executable minimal EET lifecycle fixture with source history and shared-WeiDU routes
- [x] A5 (minimal execution): verified-artifact materialization, explicit disposable bindings, sequential action receipts, and a full five-action EET run
- [x] A5 (execution integrity): pre-mutation core-layout fingerprints and final output-fingerprint receipts for the minimal EET route
- [x] A5 (recovery boundary): interrupted runs retain a non-resumable state marker; a fresh workspace is required
- [ ] A5: further real-package execution counterexamples
- [ ] A6: TP2-assisted registry ingestion and release-drift review
- [ ] A7: user-facing search/add/verify CLI polish

Product B (semantic analysis) and Product C (semantic merge/compiler) remain
future enhancements. They must not block Product A.
