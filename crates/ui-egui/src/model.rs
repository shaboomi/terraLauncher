//! What the UI knows about each app.

use artcraft_launcher_core::platform::{Platform, pick_asset};
use artcraft_launcher_core::protocol::{JobPhase, LatestRelease, PausedDownload};
use artcraft_launcher_core::release::Release;
use artcraft_launcher_core::status::{self, AppStatus, Latest};
use artcraft_launcher_core::{CraftApp, Installed};

use crate::i18n::t;

#[derive(Clone, Debug)]
pub struct AppEntry {
    pub app: &'static CraftApp,
    pub latest: Option<LatestRelease>,
    pub installed: Option<Installed>,
    /// Detection has answered at least once.
    pub detected: bool,
    pub job: Option<JobPhase>,
    /// The service `latest` came from.
    pub origin: Option<artcraft_launcher_core::protocol::Origin>,
    /// A download waiting to be resumed.
    pub paused: Option<PausedDownload>,
    /// A finished download waiting for another try at installing it.
    pub ready: bool,
    /// The backend's word on whether an install of the latest release over the installed copy
    /// would go ahead ([`Event::Installable`](artcraft_launcher_core::protocol::Event::Installable));
    /// `None` where it gives none (the web build), and then [`Self::has_download`] decides. Only
    /// read while a copy is installed.
    pub installable: Option<bool>,
}

impl AppEntry {
    pub fn new(app: &'static CraftApp) -> Self {
        Self { app, latest: None, installed: None, detected: false, job: None, origin: None, paused: None, ready: false, installable: None }
    }

    pub fn release(&self) -> Option<&Release> {
        match &self.latest {
            Some(LatestRelease::Found(r)) => Some(r),
            _ => None,
        }
    }

    /// Whether the latest release has something for `platform`.
    pub fn has_download(&self, platform: Option<Platform>) -> bool {
        let (Some(r), Some(p)) = (self.release(), platform) else { return false };
        let stems: Vec<&str> = self.app.stems().collect();
        let kinds: Vec<_> = p.download_kinds().iter().copied().filter(|k| self.app.installs(*k)).collect();
        pick_asset(r, &stems, p, &kinds).is_some()
    }

    /// Whether an install of the latest release would go ahead here. Only the backend knows which
    /// package can replace an installed copy (an AppImage with an AppImage) and which copies are
    /// left to whatever installed them, so for an installed copy its answer decides
    /// ([`Self::installable`]); otherwise, whether the release has a download `platform` takes.
    pub fn installs_here(&self, platform: Option<Platform>) -> bool {
        match (&self.installed, self.installable) {
            (Some(_), Some(installable)) => installable,
            _ => self.has_download(platform),
        }
    }

    /// The newer release out for an installed copy that the launcher won't install over it.
    /// `None` unless a newer release is out, an install wouldn't go ahead, and the copy isn't one
    /// its repository updates (whose own latest the release feed doesn't know).
    fn newer_not_installable(&self, platform: Option<Platform>) -> Option<&Release> {
        let installed = self.installed.as_ref().filter(|i| !i.externally_managed)?;
        let release = self.release()?;
        let newer = release.version.as_ref().zip(installed.version.as_ref()).is_some_and(|(l, i)| artcraft_launcher_core::version::is_update(l, i));
        (newer && !self.installs_here(platform)).then_some(release)
    }

    /// The release page of a newer release the user can install by hand: the release has a
    /// package for `platform`, but one the launcher won't install for this app (a macOS build
    /// with no signing team recorded to check it against). It is shown as an update whose button
    /// opens this page instead of installing.
    pub fn manual_update(&self, platform: Option<Platform>) -> Option<&str> {
        let release = self.newer_not_installable(platform)?;
        let p = platform?;
        let stems: Vec<&str> = self.app.stems().collect();
        let refused: Vec<_> = p.download_kinds().iter().copied().filter(|k| !self.app.installs(*k)).collect();
        (!refused.is_empty() && pick_asset(release, &stems, p, &refused).is_some()).then_some(release.html_url.as_str())
    }

    /// The newer release an installed copy can't be updated to from here, so it isn't offered as
    /// an update: one the launcher won't install that isn't a [`manual update`](Self::manual_update)
    /// either.
    pub fn newer_out_of_reach(&self, platform: Option<Platform>) -> Option<String> {
        if self.manual_update(platform).is_some() {
            return None;
        }
        self.newer_not_installable(platform).map(Release::version_label)
    }

    pub fn status(&self, platform: Option<Platform>, manage_apps: bool) -> AppStatus {
        if manage_apps && !self.detected {
            return AppStatus::Checking;
        }
        let latest = match &self.latest {
            None => Latest::Pending,
            Some(LatestRelease::Failed(_)) => Latest::Failed,
            Some(LatestRelease::None) => Latest::NoRelease,
            Some(LatestRelease::Found(r)) => {
                Latest::Found { version: r.version.as_ref(), has_asset: self.installs_here(platform) || self.manual_update(platform).is_some() }
            }
        };
        status::status(self.installed.as_ref(), latest)
    }

    pub fn installed_label(&self) -> Option<String> {
        self.installed.as_ref().map(|i| i.version.as_ref().map(ToString::to_string).unwrap_or_else(|| t("Installed").to_string()))
    }

    pub fn latest_label(&self) -> Option<String> {
        self.release().map(Release::version_label)
    }

    pub fn failure(&self) -> Option<&str> {
        match &self.latest {
            Some(LatestRelease::Failed(why)) => Some(why),
            _ => None,
        }
    }
}

