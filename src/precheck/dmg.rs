use super::Status;

use std::error::Error;
use std::path::PathBuf;

use super::util::passes_spctl;

pub(super) struct DeveloperIdCheck;

impl super::Precheck for DeveloperIdCheck {
    fn display(&self) -> &'static str {
        "Developer ID signing"
    }

    fn run(&self, input_path: &PathBuf) -> Result<Status, Box<dyn Error>> {
        // https://developer.apple.com/library/archive/technotes/tn2206/_index.html#//apple_ref/doc/uid/DTS40007919-CH1-TNTAG18
        if passes_spctl(
            &vec!["-t", "open", "--context", "context:primary-signature"],
            input_path,
        )? {
            Ok(Status::Pass)
        } else {
            Ok(Status::fail_with(
                "Disk image is not signed with a Developer ID certificate.",
                r#"Run codesign -s "Developer ID Application: <team>" <dmg_path> to sign."#,
                None,
            ))
        }
    }
}
