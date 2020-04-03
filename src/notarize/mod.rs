mod run;

use crate::util::input_path::PathType;
use std::error::Error;
use std::path::PathBuf;

pub(crate) struct NotarizeOp<'a> {
    input_path: &'a PathBuf,
    path_type: &'a PathType,
    bundle_id: &'a str,
    developer_account: &'a str,
    password_keychain_item: &'a str,
}

pub(crate) fn run(
    input_path: &PathBuf,
    path_type: &PathType,
    bundle_id: &str,
    developer_account: &str,
    password_keychain_item: &str,
) -> Result<(), Box<dyn Error>> {
    NotarizeOp::new(
        input_path,
        path_type,
        bundle_id,
        developer_account,
        password_keychain_item,
    )
    .run()
}
