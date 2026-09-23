# PA-4 Worldmap interactive-installer blocker

The exact BP-BGT Worldmap v13.1.1 author-tag ZIP was acquired and hashed in
the [PA-4 tagged-artifact cohort](../registry/cohorts/pa4-tagged-artifacts.json):
SHA-256 `3890046ab83ae9316299087345cc7011c484fbd343db3958862004bc053314a7`.
Its TP2 declares six LABEL-bearing selectors, all mapped. The independent
larger-map UI choice `#4` has a clean [disposable BG2EE receipt](pa4-bg2ee-worldmap-ui-safe-evidence.md),
but main Worldmap remains unavailable.

The main selector includes `bp-bgt-worldmap/lib/map_size.tpa`. Unless the
installed area table forces the huge map, that file executes `ACTION_READLN
how` in a loop until the user enters `1` (large) or `2` (huge). On some
games `lib/load_tables.tpa` also asks a separate original/revised travel
choice. The existing IEPM WeiDU action runs with null stdin. Merely mapping
the six numbered selectors without gating the main choice would make
lockfile preflight claim it executable while the install could not complete
unattended. We have not executed main Worldmap; only the independent UI
choice was tested in a disposable build.

Before enabling main Worldmap, IEPM needs a small, explicit way to supply validated
installer answers for this exact release/choice context, or a separately
verified noninteractive path. Those answers are **not** WeiDU components and
must not be modeled as fake component IDs. The map-size choice needs to be
portable desired state, with its selected value and release-specific prompt
contract visible in the lockfile/evidence; the huge-forced branch must not
consume the next answer accidentally. Then test a fresh disposable BG2EE or
EET build and inspect WeiDU.log and the game menu. This is a PA-4 automation
gap, not evidence that the mod is incompatible.
