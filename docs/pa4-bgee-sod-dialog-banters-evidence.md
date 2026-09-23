# PA-4 SoD Dialog Banters installation evidence

On 2026-09-23, IEPM installed the exact author v1.0 tag ZIP using the
[SoD-enabled BGEE fixture](../examples/bgee-sod-dialog-banters/modpack.yaml)
in a fresh disposable BGEE 2.6.6 workspace. The archive SHA-256 was
`64a15214c6a195c5ca2e6abbffe03ed4339bd2c896c7e204f549a4f10c4ae819`
(36,636 bytes). `review-tp2` matched the sole numeric component; an inlined
dialogue `BEGIN` is not a second WeiDU selector.

- Registry revision `37a2f49`, WeiDU 25100, clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`.
- Portable lock SHA-256
  `1d7453e91288d597bb323d13286ab6e74759412c51ffe619f8b563b2df2ccc43`;
  DLC Merger and SoD Dialog Banters both completed. Final sealed fingerprint
  `2fce4eeed35ccfa48a8163e2f8e74183cf93fc0b1fbd8d8d8f996d368301c550`.
- WeiDU.log records `SODDIALOGBANTERS` selector `#0` after DLC Merger `#1`.
  Both stderr logs were empty; successful stdout had no warning, error,
  skipped, or not-installed markers.

The TP2 requires SoD and rejects an already-EET game. This is evidence for
the BGEE/SoD source route only. No observed menu, new-game, banter behavior,
or cross-mod interaction smoke was performed, so the
[scoped assertion](../evidence/pa4-bgee-sod-dialog-banters.json) remains
**Untested**. Raw logs and the sealed game tree remain in the local IEPM
store, not Git.
