# PA-4 BGEE NPC friendship evidence

On 2026-09-22, IEPM built the three exact releases in
[`examples/bgee-npc-friendships/modpack.yaml`](../examples/bgee-npc-friendships/modpack.yaml)
into a clean disposable BGEE 2.6.6 workspace and sealed it. The portable
identity and status checks are in
[`evidence/pa4-bgee-npc-friendships.json`](../evidence/pa4-bgee-npc-friendships.json).

- Registry revision `24aa3ce`; clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `135d2bd1b944b14d2d9768bdf68c67a4d89dc725e31e74c0ce649dd0b2544a4e`.
- The run receipt says `completed`, 3/3 actions, with sealed output fingerprint
  `def27b2a9fe84d1479784c6318661f807a63e54e84a3c205e571b6d6a14be3d6`.
  All three action stderr logs were empty, and no requested component was
  skipped. Unverified classification notices were not WeiDU failures.
- Sealed `WeiDU.log` recorded Coran BG Friendship, Sirene NPC, and Xan BG1
  Friendship as English component `#0` in the planned order.
- Windows UI smoke launched the exact sealed target's `Baldur.exe` and reached
  the BGEE 2.6.6 Single Player / Multiplayer / Options main menu. The game
  was then exited through its own Quit Game control. No new-game, save, or
  gameplay smoke was performed.

These exact BGEE default choices are Supported, not Verified. Sirene's nine
optional portrait/class choices, EET source-phase placement, other languages,
gameplay behavior, and other combinations remain Untested. Raw WeiDU outputs
and the sealed game remain in the user's local IEPM store, not Git.
