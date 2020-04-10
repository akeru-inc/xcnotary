mod run;

use crate::util::input_path::PathType;
use std::error::Error;
use std::path::PathBuf;

pub(crate) struct NotarizeOp {
    input_path: PathBuf,
    path_type: PathType,
    bundle_id: String,
    developer_account: String,
    password_keychain_item: String,
    provider: Option<String>,
}

pub(crate) fn run(
    input_path: PathBuf,
    path_type: PathType,
    bundle_id: String,
    developer_account: String,
    password_keychain_item: String,
    provider: Option<String>,
) -> Result<(), Box<dyn Error>> {
    NotarizeOp::new(
        input_path,
        path_type,
        bundle_id,
        developer_account,
        password_keychain_item,
        provider,
    )
    .run()
}
