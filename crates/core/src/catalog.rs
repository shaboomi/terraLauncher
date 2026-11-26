//! The app catalog: the bundled apps, plus the link-only web apps and products.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// Who makes an app. Official apps live under [`GITHUB_OWNER`]; a community app is maintained
/// outside ArtCraft and is labelled as such wherever it is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppKind {
    Official,
    Community,
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`photocraft`).
    pub id: &'static str,
    /// Display name (`PhotoCraft`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (`PdfCraft` shipped as `PrintCraft` until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (`PdfCraft`'s MSI said `PrintCraft` until 0.2.1).
    pub former_names: &'static [&'static str],
    /// The `.app` bundle stem on macOS when it differs from `name` (`ArtCraft` for `ArtCraft Studio`).
    pub mac_bundle_name: Option<&'static str>,
    pub kind: AppKind,
    /// Who publishes it, as the card's byline shows it (`ArtCraft`, `Tangency Labs`).
    pub publisher: &'static str,
    /// The publisher its Windows installer names, when it isn't ArtCraft's (`Learning Machines LLC`).
    pub msi_publisher: Option<&'static str>,
    /// The bundle id when it isn't `ai.storyteller.<id>` (a community app brings its own).
    pub bundle: Option<&'static str>,
    /// Older bundle ids its Flatpak shipped under (`SolidCraft` was `ai.storyteller.solidcraft`).
    pub former_bundles: &'static [&'static str],
    /// Its website, when it isn't a page on getartcraft.com (a community app has its own).
    pub site: Option<&'static str>,
    /// The Apple Developer team whose Developer ID signature its macOS build must carry. `None`
    /// while those builds carry no signature the launcher can check: it then opens a copy that's
    /// there and links to the releases, but installs nothing.
    pub mac_team: Option<&'static str>,
    /// What the Windows setup program (`.exe`) is run with so it needs no wizard (`/S`, or
    /// Tauri's `/P` for a progress window only).
    pub setup_args: &'static [&'static str],
    /// Whether the newest release is offered even when it is marked as a pre-release.
    pub prereleases: bool,
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        self.bundle.map_or_else(|| format!("{BUNDLE_PREFIX}.{}", self.id), str::to_string)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's website: its own site, or its repository.
    pub fn website(&self) -> String {
        self.site.map_or_else(|| self.repo_url(), str::to_string)
    }

    /// Whether the launcher installs this app from a `kind` package.
    pub fn installs(&self, kind: crate::PackageKind) -> bool {
        kind != crate::PackageKind::Dmg || self.mac_team.is_some()
    }

    /// Whether this app runs on the given operating system.
    pub fn runs_on(&self, os: crate::Os) -> bool {
        os != crate::Os::FreeBsd
    }

    /// Bundle ids this app may be installed under, current first.
    pub fn bundle_ids(&self) -> impl Iterator<Item = String> + '_ {
        std::iter::once(self.bundle_id()).chain(self.former_bundles.iter().map(ToString::to_string))
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    /// The primary `.app` bundle stem on macOS (`ArtCraft` or `PhotoCraft`).
    pub fn mac_bundle_name(&self) -> &'static str {
        self.mac_bundle_name.unwrap_or(self.name)
    }

    /// Bundle stems a macOS `.app` may carry (`ArtCraft` and `ArtCraft Studio`), release name first.
    pub fn mac_bundle_names(&self) -> impl Iterator<Item = &'static str> + '_ {
        let primary = self.mac_bundle_name();
        std::iter::once(primary)
            .chain(std::iter::once(self.name).filter(move |&n| n != primary))
            .chain(self.former_names.iter().copied().filter(move |&n| n != primary && n != self.name))
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        crate::hosted_catalog::apps().iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp {
        id,
        name,
        tagline,
        color,
        ink_light,
        ink_dark,
        repo,
        former_ids: &[],
        former_names: &[],
        mac_bundle_name: None,
        kind: AppKind::Official,
        publisher: PUBLISHER,
        msi_publisher: None,
        bundle: None,
        former_bundles: &[],
        site: None,
        mac_team: None,
        setup_args: &[],
        prereleases: false,
    }
}

// The bundled suite, in website display order, then the approved community apps.
include!(concat!(env!("OUT_DIR"), "/catalog.rs"));

/// Who publishes the apps, as a card's byline shows it.
pub const PUBLISHER: &str = "terraPen";
/// The reverse-DNS prefix of an app that names no bundle id of its own.
pub const BUNDLE_PREFIX: &str = "xyz.terrapen";

/// terraPen's website.
pub const WEBSITE_URL: &str = "https://terrapen.xyz";
/// The GitHub account the launcher itself lives under.
pub const GITHUB_OWNER: &str = "shaboomi";
/// The launcher's source.
pub const GITHUB_URL: &str = "https://github.com/shaboomi/terraLauncher";
/// Social accounts: (service, URL). None yet.
pub const SOCIAL: [(&str, &str); 0] = [];

