# PA-4 BG2EE friendship expansion evidence

On 2026-09-22, IEPM built the four exact author-tagged releases in
`examples/bg2ee-friendship-expansion/modpack.yaml` into a clean disposable
BG2EE 2.6.6 workspace and sealed it. The portable identity and check
assertions are in [`evidence/pa4-bg2ee-friendship-expansion.json`](../evidence/pa4-bg2ee-friendship-expansion.json).

- Registry revision `af1ec05`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `bd0d03c2293ee9bdec4dc05b2ccd7acee8f56f366713de1664fad5201a9703b0`.
- Run receipt `completed`, 4/4 actions; sealed output fingerprint
  `6d841284be39035b55908ca4da31c54eae5b85afdd5b1bafb35cdea04f26f269`.
  All four action stderr logs were empty. The CLI's unverified notices were
  classification warnings, not WeiDU failure or skipped-component receipts.
- Sealed `WeiDU.log` recorded Dorn, Hexxat, Jaheira, and Korgan component
  `#0`, with Korgan's declared English language index `#1`.
- Windows UI smoke launched this exact pinned sealed target's `Baldur.exe`,
  observed the BG2EE 2.6.6 title selection, chose Shadows of Amn, and reached
  the Single Player / Multiplayer / Options menu. The game then exited from
  its own menu. No new-game, save, or gameplay smoke was performed.

The exact BG2EE default choices are Supported, not Verified. EET, other
languages, gameplay behavior, and combinations with other stacks remain
Untested. Raw WeiDU outputs and the full sealed game remain in the user's
local IEPM store, not Git.
