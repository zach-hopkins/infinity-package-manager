# Personal-use starter builds

IEPM can now build the tested BG2EE starter fixtures from a clean full copy.
They are intended for personal experimentation on Windows, never for an
installed Steam/GOG game directory. Compatibility uncertainty is a warning;
a package is blocked only when IEPM lacks a concrete way to construct and
receipt its installation command.

## Tested fixtures

| Manifest | Exact components exercised | Scope of evidence |
| --- | --- | --- |
| `examples/bg2ee-popular-starter/modpack.yaml` | Hidden Gameplay Options install-all, IWDification bard songs, Rogue Rebalancing core, Spell Revisions main | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |
| `examples/bg2ee-content-starter/modpack.yaml` | Ascension rewritten final chapter, Call of the Lost Goddess core, Throne of the Mad God core | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |
| `examples/bg2ee-bubbs-spell-menu/modpack.yaml` | EEex bootstrap/main, Bubb's Spell Menu main | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |
| `examples/bg2ee-infinity-ui/modpack.yaml` | EEex bootstrap/main, Infinity UI++ core | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |
| `examples/bg2ee-tweaks-starter/modpack.yaml` | Tweaks: Icon Improvements, More Interjections, Remove XP Cap | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |
| `examples/bg2ee-scs-starter/modpack.yaml` | SCS: batch mode, spell tweaks batch, initialize AI, smarter mages/priests | One fresh BG2EE 2.6.6 English full-copy build, shared WeiDU 25100 |

These fixtures prove their listed exact selections only. They do not prove
every optional component, a full Forge order, gameplay behavior, or EET
compatibility. Tweaks uses its exact published v18 IEMOD artifact. SCS uses
the exact official Windows release only through the named WinRAR-SFX extraction
recipe established for v35.21; it uses `UnRAR.exe` to read the payload and
never runs the SFX stub. Install WinRAR or set `IEPM_UNRAR` to a local
`UnRAR.exe` path before executing an SCS fixture. The listed SCS selections
have fresh full-copy evidence; other optional components remain unverified.

## From a clean game copy

Use a new store outside the source game. Substitute your own paths and make a
copy of a manifest to choose different mods or components. If its environment
has no fingerprint, `build` measures the clean source and records the value in
the effective manifest and lockfile without rewriting your source file.

```powershell
iepm build --registry registry `
  --manifest examples\bg2ee-tweaks-starter\modpack.yaml `
  --source C:\Games\BG2EE-clean `
  --store C:\Games\IEPM `
  --build tweaks-trial-1 `
  --weidu C:\Games\WeiDU-Windows\weidu.exe `
  --weidu-version 25100 `
  --confirm-disposable
```

The finished game is under `C:\Games\IEPM\builds\tweaks-trial-1\target`.
Warnings about incomplete compatibility evidence are shown but do not block a
technically executable build.

The same work remains available as separate expert/debugging commands:

```powershell
iepm snapshot --source C:\Games\BG2EE-clean --store C:\Games\IEPM --name bg2ee-clean --locale en_US
iepm workspace --store C:\Games\IEPM --build starter-1 --snapshot target=C:\Games\IEPM\sources\bg2ee-clean\<fingerprint>
iepm resolve --registry registry --manifest examples\bg2ee-popular-starter\modpack.yaml --output starter.lock.json --weidu-version 25100
iepm plan --lockfile starter.lock.json
iepm install --lockfile starter.lock.json --cache C:\Games\IEPM\cache --weidu C:\Games\WeiDU\weidu.exe --workspace target=C:\Games\IEPM\workspaces\starter-1\target --log-dir C:\Games\IEPM\logs\starter-1 --confirm-disposable
iepm seal --workspace-root C:\Games\IEPM\workspaces\starter-1 --store C:\Games\IEPM --name starter-1 --log-dir C:\Games\IEPM\logs\starter-1
```

Run `iepm fingerprint` on a fresh workspace if its game build differs from
the fixture's pinned BG2EE 2.6.6 fingerprint, then make that value explicit in
your manifest before resolving. Read the plan before `install`; each completed
run saves command, stdout, stderr, WeiDU, and final-fingerprint receipts.
