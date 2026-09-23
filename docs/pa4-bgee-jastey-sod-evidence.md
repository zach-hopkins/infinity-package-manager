# PA-4 Jastey's SoD Tweakpack installation evidence

On 2026-09-23, IEPM installed the exact v13 author-tag ZIP from the
[SoD fixture](../examples/bgee-jasteys-sod-tweakpack/modpack.yaml) into a
fresh disposable BGEE 2.6.6 workspace. The
[scoped assertion](../evidence/pa4-bgee-jastey-sod.json) is Untested for game
behavior until an observed menu smoke.

- Registry revision `ed6d214`, WeiDU 25100, artifact SHA-256
  `069c1d06920fa992f7a6252a3d282df335c089a5fa81b7efede17b7bf2a04ef9`.
- Clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  lock SHA-256 `73bdc14f155dda0157413ea41fa6c3ccc406abc29042558bca3816e82ba37422`.
  DLC Merger merged SoD first; 2/2 actions completed. Sealed fingerprint
  `1bc9f19fd402bbae766398457fe26e3756c4ca26cb666af9f71497b8c0e00065`.
- WeiDU.log recorded English tweakpack selectors `#1`, `#3`, `#4`, `#5`,
  `#7`, `#9`, `#12`, `#20`, and `#21` with no skipped requests. Both installer
  stderr logs were empty, and stdout contained no warning/error markers.
- This does not validate alternative dialogue, XP, or ending choices; the
  EET-only portrait; interactions with other SoD transition mods; actual
  game behavior; or EET placement. The Windows UI helper was unavailable,
  so no game menu, new-game, or known-save smoke was performed.

Raw logs and the sealed game remain in the user's local IEPM store, not Git.
