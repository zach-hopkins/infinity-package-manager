# Product A closeout plan

This is the execution plan for closing the Product A gate. It separates broad
ecosystem discovery from the smaller set for which IEPM makes strong support
promises.

The objective is not to verify every Baldur's Gate mod or every possible
combination. It is to leave Product A in this useful state:

> A player can discover nearly the whole known BGEE/SoD/BG2EE/EET ecosystem,
> attempt an unknown WeiDU package with honest warnings, and rely on measurable
> support for the mods that account for most real selections.

The source assessment behind this plan is
[Infinity Mod Forge ecosystem assessment](research/infinity-mod-forge-assessment.md).
Its operational companion is covered in the
[Infinity Mod Runner acquisition assessment](research/infinity-mod-runner-assessment.md).

## The three coverage layers

These layers must remain distinct in data and user language.

| Layer | Promise | Required evidence |
| --- | --- | --- |
| **Indexed** | IEPM knows the project exists and can help the player find it. | Stable IEPM candidate identity, source provenance, display facts, and a recently successful project/download-page check. |
| **Fetchable / Opaque** | IEPM can obtain or accept exact bytes and mechanically inspect the package. It may be installed experimentally. | Resolved or user-supplied artifact, SHA-256, safe archive inspection, detected TP2/components, and explicit unknowns. |
| **Cohort Supported** | IEPM understands the exact release well enough to make the existing `Supported`/`Verified` evidence decisions. | Executable route, complete component catalog and classifications, known rules, and evidence required by `verification-policy.md`. |

An Indexed record is not automatically a package release in the executable
registry. A reachable landing page is not an artifact. A parsed TP2 is not a
compatibility test. Unknown remains installable where technically possible,
but displays `Untested` or `Opaque` rather than a false green status.

## How the coverage cohort is chosen

“Top 100” is a planning estimate, not a predetermined answer. IEPM will freeze
the smallest cohort that meets all of these conditions:

1. It contains at least 50 packages.
2. It accounts for at least 95% of weighted package-selection occurrences in
   the frozen research corpus.
3. It includes every infrastructure dependency needed by those packages.
4. It includes the existing IEPM/Forge reference-profile packages.
5. It covers BGEE, SoD, BG2EE, and EET and the major observed use cases: fixes,
   quests/content, NPCs, tweaks/QoL, AI/tactics, UI/runtime, and EET
   infrastructure.

The corpus will contain the user's Forge reference list, all Forge presets,
approved community builds, deduplicated public WeiDU logs/mod lists, maintained
guides, available telemetry, and a dated sample of recurring recommendations
from established BG modding communities. Each stack contributes at most one
selection per package, so a 600-component install does not swamp smaller
profiles. Mirrors, reposts, and copied lists are deduplicated.

The primary quantitative measure is:

```text
coverage = weighted package selections belonging to the cohort
           ----------------------------------------------------
                  all weighted package selections
```

Source families are weighted and reported separately so twenty reposts of one
guide do not masquerade as twenty independent users. Popularity signals from
different sites are percentile-ranked within their own source; GitHub stars
are never directly compared with Weaselmods downloads. AI search is allowed to
find and normalize candidates and discussion mentions, but every observation
keeps a URL, date, source type, and deduplication key.

If the 95% threshold requires 73 packages, the cohort has 73. If it requires
126, the cohort has 126. The coverage report, not a round number, is the gate.

## Closeout milestones and confirmations

### PA-1 — Freeze the discovery source and catalog contract

Deliverables:

- a small, versioned discovery-record contract separate from executable
  package/release records;
- an importer for the pinned Infinity Mod Forge snapshot, with attribution and
  the upstream commit recorded;
- deterministic external-ID-to-IEPM-ID mapping plus collision/alias review;
- a generated catalog and a rejection report, never 813 hand-maintained files;
- fields retained for future Product B search/filter display without moving
  resolver logic into the UI.

Confirmation:

- all 813 source records are accounted for as imported or rejected with one
  explicit reason;
- regenerating from the pinned source produces no diff;
- no Forge conflict, order, or compatibility statement is promoted to a
  Supported claim by import alone;
- lineage and aliases remain review-only where identity is uncertain.

### PA-2 — Measure link health and artifact acquisition

Deliverables:

- a polite, cached link checker with rate limits, redirect recording, and GET
  fallback where HEAD is unreliable;
- distinct `catalog_url`, `artifact`, and `acquisition` outcomes;
- automatic resolvers beginning with GitHub release assets, followed by
  documented hosts that expose stable downloadable artifacts;
- a Runner-derived candidate pass over Forge data (currently 516 tagged
  GitHub archive candidates, 113 branch archive candidates, and 184 manual
  routes), with every candidate independently validated by IEPM;
- browser-assisted/manual acquisition for forum, Nexus, or ambiguous pages;
- an artifact-health report with last-check time and failure reason;
- opaque local-archive attachment using the existing safe inspector.

Confirmation:

- every imported record has a fresh catalog URL outcome;
- every automatic artifact route has downloaded exact bytes, recorded
  SHA-256, and passed safe package inspection;
- redirects, auth/manual requirements, ambiguous assets, dead links, and
  unavailable releases are explicit—none are silently counted as fetchable;
- mutable branch candidates are resolved to exact commits and labeled derived
  snapshots, never silently presented as author releases;
