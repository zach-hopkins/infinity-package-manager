# Mod support ledger

This ledger is also the working input to the future Product A coverage cohort.
Before Product B implementation begins, maintainers must freeze and name that
cohort here, include the intended popular BGEE/BG2EE/EET and infrastructure
coverage, and evaluate every exposed component under the measurable
verification policy. The current table is not yet that frozen cohort.

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
| DLC Merger | Recorded release components | Yes, for the EET fixture | **Untested** — clean install evidence exists; policy launch smoke is not yet recorded | [registry](../registry/packages/dlc-merger.yaml) |
| EE Fixpack | Recorded release components | Yes, for the EET fixture | **Untested** — source/target install evidence exists; policy launch smoke is not yet recorded | [registry](../registry/packages/ee-fixpack.yaml) |
| EET / EET_End | Recorded release components | Yes, for the narrow EET route | **Untested** — the controlled route installed; policy launch and gameplay smokes remain | [evidence](a5-eet-evidence.md) |
| Hidden Gameplay Options | Recorded v5.1 components | Yes | **Untested** — components 0 and 10 have separate install receipts; policy launch smoke remains | [registry](../registry/packages/hidden-gameplay-options.yaml) |
| Tweaks Anthology v18 | Complete mechanically-derived selector catalog in the source-tag release; Forge selections are all named | Yes | **Untested** — starter and Forge install receipts exist, but the policy launch smoke and full component classification remain | [registry](../registry/packages/tweaks-anthology.yaml) |
| Sword Coast Stratagems v35.21 | Initial named selector subset; full catalog expansion is in progress | Yes for the mapped selectors; selected component prerequisites are added before execution | **Untested** — the Forge selectors installed, but launch smoke and the remaining catalog are incomplete | [registry](../registry/packages/stratagems.yaml) |
| Ascension | Forge-selected components mapped | Yes | **Untested** — core install evidence exists; policy launch smoke and remaining selections remain | [registry](../registry/packages/ascension.yaml) |
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
