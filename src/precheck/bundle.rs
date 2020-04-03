use super::PrecheckError;
use crate::util::OperationError;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

pub(super) struct DeveloperIdCheck;

impl super::Precheck for DeveloperIdCheck {
    fn display(&self) -> &'static str {
        "Developer ID signing"
    }

    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/sbin/spctl")
            .args(&["-v", "--assess", "--type", "exec"])
            .arg(input_path.as_os_str())
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

pub(super) struct HardenedRuntimeCheck;

impl super::Precheck for HardenedRuntimeCheck {
    fn display(&self) -> &'static str {
        "Hardened runtime"
    }

    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .args(&["--display", "--verbose"])
            .arg(input_path.as_os_str())
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

pub(super) struct NoGetTaskAllowCheck;

impl super::Precheck for NoGetTaskAllowCheck {
    fn display(&self) -> &'static str {
        "No get-task-allow entitlement"
    }

    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .args(&["-d", "--entitlements", ":-"])
            .arg(input_path.as_os_str())
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

pub(super) struct SecureTimestampCheck;

impl super::Precheck for SecureTimestampCheck {
    fn display(&self) -> &'static str {
        "Secure timestamp"
    }

    fn run(&self, input_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let output = Command::new("/usr/bin/codesign")
            .arg("-dvv")
            .arg(input_path.as_os_str())
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
