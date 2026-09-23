# PA-4 SoD-enabled BGEE scene and encounter installation evidence

On 2026-09-23, IEPM resolved
[`examples/bgee-sod-bridge-encounters/modpack.yaml`](../examples/bgee-sod-bridge-encounters/modpack.yaml)
and installed all four English Boareskyr Bridge Scene v8 components and all
eight English Extra Expanded Enhanced Encounters! 4.4 components into a
clean disposable BGEE 2.6.6 workspace with SoD. The scoped status assertion
is in [`evidence/pa4-bgee-sod-bridge-encounters.json`](../evidence/pa4-bgee-sod-bridge-encounters.json).

- Registry revision `0e00c8f`; clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `0aa9fe0a3a1a901b3b5ffcc5697f0036a5b27b185652027fb6bdf34491ef0cd3`.
- Exact author-tag ZIP SHA-256s: Boareskyr Bridge Scene v8
  `f6d5b37ed4e3cb08aaac13ec716c8320e71e3697d0fbd65166b0796e9949b164`;
  Extra Expanded Enhanced Encounters! 4.4
  `967ef5ffad7b097d3d93812f94ffd8bd22f3ff9c55b485928a85f7eebbdac2d4`.
- DLC Merger 2.1 first merged the source's SoD DLC. Run receipt `completed`,
  3/3 WeiDU actions, sealed output fingerprint
  `7e3263dae357d4e70b2bf7e842197e28f1830c1771d0e6a8d4acee7a7760daa1`.
  All three installer stderr logs were empty.
- Sealed `WeiDU.log` recorded English DLC Merger `#1`, Boareskyr `#0`–`#3`,
  then Extra Expanded Enhanced Encounters `#0`–`#7`. No requested component
  was skipped; the SoD-dependent cave encounter `#3` installed.
- The Windows UI helper had failed its recovery on the preceding build and
  was not used for this one. No game menu, new-game, known-save, gameplay, or
  EET smoke was performed. An unperformed menu smoke is not a game-start failure.

Under [`verification-policy.md`](verification-policy.md), these exact BGEE
selections remain **Untested** until the game menu is directly observed.
Full builds and raw WeiDU logs remain in the user's local IEPM store, not Git.
