# IEPM Author Adoption & Verification Pipeline

Implement the complete **bootstrapped → author-owned → reviewed → automatically verified** workflow for popular Infinity Engine mods.

This feature sits on top of the existing IEPM package/registry architecture and the SvelteKit frontend.

The goal is to make mod author participation extremely low-friction while preserving IEPM's core philosophy:

> **Authors should be able to participate at whatever level of abstraction they prefer.**

A technically inclined author should be able to simply:

```text
edit bgmod.yaml
→ git commit
→ git push
→ done
```

An author who does not want to learn IEPM syntax should instead be able to use a short guided review experience.

An author facing an unusually complex change should be able to hand the ambiguity to an IEPM maintainer.

All three workflows must produce or modify the **same underlying metadata model**.

Author participation remains optional.

No package should become unusable merely because its author has not adopted IEPM.

---

# 1. Desired lifecycle

A popular mod should be able to progress through states such as:

```text
DISCOVERED
    ↓
BOOTSTRAPPED
    ↓
STRUCTURALLY UNDERSTOOD
    ↓
CURATED
    ↓
AUTHOR OWNED / AUTHOR REVIEWED
    ↓
AUTOMATED VERIFIED
```

Do not treat this as one single boolean support state.

Different facts about the same release can have different provenance and verification status.

Example:

```text
Artifact identity       verified
Component structure     mechanically-derived
EET compatibility       automated-verified
Mage AI capability      author-declared
SCS interoperability    community-curated
Author integration      upstream
```

Do not flatten all of this into:

```text
supported = true
```

---

# 2. Core state dimensions

Track support along independent dimensions.

Useful dimensions include:

```text
Package recognition:
unknown
recognized
bootstrapped
```

```text
Metadata completeness:
minimal
partial
complete
```

```text
Metadata ownership:
external-registry
author-upstream
mixed
```

```text
Compatibility confidence:
unknown
unverified
community-curated
author-declared
automated-verified
```

```text
Review state:
clean
review-suggested
review-required
maintainer-review
resolved
```

```text
Execution support:
opaque
partially-executable
executable
verified-executable
```

The normal-user UI may summarize these into a simpler presentation, but the underlying evidence must remain separate.

---

# 3. Bootstrap phase

IEPM initially bootstraps popular BGEE/BG2EE/EET mods using evidence in approximately this priority:

1. exact official release artifact
2. official repository
3. author documentation
4. mechanically derived WeiDU/TP2 metadata
5. automated testing
6. maintained community documentation
7. legacy managers/forums as lower-confidence research sources

Bootstrap metadata must record provenance.

Example:

```yaml
components:
  smarter-mages:
    weidu:
      label: smarter_mages

    provenance:
      weidu.label:
        type: mechanically-derived

      capabilities:
        type: community-curated
```

Do not mark a package fully verified merely because IEPM can install it.

---

# 4. Registry is an overlay, not the permanent brain

The long-term data hierarchy is:

```text
released package
      ↓
machine-derived metadata
      ↓
author-shipped metadata
      ↓
registry ecosystem overlay
      ↓
automated verification evidence
      ↓
resolver
```

The registry should gradually shrink in responsibility.

It should not permanently own information that:

* can be derived from the package itself;
* belongs naturally to the package author;
* has moved upstream into author-maintained metadata.

The registry remains appropriate for:

* cross-mod relationships;
* community discoveries;
* automated verification records;
* mirrors;
* compatibility exceptions involving multiple projects;
* ecosystem-level semantic knowledge.

---

# 5. `bgmod.yaml` is a first-class authoring language

This is a core design requirement.

`bgmod.yaml` must not merely be an internal serialization format that happens to use YAML.

It should be designed as a **human-authored declarative language for mod intent**.

A technically inclined modder should be able to use IEPM without ever visiting the IEPM website.

The normal direct-author workflow is:

```text
edit bgmod.yaml
      ↓
git commit
      ↓
git push
      ↓
IEPM GitHub Action validates
      ↓
done
```

Example:

```yaml
package: tactics-remix

components:
  smarter-mages:
    provides:
      - mage-ai

    conflicts:
      - capability: mage-ai

    after:
      - package: eet-end
```

Or:

```yaml
components:
  improved-demons:
    requires:
      - package: ascension
        version: ">=2.0"
```

If this metadata is valid and consistent with the package, the author should receive:

```text
✓ IEPM Metadata

Schema valid.
Package structure recognized.
All referenced components exist.
No dependency cycles introduced.
No further review required.
```

