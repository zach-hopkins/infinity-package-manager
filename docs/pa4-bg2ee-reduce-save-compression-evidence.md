# PA-4 Reduce Save Compression BG2EE evidence

On 2026-09-22, IEPM installed the single v1.2 component from
[`examples/bg2ee-reduce-save-compression/modpack.yaml`](../examples/bg2ee-reduce-save-compression/modpack.yaml)
in a clean disposable BG2EE 2.6.6 workspace and sealed it. This mod
patches `Baldur.exe` directly, so the test did not touch the immutable
source snapshot or the user's playable installation. The exact identity
and status assertions are in
[`evidence/pa4-bg2ee-reduce-save-compression.json`](../evidence/pa4-bg2ee-reduce-save-compression.json).

- Registry revision `e311657`; clean source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `cca9749d8c7ce3933c4923ec02141e9e3b5d1d78ae449ffca2bb50dd953999d3`.
- The SHA-verified author v1.2 source archive was
  `0760b00ac32543cb7d32aa81291da61de1f142713d27268032d10118986406cb`.
- Run receipt `completed`, 1/1 WeiDU action, sealed output fingerprint
  `ca304a07be61a420b3eb1c10b048644281991283872593a4a061b760664b490b`.
  The action stderr log was empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English component `#0`. The clean source
  `Baldur.exe` SHA-256 was
  `fc821a4806a0305b84fd85f1aad2bd472c8db642ed34b4494ae62351cae1c580`;
  the sealed patched executable was
  `d7d1796b14c284dffcff212449f28ee8b07f4bcd6490b47c44730771d86ba5b3`.
- The exact sealed game launched from its own folder in Windows, showed
  BG2EE 2.6.6 title selection, and reached the Shadows of Amn Single
  Player menu. One click skipped the introductory movie.

This exact Windows BG2EE 2.6.6 default is Supported, not Verified. The
save-speed tradeoff, a new-game or save smoke, other executable builds,
BGEE, EET, and non-Windows platforms remain Untested. Raw WeiDU output
and the full sealed game remain in the user's local IEPM store, not Git.
