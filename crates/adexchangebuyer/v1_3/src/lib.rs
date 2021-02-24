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
#[doc = "The configuration data for an Ad Exchange billing info."]
pub struct BillingInfo {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account id."]
    pub account_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "accountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account name."]
    pub account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "billingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of adgroup IDs associated with this particular account. These IDs may show up as part of a realtime bidding BidRequest, which indicates a bid request for this account."]
    pub billing_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(default = "billing_info_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod billing_info_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#billingInfo")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A billing info feed lists Billing Info the Ad Exchange buyer account has access to. Each entry in the feed corresponds to a single billing info."]
pub struct BillingInfoList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of billing info relevant for your account."]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BillingInfo>>>,
    #[serde(rename = "kind")]
    #[serde(default = "billing_info_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod billing_info_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#billingInfoList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration data for Ad Exchange RTB - Budget API."]
pub struct Budget {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id of the account. This is required for get and update requests."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "billingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billing id to determine which adgroup to provide budget information for. This is required for get and update requests."]
    pub billing_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "budgetAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The daily budget amount in unit amount of the account currency to apply for the billingId provided. This is required for update requests."]
    pub budget_amount: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code for the buyer. This cannot be altered here."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique id that describes this item."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "budget_defaults :: kind")]
    #[doc = "The kind of the resource, i.e. \"adexchangebuyer#budget\"."]
    pub kind: ::std::string::String,
}
mod budget_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#budget")
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
    #[serde(rename = "adTechnologyProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ad_technology_providers: ::std::option::Option<CreativeAdTechnologyProviders>,
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
    #[serde(rename = "nativeAd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If nativeAd is set, HTMLSnippet and videoURL should not be set."]
    pub native_ad: ::std::option::Option<CreativeNativeAd>,
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
    #[doc = "The URL to fetch a video ad. If set, HTMLSnippet and the nativeAd should not be set."]
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
pub struct CreativeAdTechnologyProviders {
    #[serde(rename = "detectedProviderIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If this creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the  Authorized Buyers Real-Time Bidding protocol or the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the OpenRTB protocol, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the \"Third-party Ad Technology Vendors\" section of Authorized Buyers Program Guidelines."]
    pub detected_provider_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "hasUnidentifiedProvider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the creative contains an unidentified ad technology provider. If true, a bid submitted for a European Economic Area (EEA) user with this creative is not compliant with the GDPR policies as mentioned in the \"Third-party Ad Technology Vendors\" section of Authorized Buyers Program Guidelines."]
    pub has_unidentified_provider: ::std::option::Option<::std::primitive::bool>,
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
#[doc = "If nativeAd is set, HTMLSnippet and videoURL should not be set."]
pub struct CreativeNativeAd {
    #[serde(rename = "advertiser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub advertiser: ::std::option::Option<::std::string::String>,
    #[serde(rename = "appIcon")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The app icon, for app download ads."]
    pub app_icon: ::std::option::Option<CreativeNativeAdAppIcon>,
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A long description of the ad."]
    pub body: ::std::option::Option<::std::string::String>,
    #[serde(rename = "callToAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A label for the button that the user is supposed to click."]
    pub call_to_action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clickTrackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to use for click tracking."]
    pub click_tracking_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "headline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short title for the ad."]
    pub headline: ::std::option::Option<::std::string::String>,
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A large image."]
    pub image: ::std::option::Option<CreativeNativeAdImage>,
    #[serde(rename = "impressionTrackingUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URLs are called when the impression is rendered."]
    pub impression_tracking_url: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "logo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A smaller image, for the advertiser logo."]
    pub logo: ::std::option::Option<CreativeNativeAdLogo>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price of the promoted app including the currency info."]
    pub price: ::std::option::Option<::std::string::String>,
    #[serde(rename = "starRating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The app rating in the app store. Must be in the range [0-5]."]
    pub star_rating: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The app icon, for app download ads."]
