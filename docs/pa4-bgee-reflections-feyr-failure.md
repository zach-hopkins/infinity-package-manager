# PA-4 Reflections of Destiny 0.9.4 selector 230 failure

On 2026-09-23, two independent clean disposable SoD-enabled BGEE 2.6.6
workspaces failed to install Reflections of Destiny's English component
`fear-feyr` (`D5_REFLECTIONS_FEYR_ITSELF`, selector `230`) from the same
SHA-256-pinned author tag ZIP. The first was a wider SoD story selection; the
second isolated only DLC Merger and selector 230. The isolated manifest is
[`examples/bgee-reflections-feyr-repro/modpack.yaml`](../examples/bgee-reflections-feyr-repro/modpack.yaml),
and its status assertion is
[`evidence/pa4-bgee-reflections-feyr-failure.json`](../evidence/pa4-bgee-reflections-feyr-failure.json).

- Registry revision `0f77cc1`; clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; Reflections ZIP SHA-256
  `4ef856cdd4733ad3a2ffb1f6d136b4d3536091fa6bb333d839ba9bbdc8b060a5`.
- First build lock SHA-256
  `fdd62e8c8bc1a0e510bd1e7be3b617182c6db6244cc94fcee649d759b6149bee`;
  isolated second build lock SHA-256
  `1e7cda815ac5e0d6bd1c5c35832bc35008e953d0948b8a2aaffcd1dddba5ad73`.
- Both builds first merged SoD successfully with DLC Merger. Their independent
  Reflections installer logs then reported `Parsing.Parse_error` in
  `Reflections_of_Destiny/comp/comp_230.tpa`, line 113, and did not record
  component `230` in `WeiDU.log`. The exit codes were 3 and 2 respectively;
  the relevant failure signature was identical.
- The exact source file has the unquoted sentence
  `make sure cut-scene ends...?` at that line, outside a comment or quoted
  WeiDU string. That is a source-level parse defect, not a discovered
  conflict with Road to Discovery or a warning IEPM should ignore.

Under [verification policy](verification-policy.md), the exact selector 230
route is **Incompatible** and is blocked for this release. This does not
classify the other six TP2 selectors or the whole mod as incompatible. The
failed disposable game workspaces and raw installer logs remain local, not
in Git.
