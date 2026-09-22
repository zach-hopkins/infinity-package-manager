# Compatibility and evidence

Compatibility is release-specific. “Package X supports BG2EE” is insufficient;
the useful claim is “Package X release 1.2 supports BG2EE 2.6.6, declared by
the author” or “verified by this fixture.”

Claims without evidence are not failures, but must be surfaced as unverified.
The resolver should preserve that distinction rather than projecting certainty.
The normative, machine-enforced thresholds for the four player-facing statuses
are defined in the [verification status policy](verification-policy.md).

## Verification scope: prove claims, not universes

Automated verification may establish strong support before an author ships IEPM
metadata. It must, however, record the **smallest context that the observed
claim actually needs**. Do not turn one successful large mod stack into a
package-wide or ecosystem-wide green badge.

For example, a successful Forge EET run can establish a bounded claim such as:

```text
SCS v35.21 / smarter-mages installed on EET 14.1
with game fingerprint X and WeiDU 25100.
```

It must not imply that SCS depends on every other package that happened to be
present in that fixture. Those packages are ambient fixture state unless the
test or declared metadata identifies a causal interaction.

Keep these layers separate:

1. **Component execution** — an exact release/component installs on a named
   game environment under a named toolchain.
2. **Package-local composition** — a specific selection of components from one
   package installs together.
3. **Direct interoperability** — two or more components coexist when there is
   a known, documented, or observed interaction edge.
4. **Reference-stack receipt** — an exact whole stack completed. This is a
   valuable reproducible regression build, not the root of every package's
   support status.

Each verification run should retain two artifacts:

- a full execution receipt: exact environment fingerprints, all selected
  packages/releases/components, graph order, toolchain, commands, and logs;
- extracted verification claims: the minimal component, package-local, or
  interaction facts that the run actually proves.

The first artifact makes the outcome auditable. The second prevents unrelated
fixture state from becoming accidental dependencies.

When a new release appears, invalidate narrowly. A structural change in one
Tweaks component should make the relevant component and interaction claims
unverified for that new release; it must not erase a separately established
SCS-on-EET execution claim. Preserve the historical evidence for the old
release, derive safe structural facts again, and schedule only the changed
component or known interaction edge for re-verification.

This keeps verification additive and tractable:

```text
package/component evidence
  + meaningful interaction-edge evidence
  + a small number of reference-stack receipts
  = confidence
```

It does **not** require testing every subset of every mod stack. Future
semantic analysis may make impact-based retesting more precise, but Product A
already follows this rule by testing packages individually, testing only known
edges, and retaining representative full-stack builds.
