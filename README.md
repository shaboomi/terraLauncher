<p align="center">
  <img src="assets/app-icon/hicolor/128x128/apps/xyz.terrapen.terralauncher.png" width="96" height="96" alt="terraLauncher icon">
</p>

<h1 align="center">terraLauncher</h1>

<p align="center">One place to install, update and open the terraPen apps, with links to the web apps and hardware.</p>

> [!IMPORTANT]
> Early work in progress. See [Status](#status).

## Downloads

**terraLauncher 0.1.0** — [all releases](https://github.com/shaboomi/terraLauncher/releases)

| System | Download |
|---|---|
| Windows 10/11 (64-bit) | [terralauncher-0.1.0-windows-x64-portable.zip](https://github.com/shaboomi/terraLauncher/releases/download/v0.1.0/terralauncher-0.1.0-windows-x64-portable.zip) |
| macOS 11+ (Apple silicon and Intel) | [terralauncher-0.1.0-macos-universal.dmg](https://github.com/shaboomi/terraLauncher/releases/download/v0.1.0/terralauncher-0.1.0-macos-universal.dmg) |
| Linux (x86_64) | [terralauncher-0.1.0-linux-x86_64.tar.gz](https://github.com/shaboomi/terraLauncher/releases/download/v0.1.0/terralauncher-0.1.0-linux-x86_64.tar.gz) |

Checksums: [SHA256SUMS.txt](https://github.com/shaboomi/terraLauncher/releases/download/v0.1.0/SHA256SUMS.txt)

- **Windows:** unzip and run `terralauncher.exe`. It is not code-signed, so Windows may warn about an unknown publisher. Settings stay in the unzipped folder.
- **macOS:** open the disk image and drag terraLauncher to Applications. It is not notarized, so the first time, right-click the app and choose Open.
- **Linux:** `tar -xzf terralauncher-0.1.0-linux-x86_64.tar.gz --strip-components 1 -C ~/.local`, then run `terralauncher`.

The builds come from [`.github/workflows/release.yml`](.github/workflows/release.yml): pushing a tag
`v<version>` that matches the version in `Cargo.toml` builds all three and publishes the release.

## What it shows

**Apps** — installed, updated and opened from the launcher, using each app's GitHub releases.

| App | What it's for | Source |
|---|---|---|
| **terraGen** | Generative plotter art | [shaboomi/terragen-releases](https://github.com/shaboomi/terragen-releases) |
| **terraForge** | Plotter control and G-code generator | [theworkisthework/terraForge](https://github.com/theworkisthework/terraForge) |

**Web apps** — opened in your browser.

| App | What it's for | Link |
|---|---|---|
| **terraLens** | Camera to plotter line art | [plotterlens.terrapen.xyz](https://plotterlens.terrapen.xyz/) |

**Products** — opened in your browser.

| Product | What it is | Link |
|---|---|---|
| **terraPen** | Open-source A2 pen plotter | [terrapen.xyz](https://terrapen.xyz/products/terrapen-s-a2-pen-plotter) |
| **terraDial** | Touchscreen dial for FluidNC plotters | [terrapen.xyz](https://terrapen.xyz/products/terradial) |

## How installs work

The launcher asks GitHub for each app's newest release, picks the file for your system, checks it
against the release's SHA-256 (`SHA256SUMS.txt`, or the digest GitHub records for the file), and
then runs it.

| System | Package | Notes |
|---|---|---|
| Windows | The app's setup program (`…-setup.exe`, `….Setup.<version>.exe`) | Run with the switches in the catalog (`setup_args`) so it shows at most a progress window. Installed copies are found by their uninstall record. |
| macOS | `.dmg` | Only for apps with a `mac_team` in the catalog (the Apple Developer team that signs the build). Neither app has one recorded yet, so on macOS the launcher opens a copy that is already installed but installs nothing. |
| Linux | `.AppImage`, `.deb` | Not tested yet. |

## Changing the catalog

- Apps: [`crates/core/catalog.toml`](crates/core/catalog.toml). The comments at the top explain each field.
  A new app also needs a 128 px icon in `assets/app-icons/<id>.png` and a line in `app_icon_png`
  in [`crates/core/src/catalog.rs`](crates/core/src/catalog.rs).
- Web apps and products: `WEB_APPS` and `PRODUCTS` in [`crates/core/src/catalog.rs`](crates/core/src/catalog.rs).

## Build from source

You need a recent stable Rust.

```sh
cargo run -p artcraft-launcher            # the desktop app (binary: terralauncher)
cargo test -p artcraft-launcher-core      # core tests
```

The crates keep their upstream package names (`artcraft-launcher-*`) so changes can still be
merged from upstream; only the binary and everything a user sees are renamed.

## Status

Working, checked on Windows 11:

- Lists terraGen and terraForge, finds installed copies and their versions, and offers updates.
- Updating terraForge from the launcher.
- Web apps and Products sections, opening in the browser.
- No connection to ArtCraft's servers: the hosted catalog, release feed and launcher self-update are off.

Not done yet:

- **Only updating has been run end to end**: terraForge 1.5.1-pen-preview.1 to 1.5.1 on Windows 11
  (download, checksum, silent install, re-detection). A fresh install, uninstall and the Open
  button have not been run yet, and nothing has been run for terraGen.
- terraGen's source repository is private, so the launcher reads its releases from the public
  [shaboomi/terragen-releases](https://github.com/shaboomi/terragen-releases). Updating terraGen
  from the launcher has not been run yet.
- macOS installs need each app's Apple Developer team ID in the catalog.
- From the next release, Windows gets a per-user MSI installer (`packaging/windows/terralauncher.wxs`,
  built by `build-msi.ps1`); a local build installed and uninstalled cleanly on Windows 11, but the
  release workflow's MSI step has not run yet. macOS (dmg) and Linux (tar.gz) are still plain
  archives. Nothing is signed or notarized. Upstream's other installer scripts in `packaging/`
  (.deb, .rpm, AppImage, Flatpak) have not been adapted.
- Most of the upstream test suite still refers to the Craft apps and fails or needs rewriting.
- New interface text is English only; the 14 translation catalogs have not been updated for it.
- The app icon is a placeholder.

## Credits and license

terraLauncher is a fork of [ArtCraft Launcher](https://github.com/storytold/craft-launcher) by the
ArtCraft Team and its contributors, used under its MIT / Apache-2.0 license. It is not affiliated
with or endorsed by ArtCraft. The ArtCraft name, logo and marks are not part of that license and
have been removed from this fork.

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. Required
notices are in [NOTICE](NOTICE); bundled fonts and icons are listed in [ATTRIBUTION.md](ATTRIBUTION.md).
