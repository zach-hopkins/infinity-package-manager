# Infinity Package Manager — Living Context

> This is IEPM's authoritative, updateable project context.
>
> Read it before changing architecture, schemas, resolver behavior, registry
> semantics, artifact handling, or the A5 execution boundary. Continue from
> settled decisions here; revisit them only when implementation evidence, a
> real mod/package counterexample, or a failed integration fixture materially
> disproves them.

## Status and immediate priority

IEPM is a Rust package-management control plane above WeiDU. Product A's A0–A4
foundation has completed a hostile pre-A5 review and subsequent remediation.
The project is **green for the minimal A5 EET route and yellow for broader
A5**. `iepm execute` has completed the evidence-backed five-action route in
fresh disposable Steam copies: DLC Merger, source/target EE Fixpack, EET
import, and EET_End. It verified/extracted pinned artifacts, bound named local
workspaces without storing their paths in the lockfile, measured an explicit
core-layout input fingerprint before mutation, retained action receipts and a
completed output-fingerprint receipt, and required explicit
disposable-workspace confirmation. An interrupted or prior run-state marker
is deliberately non-resumable: rebuild a fresh controlled workspace rather
than attempting rollback. A controlled interruption during EET import proved
that the marker rejects an in-place retry before fingerprinting or mutation.
The bundled-launcher counterexample, Hidden Gameplay Options v5.1, has now
completed both a narrow component-10 execution and a fresh-download
component-0 (`install-all`) execution through the managed full-copy snapshot →
workspace → sealed-build route. The latter began from a clean BG2EE 2.6.6
source snapshot and produced a distinct sealed output fingerprint. It proved
that bundled and shared WeiDU both require a declared game locale and the same
language/locale/unattended/component argument order. The Product A's initial
A7 usability surface is now complete. A Forge-derived broad-order fixture now
also resolves as intentionally `analysis-only`: the resolver exposes exact
missing release, installer, and selector evidence rather than silently
substituting package identities or mutating a workspace. The next work is
broader registry and A5 route coverage, while keeping scope deliberately
incremental. A6 now reads a selected local TP2 as inert
text and reviews its `VERSION`, language declaration order, and
`BEGIN`/`DESIGNATED`/`LABEL` selectors against one curated release. It emits
review JSON but never executes TP2 code, discovers behavior, or rewrites YAML.
The retained official Hidden Gameplay Options v5.1 source is the first
evidence point: its opaque release ID, display version, English language index
0, and selected component selectors match the registry. This corroborates but
does not replace its A5 disposable execution evidence.

The source-tagged EET v14.1 archive's TP2 declares itself v14.0, and EET core
emits non-stopping warnings under shared WeiDU 251. This is resolved evidence,
not a version-model failure: source tag, archive hash, TP2 display version,
and executor identity remain separate facts. An explicit warning policy may
continue only exit code 3 with both `INSTALLED WITH WARNINGS` output and a
matching `WeiDU.log` component record. EET_End also invokes an embedded WeiDU
24900; the shared toolchain pin identifies IEPM's top-level process, not every
subprocess a package may choose to launch.

Manifest/package records use schema 2 and replayable lockfiles use schema 3.
The resolver still reads legacy schema-1 manifests and registry records as a
migration convenience. Registry coverage is deliberately incomplete and largely
unverified; that must remain explicit rather than silently blocking analysis or
claiming installability.

## Core thesis

> Infinity Engine modding does not primarily have an installation problem. It
> has a package-description problem.

WeiDU is the first-class execution backend. It patches a stateful game tree;
the final game is the output of an ordered build, not merely a set of files.
IEPM supplies the missing package/constraint/reproducibility layer above it.

The project keeps four questions distinct:

| Layer | Question | Authority |
| --- | --- | --- |
| Manifest | What does the user want? | Human intent |
| Registry | What do we know about available releases? | Curated ecosystem facts |
| Lockfile | What exact build did resolution choose? | Canonical resolved build |
| Semantic model | What do components actually do? | Future Product B/C analysis |

Product A must not depend on semantic analysis. Future semantic knowledge may
improve conflicts and merging, but it must not replace manifest, registry, or
lockfile responsibilities.

## Settled architecture

### Environments and EET

EET is a multi-environment build graph, not a normal package in one linear
game target. A realistic build has at least a BGEE/SoD source environment and
an EET target environment:

```text
bgee-source::dlc-merger
             ↓
eet-target::eet
             ↓
eet-target::native-mods → eet-target::eet-end → post-end exceptions
```

Execution nodes identify one package in one environment. Graph edges express
real ordering, including cross-environment dependencies. Phases are hard
barriers only within an environment; they are not a global substitute for a
build graph.

EET's target workspace changes identity in place. Its initial target is
`bg2ee`; an environment that declares `after_eet_import: eet` runs packages
through the `eet-import` phase against BG2EE, then runs later phases against
EET. This is required by observed EE Fixpack behavior: it validly runs before
EET on BG2EE and explicitly rejects an already-EET game. A declared result
without an `eet-import` node is an invalid plan.

### Identity

