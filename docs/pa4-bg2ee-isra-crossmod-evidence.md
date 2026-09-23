# PA-4 Crossmod Banter Pack installer evidence

On 2026-09-23, IEPM resolved
[`examples/bg2ee-isra-crossmod/modpack.yaml`](../examples/bg2ee-isra-crossmod/modpack.yaml)
and installed Isra NPC for BGII v3.1 followed by all three English Crossmod
Banter Pack v30 selectors into a clean disposable BG2EE 2.6.6 workspace.
The scoped status assertion is
[`evidence/pa4-bg2ee-isra-crossmod.json`](../evidence/pa4-bg2ee-isra-crossmod.json).

- Registry revision `d2b9923`; clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; lockfile SHA-256
  `dce106d0625702bc997cdc655371336949fc4c1915dc6e25dd64ba3145efe64d`.
- Exact Crossmod author-tag ZIP SHA-256
  `aa8578bbfcba7fc82aed971a9eb667f952da6d7552b921debd5397bfdd63a824`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `3e108f3da457cdcd02fec30ccf828c8c49d3e741fcfa5ac3f756e72666c7619c`.
  Both stderr logs were empty; Crossmod stdout had no `WARNING:` or `ERROR:`
  lines.
- Sealed `WeiDU.log` recorded Isra `#0` before Crossmod `#0`, `#1`, `#2`.
  No requested component was skipped.
- The author says Crossmod needs at least two already-installed covered
  partners to add new dialogue. This fixture has only Isra, so the receipt
  **does not** demonstrate that a banter or romance-conflict patch was made.
  A two-partner content fixture and in-game smoke remain.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), this route remains
**Untested**. Raw logs and the sealed game remain in the user's local IEPM
store, not Git.
