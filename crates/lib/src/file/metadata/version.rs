use serde::{Deserialize, Serialize};

/// SOPS version entry of the file metadata.
///
/// Newly encrypted files get [`SopsVersion::CURRENT`], which can be overridden
/// at build time with the `ROPS_SOPS_VERSION` environment variable. An existing
/// value, or the lack thereof, is otherwise preserved as is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SopsVersion(String);

impl SopsVersion {
    pub const CURRENT: &'static str = match option_env!("ROPS_SOPS_VERSION") {
        Some(version) => version,
        None => "3.8.1",
    };
}

impl Default for SopsVersion {
    fn default() -> Self {
        Self(Self::CURRENT.to_string())
    }
}

#[cfg(feature = "test-utils")]
mod mock {
    use super::*;
    use crate::*;

    impl MockTestUtil for SopsVersion {
        fn mock() -> Self {
            Self::default()
        }
    }

    impl MockDisplayTestUtil for SopsVersion {
        fn mock_display() -> String {
            SopsVersion::CURRENT.to_string()
        }
    }
}
