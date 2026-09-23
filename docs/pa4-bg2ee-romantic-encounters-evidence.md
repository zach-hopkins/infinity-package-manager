# PA-4 Romantic Encounters BG2 installation evidence

On 2026-09-23, IEPM installed the exact official v15 tag ZIP using the
[BG2EE fixture](../examples/bg2ee-romantic-encounters/modpack.yaml) in a
fresh disposable BG2EE 2.6.6 workspace. The 4,269,386-byte source archive
SHA-256 was `66eb5761124da3253fe78344bbfc4545cda83ce46a70c3ee072eaa5909bfb5f3`.
`review-tp2` matched all 55 live numeric-only selectors, with stable IDs
based on the author's adjacent scenario comments; none were unmapped.

- Registry revision `50e167c`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `43700b3889dc4a14023e17312a7018c12a89103934b88462cc5206be3aaeb72a`;
  final sealed fingerprint
  `62710f773b80d3d97893c19843a16ed26424d4ab28df28f7be7ce20c4b5af998`.
- WeiDU.log records selectors `#0` (reactions), `#1` (Ada), `#2` (Aimi), and
  `#4` (Aran). Stderr was empty; stdout had no warning or skip markers.

Three unselected scenarios have a `mel01.cre` resource predicate in TP2.
They are not claimed to work on every game state merely because this fixture
sealed. The [scoped assertion](../evidence/pa4-bg2ee-romantic-encounters.json)
remains **Untested** pending other selectors, menu, gameplay, and EET smoke.
Raw logs and the sealed game tree remain in the local IEPM store.
