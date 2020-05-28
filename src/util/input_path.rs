use super::OperationError;
use crate::util::plist::{bundle_info_from_file, structs::BundleInfo};
use std::path::{Path, PathBuf};

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
    if let Some(val) = bundle_path.as_ref().extension() {
        if val == "app" {
            return Ok(PathType::AppBundle);
        } else if val == "dmg" {
            return Ok(PathType::DiskImage);
        } else if val == "pkg" {
            return Ok(PathType::InstallerPackage);
        }
    }

    Err(OperationError::new(&format!(
        "Expected an application bundle, disk image, or installer package at {}",
        bundle_path.as_ref().display()
    ))
    .into())
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::identify_path_type;
    use super::PathType;
    use std::path::PathBuf;

    #[test]
    fn test_identify_path() {
        assert_eq!(Some(PathType::AppBundle), identify_path_type(PathBuf::from("Foo.app")).ok());
        assert_eq!(Some(PathType::DiskImage), identify_path_type(PathBuf::from("Foo.dmg")).ok());
        assert_eq!(Some(PathType::InstallerPackage), identify_path_type(PathBuf::from("Foo.pkg")).ok());
        assert!(identify_path_type(PathBuf::from("Foo")).is_err());
    }
}