- every discovered WeiDU archive can proceed as `Untested/Opaque` when the
  technical executor facts can be derived, without requiring central registry
  membership.

Runner's patch manifest and install exceptions become a provenance-bearing
review queue in this milestone. They are not applied or promoted merely
because Runner contains them; each receives an explicit modeled/tested,
upstream-fixed, performance-only, obsolete, rejected, or unverified
disposition.

### PA-3 — Freeze the evidence-based coverage cohort

Deliverables:

- the dated, deduplicated input corpus and source ledger;
- a reproducible ranking/coverage script;
- a machine-readable cohort file with the exact releases initially targeted;
- a human coverage report showing frequency, source diversity, acquisition
  health, infrastructure role, game targets, categories, and inclusion reason;
- a deferred-candidate list so exclusion is visible rather than forgotten.

Confirmation:

- the cohort has at least 50 packages and reaches at least 95% measured
  selection coverage;
- adding/removing a package recomputes the metric deterministically;
- every required infrastructure package and current reference-profile package
  is included even when raw popularity is low;
- no source-specific raw popularity number is compared across incompatible
  sources.

### PA-4 — Make every cohort release executable and honestly classified

Deliverables for each exact target release:

- official/reputable artifact route, SHA-256, archive layout, materialization,
  installer, language map, and managed WeiDU route;
- complete mechanically derived component catalog with stable IEPM component
  IDs and release-specific selectors;
- explicit `Supported`, `Untested`, or `Incompatible` state for every exposed
  component on each intended target;
- modeled package-local requirements and only the strongest cross-package
  relationships justified by evidence;
- claim-level provenance and a drift report for the next release;
- an updated plain-English mod support ledger.

Confirmation:

- every cohort release resolves without an unexplained `analysis-only`
  blocker for its intended default/core route;
- default/core components meet the existing release support gates;
- no component is missing from the ledger merely because it has not been
  tested;
- broad package conflicts are not substituted for component-level overlap;
- exact release updates do not inherit behavioral claims silently.

This milestone does **not** require testing every optional component or every
pairwise combination. Large componentized mods such as Tweaks Anthology and
SCS must have complete catalogs; untested components may remain explicitly
Untested while evidence is accumulated.

### PA-5 — Complete reference profiles and regression evidence

Deliverables:

- one representative BGEE profile, one BG2EE profile, and one EET profile;
- the exact existing Forge/reference profile;
- two clean builds of each reference profile from immutable snapshots;
- portable lockfiles, action/warning receipts, sealed outputs, and final
  fingerprints;
- main-menu and new-game or designated known-save smoke records;
- a component/relationship evidence extraction pass after each successful
  run.

Confirmation:

- each pair resolves to the same portable lock identity and completes every
  requested action with no silent skip;
- warning exits are accepted only under the audited warning policy;
- EET_End completes where required;
- each exact reference configuration evaluates to `Verified` under the
  normative status policy;
- evidence is scoped to selected components and known interaction edges, not
  generalized to arbitrary combinations.

### PA-6 — Prove recovery and stranger-ready distribution

Deliverables:

- a deliberately failed replacement build while a prior sealed build exists;
- a successful retry in a fresh workspace;
- a release-like bundle with the registry/catalog, managed WeiDU acquisition,
  and no repository-only file assumptions;
- one clean-machine-style CLI build and one desktop Product A build using the
  same Rust core decisions;
- a short first-use guide and retained diagnostic bundle.

Confirmation:

- failure never mutates or removes the last known-good sealed build;
- the failed workspace cannot be resumed as though it were clean;
- a stranger can choose storage/game paths, import or select mods, review the
  plan, build, and find warnings/logs without a development checkout;
- CLI and desktop produce the same resolved identity for the same inputs.

### PA-7 — Product A gate review

Deliverables:

- a single gate evidence index linking every confirmation above;
- final catalog, acquisition, cohort, support, build, smoke, recovery, and
  clean-machine reports;
- a hostile review of unsupported claims and unresolved blockers;
- the Product A completion decision recorded in `ROADMAP.md` and
  `LIVING_CONTEXT.md`.

Confirmation:

- every Product A roadmap checkbox has direct evidence;
- all remaining gaps are explicitly Product B usability work, future ecosystem
  expansion, or Product C semantic analysis—not a missing Package Manager
  capability;
- the success statement is demonstrated: **IEPM can reliably construct the
  desired modded game.**

## Execution order

The work proceeds in this order:

```text
pinned catalog source
  -> indexed discovery records and link outcomes
  -> exact artifact resolution / opaque fallback
  -> measured 95% cohort
  -> cohort executable metadata and classifications
  -> reference builds and smokes
  -> recovery + clean distribution
  -> Product A gate review
```

PA-1 through PA-3 establish what “coverage” means. PA-4 is the largest body of
registry/evidence work. PA-5 and PA-6 prove the product rather than merely
proving its data structures.

## Explicit non-goals for this gate

- verifying all 813 catalog entries;
- testing every ordering or component permutation;
- treating old manager databases, forum claims, or Forge rules as truth;
- importing another tool's private patch set as silent package behavior;
- blocking an otherwise executable unknown package because it is outside the
  supported cohort;
- building Product B profiles/launching or Product C semantic analysis before
  the Product A evidence bundle passes.
