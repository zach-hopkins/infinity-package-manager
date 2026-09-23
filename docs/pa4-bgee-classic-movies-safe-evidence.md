# PA-4 Classic Movies safe component evidence

On 2026-09-23 IEPM fetched the exact official V2.4.1 tag ZIP,
586,709,034 bytes, SHA-256
`c740bce60ee33bed5652d51652015a2ad3ca5b199480a49f804dbee35f65cc58`.
All eight LABEL-bearing TP2 selectors match the registry. Only selector `#3`,
Restore BG1 Chapter and Dream Screens, has a TP2 section with no
`USER_DIRECTORY` write or interactive prompt. The seven other choices are
unavailable under IEPM's current external-write boundary.

The [safe fixture](../examples/bgee-classic-movies-safe/modpack.yaml)
installed DLC Merger then selector `#3` on a clean disposable BGEE/SoD 2.6.6
copy and sealed successfully. WeiDU.log records both selectors; stderr was
empty and stdout had no warning/skip markers.

- Registry revision `5ea3c94`, WeiDU 25100, clean source fingerprint
  `04fc6602150ff7788875573dbf0b7f4aeb7ca26aaf83b022c77d9fc417d848c3`.
- Portable lock SHA-256
  `928d192f5aa86bc8b0313a10fec8c73d3fab2ed7acb36ca1a5d4604cc842a6bd`;
  sealed target fingerprint
  `86bd67dd46c19ddf08ba1e17210aa235cb5fd73c9df19ce47ac34a49d11ca351`.

This does **not** demonstrate that IEPM can safely install the mod's main
movie-restoration choices. Those still need per-build isolation of WeiDU's
user directory. The [scoped assertion](../evidence/pa4-bgee-classic-movies-safe.json)
is **Untested** pending visual smoke. Raw logs and sealed build remain local.
