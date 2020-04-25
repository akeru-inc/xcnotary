use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use std::str;

#[test]
pub fn test_precheck_package_pass() {
    Command::cargo_bin("xcnotary")
        .unwrap()
        .arg("precheck")
        .arg(package_artifacts_path("signed_with_correctly_signed_app").as_os_str())
        .assert()
        .success()
        .stdout(predicate::str::contains("not all checks were performed"));
}

#[test]
pub fn test_precheck_package_fail() {
    Command::cargo_bin("xcnotary")
        .unwrap()
        .arg("precheck")
        .arg(package_artifacts_path("unsigned").as_os_str())
        .assert()
        .failure()
        .stderr(predicate::str::contains("Package is not signed"));
}

fn package_artifacts_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("generated_artifacts");
    path.push("pkg");
    path.push(name);
    path.set_extension("pkg");
    path
}
