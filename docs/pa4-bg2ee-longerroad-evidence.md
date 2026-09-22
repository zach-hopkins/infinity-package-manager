# PA-4 The Longer Road v2.0.7 BG2EE install evidence

On 2026-09-22, IEPM resolved
[`examples/bg2ee-longerroad/modpack.yaml`](../examples/bg2ee-longerroad/modpack.yaml)
and automatically selected both Ascension components required by Longer Road's
main component. This is an installation result, **not** a Supported status.

- Registry revision `ac5930e`; clean BG2EE 2.6.6 source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `fbb0e8185630e32a387d098d47753f26bca203847201d4052019607a74163175`.
- The SHA-verified v2.0.7 author-tag ZIP was
  `5e46d529574121d7b8712a7f427fcc4f6f5d44b9da6294ba61c81f5569ea34e6`;
  pinned Ascension 2.1.0 was
  `bd0ee14b3770d56104eaa81287c88a8672afa889a35184fb308be12d6d707e77`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `c81fb08f1e1c5eb1391c942898547037bfa2c5649f73caa33c4bf579f0f010ab`.
  Both installer stderr logs were empty; no requested component was skipped.
- Sealed `WeiDU.log` recorded English Ascension `#0` and `#10`, then Longer
  Road `#0`.
- The sealed `Baldur.exe` process started but the UI inspector repeatedly
  reported a stale prior build window and did not produce an observable menu
  screen for this build. The process was closed without interacting with the
  game. Therefore `main_menu_smoke` is **not passed**. No new-game, known-save,
  Irenicus story, portrait, or EET smoke was performed.

Under [`verification-policy.md`](verification-policy.md), the exact English
BG2EE main route remains **Untested** until its menu is directly observed.
The full sealed game and raw WeiDU logs remain in the user's local IEPM store,
not Git.
