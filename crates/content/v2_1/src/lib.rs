#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account data. After the creation of a new account it may take a few minutes before it is fully operational. The methods delete, insert, and update require the admin role."]
pub struct Account {
    #[serde(rename = "adsLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Linked Ads accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the Ads interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list."]
    pub ads_links: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountAdsLink>>>,
    #[serde(rename = "adultContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the merchant sells adult content."]
    pub adult_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "automaticLabelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Automatically created label IDs that are assigned to the account by CSS Center."]
    pub automatic_label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "businessInformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The business information of the account."]
    pub business_information: ::std::option::Option<::std::boxed::Box<AccountBusinessInformation>>,
    #[serde(rename = "cssId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of CSS the account belongs to."]
    pub css_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleMyBusinessLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GMB account which is linked or in the process of being linked with the Merchant Center account."]
    pub google_my_business_link:
        ::std::option::Option<::std::boxed::Box<AccountGoogleMyBusinessLink>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for update. Merchant Center account ID."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#account`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Manually created label IDs that are assigned to the account by CSS."]
    pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name for the account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Client-specific, locally-unique, internal ID for the child account."]
    pub seller_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "users")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Users with access to the account. Every account (except for subaccounts) must have at least one admin user."]
    pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountUser>>>,
    #[serde(rename = "websiteUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The merchant's website."]
    pub website_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "youtubeChannelLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list."]
    pub youtube_channel_links:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountYouTubeChannelLink>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountAddress {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CLDR country code (e.g. \"US\"). This value cannot be set for a sub-account of an MCA. All MCA sub-accounts inherit the country of their parent MCA."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal code or ZIP (e.g. \"94043\")."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Street-level part of the address."]
    pub street_address: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountAdsLink {
    #[serde(rename = "adsId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer ID of the Ads account."]
    pub ads_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the link between this Merchant Center account and the Ads account. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in Google Ads or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending. Acceptable values are: - \"`active`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountBusinessInformation {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The address of the business."]
    pub address: ::std::option::Option<::std::boxed::Box<AccountAddress>>,
    #[serde(rename = "customerService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The customer service information of the business."]
    pub customer_service: ::std::option::Option<::std::boxed::Box<AccountCustomerService>>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number of the business."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountCustomerService {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer service email."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer service phone number."]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer service URL."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountGoogleMyBusinessLink {
    #[serde(rename = "gmbAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the GMB account. If this is provided, then `gmbEmail` is ignored. The value of this field should match the `accountId` used by the GMB API."]
    pub gmb_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gmbEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GMB email address of which a specific account within a GMB account. A sample account within a GMB account could be a business account with set of locations, managed under the GMB account."]
    pub gmb_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the link between this Merchant Center account and the GMB account. Acceptable values are: - \"`active`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountIdentifier {
    #[serde(rename = "aggregatorId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The aggregator ID, set for aggregators and subaccounts (in that case, it represents the aggregator of the subaccount)."]
    pub aggregator_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The merchant account ID, set for individual accounts and subaccounts."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Label assigned by CSS domain or CSS group to one of its sub-accounts."]
pub struct AccountLabel {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Immutable. The ID of account this label belongs to."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of this label."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID of the label."]
    pub label_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The type of this label."]
    pub label_type: ::std::option::Option<AccountLabelLabelTypeEnum>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of this label."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Output only. The type of this label."]
pub enum AccountLabelLabelTypeEnum {
    #[serde(rename = "LABEL_TYPE_UNSPECIFIED")]
    #[doc = "Unknown label type."]
    LabelTypeUnspecified,
    #[serde(rename = "MANUAL")]
    #[doc = "Indicates that the label was created manually."]
    Manual,
    #[serde(rename = "AUTOMATIC")]
    #[doc = "Indicates that the label was created automatically by CSS Center."]
    Automatic,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " The return carrier information. This service is designed for merchants enrolled in the Buy on Google program. "]
pub struct AccountReturnCarrier {
    #[serde(rename = "carrierAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The Google-provided unique carrier ID, used to update the resource."]
    pub carrier_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "carrierAccountName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the carrier account."]
    pub carrier_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "carrierAccountNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of the carrier account."]
    pub carrier_account_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "carrierCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier code enum. Accepts the values FEDEX or UPS."]
    pub carrier_code: ::std::option::Option<AccountReturnCarrierCarrierCodeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The carrier code enum. Accepts the values FEDEX or UPS."]
pub enum AccountReturnCarrierCarrierCodeEnum {
    #[serde(rename = "CARRIER_CODE_UNSPECIFIED")]
    #[doc = "Carrier not specified"]
    CarrierCodeUnspecified,
    #[serde(rename = "FEDEX")]
    #[doc = "FedEx carrier"]
    Fedex,
    #[serde(rename = "UPS")]
    #[doc = "UPS carrier"]
    Ups,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of an account, i.e., information about its products, which is computed offline and not returned immediately at insertion time."]
pub struct AccountStatus {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account for which the status is reported."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountLevelIssues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of account level issues."]
    pub account_level_issues:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusAccountLevelIssue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accountStatus`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "products")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes."]
    pub products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusProducts>>>,
    #[serde(rename = "websiteClaimed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the account's website is claimed or not."]
    pub website_claimed: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountStatusAccountLevelIssue {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country for which this issue is reported."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination the issue applies to. If this field is empty then the issue applies to all available destinations."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional details about the issue."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of a web page to help resolving this issue."]
    pub documentation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Issue identifier."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Severity of the issue. Acceptable values are: - \"`critical`\" - \"`error`\" - \"`suggestion`\" "]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Short description of the issue."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountStatusItemLevelIssue {
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attribute's name, if the issue is caused by a single attribute."]
    pub attribute_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code of the issue."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short issue description in English."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed issue description in English."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of a web page to help with resolving this issue."]
    pub documentation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items with this issue."]
    pub num_items: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the issue can be resolved by the merchant."]
    pub resolution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How this issue affects serving of the offer."]
    pub servability: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountStatusProducts {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The channel the data applies to. Acceptable values are: - \"`local`\" - \"`online`\" "]
    pub channel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country the data applies to."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination the data applies to."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemLevelIssues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of item-level issues."]
    pub item_level_issues:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusItemLevelIssue>>>,
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Aggregated product statistics."]
    pub statistics: ::std::option::Option<::std::boxed::Box<AccountStatusStatistics>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountStatusStatistics {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of active offers."]
    pub active: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disapproved")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of disapproved offers."]
    pub disapproved: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expiring")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of expiring offers."]
    pub expiring: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pending")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of pending offers."]
    pub pending: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The tax settings of a merchant account. All methods require the admin role."]
pub struct AccountTax {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the account to which these account tax settings belong."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountTax\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax rules. Updating the tax rules will enable US taxes (not reversible). Defining no rules is equivalent to not charging tax at all."]
    pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountTaxTaxRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Tax calculation rule to apply in a state or province (USA only)."]
pub struct AccountTaxTaxRule {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code in which tax is applicable."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. State (or province) is which the tax is applicable, described by its location ID (also called criteria ID)."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ratePercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Explicit tax rate in percent, represented as a floating point number without the percentage character. Must not be negative."]
    pub rate_percent: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shippingTaxed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, shipping charges are also taxed."]
    pub shipping_taxed: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "useGlobalRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the tax rate is taken from a global tax table or specified explicitly."]
    pub use_global_rate: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountUser {
    #[serde(rename = "admin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether user is an admin."]
    pub admin: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "emailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "User's email address."]
    pub email_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderManager")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether user is an order manager."]
    pub order_manager: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paymentsAnalyst")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether user can access payment statements."]
    pub payments_analyst: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "paymentsManager")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether user can manage payment settings."]
    pub payments_manager: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountYouTubeChannelLink {
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Channel ID."]
    pub channel_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the link between this Merchant Center account and the YouTube channel. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in YT Creator Studio or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsAuthInfoResponse {
    #[serde(rename = "accountIdentifiers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account identifiers corresponding to the authenticated user. - For an individual account: only the merchant ID is defined - For an aggregator: only the aggregator ID is defined - For a subaccount of an MCA: both the merchant ID and the aggregator ID are defined. "]
    pub account_identifiers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountIdentifier>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsAuthInfoResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsClaimWebsiteResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsClaimWebsiteResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountsCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accounts request."]
pub struct AccountsCustomBatchRequestEntry {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account to create or update. Only defined if the method is `insert` or `update`."]
    pub account: ::std::option::Option<::std::boxed::Box<Account>>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the targeted account. Only defined if the method is not `insert`."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the account should be deleted if the account has offers. Only applicable if the method is `delete`."]
    pub force: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label IDs for the 'updatelabels' request."]
    pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "linkRequest")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details about the `link` request."]
    pub link_request:
        ::std::option::Option<::std::boxed::Box<AccountsCustomBatchRequestEntryLinkRequest>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`claimWebsite`\" - \"`delete`\" - \"`get`\" - \"`insert`\" - \"`link`\" - \"`update`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "overwrite")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Only applicable if the method is `claimwebsite`. Indicates whether or not to take the claim from another account in case there is a conflict."]
    pub overwrite: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "view")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Controls which fields are visible. Only applicable if the method is 'get'."]
    pub view: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsCustomBatchRequestEntryLinkRequest {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Action to perform for this link. The `\"request\"` action is only available to select merchants. Acceptable values are: - \"`approve`\" - \"`remove`\" - \"`request`\" "]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the link between the two accounts. Acceptable values are: - \"`channelPartner`\" - \"`eCommercePlatform`\" "]
    pub link_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkedAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the linked account."]
    pub linked_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provided services. Acceptable values are: - \"`shoppingAdsProductManagement`\" - \"`shoppingActionsProductManagement`\" - \"`shoppingActionsOrderManagement`\" "]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountsCustomBatchResponseEntry>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accounts response."]
