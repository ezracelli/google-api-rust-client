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
        serde_json::from_str(&"\"json\"").unwrap()
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A site's Ad Experience Report summary on a single platform."]
    pub struct PlatformSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "betterAdsStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site's Ad Experience Report status on this platform."]
        pub better_ads_status: ::std::option::Option<PlatformSummaryBetterAdsStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enforcementTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which [enforcement](https://support.google.com/webtools/answer/7308033) against the site began or will begin on this platform. Not set when the filter_status is OFF."]
        pub enforcement_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site's [enforcement status](https://support.google.com/webtools/answer/7308033) on this platform."]
        pub filter_status: ::std::option::Option<PlatformSummaryFilterStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastChangeTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time at which the site's status last changed on this platform."]
        pub last_change_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site's regions on this platform. No longer populated, because there is no longer any semantic difference between sites in different regions."]
        pub region: ::std::option::Option<::std::vec::Vec<PlatformSummaryRegionEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reportUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A link to the full Ad Experience Report for the site on this platform.. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report."]
        pub report_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "underReview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the site is currently under review on this platform."]
        pub under_review: ::std::option::Option<::std::primitive::bool>,
    }
    impl PlatformSummary {
        pub fn builder() -> PlatformSummaryBuilder {
            PlatformSummaryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The site's Ad Experience Report status on this platform."]
    pub enum PlatformSummaryBetterAdsStatusEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "Not reviewed."]
        Unknown,
        #[serde(rename = "PASSING")]
        #[doc = "Passing."]
        Passing,
        #[serde(rename = "WARNING")]
        #[doc = "Warning. No longer a possible status."]
        Warning,
        #[serde(rename = "FAILING")]
        #[doc = "Failing."]
        Failing,
    }
    impl ::std::default::Default for PlatformSummaryBetterAdsStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The site's [enforcement status](https://support.google.com/webtools/answer/7308033) on this platform."]
    pub enum PlatformSummaryFilterStatusEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "N/A."]
        Unknown,
        #[serde(rename = "ON")]
        #[doc = "Ad filtering is on."]
        On,
        #[serde(rename = "OFF")]
        #[doc = "Ad filtering is off."]
        Off,
        #[serde(rename = "PAUSED")]
        #[doc = "Ad filtering is paused."]
        Paused,
        #[serde(rename = "PENDING")]
        #[doc = "Ad filtering is pending."]
        Pending,
    }
    impl ::std::default::Default for PlatformSummaryFilterStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PlatformSummaryRegionEnum {
        #[serde(rename = "REGION_UNKNOWN")]
        #[doc = "Ad standard not yet defined for your region."]
        RegionUnknown,
        #[serde(rename = "REGION_A")]
        #[doc = "Region A."]
        RegionA,
        #[serde(rename = "REGION_B")]
        #[doc = "Region B."]
        RegionB,
        #[serde(rename = "REGION_C")]
        #[doc = "Region C."]
        RegionC,
    }
    impl ::std::default::Default for PlatformSummaryRegionEnum {
        fn default() -> Self {
            Self::RegionUnknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for GetSiteSummary."]
    pub struct SiteSummaryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "desktopSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site's Ad Experience Report summary on desktop."]
        pub desktop_summary: ::std::option::Option<::std::boxed::Box<PlatformSummary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The site's Ad Experience Report summary on mobile."]
        pub mobile_summary: ::std::option::Option<::std::boxed::Box<PlatformSummary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reviewedSite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the reviewed site, e.g. `google.com`."]
        pub reviewed_site: ::std::option::Option<::std::string::String>,
    }
    impl SiteSummaryResponse {
        pub fn builder() -> SiteSummaryResponseBuilder {
            SiteSummaryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListViolatingSites."]
    pub struct ViolatingSitesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "violatingSites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of violating sites."]
        pub violating_sites:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SiteSummaryResponse>>>,
    }
    impl ViolatingSitesResponse {
        pub fn builder() -> ViolatingSitesResponseBuilder {
            ViolatingSitesResponseBuilder::default()
        }
    }
}
