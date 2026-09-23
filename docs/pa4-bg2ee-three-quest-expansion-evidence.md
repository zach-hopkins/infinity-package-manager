# PA-4 three BG2EE quest releases: installation evidence

On 2026-09-23, IEPM resolved
[`examples/bg2ee-three-quest-expansion/modpack.yaml`](../examples/bg2ee-three-quest-expansion/modpack.yaml)
and installed the English default components of CoM Encounters, Test Your
Mettle!, and Tower of Deception into a clean disposable BG2EE 2.6.6
workspace. The scoped status assertion is in
[`evidence/pa4-bg2ee-three-quest-expansion.json`](../evidence/pa4-bg2ee-three-quest-expansion.json).

- Registry revision `289dd78`; clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `a8a424edc395f7efc32212b328ac0c028756d068dca9b1edd70a5479f1a90d41`.
- Exact author-tag ZIP SHA-256s: CoM Encounters v1.22
  `ce2f050867869d7776c5365289b160173b606830e6322e4993952812caf9d7dd`;
  Test Your Mettle! v1.6
  `ef982be26bea827ac1269a454b7e8019e21ea751de4cdbfd5dd2ca75c5e91559`;
  Tower of Deception v4.1.0
  `2b89f6418d77e76ac58f352c7fa1bdd00111cd20bc2cced8c82ee1510efcc28d`.
- Run receipt `completed`, 3/3 WeiDU actions, sealed output fingerprint
  `684d31582e51c81201de5d63c5ee78288b8bac7f508c5907a6e667fcc91f2352`.
  All installer stderr logs were empty; no requested component was skipped.
- Sealed `WeiDU.log` recorded English CoM Encounters `#0`, Test Your Mettle!
  `#0`, then Tower of Deception `#0`.
- The sealed `Baldur.exe` process started and had a main window handle, but
  the Windows UI helper failed to start inside its sandbox before it could
  observe the menu. This is an **unperformed menu smoke**, not a passed or
  failed game-start result. The disposable process was closed. No new-game,
  known-save, gameplay, optional-selector, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), these three exact
main routes remain **Untested** until the game menu is directly observed.
Full builds and raw WeiDU logs remain in the user's local IEPM store, not Git.