pub struct AccountsCustomBatchResponseEntry {
    #[serde(rename = "account")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved, created, or updated account. Not defined if the method was `delete`, `claimwebsite` or `link`."]
    pub account: ::std::option::Option<::std::boxed::Box<Account>>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accountsCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsLinkRequest {
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Action to perform for this link. The `\"request\"` action is only available to select merchants. Acceptable values are: - \"`approve`\" - \"`remove`\" - \"`request`\" "]
    pub action: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the link between the two accounts. Acceptable values are: - \"`channelPartner`\" - \"`eCommercePlatform`\" "]
    pub link_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "linkedAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the linked account."]
    pub linked_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " Acceptable values are: - \"`shoppingAdsProductManagement`\" - \"`shoppingAdsOther`\" - \"`shoppingActionsProductManagement`\" - \"`shoppingActionsOrderManagement`\" - \"`shoppingActionsOther`\" "]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsLinkResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsLinkResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsListLinksResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsListLinksResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of available links."]
    pub links: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LinkedAccount>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of links."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of accounts."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsUpdateLabelsRequest {
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The IDs of labels that should be assigned to the account."]
    pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountsUpdateLabelsResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsUpdateLabelsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountstatusesCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<AccountstatusesCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accountstatuses request."]
pub struct AccountstatusesCustomBatchRequestEntry {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the (sub-)account whose status to get."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
    pub destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
    pub method: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountstatusesCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<AccountstatusesCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountstatusesCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accountstatuses response."]
pub struct AccountstatusesCustomBatchResponseEntry {
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested account status. Defined if and only if the request was successful."]
    pub account_status: ::std::option::Option<::std::boxed::Box<AccountStatus>>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountstatusesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountstatusesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of account statuses."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccounttaxCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<AccounttaxCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accounttax request."]
pub struct AccounttaxCustomBatchRequestEntry {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account for which to get/update account tax settings."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "accountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account tax settings to update. Only defined if the method is `update`."]
    pub account_tax: ::std::option::Option<::std::boxed::Box<AccountTax>>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`update`\" "]
    pub method: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccounttaxCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<AccounttaxCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accounttaxCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch accounttax response."]
pub struct AccounttaxCustomBatchResponseEntry {
    #[serde(rename = "accountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved or updated account tax settings."]
    pub account_tax: ::std::option::Option<::std::boxed::Box<AccountTax>>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accounttaxCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccounttaxListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accounttaxListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of account tax settings."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountTax>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Amount {
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The pre-tax or post-tax price depending on the location of the order."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Tax value."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BusinessDayConfig {
    #[serde(rename = "businessDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Regular business days. May not be empty."]
    pub business_days: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierRate {
    #[serde(rename = "carrierName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Carrier service, such as `\"UPS\"` or `\"Fedex\"`. The list of supported carriers can be retrieved via the `getSupportedCarriers` method. Required."]
    pub carrier_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "carrierService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Carrier service, such as `\"ground\"` or `\"2 days\"`. The list of supported services for a carrier can be retrieved via the `getSupportedCarriers` method. Required."]
    pub carrier_service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flatAdjustment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additive shipping rate modifier. Can be negative. For example `{ \"value\": \"1\", \"currency\" : \"USD\" }` adds $1 to the rate, `{ \"value\": \"-3\", \"currency\" : \"USD\" }` removes $3 from the rate. Optional."]
    pub flat_adjustment: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the carrier rate. Must be unique per rate group. Required."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originPostalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipping origin for this carrier rate. Required."]
    pub origin_postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "percentageAdjustment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Multiplicative shipping rate modifier as a number in decimal notation. Can be negative. For example `\"5.4\"` increases the rate by 5.4%, `\"-3\"` decreases the rate by 3%. Optional."]
    pub percentage_adjustment: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarriersCarrier {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR country code of the carrier (e.g., \"US\"). Always present."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the carrier (e.g., `\"UPS\"`). Always present."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of supported services (e.g., `\"ground\"`) for that carrier. Contains at least one service."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collection message."]
pub struct Collection {
    #[serde(rename = "customLabel0")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns. [Custom label](https://support.google.com/merchants/answer/9674217)"]
    pub custom_label0: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns."]
    pub custom_label1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns."]
    pub custom_label2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns."]
    pub custom_label3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel4")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Label that you assign to a collection to help organize bidding and reporting in Shopping campaigns."]
    pub custom_label4: ::std::option::Option<::std::string::String>,
    #[serde(rename = "featuredProduct")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "This identifies one or more products associated with the collection. Used as a lookup to the corresponding product ID in your product feeds. Provide a maximum of 100 featuredProduct (for collections). Provide up to 10 featuredProduct (for Shoppable Images only) with ID and X and Y coordinates. [featured_product attribute](https://support.google.com/merchants/answer/9703736)"]
    pub featured_product:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionFeaturedProduct>>>,
    #[serde(rename = "headline")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Your collection's name. [headline attribute](https://support.google.com/merchants/answer/9673580)"]
    pub headline: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The REST ID of the collection. Content API methods that operate on collections take this as their collectionId parameter. The REST ID for a collection is of the form collectionId. [id attribute](https://support.google.com/merchants/answer/9649290)"]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of a collection’s image. [image_link attribute](https://support.google.com/merchants/answer/9703236)"]
    pub image_link: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language of a collection and the language of any featured products linked to the collection. [language attribute](https://support.google.com/merchants/answer/9673781)"]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection’s landing page. URL directly linking to your collection's page on your website. [link attribute](https://support.google.com/merchants/answer/9673983)"]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A collection’s mobile-optimized landing page when you have a different URL for mobile and desktop traffic. [mobile_link attribute](https://support.google.com/merchants/answer/9646123)"]
    pub mobile_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[product_country attribute](https://support.google.com/merchants/answer/9674155)"]
    pub product_country: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The message for FeaturedProduct. [FeaturedProduct](https://support.google.com/merchants/answer/9703736)"]
pub struct CollectionFeaturedProduct {
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique identifier for the product item."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. X-coordinate of the product callout on the Shoppable Image."]
    pub x: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Y-coordinate of the product callout on the Shoppable Image."]
    pub y: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The collectionstatus message."]
pub struct CollectionStatus {
    #[serde(rename = "collectionLevelIssuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all issues associated with the collection."]
    pub collection_level_issuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionStatusItemLevelIssue>>>,
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the collection has been created in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format: Date, time, and offset, e.g. \"2020-01-02T09:00:00+01:00\" or \"2020-01-02T09:00:00Z\""]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intended destinations for the collection."]
    pub destination_statuses: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<CollectionStatusDestinationStatus>>,
    >,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the collection for which status is reported."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdateDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the collection has been last updated in [ISO 8601](http://en.wikipedia.org/wiki/ISO_8601) format: Date, time, and offset, e.g. \"2020-01-02T09:00:00+01:00\" or \"2020-01-02T09:00:00Z\""]
    pub last_update_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Destination status message."]
pub struct CollectionStatusDestinationStatus {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the destination"]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status for the specified destination."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Issue associated with the collection."]
pub struct CollectionStatusItemLevelIssue {
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attribute's name, if the issue is caused by a single attribute."]
    pub attribute_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code of the issue."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short issue description in English."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination the issue applies to."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed issue description in English."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of a web page to help with resolving this issue."]
    pub documentation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the issue can be resolved by the merchant."]
    pub resolution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How this issue affects the serving of the collection."]
    pub servability: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Information about CSS domain."]
pub struct Css {
    #[serde(rename = "cssDomainId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The CSS domain ID."]
    pub css_domain_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "cssGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The ID of the CSS group this CSS domain is affiliated with. Only populated for CSS group users."]
    pub css_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The CSS domain's display name, used when space is constrained."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The CSS domain's full name."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "homepageUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The CSS domain's homepage."]
    pub homepage_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of label IDs that are assigned to this CSS domain by its CSS group. Only populated for CSS group users."]
    pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomAttribute {
    #[serde(rename = "groupValues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Subattributes within this attribute group. Exactly one of value or groupValues must be provided."]
    pub group_values: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute. Underscores will be replaced by spaces upon insertion."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the attribute."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomerReturnReason {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the reason."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code of the return reason. Acceptable values are: - \"`betterPriceFound`\" - \"`changedMind`\" - \"`damagedOrDefectiveItem`\" - \"`didNotMatchDescription`\" - \"`doesNotFit`\" - \"`expiredItem`\" - \"`incorrectItemReceived`\" - \"`noLongerNeeded`\" - \"`notSpecified`\" - \"`orderedWrongItem`\" - \"`other`\" - \"`qualityNotExpected`\" - \"`receivedTooLate`\" - \"`undeliverable`\" "]
    pub reason_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CutoffTime {
    #[serde(rename = "hour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hour of the cutoff time until which an order has to be placed to be processed in the same day. Required."]
    pub hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minute")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minute of the cutoff time until which an order has to be placed to be processed in the same day. Required."]
    pub minute: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timezone identifier for the cutoff time. A list of identifiers can be found in the AdWords API documentation. E.g. \"Europe/Zurich\". Required."]
    pub timezone: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Datafeed configuration data."]
pub struct Datafeed {
    #[serde(rename = "attributeLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO 639-1 language in which the attributes are defined in the data feed."]
    pub attribute_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - \"`local products`\" - \"`product inventory`\" - \"`products`\" "]
    pub content_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fetchSchedule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fetch schedule for the feed file."]
    pub fetch_schedule: ::std::option::Option<::std::boxed::Box<DatafeedFetchSchedule>>,
    #[serde(rename = "fileName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The filename of the feed. All feeds must have a unique file name."]
    pub file_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Format of the feed file."]
    pub format: ::std::option::Option<::std::boxed::Box<DatafeedFormat>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for update. The ID of the data feed."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#datafeed`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for insert. A descriptive name of the data feed."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targets")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The targets this feed should apply to (country, language, destinations)."]
    pub targets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedTarget>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The required fields vary based on the frequency of fetching. For a monthly fetch schedule, day_of_month and hour are required. For a weekly fetch schedule, weekday and hour are required. For a daily fetch schedule, only hour is required."]
pub struct DatafeedFetchSchedule {
    #[serde(rename = "dayOfMonth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The day of the month the feed file should be fetched (1-31)."]
    pub day_of_month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "fetchUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL where the feed file can be fetched. Google Merchant Center will support automatic scheduled uploads using the HTTP, HTTPS, FTP, or SFTP protocols, so the value will need to be a valid link using one of those four protocols."]
    pub fetch_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The hour of the day the feed file should be fetched (0-23)."]
    pub hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minuteOfHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minute of the hour the feed file should be fetched (0-59). Read-only."]
    pub minute_of_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional password for fetch_url."]
    pub password: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paused")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the scheduled fetch is paused or not."]
    pub paused: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time zone used for schedule. UTC by default. E.g., \"America/Los_Angeles\"."]
    pub time_zone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional user name for fetch_url."]
    pub username: ::std::option::Option<::std::string::String>,
    #[serde(rename = "weekday")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The day of the week the feed file should be fetched. Acceptable values are: - \"`monday`\" - \"`tuesday`\" - \"`wednesday`\" - \"`thursday`\" - \"`friday`\" - \"`saturday`\" - \"`sunday`\" "]
    pub weekday: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedFormat {
    #[serde(rename = "columnDelimiter")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delimiter for the separation of values in a delimiter-separated values feed. If not specified, the delimiter will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - \"`pipe`\" - \"`tab`\" - \"`tilde`\" "]
    pub column_delimiter: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileEncoding")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Character encoding scheme of the data feed. If not specified, the encoding will be auto-detected. Acceptable values are: - \"`latin-1`\" - \"`utf-16be`\" - \"`utf-16le`\" - \"`utf-8`\" - \"`windows-1252`\" "]
    pub file_encoding: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quotingMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies how double quotes are interpreted. If not specified, the mode will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - \"`normal character`\" - \"`value quoting`\" "]
    pub quoting_mode: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a datafeed, i.e., the result of the last retrieval of the datafeed computed asynchronously when the feed processing is finished."]
pub struct DatafeedStatus {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country for which the status is reported, represented as a CLDR territory code."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datafeedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the feed for which the status is reported."]
    pub datafeed_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of errors occurring in the feed."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusError>>>,
    #[serde(rename = "itemsTotal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of items in the feed that were processed."]
    pub items_total: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemsValid")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of items in the feed that were valid."]
    pub items_valid: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#datafeedStatus`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO 639-1 language for which the status is reported."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUploadDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last date at which the feed was uploaded."]
    pub last_upload_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "processingStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The processing status of the feed. Acceptable values are: - \"`\"`failure`\": The feed could not be processed or all items had errors.`\" - \"`in progress`\": The feed is being processed. - \"`none`\": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - \"`success`\": The feed was processed successfully, though some items might have had errors. "]
    pub processing_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of errors occurring in the feed."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusError>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An error occurring in the feed, like \"invalid price\"."]
pub struct DatafeedStatusError {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The code of the error, e.g., \"validation/invalid_value\"."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of occurrences of the error in the feed."]
    pub count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "examples")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of example occurrences of the error, grouped by product."]
    pub examples: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusExample>>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error message, e.g., \"Invalid price\"."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An example occurrence for a particular error."]
pub struct DatafeedStatusExample {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the example item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line number in the data feed where the example is found."]
    pub line_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The problematic value."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedTarget {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country where the items in the feed will be included in the search index, represented as a CLDR territory code."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of destinations to exclude for this target (corresponds to unchecked check boxes in Merchant Center)."]
    pub excluded_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of destinations to include for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. List of supported destinations (if available to the account): - DisplayAds - Shopping - ShoppingActions - SurfacesAcrossGoogle "]
    pub included_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targets[].country`."]
    pub language: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedsCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedsCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch datafeeds request."]
pub struct DatafeedsCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "datafeed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The data feed to insert."]
    pub datafeed: ::std::option::Option<::std::boxed::Box<Datafeed>>,
    #[serde(rename = "datafeedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the data feed to get, delete or fetch."]
    pub datafeed_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`fetchNow`\" - \"`get`\" - \"`insert`\" - \"`update`\" "]
    pub method: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedsCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DatafeedsCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch datafeeds response."]
pub struct DatafeedsCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "datafeed")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested data feed. Defined if and only if the request was successful."]
    pub datafeed: ::std::option::Option<::std::boxed::Box<Datafeed>>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedsFetchNowResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsFetchNowResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of datafeeds."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Datafeed>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedstatusesCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DatafeedstatusesCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch datafeedstatuses request."]
pub struct DatafeedstatusesCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country for which to get the datafeed status. If this parameter is provided then language must also be provided. Note that for multi-target datafeeds this parameter is required."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "datafeedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the data feed to get."]
    pub datafeed_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The language for which to get the datafeed status. If this parameter is provided then country must also be provided. Note that for multi-target datafeeds this parameter is required."]
    pub language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
    pub method: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedstatusesCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<DatafeedstatusesCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedstatusesCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch datafeedstatuses response."]
pub struct DatafeedstatusesCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "datafeedStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested data feed status. Defined if and only if the request was successful."]
    pub datafeed_status: ::std::option::Option<::std::boxed::Box<DatafeedStatus>>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatafeedstatusesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedstatusesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of datafeed statuses."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
pub struct Date {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents civil time (or occasionally physical time). This type can represent a civil time in one of a few possible ways: * When utc_offset is set and time_zone is unset: a civil time on a calendar day with a particular offset from UTC. * When time_zone is set and utc_offset is unset: a civil time on a calendar day in a particular time zone. * When neither time_zone nor utc_offset is set: a civil time on a calendar day in local time. The date is relative to the Proleptic Gregorian Calendar. If year is 0, the DateTime is considered not to have a specific year. month and day must have valid, non-zero values. This type may also be used to represent a physical time if all the date and time fields are set and either case of the `time_offset` oneof is set. Consider using `Timestamp` message for physical time instead. If your use case also would like to store the user's timezone, that can be done in another field. This type is more flexible than some applications may want. Make sure to document and validate your application's limitations."]
pub struct DateTime {
    #[serde(rename = "day")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Day of month. Must be from 1 to 31 and valid for the year and month."]
    pub day: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "hours")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
    pub hours: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Minutes of hour of day. Must be from 0 to 59."]
    pub minutes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "month")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Month of year. Must be from 1 to 12."]
    pub month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "nanos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
    pub nanos: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "seconds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
    pub seconds: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time zone."]
    pub time_zone: ::std::option::Option<::std::boxed::Box<TimeZone>>,
    #[serde(rename = "utcOffset")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "UTC offset. Must be whole seconds, between -18 hours and +18 hours. For example, a UTC offset of -4:00 would be represented as { seconds: -14400 }."]
    pub utc_offset: ::std::option::Option<::std::string::String>,
    #[serde(rename = "year")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. Year of date. Must be from 1 to 9999, or 0 if specifying a datetime without a year."]
    pub year: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeliveryTime {
    #[serde(rename = "cutoffTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Business days cutoff time definition. If not configured the cutoff time will be defaulted to 8AM PST."]
    pub cutoff_time: ::std::option::Option<::std::boxed::Box<CutoffTime>>,
    #[serde(rename = "handlingBusinessDayConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The business days during which orders can be handled. If not provided, Monday to Friday business days will be assumed."]
    pub handling_business_day_config: ::std::option::Option<::std::boxed::Box<BusinessDayConfig>>,
    #[serde(rename = "holidayCutoffs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Holiday cutoff definitions. If configured, they specify order cutoff times for holiday-specific shipping."]
    pub holiday_cutoffs: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HolidayCutoff>>>,
    #[serde(rename = "maxHandlingTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped. Must be greater than or equal to `minHandlingTimeInDays`."]
    pub max_handling_time_in_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "maxTransitTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Must be greater than or equal to `minTransitTimeInDays`."]
    pub max_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minHandlingTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped."]
    pub min_handling_time_in_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minTransitTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Either `{min,max}TransitTimeInDays` or `transitTimeTable` must be set, but not both."]
    pub min_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "transitBusinessDayConfig")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The business days during which orders can be in-transit. If not provided, Monday to Friday business days will be assumed."]
    pub transit_business_day_config: ::std::option::Option<::std::boxed::Box<BusinessDayConfig>>,
    #[serde(rename = "transitTimeTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transit time table, number of business days spent in transit based on row and column dimensions. Either `{min,max}TransitTimeInDays` or `transitTimeTable` can be set, but not both."]
    pub transit_time_table: ::std::option::Option<::std::boxed::Box<TransitTable>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "An error returned by the API."]
pub struct Error {
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The domain of the error."]
    pub domain: ::std::option::Option<::std::string::String>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A description of the error."]
    pub message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code."]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of errors returned by a failed batch entry."]
pub struct Errors {
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The HTTP status of the first error in `errors`."]
    pub code: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors."]
    pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The message of the first error in `errors`."]
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmbAccounts {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Merchant Center account."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gmbAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of GMB accounts which are available to the merchant."]
    pub gmb_accounts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmbAccountsGmbAccount>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GmbAccountsGmbAccount {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email which identifies the GMB account."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "listingCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of listings under this account."]
    pub listing_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the GMB account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the GMB account (User or Business)."]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A non-empty list of row or column headers for a table. Exactly one of `prices`, `weights`, `numItems`, `postalCodeGroupNames`, or `location` must be set."]
pub struct Headers {
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of location ID sets. Must be non-empty. Can only be set if all other fields are not set."]
    pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocationIdSet>>>,
    #[serde(rename = "numberOfItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of inclusive number of items upper bounds. The last value can be `\"infinity\"`. For example `[\"10\", \"50\", \"infinity\"]` represents the headers \"<= 10 items\", \"<= 50 items\", and \"> 50 items\". Must be non-empty. Can only be set if all other fields are not set."]
    pub number_of_items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "postalCodeGroupNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of postal group names. The last value can be `\"all other locations\"`. Example: `[\"zone 1\", \"zone 2\", \"all other locations\"]`. The referred postal code groups must match the delivery country of the service. Must be non-empty. Can only be set if all other fields are not set."]
    pub postal_code_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "prices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of inclusive order price upper bounds. The last price's value can be `\"infinity\"`. For example `[{\"value\": \"10\", \"currency\": \"USD\"}, {\"value\": \"500\", \"currency\": \"USD\"}, {\"value\": \"infinity\", \"currency\": \"USD\"}]` represents the headers \"<= $10\", \"<= $500\", and \"> $500\". All prices within a service must have the same currency. Must be non-empty. Can only be set if all other fields are not set."]
    pub prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Price>>>,
    #[serde(rename = "weights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of inclusive order weight upper bounds. The last weight's value can be `\"infinity\"`. For example `[{\"value\": \"10\", \"unit\": \"kg\"}, {\"value\": \"50\", \"unit\": \"kg\"}, {\"value\": \"infinity\", \"unit\": \"kg\"}]` represents the headers \"<= 10kg\", \"<= 50kg\", and \"> 50kg\". All weights within a service must have the same unit. Must be non-empty. Can only be set if all other fields are not set."]
    pub weights: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Weight>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HolidayCutoff {
    #[serde(rename = "deadlineDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of the order deadline, in ISO 8601 format. E.g. \"2016-11-29\" for 29th November 2016. Required."]
    pub deadline_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deadlineHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hour of the day on the deadline date until which the order has to be placed to qualify for the delivery guarantee. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Required."]
    pub deadline_hour: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "deadlineTimezone")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timezone identifier for the deadline hour. A list of identifiers can be found in the AdWords API documentation. E.g. \"Europe/Zurich\". Required."]
    pub deadline_timezone: ::std::option::Option<::std::string::String>,
    #[serde(rename = "holidayId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the holiday. Required."]
    pub holiday_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "visibleFromDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the deadline will become visible to consumers in ISO 8601 format. E.g. \"2016-10-31\" for 31st October 2016. Required."]
    pub visible_from_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HolidaysHoliday {
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory code of the country in which the holiday is available. E.g. \"US\", \"DE\", \"GB\". A holiday cutoff can only be configured in a shipping settings service with matching delivery country. Always present."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of the holiday, in ISO 8601 format. E.g. \"2016-12-25\" for Christmas 2016. Always present."]
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryGuaranteeDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the order has to arrive at the customer's, in ISO 8601 format. E.g. \"2016-12-24\" for 24th December 2016. Always present."]
    pub delivery_guarantee_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryGuaranteeHour")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Hour of the day in the delivery location's timezone on the guaranteed delivery date by which the order has to arrive at the customer's. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Always present."]
    pub delivery_guarantee_hour: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unique identifier for the holiday to be used when configuring holiday cutoffs. Always present."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The holiday type. Always present. Acceptable values are: - \"`Christmas`\" - \"`Easter`\" - \"`Father's Day`\" - \"`Halloween`\" - \"`Independence Day (USA)`\" - \"`Mother's Day`\" - \"`Thanksgiving`\" - \"`Valentine's Day`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Map of inapplicability details."]
