# Forge-derived coverage evidence

This is an evidence and coverage note for the large EET order supplied in
`Forge-Mod_Install List.txt`. It records what the current Product A route
proved, what it deliberately rejected, and the minimum fresh fixture research
performed without treating existing local mod downloads as input evidence.

## Exact identity scope

`examples/forge-coverage/modpack.yaml` now expresses every exact package
identity from the reference order. It intentionally does **not** substitute
Forge's `stratagems` request with the distinct `tactics-remix` package. Call
of the Lost Goddess and Throne of the Mad God have likewise been added under
their exact identities. This preserves the requester's intent and makes
remaining component and execution gaps visible rather than making a
plausible-looking but different build.

The fixture contains the explicit EET prerequisites required for a meaningful
graph: DLC Merger, EE Fixpack in its source and target environments, EET, and
EET_End. It also represents the currently known identities for EEex, Infinity
UI++, Bubb's Spell Menu, Hidden Gameplay Options, Call of the Lost Goddess,
Throne of the Mad God, Ascension, Tweaks Anthology, and SCS. It is a
resolver-coverage fixture, not an install manifest. The initial SCS component
subset is deliberately bounded; `inspect-package` exposes the complete 146
structural component declarations for later curated mapping.

## Large-build result

Resolving the fixture produced a schema-3 lockfile with the expected
cross-environment EET graph. `iepm verify --require-executable` and a guarded
`iepm install` both rejected it as `analysis-only` before any workspace,
artifact cache, log directory, or game tree could be changed.

The causal blockers are intentionally specific: exact artifact identity and
many TP2 selectors now exist, but Ascension, Bubb's Spell Menu, EEex, Infinity
UI++, Call of the Lost Goddess, Throne of the Mad God, Tweaks Anthology, and
SCS still lack one or more verified launcher, language, numeric-selector, or
materialization facts. SCS additionally records a Windows EXE artifact that
IEPM deliberately refuses to prepare generically. That is a passing safety
result: the resolver supports discovery and planning but does not pretend a
broad mod order is runnable from partial metadata.

## Fresh popular-mod fixtures

Fresh release downloads were made into a new evidence-only directory, separate
from the user's prior local Mods directory and from IEPM's retained cache.
They were hashed before any inspection.

| Fixture | Official release asset | SHA-256 | What the pass established |
| --- | --- | --- | --- |
| Tweaks Anthology v18 | `cdtweaks-v18.exe` | `a90620d9c8d65529002da7d17526bf79d3771f5ec7baad747e1babd82604343d` | Current official Windows distribution is an EXE, outside A4's verified-ZIP preparation path. |
| EEex v1.3.0 | `eeex-v1.3.0.zip` | `ec195c2b1be3842d35d5ab7b9627c89ba455968b7b50bb4f010cbbe5e4917e14` | The archive can be safely treated as a ZIP for structural inspection; it contains an `EEex.tp2` and native-extension content. |
| Sword Coast Stratagems v35.21 | `stratagems-35.21.exe` | `7e8754e1bb541fce1686314e0d4967b66c5740615907521dca5e63153326602f` | Current official Windows distribution is an EXE, outside A4's verified-ZIP preparation path. |

The locally inspected EEex v1.3.0 TP2 declares English at language index 0
and exposes a core component plus numbered optional modules. These are
structural observations only. They neither verify EEex's native installer
effects nor establish that any module is compatible with this build order.

The registry can now retain an EXE artifact's exact SHA-256 and platform while
the resolver/A4 explicitly reject it as non-preparable. This is not generic
EXE support: treating an EXE as an archive or installer still needs a specific
release recipe and a trusted disposable execution fixture first.

Upstream release pages and project documentation establish the distribution
context for these fixtures: [Tweaks Anthology releases](https://github.com/Gibberlings3/Tweaks-Anthology/releases),
[EEex releases](https://github.com/Bubb13/EEex/releases), and [Sword Coast
Stratagems releases](https://github.com/Gibberlings3/SwordCoastStratagems/releases).

## GUI boundary

This evidence is sufficient to start a **thin, safety-preserving GUI** over the
current Product A flow: search, previewed intent edits, resolve, readiness
explanation, managed snapshot/workspace creation, explicit install
confirmation, receipts, and sealing. The GUI must present `analysis-only` as
a hard non-installable state and show these causal blockers.

It is not evidence for a one-click import/install of the Forge order. The next
registry work is release-specific acquisition, materialization, component, and
execution evidence for the missing exact packages—not a silent substitution or
generic EXE execution feature.
