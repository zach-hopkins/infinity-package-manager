# PA-4 Northern Tales of the Sword Coast installation evidence

On 2026-09-23, IEPM installed the exact author v5.0.0 tag ZIP using the
[BGEE/SoD fixture](../examples/bgee-northern-tales/modpack.yaml) in a fresh
disposable BGEE 2.6.6 workspace. The source archive SHA-256 was
`6df6f6b4e94f0d5b3d087502be7f042d62e6830eb3566f54c554af3451c63db1`
(135,169,748 bytes). `review-tp2` matched all 15 live selectors with none
unmapped.

- Registry revision `f016841`, WeiDU 25100, clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`.
- Portable lock SHA-256
  `5e6bc198c25c73874d8a063504760d8a36e32b775a8626ffbed1c46f774367e3`;
  DLC Merger and Northern Tales both completed. Final sealed fingerprint
  `f3ea268e723629d0315ce30ce9272922f26bfe3fecb9818893e33d4c0fb27b87`.
- WeiDU.log records Northern Tales main selector `#0` after DLC Merger `#1`.
  Both stderr logs were empty; Northern Tales stdout ended in `SUCCESSFULLY
  INSTALLED` without warning or skip markers.

This is only the BGEE/SoD main route. Optional selectors, EET, menu, and
gameplay remain unobserved. The [scoped assertion](../evidence/pa4-bgee-northern-tales.json)
is therefore **Untested**, not Verified. Raw logs and the sealed game tree
remain in the local IEPM store, not Git.