pub struct InapplicabilityDetails {
    #[serde(rename = "inapplicableCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Count of this inapplicable reason code."]
    pub inapplicable_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inapplicableReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Reason code this rule was not applicable."]
    pub inapplicable_reason: ::std::option::Option<InapplicabilityDetailsInapplicableReasonEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Reason code this rule was not applicable."]
pub enum InapplicabilityDetailsInapplicableReasonEnum {
    #[serde(rename = "INAPPLICABLE_REASON_UNSPECIFIED")]
    #[doc = "Default value. Should not be used."]
    InapplicableReasonUnspecified,
    #[serde(rename = "CANNOT_BEAT_BUYBOX_WINNER")]
    #[doc = "The rule set for this product cannot beat the buybox winner."]
    CannotBeatBuyboxWinner,
    #[serde(rename = "ALREADY_WINNING_BUYBOX")]
    #[doc = "This product can already win the buybox without rule."]
    AlreadyWinningBuybox,
    #[serde(rename = "TRIUMPHED_OVER_BY_SAME_TYPE_RULE")]
    #[doc = "Another rule of the same type takes precedence over this one."]
    TriumphedOverBySameTypeRule,
    #[serde(rename = "TRIUMPHED_OVER_BY_OTHER_RULE_ON_OFFER")]
    #[doc = "Another rule of a different type takes precedence over this one."]
    TriumphedOverByOtherRuleOnOffer,
    #[serde(rename = "RESTRICTIONS_NOT_MET")]
    #[doc = "The rule restrictions are not met. For example, this may be the case if the calculated rule price is lower than floor price in the restriction."]
    RestrictionsNotMet,
    #[serde(rename = "UNCATEGORIZED")]
    #[doc = "The reason is not categorized to any known reason."]
    Uncategorized,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Installment {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount the buyer has to pay per month."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "months")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of installments the buyer has to pay."]
    pub months: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvoiceSummary {
    #[serde(rename = "additionalChargeSummaries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Summary of the total amounts of the additional charges."]
    pub additional_charge_summaries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<InvoiceSummaryAdditionalChargeSummary>>,
    >,
    #[serde(rename = "productTotal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Total price for the product."]
    pub product_total: ::std::option::Option<::std::boxed::Box<Amount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InvoiceSummaryAdditionalChargeSummary {
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Total additional charge for this type."]
    pub total_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Type of the additional charge. Acceptable values are: - \"`shipping`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The IDs of labels that should be assigned to the CSS domain."]
pub struct LabelIds {
    #[serde(rename = "labelIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of label IDs."]
    pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiaAboutPageSettings {
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the verification process for the About page. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL for the About page."]
    pub url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiaCountrySettings {
    #[serde(rename = "about")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The settings for the About page."]
    pub about: ::std::option::Option<::std::boxed::Box<LiaAboutPageSettings>>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. CLDR country code (e.g. \"US\")."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hostedLocalStorefrontActive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the \"Merchant hosted local storefront\" feature."]
    pub hosted_local_storefront_active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LIA inventory verification settings."]
    pub inventory: ::std::option::Option<::std::boxed::Box<LiaInventorySettings>>,
    #[serde(rename = "onDisplayToOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "LIA \"On Display To Order\" settings."]
    pub on_display_to_order: ::std::option::Option<::std::boxed::Box<LiaOnDisplayToOrderSettings>>,
    #[serde(rename = "posDataProvider")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The POS data provider linked with this country."]
    pub pos_data_provider: ::std::option::Option<::std::boxed::Box<LiaPosDataProvider>>,
    #[serde(rename = "storePickupActive")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the \"Store pickup\" feature."]
    pub store_pickup_active: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiaInventorySettings {
    #[serde(rename = "inventoryVerificationContactEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The email of the contact for the inventory verification process."]
    pub inventory_verification_contact_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventoryVerificationContactName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the contact for the inventory verification process."]
    pub inventory_verification_contact_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inventoryVerificationContactStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the verification contact. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub inventory_verification_contact_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the inventory verification process. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiaOnDisplayToOrderSettings {
    #[serde(rename = "shippingCostPolicyUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipping cost and policy URL."]
    pub shipping_cost_policy_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the ?On display to order? feature. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiaPosDataProvider {
    #[serde(rename = "posDataProviderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the POS data provider."]
    pub pos_data_provider_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posExternalAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID by which this merchant is known to the POS data provider."]
    pub pos_external_account_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Local Inventory ads (LIA) settings. All methods except listposdataproviders require the admin role."]
pub struct LiaSettings {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "countrySettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The LIA settings for each country."]
    pub country_settings:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiaCountrySettings>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#liaSettings`\""]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<LiasettingsCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsCustomBatchRequestEntry {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account for which to get/update account LIA settings."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory validation contact email. Required only for SetInventoryValidationContact."]
    pub contact_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Inventory validation contact name. Required only for SetInventoryValidationContact."]
    pub contact_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country code. Required only for RequestInventoryVerification."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gmbEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The GMB account. Required only for RequestGmbAccess."]
    pub gmb_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "liaSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account Lia settings to update. Only defined if the method is `update`."]
    pub lia_settings: ::std::option::Option<::std::boxed::Box<LiaSettings>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`getAccessibleGmbAccounts`\" - \"`requestGmbAccess`\" - \"`requestInventoryVerification`\" - \"`setInventoryVerificationContact`\" - \"`update`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posDataProviderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of POS data provider. Required only for SetPosProvider."]
    pub pos_data_provider_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posExternalAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account ID by which this merchant is known to the POS provider."]
    pub pos_external_account_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<LiasettingsCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry to which this entry responds."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if, and only if, the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "gmbAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of accessible GMB accounts."]
    pub gmb_accounts: ::std::option::Option<::std::boxed::Box<GmbAccounts>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#liasettingsCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "liaSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved or updated Lia settings."]
    pub lia_settings: ::std::option::Option<::std::boxed::Box<LiaSettings>>,
    #[serde(rename = "posDataProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of POS data providers."]
    pub pos_data_providers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosDataProviders>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsGetAccessibleGmbAccountsResponse {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the Merchant Center account."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gmbAccounts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of GMB accounts which are available to the merchant."]
    pub gmb_accounts:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmbAccountsGmbAccount>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsGetAccessibleGmbAccountsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsListPosDataProvidersResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsListPosDataProvidersResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posDataProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of POS data providers for each eligible country"]
    pub pos_data_providers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosDataProviders>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of LIA settings."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiaSettings>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsRequestGmbAccessResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsRequestGmbAccessResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsRequestInventoryVerificationResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsRequestInventoryVerificationResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsSetInventoryVerificationContactResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsSetInventoryVerificationContactResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LiasettingsSetPosDataProviderResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsSetPosDataProviderResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkService {
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Service provided to or by the linked account. Acceptable values are: - \"`shoppingActionsOrderManagement`\" - \"`shoppingActionsProductManagement`\" - \"`shoppingAdsProductManagement`\" "]
    pub service: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the link Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkedAccount {
    #[serde(rename = "linkedAccountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the linked account."]
    pub linked_account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of provided services."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LinkService>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `ListAccountLabels` method."]
pub struct ListAccountLabelsResponse {
    #[serde(rename = "accountLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The labels from the specified account."]
    pub account_labels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountLabel>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response for listing account return carriers."]
pub struct ListAccountReturnCarrierResponse {
    #[serde(rename = "accountReturnCarriers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all available account return carriers for the merchant."]
    pub account_return_carriers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountReturnCarrier>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListCollectionStatuses method."]
pub struct ListCollectionStatusesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collectionstatuses listed."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CollectionStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListCollections method."]
pub struct ListCollectionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The collections listed."]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Collection>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The response message for the `ListCsses` method"]
pub struct ListCssesResponse {
    #[serde(rename = "csses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CSS domains affiliated with the specified CSS group."]
    pub csses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Css>>>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `ListRegions` method."]
pub struct ListRegionsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The regions from the specified merchant."]
    pub regions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Region>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListRepricingProductReports method."]
pub struct ListRepricingProductReportsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token for retrieving the next page. Its absence means there is no subsequent page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repricingProductReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Periodic reports for the given Repricing product."]
    pub repricing_product_reports:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RepricingProductReport>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ListRepricingRuleReports method."]
pub struct ListRepricingRuleReportsResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token for retrieving the next page. Its absence means there is no subsequent page."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repricingRuleReports")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Daily reports for the given Repricing rule."]
    pub repricing_rule_reports:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RepricingRuleReport>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `ListRepricingRules` method."]
pub struct ListRepricingRulesResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repricingRules")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rules from the specified merchant."]
    pub repricing_rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RepricingRule>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the `ListReturnPolicyOnline` method."]
pub struct ListReturnPolicyOnlineResponse {
    #[serde(rename = "returnPolicies")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved return policies."]
    pub return_policies:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnPolicyOnline>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Local inventory resource. For accepted attribute values, see the local product inventory feed specification."]
pub struct LocalInventory {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Availability of the product. For accepted attribute values, see the local product inventory feed specification."]
    pub availability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "instoreProductLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "In-store product location."]
    pub instore_product_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#localInventory`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Supported pickup method for this offer. Unless the value is \"not supported\", this field must be submitted together with `pickupSla`. For accepted attribute values, see the local product inventory feed // specification."]
    pub pickup_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupSla")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Expected date that an order will be ready for pickup relative to the order date. Must be submitted together with `pickupMethod`. For accepted attribute values, see the local product inventory feed specification."]
    pub pickup_sla: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the product."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quantity of the product. Must be nonnegative."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "salePrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
    pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "salePriceEffectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates may be specified as 'null' if undecided."]
    pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Store code of this local inventory resource."]
    pub store_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalinventoryCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<LocalinventoryCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Batch entry encoding a single local inventory update request."]
pub struct LocalinventoryCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "localInventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Local inventory of the product."]
    pub local_inventory: ::std::option::Option<::std::boxed::Box<LocalInventory>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Method of the batch request entry. Acceptable values are: - \"`insert`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product for which to update local inventory."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocalinventoryCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<LocalinventoryCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#localinventoryCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Batch entry encoding a single local inventory update response."]
pub struct LocalinventoryCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#localinventoryCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocationIdSet {
    #[serde(rename = "locationIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A non-empty list of location IDs. They must all be of the same location type (e.g., state)."]
    pub location_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoyaltyPoints {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of loyalty points program. It is recommended to limit the name to 12 full-width characters or 24 Roman characters."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pointsValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retailer's loyalty points in absolute value."]
    pub points_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ratio")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ratio of a point when converted to currency. Google assumes currency based on Merchant Center settings. If ratio is left out, it defaults to 1.0."]
    pub ratio: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Order return. Production access (all methods) requires the order manager role. Sandbox access does not."]
pub struct MerchantOrderReturn {
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of creation of the return, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merchant defined order ID."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google order ID."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderReturnId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Order return ID generated by Google."]
    pub order_return_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Items of the return."]
    pub return_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MerchantOrderReturnItem>>>,
    #[serde(rename = "returnPricingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about shipping costs."]
    pub return_pricing_info: ::std::option::Option<::std::boxed::Box<ReturnPricingInfo>>,
    #[serde(rename = "returnShipments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipments of the return."]
    pub return_shipments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnShipment>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MerchantOrderReturnItem {
    #[serde(rename = "customerReturnReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason that the customer chooses to return an item."]
    pub customer_return_reason: ::std::option::Option<::std::boxed::Box<CustomerReturnReason>>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product level item ID. If the returned items are of the same product, they will have the same ID."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantRejectionReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason that the merchant chose to reject an item return."]
    pub merchant_rejection_reason:
        ::std::option::Option<::std::boxed::Box<MerchantRejectionReason>>,
    #[serde(rename = "merchantReturnReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason that merchant chooses to accept a return item."]
    pub merchant_return_reason: ::std::option::Option<::std::boxed::Box<RefundReason>>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product data from the time of the order placement."]
    pub product: ::std::option::Option<::std::boxed::Box<OrderLineItemProduct>>,
    #[serde(rename = "refundableAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum amount that can be refunded for this return item."]
    pub refundable_amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "returnItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unit level ID for the return item. Different units of the same product will have different IDs."]
    pub return_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnShipmentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of the return shipments that this return item belongs to."]
    pub return_shipment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the original shipment group. Provided for shipments with invoice support."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentUnitId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the shipment unit assigned by the merchant. Provided for shipments with invoice support."]
    pub shipment_unit_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the item. Acceptable values are: - \"`canceled`\" - \"`new`\" - \"`received`\" - \"`refunded`\" - \"`rejected`\" "]
    pub state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MerchantRejectionReason {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the reason."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code of the rejection reason."]
    pub reason_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Performance metrics. Values are only set for metrics requested explicitly in the request's search query."]
pub struct Metrics {
    #[serde(rename = "clicks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of clicks."]
    pub clicks: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ctr")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of clicks merchant's products receive (clicks) divided by the number of times the products are shown (impressions)."]
    pub ctr: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "impressions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of times merchant's products are shown."]
    pub impressions: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MinimumOrderValueTable {
    #[serde(rename = "storeCodeSetWithMovs")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub store_code_set_with_movs: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<MinimumOrderValueTableStoreCodeSetWithMov>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of store code sets sharing the same minimum order value. At least two sets are required and the last one must be empty, which signifies 'MOV for all other stores'. Each store code can only appear once across all the sets. All prices within a service must have the same currency."]
pub struct MinimumOrderValueTableStoreCodeSetWithMov {
    #[serde(rename = "storeCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of unique store codes or empty for the catch all."]
    pub store_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The minimum order value for the given stores."]
    pub value: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MonetaryAmount {
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pre-tax or post-tax price depends on the location of the order. - For countries (e.g. US) where price attribute excludes tax, this field corresponds to the pre-tax value. - For coutries (e.g. France) where price attribute includes tax, this field corresponds to the post-tax value ."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax value, present only for countries where price attribute excludes tax (e.g. US). No tax is referenced as 0 value with the corresponding `currency`."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Order. Production access (all methods) requires the order manager role. Sandbox access does not."]
pub struct Order {
    #[serde(rename = "acknowledged")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the order was acknowledged."]
    pub acknowledged: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of key-value pairs that are attached to a given order."]
    pub annotations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderOrderAnnotation>>>,
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billing address."]
    pub billing_address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the customer who placed the order."]
    pub customer: ::std::option::Option<::std::boxed::Box<OrderCustomer>>,
    #[serde(rename = "deliveryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delivery details for shipments of type `delivery`."]
    pub delivery_details: ::std::option::Option<::std::boxed::Box<OrderDeliveryDetails>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The REST ID of the order. Globally unique."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#order`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line items that are ordered."]
    pub line_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItem>>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merchant-provided ID of the order."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "netPriceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The net amount for the order (price part). For example, if an order was originally for $100 and a refund was issued for $20, the net amount will be $80."]
    pub net_price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "netTaxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The net amount for the order (tax part). Note that in certain cases due to taxable base adjustment `netTaxAmount` might not match to a sum of tax field across all lineItems and refunds."]
    pub net_tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "paymentStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the payment. Acceptable values are: - \"`paymentCaptured`\" - \"`paymentRejected`\" - \"`paymentSecured`\" - \"`pendingAuthorization`\" "]
    pub payment_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pickup details for shipments of type `pickup`."]
    pub pickup_details: ::std::option::Option<::std::boxed::Box<OrderPickupDetails>>,
    #[serde(rename = "placedDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date when the order was placed, in ISO 8601 format."]
    pub placed_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Promotions associated with the order. To determine which promotions apply to which products, check the `Promotions[].appliedItems[].lineItemId` field against the `LineItems[].id` field for each promotion. If a promotion is applied to more than 1 offerId, divide the discount value by the number of affected offers to determine how much discount to apply to each offerId. Examples: 1. To calculate price paid by the customer for a single line item including the discount: For each promotion, subtract the `LineItems[].adjustments[].priceAdjustment.value` amount from the `LineItems[].Price.value`. 2. To calculate price paid by the customer for a single line item including the discount in case of multiple quantity: For each promotion, divide the `LineItems[].adjustments[].priceAdjustment.value` by the quantity of products then subtract the resulting value from the `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPromotion>>>,
    #[serde(rename = "refunds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Refunds for the order."]
    pub refunds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderRefund>>>,
    #[serde(rename = "shipments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipments of the order."]
    pub shipments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderShipment>>>,
    #[serde(rename = "shippingCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total cost of shipping for all items."]
    pub shipping_cost: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shippingCostTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tax for the total shipping cost."]
    pub shipping_cost_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the order. Acceptable values are: - \"`canceled`\" - \"`delivered`\" - \"`inProgress`\" - \"`partiallyDelivered`\" - \"`partiallyReturned`\" - \"`partiallyShipped`\" - \"`pendingShipment`\" - \"`returned`\" - \"`shipped`\" "]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxCollector")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The party responsible for collecting and remitting taxes. Acceptable values are: - \"`marketplaceFacilitator`\" - \"`merchant`\" "]
    pub tax_collector: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderAddress {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CLDR country code (e.g. \"US\")."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Strings representing the lines of the printed label for mailing the order, for example: John Smith 1600 Amphitheatre Parkway Mountain View, CA, 94043 United States "]
    pub full_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isPostOfficeBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the address is a post office box."]
    pub is_post_office_box: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal Code or ZIP (e.g. \"94043\")."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the recipient."]
    pub recipient_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Street-level part of the address."]
    pub street_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderCancellation {
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actor that created the cancellation. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
    pub actor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the cancellation has been created, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity that was canceled."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the cancellation. Orders that are canceled with a noInventory reason will lead to the removal of the product from Buy on Google until you make an update to that product. This will not affect your Shopping ads. Acceptable values are: - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`customerCanceled`\" - \"`customerInitiatedCancel`\" - \"`customerSupportRequested`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceError`\" - \"`returnRefundAbuse`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderCustomer {
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full name of the customer."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "invoiceReceivingEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address for the merchant to send value-added tax or invoice documentation of the order. Only the last document sent is made available to the customer. For more information, see About automated VAT invoicing for Buy on Google."]
    pub invoice_receiving_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loyaltyInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Loyalty program information."]
    pub loyalty_info: ::std::option::Option<::std::boxed::Box<OrderCustomerLoyaltyInfo>>,
    #[serde(rename = "marketingRightsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer's marketing preferences. Contains the marketing opt-in information that is current at the time that the merchant call. User preference selections can change from one order to the next so preferences must be checked with every order."]
    pub marketing_rights_info:
        ::std::option::Option<::std::boxed::Box<OrderCustomerMarketingRightsInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderCustomerLoyaltyInfo {
    #[serde(rename = "loyaltyNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The loyalty card/membership number."]
    pub loyalty_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of card/membership holder, this field will be populated when"]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderCustomerMarketingRightsInfo {
    #[serde(rename = "explicitMarketingPreference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last known customer selection regarding marketing preferences. In certain cases this selection might not be known, so this field would be empty. If a customer selected `granted` in their most recent order, they can be subscribed to marketing emails. Customers who have chosen `denied` must not be subscribed, or must be unsubscribed if already opted-in. Acceptable values are: - \"`denied`\" - \"`granted`\" "]
    pub explicit_marketing_preference: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet."]
    pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketingEmailAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address that can be used for marketing purposes. The field may be empty even if `explicitMarketingPreference` is 'granted'. This happens when retrieving an old order from the customer who deleted their account."]
    pub marketing_email_address: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderDeliveryDetails {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delivery address"]
    pub address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number of the person receiving the delivery."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItem {
    #[serde(rename = "adjustments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price and tax adjustments applied on the line item."]
    pub adjustments:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItemAdjustment>>>,
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Annotations that are attached to the line item."]
    pub annotations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>>,
    #[serde(rename = "cancellations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cancellations of the line item."]
    pub cancellations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderCancellation>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total price for the line item. For example, if two items for $10 are purchased, the total price will be $20."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product data as seen by customer from the time of the order placement. Note that certain attributes values (e.g. title or gtin) might be reformatted and no longer match values submitted via product feed."]
    pub product: ::std::option::Option<::std::boxed::Box<OrderLineItemProduct>>,
    #[serde(rename = "quantityCanceled")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items canceled."]
    pub quantity_canceled: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityDelivered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items delivered."]
    pub quantity_delivered: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityOrdered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items ordered."]
    pub quantity_ordered: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityPending")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items pending."]
    pub quantity_pending: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityReadyForPickup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items ready for pickup."]
    pub quantity_ready_for_pickup: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityReturned")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items returned."]
    pub quantity_returned: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityShipped")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items shipped."]
    pub quantity_shipped: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "quantityUndeliverable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of items undeliverable."]
    pub quantity_undeliverable: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "returnInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the return policy for the line item."]
    pub return_info: ::std::option::Option<::std::boxed::Box<OrderLineItemReturnInfo>>,
    #[serde(rename = "returns")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returns of the line item."]
    pub returns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReturn>>>,
    #[serde(rename = "shippingDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the requested shipping for the line item."]
    pub shipping_details: ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetails>>,
    #[serde(rename = "tax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total tax amount for the line item. For example, if two items are purchased, and each have a cost tax of $2, the total tax amount will be $4."]
    pub tax: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemAdjustment {
    #[serde(rename = "priceAdjustment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adjustment for total price of the line item."]
    pub price_adjustment: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxAdjustment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Adjustment for total tax of the line item."]
    pub tax_adjustment: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of this adjustment. Acceptable values are: - \"`promotion`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemProduct {
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand of the item."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Condition or state of the item. Acceptable values are: - \"`new`\" - \"`refurbished`\" - \"`used`\" "]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fees")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Associated fees at order creation time."]
    pub fees: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItemProductFee>>>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number (GTIN) of the item."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The REST ID of the product."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of an image of the item."]
    pub image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shared identifier for all variants of the same product."]
    pub item_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Manufacturer Part Number (MPN) of the item."]
    pub mpn: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An identifier of the item."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shownImage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL to the cached image shown to the user when order was placed."]
    pub shown_image: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory // code of the target country of the product."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the product."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variantAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Variant attributes for the item. These are dimensions of the product, such as color, gender, material, pattern, and size. You can find a comprehensive list of variant attributes here."]
    pub variant_attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrderLineItemProductVariantAttribute>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemProductFee {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount of the fee."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the fee."]
    pub name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemProductVariantAttribute {
    #[serde(rename = "dimension")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimension of the variant."]
    pub dimension: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value for the dimension."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemReturnInfo {
    #[serde(rename = "daysToReturn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. How many days later the item can be returned."]
    pub days_to_return: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "isReturnable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Whether the item is returnable."]
    pub is_returnable: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "policyUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URL of the item return policy."]
    pub policy_url: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemShippingDetails {
    #[serde(rename = "deliverByDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The delivery by date, in ISO 8601 format."]
    pub deliver_by_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Details of the shipping method."]
    pub method: ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetailsMethod>>,
    #[serde(rename = "pickupPromiseInMinutes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The promised time in minutes in which the order will be ready for pickup. This only applies to buy-online-pickup-in-store same-day order."]
    pub pickup_promise_in_minutes: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "shipByDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ship by date, in ISO 8601 format."]
    pub ship_by_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of shipment. Indicates whether `deliveryDetails` or `pickupDetails` is applicable for this shipment. Acceptable values are: - \"`delivery`\" - \"`pickup`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItemShippingDetailsMethod {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier for the shipping. Optional. See `shipments[].carrier` for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxDaysInTransit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Maximum transit time."]
    pub max_days_in_transit: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "methodName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the shipping method."]
    pub method_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minDaysInTransit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Minimum transit time."]
    pub min_days_in_transit: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderMerchantProvidedAnnotation {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key for additional merchant provided (as key-value pairs) annotation about the line item."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for additional merchant provided (as key-value pairs) annotation about the line item."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderOrderAnnotation {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Key for additional google provided (as key-value pairs) annotation."]
    pub key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Value for additional google provided (as key-value pairs) annotation."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderPickupDetails {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Address of the pickup location where the shipment should be sent. Note that `recipientName` in the address is the name of the business at the pickup location."]
    pub address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
    #[serde(rename = "collectors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Collectors authorized to pick up shipment from the pickup location."]
    pub collectors:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPickupDetailsCollector>>>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the pickup location."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pickup type of this order. Acceptable values are: - \"`merchantStore`\" - \"`merchantStoreCurbside`\" - \"`merchantStoreLocker`\" - \"`thirdPartyPickupPoint`\" - \"`thirdPartyLocker`\" "]
    pub pickup_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderPickupDetailsCollector {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the person picking up the shipment."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Phone number of the person picking up the shipment."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderPromotion {
    #[serde(rename = "applicableItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Items that this promotion may be applied to. If empty, there are no restrictions on applicable items and quantity. This field will also be empty for shipping promotions because shipping is not tied to any specific item."]
    pub applicable_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPromotionItem>>>,
    #[serde(rename = "appliedItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Items that this promotion have been applied to. Do not provide for `orders.createtestorder`. This field will be empty for shipping promotions because shipping is not tied to any specific item."]
    pub applied_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPromotionItem>>>,
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Promotion end time in ISO 8601 format. Date, time, and offset required, e.g., \"2020-01-02T09:00:00+01:00\" or \"2020-01-02T09:00:00Z\"."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "funder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The party funding the promotion. Only `merchant` is supported for `orders.createtestorder`. Acceptable values are: - \"`google`\" - \"`merchant`\" "]
    pub funder: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantPromotionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. This field is used to identify promotions within merchants' own systems."]
    pub merchant_promotion_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priceValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated discount applied to price. Amount is pre-tax or post-tax depending on location of order."]
    pub price_value: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shortTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short title of the promotion to be shown on the checkout page. Do not provide for `orders.createtestorder`."]
    pub short_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Promotion start time in ISO 8601 format. Date, time, and offset required, e.g., \"2020-01-02T09:00:00+01:00\" or \"2020-01-02T09:00:00Z\"."]
    pub start_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subtype")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The category of the promotion. Only `moneyOff` is supported for `orders.createtestorder`. Acceptable values are: - \"`buyMGetMoneyOff`\" - \"`buyMGetNMoneyOff`\" - \"`buyMGetNPercentOff`\" - \"`buyMGetPercentOff`\" - \"`freeGift`\" - \"`freeGiftWithItemId`\" - \"`freeGiftWithValue`\" - \"`freeShippingOvernight`\" - \"`freeShippingStandard`\" - \"`freeShippingTwoDay`\" - \"`moneyOff`\" - \"`percentOff`\" - \"`rewardPoints`\" - \"`salePrice`\" "]
    pub subtype: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Estimated discount applied to tax (if allowed by law). Do not provide for `orders.createtestorder`."]
    pub tax_value: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the promotion."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The scope of the promotion. Only `product` is supported for `orders.createtestorder`. Acceptable values are: - \"`product`\" - \"`shipping`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderPromotionItem {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The line item ID of a product. Do not provide for `orders.createtestorder`."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Offer ID of a product. Only for `orders.createtestorder`."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "`orders.createtestorder`."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the associated product. Do not provide for `orders.createtestorder`."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderRefund {
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actor that created the refund. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
    pub actor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item has been created, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Order disbursement. All methods require the payment analyst role."]
pub struct OrderReportDisbursement {
    #[serde(rename = "disbursementAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disbursement amount."]
    pub disbursement_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "disbursementCreationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disbursement date, in ISO 8601 format."]
    pub disbursement_creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disbursementDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date the disbursement was initiated, in ISO 8601 format."]
    pub disbursement_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disbursementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the disbursement."]
    pub disbursement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderReportTransaction {
    #[serde(rename = "disbursementAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The disbursement amount."]
    pub disbursement_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "disbursementCreationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date the disbursement was created, in ISO 8601 format."]
    pub disbursement_creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disbursementDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date the disbursement was initiated, in ISO 8601 format."]
    pub disbursement_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disbursementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the disbursement."]
    pub disbursement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merchant-provided ID of the order."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the order."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total amount for the items."]
    pub product_amount: ::std::option::Option<::std::boxed::Box<ProductAmount>>,
    #[serde(rename = "transactionDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of the transaction, in ISO 8601 format."]
    pub transaction_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderReturn {
    #[serde(rename = "actor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The actor that created the refund. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
    pub actor: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item has been created, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quantity that is returned."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderShipment {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier handling the shipment. For supported carriers, Google includes the carrier name and tracking URL in emails to customers. For select supported carriers, Google also automatically updates the shipment status based on the provided shipment ID. *Note:* You can also use unsupported carriers, but emails to customers will not include the carrier name or tracking URL, and there will be no automatic order status updates. Supported carriers for US are: - \"`ups`\" (United Parcel Service) *automatic status updates* - \"`usps`\" (United States Postal Service) *automatic status updates* - \"`fedex`\" (FedEx) *automatic status updates * - \"`dhl`\" (DHL eCommerce) *automatic status updates* (US only) - \"`ontrac`\" (OnTrac) *automatic status updates * - \"`dhl express`\" (DHL Express) - \"`deliv`\" (Deliv) - \"`dynamex`\" (TForce) - \"`lasership`\" (LaserShip) - \"`mpx`\" (Military Parcel Xpress) - \"`uds`\" (United Delivery Service) - \"`efw`\" (Estes Forwarding Worldwide) - \"`jd logistics`\" (JD Logistics) - \"`yunexpress`\" (YunExpress) - \"`china post`\" (China Post) - \"`china ems`\" (China Post Express Mail Service) - \"`singapore post`\" (Singapore Post) - \"`pos malaysia`\" (Pos Malaysia) - \"`postnl`\" (PostNL) - \"`ptt`\" (PTT Turkish Post) - \"`eub`\" (ePacket) - \"`chukou1`\" (Chukou1 Logistics) - \"`bestex`\" (Best Express) - \"`canada post`\" (Canada Post) - \"`purolator`\" (Purolator) - \"`canpar`\" (Canpar) - \"`india post`\" (India Post) - \"`blue dart`\" (Blue Dart) - \"`delhivery`\" (Delhivery) - \"`dtdc`\" (DTDC) - \"`tpc india`\" (TPC India) Supported carriers for FR are: - \"`la poste`\" (La Poste) *automatic status updates * - \"`colissimo`\" (Colissimo by La Poste) *automatic status updates* - \"`ups`\" (United Parcel Service) *automatic status updates * - \"`chronopost`\" (Chronopost by La Poste) - \"`gls`\" (General Logistics Systems France) - \"`dpd`\" (DPD Group by GeoPost) - \"`bpost`\" (Belgian Post Group) - \"`colis prive`\" (Colis Privé) - \"`boxtal`\" (Boxtal) - \"`geodis`\" (GEODIS) - \"`tnt`\" (TNT) - \"`db schenker`\" (DB Schenker) - \"`aramex`\" (Aramex) "]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been created, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Present only if `status` is `delivered`"]
    pub delivery_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the shipment."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The line items that are shipped."]
    pub line_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>>,
    #[serde(rename = "scheduledDeliveryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delivery details of the shipment if scheduling is needed."]
    pub scheduled_delivery_details:
        ::std::option::Option<::std::boxed::Box<OrderShipmentScheduledDeliveryDetails>>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipment group ID of the shipment. This is set in shiplineitems request."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the shipment. Acceptable values are: - \"`delivered`\" - \"`readyForPickup`\" - \"`shipped`\" - \"`undeliverable`\" "]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking ID for the shipment."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderShipmentLineItemShipment {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item that is shipped. This value is assigned by Google when an order is created. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to ship. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity that is shipped."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderShipmentScheduledDeliveryDetails {
    #[serde(rename = "carrierPhoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number of the carrier fulfilling the delivery. The phone number is formatted as the international notation in ITU-T Recommendation E.123 (e.g., \"+41 44 668 1800\")."]
    pub carrier_phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date a shipment is scheduled for delivery, in ISO 8601 format."]
    pub scheduled_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a merchant trade from which signals are extracted, e.g. shipping."]
pub struct OrderTrackingSignal {
    #[serde(rename = "customerShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipping fee of the order; this value should be set to zero in the case of free shipping."]
    pub customer_shipping_fee: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "deliveryPostalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The delivery postal code, as a continuous string without spaces or dashes, e.g. \"95016\"."]
    pub delivery_postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryRegionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping destination."]
    pub delivery_region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Information about line items in the order."]
    pub line_items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrderTrackingSignalLineItemDetails>>,
    >,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google merchant ID of this order tracking signal. This value is optional. If left unset, the caller's merchant ID is used. You must request access in order to provide data on behalf of another merchant. For more information, see [Submitting Order Tracking Signals](/shopping-content/guides/order-tracking-signals)."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderCreatedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The time when the order was created on the merchant side. Include the year and timezone string, if available."]
    pub order_created_time: ::std::option::Option<::std::boxed::Box<DateTime>>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the order on the merchant side."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderTrackingSignalId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. The ID that uniquely identifies this order tracking signal."]
    pub order_tracking_signal_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentLineItemMapping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The mapping of the line items to the shipment information."]
    pub shipment_line_item_mapping: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrderTrackingSignalShipmentLineItemMapping>>,
    >,
    #[serde(rename = "shippingInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipping information for the order."]
    pub shipping_info:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderTrackingSignalShippingInfo>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The line items of the order."]
