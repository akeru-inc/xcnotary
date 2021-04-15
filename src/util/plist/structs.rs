use serde::de;
use serde::de::{Deserializer, Unexpected, Visitor};
use serde::Deserialize;
use std::fmt;

/// Info.plist
#[derive(Deserialize, Debug)]
pub(crate) struct BundleInfo {
    #[serde(rename = "CFBundleIdentifier")]
    pub(crate) id: String,
    #[serde(rename = "CFBundleVersion")]
    pub(crate) version: String,
    #[serde(rename = "CFBundleShortVersionString")]
    pub(crate) short_version_string: String,
}

/// Response from altool --notarization-info
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct NotarizationInfo {
    #[serde(rename = "notarization-info")]
    pub(crate) details: NotarizationInfoDetails,
    pub(crate) success_message: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct NotarizationInfoDetails {
    #[serde(rename = "LogFileURL")]
    pub(crate) logfile_url: Option<String>,
    #[serde(rename = "RequestUUID")]
    pub(crate) request_uuid: String,
    #[serde(rename = "Status")]
    #[serde(deserialize_with = "notarization_status_from_string")]
    pub(crate) status: NotarizationStatus,
}

#[derive(Debug)]
pub(crate) enum NotarizationStatus {
    InProgress,
    Success,
    Invalid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct BundleEntitlements {
    #[serde(rename = "com.apple.security.get-task-allow")]
    pub(crate) get_task_allow: Option<bool>,
}

/// Response from altool --upload-app
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct NotarizationUpload {
    #[serde(rename = "notarization-upload")]
    pub(crate) details: NotarizationUploadDetails,
    pub(crate) success_message: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct NotarizationUploadDetails {
    #[serde(rename = "RequestUUID")]
    pub(crate) request_uuid: String,
}

struct NotarizationStatusString;

impl<'de> Visitor<'de> for NotarizationStatusString {
    type Value = NotarizationStatus;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "one of NotarizationStatus values")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s {
            "in progress" => Ok(NotarizationStatus::InProgress),
            "success" => Ok(NotarizationStatus::Success),
            "invalid" => Ok(NotarizationStatus::Invalid),
            _ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
        }
    }
}

fn notarization_status_from_string<'de, D>(d: D) -> Result<NotarizationStatus, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_any(NotarizationStatusString)
}
