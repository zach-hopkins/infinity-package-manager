# Mod support ledger

The first Product A coverage cohort is now frozen in
[product-a-cohort.md](product-a-cohort.md): 99 packages meet 95.18% of its
dated, stack-normalized input corpus. This ledger remains the human companion
for PA-4 component and evidence work. Cohort membership schedules that work;
it does not turn an indexed package into a green support badge.

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
The [verification status policy](verification-policy.md) defines the exact
gates for the public `Verified`, `Supported`, `Untested`, and `Incompatible`
labels. This ledger must use those words only with those meanings.

| Mod | Component coverage | Can IEPM run it? | Compatibility standing | Notes |
| --- | --- | --- | --- | --- |
| DLC Merger 2.1 | Complete 4-component structural catalog | Yes, for the EET fixture | **Untested** — clean install evidence exists; the two newly mapped optional paths have no execution evidence yet | [registry](../registry/packages/dlc-merger.yaml) |
| EE Fixpack Beta 2 | 3 user-facing selectors mapped; 4 observed TP2 declarations are internal maintenance/test entries | Yes, for the EET fixture | **Untested** — source/target core install evidence exists; optional selectors still need their own evidence | [registry](../registry/packages/ee-fixpack.yaml) |
| EET / EET_End | EET core plus optional desktop shortcut structurally mapped; EET_End standard route recorded | Yes, for the narrow EET route | **Untested** — the controlled route installed; policy launch and gameplay smokes remain | [evidence](a5-eet-evidence.md) |
| Hidden Gameplay Options v5.1 | Complete 43-component structural catalog | Yes | **Untested** — components 0 and 10 have separate install receipts; the other 41 selectors and policy launch smoke remain untested | [registry](../registry/packages/hidden-gameplay-options.yaml) |
| IWDification v11 | 22 current-game selectors mapped against 24 TP2 declarations; one deprecated and one classic-only entry omitted; two component-level Tweaks overlaps guarded | Yes for mapped selectors | **Untested** — bard songs has an exact BG2EE install receipt; the newly mapped choices and policy launch smoke remain | [registry](../registry/packages/iwdification.yaml) |
| Tweaks Anthology v18 | 394 stable IDs mapped against 446 live TP2 declarations; three current-game residual selectors need custom values, while 49 are legacy/deprecated; selected EEex prerequisites and two IWDification overlaps are guarded | Yes for mapped source selectors | **Untested** — starter and Forge install receipts exist, but policy launch smoke and broad component verification remain | [registry](../registry/packages/tweaks-anthology.yaml) |
| Sirene v3.1 | Complete ten-component structural catalog from the SHA-pinned official release-tag archive | BGEE NPC default built and launched; optional portrait/class selectors structurally mapped | **Supported** for BGEE NPC default on tested 2.6.6 fingerprint; optional choices and EET Untested | [evidence](pa4-bgee-npc-friendships-evidence.md) |
| Coran's BG Friendship v5.2 | Complete one-component catalog, three languages | BGEE English default built and launched | **Supported** on tested BGEE 2.6.6 fingerprint; EET Untested | [evidence](pa4-bgee-npc-friendships-evidence.md) |
| Xan's BG1 Friendship v11 | Complete one-component catalog, five languages | BGEE English default built and launched | **Supported** on tested BGEE 2.6.6 fingerprint; EET Untested | [evidence](pa4-bgee-npc-friendships-evidence.md) |
| LeUI 4.9.1 | Complete three-component structural catalog from the SHA-pinned author tag archive; explicit before-SCS placement | All three BG2EE English selectors built and launched | **Supported** for tested BG2EE 2.6.6 selection; other games and UI-overhaul combinations Untested | [evidence](pa4-bg2ee-leui-evidence.md) |
| High Quality Soundclips v1.3 | Complete one-component catalog, three WeiDU languages | BG2EE English default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET and audio-quality behavior Untested | [evidence](pa4-bg2ee-hq-soundclips-evidence.md) |
| Banter Pack v18 | Complete four-component catalog: SoA/ToB content and two independent accelerators | All four BG2EE English selectors built and launched | **Supported** for tested BG2EE 2.6.6 selection; EET and in-game banter behavior Untested | [evidence](pa4-bg2ee-banterpack-evidence.md) |
| BioWare NPC Flirt Packs v1.07 | Complete 16-component structural catalog: four companions, SoA/ToB, with separate Solaufein interaction choices | Eight base BG2EE English choices built and launched | **Supported** for tested eight-choice BG2EE 2.6.6 selection; Solaufein choices, EET, and dialogue behavior Untested | [evidence](pa4-bg2ee-npcflirt-evidence.md) |
| Drake v1.7a | Complete six-component structural catalog, including one required portrait choice | BGEE NPC and default portrait installed in two independent sealed builds; both crashed before menu | **Incompatible** for this exact BGEE 2.6.6 NPC + default portrait configuration after two identical launch faults; other choices/EET Untested | [evidence](pa4-bgee-drake-evidence.md) |
| Cernd Friendship v1.4 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/cernd-friendship.yaml) |
| Haer'dalis Friendship v1.2 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/haerdalis-friendship.yaml) |
| Imoen Friendship v3.6 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/imoen-friendship.yaml) |
| Korgan Friendship v1.6 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/korgan-friendship.yaml) |
| Mazzy Friendship v3.5 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/mazzy-friendship.yaml) |
| Minsc Friendship v1.3 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/minsc-friendship.yaml) |
| Sarevok Friendship v2.7 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/sarevok-friendship.yaml) |
| Valygar Friendship v1.5 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/valygar-friendship.yaml) |
| Viconia Friendship v4.5 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/viconia-friendship.yaml) |
| Yoshimo Friendship v5.0 | Complete one-component catalog | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/yoshimo-friendship.yaml) |
| Assassinations v19 | Complete one-component catalog, eight languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/assassinations.yaml) |
| Sellswords v9.1 | Complete one-component catalog, six languages; TP2 requires ToB content | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/sellswords.yaml) |
| Back to Brynnlaw v9 | Complete one-component catalog, six languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/back-to-brynnlaw.yaml) |
| Dungeon Crawl v13.1 | Complete one-component catalog, seven languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/dungeon-crawl.yaml) |
| Dorn Friendship 1.2 | Complete one-component catalog, two languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/dorn-friendship.yaml) |
| Hexxat Friendship 1.2 | Complete one-component catalog, two languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/hexxat-friendship.yaml) |
| Jaheira Friendship v1.3 | Complete one-component catalog, two languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/jaheira-friendship.yaml) |
| Korgan's Redemption v10.0.1 | Complete one-component catalog, five languages | BG2EE default built and launched | **Supported** on tested BG2EE 2.6.6 fingerprint; EET Untested | [registry](../registry/packages/korgans-redemption.yaml) |
| Sword Coast Stratagems v35.21 | 134 stable IDs mapped against 143 live local TP2 declarations: every BGEE/BG2EE/EET-facing selector and the personal EET build | Yes for mapped selectors; prerequisites and documented alternative choices are checked before execution | **Untested** — bounded selectors have install evidence, but policy launch smoke and gameplay evidence remain incomplete; the remaining 9 declarations are IWD-only or legacy-only | [registry](../registry/packages/stratagems.yaml) |
| Ascension 2.1.0 | Complete 19-component structural catalog; all selectors matched against the local TP2 audit | Yes | **Untested** — the core route has install evidence, while the newly mapped optional components still require their own evidence | [registry](../registry/packages/ascension.yaml) |
| Call of the Lost Goddess v3.1 | Complete 3-component structural catalog | Yes | **Untested** — the core route has fixture install evidence; optional portraits/voiceover and policy launch smoke remain untested | [registry](../registry/packages/call-of-the-lost-goddess.yaml) |
| Throne of the Mad God v2.3 | Complete 1-component structural catalog | Yes | **Untested** — fixture install evidence exists; policy launch smoke remains | [registry](../registry/packages/throne-of-the-mad-god.yaml) |
| EEex v1.2.0 | Complete 9-component structural catalog; every optional module carries its mechanical core prerequisite | Yes | **Untested** — bootstrap/main/LuaJIT have install evidence; optional modules, launch smoke, and native-extension interactions remain | [registry](../registry/packages/eeex.yaml) |
| Infinity UI++ v1.23 source | Complete 9-component structural catalog; quicksave slot selections are component-scoped alternatives | Yes | **Untested** — the core has dedicated install evidence; optional selectors and policy launch smoke remain untested | [registry](../registry/packages/infinity-ui-plus-plus.yaml) |
| Bubb's Spell Menu v5.2 | Complete 1-component structural catalog | Yes | **Untested** — the EEex-dependent fixture installed; policy launch smoke remains | [registry](../registry/packages/bubbs-spell-menu.yaml) |

