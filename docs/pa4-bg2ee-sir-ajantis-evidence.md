# PA-4 Sir Ajantis NPC for BGII installation evidence

On 2026-09-23, IEPM installed the exact official v21 author-tag ZIP in a
fresh disposable BG2EE 2.6.6 workspace using the
[representative fixture](../examples/bg2ee-sir-ajantis/modpack.yaml).
The archive SHA-256 was
`8fb2393f84521697cf125f5628d41bc77e723de3c9c4e70a57ca9f6ae592e5b7`
(6,177,474 bytes). `review-tp2` matched all nine mapped selectors, with no
version or language drift.

- Registry revision `f048b97`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `5cf40630311b60082983f5611cc6c3da0df3963cd5a6d6aecf1464bb41ab98f7`;
  one action completed; sealed fingerprint
  `f37e6d8972a214dafb312df7a08527ee305000c65008b856627e9a7383d017f0`.
- WeiDU.log records all five requested English selectors: main `#0`, shield
  BAM `#1`, adult romance `#3`, standard 60-minute dialogue timer `#40`, and
  Cavalier kit `#5`. Stderr was empty; no warning, error, skipped, or
  not-installed marker appeared in stdout.

The other four timer subcomponent alternatives were not installed together
because WeiDU makes them mutually exclusive. No observed menu/new-game,
romance, recruitment, EET continuity, or gameplay smoke was performed. The
[scoped assertion](../evidence/pa4-bg2ee-sir-ajantis.json) therefore remains
**Untested** for player-facing behavior. Raw logs and sealed game tree remain
in the local IEPM store, not Git.
