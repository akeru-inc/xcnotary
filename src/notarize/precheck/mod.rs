mod developer_id;
mod hardened_runtime;
mod no_get_task_allow;
mod secure_timestamp;

mod precheck_error;
use std::error::Error;

pub(self) use precheck_error::PrecheckError;

pub(crate) trait BundleCheck {
    fn display(&self) -> &'static str;
    fn run(&self, bundle_path: &str) -> Result<(), Box<dyn Error>>;
}

pub(super) fn checks() -> Vec<Box<dyn BundleCheck>> {
    let checks: Vec<Box<dyn BundleCheck>> = vec![
        Box::new(developer_id::DeveloperIdCheck),
        Box::new(hardened_runtime::HardenedRuntimeCheck),
        Box::new(no_get_task_allow::NoGetTaskAllowCheck),
        Box::new(secure_timestamp::SecureTimestampCheck),
    ];

    return checks;
}
