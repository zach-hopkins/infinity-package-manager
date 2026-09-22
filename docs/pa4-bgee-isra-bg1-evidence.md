# PA-4 Isra NPC for BG v3.5 BGEE evidence

On 2026-09-22, IEPM resolved
[`examples/bgee-isra-bg1/modpack.yaml`](../examples/bgee-isra-bg1/modpack.yaml)
and installed DLC Merger followed by Isra's English main NPC component into
a clean disposable BGEE 2.6.6 workspace. The exact status assertion is in
[`evidence/pa4-bgee-isra-bg1.json`](../evidence/pa4-bgee-isra-bg1.json).

- Registry revision `1fd3090`; source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `cfa74116d3f3e41769d3efa9a649d0a3bbad7b8a5e01c61228688a19f8e6fcc3`.
- Exact v3.5 author-tag ZIP SHA-256
  `36c01fa48dd96b6aa4d04c19d4f81e4d519d66a6380bf0642c198fed8a2e129e`;
  pinned DLC Merger 2.1 ZIP SHA-256
  `16b57b32662c1ac1e6d345afaf8669e01cf26eeafc5461000757edf33447e2eb`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `deb603d6fc0bb6a8361c625f8c892c7584d79fd6e317ac1c95e790047f7ec74c`.
  Both installer stderr logs were empty; no requested component was skipped.
- Sealed `WeiDU.log` recorded English DLC Merger `#1` then Isra `#0`.
- The sealed game launched and reached the BGEE 2.6.6 main menu after the
  intro movie was skipped with a click. No new-game, known-save, NPC
  recruitment/story, Valerie/Gavin crossmod, or EET smoke was performed.

The exact English BGEE main-component route is **Supported**, not Verified.
Optional crossmod components remain Untested until their exact partner mods
are installed and the corresponding interactions are exercised. The full
sealed game and raw WeiDU logs remain in the user's local IEPM store, not Git.