pub struct OrderTrackingSignalLineItemDetails {
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID for this line item."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The manufacturer part number."]
    pub mpn: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The Content API REST ID of the product, in the form channel:contentLanguage:targetCountry:offerId."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The quantity of the line item in the order."]
    pub quantity: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents how many items are in the shipment for the given shipment_id and line_item_id."]
pub struct OrderTrackingSignalShipmentLineItemMapping {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The line item ID."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The line item quantity in the shipment."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The shipment ID."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The shipping information for the order."]
pub struct OrderTrackingSignalShippingInfo {
    #[serde(rename = "actualDeliveryTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the shipment was actually delivered. Include the year and timezone string, if available. This field is required, if one of the following fields is absent: tracking_id or carrier_name."]
    pub actual_delivery_time: ::std::option::Option<::std::boxed::Box<DateTime>>,
    #[serde(rename = "carrierName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the shipping carrier for the delivery. This field is required if one of the following fields is absent: earliest_delivery_promise_time, latest_delivery_promise_time, and actual_delivery_time."]
    pub carrier_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "carrierServiceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The service type for fulfillment, e.g., GROUND, FIRST_CLASS, etc."]
    pub carrier_service_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "earliestDeliveryPromiseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The earliest delivery promised time. Include the year and timezone string, if available. This field is required, if one of the following fields is absent: tracking_id or carrier_name."]
    pub earliest_delivery_promise_time: ::std::option::Option<::std::boxed::Box<DateTime>>,
    #[serde(rename = "latestDeliveryPromiseTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The latest delivery promised time. Include the year and timezone string, if available. This field is required, if one of the following fields is absent: tracking_id or carrier_name."]
    pub latest_delivery_promise_time: ::std::option::Option<::std::boxed::Box<DateTime>>,
    #[serde(rename = "originPostalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The origin postal code, as a continuous string without spaces or dashes, e.g. \"95016\"."]
    pub origin_postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "originRegionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The [CLDR territory code] (http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) for the shipping origin."]
    pub origin_region_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The shipment ID."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shippedTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time when the shipment was shipped. Include the year and timezone string, if available."]
    pub shipped_time: ::std::option::Option<::std::boxed::Box<DateTime>>,
    #[serde(rename = "shippingStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the shipment."]
    pub shipping_status: ::std::option::Option<OrderTrackingSignalShippingInfoShippingStatusEnum>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking ID of the shipment. This field is required if one of the following fields is absent: earliest_delivery_promise_time, latest_delivery_promise_time, and actual_delivery_time."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The status of the shipment."]
