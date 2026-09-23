# PA-4 Sirene NPC for BG2EE main/class installation evidence

On 2026-09-23, IEPM installed the exact author v2.02 tag source archive in a
fresh disposable BG2EE 2.6.6 workspace using the
[NPC-plus-class fixture](../examples/bg2ee-sirene-bg2/modpack.yaml). The
archive SHA-256 was
`dc1151a4dadca34d192f6ce708c0ae677051e4f49544ef0952994cd741c6dfdf`
(35,812,410 bytes). `review-tp2` accounts for all nine numeric selectors;
its only drift finding is the absence of a TP2 VERSION declaration.

- Registry revision `7e01cd0`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `b05caf74edec161934d7b9fdf5098f70892d2df229f213b7c947ea309ca212e9`;
  one WeiDU action completed; sealed fingerprint
  `6255921ad8e7e57a59ae449d5ba44458c0279940b2b1a63c0048caf45c8d491c`.
- WeiDU.log records main NPC `#0` and True Paladin `#5`. Stderr was empty,
  with no warning, error, skipped, or not-installed marker in stdout.

The separate [portrait failure](pa4-bg2ee-sirene-portrait-failure.md) is
specific to the exact release's four optional portrait paths; it is not a
failure of the tested NPC/class route. No observed menu, new-game, class
behavior, romance, or EET continuity smoke was performed. The
[scoped assertion](../evidence/pa4-bg2ee-sirene-main-class.json) remains
**Untested** for player-facing behavior. Logs and sealed output remain local.
