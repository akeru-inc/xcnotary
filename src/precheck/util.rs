use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use crate::util::OperationError;

pub(super) fn passes_spctl(args: &[&str], input_path: &PathBuf) -> Result<bool, Box<dyn Error>> {
    // Post by Apple DTS here: https://forums.developer.apple.com/thread/128683
    let output = Command::new("/usr/sbin/spctl")
        .args(&["-v", "--assess"])
        .args(args)
        .arg(input_path.as_os_str())
        .output()?;

    let stderr = String::from_utf8(output.stderr).unwrap();

    return match output.status.code() {
        // spctl exits zero on success, or one if an operation has failed.
        // Exit code two indicates unrecognized or unsuitable arguments.
        // If an assessment operation results in denial but no other problem
        // has occurred, the exit code is three. (man spctl)
        None => Err(OperationError::new("Process terminated by signal").into()),
        Some(0) => Ok(true),
        Some(3) => Ok(stderr.contains("source=Unnotarized Developer ID")),
        _ => Err(OperationError::new(&format!("spctl: {}", stderr)).into()),
    };
}