- A canonical package ID is a stable IEPM identity.
- An alias represents an exact historical rename and may be canonicalized early
  in resolution.
- Lineage describes a predecessor, fork, or continuation. It is informational:
  never silently install a fork/continuation in place of a requested package.
- `release_id` is opaque identity; `version` is human display text; `semver` is
  optional solver metadata. None is assumed to equal a Git tag, TP2 version,
  archive hash, or repository state.
- A stable IEPM component ID expresses user intent. Its TP2 path, `LABEL`,
  numeric fallback, and subcomponent mapping are release-specific WeiDU
  implementation metadata.

Do not force irregular Infinity Engine release text into SemVer. Plain version
requests are exact opaque matches. Only explicit range syntax opts into SemVer
solving.

### Registry knowledge and evidence

The static YAML registry is versioned, forkable, and must not depend on a
central service. It may record package/release identity, artifact locations,
hashes, component selectors, relationships, capabilities, materialization,
and evidence.

Evidence strength matters. Mechanically derived TP2 facts, author claims,
community curation, and an exact automated installation verification are
different claims. Track provenance where that distinction changes user trust;
do not attach provenance machinery to trivial display-only fields.

Unknown compatibility means **unverified**, not incompatible. Safe structural
facts may be re-derived per release. Do not silently inherit behavioral
compatibility, ordering, artifact choice, prompt answers, or verified status.

### Relationships and capabilities

The initial relationship vocabulary is intentionally small:

```text
requires, optional, recommends, conflicts, before, after
```

Relationships can be component-scoped and game-conditioned. A hard requirement
enters the dependency graph; ordering is not a fake dependency; optional and
recommendation data do not become implicit requirements.

Capabilities express narrow semantic overlap, such as mutually exclusive mage
AI components. They are registry-curated and must not become a giant ontology
or a policy DSL. Add a new condition, capability convention, or expression
only when a documented popular-mod case cannot be represented otherwise.

### Artifacts, materialization, and trust

SHA-256 identifies artifact content. A URL is merely an acquisition location;
mirrors may serve the same content without changing its identity. Platform and
architecture selection must be explicit.

Acquisition and materialization are separate:

```text
verified archive → verified extracted tree → materialization recipe → A5 workspace mutation
```

A4 owns secure acquisition and preparation. A5 owns copying/materializing into
a controlled game workspace and executing WeiDU. The extracted-tree digest,
safe ZIP path handling, and case/symlink checks are integrity safeguards—not a
reason to build a general content-addressed filesystem.

### Portable reproducibility and execution readiness

A lockfile captures canonical identities, selected artifact content,
materialization metadata, component selectors, installer program and
release-specific language-index mapping, portable installer inputs, toolchain,
environment fingerprints, registry revision, and execution graph. The current
`iepm-core-layout-v1` profile is a small, documented set of game identity and
layout facts, not a misleading full-tree hash; it is measured before execution
and again in the final receipt.

Environment names are portable; local paths are machine configuration. Never
store absolute local game paths, user-profile paths, or secrets in a manifest
or lockfile. For example, record `source-environment: bgee-source`, then bind
that name to a local path outside the lockfile.

An environment can also declare an ordered WeiDU `baseline`: components
expected to be present before IEPM's requested execution graph begins. A
baseline is a preflight assertion, never an implicit install request. It must
not be confused with EET's copied `WeiDU-BGEE.log`, which is source provenance
preserved in the target rather than a target execution queue. Until A5 has an
explicit verify-existing action, a selected component that duplicates an
environment baseline is a blocking error rather than a reinstall.

Installers may use either a package-bundled launcher or the pinned shared
WeiDU toolchain. Every WeiDU execution requires a named workspace and game
locale (for example `en_US`), separate from the package's numeric WeiDU
language index; both launcher paths render `--language`, `--use-lang`,
unattended flags, and component selection in that order rather than relying
on interactive defaults. Required arguments are typed literals or named
environment bindings, not interpolated shell commands. The distinction is
required by real packages that ship only a TP2, such as the observed EE
Fixpack archive, and by packages that ship a setup executable, such as Hidden
Gameplay Options.

Resolution and execution are separate states:

- `analysis-only`: useful resolved graph, but one or more execution facts are
  missing. It must not mutate a game tree.
- `executable`: artifact, installer program, selected numeric WeiDU component
  mapping, release-specific language-index mapping, portable required inputs,
  and execution plan are all present. A5 may preflight it.

Every `analysis-only` result must carry causal blocking reasons. Do not defer a
missing selector or artifact discovery to the middle of installation.

### Resolver and implementation style

Resolver behavior must be deterministic and globally valid. Candidate
search/backtracking rebuilds reachable dependency closure so rejected candidates
cannot leave stale dependencies behind. Normalize aliases, default components,
and shorthand before deep solving; lockfiles contain canonical results.

The data model is necessarily rich. Keep Rust implementation deliberately
boring: concrete records, requests, resolved packages, and execution nodes are
preferred over generic frameworks, trait pyramids, or speculative builders.

## Product progression

Product A is a useful package manager even if later products never ship:

```text
manifest → registry → resolver → lockfile → verified artifacts
         → materialization → execution plan → WeiDU → reproducible build
```

