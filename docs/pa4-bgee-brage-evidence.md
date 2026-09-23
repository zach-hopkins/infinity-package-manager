# PA-4 Brage's Redemption installation evidence

On 2026-09-23, IEPM installed the exact official Brage's Redemption v10.1
author-tag ZIP using the [SoD fixture](../examples/bgee-brages-redemption/modpack.yaml)
in a fresh disposable BGEE 2.6.6 build. The
[scoped assertion](../evidence/pa4-bgee-brage.json) remains Untested for game
behavior until an observed menu smoke.

- Registry revision `8663e0e`, WeiDU 25100, Brage artifact SHA-256
  `a9ee0dfa8a2819190cbcdec34e12537294cbf31830baca6067f2dda9c72bd1da`.
- Clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  lock SHA-256 `063b2fa8cfcb5bfc02311a3154a865ee8eb3659ce73e6ad0c734115075004eb2`.
  DLC Merger first merged SoD; BGQE then installed Brage's Sword `#11`;
  Brage followed. 3/3 actions completed. Sealed fingerprint
  `3a6abad32562a14997beb1f0b43283f6cee0efbe8118ea7477f5fbacd267eb7d`.
- WeiDU.log recorded all four English Brage selectors `#0`, `#1`, `#5`,
  and `#10`. All three stderr logs were empty; no skipped requests or
  warning/error markers appeared in successful stdout logs.
- This verifies the narrow component prerequisite and installation order,
  not actual NPC behavior, crossmod dialogue with other partner mods, BG2EE,
  or EET. The Windows UI helper was unavailable, so no menu, new-game, or
  known-save smoke was performed.

On 2026-09-23, a separate [BG2EE fixture](../examples/bg2ee-brages-redemption/modpack.yaml)
tested the TP2's exception to the BGQE prerequisite. Registry revision
`a0caf3d`, the same Brage artifact and WeiDU 25100, clean BG2EE fingerprint
`298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
IEPM resolved **without** BGQE. Main `#0` and crossmod `#10` completed 1/1
action, appeared in WeiDU.log, and sealed with fingerprint
`bf3729453b1f863cd4f2fb66ebb90635d2fd05d16c02c22c3f6907e7cfb8665b`.
The portable lock SHA-256 was
`a2dec4ff82ff6dcfba2bfdd13d7c295a86f67cb47355a606137236a618f788ac`.
Stderr was empty with no warning/error markers. The
[BG2EE scoped assertion](../evidence/pa4-bg2ee-brage.json) remains Untested:
the author says BG2 content is unfinished, and no menu/gameplay smoke was
observed.

Raw logs and both sealed games remain in the user's local IEPM store, not Git.
