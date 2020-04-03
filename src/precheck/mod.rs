mod bundle;
mod package;
use console::Style;

mod precheck_error;

use std::path::PathBuf;

use crate::util::display::progress_bar;
use crate::util::input_path::PathType;

#[cfg(test)]
mod tests;

use std::error::Error;

pub(self) use precheck_error::PrecheckError;

pub(crate) trait Precheck {
    fn display(&self) -> &'static str;
    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>>;
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
        PathType::InstallerPackage => vec![Box::new(package::DeveloperIdCheck)],
    };

    checks.into_iter().try_for_each(|x| {
        let pb = progress_bar(&format!("Perform check: {}", x.display()));
        let ret = x.run(path);

        if ret.is_ok() {
            pb.finish();
        }

        ret
    })?;

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
