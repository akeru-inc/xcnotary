mod bundle;
mod package;
mod util;
use console::Style;
mod dmg;
mod error;

#[cfg(test)]
mod tests;

use std::error::Error;
use std::path::PathBuf;

use crate::util::display::progress_bar;
use crate::util::input_path::PathType;

pub(self) use error::Status;

pub(crate) trait Precheck {
    fn display(&self) -> &'static str;
    fn run(&self, input_path: &PathBuf) -> Result<Status, Box<dyn Error>>;
}

pub(crate) fn run(
    path: &PathBuf,
    path_type: &PathType,
    show_message: bool,
) -> Result<(), Box<dyn Error>> {
    let checks: Vec<Box<dyn Precheck>> = match path_type {
        PathType::AppBundle => vec![
            Box::new(bundle::DeveloperIdCheck),
            Box::new(bundle::HardenedRuntimeCheck),
            Box::new(bundle::NoGetTaskAllowCheck),
            Box::new(bundle::SecureTimestampCheck),
        ],
        PathType::DiskImage => vec![Box::new(dmg::DeveloperIdCheck)],
        PathType::InstallerPackage => vec![Box::new(package::DeveloperIdCheck)],
    };

    for check in checks {
        let pb = progress_bar(&format!("Perform check: {}", check.display()));

        if let Some(error) = check.run(path)?.to_err() {
            return Err(error.into());
        }

        pb.finish();
    }

    if show_message {
        if let PathType::InstallerPackage = path_type {
            let emphasized = Style::new().yellow().bold();
            println!();
            println!(
                r#"{}Because .pkg input was specified, not all checks were performed. Run "precheck" on a bundle for additional checks."#,
                emphasized.apply_to("Note: ")
            );
        }
    }

    Ok(())
}
