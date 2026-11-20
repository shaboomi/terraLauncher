# AGENTS.md: guide for AI agents and contributors

terraLauncher installs, updates and opens the terraPen apps (terraGen, terraForge) and links to
the web apps and products. It is a fork of `storytold/craft-launcher` (ArtCraft Launcher), kept
mergeable with upstream: the `upstream` remote points there.

## What differs from upstream

- **Names.** Everything a user sees says **terraLauncher**; the binary is `terralauncher`, the app
  id `xyz.terrapen.terralauncher`. The crates keep upstream's package names
  (`artcraft-launcher-core`, `-native`, `-ui-egui`, the app package `artcraft-launcher`) so
  upstream changes still merge. Do not rename them piecemeal.
- **No ArtCraft marks.** The ArtCraft name, logo and "A" mark are trademarks and must not come
  back in a merge (`docs/brand/`, `assets/app-icon/` upstream, the header logo, the tray icon).
- **No ArtCraft services.** The hosted catalog (`core::hosted_catalog` only answers from the bundled
  catalog) and the release feed (`check_plan` in `native/src/backend.rs` never asks it) are off.
  GitHub is the only source.
- **Launcher updates.** The launcher checks its own repo (`shaboomi/terraLauncher`) and offers the
  new release as a download. Replacing itself in place stays off: `craft_update`'s launcher entry
  is ArtCraft's, and Windows only self-updates a signed copy.
- **Catalog.** `crates/core/catalog.toml` holds the installable apps, with two extra fields:
  `setup_args` and `prereleases`. Link-only cards are `WEB_APPS` and `PRODUCTS` in
  `crates/core/src/catalog.rs`, drawn by `crates/ui-egui/src/links.rs`.
- **Windows setup programs.** `PackageKind::Setup` / `InstallSource::Setup` cover the NSIS
  installers Tauri and electron-builder produce: asset names in `core/src/platform.rs`, detection
  in `native/src/detect.rs` (`setup_installs`), install and uninstall in `native/src/install.rs`
  (`run_setup`, `uninstall_setup`).

## Workspace map

```text
crates/core      pure and wasm-safe: catalog, versions, release parsing, picking an asset
crates/native    desktop only: HTTP, detecting installed apps, download, verify, install
crates/ui-egui   the egui UI
apps/artcraft-launcher       desktop shell (binary `terralauncher`)
apps/artcraft-launcher-web   the same UI in the browser (not adapted yet)
packaging/       upstream's packaging scripts (not adapted yet)
```

## Rules worth keeping

- Non-test code does not panic and has no `unsafe`; return errors instead.
- Everything from the network is hostile: downloads only from the app's own GitHub releases,
  verified against the release's SHA-256 before anything is run.
- Never run an installer with elevated rights silently.
- Every bundled asset has a row in `ATTRIBUTION.md`.
- Report only what ran: say what was tested and what was not.

## Known gaps

See "Status" in `README.md`.
