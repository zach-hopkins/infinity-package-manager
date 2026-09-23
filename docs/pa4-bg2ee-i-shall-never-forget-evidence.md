# PA-4 I Shall Never Forget installation evidence

On 2026-09-23, IEPM installed the exact author-hosted 6.5.6 archive using the
[BG2EE fixture](../examples/bg2ee-i-shall-never-forget/modpack.yaml) in a
fresh disposable BG2EE 2.6.6 workspace. The 25,037,879-byte archive SHA-256
was `070b98733bf5a40730349c6d887da0e223c00fe623f84ef1bd245d70dabe2283`.
`review-tp2` matched its sole selector with no drift.

- Registry revision `eebd90c`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `487d1317acff11477ace0e26a6e674b76c6ccd756fbc83585129d088a3b4d12e`;
  final sealed fingerprint
  `d387313ee009f8326f64a13eea097fbb0b29a215c942e5b69e55b85ea65b70c7`.
- WeiDU.log records selector `#0`. Stderr was empty and stdout contains no
  warning, error, or skipped-component marker.

The [scoped assertion](../evidence/pa4-bg2ee-i-shall-never-forget.json)
remains **Untested**: menu, quest behavior, and EET have not been observed.
Raw logs and the sealed game tree remain in the local IEPM store.
