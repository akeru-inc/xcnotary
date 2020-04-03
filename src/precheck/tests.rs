use super::Precheck;
use crate::util::input_path::PathType;

#[test]
pub fn test_precheck_package() {
    let test_path = test_utils::artifacts_path(PathType::InstallerPackage, "unsigned");
    assert!(super::package::DeveloperIdCheck.run(&test_path).is_err());

    let test_path = test_utils::artifacts_path(
        PathType::InstallerPackage,
        "signed_with_correctly_signed_app.pkg",
    );
    assert!(super::package::DeveloperIdCheck.run(&test_path).is_ok());
}
#[test]
pub fn test_precheck_bundle() {
    let artifact = test_utils::bundle_artifact("correctly_signed");
    assert!(super::bundle::DeveloperIdCheck.run(&artifact.path).is_ok());
    assert!(super::bundle::HardenedRuntimeCheck
        .run(&artifact.path)
        .is_ok());
    assert!(super::bundle::NoGetTaskAllowCheck
        .run(&artifact.path)
        .is_ok());
    assert!(super::bundle::SecureTimestampCheck
        .run(&artifact.path)
        .is_ok());

    let artifact = test_utils::bundle_artifact("unsigned");
    assert!(super::bundle::DeveloperIdCheck.run(&artifact.path).is_err());

    let artifact = test_utils::bundle_artifact("no_secure_timestamp");
    assert!(super::bundle::SecureTimestampCheck
        .run(&artifact.path)
        .is_err());

    let artifact = test_utils::bundle_artifact("no_hardened_runtime");
    assert!(super::bundle::HardenedRuntimeCheck
        .run(&artifact.path)
        .is_err());

    let artifact = test_utils::bundle_artifact("has_get_task_allow");
    assert!(super::bundle::NoGetTaskAllowCheck
        .run(&artifact.path)
        .is_err());
}

pub(super) mod test_utils {
    use crate::util::input_path::PathType;
    use std::path::PathBuf;
    use std::process::Command;
    use tempfile::{Builder as TempFileBuilder, TempDir};

    pub struct BundleArtifact {
        _temp_dir: TempDir,
        pub path: PathBuf,
    }

    pub fn bundle_artifact(name: &str) -> BundleArtifact {
        let zipped_artifact_path = artifacts_path(PathType::AppBundle, name);

        let temp_dir = TempFileBuilder::new().tempdir().unwrap();
        let temp_dir_path = temp_dir.path().to_str().unwrap();

        let status = Command::new("/usr/bin/ditto")
            .args(&[
                "-xk",
                &zipped_artifact_path.to_str().unwrap(),
                &temp_dir_path,
            ])
            .status()
            .unwrap();

        if !status.success() {
            panic!();
        }

        let mut artifact_path = PathBuf::from(temp_dir_path);
        artifact_path.push(name);
        artifact_path.set_extension("app");

        BundleArtifact {
            _temp_dir: temp_dir,
            path: artifact_path,
        }
    }

    pub(crate) fn artifacts_path(path_type: PathType, name: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/generated_artifacts");

        match path_type {
            PathType::AppBundle => {
                path.push("apps");
                path.push(name);
                path.set_extension("zip");
            }
            PathType::InstallerPackage => {
                path.push("packages");
                path.push(name);
                path.set_extension("pkg");
            }
        }

        assert!(path.exists(), "Expected test artifact to exist: {:?}", path);

        path
    }
}
