# Infinity Mod Runner acquisition assessment

Infinity Mod Runner is the operational companion to Infinity Mod Forge. Forge
describes and exports the desired WeiDU list; Runner enriches that list from
Forge data, checks local mod folders, proposes downloads, applies optional
patches, and drives WeiDU.

This makes Runner a high-value acquisition and installation research source.
It does not make Runner's derived URL, patch, or successful mega-install an
authoritative IEPM package fact.

## Snapshot inspected

- Repository: [Anprionsa/infinity-mod-runner](https://github.com/Anprionsa/infinity-mod-runner)
- Commit: `da361b4aa57e4c2afb10831b347064f12ad6cd45`
- Commit date: 2026-04-20
- License: MIT
- Inspection date: 2026-09-21

## How Runner combines with Forge

Runner fetches Forge's generated mod index, per-mod detail records, category
order, presets, known issues, and version cache. A Forge-exported WeiDU log is
the requested install list. Runner matches TP2 names to Forge records, loads
component details on demand, checks local TP2 presence, and uses Forge release
observations to propose acquisition routes.

Its download resolver uses this order:

1. an explicit Forge `dl` override;
2. an explicit Forge source-mode override;
3. a GitHub URL plus cached release tag;
4. a GitHub default-branch archive, with a `main` to `master` fallback;
5. a manual browser link for Weaselmods and other hosts.

Applied to the pinned 813-record Forge snapshot used by IEPM's source
assessment, the current Runner algorithm proposes:

| Proposed route | Records |
| --- | ---: |
| GitHub tagged source archive | 516 |
| GitHub branch source archive | 113 |
| Manual/browser acquisition | 184 |
| Missing homepage URL | 0 |

This is a substantial automatic-acquisition candidate set. It does not add
independent mod identities beyond Forge's catalog.

## What IEPM should adopt

The following ideas fit Product A:

- treat Forge and Runner as a paired discovery/acquisition input;
- use cached repository/tag observations to generate artifact candidates;
- preserve manual browser-assisted acquisition for non-automatable hosts;
- recursively locate TP2 files after safe extraction rather than assuming one
  archive layout;
- detect local package presence with one indexed scan;
- retain per-mod timeouts, batching exceptions, READLN inputs, sibling-folder
  requirements, and known patches as an operational review queue;
- compare requested components with the resulting WeiDU log and never equate
  process exit with complete installation.

Runner's bundled operational data is also useful for cohort prioritization. In
the inspected snapshot it contains 62 patch entries affecting 38 named mods
(50 marked recommended), six force-small-batch mods, targeted long-running
component exceptions, READLN defaults, sibling-directory rules, and four
essential EET package names. Each item points to a concrete case IEPM should
either model, test, reject, or leave explicitly unverified.

## What IEPM must validate or strengthen

Runner's proposed URLs are heuristics:

- a GitHub tag source archive is not necessarily the author's packaged release
  asset and can omit generated/bundled files;
- a default-branch archive is mutable and cannot identify a release without
  resolving and recording an exact commit plus content hash;
- the resolver currently parses GitHub identity from the homepage URL rather
  than using Forge's explicit `gh` identifiers;
- redirects and a successful HTTP response do not prove that the response is
  the intended mod archive;
- finding one TP2 proves package shape, not target compatibility;
- manual pages still require a user-selected artifact and subsequent identity
  checks.

IEPM already has the stronger A4 boundary: exact bytes are hashed, cache
identity is content-addressed, ZIP extraction is hardened, and execution needs
an explicit materialization/installer route. Runner's resolver should feed
candidate URLs into that boundary, not replace it.

For GitHub, IEPM should prefer this sequence:

1. inspect the repository's release metadata and author-linked release assets;
2. test each plausible asset for safe archive/package structure;
3. use a tag archive only when it is the actual usable package;
4. for an unreleased branch, resolve the branch to an exact commit and label
   the resulting artifact as a derived snapshot rather than a release;
5. store SHA-256 and the candidate-selection reason.

## Current implementation caveat

The inspected Runner tree contains both `buildDownloadInfo` and a Rust
`download_mod` command. However, its current Mods panel describes the version
cache download builder as “currently-disconnected,” and the per-mod download
callback remains a commented placeholder. The README documents the intended
download-manager behavior, while this code snapshot shows that it should not
be treated as a live end-to-end guarantee without independent execution.

This does not reduce the value of the resolver rules. It changes their status
from “known working download registry” to “well-informed acquisition
candidates requiring IEPM GET, content, and package checks.”

## Patches and installer exceptions

Runner's patch manifest and install configuration contain unusually valuable
real-world failure knowledge. IEPM should ingest them into a research queue
with Runner commit/path provenance, not silently apply them or mark their
claims Supported.

Each queue item should receive one disposition:

- fixed upstream in the exact targeted release;
- represented by existing IEPM installer inputs/materialization;
- modeled as an explicit package/release exception;
- reproduced by an automated fixture and then adopted with evidence;
- performance-only and outside correctness support;
- obsolete or rejected;
- still unknown/unverified.

If IEPM ever applies a third-party patch, the patched package tree becomes a
new content identity. The original artifact hash, patch identity/hash, output
tree digest, provenance, and applicability condition must all be recorded.
