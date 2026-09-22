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

The available local SCS tree reports TP2 `VERSION 35.21` and **143** live structural
component declarations. IEPM now maps **134** stable component IDs to that
installer: every BGEE/BG2EE/EET-facing selector, including the supplied
personal build and focused spell-tweak selectors. The newly mapped entries
retain mechanically derived selector/prerequisite evidence only; they are not
a compatibility badge for the whole large stack. The focused selectors each
carry a narrow same-package conflict with SCS's all-spell-tweaks dispatcher,
exactly as its TP2 declares; they do not falsely conflict with one another.
The remaining **9** visible declarations are deliberately not user-facing:
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
| SCS v35.21 | 143 | 134 | 9 intentionally non-user-facing |
| Tweaks Anthology v18 source | 446 | 393 | 53 |
| Sirene v3.1 tag archive | 10 | 10 | 0 |
| LeUI v4.9.1 tag archive | 3 | 3 | 0 |
| Ten Spellhold Studios BG2 friendship tag archives | 10 total | 10 | 0 |
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
The SCS count was also corrected after the inert reader excluded three
declarations inside block comments (an internal test and two resource
collectors). Earlier prose counted those disabled blocks as live components.

The first newly promoted cohort release, Sirene v3.1, exposed a TP2-reader
edge case: unindented `THEN BEGIN` action blocks previously swallowed later
components. The inert reader now preserves all ten implicit-number selectors.
Its component catalog is complete, but `review-tp2` reports
`tp2-version-unobserved` because this author TP2 has no `VERSION` declaration;
the v3.1 release tag is retained separately. This is not install or gameplay
verification.
LeUI v4.9.1 similarly has all three TP2 declarations mapped to stable user
choices. Its author documents an EE 2.6 route and installation before UI
patchers; IEPM has not run an install or launch fixture yet.
The ten friendship releases each expose one mechanically matched component.
Their author READMEs identify BG2EE and EET. A combined clean BG2EE 2.6.6
managed build completed all ten, recorded them in `WeiDU.log`, sealed, and
reached the Shadows of Amn menu. The exact-scope evidence is in
[`pa4-friendships-evidence.md`](pa4-friendships-evidence.md); EET and gameplay
remain untested.
