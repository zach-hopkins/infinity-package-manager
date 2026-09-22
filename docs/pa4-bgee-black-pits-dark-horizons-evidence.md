# PA-4 Black Pits in BG and Dark Horizons BGEE install evidence

On 2026-09-22, IEPM resolved
[`examples/bgee-black-pits-dark-horizons/modpack.yaml`](../examples/bgee-black-pits-dark-horizons/modpack.yaml)
and installed DLC Merger, Black Pits in BG, and Dark Horizons into a clean
disposable BGEE 2.6.6 workspace. The scoped status assertion is in
[`evidence/pa4-bgee-black-pits-dark-horizons.json`](../evidence/pa4-bgee-black-pits-dark-horizons.json).

- Registry revision `4bb2fec`; source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `285f2c473754df4bca3504df98fbec0e754bc89ccb31c8c6435d46b6234e1234`.
- Exact author-tag ZIP SHA-256s: Black Pits in BG 1.2.1
  `231929cf1df4009a0712a85a57ef6f4ff562b2ba0afb9e15c04918a9e085dab8`;
  Dark Horizons 3.06
  `977e7771c9717e3139e96d261279c7337fcfd062fb5ea6b0efdedddc2c8ee9c2`;
  DLC Merger 2.1
  `16b57b32662c1ac1e6d345afaf8669e01cf26eeafc5461000757edf33447e2eb`.
- Run receipt `completed`, 3/3 WeiDU actions, sealed output fingerprint
  `d65a2bc952ce324fc707dae615e412ee952aa49ad31c644821a3883f812ca130`.
  All installer stderr logs were empty; no requested component was skipped.
- Sealed `WeiDU.log` recorded English DLC Merger `#1`, Black Pits in BG
  `#100`, then Dark Horizons `#0`.
- The sealed BGEE process launched and showed its startup movie, but the
  Windows UI inspector subsequently twice reported the window as minimized
  and could not restore an observable main menu. This is an inconclusive
  **menu smoke**, not a game crash or a successful menu smoke. No new-game,
  known-save, quest, optional nerfs, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), both selected mod
components remain **Untested** until the sealed game reaches an observed menu.
The full build and raw WeiDU logs remain in the user's local IEPM store, not
Git.
