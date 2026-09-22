# Bootstrap registry: evidence overlay, not a replacement mod database

IEPM's initial Baldur's Gate Enhanced Edition registry is a versioned knowledge
overlay. It is not an assertion that IEPM permanently owns every package fact,
nor that an unlisted legacy mod is unusable. A package can be recognized and
planned with partial facts; its guarantees narrow to exactly what the registry
has evidence for.

## Evidence dimensions

The current schema uses claim-level provenance. Treat the terms below as
independent dimensions rather than a single green/red badge.

| Dimension | Meaning |
| --- | --- |
| Artifact integrity | Exact URL and SHA-256 identify bytes. A location is not the identity. |
| Mechanical metadata | TP2 path, `VERSION`, languages, selectors, and literal predicates were read from the exact release. |
| Compatibility/ordering | An author, a maintained community source, or an automated fixture supports a separate cross-mod claim. |
| Automated verification | An exact release completed a recorded disposable workspace route under stated conditions. |
| Upstream integration | The mod project ships and controls compatible metadata. This improves maintenance; it is not required for IEPM support. |

`declared`, `derived`, `community`, `verified`, and `unverified` name evidence
strength. A bootstrapped release generally remains `unverified` even when its
artifact and TP2 selectors are derived exactly. That warning does not mean
“incompatible” and should not become an expert-mode barrier when an executable
route is otherwise fully described.

## Trust and execution

An `executable` lockfile still requires an artifact, a concrete materialization
route, installer/language mapping, numeric selector, game locale, and a pinned
WeiDU version. Compatibility verification is a separate user-facing warning,
unless IEPM knows a concrete hard conflict. A release with no such technical
route remains `analysis-only` with causal blockers. A recorded `executable`
artifact means IEPM knows its content identity; A4 deliberately refuses to
prepare or run it generically.

The initial bootstrap includes exact Forge-list identities and commonly used
content, tweak, spell, AI, and EET-adjacent infrastructure packages. It keeps
`stratagems` separate from `tactics-remix`; a related project is never an
automatic substitute. The curated Forge component subset is a starting point,
not a claim that every optional component of Tweaks Anthology or SCS has been
semantically modeled.

## Opaque local releases

For an unknown extracted directory or ZIP-family archive, run:

```text
iepm inspect-package --path C:\mods\unknown-release.zip
```

The result can identify a potential WeiDU package, TP2 files, archive
SHA-256, language declaration order, `BEGIN`/`DESIGNATED`/`LABEL` selectors,
and literal game/component predicates. It reports what remains unknown rather
than treating absence from the registry as rejection. `derive-bgmod` can create
a reviewable package-record candidate, but cannot infer compatibility,
installation behavior, or cross-mod semantics.

## Maintenance flow

For a new upstream release: detect it, bind an official artifact and hash,
inspect the exact package, diff its structural observations, and flag drift.
Safe structural data may be regenerated. Compatibility, ordering, prompt,
materialization, and verification claims must be reviewed anew or explicitly
kept unverified. This is the path by which IEPM-maintained candidates can be
adopted upstream without making modder participation a prerequisite for a
release to become `Supported`.
