# PA-4 Drake BGEE install evidence

On 2026-09-22, IEPM built Drake v1.7a's NPC and required default portrait in
[`examples/bgee-drake/modpack.yaml`](../examples/bgee-drake/modpack.yaml)
into a clean disposable BGEE 2.6.6 workspace and sealed it. The exact
identity and status assertions are in
[`evidence/pa4-bgee-drake.json`](../evidence/pa4-bgee-drake.json).

- Registry revision `323c927`; clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`;
  WeiDU 25100; executed lockfile SHA-256
  `2259559f65d3ada27fbc53414acb92f152128ee2a6df198660118e01db9d223b`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `7e5badff73401c5d38b4f4b22c321125b2dae8dedcfd68d1e2521a4c039cd581`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English components `#0` and `#1`: the NPC and
  the default forced portrait choice. The TP2 displays VERSION 1.7 while the
  author source tag is v1.7a; both facts are retained.
- A second independent clean disposable build used the same pinned lockfile,
  registry revision, source fingerprint, toolchain, and artifact. It completed
  1/1 actions, recorded both components, had empty stderr, and sealed to the
  same output fingerprint
  `7e5badff73401c5d38b4f4b22c321125b2dae8dedcfd68d1e2521a4c039cd581`.
- Both sealed builds exited before a targetable BGEE window appeared. Windows
  Application Error events for each build recorded `Baldur.exe` exception
  `0xc0000409` at offset `0x0000000000515ed9` (first at 14:56:44, second at
  15:01:55 local time). A previously sealed, different BGEE 2.6.6 disposable
  build launched normally on the same host. No menu or gameplay smoke is
  claimed for Drake. The underlying game/mod defect has not been diagnosed.

Under [verification policy](verification-policy.md), the exact Drake v1.7a
NPC + default portrait configuration on this BGEE fingerprint is
**Incompatible**: two independent clean builds reproduced the same launch
failure. This does **not** establish that the NPC alone, alternate portraits,
other game versions, or EET are incompatible; those routes remain Untested.
Raw WeiDU outputs, Windows crash reports, and the full sealed games remain
local, not in Git.
