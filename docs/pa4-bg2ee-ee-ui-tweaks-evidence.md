# PA-4 EE UI Tweaks installation evidence

On 2026-09-23 IEPM fetched the exact official v4.0.7 tag archive,
22,418,163 bytes, SHA-256
`e09d451901e6b70d7dbc98750c9ec5317e92ceec63b6afd764361db6d29920cd`,
and installed [Mods Options and Hidden Game Options](../examples/bg2ee-ee-ui-tweaks/modpack.yaml)
in a fresh disposable BG2EE 2.6.6 workspace. `review-tp2` matched all 67
observed BEGIN entries: 62 live selectors and five deprecated dummy GROUP
workarounds, explicitly unavailable as user choices. No entry is unmapped.

- Registry revision `ecc6066`; WeiDU 25100; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`.
- Portable lock SHA-256
  `9c8ed30ad17484f8ad7c917047bfd04efb8984a80e03ef35b9544192992bae46`;
  sealed target fingerprint
  `1287df625ba3b41c41ba07f21bc9f31baddc688dee8a9b8ff700dea08374a2e6`.
- WeiDU.log records selectors `#1000` and `#1010`. Stderr was empty, with
  no stdout warning or skipped-component markers.

The TP2 has many conditional predicates involving LeUI, Dragonspear UI++,
EET GUI, UI.MENU, engine version, and other selections. This narrow fixture
does **not** establish compatibility with those UI stacks or the behavior
of the other 60 live selectors. Menu/visual and EET smoke remain untested.
The [scoped assertion](../evidence/pa4-bg2ee-ee-ui-tweaks.json) therefore
remains **Untested**. Raw logs and the sealed build are in the local store.
