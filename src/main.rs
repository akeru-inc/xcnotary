mod notarize;
mod precheck;
mod util;

use console::Style;
use std::error::Error;
use util::cli::Args;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("\n{}", err);
        std::process::exit(1);
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = util::cli::parse();

    let emphasized = Style::new().white().bold();
    println!("{}\n", emphasized.apply_to("Processing..."),);

    match args {
        Args::Precheck { input_path } => {
            let path_type = util::input_path::identify_path_type(&input_path)?;
            precheck::run(&input_path, &path_type, true)?;
        }
        Args::Notarize {
            developer_account,
            password_keychain_item,
            input_path,
            provider,
        } => {
            let (path_type, bundle_id) = util::input_path::path_info(&input_path)?;

            precheck::run(&input_path, &path_type, false)?;
            notarize::run(
                input_path,
                path_type,
                bundle_id,
                developer_account,
                password_keychain_item,
                provider,
            )?;
        }
    }

    Ok(())
}
