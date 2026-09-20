# Foundational guardrails

> This is a compact companion to the authoritative
> [living project context](../LIVING_CONTEXT.md). Keep the durable rationale
> and current project state there; retain this page as a focused checklist.

This document records the architecture review decisions that future maintainers
and coding agents must preserve. It is deliberately a constraint list, not a
wish list for a larger package manager.

## What is stable

IEPM is a deterministic package-management layer above WeiDU. Its stable
boundaries are: human manifest intent, curated registry facts, a canonical
resolved lockfile, and a future semantic layer that Product A does not require.

An EET build is an execution graph across named game environments. Package
phases are useful ordering barriers within one environment, but are never a
substitute for graph edges. The source-to-target EET transform is the reference
case for this rule.

Public component intent uses stable IEPM IDs. Registry releases map that intent
to the release-specific WeiDU selector. Public release identity is opaque;
SemVer is optional solver metadata, not a normalization requirement.

Artifacts are identified by hash, not by URL. Acquisition and materialization
are separate operations. Registry provenance records the strength of claims;
structural extraction, author documentation, community knowledge, and an exact
verified install are distinct evidence levels.

## Execution boundary

Resolution answers whether IEPM can describe a candidate graph. Execution
answers whether it can safely reproduce it. A lockfile marked `analysis-only`
may be useful for review but must never proceed to A5 mutation. `executable`
requires a concrete artifact, installer route, selected-component WeiDU mapping,
all required portable installer inputs, and an execution plan.

Environment names are portable; local paths are machine configuration. A
portable lockfile must not record an absolute game path, user profile path, or
secret. For example, record `source-environment: bgee-source`, then let local
configuration bind `bgee-source` to a game directory.

## Growth policy

Add a new schema field, relationship condition, capability convention, or
provenance class only when an actual documented ecosystem case cannot be
represented by the existing model. Include that case as a regression fixture.

Aliases represent an exact identity rename and may be canonicalized during
resolution. Forks, predecessors, and continuations are lineage information;
they are never an automatic substitute. Do not let `lineage` become a package
replacement mechanism.

Keep the Rust implementation straightforward. The domain has many necessary
data distinctions already; avoid adding generic abstractions merely to mirror
them. Prefer concrete record, request, resolved-package, and execution-node
types.

## A5 acceptance path

1. Generate a non-mutating `iepm plan`/preflight from a lockfile.
2. Compare its ordered commands and environment bindings with a manual install.
3. Reject `analysis-only` lockfiles with causal blocking reasons.
4. Execute only in disposable, IEPM-controlled build workspaces.
5. Add a regression fixture for every new real-mod exception before generalizing
   the data model.
