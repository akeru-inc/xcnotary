use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::io::prelude::*;
use std::process::Command;
use tempfile::{Builder as TempFileBuilder, NamedTempFile};

use crate::util::plist;
use crate::util::plist::structs::{NotarizationInfo, NotarizationStatus};
use crate::util::OperationError;

use super::NotarizeOp;

pub(super) fn notarize(config: &NotarizeOp) -> Result<(), Box<dyn Error>> {
    let bundle_path = config.bundle_path.to_str().unwrap();
    super::precheck::checks().into_iter().try_for_each(|x| {
        let pb = progress_bar(&format!("Perform check: {}", x.display()));
        let ret = x.run(bundle_path);

        if ret.is_ok() {
            pb.finish();
        }

        ret
    })?;

    let pb = progress_bar("Compressing bundle");
    let zip_file = create_zip(config)?;
    // thread::sleep(Duration::from_secs(10));
    pb.finish();

    let pb = progress_bar("Uploading to notarization service");
    let request_id = submit_app(config, zip_file)?;
    pb.finish();

    let pb = progress_bar("Waiting for notarization");

    loop {
        std::io::stdout().flush().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));

        let info = get_status(config, &request_id)?;

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

    let pb = progress_bar("Stapling bundle");
    staple_bundle(config)?;
    pb.finish();

    println!("\n{}", style("Success!").green().bold());

    Ok(())
}

fn create_zip(config: &NotarizeOp) -> Result<NamedTempFile, OperationError> {
    let zip_file = TempFileBuilder::new().suffix(".zip").tempfile().unwrap();
    let zip_file_path = zip_file.path().to_str().unwrap();

    let bundle_parent_dir_path = config.bundle_path.parent().unwrap();
    let bundle_file_name = config.bundle_path.file_name().unwrap().to_str().unwrap();

    let status = Command::new("/usr/bin/ditto")
        .current_dir(bundle_parent_dir_path)
        .args(&["-ck", "--keepParent", &bundle_file_name, &zip_file_path])
        .status()
        .unwrap();

    if !status.success() {
        return Err(OperationError::new("Notarization zip creation failed"));
    }

    return Ok(zip_file);
}

/// Submits app to the notarization service, returning the request ID.
fn submit_app(config: &NotarizeOp, zip_file: NamedTempFile) -> Result<String, OperationError> {
    let notary_zip_path = zip_file.path().to_str().unwrap();

    let output = Command::new("/usr/bin/xcrun")
        .args(&[
            "altool",
            "--notarize-app",
            "--file",
            notary_zip_path,
            "--primary-bundle-id",
            config.bundle_id,
            "-u",
            config.developer_account,
            "-p",
            &format!("@keychain:{}", config.password_keychain_item),
            "--output-format",
            "xml",
        ])
        .output()
        .unwrap();

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
fn get_status(config: &NotarizeOp, request_id: &str) -> Result<NotarizationInfo, OperationError> {
    let output = Command::new("/usr/bin/xcrun")
        .args(&[
            "altool",
            "--notarization-info",
            &request_id,
            "-u",
            config.developer_account,
            "-p",
            &format!("@keychain:{}", config.password_keychain_item),
            "--output-format",
            "xml",
        ])
        .output()
        .unwrap();

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

fn staple_bundle(config: &NotarizeOp) -> Result<(), OperationError> {
    let output = Command::new("/usr/bin/stapler")
        .args(&["staple", config.bundle_path.to_str().unwrap()])
        .output()
        .unwrap();

    if !output.status.success() {
        let output = String::from_utf8(output.stderr).unwrap();
        return Err(OperationError::detail("Staple failed", &output));
    }

    Ok(())
}

fn progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(120);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⡘⠳⠈⠍⠉⡗⡙⠙⠚⡐⢋✔") //
            .template("{spinner:.white} {msg}"),
    );
    pb.set_message(message);
    pb
}
