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
language/locale/unattended/component argument order. Product A's A7 CLI now
includes a one-command `build` workflow over the same guarded stages. It
resolves and verifies before mutation, derives a missing source fingerprint in
memory, reuses immutable snapshots by exact fingerprint, creates fresh
full-copy workspaces, installs, and seals the output with its effective
manifest, lockfile, plan, and receipts. The initial thin Tauri 2/SvelteKit
desktop shell lives in the same monorepo and calls these Rust operations
through commands and structured progress events; it does not duplicate
package-management decisions in TypeScript. The initial registry bootstrap now
covers every Forge-reference package identity plus a small popular EE ecosystem
layer. Exact artifact and TP2 facts remain claim-level derived evidence;
release compatibility and execution remain unverified until they have stronger
evidence. Fresh full-copy BG2EE 2.6.6 executions now cover a four-mod popular
starter, an Ascension/Call/Throne content fixture, EEex bootstrap/main/LuaJIT,
and separate EEex-dependent Bubb's Spell Menu and Infinity UI++ fixtures.
Their exact selected components carry bounded execution claims; untested
components and broader interactions remain explicit. The Forge-derived
broad-order fixture initially exposed missing launcher, selector,
materialization, and unsupported-EXE blockers as `analysis-only`. After those
exact routes were supplied, one 14-action EET selection completed and sealed
in a fresh managed workspace. Under the public status policy it remains
Untested until component-level launch evidence is complete, and the exact
configuration cannot become Verified until main-menu and gameplay smokes are
recorded. The next work is those smokes plus broader registry/component
coverage, while keeping scope deliberately incremental. Product B's launcher
and profile work is now the next major product phase, but it remains gated on
this Product A supportability/verification work rather than competing with it.
A6 now reads a selected local TP2 as inert
text and reviews its `VERSION`, language declaration order, and
`BEGIN`/`DESIGNATED`/`LABEL` selectors against one curated release. It emits
review JSON but never executes TP2 code, discovers behavior, or rewrites YAML.
The official SCS v35.21 Windows release now has a bounded personal-use route:
IEPM reads its SHA-pinned WinRAR SFX payload through a local `UnRAR.exe`
helper, never invokes the SFX stub, and completed a sealed fresh BG2EE 2.6.6
fixture for dispatcher 100 plus components 2000, 5900, 6030, and 6040. The
dispatcher is explicitly non-recording; its selected nested WeiDU components
remain the required receipt evidence. Other SCS components and broad EET
interactions remain unverified.
Two isolated fresh-copy EET fixture runs additionally established a narrow
negative result: selectors 3501 (BG2 spell scrolls in BG1 stores) and 3551
(maximum 3E Cure/Cause Wounds) both exit successfully but are skipped by SCS
v35.21 as failing requirements, leaving neither a WeiDU receipt nor game
resource changes. The registry blocks those selectors only for EET; it does
not infer their status on other targets.
One exact Forge-derived 14-action EET selection subsequently completed and
sealed, including SCS 6510 (Improved Fiends and Celestials), 6840, and 6850
(Ascension demons) after the registry resolved their same-package component
requirements. This strengthens evidence for those exact selections only; it
does not verify SCS's remaining catalog or arbitrary stack combinations.
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

Product A closeout now has a frozen process in
`docs/product-a-closeout-plan.md`. Broad ecosystem discovery and executable
support are separate layers. PA-1 imported the pinned Infinity Mod Forge
commit `046414eec9315eb1500f2e68147aa281ab35873d` into a generated,
non-executable 813-record catalog with no rejected records or candidate-ID
collisions. It preserves source display/filter observations and candidate
identity mappings without promoting them into resolver facts. PA-2 checked all
813 catalog entries through rate-limited landing-page and candidate-archive
probes; its report distinguishes reachability/signatures from exact,
downloaded/hash-verified artifacts. PA-3 froze a 99-package initial cohort at
95.18% of its dated, stack-normalized initial corpus. This is a PA-4 work
queue—not a 95%-of-all-users claim—and 80 cohort entries remain discovery-only
candidates until release work adds executable facts. Opaque/user-supplied
WeiDU packages remain experimentally installable where technical preflight
succeeds.
PA-4 now has a non-mutating component-inventory audit: `review-tp2` exposes
every raw TP2 declaration not yet represented by a stable, curated component
ID. The SCS v35.21 local-tree audit found 146 declarations; 78 now have stable
IDs, including every selector in the personal EET build, while 68 remain
explicitly unmapped and Untested. Hidden Gameplay Options v5.1 is the first
complete 43-component catalog: its clear author labels supported structural
mapping of all selectors, but only components 0 and 10 have separate install
receipts. The local SCS launcher hash differs from the pinned release artifact,
so its audit remains structural evidence only rather than a release-verification
claim.
The Supported coverage cohort is not an arbitrary “top 100.” It is the
smallest cohort of at least 50 packages that covers at least 95% of weighted,
deduplicated package selections in a dated public corpus, plus required
infrastructure and current reference-profile packages. Only that cohort must
complete Product A's release/component support work; the full catalog is not a
claim that all indexed mods are verified.

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
| Semantic model | What do components actually do? | Future Product C/D analysis |

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