pub enum OrderTrackingSignalShippingInfoShippingStatusEnum {
    #[serde(rename = "SHIPPING_STATE_UNSPECIFIED")]
    #[doc = "The shipping status is not known to merchant."]
    ShippingStateUnspecified,
    #[serde(rename = "SHIPPED")]
    #[doc = "All items are shipped."]
    Shipped,
    #[serde(rename = "DELIVERED")]
    #[doc = "The shipment is already delivered."]
    Delivered,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCreateChargeInvoiceRequest {
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the invoice."]
    pub invoice_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "invoiceSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Invoice summary."]
    pub invoice_summary: ::std::option::Option<::std::boxed::Box<InvoiceSummary>>,
    #[serde(rename = "lineItemInvoices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Invoice details per line item."]
    pub line_item_invoices:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentInvoiceLineItemInvoice>>>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the operation, unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCreateChargeInvoiceResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderinvoicesCreateChargeInvoiceResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCreateRefundInvoiceRequest {
    #[serde(rename = "invoiceId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the invoice."]
    pub invoice_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the operation, unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "refundOnlyOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option to create a refund-only invoice. Exactly one of `refundOnlyOption` or `returnOption` must be provided."]
    pub refund_only_option: ::std::option::Option<
        ::std::boxed::Box<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption>,
    >,
    #[serde(rename = "returnOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of `refundOnlyOption` or `returnOption` must be provided."]
    pub return_option: ::std::option::Option<
        ::std::boxed::Box<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption>,
    >,
    #[serde(rename = "shipmentInvoices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Invoice details for different shipment groups."]
    pub shipment_invoices:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentInvoice>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCreateRefundInvoiceResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderinvoicesCreateRefundInvoiceResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional description of the refund reason."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional description of the return reason."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreportsListDisbursementsResponse {
    #[serde(rename = "disbursements")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of disbursements."]
    pub disbursements:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReportDisbursement>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreportsListDisbursementsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of disbursements."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreportsListTransactionsResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreportsListTransactionsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of transactions."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transactions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of transactions."]
    pub transactions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReportTransaction>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsAcknowledgeRequest {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the operation, unique across all operations for a given order return."]
    pub operation_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsAcknowledgeResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreturnsAcknowledgeResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsCreateOrderReturnRequest {
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of line items to return."]
    pub line_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderreturnsLineItem>>>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the order."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnMethodType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The way of the package being returned."]
    pub return_method_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsCreateOrderReturnResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreturnsCreateOrderReturnResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderReturn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Created order return."]
    pub order_return: ::std::option::Option<::std::boxed::Box<MerchantOrderReturn>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsLineItem {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item. This value is assigned by Google when an order is created."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of this line item."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreturnsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of returns."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MerchantOrderReturn>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsPartialRefund {
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pre-tax or post-tax amount to be refunded, depending on the location of the order."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount to be refunded. Note: This has different meaning depending on the location of the order."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsProcessRequest {
    #[serde(rename = "fullChargeReturnShippingCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Option to charge the customer return shipping cost."]
    pub full_charge_return_shipping_cost: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The ID of the operation, unique across all operations for a given order return."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "refundShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Refunds for original shipping fee."]
    pub refund_shipping_fee: ::std::option::Option<::std::boxed::Box<OrderreturnsRefundOperation>>,
    #[serde(rename = "returnItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of items to return."]
    pub return_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderreturnsReturnItem>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsProcessResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreturnsProcessResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsRefundOperation {
    #[serde(rename = "fullRefund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the item will be fully refunded. Allowed only when payment_type is FOP. Merchant can choose this refund option to indicate the full remaining amount of corresponding object to be refunded to the customer via FOP."]
    pub full_refund: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "partialRefund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If this is set, the item will be partially refunded. Merchant can choose this refund option to specify the customized amount that to be refunded to the customer."]
    pub partial_refund: ::std::option::Option<::std::boxed::Box<OrderreturnsPartialRefund>>,
    #[serde(rename = "paymentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The payment way of issuing refund. Default value is ORIGINAL_FOP if not set."]
    pub payment_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnRefundReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code of the refund reason."]
    pub return_refund_reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsRejectOperation {
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the return."]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderreturnsReturnItem {
    #[serde(rename = "refund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Refunds the item."]
    pub refund: ::std::option::Option<::std::boxed::Box<OrderreturnsRefundOperation>>,
    #[serde(rename = "reject")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rejects the item."]
    pub reject: ::std::option::Option<::std::boxed::Box<OrderreturnsRejectOperation>>,
    #[serde(rename = "returnItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Unit level ID for the return item. Different units of the same product will have different IDs."]
    pub return_item_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersAcknowledgeRequest {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersAcknowledgeResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersAcknowledgeResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersAdvanceTestOrderResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersAdvanceTestOrderResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelLineItemRequest {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to cancel. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity to cancel."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelLineItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelLineItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelRequest {
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelTestOrderByCustomerRequest {
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the cancellation. Acceptable values are: - \"`changedMind`\" - \"`orderedWrongItem`\" - \"`other`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCancelTestOrderByCustomerResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelTestOrderByCustomerResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCreateTestOrderRequest {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via `template_name`, or the addresses of orders created via `test_order`. Acceptable values are: - \"`US`\" - \"`FR`\" Defaults to `US`."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "templateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test order template to use. Specify as an alternative to `testOrder` as a shortcut for retrieving a template and then creating an order using that template. Acceptable values are: - \"`template1`\" - \"`template1a`\" - \"`template1b`\" - \"`template2`\" - \"`template3`\" "]
    pub template_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "testOrder")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The test order to create."]
    pub test_order: ::std::option::Option<::std::boxed::Box<TestOrder>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCreateTestOrderResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCreateTestOrderResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the newly created test order."]
    pub order_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCreateTestReturnRequest {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Returned items."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntryCreateTestReturnReturnItem>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCreateTestReturnResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCreateTestReturnResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the newly created test order return."]
    pub return_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Quantity that is returned."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryRefundItemItem {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The total amount that is refunded. (e.g. refunding $5 each for 2 products should be done by setting quantity to 2 and amount to 10$) In case of multiple refunds, this should be the amount you currently want to refund to the customer."]
    pub amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "fullRefund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the full item will be refunded. If this is true, amount should not be provided and will be ignored."]
    pub full_refund: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of products that are refunded."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryRefundItemShipping {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. If this is not the first refund for the shipment, this should be the newly refunded amount."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "fullRefund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set to true, all shipping costs for the order will be refunded. If this is true, amount should not be provided and will be ignored. If set to false, submit the amount of the partial shipping refund, excluding the shipping tax. The shipping tax is calculated and handled on Google's side."]
    pub full_refund: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The ID of the shipment. This is assigned by the merchant and is unique to each shipment."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking ID for the shipment."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "ScheduledDeliveryDetails used to update the scheduled delivery order."]
pub struct OrdersCustomBatchRequestEntryUpdateShipmentScheduledDeliveryDetails {
    #[serde(rename = "carrierPhoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number of the carrier fulfilling the delivery. The phone number should be formatted as the international notation in"]
    pub carrier_phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduledDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date a shipment is scheduled for delivery, in ISO 8601 format."]
    pub scheduled_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersGetByMerchantOrderIdResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersGetByMerchantOrderIdResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested order."]
    pub order: ::std::option::Option<::std::boxed::Box<Order>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersGetTestOrderTemplateResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersGetTestOrderTemplateResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "template")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested test order template."]
    pub template: ::std::option::Option<::std::boxed::Box<TestOrder>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersInStoreRefundLineItemRequest {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount to be refunded. This may be pre-tax or post-tax depending on the location of the order. Required."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity to return and refund."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of tax to be refunded. Required."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersInStoreRefundLineItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersInStoreRefundLineItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of orders."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Order>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRefundItemRequest {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The items that are refunded. Either Item or Shipping must be provided in the request."]
    pub items: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntryRefundItemItem>>,
    >,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the refund. Acceptable values are: - \"`shippingCostAdjustment`\" - \"`priceAdjustment`\" - \"`taxAdjustment`\" - \"`feeAdjustment`\" - \"`courtesyAdjustment`\" - \"`adjustment`\" - \"`customerCancelled`\" - \"`noInventory`\" - \"`productNotAsDescribed`\" - \"`undeliverableShippingAddress`\" - \"`wrongProductShipped`\" - \"`lateShipmentCredit`\" - \"`deliveredLateByCarrier`\" - \"`productArrivedDamaged`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The refund on shipping. Optional, but either Item or Shipping must be provided in the request."]
    pub shipping:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryRefundItemShipping>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRefundItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRefundItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRefundOrderRequest {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. If this is not the first refund for the order, this should be the newly refunded amount."]
    pub amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "fullRefund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, the full order will be refunded, including shipping. If this is true, amount should not be provided and will be ignored."]
    pub full_refund: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the refund. Acceptable values are: - \"`courtesyAdjustment`\" - \"`other`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRefundOrderResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRefundOrderResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRejectReturnLineItemRequest {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity to return and refund."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the return. Acceptable values are: - \"`damagedOrUsed`\" - \"`missingComponent`\" - \"`notEligible`\" - \"`other`\" - \"`outOfReturnWindow`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRejectReturnLineItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRejectReturnLineItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersReturnRefundLineItemRequest {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount to be refunded. This may be pre-tax or post-tax depending on the location of the order. If omitted, refundless return is assumed."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity to return and refund. Quantity is required."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount of tax to be refunded. Optional, but if filled, then priceAmount must be set. Calculated automatically if not provided."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersReturnRefundLineItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersReturnRefundLineItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersSetLineItemMetadataRequest {
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersSetLineItemMetadataResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersSetLineItemMetadataResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersShipLineItemsRequest {
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line items to ship."]
    pub line_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the shipment group. Required for orders that use the orderinvoices service."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentInfos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs)."]
    pub shipment_infos: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersShipLineItemsResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersShipLineItemsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateLineItemShippingDetailsRequest {
    #[serde(rename = "deliverByDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
    pub deliver_by_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipByDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
    pub ship_by_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateLineItemShippingDetailsResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateLineItemShippingDetailsResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateMerchantOrderIdRequest {
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The merchant order id to be assigned to the order. Must be unique per merchant."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateMerchantOrderIdResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateMerchantOrderIdResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateShipmentRequest {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`."]
    pub delivery_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastPickupDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date after which the pickup will expire, in ISO 8601 format. Required only when order is buy-online-pickup-in-store(BOPIS) and `status` is `ready for pickup`."]
    pub last_pickup_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "readyPickupDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been ready for pickup, in ISO 8601 format. Optional and can be provided only if `status` is `ready for pickup`."]
    pub ready_pickup_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "scheduledDeliveryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Delivery details of the shipment if scheduling is needed."]
    pub scheduled_delivery_details: ::std::option::Option<
        ::std::boxed::Box<OrdersCustomBatchRequestEntryUpdateShipmentScheduledDeliveryDetails>,
    >,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the shipment."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "New status for the shipment. Not updated if missing. Acceptable values are: - \"`delivered`\" - \"`undeliverable`\" - \"`readyForPickup`\" "]
    pub status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking ID for the shipment. Not updated if missing."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "undeliveredDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been undeliverable, in ISO 8601 format. Optional and can be provided only if `status` is `undeliverable`."]
    pub undelivered_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersUpdateShipmentResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateShipmentResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PickupCarrierService {
    #[serde(rename = "carrierName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the pickup carrier (e.g., `\"UPS\"`). Required."]
    pub carrier_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the pickup service (e.g., `\"Access point\"`). Required."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PickupServicesPickupService {
    #[serde(rename = "carrierName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the carrier (e.g., `\"UPS\"`). Always present."]
    pub carrier_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR country code of the carrier (e.g., \"US\"). Always present."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the pickup service (e.g., `\"Access point\"`). Always present."]
    pub service_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The inventory to submit. This should be set only if the method is `inventory`."]
    pub inventory: ::std::option::Option<::std::boxed::Box<PosInventory>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the POS data provider."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" - \"`inventory`\" - \"`sale`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sale information to submit. This should be set only if the method is `sale`."]
    pub sale: ::std::option::Option<::std::boxed::Box<PosSale>>,
    #[serde(rename = "store")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The store information to submit. This should be set only if the method is `insert`."]
    pub store: ::std::option::Option<::std::boxed::Box<PosStore>>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The store code. This should be set only if the method is `delete` or `get`."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetMerchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account for which to get/submit data."]
    pub target_merchant_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosCustomBatchResponseEntry>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry to which this entry responds."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if, and only if, the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated inventory information."]
    pub inventory: ::std::option::Option<::std::boxed::Box<PosInventory>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sale")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The updated sale information."]
    pub sale: ::std::option::Option<::std::boxed::Box<PosSale>>,
    #[serde(rename = "store")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved or updated store information."]
    pub store: ::std::option::Option<::std::boxed::Box<PosStore>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosDataProviders {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Country code."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "posDataProviders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of POS data providers."]
    pub pos_data_providers:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosDataProvidersPosDataProvider>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosDataProvidersPosDataProvider {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of Pos data Provider."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full name of this POS data Provider."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "providerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account."]
    pub provider_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The absolute quantity of an item available at the given store."]
pub struct PosInventory {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posInventory`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The current price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The available quantity of the item."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosInventoryRequest {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The current price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The available quantity of the item."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosInventoryResponse {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posInventoryResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The current price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The available quantity of the item."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosStore>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The change of the available quantity of an item at the given store."]
pub struct PosSale {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posSale`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "saleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID to group items from the same sale event."]
    pub sale_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosSaleRequest {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "saleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID to group items from the same sale event."]
    pub sale_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PosSaleResponse {
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item."]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posSaleResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
    pub quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "saleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A unique ID to group items from the same sale event."]
    pub sale_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
    pub store_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Store resource."]
pub struct PosStore {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posStore`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The street address of the store."]
    pub store_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A store identifier that is unique for the given merchant."]
    pub store_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostalCodeGroup {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory code of the country the postal code group applies to. Required."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the postal code group, referred to in headers. Required."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCodeRanges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A range of postal codes. Required."]
    pub postal_code_ranges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCodeRange>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostalCodeRange {
    #[serde(rename = "postalCodeRangeBegin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A postal code or a pattern of the form `prefix*` denoting the inclusive lower bound of the range defining the area. Examples values: `\"94108\"`, `\"9410*\"`, `\"9*\"`. Required."]
    pub postal_code_range_begin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCodeRangeEnd")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A postal code or a pattern of the form `prefix*` denoting the inclusive upper bound of the range defining the area. It must have the same length as `postalCodeRangeBegin`: if `postalCodeRangeBegin` is a postal code then `postalCodeRangeEnd` must be a postal code too; if `postalCodeRangeBegin` is a pattern then `postalCodeRangeEnd` must be a pattern with the same prefix length. Optional: if not set, then the area is defined as being all the postal codes matching `postalCodeRangeBegin`."]
    pub postal_code_range_end: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Price {
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency of the price."]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price represented as a number."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The price represented as a number and currency."]
pub struct PriceAmount {
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The currency of the price."]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price represented as a number."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " Required product attributes are primarily defined by the products data specification. See the Products Data Specification Help Center article for information. Some attributes are country-specific, so make sure you select the appropriate country in the drop-down selector at the top of the page. Product data. After inserting, updating, or deleting a product, it may take several minutes before changes take effect."]
pub struct Product {
    #[serde(rename = "additionalImageLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional URLs of images of the item."]
    pub additional_image_links: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "adsGrouping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise."]
    pub ads_grouping: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adsLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Similar to ads_grouping, but only works on CPC."]
    pub ads_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "adsRedirect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allows advertisers to override the item URL when the product is shown within the context of Product Ads."]
    pub ads_redirect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Should be set to true if the item is targeted towards adults."]
    pub adult: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "ageGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target age group of the item."]
    pub age_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Availability status of the item."]
    pub availability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "availabilityDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The day a pre-ordered product becomes available for delivery, in ISO 8601 format."]
    pub availability_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand of the item."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "canonicalLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for the canonical version of your item's landing page."]
    pub canonical_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The item's channel (online or local). Acceptable values are: - \"`local`\" - \"`online`\" "]
    pub channel: ::std::option::Option<::std::string::String>,
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Color of the item."]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Condition or state of the item."]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "costOfGoodsSold")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cost of goods sold. Used for gross profit reporting."]
    pub cost_of_goods_sold: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "customAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ \"name\": \"size type\", \"value\": \"regular\" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions)."]
    pub custom_attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
    #[serde(rename = "customLabel0")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign."]
    pub custom_label0: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign."]
    pub custom_label1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign."]
    pub custom_label2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign."]
    pub custom_label3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel4")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 4 for custom grouping of items in a Shopping campaign."]
    pub custom_label4: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the item."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayAdsId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An identifier for an item for dynamic remarketing campaigns."]
    pub display_ads_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayAdsLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL directly to your item's landing page for dynamic remarketing campaigns."]
    pub display_ads_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayAdsSimilarIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertiser-specified recommendations."]
    pub display_ads_similar_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "displayAdsTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of an item for dynamic remarketing campaigns."]
    pub display_ads_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayAdsValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Offer margin for dynamic remarketing campaigns."]
    pub display_ads_value: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "energyEfficiencyClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU."]
    pub energy_efficiency_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "excludedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of destinations to exclude for this target (corresponds to unchecked check boxes in Merchant Center)."]
    pub excluded_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future."]
    pub expiration_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target gender of the item."]
    pub gender: ::std::option::Option<::std::string::String>,
    #[serde(rename = "googleProductCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API."]
    pub google_product_category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number (GTIN) of the item."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "identifierExists")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada."]
    pub identifier_exists: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "imageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL of an image of the item."]
    pub image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "includedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of destinations to include for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`."]
    pub included_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "installment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number and amount of installments to pay for an item."]
    pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
    #[serde(rename = "isBundle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price."]
    pub is_bundle: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "itemGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shared identifier for all variants of the same product."]
    pub item_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#product`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL directly linking to your item's page on your website."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loyaltyPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
    pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
    #[serde(rename = "material")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The material of which the item is made."]
    pub material: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxEnergyEfficiencyClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU."]
    pub max_energy_efficiency_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxHandlingTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximal product handling time (in business days)."]
    pub max_handling_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minEnergyEfficiencyClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU."]
    pub min_energy_efficiency_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minHandlingTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimal product handling time (in business days)."]
    pub min_handling_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mobileLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "URL for the mobile-optimized version of your item's landing page."]
    pub mobile_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Manufacturer Part Number (MPN) of the item."]
    pub mpn: ::std::option::Option<::std::string::String>,
    #[serde(rename = "multipack")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of identical products in a merchant-defined multipack."]
    pub multipack: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item's pattern (e.g. polka dots)."]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "productDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Technical specification or additional product details."]
    pub product_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductProductDetail>>>,
    #[serde(rename = "productHighlights")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Bullet points describing the most relevant highlights of a product."]
    pub product_highlights: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "productTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Categories of the item (formatted as in products data specification)."]
    pub product_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "promotionIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of a promotion."]
    pub promotion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "salePrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Advertised sale price of the item."]
    pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "salePriceEffectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date range during which the item is on sale (see products data specification )."]
    pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellOnGoogleQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
    pub sell_on_google_quantity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipping rules."]
    pub shipping: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductShipping>>>,
    #[serde(rename = "shippingHeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Height of the item for shipping."]
    pub shipping_height: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
    #[serde(rename = "shippingLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipping label of the product, used to group product in account-level shipping rules."]
    pub shipping_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shippingLength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Length of the item for shipping."]
    pub shipping_length: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
    #[serde(rename = "shippingWeight")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Weight of the item for shipping."]
    pub shipping_weight: ::std::option::Option<::std::boxed::Box<ProductShippingWeight>>,
    #[serde(rename = "shippingWidth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Width of the item for shipping."]
    pub shipping_width: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
    #[serde(rename = "shoppingAdsExcludedCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of country codes (ISO 3166-1 alpha-2) to exclude the offer from Shopping Ads destination. Countries from this list are removed from countries configured in MC feed settings."]
    pub shopping_ads_excluded_countries:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "sizeSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "System in which the size is specified. Recommended for apparel items."]
    pub size_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cut of the item. Recommended for apparel items."]
    pub size_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition)."]
    pub sizes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of the offer, i.e., how the offer was created. Acceptable values are: - \"`api`\" - \"`crawl`\" - \"`feed`\" "]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subscriptionCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of periods (months or years) and amount of payment per period for an item with an associated subscription contract."]
    pub subscription_cost: ::std::option::Option<::std::boxed::Box<ProductSubscriptionCost>>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tax category of the product, used to configure detailed tax nexus in account-level tax settings."]
    pub tax_category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax information."]
    pub taxes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductTax>>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the item."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transitTimeLabel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The transit time label of the product, used to group product in account-level transit time tables."]
    pub transit_time_label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unitPricingBaseMeasure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The preference of the denominator of the unit price."]
    pub unit_pricing_base_measure:
        ::std::option::Option<::std::boxed::Box<ProductUnitPricingBaseMeasure>>,
    #[serde(rename = "unitPricingMeasure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The measure and dimension of an item."]
    pub unit_pricing_measure: ::std::option::Option<::std::boxed::Box<ProductUnitPricingMeasure>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAmount {
    #[serde(rename = "priceAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The pre-tax or post-tax price depending on the location of the order."]
    pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "remittedTaxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Remitted tax value."]
    pub remitted_tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax value."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductProductDetail {
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the product detail."]
    pub attribute_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the product detail."]
    pub attribute_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sectionName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The section header used to group a set of product details."]
    pub section_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductShipping {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory code of the country to which an item will ship."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationGroupName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The location where the shipping is applicable, represented by a location group name."]
    pub location_group_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of a location that the shipping rate applies to as defined in the AdWords API."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The postal code range that the shipping rate applies to, represented by a postal code, a postal code prefix followed by a * wildcard, a range between two postal codes or two postal code prefixes of equal length."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed shipping price, represented as a number."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographic region to which a shipping rate applies."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A free-form description of the service class or delivery speed."]
    pub service: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductShippingDimension {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of value."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The dimension of the product used to calculate the shipping cost of the item."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductShippingWeight {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of value."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The weight of the product used to calculate the shipping cost of the item."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The status of a product, i.e., information about a product computed asynchronously."]
pub struct ProductStatus {
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item has been created, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationStatuses")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The intended destinations for the product."]
    pub destination_statuses:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatusDestinationStatus>>>,
    #[serde(rename = "googleExpirationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item expires in Google Shopping, in ISO 8601 format."]
    pub google_expiration_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemLevelIssues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of all issues associated with the product."]
    pub item_level_issues:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatusItemLevelIssue>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productStatus`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdateDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item has been last updated, in ISO 8601 format."]
    pub last_update_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The link to the product."]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product for which status is reported."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title of the product."]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductStatusDestinationStatus {
    #[serde(rename = "approvedCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of country codes (ISO 3166-1 alpha-2) where the offer is approved."]
    pub approved_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the destination"]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "disapprovedCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of country codes (ISO 3166-1 alpha-2) where the offer is disapproved."]
    pub disapproved_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "pendingCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of country codes (ISO 3166-1 alpha-2) where the offer is pending approval."]
    pub pending_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Destination approval status in `targetCountry` of the offer."]
    pub status: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductStatusItemLevelIssue {
    #[serde(rename = "applicableCountries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of country codes (ISO 3166-1 alpha-2) where issue applies to the offer."]
    pub applicable_countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The attribute's name, if the issue is caused by a single attribute."]
    pub attribute_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The error code of the issue."]
    pub code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A short issue description in English."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination the issue applies to."]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A detailed issue description in English."]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "documentation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The URL of a web page to help with resolving this issue."]
    pub documentation: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the issue can be resolved by the merchant."]
    pub resolution: ::std::option::Option<::std::string::String>,
    #[serde(rename = "servability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "How this issue affects serving of the offer."]
    pub servability: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductSubscriptionCost {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount the buyer has to pay per subscription period."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of subscription period."]
    pub period: ::std::option::Option<::std::string::String>,
    #[serde(rename = "periodLength")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of subscription periods the buyer has to pay."]
    pub period_length: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductTax {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The country within which the item is taxed, specified as a CLDR territory code."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The numeric ID of a location that the tax rate applies to as defined in the AdWords API."]
    pub location_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The postal code range that the tax rate applies to, represented by a ZIP code, a ZIP code prefix using * wildcard, a range between two ZIP codes or two ZIP code prefixes of equal length. Examples: 94114, 94*, 94002-95460, 94*-95*."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage of tax rate that applies to the item price."]
    pub rate: ::std::option::Option<::std::primitive::f64>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The geographic region to which the tax rate applies."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxShip")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Should be set to true if tax is charged on shipping."]
    pub tax_ship: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductUnitPricingBaseMeasure {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of the denominator."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The denominator of the unit price."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductUnitPricingMeasure {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unit of the measure."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The measure of an item."]
    pub value: ::std::option::Option<::std::primitive::f64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductsCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch products request."]
pub struct ProductsCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "feedId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Content API feed id."]
    pub feed_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The product to insert. Only required if the method is `insert`."]
    pub product: ::std::option::Option<::std::boxed::Box<Product>>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to get or delete. Only defined if the method is `get` or `delete`."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductsCustomBatchResponseEntry>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productsCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch products response."]
pub struct ProductsCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productsCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The inserted product. Only defined if the method is `insert` and if the request was successful."]
    pub product: ::std::option::Option<::std::boxed::Box<Product>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of products."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductstatusesCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ProductstatusesCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch productstatuses request."]
pub struct ProductstatusesCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
    pub destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "includeAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product whose status to get."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductstatusesCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ProductstatusesCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productstatusesCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch productstatuses response."]
pub struct ProductstatusesCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors, if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productstatusesCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The requested product status. Only defined if the request was successful."]
    pub product_status: ::std::option::Option<::std::boxed::Box<ProductStatus>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductstatusesListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productstatusesListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of products statuses."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatus>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settings for Pub/Sub notifications, all methods require that the caller is a direct user of the merchant center account."]
pub struct PubsubNotificationSettings {
    #[serde(rename = "cloudTopicName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Cloud pub/sub topic to which notifications are sent (read-only)."]
    pub cloud_topic_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#pubsubNotificationSettings`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "registeredEvents")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of event types. Acceptable values are: - \"`orderPendingShipment`\" "]
    pub registered_events: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RateGroup {
    #[serde(rename = "applicableShippingLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of shipping labels defining the products to which this rate group applies to. This is a disjunction: only one of the labels has to match for the rate group to apply. May only be empty for the last rate group of a service. Required."]
    pub applicable_shipping_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "carrierRates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of carrier rates that can be referred to by `mainTable` or `singleValue`."]
    pub carrier_rates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CarrierRate>>>,
    #[serde(rename = "mainTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A table defining the rate group, when `singleValue` is not expressive enough. Can only be set if `singleValue` is not set."]
    pub main_table: ::std::option::Option<::std::boxed::Box<Table>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the rate group. Optional. If set has to be unique within shipping service."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "singleValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the rate group (e.g. flat rate $10). Can only be set if `mainTable` and `subtables` are not set."]
    pub single_value: ::std::option::Option<::std::boxed::Box<Value>>,
    #[serde(rename = "subtables")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of subtables referred to by `mainTable`. Can only be set if `mainTable` is set."]
    pub subtables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Table>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefundReason {
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Description of the reason."]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Code of the refund reason. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a geographic region that you can use as a target with both the `RegionalInventory` and `ShippingSettings` services. You can define regions as collections of either postal codes or, in some countries, using predefined geotargets."]
pub struct Region {
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The display name of the region."]
    pub display_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "geotargetArea")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of geotargets that defines the region area."]
    pub geotarget_area: ::std::option::Option<::std::boxed::Box<RegionGeoTargetArea>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. Merchant that owns the region."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCodeArea")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of postal codes that defines the region area."]
    pub postal_code_area: ::std::option::Option<::std::boxed::Box<RegionPostalCodeArea>>,
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The ID uniquely identifying each region."]
    pub region_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionalInventoryEligible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates if the region is eligible to use in the Regional Inventory configuration."]
    pub regional_inventory_eligible: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "shippingEligible")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Indicates if the region is eligible to use in the Shipping Services configuration."]
    pub shipping_eligible: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of geotargets that defines the region area."]
