use super::PrecheckError;
use crate::OperationError;
use std::error::Error;
use std::process::Command;

pub(super) struct NoGetTaskAllowCheck;

impl super::BundleCheck for NoGetTaskAllowCheck {
    fn display(&self) -> &'static str {
        "No get-task-allow entitlement"
    }

    fn run(&self, bundle_path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .args(&["-d", "--entitlements", ":-", bundle_path])
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();
            return Err(OperationError::new(&stderr).into());
        }

        if String::from_utf8(output.stdout)
            .unwrap()
            .contains("com.apple.security.get-task-allow")
        {
            return Err(PrecheckError::new(
                "Bundle includes get-task-allow entitlement.",
                "Specify CODE_SIGN_INJECT_BASE_ENTITLEMENTS=NO when running xcodebuild.",
            )
            .into());
        }

        Ok(())
    }
}
