use super::PrecheckError;
use std::error::Error;
use std::process::Command;

pub(super) struct DeveloperIdCheck;

impl super::BundleCheck for DeveloperIdCheck {
    fn display(&self) -> &'static str {
        "Developer ID signing"
    }

    fn run(&self, bundle_path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/sbin/spctl")
            .args(&["-v", "--assess", "--type", "exec", &bundle_path])
            .output()
            .unwrap();

        if !output.status.success() {
            return Err(PrecheckError::new(
                "Bundle is not signed with a Developer ID certificate or bundle includes unsigned binaries.",
                "Make sure CODE_SIGN_IDENTITY was specified during the build."
            ).into());
        }

        Ok(())
    }
}
