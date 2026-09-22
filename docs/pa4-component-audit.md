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
infer compatibility. Indented WeiDU control-flow `BEGIN` blocks are excluded:
only top-level component declarations count. Line and block comments are also
excluded; this corrected Tweaks Anthology's initial count by four disabled
declarations.

## First audit: SCS v35.21 local package tree

The available local SCS tree reports TP2 `VERSION 35.21` and **146** structural
component declarations. IEPM now maps **134** stable component IDs to that
installer: every BGEE/BG2EE/EET-facing selector, including the supplied
personal build and focused spell-tweak selectors. The newly mapped entries
retain mechanically derived selector/prerequisite evidence only; they are not
a compatibility badge for the whole large stack. The focused selectors each
carry a narrow same-package conflict with SCS's all-spell-tweaks dispatcher,
exactly as its TP2 declares; they do not falsely conflict with one another.
The remaining **12** visible declarations are deliberately not user-facing:
IWD-only selectors, legacy-only helper selectors, and internal TP2 test or
resource-collection blocks.

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
| SCS v35.21 | 146 | 134 | 12 intentionally non-user-facing |
| Tweaks Anthology v18 source | 446 | 237 | 209 |
| Hidden Gameplay Options v5.1 | 43 | 43 | 0 |
| Ascension 2.1.0 | 19 | 19 | 0 |
| Call of the Lost Goddess v3.1 | 3 | 3 | 0 |
| Infinity UI++ v1.23 source | 9 | 9 | 0 |
| EEex v1.2.0 | 9 | 9 | 0 |
| Bubb's Spell Menu v5.2 | 1 | 1 | 0 |
| Throne of the Mad God v2.3 | 1 | 1 | 0 |
| DLC Merger 2.1 | 4 | 4 | 0 |
| EE Fixpack Beta 2 | 7 | 3 | 4 internal-only |

The higher Tweaks number reflects the source TP2's full structural surface,
including choices/subcomponents; it does not mean 446 independent user-facing
features. The existing Forge selections are represented, but that is not the
same as a complete curated component catalog. This audit corrects an earlier
over-broad impression: the registry has substantial **selected-route** support
for Tweaks and SCS, not complete component support yet. Ascension's six clear
author-labeled optional components were then added as mechanically derived,
explicitly Untested selectors, closing its structural catalog without claiming
new compatibility or verification evidence. DLC Merger's two clear
author-labeled optional entries were treated the same way, closing its small
four-component catalog. EE Fixpack contributes three real user-facing
selectors; its other four TP2 declarations are explicit internal generation or
test paths, so they remain recorded by the audit but are intentionally not
offered as user components.