/// There is no community server. The upstream plumbing that opens one stays (so merges from
/// upstream stay small) but the UI no longer offers it, and both answers are the website.
pub const DISCORD_URL: &str = WEBSITE_URL;
pub const DISCORD_APP_URL: &str = WEBSITE_URL;

/// A link the launcher shows as a card and opens in the browser: nothing is installed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Link {
    pub id: &'static str,
    pub name: &'static str,
    pub tagline: &'static str,
    pub url: &'static str,
    /// Brand colour (sRGB), for the card's hover wash and rule.
    pub color: [u8; 3],
    /// The brand colour for the name on a light background.
    pub ink_light: [u8; 3],
    /// The brand colour for the name on a dark background.
    pub ink_dark: [u8; 3],
}

/// terraPen's yellow-green (its mark's background), and a darker olive that reads on white.
const TERRAPEN: [[u8; 3]; 3] = [[229, 241, 92], [104, 114, 0], [229, 241, 92]];

/// Apps that run in the browser.
pub const WEB_APPS: [Link; 1] = [Link {
    id: "terralens",
    name: "terraLens",
    tagline: "Camera to plotter line art, in your browser",
    url: "https://plotterlens.terrapen.xyz/",
    // plotterLens's own green (its `theme-color`), lightened for a dark background.
    color: [62, 140, 76],
    ink_light: [23, 80, 34],
    ink_dark: [111, 199, 124],
}];

/// Hardware the apps are made for.
pub const PRODUCTS: [Link; 2] = [
    Link {
        id: "terrapen",
        name: "terraPen",
        tagline: "Open-source A2 pen plotter",
        url: "https://terrapen.xyz/products/terrapen-s-a2-pen-plotter",
        color: TERRAPEN[0],
        ink_light: TERRAPEN[1],
        ink_dark: TERRAPEN[2],
    },
    Link {
        id: "terradial",
        name: "terraDial",
        tagline: "Touchscreen dial for FluidNC plotters",
        url: "https://terrapen.xyz/products/terradial",
        color: TERRAPEN[0],
        ink_light: TERRAPEN[1],
        ink_dark: TERRAPEN[2],
    },
];

/// terraLaunch itself, described like an app so the UI can show its version with the same
/// machinery. Not part of [`SUITE`]; it does not update itself yet. It was called terraLauncher
/// until 1.0: copies installed under that name (the macOS `.app`, the registry keys the MSI
/// writes) are still found by it.
pub const LAUNCHER: CraftApp = CraftApp {
    former_names: &["terraLauncher"],
    ..app(
        "terralauncher",
        "terraLaunch",
        "Installs and updates the terraPen apps",
        [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
        "shaboomi/terraLauncher",
    )
};

/// The launcher's own repo.
pub const LAUNCHER_REPO: &str = "shaboomi/terraLauncher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "terralauncher";

/// A Craft App's 128 px icon (PNG), as copied from its repo into `assets/app-icons`: the UI
/// draws it, and the Linux install puts it in the desktop menu.
pub fn app_icon_png(id: &str) -> Option<&'static [u8]> {
    if let Some(icon) = crate::hosted_catalog::icon(id) {
        return Some(icon);
    }
    Some(match id {
        "terragen" => include_bytes!("../../../assets/app-icons/terragen.png"),
        "terraforge" => include_bytes!("../../../assets/app-icons/terraforge.png"),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_app_has_its_icon() {
        for app in &SUITE {
            let png = app_icon_png(app.id).unwrap_or_else(|| panic!("no icon for {}", app.id));
            assert!(png.starts_with(b"\x89PNG\r\n\x1a\n"), "{} icon isn't a PNG", app.id);
        }
        assert!(app_icon_png("notanapp").is_none());
    }

    #[test]
    fn suite_ids_are_unique_and_match_the_names() {
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.name.to_lowercase(), a.id);
            assert_eq!(a.repo.matches('/').count(), 1, "{} repo", a.id);
        }
    }

    #[test]
    fn terragen_is_found_under_the_name_its_installer_registers() {
        let app = CraftApp::by_id("terragen").unwrap();
        assert_eq!(app.repo, "shaboomi/terragen-releases");
        assert_eq!(app.names().collect::<Vec<_>>(), ["terraGen", "TerraGen"]);
        assert_eq!(app.bundle_id(), "app.terragen.desktop");
        assert_eq!(app.mac_bundle_name(), "TerraGen");
        assert!(app.prereleases);
        assert!(!app.installs(crate::PackageKind::Dmg));
        assert!(app.installs(crate::PackageKind::Setup));
    }

    #[test]
    fn links_open_in_the_browser() {
        for link in WEB_APPS.iter().chain(&PRODUCTS) {
            assert!(crate::release::safe_web_link(link.url), "{}", link.url);
        }
    }
}