- **A0–A4:** v2 schemas, registry loading/validation, complete resolution,
  replayable lockfile, and verified artifact preparation.
- **A5:** the minimal EET route has passed manual comparison and a full
  disposable execution with action receipts. `iepm execute` is intentionally
  narrow: verified artifacts, safe materialization, explicit environment
  bindings, pre-mutation input fingerprints, sequential process supervision,
  opt-in audited warnings, and final output receipts. A run-state marker makes
  interrupted workspaces non-resumable; rebuild rather than roll back. A
  deliberate interrupted-run test has exercised that boundary. A second real
  Windows release has completed through the bundled-launcher path. A7 adds an
  IEPM-managed full-copy lifecycle around this execution boundary: source
  snapshots are clone-only, workspace copies are the only accepted mutation
  targets, and completed workspace sets can be sealed as separate builds.
  This is an IEPM-enforced state contract, not a claim to sandbox arbitrary
  external processes or an OS-level immutable ACL. Interrupted and failed
  workspaces remain non-reusable. Prefix checkpoints are deliberately deferred
  because their identity requires the resolved prefix and toolchain facts, not
  merely the core-layout fingerprint.
  `examples/eet-minimal` remains a controlled
  fixture, not authorization to install into a user's primary game tree.
- **A6:** TP2-assisted registry ingestion, release drift review, and
  conservative structural inheritance. The implemented path is deliberately a
  manual ingestion aid: a selected TP2 yields structural observations and a
  drift report for an existing release. It does not auto-create releases or
  inherit compatibility, relationships, artifacts, installer inputs, or
  verification because those are behavioral or release-trust claims.
- **A7:** user-facing CLI ergonomics are complete for the first Product A
  workflow: curated `search`, preview-first `add`, non-mutating lockfile
  `verify`, `install` aliasing the guarded A5 executor, and full-copy
  `snapshot`/`workspace`/`seal`. It intentionally does not add automatic
  checkpoint reuse, a virtual filesystem, hardlinks, or implicit registry
  selection.
- **Next UI slice:** a thin GUI may expose the existing managed lifecycle and
  readiness diagnostics. It must preserve CLI gates: `analysis-only` locks are
  non-installable, explicit disposable confirmation remains required, and
  unavailable exact packages are shown as coverage gaps rather than replaced.
- **Product B:** semantic analyzer; controlled component installs plus useful
  resource-level diffs.
- **Product C:** optional semantic merge/compiler layer. Do not build it into
  Product A.

Unknown legacy packages should degrade guarantees rather than become
uninstallable by policy. WeiDU remains the escape hatch.

## A5 acceptance path

1. `iepm resolve` produces canonical lockfile state and readiness.
2. `iepm plan` renders a non-mutating, human-auditable sequence of
   environment bindings, materialization actions, and intended WeiDU commands.
3. Compare that plan with a manual known-good EET installation.
4. Reject `analysis-only` locks before any mutation.
5. Execute only in disposable, IEPM-controlled build workspaces—not a user's
   only Steam/GOG installation. Require an explicit runtime confirmation and
   retain command/stdout/stderr receipts for each action.
6. Treat cancellation, reboot, and WeiDU failure as normal outcomes; favor
   rebuild-from-known-clean over heroic in-place rollback.
7. Turn every real exception found during A5 into a narrowly scoped fixture
   before generalizing schemas or resolver behavior.

## Scope and maintenance discipline

- Real ecosystem evidence outranks abstract elegance.
- Once a hostile review has settled a foundation, move forward until reality
  disproves it. Do not reopen it because another abstraction looks cleaner.
- Add only distinctions that have a demonstrated use. Keep the difference
  between package facts, execution facts, and future semantic facts clear.
- Do not create a centralized registry service requirement.
- Do not claim IEPM is a sandbox: WeiDU packages are executable install logic.
- Prefer clean source installs → controlled workspace → modded output. Do not
  overengineer storage optimization yet.
- Primary development is Windows; CI covers Windows and Linux. Native Windows
  execution remains important for real end-to-end tests.

## Architecture rethink triggers

Reopen a settled decision only when one of these occurs:

1. A documented popular mod needs obvious hacks under the current model.
2. A real EET install needs behavior the environment graph cannot describe.
3. A known-good build cannot be locked without hidden machine-local state.
4. A common rule requires abusing the wrong relationship/capability primitive.
5. Identity rules conflate genuinely distinct projects or split one identity.
6. A5 fills with pervasive special cases, showing the execution graph is wrong.
7. A6 cannot safely derive/inherit metadata under the current evidence model.
8. Public adoption exposes a migration problem schema versioning cannot absorb.

## Update protocol

This is a living guide, not a raw changelog.

- Update **Status and immediate priority** after material milestone changes.
- Preserve settled decisions and their rationale; mark superseded decisions
  explicitly rather than quietly erasing useful history.
- Classify material statements as settled, provisional, or future when needed.
- Keep low-level implementation detail out unless it changes architectural
  continuity or the public contract.
- Every architecture-changing pull request must update this file or explain why
  its change does not affect project context.
