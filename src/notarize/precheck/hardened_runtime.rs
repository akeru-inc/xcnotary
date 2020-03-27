use super::PrecheckError;
use crate::OperationError;
use std::error::Error;
use std::process::Command;

pub(super) struct HardenedRuntimeCheck;

impl super::BundleCheck for HardenedRuntimeCheck {
    fn display(&self) -> &'static str {
        "Hardened runtime"
    }

    fn run(&self, bundle_path: &str) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .args(&["--display", "--verbose", bundle_path])
            .output()
            .unwrap();

        // unfortunately, codesign sends output to stderr.
        let stderr = String::from_utf8(output.stderr).unwrap();
        if !output.status.success() {
            return Err(OperationError::new(&stderr).into());
        }

        let code_directory_line = stderr
            .lines()
            .find(|s: &&str| s.starts_with("CodeDirectory"))
            .unwrap()
            .to_string();
        let flags_text = code_directory_line
            .split_ascii_whitespace()
            .into_iter()
            .find(|s: &&str| s.starts_with("flags"))
            .unwrap();

        if !flags_text.contains("runtime") {
            return Err(PrecheckError::new(
                "Bundle does not have hardened runtime enabled.",
                r#"codesign using --runtime flag, or pass OTHER_CODE_SIGN_FLAGS=--runtime to xcodebuild. You can also enable the "Hardened Runtime" capability in Xcode's target settings > "Signing and Capabilities""#,
            )
            .into());
        }

        Ok(())
    }
}
