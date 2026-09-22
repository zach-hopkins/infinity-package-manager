# PA-4 component audit

PA-4 starts with full inert structural inventories before assigning stable IEPM
component IDs. This order matters: a numeric WeiDU selector is a
release-specific implementation detail, not user intent. IEPM must not fill a
registry with durable-looking names such as `component-6500` merely to make a
coverage percentage look better.

`iepm review-tp2` now reports a `component_catalog` alongside its existing
release-drift findings:

- total TP2 `BEGIN` declarations observed;
- stable registry IDs whose release-specific selectors match; and
- every remaining raw `BEGIN`/`DESIGNATED`/`LABEL` observation.

It reads TP2 text only. It does not execute WeiDU code, modify the registry, or
infer compatibility. Indented WeiDU control-flow `BEGIN` blocks are excluded:
only top-level component declarations count. Line and block comments are also
excluded; this corrected Tweaks Anthology's initial count by four disabled
declarations.

## First audit: SCS v35.21 local package tree

The available local SCS tree reports TP2 `VERSION 35.21` and **143** live structural
component declarations. IEPM now maps **134** stable component IDs to that
installer: every BGEE/BG2EE/EET-facing selector, including the supplied
personal build and focused spell-tweak selectors. The newly mapped entries
retain mechanically derived selector/prerequisite evidence only; they are not
a compatibility badge for the whole large stack. The focused selectors each
carry a narrow same-package conflict with SCS's all-spell-tweaks dispatcher,
exactly as its TP2 declares; they do not falsely conflict with one another.
The remaining **9** visible declarations are deliberately not user-facing:
IWD-only selectors, legacy-only helper selectors, and internal TP2 test or
resource-collection blocks.

This confirms the correct next PA-4 task: name/classify the unmapped entries
from SCS's author release text, attach release-specific selectors, and keep
their state `Untested` until evidence exists. It does **not** justify calling
the local `setup-stratagems.exe` the pinned release artifact: its SHA-256 does
not match the registry's official SFX artifact hash, so it is a useful local
TP2 source only.

Run the audit with:

```text
iepm review-tp2 --registry registry --package stratagems \
  --release-id stratagems-v35.21-windows \
  --tp2 <local-scs>/stratagems/setup-stratagems.tp2 \
  --installer-tp2 stratagems/setup-stratagems.tp2
```

The same audit applies to every cohort release once its exact artifact is
available. A mismatch becomes a review item; it is never silently inherited.

## Initial local audit results

The same structural review was run against the available local source trees.
These counts are not support statuses; they are a precise PA-4 naming and
classification backlog.

| Package/release | Observed TP2 declarations | Existing stable IDs matched | Still unmapped |
| --- | ---: | ---: | ---: |
| SCS v35.21 | 143 | 134 | 9 intentionally non-user-facing |
| Tweaks Anthology v18 source | 446 | 394 | 52 |
| IWDification v11 IEMOD | 24 | 22 | 2 non-user-facing/legacy |
| Sirene v3.1 tag archive | 10 | 10 | 0 |
| LeUI v4.9.1 tag archive | 3 | 3 | 0 |
| Banter Pack v18 tag archive | 4 | 4 | 0 |
| BioWare NPC Flirt Packs v1.07 tag archive | 16 | 16 | 0 |
| Drake v1.7a tag archive | 6 | 6 | 0 |
| Ten Spellhold Studios BG2 friendship tag archives | 10 total | 10 | 0 |
| Dorn, Hexxat, and Jaheira Friendships; Korgan's Redemption tag archives | 4 total | 4 | 0 |
| Coran's BG Friendship and Xan's BG1 Friendship tag archives | 2 total | 2 | 0 |
| Hidden Gameplay Options v5.1 | 43 | 43 | 0 |
| Ascension 2.1.0 | 19 | 19 | 0 |
| Call of the Lost Goddess v3.1 | 3 | 3 | 0 |
| Infinity UI++ v1.23 source | 9 | 9 | 0 |
| EEex v1.2.0 | 9 | 9 | 0 |
| Bubb's Spell Menu v5.2 | 1 | 1 | 0 |
| Throne of the Mad God v2.3 | 1 | 1 | 0 |
| DLC Merger 2.1 | 4 | 4 | 0 |
| EE Fixpack Beta 2 | 7 | 3 | 4 internal-only |

The higher Tweaks number reflects the source TP2's full structural surface,
including choices/subcomponents; it does not mean 446 independent user-facing
features. The existing Forge selections are represented, but that is not the
same as a complete curated component catalog. This audit corrects an earlier
over-broad impression: the registry has substantial **selected-route** support
for Tweaks and SCS, not complete component support yet. Ascension's six clear
author-labeled optional components were then added as mechanically derived,
explicitly Untested selectors, closing its structural catalog without claiming
new compatibility or verification evidence. DLC Merger's two clear
author-labeled optional entries were treated the same way, closing its small
four-component catalog. EE Fixpack contributes three real user-facing
selectors; its other four TP2 declarations are explicit internal generation or
test paths, so they remain recorded by the audit but are intentionally not
offered as user components.
The Tweaks residual audit identified one additional BGEE/BG2EE/EET selector,
IWD casting graphics (70), which is now mapped. Three remaining current-game
selectors (3176, 3347, 3358) request custom user values, so they are not yet
honestly expressible as fixed one-click choices. The other 49 residual raw
declarations are deprecated or gated to classic/IWD/PST variants; they are not
missing current-game choices. The source TP2 also revealed that two previously
mapped Tweaks selectors (72 and 80) cannot run on BGEE/BG2EE/EET. They remain
visible as release-specific choices but are marked unsupported on those games.
IWDification's 22 current-game choices are now mapped; its residual 72 is
deprecated and 80 is gated to classic BG2. Its components 10/20 reciprocally
exclude Tweaks 70/100. IEPM models only those two component-level overlaps,
not a broad conflict between the packages. Every newly mapped selector is
still Untested until an exact managed install produces evidence.
The SCS count was also corrected after the inert reader excluded three
declarations inside block comments (an internal test and two resource
collectors). Earlier prose counted those disabled blocks as live components.

