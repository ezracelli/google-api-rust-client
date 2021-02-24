#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a browser version and its install count."]
pub struct GoogleChromeManagementV1BrowserVersion {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The release channel of the installed browser."]
    pub channel: ::std::option::Option<GoogleChromeManagementV1BrowserVersionChannelEnum>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Count grouped by device_system and major version"]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deviceOsVersion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Version of the system-specified operating system."]
    pub device_os_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "system")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The device operating system."]
    pub system: ::std::option::Option<GoogleChromeManagementV1BrowserVersionSystemEnum>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The full version of the installed browser."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The release channel of the installed browser."]
pub enum GoogleChromeManagementV1BrowserVersionChannelEnum {
    #[serde(rename = "RELEASE_CHANNEL_UNSPECIFIED")]
    #[doc = "No release channel specified."]
    ReleaseChannelUnspecified,
    #[serde(rename = "CANARY")]
    #[doc = "Canary release channel."]
    Canary,
    #[serde(rename = "DEV")]
    #[doc = "Dev release channel."]
    Dev,
    #[serde(rename = "BETA")]
    #[doc = "Beta release channel."]
    Beta,
    #[serde(rename = "STABLE")]
    #[doc = "Stable release channel."]
    Stable,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The device operating system."]
pub enum GoogleChromeManagementV1BrowserVersionSystemEnum {
    #[serde(rename = "DEVICE_SYSTEM_UNSPECIFIED")]
    #[doc = "No operating system specified."]
    DeviceSystemUnspecified,
    #[serde(rename = "SYSTEM_OTHER")]
    #[doc = "Other operating system."]
    SystemOther,
    #[serde(rename = "SYSTEM_ANDROID")]
    #[doc = "Android operating system."]
    SystemAndroid,
    #[serde(rename = "SYSTEM_IOS")]
    #[doc = "Apple iOS operating system."]
    SystemIos,
    #[serde(rename = "SYSTEM_CROS")]
    #[doc = "Chrome OS operating system."]
    SystemCros,
    #[serde(rename = "SYSTEM_WINDOWS")]
    #[doc = "Microsoft Windows operating system."]
    SystemWindows,
    #[serde(rename = "SYSTEM_MAC")]
    #[doc = "Apple macOS operating system."]
    SystemMac,
    #[serde(rename = "SYSTEM_LINUX")]
    #[doc = "Linux operating system."]
    SystemLinux,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing requested browser versions details and counts."]
pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
    #[serde(rename = "browserVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all browser versions and their install counts."]
    pub browser_versions: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1BrowserVersion>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to specify the next page in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number browser versions matching request."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing details of queried installed apps."]
pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
    #[serde(rename = "installedApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of installed apps matching request."]
    pub installed_apps: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1InstalledApp>>,
    >,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to specify next page in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of installed apps matching request."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes a device reporting Chrome browser information."]
pub struct GoogleChromeManagementV1Device {
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of the device that reported this Chrome browser information."]
    pub device_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "machine")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The name of the machine within its local network."]
    pub machine: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response containing a list of devices with queried app installed."]
pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field."]
    pub devices:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1Device>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token to specify the next page in the list."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total number of devices matching request."]
    pub total_size: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Describes an installed app."]
pub struct GoogleChromeManagementV1InstalledApp {
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
    pub app_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appInstallType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. How the app was installed."]
    pub app_install_type:
        ::std::option::Option<GoogleChromeManagementV1InstalledAppAppInstallTypeEnum>,
    #[serde(rename = "appSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Source of the installed app."]
    pub app_source: ::std::option::Option<GoogleChromeManagementV1InstalledAppAppSourceEnum>,
    #[serde(rename = "appType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Type of the app."]
    pub app_type: ::std::option::Option<GoogleChromeManagementV1InstalledAppAppTypeEnum>,
    #[serde(rename = "browserDeviceCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Count of browser devices with this app installed."]
    pub browser_device_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Description of the installed app."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disabled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Whether the app is disabled."]
    pub disabled: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Name of the installed app."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "homepageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Homepage uri of the installed app."]
    pub homepage_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "osUserCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Count of ChromeOS users with this app installed."]
    pub os_user_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Permissions of the installed app."]
    pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. How the app was installed."]
pub enum GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
    #[serde(rename = "APP_INSTALL_TYPE_UNSPECIFIED")]
    #[doc = "Application install type not specified."]
    AppInstallTypeUnspecified,
    #[serde(rename = "MULTIPLE")]
    #[doc = "Multiple app install types."]
    Multiple,
    #[serde(rename = "NORMAL")]
    #[doc = "Normal app install type."]
    Normal,
    #[serde(rename = "ADMIN")]
    #[doc = "Administrator app install type."]
    Admin,
    #[serde(rename = "DEVELOPMENT")]
    #[doc = "Development app install type."]
    Development,
    #[serde(rename = "SIDELOAD")]
    #[doc = "Sideloaded app install type."]
    Sideload,
    #[serde(rename = "OTHER")]
    #[doc = "Other app install type."]
    Other,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Source of the installed app."]
pub enum GoogleChromeManagementV1InstalledAppAppSourceEnum {
    #[serde(rename = "APP_SOURCE_UNSPECIFIED")]
    #[doc = "Application source not specified."]
    AppSourceUnspecified,
    #[serde(rename = "CHROME_WEBSTORE")]
    #[doc = "Generally for extensions and Chrome apps."]
    ChromeWebstore,
    #[serde(rename = "PLAY_STORE")]
    #[doc = "Play Store app."]
    PlayStore,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. Type of the app."]
pub enum GoogleChromeManagementV1InstalledAppAppTypeEnum {
    #[serde(rename = "APP_TYPE_UNSPECIFIED")]
    #[doc = "App type not specified."]
    AppTypeUnspecified,
    #[serde(rename = "EXTENSION")]
    #[doc = "Chrome extension."]
    Extension,
    #[serde(rename = "APP")]
    #[doc = "Chrome app."]
    App,
    #[serde(rename = "THEME")]
    #[doc = "Chrome theme."]
    Theme,
    #[serde(rename = "HOSTED_APP")]
    #[doc = "Chrome hosted app."]
    HostedApp,
    #[serde(rename = "ANDROID_APP")]
    #[doc = "ARC++ app."]
    AndroidApp,
}
