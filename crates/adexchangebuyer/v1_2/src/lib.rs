#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration data for an Ad Exchange buyer account."]
pub struct Account {
    #[serde(rename = "bidderLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Your bidder locations that have distinct URLs."]
    pub bidder_location: ::std::option::Option<::std::vec::Vec<AccountBidderLocation>>,
    #[serde(rename = "cookieMatchingNid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The nid parameter value used in cookie match requests. Please contact your technical account manager if you need to change this."]
    pub cookie_matching_nid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cookieMatchingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The base URL used in cookie match requests."]
    pub cookie_matching_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account id."]
    pub id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "kind")]
    #[serde(default = "account_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "maximumActiveCreatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum number of active creatives that an account can have, where a creative is active if it was inserted or bid with in the last 30 days. Please contact your technical account manager if you need to change this."]
    pub maximum_active_creatives: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maximumTotalQps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sum of all bidderLocation.maximumQps values cannot exceed this. Please contact your technical account manager if you need to change this."]
    pub maximum_total_qps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numberActiveCreatives")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of creatives that this account inserted or bid with in the last 30 days."]
    pub number_active_creatives: ::std::option::Option<::std::primitive::i64>,
}
mod account_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#account")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountBidderLocation {
    #[serde(rename = "maximumQps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum queries per second the Ad Exchange will send."]
    pub maximum_qps: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographical region the Ad Exchange should send requests from. Only used by some quota systems, but always setting the value is recommended. Allowed values:  \n- ASIA \n- EUROPE \n- US_EAST \n- US_WEST"]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to which the Ad Exchange will send bid requests."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An account feed lists Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single buyer account."]
pub struct AccountsList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of accounts."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
    #[serde(rename = "kind")]
    #[serde(default = "accounts_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod accounts_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#accountsList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A creative and its classification data."]
pub struct Creative {
    #[serde(rename = "HTMLSnippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTML snippet that displays the ad when inserted in the web page. If set, videoURL should not be set."]
    pub html_snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account id."]
    pub account_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected advertiser id, if any. Read-only. This field should not be set in requests."]
    pub advertiser_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "advertiserName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the company being advertised in the creative."]
    pub advertiser_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "agencyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The agency id for this creative."]
    pub agency_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "apiUploadTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last upload timestamp of this creative if it was uploaded via API. Read-only. The value of this field is generated, and will be ignored for uploads. (formatted RFC 3339 timestamp)."]
    pub api_upload_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All attributes for the ads that may be shown from this snippet."]
    pub attribute: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "buyerCreativeId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A buyer-specific id identifying the creative in this ad."]
    pub buyer_creative_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickThroughUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of destination urls for the snippet."]
    pub click_through_url: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "corrections")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shows any corrections that were applied to this creative. Read-only. This field should not be set in requests."]
    pub corrections: ::std::option::Option<::std::vec::Vec<CreativeCorrections>>,
    #[serde(rename = "disapprovalReasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reasons for disapproval, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue. Read-only. This field should not be set in requests."]
    pub disapproval_reasons: ::std::option::Option<::std::vec::Vec<CreativeDisapprovalReasons>>,
    #[serde(rename = "filteringReasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filtering reasons for the creative. Read-only. This field should not be set in requests."]
    pub filtering_reasons: ::std::option::Option<CreativeFilteringReasons>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad height."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "impressionTrackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of urls to be called to record an impression."]
    pub impression_tracking_url: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(default = "creative_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "productCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected product categories, if any. Read-only. This field should not be set in requests."]
    pub product_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "restrictedCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All restricted categories for the ads that may be shown from this snippet."]
    pub restricted_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "sensitiveCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected sensitive categories, if any. Read-only. This field should not be set in requests."]
    pub sensitive_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative serving status. Read-only. This field should not be set in requests."]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vendorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All vendor types for the ads that may be shown from this snippet."]
    pub vendor_type: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version for this creative. Read-only. This field should not be set in requests."]
    pub version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "videoURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url to fetch a video ad. If set, HTMLSnippet should not be set."]
    pub video_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ad width."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
mod creative_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#creative")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeCorrections {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional details about the correction."]
    pub details: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of correction that was applied to the creative."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeDisapprovalReasons {
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional details about the reason for disapproval."]
    pub details: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The categorized reason for disapproval."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The filtering reasons for the creative. Read-only. This field should not be set in requests."]
pub struct CreativeFilteringReasons {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date in ISO 8601 format for the data. The data is collected from 00:00:00 to 23:59:59 in PST."]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filtering reasons."]
    pub reasons: ::std::option::Option<::std::vec::Vec<CreativeFilteringReasonsReasons>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeFilteringReasonsReasons {
    #[serde(rename = "filteringCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of times the creative was filtered for the status. The count is aggregated across all publishers on the exchange."]
    pub filtering_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "filteringStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The filtering status code. Please refer to the creative-status-codes.txt file for different statuses."]
    pub filtering_status: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The creatives feed lists the active creatives for the Ad Exchange buyer accounts that the user has access to. Each entry in the feed corresponds to a single creative."]
pub struct CreativesList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of creatives."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Creative>>>,
    #[serde(rename = "kind")]
    #[serde(default = "creatives_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Continuation token used to page through creatives. To retrieve the next page of results, set the next request's \"pageToken\" value to this."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
mod creatives_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#creativesList")
    }
}
