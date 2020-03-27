mod notarize;
mod util;

use console::Style;
use notarize::NotarizeOp;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use util::OperationError;

#[derive(Debug, StructOpt)]
#[structopt(about = "macOS App Notarization Helper")]
struct Args {
    /// Apple developer account username
    #[structopt(short, long)]
    developer_account: String,

    /// Name of keychain item containing developer account password
    /// (see: https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/customizing_the_notarization_workflow)
    #[structopt(short = "k", long = "developer-password-keychain-item")]
    password_keychain_item: String,

    /// Path to bundle to be notarized
    #[structopt(short, long, parse(from_os_str))]
    bundle_path: PathBuf,
}

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("\n{}", err);
        std::process::exit(1);
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    let bundle_info = util::bundle::read_bundle(&args.bundle_path)?;

    let emphasized = Style::new().white().bold();
    println!(
        "{} {} {} ({})\n",
        emphasized.apply_to("Processing"),
        bundle_info.name,
        bundle_info.short_version_string,
        bundle_info.version
    );

    NotarizeOp::new(
        &bundle_info.id,
        args.bundle_path,
        &args.developer_account,
        &args.password_keychain_item,
    )
    .run()
}

