# PA-4 Innershade installation evidence

On 2026-09-23, IEPM installed the exact author-hosted archive using the
[BG2EE fixture](../examples/bg2ee-innershade/modpack.yaml) in a fresh
disposable BG2EE 2.6.6 workspace. The 43,789,721-byte archive SHA-256 was
`ee1ce60e6167387230a685149dec9094084bcdb1cb807e3b3dcb30e90be4973f`.
The TP2 declares two alternatives, `#0` without existing-save patching and
`#1` with it. Both are mapped; only `#0` was selected. The inert `review-tp2`
found both selectors but reported two language-name drift findings because its
reader truncates language display names containing commas. The registry keeps
the complete author strings; the English index used by this build is unaffected.

- Registry revision `f43a188`, WeiDU 25100, clean BG2EE source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `7080352b36eb410345cc098f9589a68b2bf82f1fd3e42cce8e967e5d1d24f683`;
  final sealed fingerprint
  `d48cae404017ae4d04370756c9539c71dcf6155cd711c02c31e1514fc4d51725`.
- WeiDU.log records selector `#0`; stderr was empty. Stdout contains
  `WARNING: EXTEND_BOTTOM #position 6 out of range 0-6`. WeiDU continued and
  recorded the install. The affected dialogue behavior has not been checked.

The existing-save patching choice remains withheld because its worldmap
function is passed `inclSv = 1` and may reach saves outside the disposable
workspace. This is an IEPM containment gap, not a finding that Innershade is
incompatible. The [scoped assertion](../evidence/pa4-bg2ee-innershade.json)
is **Untested** pending menu and quest smoke, warning review, and EET evidence.
Raw logs and the sealed game tree remain in the local IEPM store.
