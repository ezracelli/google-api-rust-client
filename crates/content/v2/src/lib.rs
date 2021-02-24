#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "Account data. After the creation of a new account it may take a few minutes before it is fully operational. The methods delete, insert, and update require the admin role."]
pub struct Account {
    #[serde(rename = "adultContent")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates whether the merchant sells adult content."]
    pub adult_content: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "adwordsLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list."]
    pub adwords_links:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountAdwordsLink>>>,
    #[serde(rename = "businessInformation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The business information of the account."]
    pub business_information: ::std::option::Option<::std::boxed::Box<AccountBusinessInformation>>,
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
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Display name for the account."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reviewsUrl")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] This field is never returned and will be ignored if provided."]
    #[deprecated]
    pub reviews_url: ::std::option::Option<::std::string::String>,
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
    #[doc = "List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list."]
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
pub struct AccountAdwordsLink {
    #[serde(rename = "adwordsId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer ID of the AdWords account."]
    pub adwords_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Status of the link between this Merchant Center account and the AdWords account. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in Google AdWords or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending. Acceptable values are: - \"`active`\" - \"`pending`\" "]
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
    #[serde(rename = "dataQualityIssues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED - never populated."]
    pub data_quality_issues:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusDataQualityIssue>>>,
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
pub struct AccountStatusDataQualityIssue {
    #[serde(rename = "country")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub displayed_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "exampleItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub example_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusExampleItem>>>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastChecked")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub last_checked: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "numItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub num_items: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = " Acceptable values are: - \"`critical`\" - \"`error`\" - \"`suggestion`\" "]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "submittedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub submitted_value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountStatusExampleItem {
    #[serde(rename = "itemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "link")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(rename = "submittedValue")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub submitted_value: ::std::option::Option<::std::string::String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueOnLandingPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value_on_landing_page: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "linkStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. This field is never set. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
    pub link_status: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "pretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Value before taxes."]
    pub pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "tax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Tax value."]
    pub tax: ::std::option::Option<::std::boxed::Box<Price>>,
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
pub struct CustomAttribute {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the attribute. Underscores will be replaced by spaces upon insertion."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of the attribute. Acceptable values are: - \"`boolean`\" - \"`datetimerange`\" - \"`float`\" - \"`group`\" - \"`int`\" - \"`price`\" - \"`text`\" - \"`time`\" - \"`url`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Free-form unit of the attribute. Unit can only be used for values of type int, float, or price."]
    pub unit: ::std::option::Option<::std::string::String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The value of the attribute."]
    pub value: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CustomGroup {
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sub-attributes."]
    pub attributes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the group. Underscores will be replaced by spaces upon insertion."]
    pub name: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`."]
    #[deprecated]
    pub content_language: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "intendedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center)."]
    #[deprecated]
    pub intended_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#datafeed`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for insert. A descriptive name of the data feed."]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code."]
    #[deprecated]
    pub target_country: ::std::option::Option<::std::string::String>,
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
pub struct Inventory {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availability of the product. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
    pub availability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel0")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label0: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel4")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label4: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number and amount of installments to pay for an item. Brazil only."]
    pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
    #[serde(rename = "instoreProductLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instore product location. Supported only for local products."]
    pub instore_product_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#inventory`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loyaltyPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
    pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
    #[serde(rename = "pickup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Store pickup information. Only supported for local inventory. Not setting `pickup` means \"don't update\" while setting it to the empty value (`{}` in JSON) means \"delete\". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is \"not supported\"."]
    pub pickup: ::std::option::Option<::std::boxed::Box<InventoryPickup>>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price of the product."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the product. Must be equal to or greater than zero. Supported only for local products."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "salePrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
    pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "salePriceEffectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided."]
    pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellOnGoogleQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
    pub sell_on_google_quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<InventoryCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch inventory request."]
pub struct InventoryCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "inventory")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price and availability of the product."]
    pub inventory: ::std::option::Option<::std::boxed::Box<Inventory>>,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product for which to update price and availability."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "storeCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The code of the store for which to update price and availability. Use `online` to update price and availability of an online product."]
    pub store_code: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<InventoryCustomBatchResponseEntry>>,
    >,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#inventoryCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
