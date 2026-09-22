# PA-4 tagged cohort artifact inventory

The frozen 99-package cohort initially contained 80 discovery-only candidates.
The PA-2 health snapshot proposed 56 tagged GitHub source archives, seven
branch snapshots, and 17 manual-browser routes among those 80. A signature
probe was not counted as an acquired artifact.

The [tagged artifact inventory](../registry/cohorts/pa4-tagged-artifacts.json)
records exact downloaded bytes for all 56 proposed tag archives. Every archive
was hashed with SHA-256 and passed IEPM's inert ZIP/TP2 inspection. The report
retains the candidate URL, size, archive hash, TP2 paths, language declarations,
and raw component selectors. It does not promote the 56 to Supported or even
to executable releases: source-tag archives can omit generated assets or have
installer-specific behavior, and game targets/order still need review.

Sirene, LeUI, ten single-component Spellhold Studios friendship mods, four
single-component Pocket Plane quest mods, three further BG2EE friendships,
Korgan's Redemption, Coran's BG Friendship, Xan's BG1 Friendship, High
Quality Soundclips, and Banter Pack now have
reviewed, executable registry routes. The first ten friendship defaults and
four quest defaults have exact-scope BG2EE
build and main-menu evidence. The four newest routes also have revision-pinned
clean disposable build and main-menu evidence in
[`pa4-bg2ee-friendship-expansion-evidence.md`](pa4-bg2ee-friendship-expansion-evidence.md).
The Coran, Sirene, and Xan English defaults also have a pinned clean BGEE
build and main-menu smoke in
[`pa4-bgee-npc-friendships-evidence.md`](pa4-bgee-npc-friendships-evidence.md).
All three LeUI English choices also completed in a separate pinned BG2EE
build and reached the Shadows of Amn menu, recorded in
[`pa4-bg2ee-leui-evidence.md`](pa4-bg2ee-leui-evidence.md). EET routes and
Sirene's optional selectors remain Untested. The remaining 32
tagged archives are exact-byte review inputs, not a promise that IEPM can
install them. The seven branch
routes and 17 manual routes remain separate acquisition work; a mutable branch
must first resolve to an exact commit and may never be presented as an author
release solely on the basis of a ZIP signature.

The pass exposed real TP2-reader edge cases: author files use both indented
component declarations and indented action `BEGIN` blocks, sometimes with
`ACTION_IF` followed by `THEN BEGIN`. The reader now requires a component title
argument and respects tracked action branches. These cases have regression
tests. No TP2 was executed during this inventory.

To regenerate the report from already-cached archives (or fetch any missing
archive), build the CLI and run:

```powershell
cargo build -p iepm
& scripts/pa4-acquire-cohort.ps1 -Skip 0 -Limit 56 -ReportPath registry/cohorts/pa4-tagged-artifacts.json
```

Downloaded ZIPs live under ignored `target/pa4-artifacts/cohort/`. They are
not checked into Git; the tracked report preserves their exact hashes and
derived observations. Removing the local ZIPs does not erase the evidence
record, but re-acquisition must recheck the pinned hash before reuse.
