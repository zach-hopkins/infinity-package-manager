# PA-4 Endless BG1 installation evidence

On 2026-09-23, IEPM resolved
[`examples/bgee-endless-bg1/modpack.yaml`](../examples/bgee-endless-bg1/modpack.yaml)
and installed sixteen English Endless BG1 20.2 selectors into a clean
disposable SoD-enabled BGEE 2.6.6 workspace. The jastey Elminster variant
was selected; its mutually exclusive restored-text variant was not. The
scoped status assertion is
[`evidence/pa4-bgee-endless-bg1.json`](../evidence/pa4-bgee-endless-bg1.json).

- Registry revision `1739280`; clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; lockfile SHA-256
  `dc454c43d937f652eacba96eb332b3e944cbcb2bee991b2ce9ca724223aff4ec`.
- Exact author-tag ZIP SHA-256
  `2623f4d43a5178acc315c3688cac721877fb8cced9d3fddddc9febd61a6233f4`.
- DLC Merger first merged SoD. Run receipt `completed`, 2/2 WeiDU actions,
  sealed output fingerprint
  `6161a8168fa68143feac6b05a41eb460711705cca17bc105a8b5b24ed4633787`.
  Both stderr logs were empty; Endless stdout had no `WARNING:` or `ERROR:`
  lines.
- Sealed `WeiDU.log` recorded English Endless `#0`–`#8` and `#10`–`#16`.
  No selected component was skipped. Selector `#9` is the unselected
  Elminster alternative. The TP2's two embedded dialogue `BEGIN` lines are
  not components.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), this exact route
remains **Untested** until the game menu is directly observed. Raw logs and
the sealed game remain in the user's local IEPM store, not Git.
