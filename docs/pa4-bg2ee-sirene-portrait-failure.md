# PA-4 Sirene BG2 v2.02 portrait-selector failure

The exact official `v2.02` tag source ZIP has SHA-256
`dc1151a4dadca34d192f6ce708c0ae677051e4f49544ef0952994cd741c6dfdf`.
On 2026-09-23, a fresh disposable BG2EE 2.6.6 build using the
[diagnostic fixture](../examples/bg2ee-sirene-bg2/portrait-repro.yaml)
selected main `#0`, BG1 default portrait `#1`, and True Paladin `#5` at
registry revision `aec93a1`. Its portable lock SHA-256 was
`e98b44cab9d26aa8cef5e566d0844c7f33a7430b00bdfc8dc343b16f2056e67d`.

WeiDU reported `Unix.ENOENT` for
`Sirene_BG2/portraits/alts/Sirene2L.bmp` during selector `#1` and did not
record that selector. It recorded `#0` and `#5`, but IEPM correctly failed
the action and did **not** seal this incomplete build. Stderr was empty;
the failure is in stdout and the action exit code was 2.

The release archive stores `Sirene2L/M/S.bmp`, `Sirene3L/M/S.bmp`,
`Sirene4L/M/S.bmp`, and `Sirene5.bmp` under `Sirene_BG2/alts/`. All four
portrait selectors `#1`–`#4` instead use `Sirene_BG2/portraits/alts/` in
their `COPY` source paths. This is an exact-artifact mechanical mismatch,
not an IEPM selection or game-version conflict. The registry now marks only
those four selectors unavailable for this artifact; the NPC and class
selectors remain separately installable. Do not silently patch the author's
release or generalize the result to an upstream-corrected artifact.

The [scoped assertion](../evidence/pa4-bg2ee-sirene-portrait-failure.json)
classifies this exact portrait selection as **Incompatible**. The failed
workspace and raw logs remain in the local IEPM store, not Git.