pub struct RegionGeoTargetArea {
    #[serde(rename = "geotargetCriteriaIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A non-empty list of [location IDs](https://developers.google.com/adwords/api/docs/appendix/geotargeting). They must all be of the same location type (e.g., state)."]
    pub geotarget_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A list of postal codes that defines the region area. Note: All regions defined using postal codes are accessible via the account's `ShippingSettings.postalCodeGroups` resource."]
pub struct RegionPostalCodeArea {
    #[serde(rename = "postalCodes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A range of postal codes."]
    pub postal_codes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<RegionPostalCodeAreaPostalCodeRange>>,
    >,
    #[serde(rename = "regionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. CLDR territory code or the country the postal code group applies to."]
    pub region_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A range of postal codes that defines the region area."]
pub struct RegionPostalCodeAreaPostalCodeRange {
    #[serde(rename = "begin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. A postal code or a pattern of the form prefix* denoting the inclusive lower bound of the range defining the area. Examples values: \"94108\", \"9410*\", \"9*\"."]
    pub begin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. A postal code or a pattern of the form prefix* denoting the inclusive upper bound of the range defining the area. It must have the same length as postalCodeRangeBegin: if postalCodeRangeBegin is a postal code then postalCodeRangeEnd must be a postal code too; if postalCodeRangeBegin is a pattern then postalCodeRangeEnd must be a pattern with the same prefix length. Optional: if not set, then the area is defined as being all the postal codes matching postalCodeRangeBegin."]
    pub end: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Regional inventory resource. contains the regional name and all attributes which are overridden for the specified region."]
pub struct RegionalInventory {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availability of the product."]
    pub availability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form."]
    pub custom_attributes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#regionalInventory\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price of the product."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "regionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID uniquely identifying each region."]
    pub region_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "salePrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
    pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "salePriceEffectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided."]
    pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalinventoryCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<RegionalinventoryCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch regional inventory request."]
pub struct RegionalinventoryCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Method of the batch request entry. Acceptable values are: - \"`insert`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product for which to update price and availability."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionalInventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price and availability of the product."]
    pub regional_inventory: ::std::option::Option<::std::boxed::Box<RegionalInventory>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegionalinventoryCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<RegionalinventoryCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#regionalinventoryCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch regional inventory response."]
pub struct RegionalinventoryCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#regionalinventoryCustomBatchResponseEntry\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "regionalInventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price and availability of the product."]
    pub regional_inventory: ::std::option::Option<::std::boxed::Box<RegionalInventory>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Result row returned from the search query."]
pub struct ReportRow {
    #[serde(rename = "metrics")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Metrics requested by the merchant in the query. Metric values are only set for metrics requested explicitly in the query."]
    pub metrics: ::std::option::Option<::std::boxed::Box<Metrics>>,
    #[serde(rename = "segments")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Segmentation dimensions requested by the merchant in the query. Dimension values are only set for dimensions requested explicitly in the query."]
    pub segments: ::std::option::Option<::std::boxed::Box<Segments>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource that represents a daily Repricing product report. Each report contains stats for a single type of Repricing rule for a single product on a given day. If there are multiple rules of the same type for the product on that day, the report lists all the rules by rule ids, combines the stats, and paginates the results by date. To retrieve the stats of a particular rule, provide the rule_id in the request."]
pub struct RepricingProductReport {
    #[serde(rename = "applicationCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total count of Repricer applications. This value captures how many times the rule of this type was applied to this product during this reporting period."]
    pub application_count: ::std::option::Option<::std::string::String>,
    #[serde(rename = "buyboxWinningProductStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stats specific to buybox winning rules for product report."]
    pub buybox_winning_product_stats:
        ::std::option::Option<::std::boxed::Box<RepricingProductReportBuyboxWinningProductStats>>,
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of the stats in this report. The report starts and ends according to the merchant's timezone."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "highWatermark")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum displayed price after repriced during this reporting period."]
    pub high_watermark: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "inapplicabilityDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all reasons the rule did not apply to the product during the specified reporting period."]
    pub inapplicability_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InapplicabilityDetails>>>,
    #[serde(rename = "lowWatermark")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum displayed price after repriced during this reporting period."]
    pub low_watermark: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "orderItemCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total unit count of impacted products ordered while the rule was active on the date of the report. This count includes all orders that were started while the rule was active, even if the rule was no longer active when the order was completed."]
    pub order_item_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ruleIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Ids of the Repricing rule for this report."]
    pub rule_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "totalGmv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total GMV generated by impacted products while the rule was active on the date of the report. This value includes all orders that were started while the rule was active, even if the rule was no longer active when the order was completed."]
    pub total_gmv: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the rule."]
    pub _type: ::std::option::Option<RepricingProductReportTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the rule."]
pub enum RepricingProductReportTypeEnum {
    #[serde(rename = "REPRICING_RULE_TYPE_UNSPECIFIED")]
    #[doc = "Unused."]
    RepricingRuleTypeUnspecified,
    #[serde(rename = "TYPE_STATS_BASED")]
    #[doc = "Statistical measurement based rules among Google SA merchants. If this rule is chosen, repricer will adjust the offer price based on statistical metrics (currently only min is available) among other merchants who sell the same product. Details need to be provdided in the RuleDefinition."]
    TypeStatsBased,
    #[serde(rename = "TYPE_COGS_BASED")]
    #[doc = "Cost of goods sale based rule. Repricer will adjust the offer price based on the offer's sale cost which is provided by the merchant."]
    TypeCogsBased,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Stats specific to buybox winning rules for product report."]
pub struct RepricingProductReportBuyboxWinningProductStats {
    #[serde(rename = "buyboxWinsCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of times this product won the buybox with these rules during this time period."]
    pub buybox_wins_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a repricing rule. A repricing rule is used by shopping serving to adjust transactable offer prices if conditions are met. Next ID: 24"]
pub struct RepricingRule {
    #[serde(rename = "cogsBasedRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rule definition for TYPE_COGS_BASED. Required when the rule type is TYPE_COGS_BASED."]
    pub cogs_based_rule: ::std::option::Option<::std::boxed::Box<RepricingRuleCostOfGoodsSaleRule>>,
    #[serde(rename = "countryCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. [CLDR country code](http://www.unicode.org/repos/cldr/tags/latest/common/main/en.xml) (e.g. \"US\")."]
    pub country_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "effectiveTimePeriod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Time period when the rule should take effect."]
    pub effective_time_period: ::std::option::Option<::std::boxed::Box<RepricingRuleEffectiveTime>>,
    #[serde(rename = "eligibleOfferMatcher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Match criteria for the eligible offers."]
    pub eligible_offer_matcher:
        ::std::option::Option<::std::boxed::Box<RepricingRuleEligibleOfferMatcher>>,
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The two-letter ISO 639-1 language code associated with the repricing rule."]
    pub language_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. Merchant that owns the repricing rule."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "paused")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Represents whether a rule is paused. A paused rule will behave like a non-paused rule within CRUD operations, with the major difference that a paused rule will not be evaluated and will have no effect on offers."]
    pub paused: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "restriction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Restriction of the rule appliance."]
    pub restriction: ::std::option::Option<::std::boxed::Box<RepricingRuleRestriction>>,
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Immutable. The ID to uniquely identify each repricing rule."]
    pub rule_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "statsBasedRule")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The rule definition for TYPE_STATS_BASED. Required when the rule type is TYPE_STATS_BASED."]
    pub stats_based_rule: ::std::option::Option<::std::boxed::Box<RepricingRuleStatsBasedRule>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The title for the rule."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Immutable. The type of the rule."]
    pub _type: ::std::option::Option<RepricingRuleTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Required. Immutable. The type of the rule."]
