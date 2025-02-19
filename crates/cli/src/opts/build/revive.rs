use clap::Parser;
use foundry_config::revive::ReviveConfig;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Serialize, Parser)]
#[clap(next_help_heading = "Revive configuration")]
/// Compiler options for revive
/// TODO: We need to add more revive specific arguments
pub struct ReviveOpts {
    #[clap(
        value_name = "REVIVE_COMPILE",
        help = "Enable compiling with revive",
        long = "revive-compile",
        action = clap::ArgAction::SetTrue,
        default_value_t = false
    )]
    pub revive_compile: bool,

    #[clap(help = "Solc path to be used by revive", long = "solc-path", value_name = "SOLC_PATH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solc_path: Option<PathBuf>,

    #[clap(
        long = "revive-path",
        visible_alias = "revive",
        help = "Specify a revive path to be used",
        value_name = "REVIVE_PATH"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revive_path: Option<PathBuf>,
}

impl ReviveOpts {
    pub(crate) fn apply_overrides(&self, mut revive: ReviveConfig) -> ReviveConfig {
        macro_rules! set_if_some {
            ($src:expr, $dst:expr) => {
                if let Some(src) = $src {
                    $dst = src.into();
                }
            };
        }

        set_if_some!(self.solc_path.clone(), revive.solc_path);
        set_if_some!(self.revive_path.clone(), revive.revive_path);
        revive.revive_compile = self.revive_compile;

        revive
    }
}

impl From<ReviveOpts> for ReviveConfig {
    fn from(args: ReviveOpts) -> Self {
        ReviveConfig {
            revive_compile: args.revive_compile,
            solc_path: args.solc_path,
            revive_path: args.revive_path,
        }
    }
}
