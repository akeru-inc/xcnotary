use super::util::passes_spctl;
use super::Status;

use std::error::Error;
use std::path::PathBuf;

pub(super) struct DeveloperIdCheck;

impl super::Precheck for DeveloperIdCheck {
    fn display(&self) -> &'static str {
        "Developer ID signing"
    }

    fn run(&self, input_path: &PathBuf) -> Result<Status, Box<dyn Error>> {
        // Note: may also use "/usr/sbin/pkgutil --check-signature"
        if passes_spctl(&vec!["-t", "install"], input_path)? {
            Ok(Status::Pass)
        } else {
            Ok(Status::fail_with(
                "Package is not signed with a Developer ID certificate.",
                "Make sure to provide the --sign <installer identity> argument to pkgbuild.",
                None,
            ))
        }
    }
}
