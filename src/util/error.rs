use console::Style;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum OperationError {
    Text(String),
    Detailed(String, String), // message: String,
}

impl OperationError {
    pub(crate) fn new(message: &str) -> Self {
        OperationError::Text(message.into())
    }

    pub(crate) fn detail(heading: &str, message: &str) -> Self {
        OperationError::Detailed(heading.into(), message.into())
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_color = Style::new().red().bold();

        match self {
            OperationError::Text(text) => write!(f, "{}{}", error_color.apply_to("Error: "), text),
            OperationError::Detailed(heading, text) => {
                write!(f, "{}:\n{}", error_color.apply_to(heading), text)
            }
        }
    }
}

impl From<String> for OperationError {
    fn from(err: String) -> Self {
        OperationError::new(&err)
    }
}

impl Error for OperationError {}
