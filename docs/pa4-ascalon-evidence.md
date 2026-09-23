# PA-4 Ascalon's Questpack BGEE and BG2EE installation evidence

On 2026-09-23, IEPM installed the exact Ascalon's Questpack 7.0 author-tag
ZIP in two independent disposable game roots. The BGEE route selected one
option from each of the three variant groups plus the six other BG1 quests;
the BG2EE route selected the sole BG2-facing quest. Manifests:
[`BGEE`](../examples/bgee-ascalon-questpack/modpack.yaml) and
[`BG2EE`](../examples/bg2ee-ascalon-questpack/modpack.yaml). Scoped status
assertions: [`BGEE`](../evidence/pa4-bgee-ascalon.json) and
[`BG2EE`](../evidence/pa4-bg2ee-ascalon.json).

- Both used registry revision `502a95c`, WeiDU 25100, and exact ZIP SHA-256
  `b6b0ca11925f782afd8db8f7207b63a4160db4a649e245bf606e301cd93af118`.
- BGEE clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  lock SHA-256 `ea3e494ea9bb6fea1eb29edcaa8ce136a287b8ce23bb620ca8a592b6ec9baa38`.
  DLC Merger first merged SoD; 2/2 actions completed. Sealed fingerprint
  `f8e0f075c3b387bda8cc460edf0a6f24d960f9f433bc714db408df75dea66117`.
  WeiDU.log recorded English `#0`, `#1`, `#2`, `#4`, `#5`, `#7`, `#8`, `#9`, `#10`.
- BG2EE clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  lock SHA-256 `a8d24effaeb7a710ff1f4ff6cdce14bbe7d067ee20ca8b13ded54dffe2840f11`.
  1/1 action completed, sealed fingerprint
  `9413e78993cc119b8cc31affc030642cb59d8ea26e3bad8ec0346879fb54838d`.
  WeiDU.log recorded English component `#12`.
- All installer stderr logs were empty; no selected component was skipped.
  The alternative BG1 choices `#3`, `#6`, and `#11` were not executed.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), both exact
configurations remain **Untested** until their game menus are observed. Raw
logs and sealed games remain in the user's local IEPM store, not Git.
