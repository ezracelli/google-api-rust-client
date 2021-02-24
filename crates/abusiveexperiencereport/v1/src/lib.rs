#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for GetSiteSummary."]
pub struct SiteSummaryResponse {
    #[serde(rename = "abusiveStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's Abusive Experience Report status."]
    pub abusive_status: ::std::option::Option<SiteSummaryResponseAbusiveStatusEnum>,
    #[serde(rename = "enforcementTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which [enforcement](https://support.google.com/webtools/answer/7538608) against the site began or will begin. Not set when the filter_status is OFF."]
    pub enforcement_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filterStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The site's [enforcement status](https://support.google.com/webtools/answer/7538608)."]
    pub filter_status: ::std::option::Option<SiteSummaryResponseFilterStatusEnum>,
    #[serde(rename = "lastChangeTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time at which the site's status last changed."]
    pub last_change_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reportUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A link to the full Abusive Experience Report for the site. Not set in ViolatingSitesResponse. Note that you must complete the [Search Console verification process](https://support.google.com/webmasters/answer/9008080) for the site before you can access the full report."]
    pub report_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reviewedSite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the reviewed site, e.g. `google.com`."]
    pub reviewed_site: ::std::option::Option<::std::string::String>,
    #[serde(rename = "underReview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the site is currently under review."]
    pub under_review: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The site's Abusive Experience Report status."]
pub enum SiteSummaryResponseAbusiveStatusEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "Not reviewed."]
    Unknown,
    #[serde(rename = "PASSING")]
    #[doc = "Passing."]
    Passing,
    #[serde(rename = "FAILING")]
    #[doc = "Failing."]
    Failing,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The site's [enforcement status](https://support.google.com/webtools/answer/7538608)."]
pub enum SiteSummaryResponseFilterStatusEnum {
    #[serde(rename = "UNKNOWN")]
    #[doc = "N/A."]
    Unknown,
    #[serde(rename = "ON")]
    #[doc = "Enforcement is on."]
    On,
    #[serde(rename = "OFF")]
    #[doc = "Enforcement is off."]
    Off,
    #[serde(rename = "PAUSED")]
    #[doc = "Enforcement is paused."]
    Paused,
    #[serde(rename = "PENDING")]
    #[doc = "Enforcement is pending."]
    Pending,
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
