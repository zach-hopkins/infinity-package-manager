# Mod support ledger

The first Product A coverage cohort is now frozen in
[product-a-cohort.md](product-a-cohort.md): 99 packages meet 95.18% of its
dated, stack-normalized input corpus. This ledger remains the human companion
for PA-4 component and evidence work. Cohort membership schedules that work;
it does not turn an indexed package into a green support badge.

This is the human-facing companion to the static registry. It answers two
different questions which should never be collapsed into one badge:

- **Can IEPM construct and run this exact package selection?** This is the
  lockfile's `executable` readiness, based on a pinned artifact, materialization
  route, launcher, language, and selected component selector.
- **Has this exact package/component combination been proven compatible in a
  disposable game build?** This is execution evidence. Absence of it is a
  warning, not a refusal to install, unless a concrete execution fact is
  missing.

The registry YAML remains the source of truth for artifact identity, selectors,
relationships, and claim provenance. Update this ledger in the same change as
a material support change, linking the registry record and any evidence note.
The [compatibility policy](compatibility.md#verification-scope-prove-claims-not-universes)
defines the deliberately narrow scope of each automated-verification claim.
The [verification status policy](verification-policy.md) defines the exact
gates for the public `Verified`, `Supported`, `Untested`, and `Incompatible`
labels. This ledger must use those words only with those meanings.

| Mod | Component coverage | Can IEPM run it? | Compatibility standing | Notes |
| --- | --- | --- | --- | --- |
| DLC Merger 2.1 | Complete 4-component structural catalog | Yes, for the EET fixture | **Untested** — clean install evidence exists; the two newly mapped optional paths have no execution evidence yet | [registry](../registry/packages/dlc-merger.yaml) |
| EE Fixpack Beta 2 | 3 user-facing selectors mapped; 4 observed TP2 declarations are internal maintenance/test entries | Yes, for the EET fixture | **Untested** — source/target core install evidence exists; optional selectors still need their own evidence | [registry](../registry/packages/ee-fixpack.yaml) |
| EET / EET_End | Recorded release components | Yes, for the narrow EET route | **Untested** — the controlled route installed; policy launch and gameplay smokes remain | [evidence](a5-eet-evidence.md) |
| Hidden Gameplay Options | Recorded v5.1 components | Yes | **Untested** — components 0 and 10 have separate install receipts; policy launch smoke remains | [registry](../registry/packages/hidden-gameplay-options.yaml) |
| Tweaks Anthology v18 | 36 stable IDs mapped against 450 locally observed TP2 declarations; Forge selections are named | Yes for mapped selectors | **Untested** — starter and Forge install receipts exist, but the policy launch smoke and 414 structural entries still need user-intent classification | [registry](../registry/packages/tweaks-anthology.yaml) |
| Sword Coast Stratagems v35.21 | 8 stable IDs mapped against 146 locally observed TP2 declarations | Yes for the mapped selectors; selected component prerequisites are added before execution | **Untested** — the Forge selectors installed, but launch smoke and 138 structural entries remain unclassified | [registry](../registry/packages/stratagems.yaml) |
| Ascension 2.1.0 | Complete 19-component structural catalog; all selectors matched against the local TP2 audit | Yes | **Untested** — the core route has install evidence, while the newly mapped optional components still require their own evidence | [registry](../registry/packages/ascension.yaml) |
| Call of the Lost Goddess | Recorded component | Yes | **Untested** — fixture install evidence exists; policy launch smoke remains | [registry](../registry/packages/call-of-the-lost-goddess.yaml) |
| Throne of the Mad God | Recorded component | Yes | **Untested** — fixture install evidence exists; policy launch smoke remains | [registry](../registry/packages/throne-of-the-mad-god.yaml) |
| EEex | Recorded components | Yes | **Untested** — bootstrap/main/LuaJIT have install evidence; launch smoke and native-extension interactions remain | [registry](../registry/packages/eeex.yaml) |
| Infinity UI++ | Recorded component | Yes | **Untested** — dedicated install evidence exists; policy launch smoke remains | [registry](../registry/packages/infinity-ui-plus-plus.yaml) |
| Bubb's Spell Menu | Recorded component | Yes | **Untested** — the EEex-dependent fixture installed; policy launch smoke remains | [registry](../registry/packages/bubbs-spell-menu.yaml) |

## Reading the Forge manifest

`examples/forge-coverage/modpack.yaml` is the exact, current Forge-derived
request. It is executable at preflight for its represented selections. A fresh
IEPM-managed build named `forge-scs-prereq-20260921` completed all 14 planned
actions and was sealed. That is evidence for this exact source fingerprint,
artifact set, component selection, toolchain, and ordering—not a claim that
every unselected component or arbitrary EET stack is verified. The completed
run retained narrow warning receipts for EET, SCS, and Tweaks; their requested
components were independently recorded in `WeiDU.log`. The warnings are
evidence to inspect, not incompatibility claims.

Do not call an Untested row impossible simply because its author has not
participated. A release can become Supported through exact artifacts, complete
selectors, curated relationships, and automated disposable-build evidence.

## Cohort work boundary

The cohort currently contains 19 packages already mapped to curated IEPM
identities and 80 indexed candidates. The 80 candidates are **not** silently
excluded: they may proceed through IEPM's opaque local-package route once
their exact bytes are supplied and safely inspected. They simply do not yet
have an executable release record or component-by-component support ledger.

For the mapped rows above, retain the same discipline: “Yes, for this exact
component route” is not “all components and combinations are supported.” The
complete selected/deferred list and PA-2 acquisition observations live in the
[Product A cohort report](product-a-cohort.md).

The PA-4 component-inventory procedure and the first SCS audit are documented
in [PA-4 component audit](pa4-component-audit.md). It deliberately exposes
unmapped TP2 selectors without turning them into misleading stable IDs.
