# PA-4 Hidden Adventures installation evidence

On 2026-09-23, IEPM resolved
[`examples/bg2ee-hidden-adventures/modpack.yaml`](../examples/bg2ee-hidden-adventures/modpack.yaml)
and installed all ten BG2EE-facing English selectors of Hidden Adventures
Beta_9 into a clean disposable BG2EE 2.6.6 workspace. The scoped status
assertion is
[`evidence/pa4-bg2ee-hidden-adventures.json`](../evidence/pa4-bg2ee-hidden-adventures.json).

- Registry revision `4041abe`; clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; lockfile SHA-256
  `dce2cc33c9124e8a2d70d0fd10f9abe1b30d8ba9b01a2dc7212dda9c0d778b6b`.
- Exact author-tag ZIP SHA-256
  `006f878be9b89ffa3c2c8e0b6d2922c49de79a05c757306433c8a3f83b491613`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `0de5f333ffb6ccf7cc1505d4771caec94354dddebe6b1bbddf6c217c697a4d3a`.
  Installer stderr was empty.
- Sealed `WeiDU.log` recorded language index `#1` (English) for `#0`–`#4`
  and `#6`–`#10`. No requested component was skipped. Selector `#5` is
  BGT/EET-only and was intentionally not selected on BG2EE.
- Installer stdout had one nonfatal `EXTEND_TOP #position 1 out of range 0-1`
  warning. The action completed and all selected components were recorded;
  this warning is preserved for behavioral review, not treated as a fatal
  error or proof of gameplay correctness.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), this exact route
remains **Untested** until the game menu is directly observed. Raw logs and
the sealed game remain in the user's local IEPM store, not Git.
