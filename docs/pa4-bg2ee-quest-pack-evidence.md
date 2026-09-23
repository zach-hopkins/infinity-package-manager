# PA-4 Quest Pack installation evidence

On 2026-09-23, IEPM installed four quest-content choices from the exact
official Quest Pack v35 tag archive (TP2 VERSION `v3.5`) in a fresh
disposable BG2EE 2.6.6 workspace using the
[fixture](../examples/bg2ee-quest-pack/modpack.yaml). The archive SHA-256 was
`dca023b5b3a0feab6c6cbcc13602c2726520a47f78e00d46687b6c82ef0119cd`
(21,664,127 bytes). `review-tp2` matched all 23 live selectors; seven
other apparent `BEGIN` declarations are inside block comments.

- Registry revision `84a9238`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `8004f507235cba5457b4a45000b6d38d2fe3c03bfe2302bda5b5f45006965502`;
  one action completed; final sealed fingerprint
  `5a6a5a37ce46c96f7a7193ae36046a02608fce8f9e520d183a624aca34ac406e`.
- WeiDU.log records all requested English selectors: Additional Shadow
  Thieves `#5`, Alternative Harper/Xzar `#6`, Extended Reynald `#7`, and
  Copper Coronet `#8`. Stderr was empty and no component was skipped.

WeiDU emitted three nonfatal `REPLACE` dialogue-state `WEIGHT` warnings,
recorded in `01-quest-pack.warnings.log`. It used existing DLG weights for
states 106–108 where the mod specified or omitted different weights. The
installer still reported success and recorded every selected component. This
is **not** proof those dialogue states behave as intended; the warning is
classified as an install-allowing, behavior-unverified signal requiring an
in-game dialogue smoke before a stronger claim.

No observed menu, new-game, affected dialogue, other-selector, or EET smoke
was performed. The [scoped assertion](../evidence/pa4-bg2ee-quest-pack.json)
remains **Untested**. Raw logs and sealed game tree remain in the local IEPM
store, not Git.
