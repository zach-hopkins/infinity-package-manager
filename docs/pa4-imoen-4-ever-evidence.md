# PA-4 Imoen 4 Ever BG2EE and SoD installation evidence

On 2026-09-23, IEPM installed the exact Imoen 4 Ever v11.6 author-tag ZIP
in two independent disposable game roots. The [BG2EE](../examples/bg2ee-imoen-4-ever/modpack.yaml)
and [SoD](../examples/bgee-imoen-4-ever/modpack.yaml) manifests deliberately
exercise the two distinct component families. Scoped assertions:
[BG2EE](../evidence/pa4-bg2ee-imoen-4-ever.json) and
[SoD](../evidence/pa4-bgee-imoen-4-ever.json).

- Both used registry revision `f8e5c46`, WeiDU 25100, and author-tag ZIP
  SHA-256 `c57fee7058159ee681e77d4c2a80d5a100b6b4adece1d4fadada16fd878593d6`.
- BG2EE clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  lock SHA-256 `9e09c637cadcf7289948e4094cf25facb501f43a465b1ddd3b0abe9c4c0225db`.
  1/1 action completed, sealed fingerprint
  `1c681e0796b5b4b45b548e44a1842f2918cb6a3853d33c2fbcd9b1d7dd3b35ec`.
  WeiDU.log recorded English components `#0`, `#1`, `#2`, `#3`, `#9`, `#30`.
- BGEE clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  lock SHA-256 `cf7df703e2550dd6593f420a9ae97bca9b0c2fe6385715ae93914adcb11ad50a`.
  DLC Merger first merged SoD; 2/2 actions completed. Sealed fingerprint
  `a0e5f43fcabaa39839eb678fa226142918a7d86e7a3ab7ff02dd0371329db373`.
  WeiDU.log recorded English `#10`, `#11`, `#12`, `#13`, `#14`, `#15`, `#16`,
  `#20`, `#25`, `#30` for Imoen 4 Ever.
- Both installer stderr logs were empty, with no recorded skips or warnings.
  The SoD alternative portrait `#17` was not selected. Some SoD components
  accept either `#10` or `#11` in TP2; both were selected here, so this
  install does not establish that IEPM preflight fully models the OR case.
- The Windows UI helper remained unavailable. No game menu, new-game,
  known-save, gameplay, or EET smoke was performed.

Under [verification-policy.md](verification-policy.md), both configurations
remain **Untested** until their game menus are observed. Raw logs and sealed
games remain in the user's local IEPM store, not Git.