They are finished.

No website visit.

No second PR.

No IEPM administrator approval.

---

# 6. What authors should and should not write

Author metadata should primarily express **intent and stable semantics**.

Good examples:

```yaml
provides:
  - mage-ai
```

```yaml
requires:
  - package: foo
```

```yaml
after:
  - package: eet-end
```

```yaml
replaces:
  - old-mage-ai
```

```yaml
deprecated: true
```

IEPM should continue mechanically deriving implementation facts where reliable.

Authors generally should not need to manually maintain:

* artifact SHA-256;
* numeric WeiDU IDs if stable LABEL information exists;
* raw archive structure;
* derived TP2 component lists;
* mechanically visible `GAME_IS`;
* other facts that tooling can recover reliably.

The author file should answer:

> **What do these parts of my mod mean?**

rather than:

> **What bytes happen to be in this release?**

---

# 7. Schema usability requirements

Because direct editing is first-class, `bgmod.yaml` must be pleasant to edit manually.

Optimize for:

* minimal nesting;
* clear names;
* stable symbolic IDs;
* sensible defaults;
* comments;
* strong validation messages;
* JSON Schema / editor autocomplete;
* examples;
* predictable semantics;
* no unnecessary duplication.

Avoid clever syntax merely to make the parser elegant.

A mod author familiar with WeiDU should be able to read most ordinary IEPM metadata without studying a large manual.

---

# 8. Three equal author workflows

The author must have three legitimate paths.

```text
                    ┌─ direct bgmod.yaml editing
                    │
mod changes ────────┼─ guided IEPM web review
                    │
                    └─ ask IEPM maintainer
                            ↓
                    same metadata model
                            ↓
                    same validator
                            ↓
                    same Git history
```

These are different interfaces, not different metadata systems.

## Path A — Direct authoring

Best for authors comfortable with structured configuration.

```text
edit metadata
→ push
→ CI validates
→ done
```

## Path B — Guided review

Best for authors who do not want to understand the schema.

```text
IEPM detects drift
→ author reviews plain-English proposal
→ wizard generates metadata changes
→ PR
```

## Path C — Maintainer assistance

Best for difficult or ambiguous changes.

```text
IEPM detects ambiguity
→ author asks for help
→ IEPM maintainer investigates
→ proposed metadata returned to author
→ author approves
→ PR
```

No path should be considered second-class.

---

# 9. Upstream onboarding

Once a bootstrapped package has sufficiently good metadata, IEPM should be able to prepare an upstream integration package.

Conceptually:

```text
iepm upstream prepare <package>
```

This may generate:

```text
bgmod.yaml
.github/workflows/iepm.yml
author-facing explanation
PR description
```

Do not send authors a giant compatibility database.

The upstream contribution should contain only package-local metadata appropriate for the project itself to own.

The desired transition is:

```text
IEPM-maintained bootstrap metadata
        ↓
author receives ready-made integration
        ↓
author merges bgmod.yaml
        ↓
future releases carry author intent
        ↓
IEPM registry stops owning those fields
```

---

# 10. Official GitHub Action

Provide an official reusable GitHub Action.

Example:

```yaml
name: IEPM Metadata

on:
  pull_request:
  push:
    branches:
      - main

jobs:
  iepm:
    uses: infinity-package-manager/actions/.github/workflows/metadata.yml@v1
```

Configuration should be minimal.

The Action should:

1. inspect TP2/package structure;
2. locate `bgmod.yaml`;
3. validate the schema;
4. compare current package structure against metadata;
5. compare the current branch/release against the previous known state;
6. detect structural and semantic drift;
7. determine which existing claims remain safely valid;
8. identify uncertain claims;
9. generate a `ReviewBundle` when necessary;
10. report the result through GitHub Checks.

---

# 11. GitHub Action as compiler/linter

For direct authors, the Action acts like the compiler for the IEPM authoring language.

Example failure:

```text
IEPM metadata validation failed

components.improved-demons.requires[0]

Unknown package:
  ascensoin

Did you mean:
  ascension
```

Another:

```text
Dependency cycle introduced:

foo
→ bar
→ baz
→ foo
```

Another:

```text
Component `smarter-mages` references:

LABEL smarter_mages

but no matching WeiDU component exists in this package state.
```

Diagnostics should reference:

* the relevant metadata field;
* the actual package state;
* likely fixes when possible.

The author should not need to reverse-engineer validator internals.

---

# 12. Direct metadata changes suppress unnecessary review

