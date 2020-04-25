use console::Style;
use std::error::Error;
use std::fmt;

static DEFAULT_HELP_URL:&str = "https://developer.apple.com/documentation/xcode/notarizing_macos_software_before_distribution/resolving_common_notarization_issues";

pub(crate) enum Status {
    Pass,
    Fail {
        message: String,
        solution: String,
        see_also: Option<String>,
    },
}

impl Status {
    pub(super) fn fail_with(message: &str, solution: &str, see_also: Option<String>) -> Self {
        Status::Fail {
            message: message.into(),
            solution: solution.into(),
            see_also,
        }
    }

    pub(super) fn to_err(self) -> Option<PrecheckError> {
        match self {
            Self::Pass => None,
            Self::Fail {
                message,
                solution,
                see_also,
            } => Some(PrecheckError {
                message,
                solution,
                see_also,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct PrecheckError {
    message: String,
    solution: String,
    see_also: Option<String>,
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
