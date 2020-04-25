use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "macOS App Notarization Helper")]
pub(crate) enum Args {
    /// Only performs code signing checks on the input bundle or package
    Precheck {
        /// Path to bundle or package
        #[structopt(parse(from_os_str))]
        input_path: PathBuf,
    },

    /// Performs code signing checks on the input bundle or package,
    /// submits it to the notarization service, and blocks waiting for completion
    Notarize {
        /// Apple developer account username
        #[structopt(short, long)]
        developer_account: String,

        /// Name of keychain item containing developer account password
        /// (see: https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow)
        #[structopt(short = "k", long = "developer-password-keychain-item")]
        password_keychain_item: String,

        /// Path to bundle or package
        #[structopt(parse(from_os_str))]
        input_path: PathBuf,

        /// Required if the developer credentials are associated with more than one team. Corresponds to "ProviderShortname" from running `xcrun altool --list-providers`
        #[structopt(long)]
        provider: Option<String>,
    },
}

pub(crate) fn parse() -> Args {
    Args::from_args()
}
