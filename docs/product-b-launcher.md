# Product B: launcher and profile platform

Product B is the consumer layer above Product A's reproducible build system.
Its user-facing model is intentionally small:

```text
PLAY    MODS    SETTINGS
```

The normal loop is:

```text
choose mods → Apply Changes → build becomes Ready → PLAY
```

This page defines the architectural boundary now. Product B implementation
does not begin until the Product A completion gate in [`ROADMAP.md`](../ROADMAP.md)
has passed.

## Non-negotiable model

Infinity Engine/WeiDU mods are build-time transformations, not runtime
plugins. A checked mod means desired manifest intent. It does not hot-load a
package into one mutable game directory.

```text
Mods UI
  ↓
desired manifest
  ↓
resolver and lockfile
  ↓
fresh build or reusable-prefix clone
  ↓
sealed ready build
```

Product B preserves Product A's lifecycle:

```text
clean installation
  → immutable source snapshot
  → disposable workspace
  → WeiDU/EET execution
  → sealed successful build
```

Source snapshots are never mutation targets. A failed replacement build never
destroys the last known-good sealed build.

## Core concepts

These will become concrete Rust records after the Product A gate. They are
domain concepts, not a generic framework.

### Profile

A durable user-facing mod configuration identity. It owns desired manifest
intent, the current resolved/built state, selected launch behavior, and save
associations. It is not merely a directory label.

### Build

An exact resolved execution and its outcome. A successful build references its
portable lockfile, receipts, sealed environments, verification assessment, and
launch recipe. Failed attempts are diagnostic history and never replace the
profile's last known-good build pointer.

### PendingChanges

A structured comparison between the profile's desired manifest and its current
built lockfile. Mod checkbox/component edits update desired state; the user
explicitly chooses **Apply Changes** before an expensive rebuild starts.

### LaunchRecipe

The Rust-owned, build-specific description of the executable, literal
arguments, and working-directory binding needed to start the game. Local paths
remain machine configuration and are not written into portable lockfiles.
Stacks requiring `InfinityLoader.exe` and ordinary `Baldur.exe` must be
distinguishable without asking the player to remember the correct executable.

### SaveAssociation

A local, external association between a stable save observation and the
profile/build under which IEPM created or first observed it. Legacy saves remain
valid and appear as unknown. The exact storage/fingerprinting mechanism requires
implementation research and is not settled by this architecture note.

## State separation

Product B must keep these states distinct:

```text
desired configuration
current successful build
active/selected build
in-progress or failed build attempt
```

Changing desired intent never silently changes the active build. A profile can
therefore be `ready`, `changes-pending`, `building`, or `build-failed` while
still retaining a playable previous build where one exists.

The frontend receives structured Rust state/events, not only log text. Initial
events should cover profile state changes, build start/completion/failure, and
package action progress. Human-readable logs remain diagnostic artifacts.

## Save mismatch behavior

The initial comparison is structural and lockfile-based:

- packages/components added;
- packages/components removed;
- releases changed;
- exact build match or different build;
- a known-risk distinction only when registry/evidence facts justify it.

Warnings are non-blocking, dismissible, and remembered per save/build mismatch.
A changed build identity may warn again. A successful observed load means only
“loaded successfully” or “no immediate failure observed”; it never proves
long-term save compatibility.

Product C may later attach semantic impact to the same difference records.
Product B contracts must therefore permit optional future classifications
without requiring semantic analysis today.

## Ownership boundary

```text
SvelteKit presentation
  ↓
thin Tauri commands and structured events
  ↓
Rust profile / build / launcher APIs
  ↓
registry / resolver / artifacts / workspace / WeiDU
```

Rust remains authoritative for profiles, manifests, resolution, lockfiles,
pending changes, build state, launch recipes, save/build comparisons, and
verification status. TypeScript renders state and collects user interaction; it
does not reimplement package-management decisions.

The same core operations remain CLI-callable. Exact command names may evolve,
but anticipated surfaces include profile listing/selection, build application,
launching, and save/build comparison.

## Support presentation

Ordinary views use the fixed `Verified`, `Supported`, `Untested`, and
`Incompatible` vocabulary plus a few plain-language facts such as artifact
verified, components recognized, and target installation tested. Users should
not need to interpret claim provenance to decide whether to proceed.

A details view may expose mechanically derived, community-curated,
author-declared, and automated evidence separately. Author-shipped metadata is
valuable for maintenance but is not a prerequisite for Supported or Verified
status. Product B is therefore not blocked on an author-adoption portal,
hosted registry service, or widespread `bgmod.yaml` adoption.

## Initial desktop experience

- **Play:** selected profile, ready/pending/failed state, last save, build-match
  status, Play, Apply Changes, or Play Previous Build as appropriate.
- **Mods:** searchable registry, package/component selection, dependencies,
  conflicts, plain-language support status, and explicit Apply Changes.
- **Settings:** clean game installations, IEPM library/build storage, artifact
  cache, registry updates, launcher behavior, and advanced recovery options.

Profiles may initially use a selector within Play; they do not require a fourth
top-level tab.

## Deferred optimization

Prefix checkpoints are compatible with this model but not required for the
first launcher milestone. A valid checkpoint key must cover source snapshots,
resolved prefix/order, artifact hashes, installer inputs, and toolchain facts.
Full fresh copies remain the correctness baseline.

## Explicit non-goals

- no in-game IEPM menu or modification of Baldur's Gate save/load UI;
- no mandatory runtime companion mod or EEex dependency;
- no hot-loading arbitrary WeiDU mods;
- no instant-toggle promise when a rebuild is required;
- no semantic analyzer/compiler work in Product B;
- no automatic claim that a mismatched save is safe;
- no destruction of the last known-good build;
- no resolver, registry, or installer logic duplicated in TypeScript;
- no requirement for widespread mod-author adoption before Product B ships.
