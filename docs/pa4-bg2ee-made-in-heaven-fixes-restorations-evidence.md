# PA-4 Made in Heaven: Fixes & Restorations installation evidence

On 2026-09-23, IEPM installed an exact author-repository commit archive using
the [BG2EE fixture](../examples/bg2ee-made-in-heaven-fixes-restorations/modpack.yaml)
in a fresh disposable BG2EE 2.6.6 workspace. The 5,754,646-byte archive
SHA-256 was `cb6f7eb44b07e0e4709a8992c61b42c5538ed7754315691f728b5aa4480231ae`.
This is an immutable commit snapshot, **not** a claimed tagged release.
`review-tp2` matched all 20 live selectors with none unmapped.

- Registry revision `40a439f`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `a5b1abd781dcea4afa80c39e7e011fba3e88d3768c7b6a21d534e036660406d9`;
  EE Fixpack core and Made in Heaven selectors `#0` and `#1` completed.
  Final sealed fingerprint
  `24bb2355bc1d3b5ae5f38794eaefa4a17ccc2474a5f440ea798838cbcac72975`.
- WeiDU.log records EE Fixpack `#0` followed by Made in Heaven `#0` and
  `#1`. Stderr was empty and stdout had no warning or skip markers.

The TP2 mechanically requires EE Fixpack `#0` for Made in Heaven `#0` and
`#12`; the fixture exercised that prerequisite for `#0`. EET selection of
those two remains withheld until IEPM can express the pre-import EE Fixpack
baseline. The [scoped assertion](../evidence/pa4-bg2ee-made-in-heaven-fixes-restorations.json)
is **Untested** pending other selectors, menu, gameplay, and EET evidence.
Raw logs and sealed game tree remain in the local IEPM store.
