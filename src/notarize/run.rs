use console::style;
use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{Builder as TempFileBuilder, TempDir};

use crate::util::display::progress_bar;
use crate::util::input_path::PathType;
use crate::util::plist;
use crate::util::plist::structs::{NotarizationInfo, NotarizationStatus};
use crate::util::OperationError;

use super::NotarizeOp;

struct InputFilePath {
    path: PathBuf,
    _temp_dir: Option<TempDir>,
}
enum AltoolArgs<'a> {
    NotarizationInfo {
        request_id: &'a str,
    },
    NotarizeApp {
        path: &'a PathBuf,
        bundle_id: &'a str,
    },
}

impl NotarizeOp {
    pub(super) fn new(
        input_path: PathBuf,
        path_type: PathType,
        bundle_id: String,
        developer_account: String,
        password_keychain_item: String,
        provider: Option<String>,
    ) -> Self {
        NotarizeOp {
            input_path,
            path_type,
            bundle_id,
            developer_account,
            password_keychain_item,
            provider,
        }
    }

    pub(super) fn run(&self) -> Result<(), Box<dyn Error>> {
        let input_path = match self.path_type {
            PathType::AppBundle => {
                let pb = progress_bar("Compressing bundle");
                let ret = self.zip_bundle()?;
                pb.finish();
                ret
            }
            PathType::DiskImage | PathType::InstallerPackage => InputFilePath {
                path: self.input_path.clone(),
                _temp_dir: None,
            },
        };

        let pb = progress_bar("Uploading to notarization service");
        let request_id = self.submit_to_service(input_path)?;
        pb.finish();

        let pb = progress_bar("Waiting for notarization");

        loop {
            std::io::stdout().flush().unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));

            let info = self.get_status(&request_id)?;

            match info.details.status {
                NotarizationStatus::InProgress => continue,
                NotarizationStatus::Success => {
                    break;
                }
                NotarizationStatus::Invalid => {
                    let log_url = info.details.logfile_url.unwrap();

                    let log_response = reqwest::blocking::get(&log_url).unwrap().text().unwrap();

                    return Err(OperationError::detail(
                        "Notarization failed. Server response",
                        &log_response,
                    )
                    .into());
                }
            }
        }

        pb.finish();

        let pb = progress_bar("Stapling");
        self.staple()?;
        pb.finish();

        println!("\n{}", style("Success!").green().bold());

        Ok(())
    }

    fn zip_bundle(&self) -> Result<InputFilePath, OperationError> {
        let temp_dir = TempFileBuilder::new().tempdir().unwrap();

        let bundle_file_name = self.input_path.file_name().unwrap();

        let mut zip_path = PathBuf::from(temp_dir.path());
        zip_path.set_file_name(bundle_file_name);
        zip_path.set_extension("zip");

        let mut bundle_parent_dir_path = self.input_path.parent().unwrap();
        // related: https://github.com/rust-lang/rust/issues/36861
        if !bundle_parent_dir_path.is_dir() {
            bundle_parent_dir_path = std::path::Path::new(".");
        }

        let status = Command::new("/usr/bin/ditto")
            .current_dir(bundle_parent_dir_path)
            .args(&[
                "-ck",
                "--keepParent",
                &bundle_file_name.to_str().unwrap(),
                &zip_path.to_str().unwrap(),
            ])
            .status()
            .unwrap();

        if !status.success() {
            return Err(OperationError::new("Notarization zip creation failed"));
        }

        return Ok(InputFilePath {
            path: zip_path,
            _temp_dir: Some(temp_dir),
        });
    }

    /// Submits app to the notarization service, returning the request ID.
    fn submit_to_service(&self, input_path: InputFilePath) -> Result<String, OperationError> {
        let output = self.run_altool(AltoolArgs::NotarizeApp {
            path: &input_path.path,
            bundle_id: &self.bundle_id,
        });

        if !output.status.success() {
            return Err(OperationError::detail(
                "Notarization upload failed",
                &String::from_utf8(output.stderr).unwrap(),
            ));
        }

        let upload = plist::notarization_upload_response(&output.stdout);

        Ok(upload.details.request_uuid)
    }

    /// Retrieves current status from the notarization service.
    fn get_status(&self, request_id: &str) -> Result<NotarizationInfo, OperationError> {
        let output = self.run_altool(AltoolArgs::NotarizationInfo {
            request_id: request_id.clone(),
        });

        if !output.status.success() {
            return Err(OperationError::detail(
                "Notarization status check failed",
                &String::from_utf8(output.stderr).unwrap(),
            ));
        }

        let info = plist::notarization_status_response(&output.stdout);

        if !info
            .success_message
            .eq("No errors getting notarization info.")
        {
            return Err(OperationError::detail(
                "Unexpected notarization message",
                &info.success_message,
            ));
        }

        Ok(info)
    }

    fn staple(&self) -> Result<(), OperationError> {
        let output = Command::new("/usr/bin/stapler")
            .arg("staple")
            .arg(self.input_path.as_os_str())
            .output()
            .unwrap();

        if !output.status.success() {
            let output = String::from_utf8(output.stderr).unwrap();
            return Err(OperationError::detail("Staple failed", &output));
        }

        Ok(())
    }

    fn run_altool(&self, args: AltoolArgs) -> std::process::Output {
        let args = match args {
            AltoolArgs::NotarizationInfo { request_id } => vec!["--notarization-info", &request_id],
            AltoolArgs::NotarizeApp { path, bundle_id } => vec![
                "--notarize-app",
                "--file",
                path.to_str().unwrap(),
                "--primary-bundle-id",
                bundle_id,
            ],
        };

        let provider_args = self
            .provider
            .as_ref()
            .map_or(vec![], |p| vec!["--asc-provider", &p]);

        Command::new("/usr/bin/xcrun")
            .args(&[
                "altool",
                "-u",
                &self.developer_account,
                "-p",
                &format!("@keychain:{}", self.password_keychain_item),
                "--output-format",
                "xml",
            ])
            .args(provider_args)
            .args(args)
            .output()
            .unwrap()
    }
}
