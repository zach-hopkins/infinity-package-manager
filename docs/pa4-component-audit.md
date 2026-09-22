# PA-4 component audit

PA-4 starts with full inert structural inventories before assigning stable IEPM
component IDs. This order matters: a numeric WeiDU selector is a
release-specific implementation detail, not user intent. IEPM must not fill a
registry with durable-looking names such as `component-6500` merely to make a
coverage percentage look better.

`iepm review-tp2` now reports a `component_catalog` alongside its existing
release-drift findings:

- total TP2 `BEGIN` declarations observed;
- stable registry IDs whose release-specific selectors match; and
- every remaining raw `BEGIN`/`DESIGNATED`/`LABEL` observation.

It reads TP2 text only. It does not execute WeiDU code, modify the registry, or
infer compatibility.

## First audit: SCS v35.21 local package tree

The available local SCS tree reports TP2 `VERSION 35.21` and **146** structural
component declarations. IEPM currently maps eight stable component IDs to that
installer: batch mode, spell-tweaks batch, AI initialization, smarter mages,
smarter priests, improved fiends/celestials, and the two Ascension integration
selectors. The remaining **138** structural entries are now explicitly visible
in the review output rather than being invisible coverage debt.

This confirms the correct next PA-4 task: name/classify the unmapped entries
from SCS's author release text, attach release-specific selectors, and keep
their state `Untested` until evidence exists. It does **not** justify calling
the local `setup-stratagems.exe` the pinned release artifact: its SHA-256 does
not match the registry's official SFX artifact hash, so it is a useful local
TP2 source only.

Run the audit with:

```text
iepm review-tp2 --registry registry --package stratagems \
  --release-id stratagems-v35.21-windows \
  --tp2 <local-scs>/stratagems/setup-stratagems.tp2 \
  --installer-tp2 stratagems/setup-stratagems.tp2
```

The same audit applies to every cohort release once its exact artifact is
available. A mismatch becomes a review item; it is never silently inherited.

## Initial local audit results

The same structural review was run against the available local source trees.
These counts are not support statuses; they are a precise PA-4 naming and
classification backlog.

| Package/release | Observed TP2 declarations | Existing stable IDs matched | Still unmapped |
| --- | ---: | ---: | ---: |
| SCS v35.21 | 146 | 8 | 138 |
| Tweaks Anthology v18 source | 450 | 36 | 414 |
| Ascension 2.1.0 | 19 | 13 | 6 |

The higher Tweaks number reflects the source TP2's full structural surface,
including choices/subcomponents; it does not mean 450 independent user-facing
features. The existing Forge selections are represented, but that is not the
same as a complete curated component catalog. This audit corrects an earlier
over-broad impression: the registry has substantial **selected-route** support
for Tweaks and SCS, not complete component support yet.