#[doc = "A batch entry encoding a single non-batch inventory response."]
pub struct InventoryCustomBatchResponseEntry {
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
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#inventoryCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventoryPickup {
    #[serde(rename = "pickupMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether store pickup is available for this offer and whether the pickup option should be shown as buy, reserve, or not supported. Only supported for local inventory. Unless the value is \"not supported\", must be submitted together with `pickupSla`. Acceptable values are: - \"`buy`\" - \"`not supported`\" - \"`reserve`\" - \"`ship to store`\" "]
    pub pickup_method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pickupSla")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The expected date that an order will be ready for pickup, relative to when the order is placed. Only supported for local inventory. Must be submitted together with `pickupMethod`. Acceptable values are: - \"`five day`\" - \"`four day`\" - \"`multi day`\" - \"`multi week`\" - \"`next day`\" - \"`same day`\" - \"`seven day`\" - \"`six day`\" - \"`three day`\" - \"`two day`\" "]
    pub pickup_sla: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySetRequest {
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The availability of the product. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
    pub availability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel0")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label0: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel1")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label1: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel2")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label2: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel3")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label3: ::std::option::Option<::std::string::String>,
    #[serde(rename = "customLabel4")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
    pub custom_label4: ::std::option::Option<::std::string::String>,
    #[serde(rename = "installment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Number and amount of installments to pay for an item. Brazil only."]
    pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
    #[serde(rename = "instoreProductLocation")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The instore product location. Supported only for local products."]
    pub instore_product_location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "loyaltyPoints")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
    pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
    #[serde(rename = "pickup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Store pickup information. Only supported for local inventory. Not setting `pickup` means \"don't update\" while setting it to the empty value (`{}` in JSON) means \"delete\". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is \"not supported\"."]
    pub pickup: ::std::option::Option<::std::boxed::Box<InventoryPickup>>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The price of the product."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the product. Must be equal to or greater than zero. Supported only for local products."]
    pub quantity: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "salePrice")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
    pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "salePriceEffectiveDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided."]
    pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sellOnGoogleQuantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
    pub sell_on_google_quantity: ::std::option::Option<::std::primitive::i64>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InventorySetResponse {
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#inventorySetResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "customerBalance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub customer_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "googleBalance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub google_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "merchantBalance")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub merchant_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "productTotal")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Total price for the product."]
    pub product_total: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "promotionSummaries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub promotion_summaries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
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
    #[serde(rename = "merchantReturnReason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason that merchant chooses to accept a return item."]
    pub merchant_return_reason: ::std::option::Option<::std::boxed::Box<RefundReason>>,
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product data from the time of the order placement."]
    pub product: ::std::option::Option<::std::boxed::Box<OrderLineItemProduct>>,
    #[serde(rename = "returnShipmentIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "IDs of the return shipments that this return item belongs to."]
    pub return_shipment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "State of the item. Acceptable values are: - \"`canceled`\" - \"`new`\" - \"`received`\" - \"`refunded`\" - \"`rejected`\" "]
    pub state: ::std::option::Option<::std::string::String>,
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
#[doc = "Order. Production access (all methods) requires the order manager role. Sandbox access does not."]
pub struct Order {
    #[serde(rename = "acknowledged")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the order was acknowledged."]
    pub acknowledged: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "channelType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Acceptable values are: - \"`googleExpress`\" - \"`purchasesOnGoogle`\" "]
    pub channel_type: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80."]
    pub net_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the payment method."]
    pub payment_method: ::std::option::Option<::std::boxed::Box<OrderPaymentMethod>>,
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
    #[doc = "The details of the merchant provided promotions applied to the order. To determine which promotions apply to which products, check the `Promotions[].Benefits[].OfferIds` field against the `LineItems[].Product.OfferId` field for each promotion. If a promotion is applied to more than 1 `offerId`, divide the discount value by the number of affected offers to determine how much discount to apply to each `offerId`. Examples: 1. To calculate the line item level discount for a single specific item: For each promotion, subtract the `Promotions[].Benefits[].Discount.value` amount from the `LineItems[].Price.value`. 2. To calculate the line item level discount for multiple quantity of a specific item: For each promotion, divide the `Promotions[].Benefits[].Discount.value` by the quantity of products and substract it from `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotion>>>,
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
    #[serde(rename = "shippingOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Shipping details are provided with line items instead. Acceptable values are: - \"`economy`\" - \"`expedited`\" - \"`oneDay`\" - \"`sameDay`\" - \"`standard`\" - \"`twoDay`\" "]
    pub shipping_option: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explicitMarketingPreference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use marketingRightsInfo instead."]
    pub explicit_marketing_preference: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full name of the customer."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "invoiceReceivingEmail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Email address for the merchant to send value-added tax or invoice documentation of the order. Only the last document sent is made available to the customer. For more information, see About automated VAT invoicing for Buy on Google."]
    pub invoice_receiving_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketingRightsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer's marketing preferences. Contains the marketing opt-in information that is current at the time that the merchant call. User preference selections can change from one order to the next so preferences must be checked with every order."]
    pub marketing_rights_info:
        ::std::option::Option<::std::boxed::Box<OrderCustomerMarketingRightsInfo>>,
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
pub struct OrderLegacyPromotion {
    #[serde(rename = "benefits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub benefits:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotionBenefit>>>,
    #[serde(rename = "effectiveDates")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The date and time frame when the promotion is active and ready for validation review. Note that the promotion live time may be delayed for a few hours due to the validation review. Start date and end date are separated by a forward slash (/). The start date is specified by the format (YYYY-MM-DD), followed by the letter ?T?, the time of the day when the sale starts (in Greenwich Mean Time, GMT), followed by an expression of the time zone for the sale. The end date is in the same format."]
    pub effective_dates: ::std::option::Option<::std::string::String>,
    #[serde(rename = "genericRedemptionCode")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Optional. The text code that corresponds to the promotion when applied on the retailer?s website."]
    pub generic_redemption_code: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The unique ID of the promotion."]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "longTitle")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The full title of the promotion."]
    pub long_title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productApplicability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the promotion is applicable to all products or only specific products. Acceptable values are: - \"`allProducts`\" - \"`specificProducts`\" "]
    pub product_applicability: ::std::option::Option<::std::string::String>,
    #[serde(rename = "redemptionChannel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Indicates that the promotion is valid online. Acceptable values are: - \"`online`\" "]
    pub redemption_channel: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLegacyPromotionBenefit {
    #[serde(rename = "discount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The discount in the order price when the promotion is applied."]
    pub discount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "offerIds")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The OfferId(s) that were purchased in this order and map to this specific benefit of the promotion."]
    pub offer_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "subType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Further describes the benefit of the promotion. Note that we will expand on this enumeration as we support new promotion sub-types. Acceptable values are: - \"`buyMGetMoneyOff`\" - \"`buyMGetNMoneyOff`\" - \"`buyMGetNPercentOff`\" - \"`buyMGetPercentOff`\" - \"`freeGift`\" - \"`freeGiftWithItemId`\" - \"`freeGiftWithValue`\" - \"`freeOvernightShipping`\" - \"`freeShipping`\" - \"`freeTwoDayShipping`\" - \"`moneyOff`\" - \"`percentageOff`\" - \"`rewardPoints`\" - \"`salePrice`\" "]
    pub sub_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxImpact")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The impact on tax when the promotion is applied."]
    pub tax_impact: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Describes whether the promotion applies to products (e.g. 20% off) or to shipping (e.g. Free Shipping). Acceptable values are: - \"`product`\" - \"`shipping`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrderLineItem {
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
pub struct OrderLineItemProduct {
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Brand of the item."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item's channel (online or local). Acceptable values are: - \"`local`\" - \"`online`\" "]
    pub channel: ::std::option::Option<::std::string::String>,
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
pub struct OrderPaymentMethod {
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billing address."]
    pub billing_address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
    #[serde(rename = "expirationMonth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The card expiration month (January = 1, February = 2 etc.)."]
    pub expiration_month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "expirationYear")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The card expiration year (4-digit, e.g. 2015)."]
    pub expiration_year: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "lastFourDigits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last four digits of the card number."]
    pub last_four_digits: ::std::option::Option<::std::string::String>,
    #[serde(rename = "phoneNumber")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billing phone number."]
    pub phone_number: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of instrument. Acceptable values are: - \"`AMEX`\" - \"`DISCOVER`\" - \"`JCB`\" - \"`MASTERCARD`\" - \"`UNIONPAY`\" - \"`VISA`\" - \"``\" "]
    pub _type: ::std::option::Option<::std::string::String>,
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
    pub product_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "productAmountWithRemittedTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Total amount with remitted tax for the items."]
    pub product_amount_with_remitted_tax: ::std::option::Option<::std::boxed::Box<ProductAmount>>,
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
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
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
pub struct OrdersCustomBatchRequest {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The request entries to be processed in the batch."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntry>>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "An entry ID, unique within the batch request."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "cancel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `cancel` method."]
    pub cancel: ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryCancel>>,
    #[serde(rename = "cancelLineItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `cancelLineItem` method."]
    pub cancel_line_item:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryCancelLineItem>>,
    #[serde(rename = "inStoreRefundLineItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `inStoreReturnLineItem` method."]
    pub in_store_refund_line_item: ::std::option::Option<
        ::std::boxed::Box<OrdersCustomBatchRequestEntryInStoreRefundLineItem>,
    >,
    #[serde(rename = "merchantId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the managing account."]
    pub merchant_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "merchantOrderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The merchant order ID. Required for `updateMerchantOrderId` and `getByMerchantOrderId` methods."]
    pub merchant_order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The method of the batch entry. Acceptable values are: - \"`acknowledge`\" - \"`cancel`\" - \"`cancelLineItem`\" - \"`get`\" - \"`getByMerchantOrderId`\" - \"`inStoreRefundLineItem`\" - \"`refund`\" - \"`rejectReturnLineItem`\" - \"`returnLineItem`\" - \"`returnRefundLineItem`\" - \"`setLineItemMetadata`\" - \"`shipLineItems`\" - \"`updateLineItemShippingDetails`\" - \"`updateMerchantOrderId`\" - \"`updateShipment`\" "]
    pub method: ::std::option::Option<::std::string::String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order. Required for all methods beside `get` and `getByMerchantOrderId`."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the order. Required for all methods beside `getByMerchantOrderId`."]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "refund")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `refund` method."]
    pub refund: ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryRefund>>,
    #[serde(rename = "rejectReturnLineItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `rejectReturnLineItem` method."]
    pub reject_return_line_item:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryRejectReturnLineItem>>,
    #[serde(rename = "returnLineItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `returnLineItem` method."]
    pub return_line_item:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryReturnLineItem>>,
    #[serde(rename = "returnRefundLineItem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `returnRefundLineItem` method."]
    pub return_refund_line_item:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryReturnRefundLineItem>>,
    #[serde(rename = "setLineItemMetadata")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `setLineItemMetadata` method."]
    pub set_line_item_metadata:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntrySetLineItemMetadata>>,
    #[serde(rename = "shipLineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `shipLineItems` method."]
    pub ship_line_items:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItems>>,
    #[serde(rename = "updateLineItemShippingDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `updateLineItemShippingDate` method."]
    pub update_line_item_shipping_details: ::std::option::Option<
        ::std::boxed::Box<OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails>,
    >,
    #[serde(rename = "updateShipment")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required for `updateShipment` method."]
    pub update_shipment:
        ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryUpdateShipment>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryCancel {
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
pub struct OrdersCustomBatchRequestEntryCancelLineItem {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to cancel. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
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
pub struct OrdersCustomBatchRequestEntryInStoreRefundLineItem {
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. Required."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that correspond to refund amount in amountPretax. Required."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
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
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryRefund {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. Either amount or amountPretax should be filled."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`deliveredLateByCarrier`\" - \"`feeAdjustment`\" - \"`lateShipmentCredit`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`shippingCostAdjustment`\" - \"`taxAdjustment`\" - \"`undeliverableShippingAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryRejectReturnLineItem {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
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
pub struct OrdersCustomBatchRequestEntryReturnLineItem {
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "quantity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The quantity to return."]
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
pub struct OrdersCustomBatchRequestEntryReturnRefundLineItem {
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method)."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
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
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntrySetLineItemMetadata {
    #[serde(rename = "annotations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub annotations:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "productId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
    pub product_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchRequestEntryShipLineItems {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItems")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Line items to ship."]
    pub line_items:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>>,
    #[serde(rename = "shipmentGroupId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "ID of the shipment group. Required for orders that use the orderinvoices service."]
    pub shipment_group_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The ID of the shipment."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentInfos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs)."]
    pub shipment_infos: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    >,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
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
pub struct OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {
    #[serde(rename = "deliverByDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
    pub deliver_by_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lineItemId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
    pub line_item_id: ::std::option::Option<::std::string::String>,
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
pub struct OrdersCustomBatchRequestEntryUpdateShipment {
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
    #[serde(rename = "deliveryDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`."]
    pub delivery_date: ::std::option::Option<::std::string::String>,
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
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchResponse {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The result of the execution of the batch requests."]
    pub entries:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchResponseEntry>>>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCustomBatchResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersCustomBatchResponseEntry {
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the request entry this entry responds to."]
    pub batch_id: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of errors defined if and only if the request failed."]
    pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Only defined if 1. the request was successful; and 2. the method is not `get`, `getByMerchantOrderId`, or one of the test methods. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#ordersCustomBatchResponseEntry`\""]
    pub kind: ::std::option::Option<::std::string::String>,
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The retrieved order. Only defined if the method is `get` and if the request was successful."]
    pub order: ::std::option::Option<::std::boxed::Box<Order>>,
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
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. Required."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that correspond to refund amount in amountPretax. Required."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
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
    #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
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
pub struct OrdersRefundRequest {
    #[serde(rename = "amount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
    pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. Either amount or amountPretax should be filled."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`deliveredLateByCarrier`\" - \"`feeAdjustment`\" - \"`lateShipmentCredit`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`shippingCostAdjustment`\" - \"`taxAdjustment`\" - \"`undeliverableShippingAddress`\" - \"`wrongProductShipped`\" "]
    pub reason: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reasonText")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The explanation of the reason."]
    pub reason_text: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersRefundResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRefundResponse\"."]
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
pub struct OrdersReturnLineItemRequest {
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
    #[doc = "The quantity to return."]
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
pub struct OrdersReturnLineItemResponse {
    #[serde(rename = "executionStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
    pub execution_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersReturnLineItemResponse\"."]
    pub kind: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OrdersReturnRefundLineItemRequest {
    #[serde(rename = "amountPretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method)."]
    pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "amountTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
    pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
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
    #[serde(rename = "carrier")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
    pub carrier: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "shipmentId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The ID of the shipment."]
    pub shipment_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shipmentInfos")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs)."]
    pub shipment_infos: ::std::option::Option<
        ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>>,
    >,
    #[serde(rename = "trackingId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment."]
    pub tracking_id: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The ID of the operation. Unique across all operations for a given order."]
    pub operation_id: ::std::option::Option<::std::string::String>,
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
#[doc = " Required product attributes are primarily defined by the products data specification. See the Products Data Specification Help Center article for information. Some attributes are country-specific, so make sure you select the appropriate country in the drop-down selector at the top of the page. Product data. After inserting, updating, or deleting a product, it may take several minutes before changes take effect."]
pub struct Product {
    #[serde(rename = "additionalImageLinks")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional URLs of images of the item."]
    pub additional_image_links: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "additionalProductTypes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Additional categories of the item (formatted as in products data specification)."]
    pub additional_product_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "adult")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Should be set to true if the item is targeted towards adults."]
    pub adult: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "adwordsGrouping")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise."]
    pub adwords_grouping: ::std::option::Option<::std::string::String>,
    #[serde(rename = "adwordsLabels")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Similar to adwords_grouping, but only works on CPC."]
    pub adwords_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "adwordsRedirect")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Allows advertisers to override the item URL when the product is shown within the context of Product Ads."]
    pub adwords_redirect: ::std::option::Option<::std::string::String>,
    #[serde(rename = "ageGroup")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target age group of the item. Acceptable values are: - \"`adult`\" - \"`infant`\" - \"`kids`\" - \"`newborn`\" - \"`toddler`\" - \"`youngAdult`\" "]
    pub age_group: ::std::option::Option<::std::string::String>,
    #[serde(rename = "aspects")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Do not use."]
    pub aspects: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductAspect>>>,
    #[serde(rename = "availability")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Availability status of the item. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
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
    #[doc = "Condition or state of the item. Acceptable values are: - \"`new`\" - \"`refurbished`\" - \"`used`\" "]
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
    #[serde(rename = "customGroups")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "A list of custom (merchant-provided) custom attribute groups."]
    pub custom_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomGroup>>>,
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
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Specifies the intended destinations for the product."]
    pub destinations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductDestination>>>,
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
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
    pub energy_efficiency_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future."]
    pub expiration_date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "gender")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Target gender of the item. Acceptable values are: - \"`female`\" - \"`male`\" - \"`unisex`\" "]
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
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
    pub max_energy_efficiency_class: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxHandlingTime")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Maximal product handling time (in business days)."]
    pub max_handling_time: ::std::option::Option<::std::string::String>,
    #[serde(rename = "minEnergyEfficiencyClass")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
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
    #[serde(rename = "onlineOnly")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub online_only: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "pattern")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The item's pattern (e.g. polka dots)."]
    pub pattern: ::std::option::Option<::std::string::String>,
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Price of the item."]
    pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "productType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Your category of the item (formatted as in products data specification)."]
    pub product_type: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "sizeSystem")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "System in which the size is specified. Recommended for apparel items. Acceptable values are: - \"`AU`\" - \"`BR`\" - \"`CN`\" - \"`DE`\" - \"`EU`\" - \"`FR`\" - \"`IT`\" - \"`JP`\" - \"`MEX`\" - \"`UK`\" - \"`US`\" "]
    pub size_system: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizeType")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The cut of the item. Recommended for apparel items. Acceptable values are: - \"`big and tall`\" - \"`maternity`\" - \"`oversize`\" - \"`petite`\" - \"`plus`\" - \"`regular`\" "]
    pub size_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "sizes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition)."]
    pub sizes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The source of the offer, i.e., how the offer was created. Acceptable values are: - \"`api`\" - \"`crawl`\" - \"`feed`\" "]
    pub source: ::std::option::Option<::std::string::String>,
    #[serde(rename = "targetCountry")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The CLDR territory code for the item."]
    pub target_country: ::std::option::Option<::std::string::String>,
    #[serde(rename = "taxes")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Tax information."]
    pub taxes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductTax>>>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Title of the item."]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "unitPricingBaseMeasure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The preference of the denominator of the unit price."]
    pub unit_pricing_base_measure:
        ::std::option::Option<::std::boxed::Box<ProductUnitPricingBaseMeasure>>,
    #[serde(rename = "unitPricingMeasure")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The measure and dimension of an item."]
    pub unit_pricing_measure: ::std::option::Option<::std::boxed::Box<ProductUnitPricingMeasure>>,
    #[serde(rename = "validatedDestinations")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. The read-only list of intended destinations which passed validation."]
    pub validated_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Read-only warnings."]
    pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
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
pub struct ProductAspect {
    #[serde(rename = "aspectName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub aspect_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub destination_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intention")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub intention: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductDestination {
    #[serde(rename = "destinationName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the destination."]
    pub destination_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intention")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the destination is required, excluded or should be validated. Acceptable values are: - \"`default`\" - \"`excluded`\" - \"`optional`\" - \"`required`\" "]
    pub intention: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "dataQualityIssues")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "DEPRECATED - never populated"]
    pub data_quality_issues:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatusDataQualityIssue>>>,
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
    #[serde(rename = "product")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Product data after applying all the join inputs."]
    pub product: ::std::option::Option<::std::boxed::Box<Product>>,
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
pub struct ProductStatusDataQualityIssue {
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub detail: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fetchStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub fetch_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<::std::string::String>,
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub severity: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueOnLandingPage")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value_on_landing_page: ::std::option::Option<::std::string::String>,
    #[serde(rename = "valueProvided")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub value_provided: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductStatusDestinationStatus {
    #[serde(rename = "approvalPending")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Whether the approval status might change due to further processing."]
    pub approval_pending: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "approvalStatus")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The destination's approval status. Acceptable values are: - \"`approved`\" - \"`disapproved`\" "]
    pub approval_status: ::std::option::Option<::std::string::String>,
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The name of the destination"]
    pub destination: ::std::option::Option<::std::string::String>,
    #[serde(rename = "intention")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Provided for backward compatibility only. Always set to \"required\". Acceptable values are: - \"`default`\" - \"`excluded`\" - \"`optional`\" - \"`required`\" "]
    pub intention: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductStatusItemLevelIssue {
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
pub struct Promotion {
    #[serde(rename = "promotionAmount")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Amount of the promotion. The values here are the promotion applied to the unit price pretax and to the total of the tax amounts."]
    pub promotion_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
    #[serde(rename = "promotionId")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] ID of the promotion."]
    pub promotion_id: ::std::option::Option<::std::string::String>,
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
    #[doc = "Type of the return method. Acceptable values are: - \"`byMail`\" - \"`contactCustomerSupport`\" - \"`returnless`\" "]
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
pub struct Row {
    #[serde(rename = "cells")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The list of cells that constitute the row. Must have the same length as `columnHeaders` for two-dimensional tables, a length of 1 for one-dimensional tables. Required."]
    pub cells: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
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
    #[serde(rename = "customer")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The details of the customer who placed the order."]
    pub customer: ::std::option::Option<::std::boxed::Box<TestOrderCustomer>>,
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
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The details of the payment method."]
    pub payment_method: ::std::option::Option<::std::boxed::Box<TestOrderPaymentMethod>>,
    #[serde(rename = "predefinedDeliveryAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Identifier of one of the predefined delivery addresses for the delivery. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_delivery_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedPickupDetails")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Identifier of one of the predefined pickup details. Required for orders containing line items with shipping type `pickup`. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_pickup_details: ::std::option::Option<::std::string::String>,
    #[serde(rename = "promotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Ignored if provided."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotion>>>,
    #[serde(rename = "shippingCost")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The price of shipping for all items. Shipping tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied. Note that shipping is not taxed in certain states."]
    pub shipping_cost: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shippingCostTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Ignored if provided."]
    pub shipping_cost_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    #[serde(rename = "shippingOption")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. The requested shipping option. Acceptable values are: - \"`economy`\" - \"`expedited`\" - \"`oneDay`\" - \"`sameDay`\" - \"`standard`\" - \"`twoDay`\" "]
    pub shipping_option: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderCustomer {
    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Email address of the customer. Acceptable values are: - \"`pog.dwight.schrute@gmail.com`\" - \"`pog.jim.halpert@gmail.com`\" - \"`penpog.pam.beesly@gmail.comding`\" "]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "explicitMarketingPreference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Please use marketingRightsInfo instead."]
    pub explicit_marketing_preference: ::std::option::Option<::std::primitive::bool>,
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Full name of the customer."]
    pub full_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketingRightsInfo")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Customer's marketing preferences."]
    pub marketing_rights_info:
        ::std::option::Option<::std::boxed::Box<TestOrderCustomerMarketingRightsInfo>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderCustomerMarketingRightsInfo {
    #[serde(rename = "explicitMarketingPreference")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty. Acceptable values are: - \"`denied`\" - \"`granted`\" "]
    pub explicit_marketing_preference: ::std::option::Option<::std::string::String>,
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet."]
    pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "unitTax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Ignored if provided."]
    pub unit_tax: ::std::option::Option<::std::boxed::Box<Price>>,
}
#[derive(Debug, Default, PartialEq, Clone, serde :: Serialize, serde :: Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestOrderLineItemProduct {
    #[serde(rename = "brand")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Required. Brand of the item."]
    pub brand: ::std::option::Option<::std::string::String>,
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated. Acceptable values are: - \"`online`\" "]
    pub channel: ::std::option::Option<::std::string::String>,
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
pub struct TestOrderPaymentMethod {
    #[serde(rename = "expirationMonth")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The card expiration month (January = 1, February = 2 etc.)."]
    pub expiration_month: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "expirationYear")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The card expiration year (4-digit, e.g. 2015)."]
    pub expiration_year: ::std::option::Option<::std::primitive::i64>,
    #[serde(rename = "lastFourDigits")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The last four digits of the card number."]
    pub last_four_digits: ::std::option::Option<::std::string::String>,
    #[serde(rename = "predefinedBillingAddress")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The billing address. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
    pub predefined_billing_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "The type of instrument. Note that real orders might have different values than the four values accepted by `createTestOrder`. Acceptable values are: - \"`AMEX`\" - \"`DISCOVER`\" - \"`MASTERCARD`\" - \"`VISA`\" "]
    pub _type: ::std::option::Option<::std::string::String>,
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
    #[serde(rename = "promotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
    #[serde(rename = "unitPricePretax")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "[required] Price of the unit, before applying taxes."]
    pub unit_price_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
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
    #[serde(rename = "additionalChargePromotions")]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    #[doc = "Deprecated."]
    pub additional_charge_promotions:
        ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
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
