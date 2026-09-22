# PA-4 Isra NPC for BGII v3.1 BG2EE evidence

On 2026-09-22, IEPM resolved
[`examples/bg2ee-isra-bg2/modpack.yaml`](../examples/bg2ee-isra-bg2/modpack.yaml)
and installed the English main NPC component into a clean disposable BG2EE
2.6.6 workspace. The exact status assertion is in
[`evidence/pa4-bg2ee-isra-bg2.json`](../evidence/pa4-bg2ee-isra-bg2.json).

- Registry revision `15ba451`; source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `b7db7468b575b98af7467822ac085ffe9781d616af9e97801cd94ee866479678`.
- The exact v3.1 author-tag ZIP matched SHA-256
  `45bd7011bdb30ef2b4759ac9a8fa2a4a25711efa545efbfc2b0c20841180a8a8`.
- Run receipt `completed`, 1/1 WeiDU actions, sealed output fingerprint
  `48b0b4c129b649aa47d56d2eaf8281dfe63fd421e9a1c42f10d2d53d5cead42b`.
  Installer stderr was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English Isra main `#0`.
- The sealed game launched, showed BG2EE 2.6.6 title selection, and reached
  the Shadows of Amn Single Player menu. The intro movie was skipped with one
  click. No new-game, known-save, NPC recruitment/story, optional crossmod, or
  EET smoke was performed.

The exact English BG2EE main-component route is **Supported**, not Verified.
The crossmod component remains Untested: installing it without the other NPCs
it detects would not demonstrate an actual interaction. The full sealed game
and raw WeiDU logs remain in the user's local IEPM store, not Git.
