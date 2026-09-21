# Forge-derived coverage evidence

This is an evidence and coverage note for the large EET order supplied in
`Forge-Mod_Install List.txt`. It records what the current Product A route
proved, what it deliberately rejected, and the minimum fresh fixture research
performed without treating existing local mod downloads as input evidence.

## Exact identity scope

`examples/forge-coverage/modpack.yaml` expresses the portion of that order
whose package identities already exist in IEPM's registry. It intentionally
does **not** substitute Forge's `stratagems` request with the distinct
`tactics-remix` package. Likewise, Call of the Lost Goddess and Throne of the
Mad God are left out because they have no registry records. This preserves the
requester's intent and makes coverage gaps visible rather than making a
plausible-looking but different build.

The fixture contains the explicit EET prerequisites required for a meaningful
graph: DLC Merger, EE Fixpack in its source and target environments, EET, and
EET_End. It also represents the currently known identities for EEex, Infinity
UI++, Bubb's Spell Menu, Hidden Gameplay Options, Ascension, and Tweaks
Anthology. It is a resolver-coverage fixture, not an install manifest.

## Large-build result

Resolving the fixture produced a schema-3 lockfile with the expected
cross-environment EET graph. `iepm verify --require-executable` and a guarded
`iepm install` both rejected it as `analysis-only` before any workspace,
artifact cache, log directory, or game tree could be changed.

The causal blockers are intentionally specific: Ascension, Bubb's Spell Menu,
EEex, Infinity UI++, and Tweaks Anthology each still lack a verified artifact,
installer definition, and selected executable component in the registry. That
is a passing safety result: the resolver supports discovery and planning but
does not pretend a broad mod order is runnable from partial metadata.

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

No support for executable/self-extracting archives was added. Treating an EXE
as an archive or an installer is an execution-boundary decision, not a format
enum cleanup; it needs a specific release recipe and a trusted disposable
execution fixture first.

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
