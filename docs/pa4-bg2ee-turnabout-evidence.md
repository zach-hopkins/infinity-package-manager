# PA-4 Turnabout v1.8 BG2EE evidence

On 2026-09-22, IEPM resolved
[`examples/bg2ee-turnabout/modpack.yaml`](../examples/bg2ee-turnabout/modpack.yaml)
and automatically selected Ascension's rewritten final chapter through the
component-level prerequisite. The exact status assertion is in
[`evidence/pa4-bg2ee-turnabout.json`](../evidence/pa4-bg2ee-turnabout.json).

- Registry revision `ad701b9`; clean BG2EE 2.6.6 source fingerprint
  `298c1d1eb13f7d5f934aaa25f868679a03fd8bfd7f13028b2646e29a4a10ff2f`;
  WeiDU 25100; executed lockfile SHA-256
  `567a42880d4eff8999f91f879b7434bf8ad54ce26e4c4678ea61b2ade61f690e`.
- The SHA-verified Turnabout v1.8 author-tag archive was
  `2e637c93eb922bd14a8290dc1c521d196a6c2a2ad2b15afac3d95eb175de711e`;
  pinned Ascension 2.1.0 was
  `bd0ee14b3770d56104eaa81287c88a8672afa889a35184fb308be12d6d707e77`.
- Run receipt `completed`, 2/2 WeiDU actions, sealed output fingerprint
  `cf10b562de89d553ccb2ab7876183174a59fe3998b71ee7bad0273a00b6e36ba`.
  Both installer stderr logs were empty and no selected component was skipped.
- Sealed `WeiDU.log` recorded English Ascension `#0` then Turnabout `#0`.
- The sealed game launched on Windows, showed BG2EE 2.6.6 title selection,
  and reached the Throne of Bhaal Single Player menu. The intro movie was
  skipped with one click. No new-game, known-save, final-battle, fallen-ally,
  or portrait smoke was performed.

This exact English BG2EE main-component pairing is Supported, not Verified.
Turnabout's optional Balthazar portrait, EET route, other languages, and
endgame behavior remain Untested. The full sealed game and raw WeiDU logs
remain in the user's local IEPM store, not Git.
