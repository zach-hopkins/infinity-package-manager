# A5 bundled-launcher evidence: Hidden Gameplay Options v5.1

This note records the second real A5 execution route. It is scoped to one
component in a fresh disposable Steam BG2EE 2.6.6 copy; it is not a general
compatibility claim or an installation guide.

## Verified route

- The official Windows v5.1 release asset is
  `win-A7-HiddenGameplayOptions-v5.1.zip` from the project's GitHub release.
  Its SHA-256 is
  `11ccf8555486a73bdd1072f311f995f193e2bfa0784bf4d169e77b0b7ea22b3e`.
- The archive contains `setup-HiddenGameplayOptions.exe` at its root and
  `HiddenGameplayOptions/HiddenGameplayOptions.tp2`.
- The selected stable IEPM component is `enable-debug-mode`, implemented by
  TP2 label `A7-HIDDENGAMEPLAYOPTIONS-OPTION_ENABLE_DEBUG_MODE` and numeric
  selector `10`; the TP2 declares version `5.1`.
- The final disposable action used the bundled launcher with
  `--language 0 --use-lang en_US --skip-at-view --no-exit-pause
  --noautoupdate --force-install 10`. `WeiDU.log` records component `#0 #10`,
  and the installer reports successful completion under WeiDU 25100.
- The final `iepm-core-layout-v1` output fingerprint was
  `bc00901965c5d19743f67da0a40f25875df7ee32fbc2c7c62462cf3f19b4fc7d`.

## Resulting contract

The first attempts intentionally exposed two executor mistakes without
touching a primary game installation: a bundled setup binary was sent
`--force-install` before its language, and it was not sent the game locale.
The installer repeatedly prompted for its game language in both cases. A
fresh-copy retry established the smallest general correction: all WeiDU
launchers require a locked game locale and receive numeric mod language, game
locale, unattended flags, then requested components. This is a process-envelope
fact, not a package-specific relationship or a new schema abstraction.
