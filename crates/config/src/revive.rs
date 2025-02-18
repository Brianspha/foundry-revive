use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Revive Config
pub struct ReviveConfig {
    /// The revive path
    pub revive_path: Option<PathBuf>,
    /// The solc path that will be used by revive
    pub solc_path: Option<PathBuf>,
    /// Enable compilation using revive
    pub revive_compile: bool,
}

