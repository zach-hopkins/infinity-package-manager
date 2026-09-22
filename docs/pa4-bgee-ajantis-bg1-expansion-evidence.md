# PA-4 Ajantis BG1 Expansion BGEE fixture

The first clean disposable BGEE 2.6.6 run of Ajantis BG1 Expansion v22
stopped before installing component 0. Its TP2 detected the source game's
unmerged SoD DLC and explicitly required DLC Merger. This is a known
prerequisite for this exact source layout, not evidence that Ajantis is
generally incompatible with BGEE. The failed workspace is not reusable.

The revised [fixture](../examples/bgee-ajantis-bg1-expansion/modpack.yaml)
selected DLC Merger's `merge-sod` before Ajantis's main component. Its
exact identity and status assertions are in
[`evidence/pa4-bgee-ajantis-bg1-expansion.json`](../evidence/pa4-bgee-ajantis-bg1-expansion.json).

- Registry revision `b71498f`; source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `5ad6cc6636a4f81c371380b82efa49a282760a1d8710519eec665de94fccf0a5`.
- The SHA-verified Ajantis author v22 archive was
  `7479c3b6d2ba8ab9acb509de3d6101ac862d8a3c7f6ec7a99e4e8defa7b60dbe`;
  the pinned DLC Merger 2.1 archive was
  `16b57b32662c1ac1e6d345afaf8669e01cf26eeafc5461000757edf33447e2eb`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `7a0f77e359b11999116e4d2b103fecc31c47f52ac20dbbd14f7449e84bcc9a6c`.
  Both action stderr logs were empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English DLC Merger `#1` and Ajantis
  Expansion `#0` in that order.
- The exact sealed game launched from its own folder in Windows and reached
  the BGEE 2.6.6 main menu. Intro movies were skipped by clicking. No
  new-game, save, Ajantis dialogue, romance, or SoD behavior smoke was run.

This exact English BGEE main-component route is Supported, not Verified.
Optional shield art (`#1`), SoD NPC crossmod content (`#30`), a BG1NPC
interaction, EET placement, other languages, and gameplay remain Untested.
The full sealed game and raw WeiDU logs remain in the user's local IEPM
store, not Git.
