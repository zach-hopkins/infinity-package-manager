# PA-4 Recorder BG1 installation evidence

On 2026-09-23, IEPM installed an exact author-repository commit archive using
the [BGEE fixture](../examples/bgee-recorder-bg1/modpack.yaml) in a fresh
disposable BGEE/SoD 2.6.6 workspace. The 5,729,151-byte archive SHA-256 was
`339b72a7003e88e6ba76fad97e573f3763706416fe7d73717e10f99b6c89d165`.
It is an immutable commit snapshot, **not** a claimed tagged release.
`review-tp2` matched both live numeric selectors with none unmapped.

- Registry revision `3e874bf`, WeiDU 25100, clean BGEE source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`.
- Portable lock SHA-256
  `14f843b2567dc637e303da8a8b49886f471f3aef7929a52e8c023d7a5f738f16`;
  DLC Merger and Recorder main both completed. Final sealed fingerprint
  `1ae0986b0b7af6e391ee5938b572027ff735b5ea4dc69f5925e664e42d08c2e1`.
- WeiDU.log records Recorder selector `#0`; stderr was empty. Stdout contains
  three nonfatal `EXTEND_TOP #position ... out of range` warnings (requested
  positions 7, 7, and 6). WeiDU recorded the install, but affected dialogue
  behavior has not been checked.

The music selector was not selected. The [scoped assertion](../evidence/pa4-bgee-recorder-bg1.json)
remains **Untested** pending menu, dialogue, music, and EET evidence. Raw
logs and sealed game tree remain in the local IEPM store.
