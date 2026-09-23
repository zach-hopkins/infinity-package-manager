# PA-4 The Lure of the Sirine's Call installation evidence

On 2026-09-23, IEPM installed the exact official v16.5.2 author-tag source
archive in two fresh disposable SoD-enabled BGEE 2.6.6 workspaces. The
[main-only fixture](../examples/bgee-lure-of-the-sirines-call/modpack.yaml)
and [both-component fixture](../examples/bgee-lure-of-the-sirines-call/optional-lighthouse.yaml)
used DLC Merger first. The exact archive SHA-256 is
`1b41e6c8c240f22b5916c852e2817481cd4228b23771143e6fca0534d4464802`
(55,647,402 bytes). Registry revision `108226b`, WeiDU 25100, and clean BGEE
source fingerprint
`04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`
were shared across both builds.

- Main-only: `pa4-bgee-sirinescall-v1652-20260923`; lock SHA-256
  `1c563f802da5c0e969fc579e114d50cb7ede82b8543d90b7957f925bbb1d01b6`;
  final sealed fingerprint
  `a368446f2da82f5cc44940cd66191d0c3f747e4ce923b30317c37fc6dd1baf7e`.
  WeiDU.log records the main quest `#0`.
- Both components: `pa4-bgee-sirinescall-optional-20260923`; lock SHA-256
  `9438ebb6bcb31c66538560ffcf572e0d55151ce4d53b161a3fb62ede7cb0d7db`;
  final sealed fingerprint
  `44ed8def5cb6e165d286a18c5adabb6a5d52a63aab8905ff029fd9692592475d`.
  WeiDU.log records main `#0` and Extended Lighthouse Area `#1`.
- Both runs completed 2/2 WeiDU actions. Stderr logs were empty, and successful
  stdout logs had no warning, error, skipped, or not-installed markers.

The exact TP2 declares VERSION `v16.5.1` despite the v16.5.2 release tag;
`review-tp2` reports only that version drift, with zero unmapped selectors.
The optional component's `REQUIRE_COMPONENT` spells its TP2 path
`SiriensCall/setup-sirinescall.tp2`, yet WeiDU accepted it in the tested
both-component action. We do not infer why or that every install context
behaves identically.

The [scoped assertion](../evidence/pa4-bgee-sirines-call.json) remains
**Untested** for player-facing behavior. Neither game was launched for an
observed menu/new-game or Lighthouse quest smoke, and EET was not run. Raw
logs and sealed game trees remain in the local IEPM store, not Git.
