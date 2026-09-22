# PA-4 Generalized Biffing v2.9 BG2EE evidence

On 2026-09-22, IEPM built
[`examples/bg2ee-generalized-biffing/modpack.yaml`](../examples/bg2ee-generalized-biffing/modpack.yaml)
from a clean BG2EE 2.6.6 snapshot. The exact status assertion is in
[`evidence/pa4-bg2ee-generalized-biffing.json`](../evidence/pa4-bg2ee-generalized-biffing.json).

- Registry revision `a7655a4`; source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `1d357184e5f0f465964df5a2784e078e9e66bddc10aaaa42ccc9d3c860021360`.
- The SHA-verified author v2.9 archive was
  `0d7b0c4ed5714b97e6362243af4f660b3337cfaa9a8984d7f2c87775d41a598d`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `9d5b5c594e58983c1528a921d2734e86598828f6a330a9b4b2f56a8aff9aca2f`.
  Both installer stderr logs were empty, and no selected component was skipped.
- Sealed `WeiDU.log` recorded English Banter Pack SoA `#0` before
  Generalized Biffing media-only `#0`.
- The sealed game launched on Windows and reached the BG2EE 2.6.6 Shadows of
  Amn Single Player menu. The intro movie was skipped by clicking. No
  new-game, known-save, game-performance, or large-stack smoke was run.

This exact English BG2EE two-mod route is Supported, not Verified. The
all-files alternative (`#1`), BGEE and EET placement, other languages, and
its usefulness or safety in a megamod build remain Untested. The author
notes that Generalized Biffing is often unnecessary for Enhanced Edition
games. Its `post-eet-end` phase puts it after earlier IEPM phases; IEPM does
not yet prove its ordering against another package also assigned to that
final phase. Raw logs and the sealed game remain local, not in Git.
