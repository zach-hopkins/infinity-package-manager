# PA-4 BG2EE quest build evidence

On 2026-09-22, IEPM built four exact Pocket Plane Group source-tag releases
from `examples/bg2ee-quests/modpack.yaml` into a clean managed disposable
BG2EE 2.6.6 workspace, then sealed it. The portable identity and check
assertions are in [`evidence/pa4-bg2ee-quests.json`](../evidence/pa4-bg2ee-quests.json).

- Registry revision `af7081f`; source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `260eb0771e89426485fbef4454ea9c89e585d628feb9f4953e06d437086e296a`.
- Run receipt `completed`, 4/4 actions; sealed output fingerprint
  `6fb30d47a8e566517c87f8c23d2279691eafb3fa9afb61b57d578e890fe86145`.
  All four action stderr logs were empty, and there was no WeiDU warning
  receipt. CLI unverified-status notices were classification notices.
- Sealed `WeiDU.log` recorded English component `#0` for Assassinations,
  Back to Brynnlaw, Dungeon Crawl, and Sellswords, in that order, with no
  skipped requested selector.
- Windows UI smoke launched this sealed target's `Baldur.exe` via Explorer,
  observed BG2EE 2.6.6 title selection, chose Shadows of Amn, and observed
  the playable Single Player / Multiplayer / Options menu. The game was quit
  from its menu. No new-game, save, or quest-playthrough smoke was performed.

This supports only the exact tested BG2EE default component routes and source
fingerprint. EET, other languages, gameplay behavior, and arbitrary stacks
remain untested. The exact four-mod configuration is Supported, not Verified,
because gameplay smoke is absent. Raw action logs and the build receipt remain
in the user's local IEPM store rather than committing a full game tree.
