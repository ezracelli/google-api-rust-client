#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Configuration data for an Ad Exchange buyer account."]
pub struct Account {
    #[serde(rename = "applyPretargetingToNonGuaranteedDeals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When this is false, bid requests that include a deal ID for a private auction or preferred deal are always sent to your bidder. When true, all active pretargeting configs will be applied to private auctions and preferred deals. Programmatic Guaranteed deals (when enabled) are always sent to your bidder."]
    pub apply_pretargeting_to_non_guaranteed_deals: ::std::option::Option<::std::primitive::bool>,
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
    #[serde(rename = "bidProtocol")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The protocol that the bidder endpoint is using. OpenRTB protocols with prefix PROTOCOL_OPENRTB_PROTOBUF use proto buffer, otherwise use JSON.  Allowed values:  \n- PROTOCOL_ADX \n- PROTOCOL_OPENRTB_2_2 \n- PROTOCOL_OPENRTB_2_3 \n- PROTOCOL_OPENRTB_2_4 \n- PROTOCOL_OPENRTB_2_5 \n- PROTOCOL_OPENRTB_PROTOBUF_2_3 \n- PROTOCOL_OPENRTB_PROTOBUF_2_4 \n- PROTOCOL_OPENRTB_PROTOBUF_2_5"]
    pub bid_protocol: ::std::option::Option<::std::string::String>,
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
pub struct AddOrderDealsRequest {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of deals to add"]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last known proposal revision number."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates an optional action to take on the proposal"]
    pub update_action: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddOrderDealsResponse {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deals added (in the same proposal as passed in the request)"]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated revision number for the proposal."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddOrderNotesRequest {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of notes to add."]
    pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceNote>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddOrderNotesResponse {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceNote>>>,
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
pub struct Buyer {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adx account id of the buyer."]
    pub account_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContactInformation {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address of the contact."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the contact."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOrdersRequest {
    #[serde(rename = "proposals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of proposals to create."]
    pub proposals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Proposal>>>,
    #[serde(rename = "webPropertyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Web property id of the seller creating these orders"]
    pub web_property_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateOrdersResponse {
    #[serde(rename = "proposals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of proposals successfully created."]
    pub proposals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Proposal>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A creative and its classification data."]
pub struct Creative {
    #[serde(rename = "HTMLSnippet")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTML snippet that displays the ad when inserted in the web page. If set, videoURL, videoVastXML, and nativeAd should not be set."]
    pub html_snippet: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Account id."]
    pub account_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "adChoicesDestinationUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to the Ad Preferences page. This is only supported for native ads."]
    pub ad_choices_destination_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adTechnologyProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub ad_technology_providers: ::std::option::Option<CreativeAdTechnologyProviders>,
    #[serde(rename = "advertiserId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected advertiser id, if any. Read-only. This field should not be set in requests."]
    pub advertiser_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "advertiserName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the company being advertised in the creative. A list of advertisers is provided in the advertisers.txt file."]
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
    #[doc = "List of buyer selectable attributes for the ads that may be shown from this snippet. Each attribute is represented by an integer as defined in  buyer-declarable-creative-attributes.txt."]
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
    #[serde(rename = "creativeStatusIdentityType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creative status identity type that the creative item applies to. Ad Exchange real-time bidding is migrating to the sizeless creative verification. Originally, Ad Exchange assigned creative verification status to a unique combination of a buyer creative ID and creative dimensions. Post-migration, a single verification status will be assigned at the buyer creative ID level. This field allows to distinguish whether a given creative status applies to a unique combination of a buyer creative ID and creative dimensions, or to a buyer creative ID as a whole."]
    pub creative_status_identity_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealsStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level deals status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=DIRECT_DEALS (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from servingRestrictions directly."]
    pub deals_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detectedDomains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected domains for this creative. Read-only. This field should not be set in requests."]
    pub detected_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
    #[serde(rename = "languages")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected languages for this creative. Read-only. This field should not be set in requests."]
    pub languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "nativeAd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)"]
    pub native_ad: ::std::option::Option<CreativeNativeAd>,
    #[serde(rename = "openAuctionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level open auction status. Read-only. This field should not be set in requests. If disapproved, an entry for auctionType=OPEN_AUCTION (or ALL) in servingRestrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case it may be preferable to read from ServingRestrictions directly."]
    pub open_auction_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected product categories, if any. Each category is represented by an integer as defined in  ad-product-categories.txt. Read-only. This field should not be set in requests."]
    pub product_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "restrictedCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All restricted categories for the ads that may be shown from this snippet. Each category is represented by an integer as defined in the  ad-restricted-categories.txt."]
    pub restricted_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "sensitiveCategories")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Detected sensitive categories, if any. Each category is represented by an integer as defined in  ad-sensitive-categories.txt. Read-only. This field should not be set in requests."]
    pub sensitive_categories: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "servingRestrictions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction). Read-only. This field should not be set in requests. See the examples in the Creatives guide for more details."]
    pub serving_restrictions: ::std::option::Option<::std::vec::Vec<CreativeServingRestrictions>>,
    #[serde(rename = "vendorType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of vendor types for the ads that may be shown from this snippet. Each vendor type is represented by an integer as defined in vendors.txt."]
    pub vendor_type: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The version for this creative. Read-only. This field should not be set in requests."]
    pub version: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "videoURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL to fetch a video ad. If set, HTMLSnippet, videoVastXML, and nativeAd should not be set. Note, this is different from resource.native_ad.video_url above."]
    pub video_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "videoVastXML")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard. If set, HTMLSnippet, videoURL, and nativeAd and should not be set."]
    pub video_vast_xml: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All known serving contexts containing serving status information."]
    pub contexts: ::std::option::Option<::std::vec::Vec<CreativeCorrectionsContexts>>,
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
pub struct CreativeCorrectionsContexts {
    #[serde(rename = "auctionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this correction applies to."]
    pub auction_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "contextType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
    pub context_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoCriteriaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this correction applies to."]
    pub geo_criteria_id: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=PLATFORM. Represents the platforms this correction applies to."]
    pub platform: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
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
    #[doc = "The filtering status code as defined in  creative-status-codes.txt."]
    pub filtering_status: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "If nativeAd is set, HTMLSnippet, videoVastXML, and the videoURL outside of nativeAd should not be set. (The videoURL inside nativeAd can be set.)"]
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
    #[serde(rename = "clickLinkUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
    pub click_link_url: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "videoURL")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of the XML VAST for a native ad. Note this is a separate field from resource.video_url."]
    pub video_url: ::std::option::Option<::std::string::String>,
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
pub struct CreativeServingRestrictions {
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "All known contexts/restrictions."]
    pub contexts: ::std::option::Option<::std::vec::Vec<CreativeServingRestrictionsContexts>>,
    #[serde(rename = "disapprovalReasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reasons for disapproval within this restriction, if any. Note that not all disapproval reasons may be categorized, so it is possible for the creative to have a status of DISAPPROVED or CONDITIONALLY_APPROVED with an empty list for disapproval_reasons. In this case, please reach out to your TAM to help debug the issue."]
    pub disapproval_reasons:
        ::std::option::Option<::std::vec::Vec<CreativeServingRestrictionsDisapprovalReasons>>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Why the creative is ineligible to serve in this context (e.g., it has been explicitly disapproved or is pending review)."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeServingRestrictionsContexts {
    #[serde(rename = "auctionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=AUCTION_TYPE. Represents the auction types this restriction applies to."]
    pub auction_type: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "contextType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of context (e.g., location, platform, auction type, SSL-ness)."]
    pub context_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geoCriteriaId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=LOCATION. Represents the geo criterias this restriction applies to. Impressions are considered to match a context if either the user location or publisher location matches a given geoCriteriaId."]
    pub geo_criteria_id: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only set when contextType=PLATFORM. Represents the platforms this restriction applies to."]
    pub platform: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeServingRestrictionsDisapprovalReasons {
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
#[doc = "The external deal ids associated with a creative."]
pub struct CreativeDealIds {
    #[serde(rename = "dealStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of external deal ids and ARC approval status."]
    pub deal_statuses: ::std::option::Option<::std::vec::Vec<CreativeDealIdsDealStatuses>>,
    #[serde(rename = "kind")]
    #[serde(default = "creative_deal_ids_defaults :: kind")]
    #[doc = "Resource type."]
    pub kind: ::std::string::String,
}
mod creative_deal_ids_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#creativeDealIds")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreativeDealIdsDealStatuses {
    #[serde(rename = "arcStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ARC approval status."]
    pub arc_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External deal ID."]
    pub deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "webPropertyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher ID."]
    pub web_property_id: ::std::option::Option<::std::primitive::i64>,
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
pub struct DealServingMetadata {
    #[serde(rename = "alcoholAdsAllowed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if alcohol ads are allowed for this deal (read-only). This field is only populated when querying for finalized orders using the method GetFinalizedOrderDeals"]
    pub alcohol_ads_allowed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "dealPauseStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tracks which parties (if any) have paused a deal. (readonly, except via PauseResumeOrderDeals action)"]
    pub deal_pause_status:
        ::std::option::Option<::std::boxed::Box<DealServingMetadataDealPauseStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tracks which parties (if any) have paused a deal. The deal is considered paused if has_buyer_paused || has_seller_paused. Each of the has_buyer_paused or the has_seller_paused bits can be set independently."]
pub struct DealServingMetadataDealPauseStatus {
    #[serde(rename = "buyerPauseReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub buyer_pause_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "firstPausedBy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the deal is paused, records which party paused the deal first."]
    pub first_paused_by: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasBuyerPaused")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub has_buyer_paused: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "hasSellerPaused")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub has_seller_paused: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "sellerPauseReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub seller_pause_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTerms {
    #[serde(rename = "brandingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Visibility of the URL in bid requests."]
    pub branding_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "crossListedExternalDealIdType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that this ExternalDealId exists under at least two different AdxInventoryDeals. Currently, the only case that the same ExternalDealId will exist is programmatic cross sell case."]
    pub cross_listed_external_deal_id_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description for the proposed terms of the deal."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "estimatedGrossSpend")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-binding estimate of the estimated gross spend for this deal Can be set by buyer or seller."]
    pub estimated_gross_spend: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "estimatedImpressionsPerDay")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Non-binding estimate of the impressions served per day Can be set by buyer or seller."]
    pub estimated_impressions_per_day: ::std::option::Option<::std::string::String>,
    #[serde(rename = "guaranteedFixedPriceTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The terms for guaranteed fixed price deals."]
    pub guaranteed_fixed_price_terms:
        ::std::option::Option<::std::boxed::Box<DealTermsGuaranteedFixedPriceTerms>>,
    #[serde(rename = "nonGuaranteedAuctionTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The terms for non-guaranteed auction deals."]
    pub non_guaranteed_auction_terms:
        ::std::option::Option<::std::boxed::Box<DealTermsNonGuaranteedAuctionTerms>>,
    #[serde(rename = "nonGuaranteedFixedPriceTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The terms for non-guaranteed fixed price deals."]
    pub non_guaranteed_fixed_price_terms:
        ::std::option::Option<::std::boxed::Box<DealTermsNonGuaranteedFixedPriceTerms>>,
    #[serde(rename = "rubiconNonGuaranteedTerms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The terms for rubicon non-guaranteed deals."]
    pub rubicon_non_guaranteed_terms:
        ::std::option::Option<::std::boxed::Box<DealTermsRubiconNonGuaranteedTerms>>,
    #[serde(rename = "sellerTimeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For deals with Cost Per Day billing, defines the timezone used to mark the boundaries of a day (buyer-readonly)"]
    pub seller_time_zone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTermsGuaranteedFixedPriceTerms {
    #[serde(rename = "billingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "External billing info for this Deal. This field is relevant when external billing info such as price has a different currency code than DFP/AdX."]
    pub billing_info:
        ::std::option::Option<::std::boxed::Box<DealTermsGuaranteedFixedPriceTermsBillingInfo>>,
    #[serde(rename = "fixedPrices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed price for the specified buyer."]
    pub fixed_prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
    #[serde(rename = "guaranteedImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy."]
    pub guaranteed_impressions: ::std::option::Option<::std::string::String>,
    #[serde(rename = "guaranteedLooks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of guaranteed looks. Required for deal, optional for product. For CPD deals, buyer changes to guaranteed_looks will be ignored."]
    pub guaranteed_looks: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumDailyLooks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of minimum daily looks for a CPD deal. For CPD deals, buyer should negotiate on this field instead of guaranteed_looks."]
    pub minimum_daily_looks: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTermsGuaranteedFixedPriceTermsBillingInfo {
    #[serde(rename = "currencyConversionTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp (in ms since epoch) when the original reservation price for the deal was first converted to DFP currency. This is used to convert the contracted price into buyer's currency without discrepancy."]
    pub currency_conversion_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dfpLineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The DFP line item id associated with this deal. For features like CPD, buyers can retrieve the DFP line item for billing reconciliation."]
    pub dfp_line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originalContractedQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original contracted quantity (# impressions) for this deal. To ensure delivery, sometimes the publisher will book the deal with a impression buffer, such that guaranteed_looks is greater than the contracted quantity. However clients are billed using the original contracted quantity."]
    pub original_contracted_quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The original reservation price for the deal, if the currency code is different from the one used in negotiation."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTermsNonGuaranteedAuctionTerms {
    #[serde(rename = "autoOptimizePrivateAuction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if open auction buyers are allowed to compete with invited buyers in this private auction (buyer-readonly)."]
    pub auto_optimize_private_auction: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "reservePricePerBuyers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reserve price for the specified buyer."]
    pub reserve_price_per_buyers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTermsNonGuaranteedFixedPriceTerms {
    #[serde(rename = "fixedPrices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed price for the specified buyer."]
    pub fixed_prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DealTermsRubiconNonGuaranteedTerms {
    #[serde(rename = "priorityPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional price for Rubicon priority access in the auction."]
    pub priority_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "standardPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional price for Rubicon standard access in the auction."]
    pub standard_price: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteOrderDealsRequest {
    #[serde(rename = "dealIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deals to delete for a given proposal"]
    pub deal_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last known proposal revision number."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates an optional action to take on the proposal"]
    pub update_action: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteOrderDealsResponse {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deals deleted (in the same proposal as passed in the request)"]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated revision number for the proposal."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryControl {
    #[serde(rename = "creativeBlockingLevel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub creative_blocking_level: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryRateType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub delivery_rate_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "frequencyCaps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub frequency_caps:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DeliveryControlFrequencyCap>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryControlFrequencyCap {
    #[serde(rename = "maxImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub max_impressions: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "numTimeUnits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub num_time_units: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timeUnitType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_unit_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message carries publisher provided breakdown. E.g. {dimension_type: 'COUNTRY', [{dimension_value: {id: 1, name: 'US'}}, {dimension_value: {id: 2, name: 'UK'}}]}"]
pub struct Dimension {
    #[serde(rename = "dimensionType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dimension_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dimensionValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub dimension_values:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DimensionDimensionValue>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Value of the dimension."]
pub struct DimensionDimensionValue {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of the dimension."]
    pub id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the dimension mainly for debugging purposes, except for the case of CREATIVE_SIZE. For CREATIVE_SIZE, strings are used instead of ids."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "percentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percent of total impressions for a dimension type. e.g. {dimension_type: 'GENDER', [{dimension_value: {id: 1, name: 'MALE', percentage: 60}}]} Gender MALE is 60% of all impressions which have gender."]
    pub percentage: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditAllOrderDealsRequest {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deals to edit. Service may perform 3 different operations based on comparison of deals in this list vs deals already persisted in database: 1. Add new deal to proposal If a deal in this list does not exist in the proposal, the service will create a new deal and add it to the proposal. Validation will follow AddOrderDealsRequest. 2. Update existing deal in the proposal If a deal in this list already exist in the proposal, the service will update that existing deal to this new deal in the request. Validation will follow UpdateOrderDealsRequest. 3. Delete deals from the proposal (just need the id) If a existing deal in the proposal is not present in this list, the service will delete that deal from the proposal. Validation will follow DeleteOrderDealsRequest."]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
    #[serde(rename = "proposal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If specified, also updates the proposal in the batch transaction. This is useful when the proposal and the deals need to be updated in one transaction."]
    pub proposal: ::std::option::Option<::std::boxed::Box<Proposal>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last known revision number for the proposal."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates an optional action to take on the proposal"]
    pub update_action: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditAllOrderDealsResponse {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all deals in the proposal after edit."]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
    #[serde(rename = "orderRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest revision number after the update has been applied."]
    pub order_revision_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetOffersResponse {
    #[serde(rename = "products")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The returned list of products."]
    pub products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetOrderDealsResponse {
    #[serde(rename = "deals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of deals for the proposal"]
    pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceDeal>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetOrderNotesResponse {
    #[serde(rename = "notes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching notes. The notes for a proposal are ordered from oldest to newest. If the notes span multiple proposals, they will be grouped by proposal, with the notes for the most recently modified proposal appearing first."]
    pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceNote>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetOrdersResponse {
    #[serde(rename = "proposals")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of matching proposals."]
    pub proposals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Proposal>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetPublisherProfilesByAccountIdResponse {
    #[serde(rename = "profiles")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Profiles for the requested publisher"]
    pub profiles:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublisherProfileApiProto>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A proposal can contain multiple deals. A deal contains the terms and targeting information that is used for serving."]
pub struct MarketplaceDeal {
    #[serde(rename = "buyerPrivateData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Buyer private data (hidden from seller)."]
    pub buyer_private_data: ::std::option::Option<::std::boxed::Box<PrivateData>>,
    #[serde(rename = "creationTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time (ms since epoch) of the deal creation. (readonly)"]
    pub creation_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativePreApprovalPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the creative pre-approval policy (buyer-readonly)"]
    pub creative_pre_approval_policy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creativeSafeFrameCompatibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies whether the creative is safeFrame compatible (buyer-readonly)"]
    pub creative_safe_frame_compatibility: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique deal-id for the deal (readonly)."]
    pub deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealServingMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metadata about the serving status of this deal (readonly, writes via custom actions)"]
    pub deal_serving_metadata: ::std::option::Option<::std::boxed::Box<DealServingMetadata>>,
    #[serde(rename = "deliveryControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
    pub delivery_control: ::std::option::Option<::std::boxed::Box<DeliveryControl>>,
    #[serde(rename = "externalDealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The external deal id assigned to this deal once the deal is finalized. This is the deal-id that shows up in serving/reporting etc. (readonly)"]
    pub external_deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flightEndTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proposed flight end time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
    pub flight_end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flightStartTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Proposed flight start time of the deal (ms since epoch) This will generally be stored in a granularity of a second. (updatable)"]
    pub flight_start_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventoryDescription")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description for the deal terms. (buyer-readonly)"]
    pub inventory_description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRfpTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the current deal is a RFP template. RFP template is created by buyer and not based on seller created products."]
    pub is_rfp_template: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSetupComplete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True, if the buyside inventory setup is complete for this deal. (readonly, except via OrderSetupCompleted action)"]
    pub is_setup_complete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "marketplace_deal_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceDeal\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "lastUpdateTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time (ms since epoch) when the deal was last updated. (readonly)"]
    pub last_update_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "makegoodRequestedReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub makegood_requested_reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the deal. (updatable)"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product-id from which this deal was created. (readonly, except on create)"]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision number of the product that the deal was created from (readonly, except on create)"]
    pub product_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "programmaticCreativeSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the creative source for programmatic deals, PUBLISHER means creative is provided by seller and ADVERTISR means creative is provided by buyer. (buyer-readonly)"]
    pub programmatic_creative_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proposalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub proposal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellerContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional Seller contact information for the deal (buyer-readonly)"]
    pub seller_contacts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
    #[serde(rename = "sharedTargetings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together. (updatable)"]
    pub shared_targetings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SharedTargeting>>>,
    #[serde(rename = "syndicationProduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
    pub syndication_product: ::std::option::Option<::std::string::String>,
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The negotiable terms of the deal. (updatable)"]
    pub terms: ::std::option::Option<::std::boxed::Box<DealTerms>>,
    #[serde(rename = "webPropertyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub web_property_code: ::std::option::Option<::std::string::String>,
}
mod marketplace_deal_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#marketplaceDeal")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceDealParty {
    #[serde(rename = "buyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal-party."]
    pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "seller")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The buyer/seller associated with the deal. One of buyer/seller is specified for a deal party."]
    pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarketplaceLabel {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The accountId of the party that created the label."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "createTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creation time (in ms since epoch) for the label."]
    pub create_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deprecatedMarketplaceDealParty")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the party that created the label."]
    pub deprecated_marketplace_deal_party:
        ::std::option::Option<::std::boxed::Box<MarketplaceDealParty>>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The label to use."]
    pub label: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A proposal is associated with a bunch of notes which may optionally be associated with a deal and/or revision number."]
pub struct MarketplaceNote {
    #[serde(rename = "creatorRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role of the person (buyer/seller) creating the note. (readonly)"]
    pub creator_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "dealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Notes can optionally be associated with a deal. (readonly, except on create)"]
    pub deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "marketplace_note_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#marketplaceNote\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actual note to attach. (readonly, except on create)"]
    pub note: ::std::option::Option<::std::string::String>,
    #[serde(rename = "noteId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique id for the note. (readonly)"]
    pub note_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proposalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The proposalId that a note is attached to. (readonly)"]
    pub proposal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the note is associated with a proposal revision number, then store that here. (readonly, except on create)"]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestampMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The timestamp (ms since epoch) that this note was created. (readonly)"]
    pub timestamp_ms: ::std::option::Option<::std::string::String>,
}
mod marketplace_note_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#marketplaceNote")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MobileApplication {
    #[serde(rename = "appStore")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub app_store: ::std::option::Option<::std::string::String>,
    #[serde(rename = "externalAppId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub external_app_id: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "minimumViewabilityDecile")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests where the predicted viewability is below the specified decile will not match. E.g. if the buyer sets this value to 5, requests from slots where the predicted viewability is below 50% will not match. If the predicted viewability is unknown this field will be ignored."]
    pub minimum_viewability_decile: ::std::option::Option<::std::primitive::i64>,
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
    #[serde(rename = "userIdentifierDataRequired")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Requests containing the specified type of user data will match. Possible values are HOSTED_MATCH_DATA, which means the request is cookie-targetable and has a match in the buyer's hosted match table, and COOKIE_OR_IDFA, which means the request has either a targetable cookie or an iOS IDFA."]
    pub user_identifier_data_required:
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
    #[serde(rename = "videoPlayerSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Video requests satisfying any of these player size constraints will match."]
    pub video_player_sizes:
        ::std::option::Option<::std::vec::Vec<PretargetingConfigVideoPlayerSizes>>,
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
pub struct PretargetingConfigVideoPlayerSizes {
    #[serde(rename = "aspectRatio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of aspect ratio. Leave this field blank to match all aspect ratios."]
    pub aspect_ratio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minHeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum player height in pixels. Leave this field blank to match any player height."]
    pub min_height: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minWidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum player width in pixels. Leave this field blank to match any player width."]
    pub min_width: ::std::option::Option<::std::string::String>,
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
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Price {
    #[serde(rename = "amountMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price value in micros."]
    pub amount_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency code for the price."]
    pub currency_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expectedCpmMicros")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In case of CPD deals, the expected CPM in micros."]
    pub expected_cpm_micros: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "pricingType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pricing type for the deal/product."]
    pub pricing_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Used to specify pricing rules for buyers. Each PricePerBuyer in a product can become [0,1] deals. To check if there is a PricePerBuyer for a particular buyer we look for the most specific matching rule - we first look for a rule matching the buyer and otherwise look for a matching rule where no buyer is set."]
pub struct PricePerBuyer {
    #[serde(rename = "auctionTier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional access type for this buyer."]
    pub auction_tier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "billedBuyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the buyer that will get billed."]
    pub billed_buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "buyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer)."]
    pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The specified price"]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateData {
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reference_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "referencePayload")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub reference_payload: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A product is segment of inventory that a seller wishes to sell. It is associated with certain terms and targeting information which helps buyer know more about the inventory. Each field in a product can have one of the following setting:\n\n(readonly) - It is an error to try and set this field. (buyer-readonly) - Only the seller can set this field. (seller-readonly) - Only the buyer can set this field. (updatable) - The field is updatable at all times by either buyer or the seller."]
pub struct Product {
    #[serde(rename = "billedBuyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billed buyer corresponding to the buyer that created the offer. (readonly, except on create)"]
    pub billed_buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "buyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The buyer that created the offer if this is a buyer initiated offer (readonly, except on create)"]
    pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "creationTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Creation time in ms. since epoch (readonly)"]
    pub creation_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creatorContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional contact information for the creator of this product. (buyer-readonly)"]
    pub creator_contacts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
    #[serde(rename = "creatorRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role that created the offer. Set to BUYER for buyer initiated offers."]
    pub creator_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryControl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher. This message is assigned an id of 100 since some day we would want to model this as a protobuf extension."]
    pub delivery_control: ::std::option::Option<::std::boxed::Box<DeliveryControl>>,
    #[serde(rename = "flightEndTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The proposed end time for the deal (ms since epoch) (buyer-readonly)"]
    pub flight_end_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flightStartTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory availability dates. (times are in ms since epoch) The granularity is generally in the order of seconds. (buyer-readonly)"]
    pub flight_start_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hasCreatorSignedOff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false."]
    pub has_creator_signed_off: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inventorySource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "What exchange will provide this inventory (readonly, except on create)."]
    pub inventory_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(default = "product_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#product\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional List of labels for the product (optional, buyer-readonly)."]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceLabel>>>,
    #[serde(rename = "lastUpdateTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time of last update in ms. since epoch (readonly)"]
    pub last_update_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "legacyOfferId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional legacy offer id if this offer is a preferred deal offer."]
    pub legacy_offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketplacePublisherProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Marketplace publisher profile Id. This Id differs from the regular publisher_profile_id in that 1. This is a new id, the old Id will be deprecated in 2017. 2. This id uniquely identifies a publisher profile by itself."]
    pub marketplace_publisher_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name for this product as set by the seller. (buyer-readonly)"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateAuctionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional private auction id if this offer is a private auction offer."]
    pub private_auction_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique id for the product (readonly)"]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of the publisher profile for a given seller. A (seller.account_id, publisher_profile_id) pair uniquely identifies a publisher profile. Buyers can call the PublisherProfiles::List endpoint to get a list of publisher profiles for a given seller."]
    pub publisher_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherProvidedForecast")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher self-provided forecast information."]
    pub publisher_provided_forecast:
        ::std::option::Option<::std::boxed::Box<PublisherProvidedForecast>>,
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision number of the product. (readonly)"]
    pub revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seller")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about the seller that created this product (readonly, except on create)"]
    pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
    #[serde(rename = "sharedTargetings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Targeting that is shared between the buyer and the seller. Each targeting criteria has a specified key and for each key there is a list of inclusion value or exclusion values. (buyer-readonly)"]
    pub shared_targetings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SharedTargeting>>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The state of the product. (buyer-readonly)"]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "syndicationProduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The syndication product associated with the deal. (readonly, except on create)"]
    pub syndication_product: ::std::option::Option<::std::string::String>,
    #[serde(rename = "terms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The negotiable terms of the deal (buyer-readonly)"]
    pub terms: ::std::option::Option<::std::boxed::Box<DealTerms>>,
    #[serde(rename = "webPropertyCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The web property code for the seller. This field is meant to be copied over as is when creating deals."]
    pub web_property_code: ::std::option::Option<::std::string::String>,
}
mod product_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#product")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a proposal in the marketplace. A proposal is the unit of negotiation between a seller and a buyer and contains deals which are served. Each field in a proposal can have one of the following setting:\n\n(readonly) - It is an error to try and set this field. (buyer-readonly) - Only the seller can set this field. (seller-readonly) - Only the buyer can set this field. (updatable) - The field is updatable at all times by either buyer or the seller."]
pub struct Proposal {
    #[serde(rename = "billedBuyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the buyer that will get billed for this proposal. (readonly)"]
    pub billed_buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "buyer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the buyer on the proposal. (readonly, except on create)"]
    pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
    #[serde(rename = "buyerContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional contact information of the buyer. (seller-readonly)"]
    pub buyer_contacts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
    #[serde(rename = "buyerPrivateData")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Private data for buyer. (hidden from seller)."]
    pub buyer_private_data: ::std::option::Option<::std::boxed::Box<PrivateData>>,
    #[serde(rename = "dbmAdvertiserIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of DBM advertisers permission to this proposal."]
    pub dbm_advertiser_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "hasBuyerSignedOff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off. Once both sides have signed off on a deal, the proposal can be finalized by the seller. (seller-readonly)"]
    pub has_buyer_signed_off: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "hasSellerSignedOff")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When an proposal is in an accepted state, indicates whether the buyer has signed off Once both sides have signed off on a deal, the proposal can be finalized by the seller. (buyer-readonly)"]
    pub has_seller_signed_off: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inventorySource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "What exchange will provide this inventory (readonly, except on create)."]
    pub inventory_source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isRenegotiating")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True if the proposal is being renegotiated (readonly)."]
    pub is_renegotiating: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isSetupComplete")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True, if the buyside inventory setup is complete for this proposal. (readonly, except via OrderSetupCompleted action) Deprecated in favor of deal level setup complete flag."]
    pub is_setup_complete: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "proposal_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#proposal\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of labels associated with the proposal. (readonly)"]
    pub labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MarketplaceLabel>>>,
    #[serde(rename = "lastUpdaterOrCommentorRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The role of the last user that either updated the proposal or left a comment. (readonly)"]
    pub last_updater_or_commentor_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name for the proposal (updatable)"]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "negotiationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional negotiation id if this proposal is a preferred deal proposal."]
    pub negotiation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originatorRole")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the buyer/seller created the proposal.(readonly)"]
    pub originator_role: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privateAuctionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional private auction id if this proposal is a private auction proposal."]
    pub private_auction_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proposalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique id of the proposal. (readonly)."]
    pub proposal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "proposalState")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current state of the proposal. (readonly)"]
    pub proposal_state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The revision number for the proposal (readonly)."]
    pub revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "revisionTimeMs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time (ms since epoch) when the proposal was last revised (readonly)."]
    pub revision_time_ms: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seller")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reference to the seller on the proposal. (readonly, except on create)"]
    pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
    #[serde(rename = "sellerContacts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional contact information of the seller (buyer-readonly)."]
    pub seller_contacts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
}
mod proposal_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#proposal")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublisherProfileApiProto {
    #[serde(rename = "audience")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided info on its audience."]
    pub audience: ::std::option::Option<::std::string::String>,
    #[serde(rename = "buyerPitchStatement")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A pitch statement for the buyer"]
    pub buyer_pitch_statement: ::std::option::Option<::std::string::String>,
    #[serde(rename = "directContact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Direct contact for the publisher profile."]
    pub direct_contact: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exchange")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Exchange where this publisher profile is from. E.g. AdX, Rubicon etc..."]
    pub exchange: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googlePlusLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to publisher's Google+ page."]
    pub google_plus_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isParent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True, if this is the parent profile, which represents all domains owned by the publisher."]
    pub is_parent: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "isPublished")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "True, if this profile is published. Deprecated for state."]
    pub is_published: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(default = "publisher_profile_api_proto_defaults :: kind")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"adexchangebuyer#publisherProfileApiProto\"."]
    pub kind: ::std::string::String,
    #[serde(rename = "logoUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url to the logo for the publisher."]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mediaKitLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The url for additional marketing and sales materials."]
    pub media_kit_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overview")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided overview."]
    pub overview: ::std::option::Option<::std::string::String>,
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pair of (seller.account_id, profile_id) uniquely identifies a publisher profile for a given publisher."]
    pub profile_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "programmaticContact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Programmatic contact for the publisher profile."]
    pub programmatic_contact: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherAppIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of app IDs represented in this pubisher profile. Empty if this is a parent profile. Deprecated in favor of publisher_app."]
    pub publisher_app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "publisherApps")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of apps represented in this pubisher profile. Empty if this is a parent profile."]
    pub publisher_apps:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MobileApplication>>>,
    #[serde(rename = "publisherDomains")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of domains represented in this publisher profile. Empty if this is a parent profile."]
    pub publisher_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "publisherProfileId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique Id for publisher profile."]
    pub publisher_profile_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "publisherProvidedForecast")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided forecasting information."]
    pub publisher_provided_forecast:
        ::std::option::Option<::std::boxed::Box<PublisherProvidedForecast>>,
    #[serde(rename = "rateCardInfoLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link to publisher rate card"]
    pub rate_card_info_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "samplePageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Link for a sample content page."]
    pub sample_page_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "seller")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Seller of the publisher profile."]
    pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the publisher profile."]
    pub state: ::std::option::Option<::std::string::String>,
    #[serde(rename = "topHeadlines")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided key metrics and rankings."]
    pub top_headlines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