pub enum RepricingRuleTypeEnum {
    #[serde(rename = "REPRICING_RULE_TYPE_UNSPECIFIED")]
    #[doc = "Unused."]
    RepricingRuleTypeUnspecified,
    #[serde(rename = "TYPE_STATS_BASED")]
    #[doc = "Statistical measurement based rules among Google SA merchants. If this rule is chosen, repricer will adjust the offer price based on statistical metrics (currently only min is available) among other merchants who sell the same product. Details need to be provdided in the RuleDefinition."]
    TypeStatsBased,
    #[serde(rename = "TYPE_COGS_BASED")]
    #[doc = "Cost of goods sale based rule. Repricer will adjust the offer price based on the offer's sale cost which is provided by the merchant."]
    TypeCogsBased,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A repricing rule that changes the sale price based on cost of goods sale."]
pub struct RepricingRuleCostOfGoodsSaleRule {
    #[serde(rename = "percentageDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percent change against the COGS. Ex: 20 would mean to set the adjusted price 1.2X of the COGS data."]
    pub percentage_delta: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "priceDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price delta against the COGS. E.g. 2 means $2 more of the COGS."]
    pub price_delta: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepricingRuleEffectiveTime {
    #[serde(rename = "fixedTimePeriods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of fixed time periods combined with OR. The maximum number of entries is limited to 5."]
    pub fixed_time_periods: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<RepricingRuleEffectiveTimeFixedTimePeriod>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a fixed time period."]
pub struct RepricingRuleEffectiveTimeFixedTimePeriod {
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end time (exclusive) of the period. It can only be hour granularity."]
    pub end_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start time (inclusive) of the period. It can only be hour granularity."]
    pub start_time: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Matcher that specifies eligible offers. When the USE_FEED_ATTRIBUTE option is selected, only the repricing_rule_id attribute on the product feed is used to specify offer-rule mapping. When the CUSTOM_FILTER option is selected, only the *_matcher fields are used to filter the offers for offer-rule mapping. If the CUSTOM_FILTER option is selected, an offer needs to satisfy each custom filter matcher to be eligible for a rule. Size limit: the sum of the number of entries in all the matchers should not exceed 20. For example, there can be 15 product ids and 5 brands, but not 10 product ids and 11 brands."]
pub struct RepricingRuleEligibleOfferMatcher {
    #[serde(rename = "brandMatcher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter by the brand."]
    pub brand_matcher:
        ::std::option::Option<::std::boxed::Box<RepricingRuleEligibleOfferMatcherStringMatcher>>,
    #[serde(rename = "itemGroupIdMatcher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter by the item group id."]
    pub item_group_id_matcher:
        ::std::option::Option<::std::boxed::Box<RepricingRuleEligibleOfferMatcherStringMatcher>>,
    #[serde(rename = "matcherOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Determines whether to use the custom matchers or the product feed attribute \"repricing_rule_id\" to specify offer-rule mapping."]
    pub matcher_option: ::std::option::Option<RepricingRuleEligibleOfferMatcherMatcherOptionEnum>,
    #[serde(rename = "offerIdMatcher")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Filter by the offer id."]
    pub offer_id_matcher:
        ::std::option::Option<::std::boxed::Box<RepricingRuleEligibleOfferMatcherStringMatcher>>,
    #[serde(rename = "skipWhenOnPromotion")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "When true, the rule won't be applied to offers with active promotions."]
    pub skip_when_on_promotion: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Determines whether to use the custom matchers or the product feed attribute \"repricing_rule_id\" to specify offer-rule mapping."]
pub enum RepricingRuleEligibleOfferMatcherMatcherOptionEnum {
    #[serde(rename = "MATCHER_OPTION_UNSPECIFIED")]
    #[doc = "Unused."]
    MatcherOptionUnspecified,
    #[serde(rename = "MATCHER_OPTION_CUSTOM_FILTER")]
    #[doc = "Use custom filters."]
    MatcherOptionCustomFilter,
    #[serde(rename = "MATCHER_OPTION_USE_FEED_ATTRIBUTE")]
    #[doc = "Use repricing_rule_id feed attribute on the product resource to specify offer-rule mapping."]
    MatcherOptionUseFeedAttribute,
    #[serde(rename = "MATCHER_OPTION_ALL_PRODUCTS")]
    #[doc = "Matching all products."]
    MatcherOptionAllProducts,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Matcher by string attributes."]
pub struct RepricingRuleEligibleOfferMatcherStringMatcher {
    #[serde(rename = "strAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "String attributes, as long as such attribute of an offer is one of the string attribute values, the offer is considered as passing the matcher. The string matcher checks an offer for inclusivity in the string attributes, not equality. Only literal string matching is supported, no regex."]
    pub str_attributes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Resource that represents a daily Repricing rule report. Next ID: 11"]
pub struct RepricingRuleReport {
    #[serde(rename = "buyboxWinningRuleStats")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Stats specific to buybox winning rules for rule report."]
    pub buybox_winning_rule_stats:
        ::std::option::Option<::std::boxed::Box<RepricingRuleReportBuyboxWinningRuleStats>>,
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date of the stats in this report. The report starts and ends according to the merchant's timezone."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "impactedProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of product ids that are impacted by this rule during this reporting period. Out of stock products and products not searched for by customers are examples of non-impacted products."]
    pub impacted_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "inapplicabilityDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of all reasons the rule did not apply to the inapplicable products during the specified reporting period."]
    pub inapplicability_details:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InapplicabilityDetails>>>,
    #[serde(rename = "inapplicableProducts")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of product ids that are inapplicable to this rule during this reporting period. To get the inapplicable reason for a specific product, see RepricingProductReport."]
    pub inapplicable_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "orderItemCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total unit count of impacted products ordered while the rule was active on the date of the report. This count includes all orders that were started while the rule was active, even if the rule was no longer active when the order was completed."]
    pub order_item_count: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Id of the Repricing rule for this report."]
    pub rule_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalGmv")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total GMV generated by impacted products while the rule was active on the date of the report. This value includes all orders that were started while the rule was active, even if the rule was no longer active when the order was completed."]
    pub total_gmv: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the rule."]
    pub _type: ::std::option::Option<RepricingRuleReportTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of the rule."]
pub enum RepricingRuleReportTypeEnum {
    #[serde(rename = "REPRICING_RULE_TYPE_UNSPECIFIED")]
    #[doc = "Unused."]
    RepricingRuleTypeUnspecified,
    #[serde(rename = "TYPE_STATS_BASED")]
    #[doc = "Statistical measurement based rules among Google SA merchants. If this rule is chosen, repricer will adjust the offer price based on statistical metrics (currently only min is available) among other merchants who sell the same product. Details need to be provdided in the RuleDefinition."]
    TypeStatsBased,
    #[serde(rename = "TYPE_COGS_BASED")]
    #[doc = "Cost of goods sale based rule. Repricer will adjust the offer price based on the offer's sale cost which is provided by the merchant."]
    TypeCogsBased,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Stats specific to buybox winning rules for rule report."]
pub struct RepricingRuleReportBuyboxWinningRuleStats {
    #[serde(rename = "buyboxWonProductCount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of unique products that won the buybox with this rule during this period of time."]
    pub buybox_won_product_count: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a rule restriction. At least one of the following needs to be true: (1) use_auto_pricing_min_price is true (2) floor.price_delta exists (3) floor.percentage_delta exists If floor.price_delta and floor.percentage_delta are both set on a rule, the highest value will be chosen by the Repricer. In other words, for a product with a price of $50, if the `floor.percentage_delta` is \"-10\" and the floor.price_delta is \"-12\", the offer price will only be lowered $5 (10% lower than the original offer price)."]
pub struct RepricingRuleRestriction {
    #[serde(rename = "floor")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The inclusive floor lower bound. The repricing rule only applies when new price >= floor."]
    pub floor: ::std::option::Option<::std::boxed::Box<RepricingRuleRestrictionBoundary>>,
    #[serde(rename = "useAutoPricingMinPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, use the AUTO_PRICING_MIN_PRICE offer attribute as the lower bound of the rule. If use_auto_pricing_min_price is true, then only offers with `AUTO_PRICING_MIN_PRICE` existing on the offer will get Repricer treatment, even if a floor value is set on the rule. Also, if use_auto_pricing_min_price is true, the floor restriction will be ignored."]
    pub use_auto_pricing_min_price: ::std::option::Option<::std::primitive::bool>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of a boundary."]
pub struct RepricingRuleRestrictionBoundary {
    #[serde(rename = "percentageDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percentage delta relative to the offer selling price. This field is signed. It must be negative in floor. When it is used in floor, it should be > -100. For example, if an offer is selling at $10 and this field is -30 in floor, the repricing rule only applies if the calculated new price is >= $7."]
    pub percentage_delta: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "priceDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price micros relative to the offer selling price. This field is signed. It must be negative in floor. For example, if an offer is selling at $10 and this field is -$2 in floor, the repricing rule only applies if the calculated new price is >= $8."]
    pub price_delta: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Definition of stats based rule."]
pub struct RepricingRuleStatsBasedRule {
    #[serde(rename = "percentageDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The percent change against the price target. Valid from 0 to 100 inclusively."]
    pub percentage_delta: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "priceDelta")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price delta against the above price target. A positive value means the price should be adjusted to be above statistical measure, and a negative value means below. Currency code must not be included."]
    pub price_delta: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Return address resource."]
pub struct ReturnAddress {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The address."]
    pub address: ::std::option::Option<::std::boxed::Box<ReturnAddressAddress>>,
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The country of sale where the return address is applicable."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#returnAddress`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user-defined label of the return address. For the default address, use the label \"default\"."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The merchant's contact phone number regarding the return."]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnAddressId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return address ID generated by Google."]
    pub return_address_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnAddressAddress {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CLDR country code (e.g. \"US\")."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal code or ZIP (e.g. \"94043\")."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the recipient to address returns to."]
    pub recipient_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Street-level part of the address. May be up to two lines, each line specified as an array element."]
    pub street_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Return policy resource."]
pub struct ReturnPolicy {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The country of sale where the return policy is applicable."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#returnPolicy`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The user-defined label of the return policy. For the default policy, use the label \"default\"."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the policy as shown in Merchant Center."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nonFreeReturnReasons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return reasons that will incur return fees."]
    pub non_free_return_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The policy."]
    pub policy: ::std::option::Option<::std::boxed::Box<ReturnPolicyPolicy>>,
    #[serde(rename = "returnPolicyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Return policy ID generated by Google."]
    pub return_policy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return shipping fee that will apply to non free return reasons."]
    pub return_shipping_fee: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "seasonalOverrides")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An optional list of seasonal overrides."]
    pub seasonal_overrides:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnPolicySeasonalOverride>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Return policy online object. This is currently used to represent return policies for ads and free listings programs."]
pub struct ReturnPolicyOnline {
    #[serde(rename = "countries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The countries of sale where the return policy is applicable. The values must be a valid 2 letter ISO 3166 code, e.g. \"US\"."]
    pub countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "itemConditions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item conditions that are accepted for returns. This is required to not be empty unless the type of return policy is noReturns."]
    pub item_conditions:
        ::std::option::Option<::std::vec::Vec<ReturnPolicyOnlineItemConditionsEnum>>,
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique user-defined label of the return policy. The same label cannot be used in different return policies for the same country. Policies with the label 'default' will apply to all products, unless a product specifies a return_policy_label attribute."]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the policy as shown in Merchant Center."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return policy."]
    pub policy: ::std::option::Option<::std::boxed::Box<ReturnPolicyOnlinePolicy>>,
    #[serde(rename = "restockingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The restocking fee that applies to all return reason categories. This would be treated as a free restocking fee if the value is not set."]
    pub restocking_fee: ::std::option::Option<::std::boxed::Box<ReturnPolicyOnlineRestockingFee>>,
    #[serde(rename = "returnMethods")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return methods of how customers can return an item. This value is required to not be empty unless the type of return policy is noReturns."]
    pub return_methods: ::std::option::Option<::std::vec::Vec<ReturnPolicyOnlineReturnMethodsEnum>>,
    #[serde(rename = "returnPolicyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Output only. Return policy ID generated by Google."]
    pub return_policy_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnPolicyUri")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return policy uri. This can used by Google to do a sanity check for the policy."]
    pub return_policy_uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnReasonCategoryInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return reason category information. This required to not be empty unless the type of return policy is noReturns."]
    pub return_reason_category_info: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ReturnPolicyOnlineReturnReasonCategoryInfo>>,
    >,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ReturnPolicyOnlineItemConditionsEnum {
    #[serde(rename = "ITEM_CONDITION_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ItemConditionUnspecified,
    #[serde(rename = "NEW")]
    #[doc = "New."]
    New,
    #[serde(rename = "USED")]
    #[doc = "Used."]
    Used,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub enum ReturnPolicyOnlineReturnMethodsEnum {
    #[serde(rename = "RETURN_METHOD_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ReturnMethodUnspecified,
    #[serde(rename = "BY_MAIL")]
    #[doc = "By mail."]
    ByMail,
    #[serde(rename = "IN_STORE")]
    #[doc = "In store."]
    InStore,
    #[serde(rename = "AT_A_KIOSK")]
    #[doc = "At a kiosk."]
    AtAKiosk,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The available policies."]
pub struct ReturnPolicyOnlinePolicy {
    #[serde(rename = "days")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of days items can be returned after delivery, where one day is defined to be 24 hours after the delivery timestamp. Required for `numberOfDaysAfterDelivery` returns."]
    pub days: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy type."]
    pub _type: ::std::option::Option<ReturnPolicyOnlinePolicyTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Policy type."]
pub enum ReturnPolicyOnlinePolicyTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    TypeUnspecified,
    #[serde(rename = "NUMBER_OF_DAYS_AFTER_DELIVERY")]
    #[doc = "Number of days after a return is delivered."]
    NumberOfDaysAfterDelivery,
    #[serde(rename = "NO_RETURNS")]
    #[doc = "No returns."]
    NoReturns,
    #[serde(rename = "LIFETIME_RETURNS")]
    #[doc = "Life time returns."]
    LifetimeReturns,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The restocking fee. This can either be a fixed fee or a micro percent."]
pub struct ReturnPolicyOnlineRestockingFee {
    #[serde(rename = "fixedFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed restocking fee."]
    pub fixed_fee: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "microPercent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Percent of total price in micros. 15,000,000 means 15% of the total price would be charged."]
    pub micro_percent: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The return reason category info wrapper."]
pub struct ReturnPolicyOnlineReturnReasonCategoryInfo {
    #[serde(rename = "returnLabelSource")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corresponding return label source."]
    pub return_label_source:
        ::std::option::Option<ReturnPolicyOnlineReturnReasonCategoryInfoReturnLabelSourceEnum>,
    #[serde(rename = "returnReasonCategory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return reason category."]
    pub return_reason_category:
        ::std::option::Option<ReturnPolicyOnlineReturnReasonCategoryInfoReturnReasonCategoryEnum>,
    #[serde(rename = "returnShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The corresponding return shipping fee. This is only applicable when returnLabelSource is not the customer's responsibility."]
    pub return_shipping_fee:
        ::std::option::Option<::std::boxed::Box<ReturnPolicyOnlineReturnShippingFee>>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The corresponding return label source."]
pub enum ReturnPolicyOnlineReturnReasonCategoryInfoReturnLabelSourceEnum {
    #[serde(rename = "RETURN_LABEL_SOURCE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ReturnLabelSourceUnspecified,
    #[serde(rename = "DOWNLOAD_AND_PRINT")]
    #[doc = "Download and print the label."]
    DownloadAndPrint,
    #[serde(rename = "IN_THE_BOX")]
    #[doc = "Label in the box."]
    InTheBox,
    #[serde(rename = "CUSTOMER_RESPONSIBILITY")]
    #[doc = "Customers' responsibility to get the label."]
    CustomerResponsibility,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "The return reason category."]
pub enum ReturnPolicyOnlineReturnReasonCategoryInfoReturnReasonCategoryEnum {
    #[serde(rename = "RETURN_REASON_CATEGORY_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    ReturnReasonCategoryUnspecified,
    #[serde(rename = "BUYER_REMORSE")]
    #[doc = "Buyer remorse."]
    BuyerRemorse,
    #[serde(rename = "ITEM_DEFECT")]
    #[doc = "Item defect."]
    ItemDefect,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The return shipping fee. This can either be a fixed fee or a boolean to indicate that the customer pays the actual shipping cost."]
pub struct ReturnPolicyOnlineReturnShippingFee {
    #[serde(rename = "fixedFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fixed return shipping fee amount. This value is only applicable when type is FIXED. We will treat the return shipping fee as free if type is FIXED and this value is not set."]
    pub fixed_fee: ::std::option::Option<::std::boxed::Box<PriceAmount>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of return shipping fee."]
    pub _type: ::std::option::Option<ReturnPolicyOnlineReturnShippingFeeTypeEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Type of return shipping fee."]
pub enum ReturnPolicyOnlineReturnShippingFeeTypeEnum {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    #[doc = "Default value. This value is unused."]
    TypeUnspecified,
    #[serde(rename = "FIXED")]
    #[doc = "The return shipping fee is a fixed value."]
    Fixed,
    #[serde(rename = "CUSTOMER_PAYING_ACTUAL_FEE")]
    #[doc = "Customer will pay the actual return shipping fee."]
    CustomerPayingActualFee,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnPolicyPolicy {
    #[serde(rename = "lastReturnDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Last day for returning the items. In ISO 8601 format. When specifying the return window like this, set the policy type to \"lastReturnDate\". Use this for seasonal overrides only."]
    pub last_return_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numberOfDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The number of days items can be returned after delivery, where one day is defined to be 24 hours after the delivery timestamp. When specifying the return window like this, set the policy type to \"numberOfDaysAfterDelivery\". Acceptable values are 30, 45, 60, 90, 100, 180, 270 and 365 for the default policy. Additional policies further allow 14, 15, 21 and 28 days, but note that for most items a minimum of 30 days is required for returns. Exceptions may be made for electronics. A policy of less than 30 days can only be applied to those items."]
    pub number_of_days: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Policy type. Use \"lastReturnDate\" for seasonal overrides only. Note that for most items a minimum of 30 days is required for returns. Exceptions may be made for electronics or non-returnable items such as food, perishables, and living things. A policy of less than 30 days can only be applied to those items. Acceptable values are: - \"`lastReturnDate`\" - \"`lifetimeReturns`\" - \"`noReturns`\" - \"`numberOfDaysAfterDelivery`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnPolicySeasonalOverride {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Last day on which the override applies. In ISO 8601 format."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The name of the seasonal override as shown in Merchant Center."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The policy which is in effect during that time."]
    pub policy: ::std::option::Option<::std::boxed::Box<ReturnPolicyPolicy>>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. First day on which the override applies. In ISO 8601 format."]
    pub start_date: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnPricingInfo {
    #[serde(rename = "chargeReturnShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Default option for whether merchant should charge the customer for return shipping costs, based on customer selected return reason and merchant's return policy for the items being returned."]
    pub charge_return_shipping_fee: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "maxReturnShippingFee")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum return shipping costs that may be charged to the customer depending on merchant's assessment of the return reason and the merchant's return policy for the items being returned."]
    pub max_return_shipping_fee: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "refundableItemsTotalAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total amount that can be refunded for the items in this return. It represents the total amount received by the merchant for the items, after applying merchant coupons."]
    pub refundable_items_total_amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "refundableShippingAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximum amount that can be refunded for the original shipping fee."]
    pub refundable_shipping_amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
    #[serde(rename = "totalRefundedAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total amount already refunded by the merchant. It includes all types of refunds (items, shipping, etc.) Not provided if no refund has been applied yet."]
    pub total_refunded_amount: ::std::option::Option<::std::boxed::Box<MonetaryAmount>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnShipment {
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of creation of the shipment, in ISO 8601 format."]
    pub creation_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of delivery of the shipment, in ISO 8601 format."]
    pub delivery_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnMethodType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of the return method. Acceptable values are: - \"`byMail`\" - \"`contactCustomerSupport`\" - \"`returnless`\" - \"`inStore`\" "]
    pub return_method_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipment ID generated by Google."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentTrackingInfos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tracking information of the shipment. One return shipment might be handled by several shipping carriers sequentially."]
    pub shipment_tracking_infos:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentTrackingInfo>>>,
    #[serde(rename = "shippingDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date of shipping of the shipment, in ISO 8601 format."]
    pub shipping_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the shipment. Acceptable values are: - \"`completed`\" - \"`new`\" - \"`shipped`\" - \"`undeliverable`\" - \"`pending`\" "]
    pub state: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnaddressCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ReturnaddressCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnaddressCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Merchant Center account ID."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Method of the batch request entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return address to submit. This should be set only if the method is `insert`."]
    pub return_address: ::std::option::Option<::std::boxed::Box<ReturnAddress>>,
    #[serde(rename = "returnAddressId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return address ID. This should be set only if the method is `delete` or `get`."]
    pub return_address_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnaddressCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ReturnaddressCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#returnaddressCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnaddressCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry to which this entry responds."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if, and only if, the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#returnaddressCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved return address."]
    pub return_address: ::std::option::Option<::std::boxed::Box<ReturnAddress>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnaddressListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#returnaddressListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of addresses."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnAddress>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnpolicyCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ReturnpolicyCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnpolicyCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Merchant Center account ID."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Method of the batch request entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return policy to submit. This should be set only if the method is `insert`."]
    pub return_policy: ::std::option::Option<::std::boxed::Box<ReturnPolicy>>,
    #[serde(rename = "returnPolicyId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The return policy ID. This should be set only if the method is `delete` or `get`."]
    pub return_policy_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnpolicyCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ReturnpolicyCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#returnpolicyCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnpolicyCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry to which this entry responds."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if, and only if, the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#returnpolicyCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "returnPolicy")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved return policy."]
    pub return_policy: ::std::option::Option<::std::boxed::Box<ReturnPolicy>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReturnpolicyListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#returnpolicyListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnPolicy>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Row {
    #[serde(rename = "cells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of cells that constitute the row. Must have the same length as `columnHeaders` for two-dimensional tables, a length of 1 for one-dimensional tables. Required."]
    pub cells: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Request message for the ReportService.Search method."]
pub struct SearchRequest {
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number of ReportRows to retrieve in a single page. Defaults to the maximum of 1000. Values above 1000 are coerced to 1000."]
    pub page_size: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token of the page to retrieve. If not specified, the first page of results is returned. In order to request the next page of results, the value obtained from `next_page_token` in the previous response should be used."]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Query that defines performance metrics to retrieve and dimensions according to which the metrics are to be segmented. "]
    pub query: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Response message for the ReportService.Search method."]
pub struct SearchResponse {
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Token which can be sent as `page_token` to retrieve the next page. If omitted, there are no subsequent pages."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "results")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rows that matched the search query."]
    pub results: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReportRow>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Dimensions according to which metrics are segmented in the response. Values of product dimensions, e.g., offer id, reflect the state of a product at the time of the corresponding event, e.g., impression or order. Segment fields cannot be selected in queries without also selecting at least one metric field. Values are only set for dimensions requested explicitly in the request's search query."]
pub struct Segments {
    #[serde(rename = "date")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date in the merchant timezone to which metrics apply."]
    pub date: ::std::option::Option<::std::boxed::Box<Date>>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Merchant-provided id of the product."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "program")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Program to which metrics apply, e.g., Free Product Listing."]
    pub program: ::std::option::Option<SegmentsProgramEnum>,
}
#[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
#[doc = "Program to which metrics apply, e.g., Free Product Listing."]
pub enum SegmentsProgramEnum {
    #[serde(rename = "PROGRAM_UNSPECIFIED")]
    #[doc = "Not specified."]
    ProgramUnspecified,
    #[serde(rename = "SHOPPING_ADS")]
    #[doc = "Shopping Ads."]
    ShoppingAds,
    #[serde(rename = "FREE_PRODUCT_LISTING")]
    #[doc = "Free Product Listing."]
    FreeProductListing,
    #[serde(rename = "FREE_LOCAL_PRODUCT_LISTING")]
    #[doc = "Free Local Product Listing."]
    FreeLocalProductListing,
    #[serde(rename = "BUY_ON_GOOGLE_LISTING")]
    #[doc = "Buy on Google Listing."]
    BuyOnGoogleListing,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    #[serde(rename = "active")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A boolean exposing the active status of the shipping service. Required."]
    pub active: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "currency")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR code of the currency to which this service applies. Must match that of the prices in rate groups."]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The CLDR territory code of the country to which the service applies. Required."]
    pub delivery_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Time spent in various aspects from order to the delivery of the product. Required."]
    pub delivery_time: ::std::option::Option<::std::boxed::Box<DeliveryTime>>,
    #[serde(rename = "eligibility")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Eligibility for this service. Acceptable values are: - \"`All scenarios`\" - \"`All scenarios except Shopping Actions`\" - \"`Shopping Actions`\" "]
    pub eligibility: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minimumOrderValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Minimum order value for this service. If set, indicates that customers will have to spend at least this amount. All prices within a service must have the same currency. Cannot be set together with minimum_order_value_table."]
    pub minimum_order_value: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "minimumOrderValueTable")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Table of per store minimum order values for the pickup fulfillment type. Cannot be set together with minimum_order_value."]
    pub minimum_order_value_table: ::std::option::Option<::std::boxed::Box<MinimumOrderValueTable>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form name of the service. Must be unique within target account. Required."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupService")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier-service pair delivering items to collection points. The list of supported pickup services can be retrieved via the `getSupportedPickupServices` method. Required if and only if the service delivery type is `pickup`."]
    pub pickup_service: ::std::option::Option<::std::boxed::Box<PickupCarrierService>>,
    #[serde(rename = "rateGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipping rate group definitions. Only the last one is allowed to have an empty `applicableShippingLabels`, which means \"everything else\". The other `applicableShippingLabels` must not overlap."]
    pub rate_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RateGroup>>>,
    #[serde(rename = "shipmentType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Type of locations this service ships orders to. Acceptable values are: - \"`delivery`\" - \"`pickup`\" "]
    pub shipment_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = " Settlement reports detail order-level and item-level credits and debits between you and Google."]
pub struct SettlementReport {
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The end date on which all transactions are included in the report, in ISO 8601 format."]
    pub end_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#settlementReport`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "previousBalance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The residual amount from the previous invoice. This is set only if the previous invoices are not paid because of negative balance."]
    pub previous_balance: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "settlementId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the settlement report."]
    pub settlement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The start date on which all transactions are included in the report, in ISO 8601 format."]
    pub start_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The money due to the merchant."]
    pub transfer_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "transferDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which transfer for this payment was initiated by Google, in ISO 8601 format."]
    pub transfer_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transferIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of bank identifiers used for the transfer. e.g. Trace ID for Federal Automated Clearing House (ACH). This may also be known as the Wire ID."]
    pub transfer_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Settlement transactions give a detailed breakdown of the settlement report."]
pub struct SettlementTransaction {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount for the transaction."]
    pub amount: ::std::option::Option<::std::boxed::Box<SettlementTransactionAmount>>,
    #[serde(rename = "identifiers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifiers of the transaction."]
    pub identifiers: ::std::option::Option<::std::boxed::Box<SettlementTransactionIdentifiers>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#settlementTransaction`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transaction")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Details of the transaction."]
    pub transaction: ::std::option::Option<::std::boxed::Box<SettlementTransactionTransaction>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementTransactionAmount {
    #[serde(rename = "commission")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub commission: ::std::option::Option<::std::boxed::Box<SettlementTransactionAmountCommission>>,
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The description of the event. Acceptable values are: - \"`taxWithhold`\" - \"`principal`\" - \"`principalAdjustment`\" - \"`shippingFee`\" - \"`merchantRemittedSalesTax`\" - \"`googleRemittedSalesTax`\" - \"`merchantCoupon`\" - \"`merchantCouponTax`\" - \"`merchantRemittedDisposalTax`\" - \"`googleRemittedDisposalTax`\" - \"`merchantRemittedRedemptionFee`\" - \"`googleRemittedRedemptionFee`\" - \"`eeeEcoFee`\" - \"`furnitureEcoFee`\" - \"`copyPrivateFee`\" - \"`eeeEcoFeeCommission`\" - \"`furnitureEcoFeeCommission`\" - \"`copyPrivateFeeCommission`\" - \"`principalRefund`\" - \"`principalRefundTax`\" - \"`itemCommission`\" - \"`adjustmentCommission`\" - \"`shippingFeeCommission`\" - \"`commissionRefund`\" - \"`damaged`\" - \"`damagedOrDefectiveItem`\" - \"`expiredItem`\" - \"`faultyItem`\" - \"`incorrectItemReceived`\" - \"`itemMissing`\" - \"`qualityNotExpected`\" - \"`receivedTooLate`\" - \"`storePackageMissing`\" - \"`transitPackageMissing`\" - \"`unsuccessfulDeliveryUndeliverable`\" - \"`wrongChargeInStore`\" - \"`wrongItem`\" - \"`returns`\" - \"`undeliverable`\" - \"`refundFromMerchant`\" - \"`returnLabelShippingFee`\" - \"`pspFee`\" "]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "transactionAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that contributes to the line item price."]
    pub transaction_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the amount. Acceptable values are: - \"`itemPrice`\" - \"`orderPrice`\" - \"`refund`\" - \"`earlyRefund`\" - \"`courtesyRefund`\" - \"`returnRefund`\" - \"`returnLabelShippingFeeAmount`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementTransactionAmountCommission {
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The category of the commission. Acceptable values are: - \"`animalsAndPetSupplies`\" - \"`dogCatFoodAndCatLitter`\" - \"`apparelAndAccessories`\" - \"`shoesHandbagsAndSunglasses`\" - \"`costumesAndAccessories`\" - \"`jewelry`\" - \"`watches`\" - \"`hobbiesArtsAndCrafts`\" - \"`homeAndGarden`\" - \"`entertainmentCollectibles`\" - \"`collectibleCoins`\" - \"`sportsCollectibles`\" - \"`sportingGoods`\" - \"`toysAndGames`\" - \"`musicalInstruments`\" - \"`giftCards`\" - \"`babyAndToddler`\" - \"`babyFoodWipesAndDiapers`\" - \"`businessAndIndustrial`\" - \"`camerasOpticsAndPhotography`\" - \"`consumerElectronics`\" - \"`electronicsAccessories`\" - \"`personalComputers`\" - \"`videoGameConsoles`\" - \"`foodAndGrocery`\" - \"`beverages`\" - \"`tobaccoProducts`\" - \"`furniture`\" - \"`hardware`\" - \"`buildingMaterials`\" - \"`tools`\" - \"`healthAndPersonalCare`\" - \"`beauty`\" - \"`householdSupplies`\" - \"`kitchenAndDining`\" - \"`majorAppliances`\" - \"`luggageAndBags`\" - \"`media`\" - \"`officeSupplies`\" - \"`softwareAndVideoGames`\" - \"`vehiclePartsAndAccessories`\" - \"`vehicleTiresAndWheels`\" - \"`vehicles`\" - \"`everythingElse`\" "]
    pub category: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Rate of the commission in percentage."]
    pub rate: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementTransactionIdentifiers {
    #[serde(rename = "adjustmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the adjustments, if it is available."]
    pub adjustment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The merchant provided order ID."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The identifier of the item."]
    pub order_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "settlementEntryId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of the settlement transaction entry."]
    pub settlement_entry_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipment ids for the item."]
    pub shipment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The Google transaction ID."]
    pub transaction_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementTransactionTransaction {
    #[serde(rename = "postDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The time on which the event occurred in ISO 8601 format."]
    pub post_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the transaction that occurred. Acceptable values are: - \"`order`\" - \"`reversal`\" - \"`orderRefund`\" - \"`reversalRefund`\" - \"`issueRelatedRefundAndReplacement`\" - \"`returnLabelShippingFeeTransaction`\" - \"`reversalIssueRelatedRefundAndReplacement`\" - \"`reversalReturnLabelShippingFeeTransaction`\" - \"`lumpSumCorrectionTransaction`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementreportsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#settlementreportsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of returns."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SettlementReport>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SettlementtransactionsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#settlementtransactionsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of returns."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SettlementTransaction>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipmentInvoice {
    #[serde(rename = "invoiceSummary")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Invoice summary."]
    pub invoice_summary: ::std::option::Option<::std::boxed::Box<InvoiceSummary>>,
    #[serde(rename = "lineItemInvoices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Invoice details per line item."]
    pub line_item_invoices:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentInvoiceLineItemInvoice>>>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipmentInvoiceLineItemInvoice {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the line item. Either lineItemId or productId must be set."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the product. This is the REST ID used in the products service. Either lineItemId or productId must be set."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentUnitIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] The shipment unit ID is assigned by the merchant and defines individual quantities within a line item. The same ID can be assigned to units that are the same while units that differ must be assigned a different ID (for example: free or promotional units)."]
    pub shipment_unit_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "unitInvoice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Invoice details for a single unit."]
    pub unit_invoice: ::std::option::Option<::std::boxed::Box<UnitInvoice>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShipmentTrackingInfo {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The shipping carrier that handles the package. Acceptable values are: - \"`boxtal`\" - \"`bpost`\" - \"`chronopost`\" - \"`colisPrive`\" - \"`colissimo`\" - \"`cxt`\" - \"`deliv`\" - \"`dhl`\" - \"`dpd`\" - \"`dynamex`\" - \"`eCourier`\" - \"`easypost`\" - \"`efw`\" - \"`fedex`\" - \"`fedexSmartpost`\" - \"`geodis`\" - \"`gls`\" - \"`googleCourier`\" - \"`gsx`\" - \"`jdLogistics`\" - \"`laPoste`\" - \"`lasership`\" - \"`manual`\" - \"`mpx`\" - \"`onTrac`\" - \"`other`\" - \"`tnt`\" - \"`uds`\" - \"`ups`\" - \"`usps`\" "]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "trackingNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The tracking number for the package."]
    pub tracking_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The merchant account's shipping settings. All methods except getsupportedcarriers and getsupportedholidays require the admin role."]
