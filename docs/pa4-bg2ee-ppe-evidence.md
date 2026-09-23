# PA-4 Portraits Portraits Everywhere installation evidence

On 2026-09-23 IEPM fetched the exact upstream author commit archive,
332,680,130 bytes, SHA-256
`1ff8c64453e0d956b41f93ddd5a1caae335026572889993f7af3e130e33389df`,
and installed the [BG2EE fixture](../examples/bg2ee-portraits-portraits-everywhere/modpack.yaml)
into a fresh disposable BG2EE 2.6.6 workspace. The upstream README calls
this v1.03, while the TP2 still declares `VERSION 1.01`; these are retained
as distinct facts. All eight live TP2 selectors are mapped; no observed
component is unmapped.

- Registry revision `34eb4e9`, clean source fingerprint: `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`; WeiDU 25100.
- Portable lock SHA-256: `af0e7dde54d89c8bb5b5ff5fc9db8f9b86413795d6fa03a252de61f68819a25d`; sealed target fingerprint: `ba2cff5ea7fe139e4130a24de0506b2dde061f551d7c5bd728814583a01b98df`.
- WeiDU.log records core `#0` and sequenced category portraits `#100`.
  Stderr was empty, with no WeiDU warning or skip markers. WeiDU reported
  category portraits applied to 913 actors; this is installer output, not
  independent in-game visual verification.

The unselected random category alternative, other optional portraits,
BGEE/EET, menu, and actual portrait behavior remain untested. The
[scoped assertion](../evidence/pa4-bg2ee-ppe.json) remains **Untested**
under the public status gates. Raw logs and the sealed build remain in the
local IEPM store.