The first newly promoted cohort release, Sirene v3.1, exposed a TP2-reader
edge case: unindented `THEN BEGIN` action blocks previously swallowed later
components. The inert reader now preserves all ten implicit-number selectors.
Its component catalog is complete, but `review-tp2` reports
`tp2-version-unobserved` because this author TP2 has no `VERSION` declaration;
the v3.1 release tag is retained separately. This is not install or gameplay
verification.
LeUI v4.9.1 similarly has all three TP2 declarations mapped to stable user
choices. Its author documents an EE 2.6 route and installation before UI
patchers. All three BG2EE choices completed in one clean revision-pinned
disposable build, were recorded in `WeiDU.log`, and reached the Shadows of
Amn menu. See [`pa4-bg2ee-leui-evidence.md`](pa4-bg2ee-leui-evidence.md).
The ten friendship releases each expose one mechanically matched component.
Their author READMEs identify BG2EE and EET. A combined clean BG2EE 2.6.6
managed build completed all ten, recorded them in `WeiDU.log`, sealed, and
reached the Shadows of Amn menu. The exact-scope evidence is in
[`pa4-friendships-evidence.md`](pa4-friendships-evidence.md); EET and gameplay
remain untested.

Assassinations v19, Sellswords v9.1, Back to Brynnlaw v9, and Dungeon Crawl
v13.1 each have one TP2 declaration matched to a stable IEPM component ID.
`review-tp2` reports `match` for all four, including exact author language
labels. Their combined BG2EE 2.6.6 route completed all four actions, recorded
all four in `WeiDU.log`, sealed, and reached the playable Shadows of Amn
menu. See [`pa4-bg2ee-quests-evidence.md`](pa4-bg2ee-quests-evidence.md).
EET and gameplay remain untested.
The follow-up Dorn, Hexxat, and Jaheira Friendships and Korgan's Redemption
also each have one exact author-tagged TP2 declaration and a `review-tp2`
`match`. All four English defaults completed in one revision-pinned fresh
disposable BG2EE 2.6.6 build and reached the Shadows of Amn menu. See
[`pa4-bg2ee-friendship-expansion-evidence.md`](pa4-bg2ee-friendship-expansion-evidence.md);
EET and gameplay are not inferred.
Coran's BG Friendship v5.2, Sirene v3.1, and Xan's BG1 Friendship v11 each
had their English default selector recorded in one clean revision-pinned
disposable BGEE 2.6.6 build, which sealed and reached the main menu. See
[`pa4-bgee-npc-friendships-evidence.md`](pa4-bgee-npc-friendships-evidence.md).
Sirene's nine optional selectors and all EET paths remain Untested.
High Quality Soundclips v1.3 has its one LABEL-bearing component structurally
matched and completed in a clean revision-pinned disposable BG2EE 2.6.6
build, followed by a Shadows of Amn menu smoke. See
[`pa4-bg2ee-hq-soundclips-evidence.md`](pa4-bg2ee-hq-soundclips-evidence.md).
EET and audible output remain Untested.
Banter Pack v18 has four LABEL-bearing SoA/ToB content and accelerator
components structurally matched to stable IDs. All four English selections
installed in a clean revision-pinned disposable BG2EE build and reached the
Shadows of Amn menu. See
[`pa4-bg2ee-banterpack-evidence.md`](pa4-bg2ee-banterpack-evidence.md).
EET and dialogue behavior remain Untested.
BioWare NPC Flirt Packs v1.07 has all sixteen implicit-number selectors
mapped to stable IDs: four companions, SoA and ToB content, and separate
Solaufein interaction choices. Its archived TP2 is Windows-1252 rather than
UTF-8, so `review-tp2` used a mechanically transcoded temporary copy; the
exact artifact hash remains the identity. The eight base English choices
completed a clean revision-pinned BG2EE build and reached the Shadows of Amn
menu; see [`pa4-bg2ee-npcflirt-evidence.md`](pa4-bg2ee-npcflirt-evidence.md).
The eight Solaufein choices and dialogue behavior remain Untested.
Drake v1.7a maps all six TP2 declarations. The default fixture selects the
NPC and a required portrait alternative. `review-tp2` flags display-version
drift because the exact v1.7a tag's TP2 declares VERSION 1.7; these are
distinct facts, not evidence of a mismatched artifact. Other portrait choices,
crossbow proficiency, and the original soundset remain Untested.