pub struct CreativeNativeAdAppIcon {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A large image."]
pub struct CreativeNativeAdImage {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A smaller image, for the advertiser logo."]
pub struct CreativeNativeAdLogo {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub width: ::std::option::Option<::std::primitive::i64>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration data for an Ad Exchange direct deal."]
pub struct DirectDeal {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account id of the buyer this deal is for."]
    pub account_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "advertiser")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the advertiser this deal is for."]
    pub advertiser: ::std::option::Option<::std::string::String>,
    #[serde(rename = "allowsAlcohol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the publisher for this deal is eligible for alcohol ads."]
    pub allows_alcohol: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "buyerAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account id that this deal was negotiated for. It is either the buyer or the client that this deal was negotiated on behalf of."]
    pub buyer_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code that applies to the fixed_cpm value. If not set then assumed to be USD."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The deal type such as programmatic reservation or fixed price and so on."]
    pub deal_tier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "End time for when this deal stops being active. If not set then this deal is valid until manually disabled by the publisher. In seconds since the epoch."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fixedCpm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The fixed price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the fixed price tier of buying (highest priority, pay exactly the configured fixed price)."]
    pub fixed_cpm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deal id."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "direct_deal_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deal name."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateExchangeMinCpm")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum price for this direct deal. In cpm micros of currency according to currency_code. If set, then this deal is eligible for the private exchange tier of buying (below fixed price priority, run as a second price auction)."]
    pub private_exchange_min_cpm: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherBlocksOverriden")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the publisher has opted to have their blocks ignored when a creative is bid with for this deal."]
    pub publisher_blocks_overriden: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sellerNetwork")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the publisher offering this direct deal."]
    pub seller_network: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Start time for when this deal becomes active. If not set then this deal is active immediately upon creation. In seconds since the epoch."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