If the author changes their mod **and correctly updates `bgmod.yaml` in the same commit**, IEPM should validate the result and move on.

Example:

```text
✓ IEPM Metadata

Structural changes detected and corresponding metadata updates found.

No review required.
```

Do not force authors through the wizard merely because a structural change occurred.

The wizard exists to resolve **unaccounted-for drift**, not to second-guess explicit valid author metadata.

---

# 13. Contradictory evidence

Author intent does not erase external evidence.

Suppose the author removes:

```yaml
conflicts:
  - package: scs
    component: smarter-mages
```

but IEPM has automated or curated evidence that the combination still fails.

The Action or ecosystem system should report:

```text
⚠ Metadata changed

Author-declared conflict removed.

Existing IEPM verification evidence still indicates
an interoperability failure with SCS smarter-mages.

This does not block the metadata change,
but the evidence conflict will remain visible.
```

Preserve both facts:

```text
author declaration:
no conflict

automated evidence:
known failing combination
```

Do not silently overwrite evidence merely because one source changed.

---

# 14. ReviewBundle

Create a stable machine-readable review format.

Conceptually:

```json
{
  "schema": 1,
  "repository": "owner/mod",
  "commit": "abc123",
  "base_commit": "def456",
  "package_id": "tactics-remix",
  "release_candidate": "8.3",
  "findings": [],
  "proposed_changes": [],
  "invalidated_claims": [],
  "unchanged_claims": []
}
```

Every review surface consumes the same bundle:

```text
GitHub Action
      ↓
ReviewBundle
      ↓
CLI
Desktop GUI
SvelteKit web review
      ↓
metadata patch
```

Review logic must not live only inside the website.

---

# 15. GitHub result — no drift

When everything remains coherent:

```text
✓ IEPM Metadata

42 components recognized.
Metadata matches current package structure.
No semantic review required.
```

No bot comment.

No author action.

---

# 16. GitHub result — review suggested

When IEPM encounters uncertainty:

```text
⚠ IEPM Metadata — Review suggested

2 existing metadata assumptions may have changed.

[Review changes]
```

This should appear as a GitHub Check.

Only comment on the pull request if actual author attention is required.

---

# 17. GitHub result — major drift

For a large rewrite:

```text
⚠ IEPM Metadata — Major changes detected

18 components removed
27 components added
9 semantic claims can no longer be safely inherited

This release remains usable through IEPM,
but affected claims will become unverified
unless reviewed.

[Start review]
```

Do not block the release by default.

---

# 18. Signed review link

The Action generates a scoped review link:

```text
https://iepm.dev/review/<signed-token>
```

The token should identify:

* repository;
* package;
* commit SHA;
* ReviewBundle;
* metadata revision;
* expiration;
* allowed operation scope.

Possession of the URL alone should not authorize repository changes.

Before any PR creation:

* authenticate with GitHub;
* verify repository permissions;
* verify review token;
* verify commit/revision is still current.

---

# 19. Guided review UX

Lead with the meaning of the change, not YAML.

Example:

```text
Tactics Remix

IEPM detected that `smarter-mages`
no longer exists.

A new component named `mage-ai-v2`
appears structurally similar.

The old component was understood to:
• provide Mage AI
• conflict with other exclusive Mage AI providers

What happened?
```

Choices:

```text
○ Same component, renamed
○ New component replacing the old one
○ Old component was removed
○ Something else
○ Leave unresolved
```

Every structured question must contain:

```text
Something else
```

and:

```text
Leave unresolved
```

Never trap authors in an incomplete decision tree.

---

# 20. Natural-language escalation

If the author chooses:

```text
Something else
```

show:

```text
Describe what changed in your own words.
```

Example:

```text
I split the old mage AI into separate
prebuffing and spell-selection components.
Spell selection can coexist with SCS now,
but prebuffing still conflicts.
```

IEPM converts that into a proposed structured interpretation.

Example:

```text
Proposed interpretation

Old:
  smarter-mages
    deprecated

New:
  mage-prebuffing
    provides mage-ai.prebuffing
    exclusive mage-ai.prebuffing

  mage-spell-selection
    provides mage-ai.spell-selection
```

The author reviews the meaning.

They may optionally expand:

```text
View YAML diff
```

---

# 21. Complex rewrites should begin broad

Do not transform a major mod rewrite into 50 yes/no questions.

First detect that a major restructure occurred.

Show:

