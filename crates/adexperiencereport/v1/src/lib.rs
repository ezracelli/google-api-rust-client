#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A site's Ad Experience Report summary on a single platform."]
pub struct PlatformSummary {
    #[serde(rename = "betterAdsStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's Ad Experience Report status on this platform."]
    pub better_ads_status: ::std::option::Option<PlatformSummaryBetterAdsStatusEnum>,
    #[serde(rename = "enforcementTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which [enforcement](https://support.google.com/webtools/answer/7308033) against the site began or will begin on this platform. Not set when the filter_status is OFF."]
    pub enforcement_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filterStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's [enforcement status](https://support.google.com/webtools/answer/7308033) on this platform."]
    pub filter_status: ::std::option::Option<PlatformSummaryFilterStatusEnum>,
    #[serde(rename = "lastChangeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the site's status last changed on this platform."]
    pub last_change_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's regions on this platform. No longer populated, because there is no longer any semantic difference between sites in different regions."]
    pub region: ::std::option::Option<::std::vec::Vec<PlatformSummaryRegionEnum>>,
    #[serde(rename = "reportUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the full Ad Experience Report for the site on this platform.. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report."]
    pub report_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "underReview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the site is currently under review on this platform."]
    pub under_review: ::std::option::Option<::std::primitive::bool>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GetSiteSummary."]
pub struct SiteSummaryResponse {
    #[serde(rename = "desktopSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's Ad Experience Report summary on desktop."]
    pub desktop_summary: ::std::option::Option<::std::boxed::Box<PlatformSummary>>,
    #[serde(rename = "mobileSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's Ad Experience Report summary on mobile."]
    pub mobile_summary: ::std::option::Option<::std::boxed::Box<PlatformSummary>>,
    #[serde(rename = "reviewedSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the reviewed site, e.g. `google.com`."]
    pub reviewed_site: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for ListViolatingSites."]
pub struct ViolatingSitesResponse {
    #[serde(rename = "violatingSites")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of violating sites."]
    pub violating_sites:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SiteSummaryResponse>>>,
}
