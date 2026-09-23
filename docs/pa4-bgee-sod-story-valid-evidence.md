# PA-4 SoD story selection installation evidence

On 2026-09-23, IEPM resolved
[`examples/bgee-sod-story-comparison/modpack.yaml`](../examples/bgee-sod-story-comparison/modpack.yaml)
and installed four compatible Reflections of Destiny 0.9.4 selectors plus
all eight active Road to Discovery 6.0 selectors into a clean disposable
SoD-enabled BGEE 2.6.6 workspace. Reflections' broken selector 230 and
author-documented Road-incompatible Caelar rewrite 200 were deliberately
excluded. The scoped status assertion is
[`evidence/pa4-bgee-sod-story-valid.json`](../evidence/pa4-bgee-sod-story-valid.json).

- Registry revision `b94a125`; clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; lockfile SHA-256
  `e13b060fe09da82e0ab0d6a7fd03fa376d13801affa4108ad8b4bcfd36de13e7`.
- Reflections author-tag ZIP SHA-256
  `4ef856cdd4733ad3a2ffb1f6d136b4d3536091fa6bb333d839ba9bbdc8b060a5`;
  Road to Discovery author-tag ZIP SHA-256
  `f331c3d4da6d55dc741687cce0c03e806e36c3268295c4de29a84c3aae6fdcbf`.
- DLC Merger first merged SoD. Run receipt `completed`, 3/3 WeiDU actions,
  sealed output fingerprint
  `73bd36069f17be225ed4367c9bf9b3aa772d4dcd6a36720cd125613b95fb1a4a`.
  All three stderr logs were empty.
- Sealed `WeiDU.log` recorded Reflections `#100`, `#110`, `#220`, `#240`,
  then Road to Discovery `#0`, `#10`, `#20`, `#30`, `#40`, `#50`, `#60`, `#70`.
  No selected component was skipped.
- Reflections' stdout contains four nonfatal patch-search warnings for
  Imoen/SoD script patterns that were not found. The action completed and
  every selected component was logged; the warnings are retained for
  behavioral review, not silently treated as proof of gameplay correctness.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), this exact
installation remains **Untested** until the game menu is directly observed.
Raw logs and the sealed game remain in the user's local IEPM store, not Git.