```text
Major package restructure detected.

Still valid:
✓ package identity
✓ artifact configuration
✓ supported game family

Needs review:
⚠ 23 component mappings
⚠ 7 capability declarations
⚠ 4 semantic relationships
```

Then ask:

```text
How would you describe this change?
```

Use the author's natural-language description to generate a proposed mapping.

Ask targeted follow-up questions only where ambiguity remains.

---

# 22. Review outcome

At the end:

```text
[ Looks good — create pull request ]

[ Ask IEPM maintainer to review ]

[ Leave unresolved ]
```

---

# 23. Looks good → PR

When the author approves:

1. generate the metadata patch;
2. show the final plain-English interpretation;
3. show the exact structured diff;
4. verify GitHub permissions;
5. create or update a metadata PR;
6. provide the PR link.

Never directly modify the repository's default branch.

Use a pull request.

Claims become `author-declared` when the metadata is actually merged upstream.

---

# 24. Ask IEPM maintainer

If the author is uncertain:

```text
Ask IEPM maintainer to review
```

Do not create a speculative upstream PR.

Create an IEPM ecosystem review item:

```text
Package:
Tactics Remix

Commit:
abc123

Author message:
"I split the AI system but I'm unsure how
your capability system should represent it."

State:
NEEDS_IEPM_REVIEW
```

An IEPM maintainer can:

* inspect the package;
* inspect previous metadata;
* research compatibility;
* generate a proposed interpretation.

Then the author receives another review request.

```text
author asks for help
      ↓
maintainer investigates
      ↓
maintainer proposes metadata
      ↓
author receives proposal
      ↓
author approves
      ↓
PR generated
```

---

# 25. Leave unresolved

This must be a legitimate outcome.

If the author chooses:

```text
Leave unresolved
```

only affected claims become:

```text
unverified
```

or:

```text
needs-review
```

Unaffected metadata remains valid.

The package remains installable where technically possible.

---

# 26. Authors describe their mod, not the whole ecosystem

Do not push BWS-style compatibility maintenance onto authors.

Authors should generally not have to enumerate:

```text
conflicts with mod A
conflicts with mod B
conflicts with mod C
conflicts with mod D
...
```

Prefer semantic declarations.

Example:

```yaml
components:
  mage-prebuffing:
    provides:
      - mage-ai.prebuffing

    exclusive:
      - mage-ai.prebuffing
```

Another mod independently declares the same exclusive capability.

IEPM derives the overlap.

Package-specific exceptions that cannot be represented semantically may remain in the external registry overlay.

---

# 27. Author metadata versus ecosystem metadata

Use this ownership principle:

```text
Author repository owns:
"What does my mod mean?"

IEPM registry owns:
"How does this mod interact with the wider ecosystem?"

Automated verification owns:
"What actually happened when we tested it?"
```

There will be overlap, but this should guide defaults.

---

# 28. Claim invalidation

New releases should invalidate only affected evidence.

Example release 8.2:

```text
artifact              verified
components            author-declared
EET compatibility     automated-verified
SCS relationship      author-declared
```

Major changes in 8.3 might yield:

```text
artifact              verified
components            mechanically-derived
EET compatibility     unverified-for-8.3
SCS relationship      needs-review
```

Do not reset the entire package to unknown.

---

# 29. Author approval is not runtime verification

Keep evidence types separate.

After author metadata merges:

```text
capability:
author-declared
```

Then automated install tests may produce:

```text
EET compatibility:
automated-verified
```

If automated testing contradicts an author declaration, store both.

Example:

```text
Author:
supports EET

Automated verification:
failed on fingerprint X
```

Do not silently mutate one into the other.

---

# 30. Automated verification pipeline

After metadata integration, IEPM may test releases against controlled fixtures.

Examples:

```text
clean BGEE
clean BG2EE
EET reference fixture
specific interoperability fixture
```

Successful verification adds evidence.

It does not rewrite author intent.

---

# 31. User-facing status

Normal users should see something simpler.

Example:

```text
Tactics Remix 8.3

✓ Artifact verified
✓ Components recognized
✓ Author metadata integrated
✓ BG2EE tested
⚠ EET interoperability partially verified

[ Install ]
```

Unverified packages remain installable unless IEPM knows of a concrete hard blocker.

Example:

```text
⚠ Compatibility for this release
has not been fully verified.

[ Install anyway ]
```

Do not hide the install button merely because evidence is incomplete.

---

# 32. Metadata PR behavior

Avoid PR spam.

Rules:

* one active IEPM metadata PR per logical release/update;
* new changes update the existing PR;
* do not create a PR for every Action run;
* use stable PR titles.

