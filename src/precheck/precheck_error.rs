use console::Style;
use std::error::Error;
use std::fmt;

static DEFAULT_HELP_URL:&str = "https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/resolving_common_notarization_issues";

#[derive(Debug, Clone)]
pub(crate) struct PrecheckError {
    message: String,
    solution: String,
    see_also: Option<String>,
}

impl PrecheckError {
    pub(super) fn new(message: &str, solution: &str) -> Self {
        PrecheckError {
            message: message.into(),
            solution: solution.into(),
            see_also: None,
        }
    }

    #[allow(dead_code)]
    pub(super) fn new_with_url(message: &str, solution: &str, see_also: &str) -> Self {
        PrecheckError {
            message: message.into(),
            solution: solution.into(),
            see_also: Some(see_also.into()),
        }
    }
}

impl std::fmt::Display for PrecheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let heading_style = Style::new().white().bold();
        let error_style = Style::new().red().bold();

        write!(
            f,
            r#"{}
{}

{}
   {}

{}
   {}"#,
            error_style.apply_to("Pre-notarization check failed:"),
            self.message,
            heading_style.apply_to("Suggested fix:"),
            self.solution,
            heading_style.apply_to("See also:"),
            self.see_also
                .as_ref()
                .unwrap_or(&DEFAULT_HELP_URL.to_string())
        )
    }
}

impl Error for PrecheckError {}
