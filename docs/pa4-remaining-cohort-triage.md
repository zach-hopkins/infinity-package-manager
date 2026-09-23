# PA-4 remaining cohort triage (2026-09-23)

The first 99-package Product A cohort has 56 curated executable identities
and **24 discovery-only candidates** remaining beyond the 19 packages
already curated when the cohort was frozen. This page is an acquisition and
review queue, not a support-status upgrade. The source snapshot is the
[frozen cohort](../registry/cohorts/product-a-cohort.json), with acquisition
leads in the [PA-2 health report](../registry/catalog/infinity-mod-forge-health.json)
and exact tagged-ZIP observations in the
[PA-4 artifact report](../registry/cohorts/pa4-tagged-artifacts.json).

## Five exact tagged archives already acquired

| Source ID | Candidate | Next concrete step |
| ---: | --- | --- |
| 18 | EE UI Tweaks | Review 67 TP2 selectors, UI framework interactions, and a bounded BG2EE install. |
| 24 | BG:EE Classic Movies | [Contain writes outside the disposable game](pa4-classic-movies-external-write-blocker.md) before any install. Eight selectors are mechanically known. |
| 86 | Romantic Encounters (BG2) | Review 55 numeric-only selectors and file predicates (notably `mel01.cre`) before stabilizing IDs or selecting a fixture. No LABELs were observed. |
| 246 | EET Tweaks | Review 65 selectors and EET-specific placement; then test on a disposable EET target. |
| 428 | BP-BGT Worldmap | [Model its interactive map-size choice](pa4-worldmap-interactive-blocker.md) before declaring the main component executable. Six selectors are mechanically known. |

The exact tagged ZIPs above were SHA-256 hashed; a PA-2 ZIP signature alone
would not be sufficient. The Classic Movies and Worldmap blockers are IEPM
execution-boundary gaps, **not** claims that those mods are incompatible.

## Six branch-archive acquisition leads

These are not release identities. A mutable branch URL can change without
changing its spelling, so pin an exact commit or released asset before
deriving selectors and updating the executable registry.

| Source ID | Candidate | PA-2 observation |
| ---: | --- | --- |
| 25 | Baldur's Gate Graphics Overhaul | Branch archive probe returned HTTP error. |
| 29 | Made in Heaven: Fixes & Restorations | Branch archive had ZIP signature; still mutable. |
| 134 | Recorder | Branch archive probe returned HTTP error. |
| 175 | Sirene NPC (BG2) | Branch archive probe returned HTTP error. |
| 470 | Infinity Sounds | Branch archive had ZIP signature; still mutable. |
| 473 | Northern Tales of the Sword Coast | Branch archive probe returned HTTP error. |

## Thirteen manual-browser acquisition leads

PA-2 did not establish a direct artifact route for source IDs **13** Revert
Pathfinding, **64** Quest Pack, **87–91** the five
Colours of Infinity entries (Tales of the Deep Gardens, Innershade, The
White Queen, I Shall Never Forget, Eilistraee's Song), **92–96** the five
Athkatlan Grounds entries (Southern Edge, The Ooze's Lounge, The Tangled
Oak Isle, Bridge's Block, Alabaster Sands), and **474** Portraits Portraits
Everywhere.

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

For each, locate an author or maintained release page, fetch exact bytes,
hash and safely inspect the archive, and only then decide whether it can
become a curated executable record. An unknown or unverified package may
still follow the opaque local-package path when the user supplies bytes;
absence from this registry is not a hard incompatibility.