Example:

```text
IEPM metadata update for v8.3
```

The body should explain:

* what changed;
* what the author confirmed;
* remaining uncertainty;
* automated validation result.

---

# 33. GitHub comment behavior

Prefer GitHub Checks.

Only create a bot comment when author action is actually required.

Use one mutable comment per source PR.

Do not add a new comment after every push.

If the issue resolves, update or collapse the existing comment.

---

# 34. Rate limiting and abuse prevention

Protect both authors and IEPM infrastructure.

Use:

```text
one ReviewBundle
per repository + commit + metadata revision
```

Deduplicate identical analysis.

Apply reasonable:

* per-user limits;
* per-repository limits;
* review-generation limits;
* PR-generation limits.

Before PR creation:

* require GitHub authentication;
* verify repository permission;
* verify review token;
* verify commit/revision;
* check for an existing IEPM metadata PR.

Do not place powerful long-lived credentials inside review URLs.

---

# 35. Maintainer/admin interface

Build an ecosystem administration area in the SvelteKit application.

Useful views:

```text
Bootstrapping
Ready for upstream
Needs author review
Needs IEPM maintainer review
PR open
Author integrated
Verification pending
Verified
Stale
```

Example:

```text
Tactics Remix 8.3

Bootstrap             complete
Metadata completeness 94%
Upstream integration  yes
Review state          2 items
Verification          partial

[ Review bundle ]
[ Inspect diff ]
[ Research package ]
[ Send author review ]
[ Run verification ]
```

---

# 36. Adoption pipeline administration

Track outreach separately from mod metadata.

Potential states:

```text
not-contacted
ready-to-upstream
review-link-sent
author-reviewing
maintainer-review
pr-ready
pr-open
changes-requested
merged
declined
inactive
```

Do not encode these administrative states inside `bgmod.yaml`.

---

# 37. Community contributors

Do not make one IEPM maintainer the permanent bottleneck.

Eventually trusted contributors should be able to:

```text
select package
→ inspect bootstrap
→ research unresolved claims
→ prepare proposal
→ submit for IEPM review
```

Do not allow arbitrary users to create official-looking author PRs without appropriate trust or approval.

Contributor permissions can be added later.

---

# 38. Local/CLI fallback

The hosted workflow is convenience infrastructure.

Support equivalent local operations:

```text
iepm metadata validate
```

```text
iepm metadata review review-bundle.json
```

```text
iepm upstream prepare
```

The same underlying review engine should power:

* CLI;
* desktop app;
* website.

---

# 39. GitHub integration

Prefer a GitHub App over personal access tokens.

Use the minimum permissions required.

Likely needs:

* repository metadata: read;
* contents: read;
* checks: write;
* pull requests: write when explicitly requested;
* narrow contents write access only when needed for PR branch creation.

State-changing actions require explicit authorized user action.

---

# 40. Website responsibilities

The SvelteKit system may provide:

## Public package UI

* support status;
* verification status;
* metadata viewer;
* evidence/provenance view.

## Authenticated author UI

* pending reviews;
* guided wizard;
* natural-language explanation;
* proposed metadata;
* diff review;
* PR creation;
* past review decisions.

## Maintainer UI

* unresolved queue;
* bootstrap queue;
* author requests;
* stale metadata;
* verification failures;
* PR status;
* adoption tracking.

## Backend

* GitHub authentication;
* GitHub App operations;
* signed review tokens;
* ReviewBundle persistence;
* rate limiting;
* PR generation;
* verification jobs;
* audit history.

---

# 41. Durable metadata versus hosted operational state

The website database is **not the canonical metadata source**.

Durable mod metadata belongs in:

```text
author Git repository
```

and/or:

```text
IEPM registry Git repository
```

The hosted service may store operational state such as:

* review sessions;
* author responses;
* OAuth/session data;
* adoption pipeline status;
* pending maintainer reviews;
* PR tracking;
* rate-limit counters;
* verification job history.

Final metadata changes should become Git commits.

---

# 42. Auditability

Every semantic metadata change should be explainable.

Retain enough history to understand:

* old claim;
* new claim;
* source package commit;
* ReviewBundle;
* user/maintainer decision;
* provenance change;
* resulting PR;
* verification outcome.

Compatibility knowledge will evolve over many years.

Do not make it impossible to reconstruct why a claim exists.

---

# 43. Plain-English diff

Before generating a PR, always show the semantic effect.

Example:

