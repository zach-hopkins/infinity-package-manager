# PA-4 BG2EE friendship build evidence

On 2026-09-22, IEPM built the ten single-component Spellhold Studios BG2
friendship releases in `examples/bg2ee-friendships/modpack.yaml` from a clean
BG2EE 2.6.6 source. This was an IEPM-managed disposable target, not the user's
only game installation. The exact portable identity and check assertions are in
[`evidence/pa4-friendships-bg2ee.json`](../evidence/pa4-friendships-bg2ee.json).

- Registry revision: `d8487e6`; source layout fingerprint:
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- WeiDU 25100; SHA-256 of the executed portable lockfile:
  `96a7f8c7d155f29e79a6992e53b23503f6e8353e0c2c04a62e3dfd6e046c781f`.
- Run receipt: `completed`, 10/10 actions; final layout fingerprint:
  `8fdbc40459553f527a553c8152e43bb93161e04908d3c1f2527d8ac94343d9a1`.
  The output was sealed. All ten action stderr logs were empty and no WeiDU
  warning receipt was produced. The CLI's unverified-status notices were
  classification notices, not installer warnings.
- The sealed target's `WeiDU.log` recorded English component `#0` once for
  each TP2, in order: `cernd`, `haerdalis_friendship`, `imoenfriendship`,
  `korganfriendship`, `mazzy`, `minscfriendship`, `sarevokfriendship`,
  `valygarfriendship`, `viconia`, `yoshimo`. No requested selector was skipped.
- Windows UI smoke: launched this sealed target's `Baldur.exe` via Explorer,
  observed the BG2EE 2.6.6 title selection, chose Shadows of Amn, and
  observed its playable Single Player / Multiplayer / Options menu after the
  intro. Quit using the game's menu. No new-game or save smoke was performed.

This supports the exact ten BG2EE component routes under the recorded source
fingerprint, artifact hashes, platform, and WeiDU version. It does **not**
verify EET, alternative language routes, gameplay behavior, newer releases,
or arbitrary stacks. The exact configuration is Supported, not Verified,
because gameplay smoke is absent. The raw action logs and build receipt are
retained in the user's local IEPM store rather than committed with a game tree.
