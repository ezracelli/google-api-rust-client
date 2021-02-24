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
    pub mod v1 {
        pub mod methods {
            pub mod get_link_stats {
                #[derive(
                    Clone,
                    Debug,
                    PartialEq,
                    derive_builder :: Builder,
                    serde :: Serialize,
                    serde :: Deserialize,
                )]
                pub struct QueryParameters {
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "durationDays")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The span of time requested in days."]
                    pub duration_days: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sdkVersion")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
                    pub sdk_version: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
    #[doc = "Tracking parameters supported by Dynamic Link."]
    pub struct AnalyticsInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googlePlayAnalytics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google Play Campaign Measurements."]
        pub google_play_analytics: ::std::option::Option<::std::boxed::Box<GooglePlayAnalytics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itunesConnectAnalytics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iTunes Connect App Analytics."]
        pub itunes_connect_analytics:
            ::std::option::Option<::std::boxed::Box<ITunesConnectAnalytics>>,
    }
    impl AnalyticsInfo {
        pub fn builder() -> AnalyticsInfoBuilder {
            AnalyticsInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Android related attributes to the Dynamic Link."]
    pub struct AndroidInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidFallbackLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to open on Android if the app is not installed."]
        pub android_fallback_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, this overrides the ‘link’ parameter on Android."]
        pub android_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidMinPackageVersionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum version code for the Android app. If the installed app’s version code is lower, then the user is taken to the Play Store."]
        pub android_min_package_version_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidPackageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Android package name of the app."]
        pub android_package_name: ::std::option::Option<::std::string::String>,
    }
    impl AndroidInfo {
        pub fn builder() -> AndroidInfoBuilder {
            AndroidInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to create a managed Short Dynamic Link."]
    pub struct CreateManagedShortLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicLinkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        pub dynamic_link_info: ::std::option::Option<::std::boxed::Box<DynamicLinkInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longDynamicLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full long Dynamic Link URL with desired query parameters specified. For example, \"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        pub long_dynamic_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link name to associate with the link. It's used for marketer to identify manually-created links in the Firebase console (https://console.firebase.google.com/). Links must be named to be tracked."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        pub sdk_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short Dynamic Link suffix. Optional."]
        pub suffix: ::std::option::Option<::std::boxed::Box<Suffix>>,
    }
    impl CreateManagedShortLinkRequest {
        pub fn builder() -> CreateManagedShortLinkRequestBuilder {
            CreateManagedShortLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to create a short Dynamic Link."]
    pub struct CreateManagedShortLinkResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "managedShortLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        pub managed_short_link: ::std::option::Option<::std::boxed::Box<ManagedShortLink>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previewLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        pub preview_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about potential warnings on link creation."]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicLinkWarning>>>,
    }
    impl CreateManagedShortLinkResponse {
        pub fn builder() -> CreateManagedShortLinkResponseBuilder {
            CreateManagedShortLinkResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to create a short Dynamic Link."]
    pub struct CreateShortDynamicLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicLinkInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the Dynamic Link to be shortened. [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        pub dynamic_link_info: ::std::option::Option<::std::boxed::Box<DynamicLinkInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longDynamicLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full long Dynamic Link URL with desired query parameters specified. For example, \"https://sample.app.goo.gl/?link=http://www.google.com&apn=com.sample\", [Learn more](https://firebase.google.com/docs/reference/dynamic-links/link-shortener)."]
        pub long_dynamic_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        pub sdk_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short Dynamic Link suffix. Optional."]
        pub suffix: ::std::option::Option<::std::boxed::Box<Suffix>>,
    }
    impl CreateShortDynamicLinkRequest {
        pub fn builder() -> CreateShortDynamicLinkRequestBuilder {
            CreateShortDynamicLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to create a short Dynamic Link."]
    pub struct CreateShortDynamicLinkResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "previewLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Preview link to show the link flow chart. (debug info.)"]
        pub preview_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shortLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short Dynamic Link value. e.g. https://abcd.app.goo.gl/wxyz"]
        pub short_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warning")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about potential warnings on link creation."]
        pub warning: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicLinkWarning>>>,
    }
    impl CreateShortDynamicLinkResponse {
        pub fn builder() -> CreateShortDynamicLinkResponseBuilder {
            CreateShortDynamicLinkResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Desktop related attributes to the Dynamic Link."]
    pub struct DesktopInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desktopFallbackLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to open on desktop."]
        pub desktop_fallback_link: ::std::option::Option<::std::string::String>,
    }
    impl DesktopInfo {
        pub fn builder() -> DesktopInfoBuilder {
            DesktopInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Signals associated with the device making the request."]
    pub struct DeviceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceModelName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device model name."]
        pub device_model_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device language code setting."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCodeFromWebview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device language code setting obtained by executing JavaScript code in WebView."]
        pub language_code_from_webview: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCodeRaw")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device language code raw setting. iOS does returns language code in different format than iOS WebView. For example WebView returns en_US, but iOS returns en-US. Field below will return raw value returned by iOS."]
        pub language_code_raw: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenResolutionHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device display resolution height."]
        pub screen_resolution_height: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "screenResolutionWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device display resolution width."]
        pub screen_resolution_width: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device timezone setting."]
        pub timezone: ::std::option::Option<::std::string::String>,
    }
    impl DeviceInfo {
        pub fn builder() -> DeviceInfoBuilder {
            DeviceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dynamic Link event stat."]
    pub struct DynamicLinkEventStat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of times this event occurred."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link event."]
        pub event: ::std::option::Option<DynamicLinkEventStatEventEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Requested platform."]
        pub platform: ::std::option::Option<DynamicLinkEventStatPlatformEnum>,
    }
    impl DynamicLinkEventStat {
        pub fn builder() -> DynamicLinkEventStatBuilder {
            DynamicLinkEventStatBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Link event."]
    pub enum DynamicLinkEventStatEventEnum {
        #[serde(rename = "DYNAMIC_LINK_EVENT_UNSPECIFIED")]
        #[doc = "Unspecified type."]
        DynamicLinkEventUnspecified,
        #[serde(rename = "CLICK")]
        #[doc = "Indicates that an FDL is clicked by users."]
        Click,
        #[serde(rename = "REDIRECT")]
        #[doc = "Indicates that an FDL redirects users to fallback link."]
        Redirect,
        #[serde(rename = "APP_INSTALL")]
        #[doc = "Indicates that an FDL triggers an app install from Play store, currently it's impossible to get stats from App store."]
        AppInstall,
        #[serde(rename = "APP_FIRST_OPEN")]
        #[doc = "Indicates that the app is opened for the first time after an install triggered by FDLs"]
        AppFirstOpen,
        #[serde(rename = "APP_RE_OPEN")]
        #[doc = "Indicates that the app is opened via an FDL for non-first time."]
        AppReOpen,
    }
    impl ::std::default::Default for DynamicLinkEventStatEventEnum {
        fn default() -> Self {
            Self::DynamicLinkEventUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Requested platform."]
    pub enum DynamicLinkEventStatPlatformEnum {
        #[serde(rename = "DYNAMIC_LINK_PLATFORM_UNSPECIFIED")]
        #[doc = "Unspecified platform."]
        DynamicLinkPlatformUnspecified,
        #[serde(rename = "ANDROID")]
        #[doc = "Represents Android platform. All apps and browsers on Android are classfied in this category."]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "Represents iOS platform. All apps and browsers on iOS are classfied in this category."]
        Ios,
        #[serde(rename = "DESKTOP")]
        #[doc = "Represents desktop."]
        Desktop,
        #[serde(rename = "OTHER")]
        #[doc = "Platforms are not categorized as Android/iOS/Destop fall into here."]
        Other,
    }
    impl ::std::default::Default for DynamicLinkEventStatPlatformEnum {
        fn default() -> Self {
            Self::DynamicLinkPlatformUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a Dynamic Link."]
    pub struct DynamicLinkInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyticsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters used for tracking. See all tracking parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        pub analytics_info: ::std::option::Option<::std::boxed::Box<AnalyticsInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Android related information. See Android related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        pub android_info: ::std::option::Option<::std::boxed::Box<AndroidInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desktopInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Desktop related information. See desktop related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        pub desktop_info: ::std::option::Option<::std::boxed::Box<DesktopInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainUriPrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "E.g. https://maps.app.goo.gl, https://maps.page.link, https://g.co/maps More examples can be found in description of getNormalizedUriPrefix in j/c/g/firebase/dynamiclinks/uri/DdlDomain.java Will fallback to dynamic_link_domain is this field is missing"]
        pub domain_uri_prefix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dynamicLinkDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dynamic Links domain that the project owns, e.g. abcd.app.goo.gl [Learn more](https://firebase.google.com/docs/dynamic-links/android/receive) on how to set up Dynamic Link domain associated with your Firebase project. Required if missing domain_uri_prefix."]
        pub dynamic_link_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS related information. See iOS related parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually)."]
        pub ios_info: ::std::option::Option<::std::boxed::Box<IosInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link your app will open, You can specify any URL your app can handle. This link must be a well-formatted URL, be properly URL-encoded, and use the HTTP or HTTPS scheme. See 'link' parameters in the [documentation](https://firebase.google.com/docs/dynamic-links/create-manually). Required."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "navigationInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information of navigation behavior of a Firebase Dynamic Links."]
        pub navigation_info: ::std::option::Option<::std::boxed::Box<NavigationInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socialMetaTagInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters for social meta tag params. Used to set meta tag data for link previews on social sites."]
        pub social_meta_tag_info: ::std::option::Option<::std::boxed::Box<SocialMetaTagInfo>>,
    }
    impl DynamicLinkInfo {
        pub fn builder() -> DynamicLinkInfoBuilder {
            DynamicLinkInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Analytics stats of a Dynamic Link for a given timeframe."]
    pub struct DynamicLinkStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkEventStats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Dynamic Link event stats."]
        pub link_event_stats:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DynamicLinkEventStat>>>,
    }
    impl DynamicLinkStats {
        pub fn builder() -> DynamicLinkStatsBuilder {
            DynamicLinkStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dynamic Links warning messages."]
    pub struct DynamicLinkWarning {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warningCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The warning code."]
        pub warning_code: ::std::option::Option<DynamicLinkWarningWarningCodeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warningDocumentLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The document describing the warning, and helps resolve."]
        pub warning_document_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warningMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The warning message to help developers improve their requests."]
        pub warning_message: ::std::option::Option<::std::string::String>,
    }
    impl DynamicLinkWarning {
        pub fn builder() -> DynamicLinkWarningBuilder {
            DynamicLinkWarningBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The warning code."]
    pub enum DynamicLinkWarningWarningCodeEnum {
        #[serde(rename = "CODE_UNSPECIFIED")]
        #[doc = "Unknown code."]
        CodeUnspecified,
        #[serde(rename = "NOT_IN_PROJECT_ANDROID_PACKAGE_NAME")]
        #[doc = "The Android package does not match any in developer's DevConsole project."]
        NotInProjectAndroidPackageName,
        #[serde(rename = "NOT_INTEGER_ANDROID_PACKAGE_MIN_VERSION")]
        #[doc = "The Android minimum version code has to be a valid integer."]
        NotIntegerAndroidPackageMinVersion,
        #[serde(rename = "UNNECESSARY_ANDROID_PACKAGE_MIN_VERSION")]
        #[doc = "Android package min version param is not needed, e.g. when 'apn' is missing."]
        UnnecessaryAndroidPackageMinVersion,
        #[serde(rename = "NOT_URI_ANDROID_LINK")]
        #[doc = "Android link is not a valid URI."]
        NotUriAndroidLink,
        #[serde(rename = "UNNECESSARY_ANDROID_LINK")]
        #[doc = "Android link param is not needed, e.g. when param 'al' and 'link' have the same value.."]
        UnnecessaryAndroidLink,
        #[serde(rename = "NOT_URI_ANDROID_FALLBACK_LINK")]
        #[doc = "Android fallback link is not a valid URI."]
        NotUriAndroidFallbackLink,
        #[serde(rename = "BAD_URI_SCHEME_ANDROID_FALLBACK_LINK")]
        #[doc = "Android fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeAndroidFallbackLink,
        #[serde(rename = "NOT_IN_PROJECT_IOS_BUNDLE_ID")]
        #[doc = "The iOS bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIosBundleId,
        #[serde(rename = "NOT_IN_PROJECT_IPAD_BUNDLE_ID")]
        #[doc = "The iPad bundle ID does not match any in developer's DevConsole project."]
        NotInProjectIpadBundleId,
        #[serde(rename = "UNNECESSARY_IOS_URL_SCHEME")]
        #[doc = "iOS URL scheme is not needed, e.g. when 'ibi' are 'ipbi' are all missing."]
        UnnecessaryIosUrlScheme,
        #[serde(rename = "NOT_NUMERIC_IOS_APP_STORE_ID")]
        #[doc = "iOS app store ID format is incorrect, e.g. not numeric."]
        NotNumericIosAppStoreId,
        #[serde(rename = "UNNECESSARY_IOS_APP_STORE_ID")]
        #[doc = "iOS app store ID is not needed."]
        UnnecessaryIosAppStoreId,
        #[serde(rename = "NOT_URI_IOS_FALLBACK_LINK")]
        #[doc = "iOS fallback link is not a valid URI."]
        NotUriIosFallbackLink,
        #[serde(rename = "BAD_URI_SCHEME_IOS_FALLBACK_LINK")]
        #[doc = "iOS fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIosFallbackLink,
        #[serde(rename = "NOT_URI_IPAD_FALLBACK_LINK")]
        #[doc = "iPad fallback link is not a valid URI."]
        NotUriIpadFallbackLink,
        #[serde(rename = "BAD_URI_SCHEME_IPAD_FALLBACK_LINK")]
        #[doc = "iPad fallback link has an invalid (non http/https) URI scheme."]
        BadUriSchemeIpadFallbackLink,
        #[serde(rename = "BAD_DEBUG_PARAM")]
        #[doc = "Debug param format is incorrect."]
        BadDebugParam,
        #[serde(rename = "BAD_AD_PARAM")]
        #[doc = "isAd param format is incorrect."]
        BadAdParam,
        #[serde(rename = "DEPRECATED_PARAM")]
        #[doc = "Indicates a certain param is deprecated."]
        DeprecatedParam,
        #[serde(rename = "UNRECOGNIZED_PARAM")]
        #[doc = "Indicates certain paramater is not recognized."]
        UnrecognizedParam,
        #[serde(rename = "TOO_LONG_PARAM")]
        #[doc = "Indicates certain paramater is too long."]
        TooLongParam,
        #[serde(rename = "NOT_URI_SOCIAL_IMAGE_LINK")]
        #[doc = "Social meta tag image link is not a valid URI."]
        NotUriSocialImageLink,
        #[serde(rename = "BAD_URI_SCHEME_SOCIAL_IMAGE_LINK")]
        #[doc = "Social meta tag image link has an invalid (non http/https) URI scheme."]
        BadUriSchemeSocialImageLink,
        #[serde(rename = "NOT_URI_SOCIAL_URL")]
        #[doc = ""]
        NotUriSocialUrl,
        #[serde(rename = "BAD_URI_SCHEME_SOCIAL_URL")]
        #[doc = ""]
        BadUriSchemeSocialUrl,
        #[serde(rename = "LINK_LENGTH_TOO_LONG")]
        #[doc = "Dynamic Link URL length is too long."]
        LinkLengthTooLong,
        #[serde(rename = "LINK_WITH_FRAGMENTS")]
        #[doc = "Dynamic Link URL contains fragments."]
        LinkWithFragments,
        #[serde(rename = "NOT_MATCHING_IOS_BUNDLE_ID_AND_STORE_ID")]
        #[doc = "The iOS bundle ID does not match with the given iOS store ID."]
        NotMatchingIosBundleIdAndStoreId,
    }
    impl ::std::default::Default for DynamicLinkWarningWarningCodeEnum {
        fn default() -> Self {
            Self::CodeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for iSDK to execute strong match flow for post-install attribution. This is meant for iOS requests only. Requests from other platforms will not be honored."]
    pub struct GetIosPostInstallAttributionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appInstallationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App installation epoch time (https://en.wikipedia.org/wiki/Unix_time). This is a client signal for a more accurate weak match."]
        pub app_installation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "APP bundle ID."]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "device")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Device information."]
        pub device: ::std::option::Option<::std::boxed::Box<DeviceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS version, ie: 9.3.5. Consider adding \"build\"."]
        pub ios_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retrievalMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link."]
        pub retrieval_method:
            ::std::option::Option<GetIosPostInstallAttributionRequestRetrievalMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        pub sdk_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueMatchLinkToCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Possible unique matched link that server need to check before performing fingerprint match. If passed link is short server need to expand the link. If link is long server need to vslidate the link."]
        pub unique_match_link_to_check: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visualStyle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie."]
        pub visual_style: ::std::option::Option<GetIosPostInstallAttributionRequestVisualStyleEnum>,
    }
    impl GetIosPostInstallAttributionRequest {
        pub fn builder() -> GetIosPostInstallAttributionRequestBuilder {
            GetIosPostInstallAttributionRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "App post install attribution retrieval information. Disambiguates mechanism (iSDK or developer invoked) to retrieve payload from clicked link."]
    pub enum GetIosPostInstallAttributionRequestRetrievalMethodEnum {
        #[serde(rename = "UNKNOWN_PAYLOAD_RETRIEVAL_METHOD")]
        #[doc = "Unknown method."]
        UnknownPayloadRetrievalMethod,
        #[serde(rename = "IMPLICIT_WEAK_MATCH")]
        #[doc = "iSDK performs a server lookup by device fingerprint in the background when app is first-opened; no API called by developer."]
        ImplicitWeakMatch,
        #[serde(rename = "EXPLICIT_WEAK_MATCH")]
        #[doc = "iSDK performs a server lookup by device fingerprint upon a dev API call."]
        ExplicitWeakMatch,
        #[serde(rename = "EXPLICIT_STRONG_AFTER_WEAK_MATCH")]
        #[doc = "iSDK performs a strong match only if weak match is found upon a dev API call."]
        ExplicitStrongAfterWeakMatch,
    }
    impl ::std::default::Default for GetIosPostInstallAttributionRequestRetrievalMethodEnum {
        fn default() -> Self {
            Self::UnknownPayloadRetrievalMethod
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Strong match page information. Disambiguates between default UI and custom page to present when strong match succeeds/fails to find cookie."]
    pub enum GetIosPostInstallAttributionRequestVisualStyleEnum {
        #[serde(rename = "UNKNOWN_VISUAL_STYLE")]
        #[doc = "Unknown style."]
        UnknownVisualStyle,
        #[serde(rename = "DEFAULT_STYLE")]
        #[doc = "Default style."]
        DefaultStyle,
        #[serde(rename = "CUSTOM_STYLE")]
        #[doc = "Custom style."]
        CustomStyle,
    }
    impl ::std::default::Default for GetIosPostInstallAttributionRequestVisualStyleEnum {
        fn default() -> Self {
            Self::UnknownVisualStyle
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for iSDK to execute strong match flow for post-install attribution."]
    pub struct GetIosPostInstallAttributionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appMinimumVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum version for app, specified by dev through ?imv= parameter. Return to iSDK to allow app to evaluate if current version meets this."]
        pub app_minimum_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence of the returned attribution."]
        pub attribution_confidence:
            ::std::option::Option<GetIosPostInstallAttributionResponseAttributionConfidenceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deepLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deep-link attributed post-install via one of several techniques (fingerprint, copy unique)."]
        pub deep_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalBrowserDestinationLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User-agent specific custom-scheme URIs for iSDK to open. This will be set according to the user-agent tha the click was originally made in. There is no Safari-equivalent custom-scheme open URLs. ie: googlechrome://www.example.com ie: firefox://open-url?url=http://www.example.com ie: opera-http://example.com"]
        pub external_browser_destination_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fallbackLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to navigate to update the app if min version is not met. This is either (in order): 1) fallback link (from ?ifl= parameter, if specified by developer) or 2) AppStore URL (from ?isi= parameter, if specified), or 3) the payload link (from required link= parameter)."]
        pub fallback_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Invitation ID attributed post-install via one of several techniques (fingerprint, copy unique)."]
        pub invitation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isStrongMatchExecutable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instruction for iSDK to attemmpt to perform strong match. For instance, if browser does not support/allow cookie or outside of support browsers, this will be false."]
        pub is_strong_match_executable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes why match failed, ie: \"discarded due to low confidence\". This message will be publicly visible."]
        pub match_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestIpVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which IP version the request was made from."]
        pub request_ip_version:
            ::std::option::Option<GetIosPostInstallAttributionResponseRequestIpVersionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entire FDL (short or long) attributed post-install via one of several techniques (fingerprint, copy unique)."]
        pub requested_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long. Parameters from this should not be used directly (ie: server can default utm_[campaign|medium|source] to a value when requested_link lack them, server determine the best fallback_link when requested_link specifies >1 fallback links)."]
        pub resolved_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmCampaign")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at post-install."]
        pub utm_campaign: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmMedium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion medium value to be propagated by iSDK to Scion at post-install."]
        pub utm_medium: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion source value to be propagated by iSDK to Scion at post-install."]
        pub utm_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmTerm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_term: ::std::option::Option<::std::string::String>,
    }
    impl GetIosPostInstallAttributionResponse {
        pub fn builder() -> GetIosPostInstallAttributionResponseBuilder {
            GetIosPostInstallAttributionResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The confidence of the returned attribution."]
    pub enum GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
        #[serde(rename = "UNKNOWN_ATTRIBUTION_CONFIDENCE")]
        #[doc = "Unset."]
        UnknownAttributionConfidence,
        #[serde(rename = "WEAK")]
        #[doc = "Weak confidence, more than one matching link found or link suspected to be false positive"]
        Weak,
        #[serde(rename = "DEFAULT")]
        #[doc = "Default confidence, match based on fingerprint"]
        Default,
        #[serde(rename = "UNIQUE")]
        #[doc = "Unique confidence, match based on \"unique match link to check\" or other means"]
        Unique,
    }
    impl ::std::default::Default for GetIosPostInstallAttributionResponseAttributionConfidenceEnum {
        fn default() -> Self {
            Self::UnknownAttributionConfidence
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Which IP version the request was made from."]
    pub enum GetIosPostInstallAttributionResponseRequestIpVersionEnum {
        #[serde(rename = "UNKNOWN_IP_VERSION")]
        #[doc = "Unset."]
        UnknownIpVersion,
        #[serde(rename = "IP_V4")]
        #[doc = "Request made from an IPv4 IP address."]
        IpV4,
        #[serde(rename = "IP_V6")]
        #[doc = "Request made from an IPv6 IP address."]
        IpV6,
    }
    impl ::std::default::Default for GetIosPostInstallAttributionResponseRequestIpVersionEnum {
        fn default() -> Self {
            Self::UnknownIpVersion
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests."]
    pub struct GetIosReopenAttributionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "APP bundle ID."]
        pub bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "FDL link to be verified from an app universal link open. The FDL link can be one of: 1) short FDL. e.g. .page.link/, or 2) long FDL. e.g. .page.link/?{query params}, or 3) Invite FDL. e.g. .page.link/i/"]
        pub requested_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sdkVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google SDK version. Version takes the form \"$major.$minor.$patch\""]
        pub sdk_version: ::std::option::Option<::std::string::String>,
    }
    impl GetIosReopenAttributionRequest {
        pub fn builder() -> GetIosReopenAttributionRequestBuilder {
            GetIosReopenAttributionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for iSDK to get reopen attribution for app universal link open deeplinking. This endpoint is meant for only iOS requests."]
    pub struct GetIosReopenAttributionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deepLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deep-link attributed the app universal link open. For both regular FDL links and invite FDL links."]
        pub deep_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional invitation ID, for only invite typed requested FDL links."]
        pub invitation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosMinAppVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "FDL input value of the \"&imv=\" parameter, minimum app version to be returned to Google Firebase SDK running on iOS-9."]
        pub ios_min_app_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The entire FDL, expanded from a short link. It is the same as the requested_link, if it is long."]
        pub resolved_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmCampaign")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion campaign value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_campaign: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion content value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmMedium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion medium value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_medium: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion source value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmTerm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Scion term value to be propagated by iSDK to Scion at app-reopen."]
        pub utm_term: ::std::option::Option<::std::string::String>,
    }
    impl GetIosReopenAttributionResponse {
        pub fn builder() -> GetIosReopenAttributionResponseBuilder {
            GetIosReopenAttributionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for Google Play Campaign Measurements. [Learn more](https://developers.google.com/analytics/devguides/collection/android/v4/campaigns#campaign-params)"]
    pub struct GooglePlayAnalytics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gclid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[AdWords autotagging parameter](https://support.google.com/analytics/answer/1033981?hl=en); used to measure Google AdWords ads. This value is generated dynamically and should never be modified."]
        pub gclid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmCampaign")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign name; used for keyword analysis to identify a specific product promotion or strategic campaign."]
        pub utm_campaign: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign content; used for A/B testing and content-targeted ads to differentiate ads or links that point to the same URL."]
        pub utm_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmMedium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign medium; used to identify a medium such as email or cost-per-click."]
        pub utm_medium: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign source; used to identify a search engine, newsletter, or other source."]
        pub utm_source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "utmTerm")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign term; used with paid search to supply the keywords for ads."]
        pub utm_term: ::std::option::Option<::std::string::String>,
    }
    impl GooglePlayAnalytics {
        pub fn builder() -> GooglePlayAnalyticsBuilder {
            GooglePlayAnalyticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for iTunes Connect App Analytics."]
    pub struct ITunesConnectAnalytics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "at")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Affiliate token used to create affiliate-coded links."]
        pub at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ct")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Campaign text that developers can optionally add to any link in order to track sales from a specific marketing campaign."]
        pub ct: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iTune media types, including music, podcasts, audiobooks and so on."]
        pub mt: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provider token that enables analytics for Dynamic Links from within iTunes Connect."]
        pub pt: ::std::option::Option<::std::string::String>,
    }
    impl ITunesConnectAnalytics {
        pub fn builder() -> ITunesConnectAnalyticsBuilder {
            ITunesConnectAnalyticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "iOS related attributes to the Dynamic Link.."]
    pub struct IosInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosAppStoreId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS App Store ID."]
        pub ios_app_store_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosBundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS bundle ID of the app."]
        pub ios_bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosCustomScheme")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom (destination) scheme to use for iOS. By default, we’ll use the bundle ID as the custom scheme. Developer can override this behavior using this param."]
        pub ios_custom_scheme: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosFallbackLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to open on iOS if the app is not installed."]
        pub ios_fallback_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosIpadBundleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iPad bundle ID of the app."]
        pub ios_ipad_bundle_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosIpadFallbackLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If specified, this overrides the ios_fallback_link value on iPads."]
        pub ios_ipad_fallback_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosMinimumVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iOS minimum version."]
        pub ios_minimum_version: ::std::option::Option<::std::string::String>,
    }
    impl IosInfo {
        pub fn builder() -> IosInfoBuilder {
            IosInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Managed Short Link."]
    pub struct ManagedShortLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation timestamp of the short link."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flaggedAttribute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Attributes that have been flagged about this short url."]
        pub flagged_attribute:
            ::std::option::Option<::std::vec::Vec<ManagedShortLinkFlaggedAttributeEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full Dyamic Link info"]
        pub info: ::std::option::Option<::std::boxed::Box<DynamicLinkInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short durable link url, for example, \"https://sample.app.goo.gl/xyz123\". Required."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link name defined by the creator. Required."]
        pub link_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Visibility status of link."]
        pub visibility: ::std::option::Option<ManagedShortLinkVisibilityEnum>,
    }
    impl ManagedShortLink {
        pub fn builder() -> ManagedShortLinkBuilder {
            ManagedShortLinkBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ManagedShortLinkFlaggedAttributeEnum {
        #[serde(rename = "UNSPECIFIED_ATTRIBUTE")]
        #[doc = "Indicates that no attributes were found for this short url."]
        UnspecifiedAttribute,
        #[serde(rename = "SPAM")]
        #[doc = "Indicates that short url has been flagged by AbuseIAm team as spam."]
        Spam,
    }
    impl ::std::default::Default for ManagedShortLinkFlaggedAttributeEnum {
        fn default() -> Self {
            Self::UnspecifiedAttribute
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Visibility status of link."]
    pub enum ManagedShortLinkVisibilityEnum {
        #[serde(rename = "UNSPECIFIED_VISIBILITY")]
        #[doc = "Visibility of the link is not specified."]
        UnspecifiedVisibility,
        #[serde(rename = "UNARCHIVED")]
        #[doc = "Link created in console and should be shown in console."]
        Unarchived,
        #[serde(rename = "ARCHIVED")]
        #[doc = "Link created in console and should not be shown in console (but can be shown in the console again if it is unarchived)."]
        Archived,
        #[serde(rename = "NEVER_SHOWN")]
        #[doc = "Link created outside of console and should never be shown in console."]
        NeverShown,
    }
    impl ::std::default::Default for ManagedShortLinkVisibilityEnum {
        fn default() -> Self {
            Self::UnspecifiedVisibility
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information of navigation behavior."]
    pub struct NavigationInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableForcedRedirect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this option is on, FDL click will be forced to redirect rather than show an interstitial page."]
        pub enable_forced_redirect: ::std::option::Option<::std::primitive::bool>,
    }
    impl NavigationInfo {
        pub fn builder() -> NavigationInfoBuilder {
            NavigationInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Parameters for social meta tag params. Used to set meta tag data for link previews on social sites."]
    pub struct SocialMetaTagInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socialDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short description of the link. Optional."]
        pub social_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socialImageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image url string. Optional."]
        pub social_image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socialTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title to be displayed. Optional."]
        pub social_title: ::std::option::Option<::std::string::String>,
    }
    impl SocialMetaTagInfo {
        pub fn builder() -> SocialMetaTagInfoBuilder {
            SocialMetaTagInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Short Dynamic Link suffix."]
    pub struct Suffix {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customSuffix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only applies to Option.CUSTOM."]
        pub custom_suffix: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "option")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Suffix option."]
        pub option: ::std::option::Option<SuffixOptionEnum>,
    }
    impl Suffix {
        pub fn builder() -> SuffixBuilder {
            SuffixBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Suffix option."]
    pub enum SuffixOptionEnum {
        #[serde(rename = "OPTION_UNSPECIFIED")]
        #[doc = "The suffix option is not specified, performs as UNGUESSABLE ."]
        OptionUnspecified,
        #[serde(rename = "UNGUESSABLE")]
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] encoded string of a random generated 96 bit random number, which has a length of 17 chars. For example, \"nlAR8U4SlKRZw1cb2\". It prevents other people from guessing and crawling short Dynamic Links that contain personal identifiable information."]
        Unguessable,
        #[serde(rename = "SHORT")]
        #[doc = "Short Dynamic Link suffix is a base62 [0-9A-Za-z] string starting with a length of 4 chars. the length will increase when all the space is occupied."]
        Short,
        #[serde(rename = "CUSTOM")]
        #[doc = "Custom DDL suffix is a client specified string, for example, \"buy2get1free\". NOTE: custom suffix should only be available to managed short link creation"]
        Custom,
    }
    impl ::std::default::Default for SuffixOptionEnum {
        fn default() -> Self {
            Self::OptionUnspecified
        }
    }
}