```text
This metadata update tells IEPM:

• `smarter-mages` was split into two components.
• mage-prebuffing remains exclusive with other prebuffing systems.
• mage-spell-selection can coexist with other mage AI providers.
```

Then offer:

```text
View structured diff
```

Example:

```yaml
components:
  mage-prebuffing:
    provides:
      - mage-ai.prebuffing
```

Plain language first.

Structured representation second.

---

# 44. Failure behavior

If any hosted service becomes unavailable:

```text
website unavailable
GitHub App unavailable
verification runner unavailable
```

packages must remain installable.

At worst:

* verification becomes stale;
* author review waits;
* registry remains last-known-good;
* affected claims remain unverified.

The hosted service must not become required infrastructure for ordinary IEPM installation.

---

# 45. Rollout order

Implement progressively.

## Phase 1 — Metadata language + local tooling

* stabilize `bgmod.yaml`;
* JSON Schema;
* strong diagnostics;
* `iepm metadata validate`;
* ReviewBundle model;
* drift detection;
* local review generation.

This phase proves the semantics.

## Phase 2 — GitHub Action

* validation on push/PR;
* package drift detection;
* GitHub Checks;
* ReviewBundle artifacts;
* no hosted wizard required yet.

Direct-editing authors can already fully participate here.

## Phase 3 — SvelteKit guided review

* signed review URLs;
* GitHub authentication;
* plain-English wizard;
* natural-language interpretation;
* final diff;
* create/update metadata PR.

## Phase 4 — Maintainer escalation

* Ask IEPM maintainer flow;
* admin review queue;
* proposal return to author;
* adoption dashboard;
* abuse/rate-limit hardening.

## Phase 5 — Automated verification

* controlled game fixtures;
* package install verification;
* EET fixtures;
* interoperability tests;
* verification evidence.

## Phase 6 — Community scale

* contributor workflows;
* broader author outreach;
* automated release drift detection;
* upstream metadata adoption campaigns.

Do not build the hosted platform before the metadata language, ReviewBundle, and drift detector are solid.

---

# 46. Success criteria

For a power-user modder:

```text
change mod
→ edit bgmod.yaml
→ push
→ IEPM CI passes
→ done
```

For an author who does not want to edit metadata:

```text
change mod
→ IEPM detects ambiguity
→ GitHub says review needed
→ click review link
→ approve plain-English interpretation
→ metadata PR generated
→ merge
→ done
```

For a difficult rewrite:

```text
change mod
→ IEPM detects major drift
→ author explains change naturally
→ IEPM proposes metadata

OR

author selects:
Ask IEPM maintainer
```

For a non-participating author:

```text
new release
→ IEPM derives what it can
→ uncertain claims become unverified
→ users may still install
```

All four cases must work.

---

# 47. Core principles

1. **Direct `bgmod.yaml` editing is first-class.**
2. **The guided wizard is an alternative editor for the same metadata.**
3. **Maintainer assistance is the escape hatch for difficult semantics.**
4. **Author participation is optional.**
5. **Unknown does not mean unsupported.**
6. **Metadata drift reduces confidence rather than breaking installation.**
7. **If the author already updated valid metadata, do not force a review.**
8. **Authors describe their own mod, not the whole ecosystem.**
9. **Machine-derived facts remain machine-derived.**
10. **Cross-mod knowledge primarily belongs in the ecosystem overlay.**
11. **Author declarations and automated evidence are different things.**
12. **Git is the durable metadata source of truth.**
13. **Hosted services provide convenience, not fundamental availability.**
14. **Every guided question needs `Something else`.**
15. **Every guided workflow needs `Leave unresolved`.**
16. **Do not block mod releases by default.**
17. **Do not spam GitHub comments or PRs.**
18. **Natural language is the primary escalation path before manual YAML.**
19. **Complex changes should escalate to human review rather than force incorrect structure.**
20. **The authoring language should be useful even if the IEPM website disappears.**

The intended long-term ecosystem is:

```text
bootstrap existing mod
       ↓
machine derives structure
       ↓
IEPM/community fills remaining gaps
       ↓
author optionally takes ownership
       ↓
author edits metadata directly
          OR
       uses guided review
       ↓
automated testing adds evidence
       ↓
new releases require progressively less centralized maintenance
```

The end goal is not merely broad mod coverage.

The end goal is an ecosystem in which IEPM becomes **easier to maintain as adoption increases**, because package-local knowledge increasingly lives with the packages themselves while IEPM focuses on resolution, verification, and genuinely ecosystem-level relationships.
