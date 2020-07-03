use std::path::Path;
use std::str;

pub mod structs;

pub(crate) fn bundle_info_from_file<P: AsRef<Path>>(path: P) -> structs::BundleInfo {
    plist::from_file(path).unwrap()
}

pub(crate) fn notarization_upload_response(bytes: &[u8]) -> structs::NotarizationUpload {
    plist::from_bytes(bytes).unwrap()
}

pub(crate) fn notarization_status_response(bytes: &[u8]) -> structs::NotarizationInfo {
    plist::from_bytes(bytes).unwrap()
}

pub(crate) fn bundle_entitlemens(bytes: &[u8]) -> structs::BundleEntitlements {
    plist::from_bytes(bytes).unwrap()
}

#[cfg(test)]
mod tests {
    use super::structs::*;
    use tempfile::NamedTempFile;

    static REQUEST_UUID: &str = "c0dec0de-1234-5678-1234-b4d961a1d182";

    static VALID_INFO_PLIST: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>CFBundleIdentifier</key>
        <string>com.example.helloworld</string>
        <key>CFBundleName</key>
        <string>HelloWorld</string>
        <key>CFBundleShortVersionString</key>
        <string>1.0.14</string>        
        <key>CFBundleVersion</key>
        <string>14</string>
    </dict>
    </plist>
"#;

    static VALID_IN_PROGRESS_NOTARIZATION_INFO_PLIST: &str = r#"
    <?xml version="1.0" encoding="UTF-8"?>
    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
    <plist version="1.0">
    <dict>
        <key>notarization-info</key>
        <dict>
            <key>Date</key>
            <date>2020-03-01T00:00:00Z</date>
            <key>Hash</key>
            <string>3caffa321f3adb01d1e0eebabfd8bdb5dbfcfc467522b903ec7b64fdad24ada8</string>
            <key>RequestUUID</key>
            <string>c0dec0de-1234-5678-1234-b4d961a1d182</string>
            <key>Status</key>
            <string>in progress</string>
        </dict>
        <key>os-version</key>
        <string>10.15.3</string>
        <key>success-message</key>
        <string>No errors getting notarization info.</string>
        <key>tool-path</key>
        <string>/Applications/Xcode.app/Contents/SharedFrameworks/ContentDeliveryServices.framework/Versions/A/Frameworks/AppStoreService.framework</string>
        <key>tool-version</key>
        <string>4.00.1181</string>
    </dict>
    </plist>
    "#;

    static VALID_SUCCESS_NOTARIZATION_INFO_PLIST: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>notarization-info</key>
	<dict>
		<key>Date</key>
		<date>2020-03-01T00:00:00Z</date>
		<key>Hash</key>
		<string>3caffa321f3adb01d1e0eebabfd8bdb5dbfcfc467522b903ec7b64fdad24ada8</string>
		<key>LogFileURL</key>
		<string>https://osxapps-ssl.itunes.apple.com/foo</string>
		<key>RequestUUID</key>
		<string>c0dec0de-1234-5678-1234-b4d961a1d182</string>
		<key>Status</key>
		<string>success</string>
		<key>Status Code</key>
		<integer>0</integer>
		<key>Status Message</key>
		<string>Package Approved</string>
	</dict>
	<key>os-version</key>
	<string>10.15.3</string>
	<key>success-message</key>
	<string>No errors getting notarization info.</string>
	<key>tool-path</key>
	<string>/Applications/Xcode.app/Contents/SharedFrameworks/ContentDeliveryServices.framework/Versions/A/Frameworks/AppStoreService.framework</string>
	<key>tool-version</key>
	<string>4.00.1181</string>
</dict>
</plist>
"#;

    static VALID_SUCCESS_UPLOAD_PLIST: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>notarization-upload</key>
	<dict>
		<key>RequestUUID</key>
		<string>c0dec0de-1234-5678-1234-b4d961a1d182</string>
	</dict>
	<key>os-version</key>
	<string>10.15.3</string>
	<key>success-message</key>
	<string>No errors uploading 'foo.zip'.</string>
	<key>tool-path</key>
	<string>/Applications/Xcode.app/Contents/SharedFrameworks/ContentDeliveryServices.framework/Versions/A/Frameworks/AppStoreService.framework</string>
	<key>tool-version</key>
	<string>4.00.1181</string>
</dict>
</plist>
"#;

    static ENTITLEMENTS_OUTPUT:&str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.get-task-allow</key>
    <true/>
</dict>
</plist>
    "#;

    #[test]
    fn test_deserialize_notarization_success_info() {
        let result =
            super::notarization_status_response(VALID_SUCCESS_NOTARIZATION_INFO_PLIST.as_bytes());

        assert_eq!(result.details.request_uuid, REQUEST_UUID);
        assert!(if let NotarizationStatus::Success = result.details.status {
            true
        } else {
            false
        });

        assert!(result.details.logfile_url.is_some());
        assert!(result.details.logfile_url.unwrap().starts_with("https"));

        assert!(result.success_message.starts_with("No errors"));
    }

    #[test]
    fn test_deserialize_notarization_in_progress_info() {
        let result = super::notarization_status_response(
            VALID_IN_PROGRESS_NOTARIZATION_INFO_PLIST.as_bytes(),
        );

        assert_eq!(result.details.request_uuid, REQUEST_UUID);
        assert!(
            if let NotarizationStatus::InProgress = result.details.status {
                true
            } else {
                false
            }
        );

        assert!(result.details.logfile_url.is_none());
        assert!(result.success_message.starts_with("No errors"));
    }

    #[test]
    fn test_deserialize_notarization_upload() {
        let result = super::notarization_upload_response(VALID_SUCCESS_UPLOAD_PLIST.as_bytes());
        assert_eq!(result.details.request_uuid, REQUEST_UUID);
        assert!(result.success_message.starts_with("No errors"));
    }

    #[test]
    fn test_deserialize_info_plist() {
        let temp_file = NamedTempFile::new().unwrap();
        std::fs::write(&temp_file.path().as_os_str(), VALID_INFO_PLIST).unwrap();

        let result = super::bundle_info_from_file(temp_file);

        assert_eq!(result.id, "com.example.helloworld");
        assert!(result.name.starts_with("HelloWorld"));
    }

    #[test]
    fn test_parse_entitlements() {
        let result = super::bundle_entitlemens(ENTITLEMENTS_OUTPUT.as_bytes());

        assert!(result.get_task_allow.is_some());
        assert!(result.get_task_allow.unwrap());
    }
}