Player-facing support status is a deterministic projection, not a curator's
free-form judgment. The fixed vocabulary is `Verified`, `Supported`,
`Untested`, and `Incompatible`; its quantitative gates and maintainer workflow
are normative in `docs/verification-policy.md`, represented portably by
`schemas/verification-evidence.schema.json`, and evaluated by `iepm-core`.
`Verified` is reserved for one exact locked configuration with a complete
identity, successful clean disposable receipt, sealed output, main-menu smoke,
and new-game or known-save smoke. `Supported` means every selected part and
known rule is supported but the exact combination lacks one or more Verified
run gates. Missing evidence is `Untested`. `Incompatible` requires concrete
mechanical, exclusive-capability, applicable author, or repeated deterministic
failure evidence. These labels cover install/startup confidence, not complete
gameplay correctness.

### Relationships and capabilities

The initial relationship vocabulary is intentionally small:

```text
requires, optional, recommends, conflicts, before, after
```

Relationships can be component-scoped and game-conditioned. A hard requirement
enters the dependency graph; ordering is not a fake dependency; optional and
recommendation data do not become implicit requirements.

A component may also declare package-local `requires` or `conflicts` by stable
IEPM component ID. Prerequisites extend the same package's single WeiDU action;
documented sibling conflicts reject only that incompatible selection pair, not
the whole package. Neither creates a self-edge in the package graph. The first
concrete conflict is SCS's focused spell-tweak selectors versus its all-spell-
tweaks dispatcher; focused selectors may still coexist. Cross-package
requirements remain release relationships, where package identity and
environment binding are explicit.

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

The first non-ZIP preparation exception is `windows-rar-sfx`: an explicitly
typed WinRAR self-extracting payload may be read by local `UnRAR.exe` into the
artifact cache. IEPM does not execute the SFX stub or thereby claim an OS-level
sandbox; unclassified executable artifacts remain analysis-only.

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

The settled sequence is:

```text
Product A — Package Manager / Reproducible Build System
Product B — Modded Game Launcher / Profile Platform
Product C — Semantic Analyzer
Product D — Semantic Merge / Compiler
```

Product A must pass the explicit completion gate in `ROADMAP.md` before
Product B becomes the implementation focus. The existing desktop shell is an
A7 usability surface and architectural foundation, not evidence that the gate
has passed.

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
  automatically accepted, audited warning receipts, and final output receipts.
  A run-state marker makes
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
  Bootstrap extension: `inspect-package` safely reads an extracted directory
  or ZIP-family archive and reports content identity plus inert TP2 structure;
  `derive-bgmod` creates an author-reviewable schema-2 candidate only when
  callers explicitly provide nonmechanical game/phase inputs. Literal
  predicates are evidence, not auto-created resolver relationships.
- **A7:** user-facing CLI ergonomics are complete for the first Product A
  workflow: curated `search`, preview-first `add`, non-mutating lockfile
  `verify`, `install` aliasing the guarded A5 executor, full-copy
  `snapshot`/`workspace`/`seal`, and one-command `build`. It intentionally does
  not add automatic checkpoint reuse, a virtual filesystem, hardlinks, or
  implicit registry selection.
- **Desktop base:** `apps/desktop` is a Tauri 2, SvelteKit, TypeScript,
  Tailwind, and Bun application. SvelteKit is configured as a static SPA with
  SSR disabled. Its thin Tauri commands call the Rust build/preflight APIs and
  forward structured progress events. Future UI work must preserve the CLI
  gates: `analysis-only` locks are non-installable, explicit disposable
  confirmation remains required, and unavailable exact packages are shown as
  coverage gaps rather than replaced. Artifact integrity, mechanical metadata,
  compatibility verification, and upstream integration remain separate status
  dimensions; author participation is never required for IEPM verification.
  The guided desktop route owns only user-facing local setup: a persisted IEPM
  library location, ordinary Steam discovery plus explicit Browse choices for
  clean BG:EE/BG2:EE sources, and an optional friendly mod-experience name.
  It ships the static registry as an application resource; users do not choose
  a registry path. The normal Windows toolchain is the exact WeiDU v251 release
  archive with GitHub-published SHA-256, prepared under the library and passed
  to the existing Rust build API. A local v251 executable override exists only
  as an advanced recovery path when Windows Security blocks acquisition; it is
  version-checked but must never be represented as archive-hash verified.
- **Product B:** external launcher and profile platform over Product A. Its
  core concepts are Profile, Build, PendingChanges, LaunchRecipe, and
  SaveAssociation. Rust owns them; Svelte/Tauri presents them. Mod checkboxes
  edit desired manifest state and require Apply Changes; they are not runtime
  plugin toggles. Failed rebuilds preserve the last known-good sealed build.
  Save associations and mismatch warnings remain external, non-blocking, and
  conservative. Prefix checkpoints are an optional optimization, not a first
  milestone requirement. See `docs/product-b-launcher.md`.
- **Product C:** semantic analyzer for structured resource/component effects,
  overlap, explainable compatibility, and richer save-risk classification. It
  enhances Product B but does not block its initial launcher experience.
- **Product D:** optional semantic merge/compiler layer for safe composition,
  semantic ordering, generators, and conflict synthesis. Do not build it into
  Product A or B.

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