mod direct_deal_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#directDeal")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A direct deals feed lists Direct Deals the Ad Exchange buyer account has access to. This includes direct deals set up for the buyer account as well as its merged stream seats."]
pub struct DirectDealsList {
    #[serde(rename = "directDeals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of direct deals relevant for your account."]
    pub direct_deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DirectDeal>>>,
    #[serde(rename = "kind")]
    #[serde(default = "direct_deals_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod direct_deals_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#directDealsList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration data for an Ad Exchange performance report list."]
pub struct PerformanceReport {
    #[serde(rename = "bidRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bid responses with an ad."]
    pub bid_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "bidRequestRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bid requests sent to your bidder."]
    pub bid_request_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "calloutStatusRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate of various prefiltering statuses per match. Please refer to the callout-status-codes.txt file for different statuses."]
    pub callout_status_rate: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "cookieMatcherStatusRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average QPS for cookie matcher operations."]
    pub cookie_matcher_status_rate: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "creativeStatusRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate of ads with a given status. Please refer to the creative-status-codes.txt file for different statuses."]
    pub creative_status_rate: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "filteredBidRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bid responses that were filtered due to a policy violation or other errors."]
    pub filtered_bid_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "hostedMatchStatusRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average QPS for hosted match operations."]
    pub hosted_match_status_rate: ::std::option::Option<::std::vec::Vec<::serde_json::Value>>,
    #[serde(rename = "inventoryMatchRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of potential queries based on your pretargeting settings."]
    pub inventory_match_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "kind")]
    #[serde(default = "performance_report_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "latency50thPercentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 50th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
    pub latency50th_percentile: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "latency85thPercentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 85th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
    pub latency85th_percentile: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "latency95thPercentile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The 95th percentile round trip latency(ms) as perceived from Google servers for the duration period covered by the report."]
    pub latency95th_percentile: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "noQuotaInRegion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate of various quota account statuses per quota check."]
    pub no_quota_in_region: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "outOfQuota")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate of various quota account statuses per quota check."]
    pub out_of_quota: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pixelMatchRequests")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average QPS for pixel match requests from clients."]
    pub pixel_match_requests: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pixelMatchResponses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Average QPS for pixel match responses from clients."]
    pub pixel_match_responses: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "quotaConfiguredLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The configured quota limits for this account."]
    pub quota_configured_limit: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "quotaThrottledLimit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The throttled quota limits for this account."]
    pub quota_throttled_limit: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The trading location of this data."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "successfulRequestRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of properly formed bid responses received by our servers within the deadline."]
    pub successful_request_rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unix timestamp of the starting time of this performance data."]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unsuccessfulRequestRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of bid responses that were unsuccessful due to timeouts, incorrect formatting, etc."]
    pub unsuccessful_request_rate: ::std::option::Option<::std::primitive::f64>,
}
mod performance_report_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#performanceReport")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The configuration data for an Ad Exchange performance report list."]
pub struct PerformanceReportList {
    #[serde(rename = "kind")]
    #[serde(default = "performance_report_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
    #[serde(rename = "performanceReport")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of performance reports relevant for the account."]
    pub performance_report:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PerformanceReport>>>,
}
mod performance_report_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#performanceReportList")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PretargetingConfig {
    #[serde(rename = "billingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The id for billing purposes, provided for reference. Leave this field blank for insert requests; the id will be generated automatically."]
    pub billing_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "configId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The config id; generated automatically. Leave this field blank for insert requests."]
    pub config_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "configName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the config. Must be unique. Required for all requests."]
    pub config_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List must contain exactly one of PRETARGETING_CREATIVE_TYPE_HTML or PRETARGETING_CREATIVE_TYPE_VIDEO."]
    pub creative_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests which allow one of these (width, height) pairs will match. All pairs must be supported ad dimensions."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<PretargetingConfigDimensions>>,
    #[serde(rename = "excludedContentLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests with any of these content labels will not match. Values are from content-labels.txt in the downloadable files section."]
    pub excluded_content_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "excludedGeoCriteriaIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these geo criteria ids will not match."]
    pub excluded_geo_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "excludedPlacements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these placements will not match."]
    pub excluded_placements:
        ::std::option::Option<::std::vec::Vec<PretargetingConfigExcludedPlacements>>,
    #[serde(rename = "excludedUserLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these users list ids will not match."]
    pub excluded_user_lists: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "excludedVerticals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these vertical ids will not match. Values are from the publisher-verticals.txt file in the downloadable files section."]
    pub excluded_verticals: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "geoCriteriaIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these geo criteria ids will match."]
    pub geo_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether this config is active. Required for all requests."]
    pub is_active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "pretargeting_config_defaults :: kind")]
    #[doc = "The kind of the resource, i.e. \"adexchangebuyer#pretargetingConfig\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Request containing any of these language codes will match."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "maximumQps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The maximum QPS allocated to this pretargeting configuration, used for pretargeting-level QPS limits. By default, this is not set, which indicates that there is no QPS limit at the configuration level (a global or account-level limit may still be imposed)."]
    pub maximum_qps: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileCarriers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these mobile carrier ids will match. Values are from mobile-carriers.csv in the downloadable files section."]
    pub mobile_carriers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mobileDevices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these mobile device ids will match. Values are from mobile-devices.csv in the downloadable files section."]
    pub mobile_devices: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mobileOperatingSystemVersions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these mobile operating system version ids will match. Values are from mobile-os.csv in the downloadable files section."]
    pub mobile_operating_system_versions:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "placements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these placements will match."]
    pub placements: ::std::option::Option<::std::vec::Vec<PretargetingConfigPlacements>>,
    #[serde(rename = "platforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests matching any of these platforms will match. Possible values are PRETARGETING_PLATFORM_MOBILE, PRETARGETING_PLATFORM_DESKTOP, and PRETARGETING_PLATFORM_TABLET."]
    pub platforms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "supportedCreativeAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative attributes should be declared here if all creatives corresponding to this pretargeting configuration have that creative attribute. Values are from pretargetable-creative-attributes.txt in the downloadable files section."]
    pub supported_creative_attributes:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "userLists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these user list ids will match."]
    pub user_lists: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "vendorTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests that allow any of these vendor ids will match. Values are from vendors.txt in the downloadable files section."]
    pub vendor_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "verticals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing any of these vertical ids will match."]
    pub verticals: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod pretargeting_config_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#pretargetingConfig")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PretargetingConfigDimensions {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height in pixels."]
    pub height: ::std::option::Option<::std::string::String>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width in pixels."]
    pub width: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PretargetingConfigExcludedPlacements {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the placement."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PretargetingConfigPlacements {
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the placement. Interpretation depends on the placement type, e.g. URL for a site placement, channel name for a channel placement, app id for a mobile app placement."]
    pub token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the placement."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PretargetingConfigList {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of pretargeting configs"]
    pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PretargetingConfig>>>,
    #[serde(rename = "kind")]
    #[serde(default = "pretargeting_config_list_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod pretargeting_config_list_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#pretargetingConfigList")
    }
}
