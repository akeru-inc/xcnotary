use std::path::PathBuf;

use super::Precheck;
use crate::util::input_path::PathType;

impl super::Status {
    fn is_pass(&self) -> bool {
        match self {
            Self::Pass => true,
            Self::Fail { .. } => false,
        }
    }

    fn is_fail(&self) -> bool {
        !self.is_pass()
    }
}

#[test]
pub fn test_precheck_package() {
    let artifact = test_utils::artifact(PathType::InstallerPackage, "unsigned");
    assert!(super::package::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());

    let artifact = test_utils::artifact(
        PathType::InstallerPackage,
        "signed_with_correctly_signed_app",
    );
    assert!(super::package::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());
}

#[test]
pub fn test_malformed_input() {
    assert!(super::package::DeveloperIdCheck
        .run(&PathBuf::from("foobar"))
        .is_err());
}

#[test]
pub fn test_precheck_dmg() {
    let artifact = test_utils::artifact(PathType::DiskImage, "unsigned");
    assert!(super::dmg::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());

    let artifact = test_utils::artifact(PathType::DiskImage, "signed_with_correctly_signed_app");
    assert!(super::dmg::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());
}

#[test]
pub fn test_precheck_bundle() {
    let artifact = test_utils::artifact(PathType::AppBundle, "correctly_signed");
    assert!(super::bundle::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());
    assert!(super::bundle::HardenedRuntimeCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());
    assert!(super::bundle::NoGetTaskAllowCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());
    assert!(super::bundle::SecureTimestampCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());

    let artifact = test_utils::artifact(PathType::AppBundle, "unsigned");
    assert!(super::bundle::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());

    let artifact = test_utils::artifact(PathType::AppBundle, "manually_signed");
    assert!(super::bundle::DeveloperIdCheck
        .run(&artifact.path)
        .unwrap()
        .is_pass());

    let artifact = test_utils::artifact(PathType::AppBundle, "no_secure_timestamp");
    assert!(super::bundle::SecureTimestampCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());

    let artifact = test_utils::artifact(PathType::AppBundle, "no_hardened_runtime");
    assert!(super::bundle::HardenedRuntimeCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());

    let artifact = test_utils::artifact(PathType::AppBundle, "has_get_task_allow");
    assert!(super::bundle::NoGetTaskAllowCheck
        .run(&artifact.path)
        .unwrap()
        .is_fail());
}

pub(super) mod test_utils {
    use crate::util::input_path::PathType;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::{Builder as TempFileBuilder, TempDir};

    pub struct Artifact {
        pub path: PathBuf,
        _temp_dir: Option<TempDir>,
    }

    fn bundle_artifact(zipped_path: &str, name: &str) -> Artifact {
        let temp_dir = TempFileBuilder::new().tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap();

        let status = Command::new("/usr/bin/ditto")
            .args(&["-xk", zipped_path, &temp_dir_path])
            .status()
            .unwrap();

        if !status.success() {
            panic!();
        }

        let mut artifact_path = PathBuf::from(temp_dir_path);
        artifact_path.push(name);
        artifact_path.set_extension("app");

        Artifact {
            path: artifact_path,
            _temp_dir: Some(temp_dir),
        }
    }

    pub(crate) fn artifact(path_type: PathType, name: &str) -> Artifact {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/generated_artifacts");

        let artifact = match path_type {
            PathType::AppBundle => {
                path.push("app");
                path.push(name);
                path.set_extension("zip");

                bundle_artifact(path.to_str().unwrap(), name)
            }
            PathType::DiskImage => {
                path.push("dmg");
                path.push(name);
                path.set_extension("dmg");

                Artifact {
                    path: path,
                    _temp_dir: None,
                }
            }
            PathType::InstallerPackage => {
                path.push("pkg");
                path.push(name);
                path.set_extension("pkg");

                Artifact {
                    path: path,
                    _temp_dir: None,
                }
            }
        };

        assert!(
            artifact.path.exists(),
            "Expected test artifact to exist: {:?}",
            artifact.path
        );

        artifact
    }
}
