# Mod support ledger

This is the human-facing companion to the static registry. It answers two
different questions which should never be collapsed into one badge:

- **Can IEPM construct and run this exact package selection?** This is the
  lockfile's `executable` readiness, based on a pinned artifact, materialization
  route, launcher, language, and selected component selector.
- **Has this exact package/component combination been proven compatible in a
  disposable game build?** This is execution evidence. Absence of it is a
  warning, not a refusal to install, unless a concrete execution fact is
  missing.

The registry YAML remains the source of truth for artifact identity, selectors,
relationships, and claim provenance. Update this ledger in the same change as
a material support change, linking the registry record and any evidence note.
The [compatibility policy](compatibility.md#verification-scope-prove-claims-not-universes)
defines the deliberately narrow scope of each automated-verification claim.

| Mod | Component coverage | Can IEPM run it? | Compatibility standing | Notes |
| --- | --- | --- | --- | --- |
| DLC Merger | Recorded release components | Yes, for the EET fixture | Verified for the narrow EET source route | [registry](../registry/packages/dlc-merger.yaml) |
| EE Fixpack | Recorded release components | Yes, for the EET fixture | Verified for its source/target fixture selections | [registry](../registry/packages/ee-fixpack.yaml) |
| EET / EET_End | Recorded release components | Yes, for the narrow EET route | Verified only for the documented controlled route | [evidence](a5-eet-evidence.md) |
| Hidden Gameplay Options | Recorded v5.1 components | Yes | Components 0 and 10 verified separately; other combinations unverified | [registry](../registry/packages/hidden-gameplay-options.yaml) |
| Tweaks Anthology v18 | Complete mechanically-derived selector catalog in the source-tag release; Forge selections are all named | Yes | Three-component starter verified. The full Forge Tweaks selection ran in a fresh BG2EE workspace: 29 recorded, 7 correctly reported unavailable outside its intended game context. The EET combination remains unverified. | [registry](../registry/packages/tweaks-anthology.yaml) |
| Sword Coast Stratagems v35.21 | Initial named selector subset; full catalog expansion is in progress | Yes for the mapped selectors; selected component prerequisites are added before execution | Batch dispatcher plus 2000/5900/6030/6040 verified. The Ascension-demons choice now carries its TP2-declared SCS prerequisites (6510, then 5900); its broader EET combination remains unverified. | [registry](../registry/packages/stratagems.yaml) |
| Ascension | Forge-selected components mapped | Yes | Core component verified with the documented content fixture; remaining selections unverified | [registry](../registry/packages/ascension.yaml) |
| Call of the Lost Goddess | Recorded component | Yes | Verified in the documented content fixture; broad EET stack unverified | [registry](../registry/packages/call-of-the-lost-goddess.yaml) |
| Throne of the Mad God | Recorded component | Yes | Verified in the documented content fixture; broad EET stack unverified | [registry](../registry/packages/throne-of-the-mad-god.yaml) |
| EEex | Recorded components | Yes | Bootstrap/main/LuaJIT have fixture evidence; native-extension interactions remain unverified | [registry](../registry/packages/eeex.yaml) |
| Infinity UI++ | Recorded component | Yes | Its dedicated fixture is verified; broad EET stack unverified | [registry](../registry/packages/infinity-ui-plus-plus.yaml) |
| Bubb's Spell Menu | Recorded component | Yes | Its EEex-dependent fixture is verified; broad EET stack unverified | [registry](../registry/packages/bubbs-spell-menu.yaml) |

## Reading the Forge manifest

`examples/forge-coverage/modpack.yaml` is the exact, current Forge-derived
request. It is now executable at preflight for its represented selections. That
means IEPM knows how to obtain, prepare, select, and launch every selected
package. It does **not** mean the entire multi-mod EET result has been run and
verified as one build. The full Tweaks selection was also run in a fresh BG2EE
workspace to test the route: seven choices were skipped by Tweaks as
unavailable in that non-EET context, so the EET interaction remains a real
verification task rather than a green badge. The warnings are intentional
evidence gaps, not incompatibility claims.

Do not call an unverified row unsupported simply because its author has not
participated. A release can become verified through exact artifacts, complete
selectors, curated relationships, and automated disposable-build evidence.
