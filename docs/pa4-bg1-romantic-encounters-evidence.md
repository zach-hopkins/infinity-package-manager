# PA-4 BG1 Romantic Encounters v16 installation evidence

IEPM acquired the exact official [v16 author-tag ZIP](https://github.com/Gibberlings3/BG1_Romantic_Encounters/releases/tag/v16),
SHA-256 `83ab63ca9521c6edd736b50ae3b704832b45036625c337e6754869962f7e493f`
(7,105,299 bytes). The inert TP2 inspector initially undercounted this
release because a complex action scope hid its early declarations. After the
regression fix it detected all **50** LABEL-bearing components: six
mutually exclusive content-style choices `#100`–`#105` and forty-four
encounters `#1`–`#44`. `review-tp2` matches all fifty registry selectors.

On 2026-09-23 IEPM installed the [BGEE fixture](../examples/bgee-bg1-romantic-encounters/modpack.yaml)
in a fresh disposable BGEE 2.6.6 workspace. The selected style was adult
BG-style/show-warnings `#103`, which creates the marker files required by
some encounters; the fixture is a coverage test, not a player recommendation.

- Registry revision `14cdebd`, WeiDU 25100, clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`.
- Lock SHA-256 `3676422903a4223474e7f1536f6d42257d8b16194d21d227da0fde1373a1bd9e`.
  DLC Merger merged SoD first; 2/2 WeiDU actions completed. Final sealed
  fingerprint `71685a61ce47cc2c877c21aa7f6bac01731f5ab4025352b7ca975ec3bc8b8c71`.
- WeiDU.log recorded English style `#103` and every encounter `#1`–`#44`.
  No selector was skipped. Both stderr logs were empty and stdout contained
  no WeiDU warning/error markers. The phrase “Show/Install all Components
  with Warnings” is an author content-style choice, not an IEPM install warning.
- Other style variants, EET, cross-mod behavior, and gameplay are untested.
  In particular, several encounter `REQUIRE_FILE` checks accept marker
  files from *more than one* style variant. IEPM's current narrow
  `requires` primitive cannot fully preflight that OR condition, so this
  release remains unverified rather than falsely complete.
- The Windows UI helper remained unavailable; no menu, new-game, or
  known-save smoke was performed.

The [scoped evidence assertion](../evidence/pa4-bgee-bg1-romantic-encounters.json)
therefore remains **Untested** under the verification policy. Raw logs and
the sealed game remain in the local IEPM store, not Git.
