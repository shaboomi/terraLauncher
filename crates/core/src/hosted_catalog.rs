//! Where the rest of the launcher asks for the catalog. Upstream replaces the bundled catalog
//! with a signed copy fetched at startup; terraLaunch has no such service, so every answer
//! here is the bundled one ([`SUITE`]) and nothing is fetched.

use crate::catalog::{CraftApp, SUITE};
use crate::details::AppDetails;

/// Desktop metadata a hosted catalog could carry for an app. None is hosted, so none exists.
pub struct Metadata {
    pub desktop_categories: String,
    pub startup_wm_class: String,
    pub icon_sha256: String,
}

/// The apps the launcher lists, in display order.
pub fn apps() -> &'static [CraftApp] {
    &SUITE
}

/// A hosted details page for `id`: always `None`, so the bundled page is used.
pub fn details(_id: &str) -> Option<&'static AppDetails> {
    None
}

/// Hosted desktop metadata for `id`: always `None`.
pub fn metadata(_id: &str) -> Option<&'static Metadata> {
    None
}

/// A hosted icon for `id`: always `None`, so the bundled icon is used.
pub fn icon(_id: &str) -> Option<&'static [u8]> {
    None
}
