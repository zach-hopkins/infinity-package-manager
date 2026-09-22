# Ecosystem discovery catalog

The discovery catalog is IEPM's broad, non-executable view of known mods. It
exists so Product A can browse and explain the ecosystem without pretending
that every discovered project has a verified artifact route or compatibility
status.

It is deliberately separate from `registry/packages/*.yaml`:

| Data | Used by resolver/executor? | Meaning |
| --- | --- | --- |
| `registry/packages/*.yaml` | Yes | Curated exact releases, artifacts, selectors, relationships, and evidence. |
| `registry/catalog/infinity-mod-forge.json` | No | Discovery and display observations imported from a pinned Forge snapshot. |

The catalog cannot make a package installable, create a resolver alias, assert
a game is compatible, or turn a community conflict/order statement into an
IEPM claim. `observed_game_targets` means only that Forge published target
filtering information for the record. Absence means no source predicate was
published; neither state is a compatibility verdict.

## Current source

The checked-in catalog is generated from Infinity Mod Forge commit
`046414eec9315eb1500f2e68147aa281ab35873d`, using its
`data/mods-index.json` input. The exact input SHA-256, source repository,
license, imported count, rejected records, candidate collisions, and matches to
existing curated packages are retained beside it in
`registry/catalog/infinity-mod-forge-report.json`.

At this revision the importer accounted for all 813 records with zero rejected
records and zero candidate-ID collisions. Nineteen records match existing
curated IEPM packages. All other `candidate_package` values are generated
suggestions for review, not canonical package identities.

## Regeneration

Use a pinned upstream checkout. Do not regenerate from an unreviewed moving
branch and then label the output as the recorded revision.

```text
git clone https://github.com/krionashnald/InfinityModForge-Personal.git forge-source
git -C forge-source checkout 046414eec9315eb1500f2e68147aa281ab35873d

cargo run -p iepm -- catalog-import-forge \
  --source forge-source/data/mods-index.json \
  --output registry/catalog/infinity-mod-forge.json \
  --report registry/catalog/infinity-mod-forge-report.json
```

The command loads the existing curated registry only to report unambiguous
identity matches. It never writes YAML package records. A changed upstream
revision must be supplied explicitly with `--source-revision`, reviewed as an
input change, and accompanied by the generated diff/report.

## What comes next

PA-2 operates on this catalog. It will separately track whether a project page
is reachable, whether exact bytes can be acquired and hashed, and whether a
user must supply an archive manually. A successful project-page GET will not
be counted as a verified artifact.
