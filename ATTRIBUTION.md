# Attribution

Every non-code asset in this repository, with its author, source and license. terraLauncher's own code and original assets are MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/brand/terrapen.svg` | terraPen mark, shown on the web app and product cards | terraPen | The site icon of [terrapen.xyz](https://terrapen.xyz), copied unmodified | terraPen's own mark, not MIT/Apache; used to link to terraPen's pages |
| `assets/app-icon/` (all files) | terraLauncher app icon and tray icons (placeholder mark) | terraLauncher contributors | Original work, drawn by `assets/app-icon/make-icons.ps1` to match `terralauncher.svg` | MIT OR Apache-2.0 |
| `assets/app-icons/terragen.png` | terraGen app icon, 128 px | terraGen authors | [`shaboomi/terraGen`](https://github.com/shaboomi/terraGen) `desktop/src-tauri/icons/128x128.png`, copied unmodified | Used with the owner's permission (same owner as this repository) |
| `assets/app-icons/terraforge.png` | terraForge app icon, 128 px | terraForge authors | [`theworkisthework/terraForge`](https://github.com/theworkisthework/terraForge) `build/icon.png`, scaled from 512 px | MIT |
| `assets/social/linkedin.svg` | LinkedIn logo, unused | Fonticons, Inc. ([Font Awesome Free](https://fontawesome.com) 6.7.2) | Font Awesome Free `svgs/brands/linkedin.svg`; white fill and a square view box, see [`assets/social/README.md`](assets/social/README.md) | CC BY 4.0. The logo is a trademark of LinkedIn Corporation. |
| `assets/social/x.svg`, `youtube.svg`, `tiktok.svg`, `instagram.svg`, `discord.svg` | X, YouTube, TikTok, Instagram and Discord logos, for links to social accounts | [Simple Icons](https://simpleicons.org/) contributors | Simple Icons, `fill` set to white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0. The logos are trademarks of X Corp., Google LLC, TikTok / ByteDance, Meta Platforms, Inc. and Discord Inc. |
| `assets/social/github.svg`, `assets/social/globe.svg` | GitHub logo (linking to the source) and a globe (linking to the website) | [Simple Icons](https://simpleicons.org/) contributors; [Lucide](https://lucide.dev) contributors | Simple Icons `github.svg` and Lucide `globe.svg` (lucide-static 0.468.0), white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0 (GitHub logo; a trademark of GitHub, Inc.); ISC (globe) |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (body text) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (headings and app names) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (labels and buttons) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |
| (dependency) `epaint_default_fonts/fonts/NotoEmoji-Regular.ttf` | Noto Emoji Regular, monochrome emoji fallback for arbitrary release-note, feed and alert text | The Noto Project Authors (Google) | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts> | SIL OFL 1.1, `epaint_default_fonts/fonts/OFL.txt`; see also the dependency `README` and egui `README.md` |
| (dependency) `epaint_default_fonts/fonts/emoji-icon-font.ttf` | emoji-icon-font, monochrome symbol/emoji fallback | John Slegers | Bundled transitively by `egui 0.37`'s `monochrome_emoji_fonts` feature from `epaint_default_fonts`; <https://github.com/jslegers/emoji-icon-font> | MIT, Copyright (c) 2014 John Slegers, `epaint_default_fonts/fonts/emoji-icon-font-mit-license.txt` |
| (dependency) `epaint_default_fonts/fonts/egui-icons.ttf` | egui-icons, five-glyph subset of emoji-icon-font | John Slegers | [egui 6d518fc7](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts); included by default_fonts independently of monochrome_emoji_fonts | MIT, Copyright (c) 2014 John Slegers; full upstream text in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Ubuntu-Light.ttf` | Ubuntu Light, default font fallback | Canonical Ltd. | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | Ubuntu Font Licence 1.0; full upstream text and copyright in `NOTICE` |
| (dependency) `epaint_default_fonts/fonts/Hack-Regular.ttf` | Hack Regular, default monospace fallback | Source Foundry Authors; Bitstream Inc.; DejaVu contributors | [Pinned egui source](https://github.com/storytold/egui/tree/6d518fc799681571963842f95b55a6b68431b15a/crates/epaint_default_fonts/fonts) | MIT and Bitstream Vera License; DejaVu work public domain; full upstream notices in `NOTICE` |

## Documentation and README

| Path | Title | Author | Source | License |
|---|---|---|---|---|

## Hosted catalog verification

| Path                                                 | Title                                     | Author        | Source                                               | License           |
| ---------------------------------------------------- | ----------------------------------------- | ------------- | ---------------------------------------------------- | ----------------- |
| `crates/core/catalog.toml` | Bundled launcher catalog | terraLauncher contributors | Original work | MIT OR Apache-2.0 |
