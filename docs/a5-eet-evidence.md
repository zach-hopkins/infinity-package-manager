# A5 EET execution evidence

This note records the first local, known-good EET evidence used to constrain
A5. It is not an installation guide and does not authorize execution against
an existing game tree.

## Observed facts

- An EET source environment can contain a short `WeiDU.log` preparation
  history, including DLC Merger and EE Fixpack. The selected DLC Merger
  component differed between observed builds (`1` and `3`), so IEPM must never
  make either choice a global default.
- EET itself copies the source environment's `WeiDU.log` to the target as
  `WeiDU-BGEE.log`. That file is retained source provenance and compatibility
  input; it is not a second target-environment execution queue.
- The target's ordinary `WeiDU.log` remains a distinct target history. It may
  include pre-existing target preparation such as EE Fixpack before EET.
- A known-good runner invokes the shared WeiDU executable with the TP2 path,
  numeric `--force-install` values, and a numeric `--language` value. A
  package must not be assumed to ship a `setup-*.exe`; EE Fixpack is a concrete
  counterexample.
- EET v14.1's documented noninteractive route supplies the BGEE source path
  through `--args-list sp <path>`. IEPM records the portable
  `source-environment` name in the lockfile and renders a runtime binding
  placeholder instead of an absolute path.

## Contract implications

`GameEnvironment.baseline` is an ordered list of expected WeiDU log entries.
It is a preflight assertion, not an implicit package request. The current A5
plan renderer displays it once per environment and rejects any selected
component that also appears in that environment baseline; that conservative
failure prevents a hidden reinstall until the future executor has an explicit
verify-existing action.

Installer metadata supports only two launchers: a bundled program or the
pinned shared WeiDU toolchain. Required extra arguments are typed literal
tokens or named environment bindings, never a shell-command template. This is
the smallest extension supported by the EET evidence and preserves portable
lockfiles.

The EET v14.1 archive content identity now recorded in the registry is
`cb1451e7ef341672fcd71ef8aa5edb010134f880e1eb3d398ed42a9ed3d934a7`.
Its execution route is `EET/EET.tp2`, component `0`, English language index
`0`, and a toolchain launcher. This establishes route mechanics only. The
complete example stack remains analysis-only until every selected package has
its own verified artifact and release-specific selectors.
