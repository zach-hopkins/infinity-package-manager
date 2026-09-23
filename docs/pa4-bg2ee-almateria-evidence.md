# PA-4 Almateria's Restoration Project installation evidence

On 2026-09-23, IEPM resolved
[`examples/bg2ee-almateria-restoration/modpack.yaml`](../examples/bg2ee-almateria-restoration/modpack.yaml)
and installed all twelve active BG2EE-facing English selectors of
Almateria's Restoration Project v10.1 into a clean disposable BG2EE 2.6.6
workspace. The scoped status assertion is
[`evidence/pa4-bg2ee-almateria.json`](../evidence/pa4-bg2ee-almateria.json).

- Registry revision `4b827d8`; clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; lockfile SHA-256
  `e11c3ad895ee7ff71649bdeb4d957f886c392524511e02b3c64fcace5cbecb58`.
- Exact maintainer-tag ZIP SHA-256
  `97ca579bac66c807b4628938ba7b809c81d5513eb8432e10c6f4705ed6dbf842`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `01e47c1720976e25ee272cfafd7a170878b003d77af4f2d985a4098402863324`.
  Installer stderr was empty; stdout had no `WARNING:` or `ERROR:` lines.
- Sealed `WeiDU.log` recorded language index `#1` (English) for components
  `#0`–`#8` and `#11`–`#13`. No requested component was skipped. Selector
  `#9` is deprecated in the TP2 and `#10` explicitly rejects BG2EE/EET.
- The Windows UI helper remained unavailable after its prior failed recovery.
  No game menu, new-game, known-save, gameplay, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), this exact route
remains **Untested** until the game menu is directly observed. Raw logs and
the sealed game remain in the user's local IEPM store, not Git.
