# A6: TP2 structural ingestion and release-drift review

A6 adds a small curator-facing evidence path. It reads a **specific local TP2
source file as text** and produces versioned JSON observations for a human to
review against an existing registry release:

```text
iepm inspect-tp2 --tp2 path\\to\\mod.tp2
iepm review-tp2 --registry registry --package package-id \
  --release-id opaque-release-id --tp2 path\\to\\mod.tp2
```

`review-tp2` selects the sole installer automatically. A release with more
than one installer requires `--installer-tp2 registry/relative/path.tp2`, so a
curator never silently compares an arbitrary TP2 with the wrong installer.

## What it observes

The observation format is schema 1 and contains only release-local structural
facts:

- a `VERSION` string, when present;
- `LANGUAGE` declaration order and its numeric WeiDU index;
- `BEGIN` raw token, `DESIGNATED` number, and `LABEL` string.

The review compares these facts with already-curated display version, installer
language mapping, and release-specific component selectors. It reports drift
for missing or changed version, language index, designated component number,
or label. Duplicate observed designated numbers and labels are also reported.
A `match` means these narrow structural selectors agree; it is **not** a claim
that the mod is compatible, safe, installable, or behaviorally unchanged.

## Deliberate boundaries

TP2 is executable installer source. A6 does not execute it, expand `INCLUDE`,
evaluate `ACTION_*`, translate strings, answer `READLN`, discover dependencies,
or run WeiDU. It does not modify YAML, choose artifacts, inherit relationships,
compatibility, prompt answers, materialization rules, or verification status.
The JSON report is instead an ingestion aid: a curator can make a small,
evidence-backed registry edit after review.

This distinction is important because existing ecosystem tools must handle
real-world TP2 discovery, prompt answers, patches, retries, and installer
workarounds. Those are useful operational lessons, but they are not safe
registry facts and do not belong in an automatic inheritance mechanism.

Release identities remain opaque. A TP2 `VERSION` difference is surfaced as
drift, not normalized away: source tag, display version, artifact hash, TP2
version, and executor build may all be separately true facts.

## First evidence point

The retained official Windows Hidden Gameplay Options v5.1 archive was read
locally without execution. Its TP2 reports `VERSION 5.1`, English at language
index 0, and the selectors curated for `install-all` (0) and
`enable-debug-mode` (10). `review-tp2` returned `match` for opaque release ID
`a7-hidden-gameplay-options-v5.1`.

That adds structural corroboration to, but does not replace, the A5 disposable
installation evidence in [a5-hidden-gameplay-options-evidence.md](a5-hidden-gameplay-options-evidence.md).

## External implementation scan

Before defining this boundary, we reviewed the source-oriented catalog and
installer approaches in [Big World Setup's BG2EE list](https://github.com/GrimLefourbe/BWS/blob/master/Config/ModList-BG2EE.ini)
and [Infinity Mod Runner](https://github.com/Anprionsa/infinity-mod-runner).
They reinforce two constraints: download location and mutable upstream
metadata are not content identity, while operational installers often need
mod-specific prompt/patch/recovery behavior. IEPM retains SHA-256 as artifact
identity and keeps such behavior outside A6's inert structural parser.
