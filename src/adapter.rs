//! What this adapter says it is, and the kit that serves its contract.

use std::sync::Arc;

use adapter_kit_servarr::client::Servarr;
use adapter_kit_servarr::curator::{Curator, Files, NoMusic};
use lemonfiber_adapter_kit::credential::Holding;
use lemonfiber_adapter_kit::{Kit, Readiness};
use lemonfiber_contract::adapter::{About, Release};
use lemonfiber_contract::capabilities::library;
use lemonfiber_ports::http::Http;

/// Where Radarr answers, by its service id and the port it listens on.
pub const ADDRESS: &str = "http://radarr:7878";

/// The upstream this adapter fronts, as its project names itself.
pub const UPSTREAM: &str = "Radarr";

/// The Radarr release this adapter's recordings were taken from.
pub const RELEASE: &str = "5.28.0.10274-ls286";

/// The digest of the image that release's recordings were taken from.
pub const DIGEST: &str = "sha256:c984533510abe0219a70e80d15bd0d212b7df21baa0913759c4ce6cc9092240b";

/// What every failure is blamed on.
pub const SERVICE: &str = "radarr";

/// The version of Radarr's API this adapter speaks.
pub const API_VERSION: u32 = 3;

/// What this adapter says it is: the contract it speaks and the release it supports.
#[must_use]
pub fn about() -> About {
    About {
        speaks: vec![lemonfiber_contract::spoken(
            library::curate::CAPABILITY,
            library::curate::MAJOR,
        )],
        upstream: UPSTREAM.to_owned(),
        releases: vec![Release {
            version: RELEASE.to_owned(),
            digest: DIGEST.to_owned(),
        }],
    }
}

/// The Radarr reached at `base` over `http`, curating films with the key `held`
/// supplies.
#[must_use]
pub fn radarr(http: Arc<dyn Http>, base: impl Into<String>, held: Arc<dyn Holding>) -> Curator {
    Curator::new(
        Servarr::new(http, base, held, SERVICE, API_VERSION),
        Files::FILMS,
        NoMusic,
    )
}

/// The contract, served from `radarr` under `key`, ready while Radarr answers.
#[must_use]
pub fn kit(key: String, radarr: Arc<Curator>) -> Kit {
    let asked = Arc::clone(&radarr);
    let ready: Readiness = Arc::new(move || {
        let asked = Arc::clone(&asked);
        Box::pin(async move { asked.servarr().answers().await })
    });
    Kit::new(key, about())
        .ready_when(ready)
        .serving(library::curate::served(radarr))
}

#[cfg(test)]
mod tests;
