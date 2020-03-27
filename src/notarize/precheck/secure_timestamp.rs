use super::PrecheckError;
use crate::OperationError;
use std::error::Error;
use std::process::Command;

pub(super) struct SecureTimestampCheck;

impl super::BundleCheck for SecureTimestampCheck {
    fn display(&self) -> &'static str {
        "Secure timestamp"
    }

    fn run(&self, bundle_path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .args(&["-dvv", bundle_path])
            .output()
            .unwrap();

        // unfortunately, codesign sends successful output to stderr.
        let stderr = String::from_utf8(output.stderr).unwrap();
        if !output.status.success() {
            return Err(OperationError::new(&stderr).into());
        }

        // Note: Presence of "Signed Time" would also indicate that a secure timestamp is missing
        if !stderr.contains("Timestamp=") {
            return Err(PrecheckError::new(
            r#"The bundle is missing a secure timestamp."#,
             "codesign using --timestamp flag, or pass OTHER_CODE_SIGN_FLAGS=--timestamp to xcodebuild."            
            ).into());
        }

        Ok(())
    }
}
