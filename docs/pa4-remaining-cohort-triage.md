# PA-4 remaining cohort triage (2026-09-23)

The first 99-package Product A cohort now has 80 curated identities from
the frozen 80-candidate expansion, in addition to the 19 packages already
curated when the cohort was selected. **No cohort identity remains
discovery-only**, but two packages have headline components gated by known
execution-boundary gaps. This page is a scope and evidence ledger, not a
support-status upgrade. The source snapshot is the
[frozen cohort](../registry/cohorts/product-a-cohort.json), with acquisition
leads in the [PA-2 health report](../registry/catalog/infinity-mod-forge-health.json)
and exact tagged-ZIP observations in the
[PA-4 artifact report](../registry/cohorts/pa4-tagged-artifacts.json).

## Two partially executable packages

| Source ID | Candidate | Safe tested choice | Headline limitation |
| ---: | --- | --- | --- |
| 24 | BG:EE Classic Movies | Chapter/dream screens installed after DLC Merger and sealed | Seven movie choices remain gated until [writes outside the disposable game are contained](pa4-classic-movies-external-write-blocker.md). |
| 428 | BP-BGT Worldmap | Independent BG2-style larger-map UI installed and sealed | Main Worldmap and dependent ToB-map choices remain gated until [interactive answers are portable build inputs](pa4-worldmap-interactive-blocker.md). |

The exact tagged ZIPs above were SHA-256 hashed and all selectors mapped;
a PA-2 ZIP signature alone would not be sufficient. The Classic Movies and
Worldmap blockers are IEPM execution-boundary gaps, **not** claims that
those mods are incompatible or that their gated main features were tested.

Source ID **57**, The Lure of the Sirine's Call, now has an exact author-tagged
v16.5.2 ZIP, complete two-selector mapping, and clean BGEE main-only and
main-plus-lighthouse install receipts. Its menu/gameplay and EET routes are
still untested; the TP2 VERSION lags the release tag.

Source ID **140**, Sir Ajantis NPC for BGII, now has an exact author-tagged v21
ZIP, a complete nine-selector mapping, and a clean five-selection BG2EE
receipt. The other dialogue-speed alternatives, EET, and in-game behavior
remain untested.

Source ID **234**, SoD Dialog Banters, now has an exact v1.0 author tag,
complete one-selector mapping, and a clean BGEE/SoD install receipt. Its TP2
explicitly rejects installation into an already-EET game. No menu or banter
behavior smoke has been observed.

Source ID **64**, Quest Pack, now has the exact author v35 tag (TP2 VERSION
v3.5), all 23 live selectors mapped, and a clean four-choice BG2EE install
receipt. Three nonfatal dialogue-state WEIGHT warnings require a future
dialogue smoke before stronger support claims.

Source ID **175**, Sirene NPC for BG2EE, now has an exact author v2.02 tag and
all nine selectors classified. NPC plus True Paladin installed and sealed on
BG2EE. All four portrait choices are blocked for this exact artifact because
their TP2 COPY paths do not exist in the archive; the first portrait failed
in a disposable build. Other class choices, EET, and gameplay remain untested.

Source ID **473**, Northern Tales of the Sword Coast, now has an exact author
v5.0.0 tag, all 15 selectors mapped, and a clean BGEE/SoD main-component
install and seal. Optional selectors, EET, menu, and gameplay remain untested.

Source ID **470**, Infinity Sounds, now has an exact author v2.2 tag, all 20
selectors mapped, and a clean five-choice BG2EE install and seal. Two
`Baldur.lua`-writing choices remain gated by IEPM's external-write boundary;
two original-ToB-only choices are unavailable on EE games. Other choices,
menu, EET, and audio behavior remain untested.

Source ID **88**, Innershade, now has exact author-hosted v11.16 bytes and a
complete two-alternative selector map. The no-save-patching choice installed
and sealed on BG2EE with one nonfatal EXTEND_BOTTOM warning; the save-patching
choice is withheld pending containment of its possible external save writes.

Source ID **90**, I Shall Never Forget, now has exact author-hosted 6.5.6
bytes, its sole selector mapped, and a clean BG2EE install and seal. Menu,
quest behavior, and EET remain untested.

