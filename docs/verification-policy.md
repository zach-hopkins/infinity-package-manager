# Verification status policy

This document is the normative public-status policy for IEPM. The machine
contract is [`schemas/verification-evidence.schema.json`](../schemas/verification-evidence.schema.json),
and the Rust decision table is `VerificationRecord::assess` in `iepm-core`.
Changing a threshold requires changing all three in one reviewed commit.

The detailed evidence model remains deliberately richer than the words shown
to players. A normal user sees one of four statuses:

| Status | Exact meaning |
| --- | --- |
| **Verified** | This exact locked configuration completed a clean disposable build and the required launch smokes under the recorded environment. |
| **Supported** | IEPM has tested install routes for every selected part and can satisfy all known rules, but this exact locked combination has not passed every Verified gate. |
| **Untested** | IEPM can still attempt the install, but at least one required support fact has not been demonstrated. |
| **Incompatible** | Concrete evidence establishes that the request cannot work as stated. |

Unknown is **Untested**, never Incompatible. Untested and opaque packages lose
guarantees, not installability. These statuses describe installation and basic
startup confidence; they do not promise that every quest, dialogue, or combat
interaction has been played through.

## Fixed measurable gates

### Component becomes Supported

For one exact package release, component, game fingerprint, platform, and
WeiDU version, all of these must be true:

1. The downloaded artifact matched its recorded SHA-256.
2. The stable component ID has an executable release-specific selector.
3. Package-local and known cross-package requirements are modeled.
4. A clean IEPM-managed disposable install completed.
5. Every planned action completed.
6. Every requested recording component appears in `WeiDU.log`.
7. No requested component was skipped.
8. Every warning was retained and classified.
9. The game reached its main menu after installation.

Missing any item leaves that component Untested for that exact target. A
success on BG2EE does not automatically support EET, another release, or
another component.

### Release becomes Supported

For one exact release and target, all of these must be true:

1. Every artifact used by the release is SHA-verified.
2. The exposed component catalog is complete.
3. Every exposed component has an explicit Supported, Untested, or
   Incompatible state; none is silently absent.
4. Default/core components are Supported.
5. Known dependencies, conflicts, and ordering rules are modeled.

This does not claim every optional component combination is Supported.

### Relationship becomes Supported

A direct compatibility or ordering relationship needs exact participating
release identities and at least one of:

- a mechanically enforced fact;
- an author declaration; or
- a clean automated interaction fixture.

Community reports may motivate a test or warning, but do not alone satisfy
this gate. Broad package conflicts must not replace component-level facts.

### Configuration becomes Supported

One exact lockfile is Supported when:

1. Every artifact is SHA-verified.
2. Every selected component is Supported for its recorded target.
3. All known dependencies, conflicts, capabilities, and ordering constraints
   are satisfied.
4. No selected package is opaque or Untested.
5. The portable identity is complete: lock SHA-256, registry revision,
   platform, WeiDU version, environment fingerprints, release IDs, artifact
   hashes, component IDs, environment bindings, and resolved execution order.
   The registry revision must identify an exact committed state, not
   `working-tree`.

### Configuration becomes Verified

A Supported configuration becomes Verified after one exact evidence-bearing
run satisfies every additional gate:

1. A clean IEPM-managed disposable install completed.
2. Every planned action completed.
3. Every requested recording component appears in `WeiDU.log`.
4. No requested component was skipped.
5. Every warning was retained and classified.
6. EET_End completed when the configuration requires it.
7. The successful output was sealed.
8. The game reached its main menu.
9. A new-game or designated known-save smoke passed.

One passing run is enough for this exact identity. Repeated runs remain useful
internal reliability evidence, but do not create a fifth public badge.

### Configuration becomes Incompatible

Incompatible requires at least one of:

- a mechanical impossibility in the requested graph or selectors;
- mutually exclusive capabilities selected together;
- an author-declared incompatibility applying to these exact releases; or
- the same deterministic failure in at least two independent clean disposable
  runs.

A single failure, an old forum post, an unclassified warning, or missing
metadata is not enough.

## Maintainer working process

1. Resolve the manifest and retain the portable lockfile.
2. Run only in fresh IEPM-managed workspaces.
3. Retain the full build receipt, action logs, final fingerprints, warning
   receipts, and sealed-build receipt.
4. Perform the main-menu and new-game/known-save smokes; record pass/fail
   explicitly rather than inferring launch success from WeiDU.
5. Write a schema-1 portable evidence record. Use repository-relative paths or
   durable URLs in `evidence`; never write local absolute paths or secrets.
6. Assess it with:

   ```text
   cargo run -p iepm -- evidence-status --record path/to/evidence.json
   ```

   Add `--json` for automation.
7. Extract the smallest claims the run proves: component execution,
   package-local composition, direct interaction, and/or exact reference stack.
   Do not turn ambient packages into dependencies.
8. Update the registry claims and [`mod-support.md`](mod-support.md) in the same
   change. Preserve old evidence when a new release appears.

## Invalidation and retesting

Evidence keys are exact. A changed artifact hash, release ID, selected
component, environment fingerprint, platform, WeiDU version, registry rules,
or execution order means it is a different configuration and is not Verified
by the old run. Old evidence remains historically valid for its old identity.

Retest narrowly. A changed Tweaks component invalidates that component and its
known interaction edges, not an unrelated SCS component. Representative full
stacks are regression evidence, not a demand to test every permutation of the
ecosystem.
