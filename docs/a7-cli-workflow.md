# A7: CLI workflow and managed workspaces

A7 makes the existing Product A flow usable without weakening its safety
boundaries. The CLI now has four groups of operations:

- discover intent with `search` and preview explicit edits with `add`;
- validate a portable lockfile with `verify` and prepare archive bytes with
  `fetch`;
- create managed full-copy source snapshots and disposable workspaces;
- run the existing A5 executor (also available as `iepm install`) and seal a
  successful output as a separate full copy.

## Intent and preflight

```text
iepm search hidden --game bg2ee
iepm add --registry registry --manifest modpack.yaml \
  --package hidden-gameplay-options --environment eet-target \
  --component enable-debug-mode
iepm verify --lockfile modpack.lock.json --require-executable
```

`search` displays curated metadata only; declared games and provenance are not
an independent compatibility claim. `add` canonicalizes exact aliases, never
uses lineage as a replacement, requires an explicit environment for
multi-environment manifests, and runs resolver validation before displaying a
preview. It writes only with `--write`; writing normalizes YAML and therefore
does not preserve comments. `verify` checks portable lockfile references and
readiness only. It never fetches bytes, so `fetch` remains the content-hash
verification and preparation step.

## Full-copy workspace lifecycle

Use a local store outside your Steam/GOG installation. Each command refuses to
overwrite an existing destination.

```text
# The command prints the immutable IEPM snapshot path, including its measured
# core-layout fingerprint. This reads the source and makes a full copy.
iepm snapshot --source C:\\Steam\\BGEE --store C:\\IEPM\\store \
  --name bgee-clean --locale en_US
iepm snapshot --source C:\\Steam\\BG2EE --store C:\\IEPM\\store \
  --name bg2ee-clean --locale en_US

# Substitute each snapshot path printed above. This creates full copies under
# C:\\IEPM\\store\\workspaces\\eet-trial-1\\.
iepm workspace --store C:\\IEPM\\store --build eet-trial-1 \
  --snapshot bgee-source=<bgee-snapshot-path> \
  --snapshot eet-target=<bg2ee-snapshot-path>

iepm install --lockfile modpack.lock.json --cache C:\\IEPM\\cache \
  --weidu C:\\tools\\weidu.exe \
  --workspace bgee-source=C:\\IEPM\\store\\workspaces\\eet-trial-1\\bgee-source \
  --workspace eet-target=C:\\IEPM\\store\\workspaces\\eet-trial-1\\eet-target \
  --log-dir C:\\IEPM\\store\\logs\\eet-trial-1 --confirm-disposable

# This requires a completed execution receipt and completed managed workspace
# states; it then makes a separate full copy under store\\builds.
iepm seal --workspace-root C:\\IEPM\\store\\workspaces\\eet-trial-1 \
  --store C:\\IEPM\\store --name zach-eet-2026-09 \
  --log-dir C:\\IEPM\\store\\logs\\eet-trial-1
```

An IEPM source snapshot is immutable **to IEPM**: it is marked `source-snapshot`
and cannot be passed to `execute`. A managed workspace begins `ready`, becomes
`running` before mutation, and becomes `completed` only after the final A5
receipt is written. Any interruption or execution failure leaves it
non-reusable; create a fresh workspace instead. A sealed build is likewise
refused as an execution target. The marker is an IEPM safety contract, not an
operating-system ACL against unrelated tools, so keep the store under normal
user control.

## Intentionally deferred: checkpoints

No prefix/checkpoint cache is created in A7. A correct reusable checkpoint
needs a full identity over source snapshots, resolved prefix, artifacts,
installer inputs, WeiDU/toolchain behavior, and execution order—not merely the
small core-layout fingerprint used for pre-mutation checks. Until that evidence
and retention model exist, a fresh full-copy workspace is the reliable route.
Native copy-on-write clones may later optimize the same source → workspace
contract. A virtual filesystem and ordinary hardlinks are deliberately out of
scope: either would add filesystem semantics that can invalidate real WeiDU
behavior or mutate a supposedly immutable source.