Source IDs **87**, **89**, **91–96** now have exact author-hosted archive
hashes and complete live component mappings. Eight separate clean BG2EE
fixtures installed and sealed their English main/no-save-patching choices
without WeiDU warning or skip markers. Their exact receipts are in the
[batch evidence](pa4-bg2ee-weasel-quest-batch-evidence.md). Existing-save
patching alternatives are withheld pending containment of possible writes
outside the disposable workspace. Menu, quest behavior, and EET remain
untested.

Source ID **134**, Recorder BG1, now has an exact author commit archive,
complete two-selector numeric mapping, and a BGEE/SoD main-NPC install and
seal. Three nonfatal EXTEND_TOP warnings require dialogue smoke. The optional
music choice, EET, menu, and gameplay remain untested.

Source ID **29**, Made in Heaven: Fixes & Restorations, now has an exact
author commit archive, all 20 selectors mapped, and a clean BG2EE EE Fixpack
core plus P&P/Intelligence build. Its P&P and Volo selectors require EE
Fixpack component 0; those selectors remain unavailable on EET until the
pre-import baseline is modeled. Other selectors, EET, menu, and gameplay
remain untested.

Source ID **25**, Baldur's Gate Graphical Overhaul, now has an exact official
v3.6 tag ZIP, four mapped selectors, and a clean BG2EE core install and seal.
The two Android-directory-dependent platform selectors are unavailable in
this Windows source route; the core auto-selects Windows graphics. Alternate
Graphics, EET, menu, and actual visual behavior remain untested.

Source ID **86**, Romantic Encounters BG2, now has an exact author v15 tag,
all 55 numeric-only selectors mapped to adjacent author scenario comments,
and a clean four-choice BG2EE install and seal. Three ToB-related scenarios
still depend on the `mel01.cre` resource predicate. Other scenarios, EET,
menu, and gameplay remain untested.

Source ID **474**, Portraits Portraits Everywhere, now has an immutable
upstream author commit archive, all eight selectors mapped, and a clean
BG2EE core-plus-sequenced-category install and seal. The author's README
v1.03 and TP2 VERSION 1.01 differ, so they are recorded separately. Other
selectors, BGEE/EET, menu, and actual portrait behavior remain untested.

Source ID **18**, EE UI Tweaks, now has an exact official v4.0.7 tag ZIP,
all 62 live selectors mapped and five deprecated dummy workarounds classified
as unavailable. Mods Options plus Hidden Game Options installed and sealed on
clean BG2EE. The TP2's UI-framework predicates remain WeiDU-enforced; other
selectors, framework combinations, EET, and visual behavior remain untested.

Source ID **246**, EET Tweaks, now has the exact v1.12 tag, all 65 selectors
mapped, and seven interactive custom-value choices gated. A generic XP-cap
choice installed cleanly on BG2EE; EET-only selectors still require an EET
fixture and are not verified by this BG2EE run.

Source ID **13**, Bubb Revert Pathfinding, now has exact v1.1 bytes from the
author's 2.6.6 forum attachment. Its one component installed on a disposable
BG2EE copy, and only one byte changed in the copied Baldur.exe. Actual
movement behavior and other executable variants remain untested.

Source ID **24**, BG:EE Classic Movies, now has exact official V2.4.1 bytes
and all eight selectors classified. The chapter/dream-screen choice installed
and sealed on BGEE after DLC Merger. Seven movie choices are explicitly
unavailable until IEPM isolates their user-settings writes; the safe choice
does not imply the main movie-restoration feature works unattended.

Source ID **428**, BP-BGT Worldmap, now has exact official v13.1.1 bytes and
all six selectors classified. Its independent BG2-style larger-map UI choice
installed and sealed on BG2EE. Main Worldmap still requires portable typed
answers for conditional installer prompts, and dependent ToB-map choices
are unavailable until that is solved.

Further work is stronger verification, EET-route tests, and the two scoped
execution gaps above—not acquisition of another candidate from this frozen
cohort. Unknown packages outside the cohort may still follow IEPM's opaque
local-package path when the user supplies bytes; absence from this registry
is not a hard incompatibility.
