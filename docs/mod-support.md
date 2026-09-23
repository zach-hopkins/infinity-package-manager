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
| Unfinished Business v28 | 24 current BG2EE/EET choices mapped from 26 main-TP2 declarations; two deprecated BG2EE choices withheld | Three English BG2EE restorations built and launched | **Supported** for the tested BG2EE 2.6.6 three-choice route; the other 21 choices, EET, and gameplay Untested | [evidence](pa4-bg2ee-unfinished-business-evidence.md) |
| Reduce Save Compression v1.2 | Complete one-component catalog; directly patches game executable bytes | BG2EE 2.6.6 default installed and launched from a sealed disposable build | **Supported** for the tested Windows BG2EE binary; save behavior, other game binaries, and EET Untested | [evidence](pa4-bg2ee-reduce-save-compression-evidence.md) |
| Ajantis BG1 Expansion v22 | Complete three-component catalog: main, optional shield art, optional SoD NPC crossmod content | DLC Merger plus Ajantis main built and launched from this unmerged-SoD BGEE source | **Supported** for the tested BGEE 2.6.6 route; optional choices, BG1NPC interaction, EET, and dialogue Untested | [evidence](pa4-bgee-ajantis-bg1-expansion-evidence.md) |
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
| Turnabout v1.8 | Complete two-component catalog; main requires Ascension's rewritten final chapter, while optional portrait is independent | Ascension plus Turnabout main built and launched from sealed BG2EE | **Supported** for the tested English BG2EE 2.6.6 route; optional portrait, EET, and endgame behavior Untested | [evidence](pa4-bg2ee-turnabout-evidence.md) |
| The Longer Road v2.0.7 | Complete two-component catalog; main requires two exact Ascension choices, optional portrait requires main | Ascension plus Longer Road main built and launched from sealed BG2EE | **Supported** for the tested English BG2EE 2.6.6 route; optional portrait, EET, and story behavior Untested | [evidence](pa4-bg2ee-longerroad-evidence.md) |
| Isra NPC for BGII v3.1 | Complete two-component catalog; optional crossmod requires main and detects other installed NPCs by resource | Main NPC component built and launched from sealed BG2EE | **Supported** for the tested English BG2EE 2.6.6 main route; crossmod, EET, and story behavior Untested | [evidence](pa4-bg2ee-isra-bg2-evidence.md) |
| Isra NPC for BG v3.5 | Complete three-component catalog; Valerie and Gavin crossmod choices have external partner requirements | DLC Merger then main NPC built and launched from sealed BGEE | **Supported** for tested English BGEE 2.6.6 main route; crossmod, EET, and story behavior Untested | [evidence](pa4-bgee-isra-bg1-evidence.md) |
| Black Pits in BG 1.2.1 | Complete one-component catalog; TP2 accepts BGEE/EET and author recommends very early placement | Main component installed and sealed after DLC Merger; menu inspection inconclusive | **Untested** pending observed BGEE menu and EET route | [install evidence](pa4-bgee-black-pits-dark-horizons-evidence.md) |
| Dark Horizons 3.06 | Complete two-component catalog after multiline TP2-title parser fix; optional nerfs requires main | Main quest component installed and sealed after Black Pits in BG; menu inspection inconclusive | **Untested** pending observed BGEE menu; optional nerfs and EET also Untested | [install evidence](pa4-bgee-black-pits-dark-horizons-evidence.md) |
| CoM Encounters v1.22 | Complete three-component catalog; Improved Druids and Improved Shagbag separate | Main component installed and sealed in clean BG2EE quest build; menu smoke unavailable | **Untested** pending observed menu; optional selectors and EET also Untested | [install evidence](pa4-bg2ee-three-quest-expansion-evidence.md) |
| Tower of Deception v4.1.0 | Complete four-component catalog; three options require main | Main component installed and sealed in clean BG2EE quest build; menu smoke unavailable | **Untested** pending observed menu; optional selectors and EET also Untested | [install evidence](pa4-bg2ee-three-quest-expansion-evidence.md) |
| Test Your Mettle! 1.6 | Complete six-component catalog; four XP alternatives require main and conflict with each other, Spacewarp stores independent | Main component installed and sealed in clean BG2EE quest build; menu smoke unavailable | **Untested** pending observed menu; optional selectors and EET also Untested | [install evidence](pa4-bg2ee-three-quest-expansion-evidence.md) |
| The Boareskyr Bridge Scene v8 | Complete four-component SoD/EET chain; each choice requires its predecessor | All four English choices installed and sealed on SoD-enabled BGEE | **Untested** pending observed menu and gameplay; EET also Untested | [install evidence](pa4-bgee-sod-bridge-encounters-evidence.md) |
| Extra Expanded Enhanced Encounters! 4.4 | Complete eight-component BGEE/EET catalog; cave bears require SoD content | All eight English choices installed and sealed on SoD-enabled BGEE | **Untested** pending observed menu and gameplay; EET also Untested | [install evidence](pa4-bgee-sod-bridge-encounters-evidence.md) |
| Reflections of Destiny 0.9.4 | Seven TP2 selectors: five other SoD/BGEE choices, one EET-only, and selector 230 blocked after two parse-error failures; four commented-out blocks excluded | Four valid BGEE choices installed and sealed with Road to Discovery; Caelar rewrite guarded against it | **Incompatible** for exact selector 230; tested install route and other choices **Untested** pending menu/gameplay | [failure evidence](pa4-bgee-reflections-feyr-failure.md), [install evidence](pa4-bgee-sod-story-valid-evidence.md) |
| Road to Discovery 6.0 | Eight active selectors with package-local requirements; selector 80 is deprecated and unavailable | All eight English choices installed and sealed on SoD-enabled BGEE | **Untested** pending menu/gameplay; EET also Untested | [install evidence](pa4-bgee-sod-story-valid-evidence.md) |
| Almateria's Restoration Project v10.1 | All 14 TP2 declarations accounted for: 12 EE-facing, one deprecated, one classic-only; Alt Slayer requires Final Slayer Dream | All 12 EE-facing English choices installed and sealed on clean BG2EE | **Untested** pending observed menu and gameplay; EET also Untested | [install evidence](pa4-bg2ee-almateria-evidence.md) |
| Hidden Adventures Beta_9 | Complete eleven-component catalog; Silver Dagger import EET-only, ten others BG2EE-facing | All ten English BG2EE choices installed and sealed | **Untested** pending observed menu/gameplay; EET and Silver Dagger also Untested | [install evidence](pa4-bg2ee-hidden-adventures-evidence.md) |
| Crossmod Banter Pack v30 | Complete three-component catalog; actual content depends on already-installed covered NPC mods | All three English selectors installed and sealed after Isra BG2 | **Untested** pending menu and two-partner content test; one-partner receipt proves only the installer route | [install evidence](pa4-bg2ee-isra-crossmod-evidence.md) |
| Ascalon's Questpack 7.0 | All thirteen TP2 choices mapped: nine BG1-facing choices with three mutually exclusive variant pairs, one BG2-facing quest, three alternatives | Nine BGEE choices and the BG2EE-only quest installed and sealed in separate clean builds | **Untested** pending observed menus/gameplay; alternate variants and EET also Untested | [install evidence](pa4-ascalon-evidence.md) |
| Endless BG1 20.2 | Complete 17-component catalog after excluding two embedded-dialogue BEGINs; all options require main, two Elminster variants conflict; known Reflections overlap guarded | Sixteen English selectors installed and sealed after DLC Merger on BGEE | **Untested** pending observed menu/gameplay; alternate Elminster and EET also Untested | [install evidence](pa4-bgee-endless-bg1-evidence.md) |
| Imoen 4 Ever v11.6 | Complete 16-component catalog spanning BG2 and SoD; BG2 prerequisites and four SoD camp prerequisites mapped, SoD portrait alternatives guarded; two SoD OR predicates remain WeiDU-enforced | BGEE/SoD and BG2EE configurations installed and sealed | **Untested** pending menu smokes, alternate portrait, and OR-predicate preflight coverage | [install evidence](pa4-imoen-4-ever-evidence.md) |
| Jastey's SoD Tweakpack v13 | Complete 17-component catalog; dialogue, starting-XP, and ending alternatives guarded; EET-only portrait and SoD-only fight distinguished | Nine SoD selectors installed and sealed after DLC Merger | **Untested** pending observed menu/gameplay, other alternatives, and EET route | [install evidence](pa4-bgee-jastey-sod-evidence.md) |
| BG Mini Quests and Encounters v31 | Complete 18-component catalog; all BGEE/EET eligible, mostly before EET_End; author recommends before BG1 NPC Project | All eighteen BGEE selectors installed and sealed after DLC Merger; unmerged-SoD source correctly rejected first | **Untested** pending observed menu/gameplay and EET route | [install evidence](pa4-bgee-bg-mini-quests-evidence.md) |
| Brage's Redemption v10.1 | Exact official tag and four labeled selectors; BGEE/EET main requires BGQE Brage's Sword, unlike BG2EE | Four selectors sealed after BGQE on BGEE; main and crossmod sealed without BGQE on BG2EE | **Untested** pending observed menu/gameplay and EET route; BG2 content incomplete per author | [install evidence](pa4-bgee-brage-evidence.md) |
| BP-BGT Worldmap v13.1.1 | Exact ZIP hashed, six selectors mechanically detected | **Not yet executable in IEPM**: main TP2 asks for map-size input but IEPM gives WeiDU null stdin | **Untested**; discovery-only pending explicit installer-answer support, not known incompatible | [blocker](pa4-worldmap-interactive-blocker.md) |
| BG:EE Classic Movies V2.4.1 | Exact ZIP hashed, eight selectors mechanically detected | **Not yet executable in IEPM**: TP2 writes to `%USER_DIRECTORY%/Baldur.lua` or `Baldur.ini` outside disposable game root | **Untested**; discovery-only pending containment of installer side effects, not known incompatible | [blocker](pa4-classic-movies-external-write-blocker.md) |
| Generalized Biffing v2.9 | Complete two-choice subcomponent catalog; final-phase placement reflects author's after-all-mods direction | Banter Pack then media-only Biffing built and launched from sealed BG2EE | **Supported** for this narrow English BG2EE 2.6.6 route; all-files choice, EET, large stacks, and performance Untested | [evidence](pa4-bg2ee-generalized-biffing-evidence.md) |
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
selected and 80 indexed candidates. Fifty-two of those candidates (Sirene,
LeUI, thirteen BG2 friendship releases, four Pocket Plane quest mods,
Korgan's Redemption, Coran's BG Friendship, Xan's BG1 Friendship, and High
Quality Soundclips, Banter Pack, BioWare NPC Flirt Packs, Drake, Unfinished
Business, Reduce Save Compression, Ajantis BG1 Expansion, Turnabout,
Generalized Biffing, The Longer Road, Isra NPC for BGII, Isra NPC for BG,
Black Pits in BG, Dark Horizons, CoM Encounters, Tower of Deception, Test
Your Mettle!, The Boareskyr Bridge Scene, Extra Expanded Enhanced Encounters!,
Reflections of Destiny, Road to Discovery, Almateria's Restoration Project,
Hidden Adventures, Crossmod Banter Pack, Ascalon's Questpack, Endless BG1,
Imoen 4 Ever, Jastey's SoD Tweakpack, BG Mini Quests and Encounters, and
Brage's Redemption) now have curated executable identities; 28 remain
discovery-only. The remaining
candidates are **not** silently
excluded: the [remaining cohort triage](pa4-remaining-cohort-triage.md)
names each acquisition state. They may proceed through IEPM's opaque
local-package route once
their exact bytes are supplied and safely inspected. They simply do not yet
have an executable release record or component-by-component support ledger.

For the mapped rows above, retain the same discipline: “Yes, for this exact
component route” is not “all components and combinations are supported.” The
complete selected/deferred list and PA-2 acquisition observations live in the
[Product A cohort report](product-a-cohort.md).

The PA-4 component-inventory procedure and the first SCS audit are documented
in [PA-4 component audit](pa4-component-audit.md). It deliberately exposes
unmapped TP2 selectors without turning them into misleading stable IDs.
