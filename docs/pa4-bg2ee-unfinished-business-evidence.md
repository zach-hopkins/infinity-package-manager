# PA-4 Unfinished Business BG2EE evidence

On 2026-09-22, IEPM built the three English restorations in
[`examples/bg2ee-unfinished-business/modpack.yaml`](../examples/bg2ee-unfinished-business/modpack.yaml)
into a clean disposable BG2EE 2.6.6 workspace and sealed it. The exact
identity and status assertions are in
[`evidence/pa4-bg2ee-unfinished-business.json`](../evidence/pa4-bg2ee-unfinished-business.json).

- Registry revision `b0c9d0f`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `eb19e4ab1efd95bf284190333213eeadd8b4f91d1cb47b58bdfc8ae32df9c927`.
- The SHA-verified author v28 archive was
  `953b849329626a50e8de5927170c927b1ade0641f409aa83be2a6b63fd0b0203`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `fbc3a123de667d67b994d256d21d815999bd9d1af1e651617c9a34304bc4f364`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English components `#0` (The Kidnapping of
  Boo), `#12` (Item Restorations), and `#18` (Restored Minor Dialogs).
- The exact sealed game launched from its own folder in Windows, showed
  BG2EE 2.6.6 title selection, and reached the Shadows of Amn Single Player
  menu. One click skipped the introductory movie. No new-game, save, quest,
  item, or dialogue behavior smoke was performed.

These three exact BG2EE English choices are Supported, not Verified. The
other 21 current-game choices are classified Untested. Two additional
main-TP2 declarations are explicitly deprecated for BG2EE and not exposed.
EET, other languages, other mods, and in-game behavior remain Untested. Raw
WeiDU output and the full sealed game remain in the user's local IEPM store,
not Git.
