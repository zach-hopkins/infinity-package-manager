# PA-4 High Quality Soundclips BG2EE evidence

On 2026-09-22, IEPM built the exact v1.3 author-tagged release in
[`examples/bg2ee-hq-soundclips/modpack.yaml`](../examples/bg2ee-hq-soundclips/modpack.yaml)
into a clean disposable BG2EE 2.6.6 workspace and sealed it. The portable
identity and status checks are in
[`evidence/pa4-bg2ee-hq-soundclips.json`](../evidence/pa4-bg2ee-hq-soundclips.json).

- Registry revision `2f765b6`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `f6c3ef774d3679a191f76b0c7d9c9ad8c5e35c89cf721a8ccae06d0960694d7a`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `3ec3c90d5781bc5293847f8b8bbd6033416d796878c8dbae7b750423f5786fea`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English component `#0`, high quality
  soundclips for new BG2EE content.
- Windows UI smoke launched the exact sealed target's `Baldur.exe`, showed
  the BG2EE 2.6.6 title selection, chose Shadows of Amn, and reached its
  Single Player / Multiplayer / Options menu. No new-game, save, gameplay,
  or subjective audio-quality check was performed.

The exact BG2EE English default is Supported, not Verified. EET, non-English
WeiDU language choices, other mod combinations, and audible output remain
Untested. Raw WeiDU output and the full sealed game remain in the user's local
IEPM store, not Git.
