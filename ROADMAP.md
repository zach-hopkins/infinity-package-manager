# Roadmap

IEPM now has four product phases:

```text
Product A — Package Manager / Reproducible Build System
Product B — Modded Game Launcher / Profile Platform
Product C — Semantic Analyzer
Product D — Semantic Merge / Compiler
```

The current focus remains Product A. The desktop shell exists so the Product A
workflow can be exercised by ordinary users; its existence does not mean the
Product B transition gate has passed.

## Product A — Package Manager / Reproducible Build System

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
- [x] A5 (bundled launcher): Hidden Gameplay Options v5.1 component 10 installed on a fresh disposable BG2EE copy
- [x] A6: inert TP2 structural ingestion aid and release-drift review
- [x] A7: search, preview-first add, lockfile verify, and managed full-copy workspace CLI
- [x] A7 desktop foundation: thin Tauri/Svelte shell over the Rust build APIs
- [x] Evidence policy: measurable Verified/Supported/Untested/Incompatible gates

### Product A completion gate — current priority

Product B implementation begins only after these measurable gates pass:

The ordered implementation milestones, discovery/support layer definitions,
95% cohort metric, and confirmation artifacts are normative in the
[Product A closeout plan](docs/product-a-closeout-plan.md).

- [ ] import the pinned ecosystem discovery catalog, account for every source record, and classify landing-page and exact-artifact health separately;
- [ ] freeze and name the smallest Product A coverage cohort of at least 50 packages that reaches at least 95% of the documented selection corpus;
- [ ] give every cohort release an exact artifact route and complete component catalog, with every exposed component explicitly Supported, Untested, or Incompatible for each intended target;
- [ ] ensure every component advertised as Supported satisfies the quantitative evidence policy—nothing becomes green merely because it is present in the registry;
- [ ] build one BGEE, one BG2EE, and one EET reference profile twice from clean snapshots, producing the same portable lock identity and complete action receipts on both runs;
- [ ] resolve and build the exact Forge/reference profile with all selected components recorded, all warnings classified, EET_End completed, and no skipped requests;
- [ ] record main-menu plus new-game/known-save smokes for each reference profile, making the exact evidence records Verified;
- [ ] inject a failed replacement build and demonstrate that the previous sealed build remains playable while a fresh retry can succeed;
- [ ] complete one clean-machine-style build through the ordinary CLI and desktop Product A paths without repository-only inputs.

The 14-action Forge-derived EET build is important installation evidence, but
the gate is not yet complete: the coverage cohort is not frozen, component
catalog/support coverage remains incomplete, repeat builds are not yet recorded,
and policy launch smokes remain.

Success condition:

> IEPM can reliably construct the desired modded game.

## Product B — Modded Game Launcher / Profile Platform

Product B begins after the Product A completion gate. It builds on manifests,
lockfiles, sealed builds, and evidence rather than replacing them.

- [ ] B0: formalize GUI-independent Rust `Profile`, `Build`, `LaunchRecipe`, `PendingChanges`, and `SaveAssociation` records
- [ ] B1: expose shared structured profile/build state and events through CLI and thin Tauri commands
- [ ] B2: implement the Play / Mods / Settings desktop information architecture
- [ ] B3: edit desired manifests, review pending changes, and apply them explicitly
- [ ] B4: preserve the last known-good sealed build across failed or pending rebuilds
- [ ] B5: resolve launch recipes and provide one-click Play for the selected build
- [ ] B6: index saves externally, associate them with builds/profiles, and compare lockfile identities
- [ ] B7: provide dismissible, non-blocking save mismatch warnings and conservative observed outcomes
- [ ] B8: dogfood the complete experience with the real Forge EET profile

Checkpoint/prefix reuse may optimize rebuilds later, but is not a B0/B1
requirement. The user model remains “change mods → Apply Changes” whether the
backend performs a full rebuild, checkpoint clone, or future copy-on-write
clone.

Success condition:

> IEPM becomes the normal way a user manages and launches modded Baldur's Gate.

See [the Product B architecture](docs/product-b-launcher.md).

## Product C — Semantic Analyzer

- [ ] structured resource and component-effect diffs
- [ ] semantic selectors and transformations
- [ ] resource overlap and semantic capability discovery
- [ ] explainable compatibility and save-risk classification

Product C enriches Product B; it is not required for profiles, launching,
lockfile-based save differences, or non-blocking mismatch warnings.

Success condition:

> IEPM increasingly understands what mods do, not merely how to execute them.

## Product D — Semantic Merge / Compiler

- [ ] safe semantic composition and explicit semantic ordering
- [ ] mergeable and order-sensitive transformations
- [ ] semantic generators and conflict synthesis
- [ ] reduction of unnecessary historical install-order constraints

Success condition:

> IEPM can compose large portions of the ecosystem and explain the remaining
> order and conflict boundaries.
