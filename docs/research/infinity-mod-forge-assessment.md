# Infinity Mod Forge ecosystem assessment

This assessment records the input used to plan Product A's final registry and
coverage work. Infinity Mod Forge is a valuable discovery and prioritization
source. It is not automatically authoritative for artifact identity,
compatibility, or IEPM support status.

## Snapshot inspected

- Project: [Infinity Mod Forge](https://krion64.github.io/)
- Repository: [InfinityModForge-Personal](https://github.com/krionashnald/InfinityModForge-Personal)
- Commit: `046414eec9315eb1500f2e68147aa281ab35873d`
- License: MIT
- Inspection date: 2026-09-21

The inspected snapshot contains:

| Measure | Count |
| --- | ---: |
| Catalog/index/detail records | 813 |
| Component rows | 7,575 |
| Records with GitHub repository identifiers | 651 |
| GitHub-identified records with a cached release page | 530 |
| Version-cache entries | 690 |
| Version-cache entries with a release page | 619 |
| Records whose `dl` value is identical to the homepage | 716 |
| Records with conflict rules | 239 |
| Conflict rules in the inspected files | 1,020 |
| Records with dependency rules | 63 |
| Dependency rules in the inspected files | 135 |

The counts are measurements of this exact commit, not permanent claims about
the live site. They may differ from rounded figures in the Forge README as its
data changes.

The catalog is broad enough to seed IEPM discovery. The 651 GitHub identifiers
and 530 cached GitHub release pages also provide a strong first artifact-
resolution tranche. They do not prove that the release page exposes one
unambiguous artifact suitable for IEPM.

## Useful source structure

Forge keeps full per-mod records in `data/mods/`, a generated browse index in
`data/mods-index.json`, an ID-to-file catalog in `data/mods/_catalog.json`, and
dynamic release observations in `data/version_cache.json`. Its three curated
presets contain 15, 65, and 114 unique mods respectively. This is useful for
bootstrap ranking and for future Product B filtering.

The records include names, authors, summaries, categories, narrative phases,
tags, component numbers, TP2 paths, language indices, subcomponent groups,
same-package requirements, game predicates, conflicts, dependencies, known
issues, recommendations, release pages, and repository identifiers. The
project also exposes 26 browsing/install-order categories and richer display
facts such as portraits, kits, spells, and items.

These fields fall into three IEPM buckets:

1. **Discovery/display candidates:** names, summaries, authors, categories,
   tags, phases, homepages, and external IDs. Preserve Forge provenance.
2. **Mechanical candidates to re-derive:** TP2 paths, components, labels,
   language order, subcomponents, and literal predicates. The exact released
   artifact remains the authority.
3. **Behavioral research leads:** conflicts, dependencies, order, EET support,
   known issues, patches, and recommendations. These require author evidence,
   mechanical enforcement, or IEPM fixtures before becoming Supported claims.

Forge documents an omitted `games` field as universal when it found no TP2
game predicate. IEPM must retain the narrower fact: no predicate was observed.
That is not proof of behavioral compatibility with every game target.

## Downloadability finding

Forge's `dl` field cannot be treated as an artifact URL. In the inspected
snapshot, 716 of 813 `dl` values equal the project homepage, and most URLs have
no archive-like extension. GitHub repository roots, release pages, forums,
Nexus pages, and community download pages all appear in the field.

IEPM therefore needs separate outcomes:

- `catalog_url`: the public project or download page is reachable;
- `artifact`: exact downloadable bytes were resolved, fetched, hashed, and
  recognized;
- `acquisition`: automatic, browser-assisted/manual, or unavailable.

A successful GET against a forum page proves discovery health, not
installability. Conversely, a recognized mod with only a manual download page
can remain useful: IEPM can open the page, accept a user-selected archive,
inspect it as opaque, and install with reduced guarantees when technically
possible.

The [Infinity Mod Runner](https://github.com/Anprionsa/infinity-mod-runner)
implements a similar practical split: GitHub releases can be downloaded from
Forge's cached release data, while non-GitHub sources are generally presented
as manual links. Its patches and installer exceptions are valuable research
leads, but they are tool-specific behavior and must not silently become IEPM
package truth.

The paired implementation and its exact acquisition-candidate counts are
analyzed separately in the
[Infinity Mod Runner acquisition assessment](infinity-mod-runner-assessment.md).

## Popularity and compatibility signals

No single Forge field is a defensible popularity score. In particular, GitHub
stars and Weaselmods download-like counts share a cache field but are not
comparable raw values. Preset membership is a useful recommendation signal but
is not independent usage evidence.

The [Infinity Mod Telemetry](https://github.com/Anprionsa/infinity-mod-telemetry)
project provides anonymized mod/component outcomes and approved community
builds. It should be incorporated when available, but its current sample is a
weak signal rather than proof. Public WeiDU logs, maintained guides, official
documentation, and recurring recommendations in established communities can
add independent observations. AI-assisted research may collect and normalize
those observations; it may not manufacture a compatibility claim.

## Product B value retained for later

The following Forge-derived fields are worth retaining with provenance even
when Product A does not use them to resolve a build:

- human-readable names, summaries, authors, and homepages;
- category, tags, narrative phase, intended game/engine observations;
- language availability and platform hints;
- component names and subcomponent/group presentation;
- project status, last release observation, and acquisition mode;
- portraits, kits, spells, items, and recommendations as optional future
  enrichment.

Product B can use this material for browse, search, filters, and explanations.
It must still ask the Rust Product A core for resolution, support status,
artifact identity, and installation decisions.