pub struct ShippingSettings {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCodeGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of postal code groups that can be referred to in `services`. Optional."]
    pub postal_code_groups:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCodeGroup>>>,
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The target account's list of services. Optional."]
    pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ShippingsettingsCustomBatchRequestEntry>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch shippingsettings request."]
pub struct ShippingsettingsCustomBatchRequestEntry {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the account for which to get/update account shipping settings."]
    pub account_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`update`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shippingSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The account shipping settings to update. Only defined if the method is `update`."]
    pub shipping_settings: ::std::option::Option<::std::boxed::Box<ShippingSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<ShippingsettingsCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch shipping settings response."]
pub struct ShippingsettingsCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry to which this entry responds."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if, and only if, the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#shippingsettingsCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shippingSettings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved or updated account shipping settings."]
    pub shipping_settings: ::std::option::Option<::std::boxed::Box<ShippingSettings>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsGetSupportedCarriersResponse {
    #[serde(rename = "carriers")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of supported carriers. May be empty."]
    pub carriers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CarriersCarrier>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedCarriersResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsGetSupportedHolidaysResponse {
    #[serde(rename = "holidays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of holidays applicable for delivery guarantees. May be empty."]
    pub holidays: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HolidaysHoliday>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedHolidaysResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsGetSupportedPickupServicesResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedPickupServicesResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupServices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of supported pickup services. May be empty."]
    pub pickup_services:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PickupServicesPickupService>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShippingsettingsListResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsListResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The token for the retrieval of the next page of shipping settings."]
    pub next_page_token: ::std::option::Option<::std::string::String>,
    #[serde(rename = "resources")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShippingSettings>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    #[serde(rename = "columnHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Headers of the table's columns. Optional: if not set then the table has only one dimension."]
    pub column_headers: ::std::option::Option<::std::boxed::Box<Headers>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the table. Required for subtables, ignored for the main table."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "rowHeaders")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Headers of the table's rows. Required."]
    pub row_headers: ::std::option::Option<::std::boxed::Box<Headers>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of rows that constitute the table. Must have the same length as `rowHeaders`. Required."]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrder {
    #[serde(rename = "deliveryDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overrides the predefined delivery details if provided."]
    pub delivery_details: ::std::option::Option<::std::boxed::Box<TestOrderDeliveryDetails>>,
    #[serde(rename = "enableOrderinvoices")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the orderinvoices service should support this order."]
    pub enable_orderinvoices: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#testOrder`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Line items that are ordered. At least one line item must be provided."]
    pub line_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestOrderLineItem>>>,
    #[serde(rename = "notificationMode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Restricted. Do not use."]
    pub notification_mode: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Overrides the predefined pickup details if provided."]
    pub pickup_details: ::std::option::Option<::std::boxed::Box<TestOrderPickupDetails>>,
    #[serde(rename = "predefinedBillingAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The billing address. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_billing_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedDeliveryAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifier of one of the predefined delivery addresses for the delivery. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_delivery_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Email address of the customer. Acceptable values are: - \"`pog.dwight.schrute@gmail.com`\" - \"`pog.jim.halpert@gmail.com`\" - \"`penpog.pam.beesly@gmail.comding`\" "]
    pub predefined_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedPickupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of one of the predefined pickup details. Required for orders containing line items with shipping type `pickup`. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_pickup_details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Promotions associated with the order."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPromotion>>>,
    #[serde(rename = "shippingCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price of shipping for all items. Shipping tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied. Note that shipping is not taxed in certain states."]
    pub shipping_cost: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shippingOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The requested shipping option. Acceptable values are: - \"`economy`\" - \"`expedited`\" - \"`oneDay`\" - \"`sameDay`\" - \"`standard`\" - \"`twoDay`\" "]
    pub shipping_option: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderAddress {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "CLDR country code (e.g. \"US\")."]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fullAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Strings representing the lines of the printed label for mailing the order, for example: John Smith 1600 Amphitheatre Parkway Mountain View, CA, 94043 United States "]
    pub full_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isPostOfficeBox")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the address is a post office box."]
    pub is_post_office_box: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "locality")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
    pub locality: ::std::option::Option<::std::string::String>,
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Postal Code or ZIP (e.g. \"94043\")."]
    pub postal_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "recipientName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Name of the recipient."]
    pub recipient_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
    pub region: ::std::option::Option<::std::string::String>,
    #[serde(rename = "streetAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Street-level part of the address."]
    pub street_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderDeliveryDetails {
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The delivery address"]
    pub address: ::std::option::Option<::std::boxed::Box<TestOrderAddress>>,
    #[serde(rename = "isScheduledDelivery")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the order is scheduled delivery order."]
    pub is_scheduled_delivery: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The phone number of the person receiving the delivery."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderLineItem {
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Product data from the time of the order placement."]
    pub product: ::std::option::Option<::std::boxed::Box<TestOrderLineItemProduct>>,
    #[serde(rename = "quantityOrdered")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Number of items ordered."]
    pub quantity_ordered: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "returnInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Details of the return policy for the line item."]
    pub return_info: ::std::option::Option<::std::boxed::Box<OrderLineItemReturnInfo>>,
    #[serde(rename = "shippingDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Details of the requested shipping for the line item."]
    pub shipping_details: ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetails>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderLineItemProduct {
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Brand of the item."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Condition or state of the item. Acceptable values are: - \"`new`\" "]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The two-letter ISO 639-1 language code for the item. Acceptable values are: - \"`en`\" - \"`fr`\" "]
    pub content_language: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fees")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Fees for the item. Optional."]
    pub fees: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItemProductFee>>>,
    #[serde(rename = "gtin")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Global Trade Item Number (GTIN) of the item. Optional."]
    pub gtin: ::std::option::Option<::std::string::String>,
    #[serde(rename = "imageLink")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. URL of an image of the item."]
    pub image_link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "itemGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shared identifier for all variants of the same product. Optional."]
    pub item_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mpn")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Manufacturer Part Number (MPN) of the item. Optional."]
    pub mpn: ::std::option::Option<::std::string::String>,
    #[serde(rename = "offerId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. An identifier of the item."]
    pub offer_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price for the product. Tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory // code of the target country of the product."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The title of the product."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "variantAttributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Variant attributes for the item. Optional."]
    pub variant_attributes: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrderLineItemProductVariantAttribute>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderPickupDetails {
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Code of the location defined by provider or merchant."]
    pub location_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupLocationAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Pickup location address."]
    pub pickup_location_address: ::std::option::Option<::std::boxed::Box<TestOrderAddress>>,
    #[serde(rename = "pickupLocationType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Pickup location type. Acceptable values are: - \"`locker`\" - \"`store`\" - \"`curbside`\" "]
    pub pickup_location_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupPersons")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. all pickup persons set by users."]
    pub pickup_persons: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<TestOrderPickupDetailsPickupPerson>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderPickupDetailsPickupPerson {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Full name of the pickup person."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The phone number of the person picking up the items."]
    pub phone_number: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Represents a time zone from the [IANA Time Zone Database](https://www.iana.org/time-zones)."]
pub struct TimeZone {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IANA Time Zone Database time zone, e.g. \"America/New_York\"."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. IANA Time Zone Database version number, e.g. \"2019a\"."]
    pub version: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitTable {
    #[serde(rename = "postalCodeGroupNames")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of postal group names. The last value can be `\"all other locations\"`. Example: `[\"zone 1\", \"zone 2\", \"all other locations\"]`. The referred postal code groups must match the delivery country of the service."]
    pub postal_code_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "rows")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitTableTransitTimeRow>>>,
    #[serde(rename = "transitTimeLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of transit time labels. The last value can be `\"all other labels\"`. Example: `[\"food\", \"electronics\", \"all other labels\"]`."]
    pub transit_time_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitTableTransitTimeRow {
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<TransitTableTransitTimeRowTransitTimeValue>>,
    >,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitTableTransitTimeRowTransitTimeValue {
    #[serde(rename = "maxTransitTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Must be greater than or equal to `minTransitTimeInDays`."]
    pub max_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "minTransitTimeInDays")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Transit time range (min-max) in business days. 0 means same day delivery, 1 means next day delivery."]
    pub min_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitInvoice {
    #[serde(rename = "additionalCharges")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional charges for a unit, e.g. shipping costs."]
    pub additional_charges:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnitInvoiceAdditionalCharge>>>,
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Pre-tax or post-tax price of the unit depending on the locality of the order."]
    pub unit_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "unitPriceTaxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amounts to apply to the unit price."]
    pub unit_price_taxes:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnitInvoiceTaxLine>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitInvoiceAdditionalCharge {
    #[serde(rename = "additionalChargeAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Amount of the additional charge."]
    pub additional_charge_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Type of the additional charge. Acceptable values are: - \"`shipping`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnitInvoiceTaxLine {
    #[serde(rename = "taxAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Tax amount for the tax type."]
    pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "taxName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional name of the tax type. This should only be provided if `taxType` is `otherFeeTax`."]
    pub tax_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Type of the tax. Acceptable values are: - \"`otherFee`\" - \"`otherFeeTax`\" - \"`sales`\" "]
    pub tax_type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "The single value of a rate group or the value of a rate group table's cell. Exactly one of `noShipping`, `flatRate`, `pricePercentage`, `carrierRateName`, `subtableName` must be set."]
pub struct Value {
    #[serde(rename = "carrierRateName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a carrier rate referring to a carrier rate defined in the same rate group. Can only be set if all other fields are not set."]
    pub carrier_rate_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "flatRate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A flat rate. Can only be set if all other fields are not set."]
    pub flat_rate: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "noShipping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "If true, then the product can't ship. Must be true when set, can only be set if all other fields are not set."]
    pub no_shipping: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pricePercentage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A percentage of the price represented as a number in decimal notation (e.g., `\"5.4\"`). Can only be set if all other fields are not set."]
    pub price_percentage: ::std::option::Option<::std::string::String>,
    #[serde(rename = "subtableName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of a subtable. Can only be set in table cells (i.e., not for single values), and only if all other fields are not set."]
    pub subtable_name: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The weight unit. Acceptable values are: - \"`kg`\" - \"`lb`\" "]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The weight represented as a number."]
    pub value: ::std::option::Option<::std::string::String>,
}
