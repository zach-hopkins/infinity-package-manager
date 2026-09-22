# PA-4 NPC Flirt Packs BG2EE evidence

On 2026-09-22, IEPM built the eight base English choices (SoA and ToB for
Aerie, Jaheira, Viconia, and Anomen) in
[`examples/bg2ee-npcflirt/modpack.yaml`](../examples/bg2ee-npcflirt/modpack.yaml)
into a clean disposable BG2EE 2.6.6 workspace and sealed it. The exact
identity and status assertions are in
[`evidence/pa4-bg2ee-npcflirt.json`](../evidence/pa4-bg2ee-npcflirt.json).

- Registry revision `08e424e`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `1ee83cb9506779b4156123797c6ecbff129074ed1f279efe0e78d0678148b4ac`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `dc38bab71e7c411105eca6d903b0ab6035d4c94e5d574256113ac95e8d91e8e9`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English components `#0`, `#2`, `#4`, `#6`,
  `#8`, `#10`, `#12`, and `#14`.
- The exact sealed game launched from its own folder in Windows, showed
  BG2EE 2.6.6 title selection, and reached the Shadows of Amn Single Player
  menu. A direct executable launch with a different working-directory context
  had shown a game crash dialog first; this did not recur when launched from
  the game folder. The cause of that launcher-context difference was not
  established. No new-game, save, or dialogue behavior smoke was performed.

These eight exact BG2EE English choices are Supported, not Verified. The
eight optional Solaufein interaction choices, EET, non-English choices,
other romance mods, and actual in-game flirts remain Untested. Raw WeiDU
output and the full sealed game remain in the user's local IEPM store, not Git.
