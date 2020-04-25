use super::OperationError;
use crate::util::plist::{bundle_info_from_file, structs::BundleInfo};
use std::path::{Path, PathBuf};
use std::process::Command;

pub(crate) fn path_info<P: AsRef<Path>>(
    input_path: P,
) -> Result<(PathType, String), OperationError> {
    let path_type = identify_path_type(&input_path)?;
    let bundle_id = match &path_type {
        PathType::AppBundle => read_bundle_info(&input_path)?.id,
        // Generate a pseudo-bundle ID. This value is used for informational purposes
        // e.g. to notify of notarization status.
        PathType::DiskImage | PathType::InstallerPackage => input_path
            .as_ref()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '.' })
            .collect::<Vec<char>>()
            .into_iter()
            .collect::<String>(),
    };

    Ok((path_type, bundle_id))
}

pub(crate) fn identify_path_type<P: AsRef<Path>>(
    bundle_path: P,
) -> Result<PathType, OperationError> {
    // Alternatively, could just check the extension.
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

    if stdout.contains(r#""com.apple.application-bundle""#) {
        return Ok(PathType::AppBundle);
    } else if stdout.contains(r#""com.apple.disk-image""#) {
        return Ok(PathType::DiskImage);
    } else if stdout.contains(r#""com.apple.installer-package-archive""#) {
        return Ok(PathType::InstallerPackage);
    } else {
        return Err(OperationError::new(&format!(
            "Expected an application bundle, disk image, or installer package at {}",
            bundle_path.as_ref().display()
        ))
        .into());
    }
}

pub(crate) enum PathType {
    AppBundle,
    DiskImage,
    InstallerPackage,
}

fn read_bundle_info<P: AsRef<Path>>(bundle_path: P) -> Result<BundleInfo, OperationError> {
    let bundle_path = bundle_path.as_ref();

    if !bundle_path.exists() {
        return Err(
            OperationError::new(&format!("Path not found: {}", bundle_path.display())).into(),
        );
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
