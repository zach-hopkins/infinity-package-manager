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
  a bound game directory, numeric `--force-install` values, a numeric
  `--language` value, the game locale (`--use-lang en_US`), and unattended
  flags (`--skip-at-view`, `--no-exit-pause`, and `--noautoupdate`). A package
  must not be assumed to ship a `setup-*.exe`; EE Fixpack is a concrete
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
pinned shared WeiDU toolchain. A shared-toolchain plan requires an environment
locale and renders its runtime workspace binding, numeric language index, and
the observed unattended command envelope. Required extra arguments are typed
literal tokens or named environment bindings, never a shell-command template.
This is the smallest extension supported by the EET evidence and preserves
portable lockfiles.

The EET v14.1 archive content identity now recorded in the registry is
`cb1451e7ef341672fcd71ef8aa5edb010134f880e1eb3d398ed42a9ed3d934a7`.
Its execution route is `EET/EET.tp2`, component `0`, English language index
`0`, and a toolchain launcher. This establishes route mechanics only. The
complete example stack remains analysis-only until every selected package has
its own verified artifact and release-specific selectors.

## Minimal executable route fixture

`examples/eet-minimal/modpack.yaml` describes the observed five-package base:

```text
BGEE: DLC Merger → EE Fixpack
BG2EE: EE Fixpack → EET import → EET_End (now running against EET)
```

The EET workspace begins as `bg2ee` and declares `after_eet_import: eet`.
This is a lifecycle transition, not an assertion that EE Fixpack supports EET:
its TP2 explicitly rejects EET, while EET_End explicitly requires EET core.
The registry records local-archive-verified content identities for DLC Merger
v2.1, EE Fixpack Beta 2, EET v14.1, and EET_End v14.1. The fixture's game
fingerprints are synthetic test values and must be replaced by measured
disposable-workspace fingerprints before any future execution.

## Manual comparison status

The rendered five-package route was compared read-only with the local
known-good runner and its logs. The component order and numeric selections
match: source DLC Merger `#0 #1`, source EE Fixpack `#0 #0`, target EE Fixpack
`#0 #0`, EET core `#0 #0`, then EET_End `#0 #0`. The plan now also renders the
runner's workspace, locale, and unattended shared-WeiDU arguments for every
TP2-only route.

The comparison is deliberately not marked as an EET v14.1 execution success:
the historical successful target log records EET v14.0, whereas IEPM pins the
separately hash-verified v14.1 archive. The manual evidence validates the
model and command shape; a disposable v14.1 run is still required to validate
the selected release and its resulting workspace fingerprint.