/// The colour role of a status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Good,
    Warn,
    Info,
    Neutral,
    Faint,
}

pub fn status_text(status: &AppStatus, manage_apps: bool) -> (String, Tone) {
    match status {
        AppStatus::Checking => (t("Checking…").to_string(), Tone::Faint),
        AppStatus::Available if manage_apps => (t("Available to install").to_string(), Tone::Info),
        AppStatus::Available => (t("Available").to_string(), Tone::Info),
        AppStatus::UpToDate => (t("Installed · up to date").to_string(), Tone::Good),
        AppStatus::UpdateAvailable => (t("Update available").to_string(), Tone::Warn),
        AppStatus::Installed => (t("Installed").to_string(), Tone::Neutral),
        AppStatus::Unavailable => (t("Not available here").to_string(), Tone::Faint),
        AppStatus::NoRelease => (t("Coming soon").to_string(), Tone::Faint),
    }
}

/// [`status_text`], but saying so when a download is paused or reconnecting.
pub fn entry_status_text(e: &AppEntry, status: &AppStatus, manage_apps: bool) -> (String, Tone) {
    match (e.job, e.paused) {
        (Some(JobPhase::Reconnecting { .. }), _) => (t("Reconnecting…").to_string(), Tone::Warn),
        (Some(JobPhase::Downloading { .. }), _) if status.is_installed() => (t("Updating…").to_string(), Tone::Info),
        (Some(JobPhase::Downloading { .. }), _) => (t("Downloading…").to_string(), Tone::Info),
        (None, Some(_)) if status.is_installed() => (t("Update paused").to_string(), Tone::Warn),
        (None, Some(_)) => (t("Download paused").to_string(), Tone::Warn),
        _ => status_text(status, manage_apps),
    }
}

/// Sort rank for "by status": things needing attention first.
pub fn status_rank(status: &AppStatus) -> u8 {
    match status {
        AppStatus::UpdateAvailable => 0,
        AppStatus::Installed => 1,
        AppStatus::UpToDate => 2,
        AppStatus::Available => 3,
        AppStatus::Checking => 4,
        AppStatus::Unavailable => 5,
        AppStatus::NoRelease => 6,
    }
}

#[cfg(test)]
mod tests {
    use artcraft_launcher_core::release::Asset;
    use artcraft_launcher_core::{Arch, CraftApp, InstallSource, Os, version};

    use super::*;

    const MAC: Option<Platform> = Some(Platform { os: Os::MacOs, arch: Arch::Arm64 });
    const WINDOWS: Option<Platform> = Some(Platform { os: Os::Windows, arch: Arch::X64 });
    const PAGE: &str = "https://github.com/shaboomi/terragen-releases/releases/tag/v0.3.0";

    /// terraGen (no macOS signing team) at `installed`, with 0.3.0 out as `files`.
    fn terragen(installed: &str, source: InstallSource, files: &[&str]) -> AppEntry {
        let app = CraftApp::by_id("terragen").unwrap();
        assert!(app.mac_team.is_none(), "these tests are about an app whose macOS build isn't signed");
        let mut e = AppEntry::new(app);
        e.detected = true;
        e.installed = Some(Installed {
            version: version::parse(installed),
            source,
            location: String::new(),
            package: None,
            externally_managed: false,
            origin: None,
            installation: None,
        });
        let assets = files
            .iter()
            .map(|name| Asset {
                name: (*name).to_string(),
                size: 1,
                url: format!("https://github.com/shaboomi/terragen-releases/releases/download/v0.3.0/{name}"),
                sha256: None,
            })
            .collect();
        e.latest = Some(LatestRelease::Found(Release {
            tag: "v0.3.0".into(),
            version: version::parse("0.3.0"),
            html_url: PAGE.into(),
            assets,
            ..Default::default()
        }));
        e
    }

    const FILES: &[&str] = &["TerraGen_0.3.0_universal.dmg", "TerraGen_0.3.0_x64-setup.exe"];

    #[test]
    fn an_unsigned_mac_build_is_an_update_that_opens_the_release_page() {
        let mut e = terragen("0.2.0", InstallSource::MacApp, FILES);
        // The backend won't install the .dmg over the copy in Applications.
        e.installable = Some(false);
        assert_eq!(e.manual_update(MAC), Some(PAGE));
        assert_eq!(e.status(MAC, true), AppStatus::UpdateAvailable);
        // Shown as an update, so not also noted as out of reach.
        assert_eq!(e.newer_out_of_reach(MAC), None);
    }

    #[test]
    fn an_up_to_date_mac_copy_has_no_manual_update() {
        let mut e = terragen("0.3.0", InstallSource::MacApp, FILES);
        e.installable = Some(false);
        assert_eq!(e.manual_update(MAC), None);
        assert_eq!(e.status(MAC, true), AppStatus::UpToDate);
    }

    #[test]
    fn a_release_without_a_mac_build_stays_out_of_reach() {
        let mut e = terragen("0.2.0", InstallSource::MacApp, &["TerraGen_0.3.0_x64-setup.exe"]);
        e.installable = Some(false);
        assert_eq!(e.manual_update(MAC), None);
        assert_eq!(e.status(MAC, true), AppStatus::Installed);
        assert_eq!(e.newer_out_of_reach(MAC).as_deref(), Some("0.3.0"));
    }

    #[test]
    fn windows_installs_the_update_itself() {
        let mut e = terragen("0.2.0", InstallSource::Setup, FILES);
        e.installable = Some(true);
        assert_eq!(e.manual_update(WINDOWS), None);
        assert_eq!(e.status(WINDOWS, true), AppStatus::UpdateAvailable);
    }
}
