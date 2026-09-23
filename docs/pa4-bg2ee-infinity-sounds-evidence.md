# PA-4 Infinity Sounds installation evidence

On 2026-09-23, IEPM installed the exact author v2.2 tag ZIP using the
[BG2EE fixture](../examples/bg2ee-infinity-sounds/modpack.yaml) in a fresh
disposable BG2EE 2.6.6 workspace. The archive SHA-256 was
`f6a2b462a9f309fe2b12362696726d6a79f5395d1945ee9a40507a7200c6c953`
(35,133,232 bytes). `review-tp2` matched all 20 live selectors with none
unmapped. The commented-out selector 260 was correctly excluded.

- Registry revision `0bf0773`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `13969af6713687e5090d2a2bc21c85ae5766947f47c3045895fa17a7b5bac90e`;
  the five selected sound-restoration components completed. Final sealed
  fingerprint `e6bb2a14967e2efd7c3e9ad0d3ccbeeecb76233f11d4025dad2f799f6db0aabd`.
- WeiDU.log records selectors `#100`, `#110`, `#120`, `#130`, and `#140`.
  Stderr was empty; stdout records all five as installed with no warning or
  skipped-component marker.

Selectors `#189` and `#221` can write to `USER_DIRECTORY/Baldur.lua` outside
the disposable workspace and were **not** executed. They remain blocked by
IEPM's execution boundary, not by a finding of mod incompatibility. Selectors
`#150` and `#220` require original ToB and are inapplicable to the EE targets.
The [scoped assertion](../evidence/pa4-bg2ee-infinity-sounds.json) is
**Untested** because menu, gameplay, and actual audio output have not been
observed. Raw logs and sealed game tree remain in the local IEPM store.
