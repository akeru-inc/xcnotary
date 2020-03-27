mod precheck;
mod run;

use std::error::Error;
use std::path::PathBuf;

pub(crate) struct NotarizeOp<'a> {
    bundle_id: &'a str,
    bundle_path: PathBuf,
    developer_account: &'a str,
    password_keychain_item: &'a str,
}

impl<'a> NotarizeOp<'a> {
    pub(crate) fn new(
        bundle_id: &'a str,
        bundle_path: PathBuf,
        developer_account: &'a str,
        password_keychain_item: &'a str,
    ) -> NotarizeOp<'a> {
        NotarizeOp {
            bundle_id,
            bundle_path,
            developer_account,
            password_keychain_item,
        }
    }

    pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
        run::notarize(&self)
    }
}
