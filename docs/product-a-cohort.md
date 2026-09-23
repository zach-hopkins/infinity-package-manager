# Product A initial coverage cohort

This is the frozen first-pass cohort for Product A release and component work.
It is a planning queue, not a claim that all entries are installable,
compatible, Supported, or Verified.

The machine-readable inputs and results are:

- [selection corpus](../registry/cohorts/product-a-selection-corpus.json)
- [99-package cohort](../registry/cohorts/product-a-cohort.json)
- [coverage report](../registry/cohorts/product-a-coverage-report.json)
- [full acquisition-health report](../registry/catalog/infinity-mod-forge-health.json)

## Measured result

On 2026-09-21, the deterministic selector chose **99 packages**. That is the
smallest cohort meeting the configured floor of 50 packages and 95% coverage:
it covers **95.18%** of the frozen corpus's weighted selections (3.8070 of 4
stack-normalized votes). Twenty-two observed candidates are explicitly
deferred rather than silently forgotten.

The four counted stacks are the three pinned public Infinity Mod Forge presets
and the user-supplied Forge reference list. Each stack contributes one total
vote split across its distinct packages. This intentionally prevents a
component-heavy mega-list from outweighing a small, independent list merely
because it has more components. Thirteen checked-in IEPM reference manifests
are included as required-current-profile inputs, but contribute no popularity
weight.

The corpus currently has only two source families (`forge-preset` and
`user-reference`). Therefore 95.18% means “95.18% of this defined corpus,” not
“95% of every BG player.” Additional independent guides, public mod lists, and
consented telemetry should be added as separately dated sources before making
any broader ecosystem-popularity statement. The selector will recompute the
same metric without hand-editing rankings.

## PA-2 acquisition snapshot

The bounded, rate-limited PA-2 pass checked all 813 indexed records. At the
recorded time, 675 catalog pages were reachable and 55 redirected; 83 failed.
It produced 516 tagged GitHub source-archive candidates, 113 mutable-branch
candidates, and 184 manual-browser routes. A ZIP signature was observed for
560 candidate probes.

Those are reachability observations only. A source archive candidate is not a
release artifact, and a signature probe is not a full download, SHA-256, safe
inspection, installer route, or compatibility claim. The PA-4 process must
promote each cohort package only after it obtains those exact facts.

## PA-4 starting ledger

At the 2026-09-21 freeze, 19 of the 99 cohort packages already mapped to a
curated IEPM package identity. The remaining 80 were indexed candidates with
no executable registry release. As of the [2026-09-23 PA-4 triage](pa4-remaining-cohort-triage.md),
all 80 have exact artifact routes and selector maps, but two have only safe
subsets executable; this historical starting count is not the current count.
Both groups remain useful to users: candidates can still follow the opaque
local-package route once a user supplies exact bytes, but they are not green
badges.

For every cohort package, PA-4 must retain one explicit state:

| State | Meaning |
| --- | --- |
| `indexed candidate` | Discovery information exists, but exact executable release facts are absent. |
| `opaque / untested` | Exact local bytes and inert TP2 facts are available, but semantic compatibility or execution evidence is incomplete. |
| `Supported` / `Verified` | Only after the quantitative gates in [verification policy](verification-policy.md) are met. |
| `Incompatible` | Only with concrete, applicable evidence; never inferred from absence. |

The existing detailed rows in [mod support ledger](mod-support.md) are the
first PA-4 work already underway. Large packages such as Tweaks and SCS still
need complete component classifications; a package-level install receipt does
not complete that requirement. The exact structural gap is tracked by the
[PA-4 component audit](pa4-component-audit.md).

## Reproduce

From a checkout with the pinned Forge source data available, capture the
corpus, then build the cohort. The exact current arguments are deliberately
visible in the CLI help: `iepm catalog-capture-cohort --help` and
`iepm catalog-build-cohort --help`. Include every checked-in
`examples/**/modpack.yaml`, then the user list as an optional input. The output
stores its content hash and a logical source label, never the user's absolute
path or the user list itself.
