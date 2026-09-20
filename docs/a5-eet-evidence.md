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
- The EET archive retrieved from the source tag `v14.1` supplies the BGEE source path
  through `--args-list sp <path>`. IEPM records the portable
  `source-environment` name in the lockfile and renders a runtime binding
  placeholder instead of an absolute path.
- A clean disposable run completed all five selected components with shared
  WeiDU 251. EET core returned exit code `3` after `INSTALLED WITH WARNINGS`,
  but its requested component was recorded in `WeiDU.log`; EET_End then
  completed normally. The EET TP2 in the hash-verified source-tagged archive
  declares `VERSION ~v14.0~`, so a source tag, archive hash, and TP2 display
  version must remain distinct facts.
- EET_End's output reported an embedded `EET/bin/win32/x86_64/weidu.exe` at
  version `24900`. The shared WeiDU 251 lockfile value identifies IEPM's
  top-level invocation, not every executable a package may spawn internally.

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

The A5 `execute` command binds every named environment to an existing local
workspace, requires `chitin.key`, rejects nested/duplicate bindings and
baseline environments, copies only verified extracted content without
overwriting an existing materialized path, and retains one command/stdout/stderr
receipt per action. Mutation additionally requires `--confirm-disposable`.
`--allow-weidu-warnings` is deliberately separate: it accepts only exit code
`3` when the output says `INSTALLED WITH WARNINGS` *and* all requested numeric
components are independently present in `WeiDU.log`; every other nonzero exit
remains fatal.

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
v2.1, EE Fixpack Beta 2, and the source-tagged EET/EET_End archive v14.1. The fixture's game
fingerprints are synthetic test values and must be replaced by measured
disposable-workspace fingerprints before any future execution.

## Manual comparison status

The rendered five-package route was compared read-only with the local
known-good runner and its logs. The component order and numeric selections
match: source DLC Merger `#0 #1`, source EE Fixpack `#0 #0`, target EE Fixpack
`#0 #0`, EET core `#0 #0`, then EET_End `#0 #0`. The plan now also renders the
runner's workspace, locale, and unattended shared-WeiDU arguments for every
TP2-only route.

The plan has now completed in a disposable workspace using the separately
hash-verified source-tagged v14.1 archive. Its resulting target log records
the TP2's own v14.0 display value, then EET_End standard component `#0 #0`.
The run is evidence for this exact archive, command envelope, and component
sequence—not a blanket compatibility claim for another EET release or a full
mod stack. The resulting workspace fingerprint is still outstanding because
the fixture's fingerprint profile is intentionally only a synthetic example.
