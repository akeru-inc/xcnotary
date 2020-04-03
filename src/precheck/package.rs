use super::PrecheckError;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub(super) struct DeveloperIdCheck;

impl super::Precheck for DeveloperIdCheck {
    fn display(&self) -> &'static str {
        "Developer ID signing"
    }

    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/sbin/pkgutil")
            .arg("--check-signature")
            .arg(input_path.as_os_str())
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(PrecheckError::new(
                "Package is not signed with a Developer ID certificate.",
                "Make sure to provide the --sign <installer identity> argument to pkgbuild.",
            )
            .into());
        }

        Ok(())
    }
}
