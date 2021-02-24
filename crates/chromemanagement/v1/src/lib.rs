#[derive(
    Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
)]
pub struct QueryParameters {
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "$.xgafv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "V1 error format."]
    pub xgafv: ::std::option::Option<QueryParametersXgafvEnum>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "access_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth access token."]
    pub access_token: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ query_parameters_defaults :: alt () }", setter(into))]
    #[serde(rename = "alt")]
    #[serde(default = "query_parameters_defaults :: alt")]
    #[doc = "Data format for response."]
    pub alt: QueryParametersAltEnum,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "callback")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "JSONP"]
    pub callback: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "fields")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Selector specifying which fields to include in a partial response."]
    pub fields: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
    pub key: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "oauth_token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "OAuth 2.0 token for the current user."]
    pub oauth_token: ::std::option::Option<::std::string::String>,
    #[builder(
        default = "{ query_parameters_defaults :: pretty_print () }",
        setter(into)
    )]
    #[serde(rename = "prettyPrint")]
    #[serde(default = "query_parameters_defaults :: pretty_print")]
    #[doc = "Returns response with indentations and line breaks."]
    pub pretty_print: ::std::primitive::bool,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "quotaUser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
    pub quota_user: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "uploadType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
    pub upload_type: ::std::option::Option<::std::string::String>,
    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
    #[serde(rename = "upload_protocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
    pub upload_protocol: ::std::option::Option<::std::string::String>,
}
impl QueryParameters {
    pub fn builder() -> QueryParametersBuilder {
        QueryParametersBuilder::default()
    }
}
mod query_parameters_defaults {
    pub fn alt() -> super::QueryParametersAltEnum {
        serde_json::from_str(&"json").unwrap()
    }
    pub fn pretty_print() -> ::std::primitive::bool {
        serde_json::from_str(&"true").unwrap()
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "V1 error format."]
pub enum QueryParametersXgafvEnum {
    #[serde(rename = "1")]
    #[doc = "v1 error format"]
    _1,
    #[serde(rename = "2")]
    #[doc = "v2 error format"]
    _2,
}
impl ::std::default::Default for QueryParametersXgafvEnum {
    fn default() -> Self {
        Self::_1
    }
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Data format for response."]
pub enum QueryParametersAltEnum {
    #[serde(rename = "json")]
    #[doc = "Responses with Content-Type of application/json"]
    Json,
    #[serde(rename = "media")]
    #[doc = "Media download with context-dependent Content-Type"]
    Media,
    #[serde(rename = "proto")]
    #[doc = "Responses with Content-Type of application/x-protobuf"]
    Proto,
}
impl ::std::default::Default for QueryParametersAltEnum {
    fn default() -> Self {
        Self::Json
    }
}
pub mod resources {
    pub mod customers {
        pub mod resources {
            pub mod reports {
                pub mod methods {
                    pub mod count_chrome_versions {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orgUnitId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID of the organizational unit."]
                            pub org_unit_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return. Maximum and default are 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify the next page in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod count_installed_apps {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * app_name * app_type * install_type * number_of_permissions * total_install_count * latest_profile_active_date * permission_name"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Field used to order results. Supported order by fields: * app_name * app_type * install_type * number_of_permissions * total_install_count"]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orgUnitId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID of the organizational unit."]
                            pub org_unit_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return. Maximum and default are 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify next page in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                    pub mod find_installed_app_devices {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "appId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
                            pub app_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "appType")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Type of the app."]
                            pub app_type: ::std::option::Option<QueryParametersAppTypeEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Query string to filter results, AND-separated fields in EBNF syntax. Note: OR operations are not supported in this filter. Supported filter fields: * last_active_date"]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orderBy")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Field used to order results. Supported order by fields: * machine_name * device_id"]
                            pub order_by: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "orgUnitId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The ID of the organizational unit."]
                            pub org_unit_id: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Maximum number of results to return. Maximum and default are 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Token to specify the next page in the list."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Type of the app."]
                        pub enum QueryParametersAppTypeEnum {
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
                        impl ::std::default::Default for QueryParametersAppTypeEnum {
                            fn default() -> Self {
                                Self::AppTypeUnspecified
                            }
                        }
                    }
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a browser version and its install count."]
    pub struct GoogleChromeManagementV1BrowserVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The release channel of the installed browser."]
        pub channel: ::std::option::Option<GoogleChromeManagementV1BrowserVersionChannelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Count grouped by device_system and major version"]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceOsVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Version of the system-specified operating system."]
        pub device_os_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "system")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The device operating system."]
        pub system: ::std::option::Option<GoogleChromeManagementV1BrowserVersionSystemEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The full version of the installed browser."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleChromeManagementV1BrowserVersion {
        pub fn builder() -> GoogleChromeManagementV1BrowserVersionBuilder {
            GoogleChromeManagementV1BrowserVersionBuilder::default()
        }
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
    impl ::std::default::Default for GoogleChromeManagementV1BrowserVersionChannelEnum {
        fn default() -> Self {
            Self::ReleaseChannelUnspecified
        }
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
    impl ::std::default::Default for GoogleChromeManagementV1BrowserVersionSystemEnum {
        fn default() -> Self {
            Self::DeviceSystemUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing requested browser versions details and counts."]
    pub struct GoogleChromeManagementV1CountChromeVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "browserVersions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of all browser versions and their install counts."]
        pub browser_versions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1BrowserVersion>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to specify the next page in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number browser versions matching request."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleChromeManagementV1CountChromeVersionsResponse {
        pub fn builder() -> GoogleChromeManagementV1CountChromeVersionsResponseBuilder {
            GoogleChromeManagementV1CountChromeVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing details of queried installed apps."]
    pub struct GoogleChromeManagementV1CountInstalledAppsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installedApps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of installed apps matching request."]
        pub installed_apps: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1InstalledApp>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to specify next page in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of installed apps matching request."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleChromeManagementV1CountInstalledAppsResponse {
        pub fn builder() -> GoogleChromeManagementV1CountInstalledAppsResponseBuilder {
            GoogleChromeManagementV1CountInstalledAppsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a device reporting Chrome browser information."]
    pub struct GoogleChromeManagementV1Device {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The ID of the device that reported this Chrome browser information."]
        pub device_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "machine")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The name of the machine within its local network."]
        pub machine: ::std::option::Option<::std::string::String>,
    }
    impl GoogleChromeManagementV1Device {
        pub fn builder() -> GoogleChromeManagementV1DeviceBuilder {
            GoogleChromeManagementV1DeviceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response containing a list of devices with queried app installed."]
    pub struct GoogleChromeManagementV1FindInstalledAppDevicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "devices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of devices which have the app installed. Sorted in ascending alphabetical order on the Device.machine field."]
        pub devices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleChromeManagementV1Device>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to specify the next page in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of devices matching request."]
        pub total_size: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleChromeManagementV1FindInstalledAppDevicesResponse {
        pub fn builder() -> GoogleChromeManagementV1FindInstalledAppDevicesResponseBuilder {
            GoogleChromeManagementV1FindInstalledAppDevicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes an installed app."]
    pub struct GoogleChromeManagementV1InstalledApp {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Unique identifier of the app. For Chrome apps and extensions, the 32-character id (e.g. ehoadneljpdggcbbknedodolkkjodefl). For Android apps, the package name (e.g. com.evernote)."]
        pub app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appInstallType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. How the app was installed."]
        pub app_install_type:
            ::std::option::Option<GoogleChromeManagementV1InstalledAppAppInstallTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Source of the installed app."]
        pub app_source: ::std::option::Option<GoogleChromeManagementV1InstalledAppAppSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Type of the app."]
        pub app_type: ::std::option::Option<GoogleChromeManagementV1InstalledAppAppTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "browserDeviceCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Count of browser devices with this app installed."]
        pub browser_device_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Description of the installed app."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the app is disabled."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Name of the installed app."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "homepageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Homepage uri of the installed app."]
        pub homepage_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "osUserCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Count of ChromeOS users with this app installed."]
        pub os_user_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Permissions of the installed app."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleChromeManagementV1InstalledApp {
        pub fn builder() -> GoogleChromeManagementV1InstalledAppBuilder {
            GoogleChromeManagementV1InstalledAppBuilder::default()
        }
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
    impl ::std::default::Default for GoogleChromeManagementV1InstalledAppAppInstallTypeEnum {
        fn default() -> Self {
            Self::AppInstallTypeUnspecified
        }
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
    impl ::std::default::Default for GoogleChromeManagementV1InstalledAppAppSourceEnum {
        fn default() -> Self {
            Self::AppSourceUnspecified
        }
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
    impl ::std::default::Default for GoogleChromeManagementV1InstalledAppAppTypeEnum {
        fn default() -> Self {
            Self::AppTypeUnspecified
        }
    }
}