## Reading the Forge manifest

`examples/forge-coverage/modpack.yaml` is the exact, current Forge-derived
request. It is executable at preflight for its represented selections. A fresh
IEPM-managed build named `forge-scs-prereq-20260921` completed all 14 planned
actions and was sealed. That is evidence for this exact source fingerprint,
artifact set, component selection, toolchain, and ordering—not a claim that
every unselected component or arbitrary EET stack is verified. The completed
run retained narrow warning receipts for EET, SCS, and Tweaks; their requested
components were independently recorded in `WeiDU.log`. The warnings are
evidence to inspect, not incompatibility claims.

Do not call an Untested row impossible simply because its author has not
participated. A release can become Supported through exact artifacts, complete
selectors, curated relationships, and automated disposable-build evidence.

## Cohort work boundary

The frozen cohort contains 19 packages that were already mapped when it was
selected and 80 indexed candidates. Twenty-six of those candidates (Sirene,
LeUI, thirteen BG2 friendship releases, four Pocket Plane quest mods,
Korgan's Redemption, Coran's BG Friendship, Xan's BG1 Friendship, and High
Quality Soundclips, Banter Pack, BioWare NPC Flirt Packs, and Drake) now have curated executable identities; 54 remain
discovery-only. The remaining
candidates are **not** silently
excluded: they may proceed through IEPM's opaque local-package route once
their exact bytes are supplied and safely inspected. They simply do not yet
have an executable release record or component-by-component support ledger.

For the mapped rows above, retain the same discipline: “Yes, for this exact
component route” is not “all components and combinations are supported.” The
complete selected/deferred list and PA-2 acquisition observations live in the
[Product A cohort report](product-a-cohort.md).

The PA-4 component-inventory procedure and the first SCS audit are documented
in [PA-4 component audit](pa4-component-audit.md). It deliberately exposes
unmapped TP2 selectors without turning them into misleading stable IDs.
