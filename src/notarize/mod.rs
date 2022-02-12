mod run;

use crate::util::input_path::PathType;
use std::error::Error;
use std::path::{Path, PathBuf};

pub(crate) struct NotarizeOp {
    input_path: PathBuf,
    path_type: PathType,
    bundle_id: String,
    developer_account: String,
    password_keychain_item: String,
    provider: Option<String>,
}

pub(crate) fn run(
    input_path: &Path,
    path_type: PathType,
    bundle_id: &str,
    developer_account: &str,
    password_keychain_item: &str,
    provider: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    NotarizeOp::new(
        input_path.into(),
        path_type,
        bundle_id.into(),
        developer_account.into(),
        password_keychain_item.into(),
        provider.clone(),
    )
    .run()
}
