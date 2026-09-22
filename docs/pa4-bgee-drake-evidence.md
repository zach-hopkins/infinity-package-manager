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
- Launch attempts from the sealed game folder and with an explicit working
  directory exited before a targetable BGEE window appeared. No new game
  crash dump was observed during these attempts. The reason is not yet
  established; no menu or gameplay smoke is claimed.

This is exact-scope install evidence, **not** Supported status. The selected
BGEE route remains Untested pending a successful launch smoke or a diagnosed
environmental cause. Alternate portraits, crossbow proficiency, original
soundset, EET, and gameplay also remain Untested. Raw WeiDU output and the
full sealed game remain in the user's local IEPM store, not Git.
