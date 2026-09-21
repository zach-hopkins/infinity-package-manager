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

## Develop

Install Bun, then from this directory:

```text
bun install
bun run check
bun run tauri dev
```

`bun run build` verifies the static frontend build. `bun run tauri build`
builds the desktop application and installer bundles.
