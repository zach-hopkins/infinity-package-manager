# PA-4 LeUI BG2EE evidence

On 2026-09-22, IEPM built all three LeUI v4.9.1 components in
[`examples/bg2ee-leui/modpack.yaml`](../examples/bg2ee-leui/modpack.yaml)
into a clean disposable BG2EE 2.6.6 workspace and sealed it. The exact
identity and status assertions are in
[`evidence/pa4-bg2ee-leui.json`](../evidence/pa4-bg2ee-leui.json).

- Registry revision `04eda3a`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `7f2a6a0d559e710de0ba256b90a0e9b7291c477798991acced4358854da3c2c3`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `80914a48472a58fcd6b939e942105f02ec4b4cc9c92d4ab5a31be5402111d8aa`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English components `#0`, `#1`, and `#2`:
  core UI, BG2 vanilla spell icons, and BG2 vanilla description fonts.
- Windows UI smoke launched the exact sealed target's `Baldur.exe`, showed
  the BG2EE 2.6.6 title selection, selected Shadows of Amn, and reached its
  Single Player / Multiplayer / Options menu with the LeUI layout visible.
  No new-game, save, or gameplay smoke was performed.

These exact BG2EE component choices are Supported, not Verified. BGEE, SoD,
EET, combinations with other UI overhauls or SCS, and gameplay behavior
remain Untested. Raw WeiDU outputs and the full sealed game remain in the
user's local IEPM store, not Git.