mod publisher_profile_api_proto_defaults {
    pub fn kind() -> ::std::string::String {
        String::from("adexchangebuyer#publisherProfileApiProto")
    }
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "This message carries publisher provided forecasting information."]
pub struct PublisherProvidedForecast {
    #[serde(rename = "dimensions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided dimensions. E.g. geo, sizes etc..."]
    pub dimensions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Dimension>>>,
    #[serde(rename = "weeklyImpressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided weekly impressions."]
    pub weekly_impressions: ::std::option::Option<::std::string::String>,
    #[serde(rename = "weeklyUniques")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Publisher provided weekly uniques."]
    pub weekly_uniques: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Seller {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique id for the seller. The seller fills in this field. The seller account id is then available to buyer in the product."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional sub-account id for the seller."]
    pub sub_account_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedTargeting {
    #[serde(rename = "exclusions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of values to exclude from targeting. Each value is AND'd together."]
    pub exclusions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingValue>>>,
    #[serde(rename = "inclusions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of value to include as part of the targeting. Each value is OR'd together."]
    pub inclusions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingValue>>>,
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The key representing the shared targeting criterion."]
    pub key: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValue {
    #[serde(rename = "creativeSizeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The creative size value to exclude/include."]
    pub creative_size_value: ::std::option::Option<::std::boxed::Box<TargetingValueCreativeSize>>,
    #[serde(rename = "dayPartTargetingValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING."]
    pub day_part_targeting_value:
        ::std::option::Option<::std::boxed::Box<TargetingValueDayPartTargeting>>,
    #[serde(rename = "demogAgeCriteriaValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub demog_age_criteria_value:
        ::std::option::Option<::std::boxed::Box<TargetingValueDemogAgeCriteria>>,
    #[serde(rename = "demogGenderCriteriaValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub demog_gender_criteria_value:
        ::std::option::Option<::std::boxed::Box<TargetingValueDemogGenderCriteria>>,
    #[serde(rename = "longValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The long value to exclude/include."]
    pub long_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requestPlatformTargetingValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_platform_targeting_value:
        ::std::option::Option<::std::boxed::Box<TargetingValueRequestPlatformTargeting>>,
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The string value to exclude/include."]
    pub string_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Next Id: 7"]
pub struct TargetingValueCreativeSize {
    #[serde(rename = "allowedFormats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The formats allowed by the publisher."]
    pub allowed_formats: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "companionSizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For video size type, the list of companion sizes."]
    pub companion_sizes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingValueSize>>>,
    #[serde(rename = "creativeSizeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Creative size type."]
    pub creative_size_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nativeTemplate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The native template for native ad."]
    pub native_template: ::std::option::Option<::std::string::String>,
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "For regular or video creative size type, specifies the size of the creative."]
    pub size: ::std::option::Option<::std::boxed::Box<TargetingValueSize>>,
    #[serde(rename = "skippableAdType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The skippable ad type for video size."]
    pub skippable_ad_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueDayPartTargeting {
    #[serde(rename = "dayParts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub day_parts: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<TargetingValueDayPartTargetingDayPart>>,
    >,
    #[serde(rename = "timeZoneType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub time_zone_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueDayPartTargetingDayPart {
    #[serde(rename = "dayOfWeek")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub day_of_week: ::std::option::Option<::std::string::String>,
    #[serde(rename = "endHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "endMinute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub end_minute: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "startMinute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub start_minute: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueDemogAgeCriteria {
    #[serde(rename = "demogAgeCriteriaIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub demog_age_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueDemogGenderCriteria {
    #[serde(rename = "demogGenderCriteriaIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub demog_gender_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueRequestPlatformTargeting {
    #[serde(rename = "requestPlatforms")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub request_platforms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetingValueSize {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The height of the creative."]
    pub height: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The width of the creative."]
    pub width: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdatePrivateAuctionProposalRequest {
    #[serde(rename = "externalDealId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The externalDealId of the deal to be updated."]
    pub external_deal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "note")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional note to be added."]
    pub note: ::std::option::Option<::std::boxed::Box<MarketplaceNote>>,
    #[serde(rename = "proposalRevisionNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The current revision number of the proposal to be updated."]
    pub proposal_revision_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "updateAction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The proposed action on the private auction proposal."]
    pub update_action: ::std::option::Option<::std::string::String>,
}
