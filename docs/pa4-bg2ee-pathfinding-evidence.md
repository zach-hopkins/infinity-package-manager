# PA-4 Bubb pathfinding patch installation evidence

The exact 1,266,668-byte ZIP [attached by Bubb to the 2.6.6 discussion](https://forums.beamdog.com/discussion/72639/comments-on-new-v2-5-pathfinding-it-is-bad-news/p2)
was SHA-256 `d07da360f7b51dc51f0da77c68691d10e47b8af1a4ebba4308efac61b6a8e32b`.
Its TP2 VERSION is 1.1 and it declares one component. IEPM deliberately
used pinned WeiDU 25100, not the bundled setup executable, to install the
[fixture](../examples/bg2ee-bubb-revert-pathfinding/modpack.yaml) into a
fresh disposable BG2EE 2.6.6 workspace on 2026-09-23.

- Registry revision `c17ca5d`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  source `Baldur.exe` SHA-256
  `fc821a4806a0305b84fd85f1aad2bd472c8db642ed34b4494ae62351cae1c580`.
- Portable lock SHA-256
  `a8dd94fa64dcaa9ac31360f9ac6cb25bbe9a116ef598a039ed5656aa5252446a`;
  sealed target fingerprint
  `15e76bb93f9f2a702975b8f9cbc001ea65813833268b5126f5c28fc03251aaba`.
- The sealed target `Baldur.exe` SHA-256 is
  `5a6e428b27b50579164f45ae6ccab48a5ac5a037091a896eca7c4b6dce56a112`.
  Source and target executables are the same length, 7,182,336 bytes,
  and differ at exactly offset 3,922,938 (`0x01` → `0x00`). The source
  snapshot stayed unchanged.
- WeiDU.log records component `#0`. Stderr was empty, with no warning or
  skipped-component marker. The TP2 has no LANGUAGE declaration; WeiDU
  accepted English index 0, while structural review reports that distinction
  as language-mapping drift.

This is installation evidence for **this exact executable**, not proof of
better in-game pathfinding, other storefront binaries, or future game
versions. The [scoped assertion](../evidence/pa4-bg2ee-pathfinding.json)
remains **Untested** pending movement smoke. Raw logs and sealed build remain
in the local store.
