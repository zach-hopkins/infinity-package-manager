# PA-4 BG Mini Quests and Encounters installation evidence

On 2026-09-23, IEPM exercised all eighteen English components from the
exact v31 author-tag ZIP using the
[BGEE fixture](../examples/bgee-bg-mini-quests/modpack.yaml). The
[scoped assertion](../evidence/pa4-bgee-bg-mini-quests.json) remains Untested
for game behavior until an observed menu smoke.

The first disposable run used the clean Steam BGEE source *without* DLC
Merger. Every component refused to install because that source also includes
unmerged SoD. WeiDU reported: “Your SoD game needs to be modmerged before
mods can be installed on this game.” IEPM did not seal this failed build.
This is a source-preparation condition, not a BGQE incompatibility. The
fixture was revised to install DLC Merger first and run in a **new**
disposable workspace.

- Successful registry revision `ad92290`, WeiDU 25100, BGQE artifact SHA-256
  `275e82396aed28d944d80ea7ac05ca48c2705beff0621f85ba1caaa536216f6f`.
- Clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  lock SHA-256 `a2518d566d4068407ba44f0d1477c90afe223c87cb0adf92190d8fd4ba855dc0`.
  DLC Merger first merged SoD; 2/2 actions completed. Sealed fingerprint
  `708392fbb35c8fabd5a370b8a9ae6ba043bae5803a406b2c36802455819d8f95`.
- WeiDU.log recorded English BGQE components `#0` through `#17`; none
  were skipped. The successful action had no warning or error markers.
- The Windows UI helper remained unavailable. No game menu, new-game,
  known-save, gameplay, or EET smoke was performed.

Both attempts' raw logs and the successful sealed game remain in the user's
local IEPM store, not Git.
