# IEPM desktop

This is the thin desktop client for Infinity Package Manager. It is a Tauri 2
shell around a static SvelteKit SPA using TypeScript, Tailwind, and Bun.

The architecture boundary is intentional:

```text
SvelteKit UI -> Tauri commands/events -> iepm Rust crate -> resolver/registry/artifacts
```

The frontend collects paths and displays plans, warnings, progress, and
receipts. It does not select releases, resolve dependencies, interpret
compatibility, calculate fingerprints, or install mods.

This app is currently the Product A desktop foundation, not the completed
Product B launcher. Product B will evolve it toward Play / Mods / Settings,
profiles, pending changes, last-known-good builds, launch recipes, and external
save awareness only after the Product A completion gate passes. Those concepts
remain Rust-owned and GUI-independent. See
[`docs/product-b-launcher.md`](../../docs/product-b-launcher.md).

## First use

The desktop app asks once for an **IEPM library folder**. It keeps downloaded
artifacts, source snapshots, disposable workspaces, logs, and sealed mod
experiences there; it must not be a game folder. The choice is saved and can be
changed in Settings.

The app ships its registry and detects the common Steam BG:EE and BG2:EE
locations. A user can always use **Browse** to select clean game folders and an
IEPM YAML mod list. BG:EE / SoD is required only when that list has a BGEE
environment (such as EET). The optional **Mod Experience Name** is only a
friendly output name; IEPM assigns a safe one when it is blank.

By default, IEPM acquires the normal 64-bit Windows WeiDU v251 release from
the official release URL, checks its published SHA-256, and caches it under
the library. It records the executor version in the resulting lockfile. If
Windows Security blocks that download, IEPM does not bypass it. Settings has a
narrow advanced local override that accepts only an executable reporting
WeiDU 25100; that route is visibly less trustworthy because a loose executable
cannot be verified against the release archive hash.

## Develop

Install Bun, then from this directory:

```text
bun install
bun run check
bun run tauri dev
```

On Windows, ensure Bun's install folder (normally `%USERPROFILE%\\.bun\\bin`) is
on `PATH` before using `bun run tauri ...`; Tauri launches `bun` again for its
frontend build step. This is a local machine setup requirement, not a path the
repository should hard-code.

`bun run build` verifies the static frontend build. `bun run tauri build`
builds the desktop application and installer bundles.
