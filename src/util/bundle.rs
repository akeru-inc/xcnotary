use super::OperationError;
use crate::util::plist::{bundle_info_from_file, structs::BundleInfo};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn read_bundle<P: AsRef<Path>>(bundle_path: P) -> Result<BundleInfo, OperationError> {
    let bundle_path = bundle_path.as_ref();

    if !bundle_path.exists() {
        return Err(
            OperationError::new(&format!("Path not found: {}", bundle_path.display())).into(),
        );
    }

    if !is_app_bundle(bundle_path).unwrap() {
        return Err(OperationError::new(&format!(
            "Expected an application bundle at {}",
            bundle_path.display()
        ))
        .into());
    }

    let mut info_plist_path = PathBuf::from(bundle_path);
    info_plist_path.push("Contents");
    info_plist_path.push("Info.plist");
    if !info_plist_path.exists() {
        return Err(OperationError::new(&format!(
            "Info.plist was not found at {}",
            info_plist_path.display()
        ))
        .into());
    }

    Ok(bundle_info_from_file(info_plist_path))
}

fn is_app_bundle<P: AsRef<Path>>(bundle_path: P) -> Result<bool, OperationError> {
    let output = Command::new("/usr/bin/mdls")
        .args(&[
            "-name",
            "kMDItemContentTypeTree",
            bundle_path.as_ref().to_str().unwrap(),
        ])
        .output()
        .unwrap();

    if !output.status.success() {
        // mdls sends error message to stdout
        return Err(OperationError::new(
            &String::from_utf8(output.stdout).unwrap(),
        ));
    }

    let stdout = String::from_utf8(output.stdout).unwrap();
    Ok(stdout.contains(r#""com.apple.application-bundle""#))
}
