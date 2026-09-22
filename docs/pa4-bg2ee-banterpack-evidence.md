# PA-4 Banter Pack BG2EE evidence

On 2026-09-22, IEPM built all four Banter Pack v18 English components in
[`examples/bg2ee-banterpack/modpack.yaml`](../examples/bg2ee-banterpack/modpack.yaml)
into a clean disposable BG2EE 2.6.6 workspace and sealed it. The portable
identity and status assertions are in
[`evidence/pa4-bg2ee-banterpack.json`](../evidence/pa4-bg2ee-banterpack.json).

- Registry revision `6f49521`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `a90eb3f01b3bb9ac606ba1234937afdaaeb2d94e7cb03727f1894309829090a8`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `49b898325d8bf1b0b8ca9c0c83a28f217ba983005167c0000d4a2c0cce49ae72`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English components `#0` through `#3`:
  SoA banters, SoA accelerator, ToB banters, and ToB accelerator.
- Windows UI smoke launched the exact sealed target's `Baldur.exe`, showed
  the BG2EE 2.6.6 title selection, chose Shadows of Amn, and reached its
  Single Player / Multiplayer / Options menu. No new-game, save, or dialogue
  behavior smoke was performed.

These exact BG2EE English selections are Supported, not Verified. EET,
non-English choices, interactions with other banter-changing mods, and actual
in-game banter frequency/content remain Untested. Raw WeiDU output and the
full sealed game remain in the user's local IEPM store, not Git.
