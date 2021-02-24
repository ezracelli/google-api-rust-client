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
pub mod resources {
    pub mod customers {
        pub mod methods {
            pub mod insert {
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
                    #[serde(rename = "customerAuthToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `customerAuthToken` query string is required when creating a resold account that transfers a direct customer's subscription or transfers another reseller customer's subscription to your reseller management. This is a hexadecimal authentication token needed to complete the subscription transfer. For more information, see the administrator help center."]
                    pub customer_auth_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod resellernotify {
        pub mod methods {
            pub mod register {
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
                    #[serde(rename = "serviceAccountEmailAddress")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The service account which will own the created Cloud-PubSub topic."]
                    pub service_account_email_address: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod unregister {
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
                    #[serde(rename = "serviceAccountEmailAddress")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The service account which owns the Cloud-PubSub topic."]
                    pub service_account_email_address: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod subscriptions {
        pub mod methods {
            pub mod delete {
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
                    #[serde(rename = "deletionType")]
                    #[doc = "The `deletionType` query string enables the cancellation, downgrade, or suspension of a subscription."]
                    pub deletion_type: QueryParametersDeletionTypeEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The `deletionType` query string enables the cancellation, downgrade, or suspension of a subscription."]
                pub enum QueryParametersDeletionTypeEnum {
                    #[serde(rename = "deletion_type_undefined")]
                    #[doc = ""]
                    DeletionTypeUndefined,
                    #[serde(rename = "cancel")]
                    #[doc = "Cancels the subscription immediately. This does not apply to a G Suite subscription."]
                    Cancel,
                    #[serde(rename = "transfer_to_direct")]
                    #[doc = "Transfers a subscription directly to Google. The customer is immediately transferred to a direct billing relationship with Google and is given a short amount of time with no service interruption. The customer can then choose to set up billing directly with Google by using a credit card, or they can transfer to another reseller."]
                    TransferToDirect,
                }
                impl ::std::default::Default for QueryParametersDeletionTypeEnum {
                    fn default() -> Self {
                        Self::DeletionTypeUndefined
                    }
                }
            }
            pub mod insert {
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
                    #[serde(rename = "customerAuthToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `customerAuthToken` query string is required when creating a resold account that transfers a direct customer's subscription or transfers another reseller customer's subscription to your reseller management. This is a hexadecimal authentication token needed to complete the subscription transfer. For more information, see the administrator help center."]
                    pub customer_auth_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod list {
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
                    #[serde(rename = "customerAuthToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The `customerAuthToken` query string is required when creating a resold account that transfers a direct customer's subscription or transfers another reseller customer's subscription to your reseller management. This is a hexadecimal authentication token needed to complete the subscription transfer. For more information, see the administrator help center."]
                    pub customer_auth_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customerId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Either the customer's primary domain name or the customer's unique identifier. If using the domain name, we do not recommend using a `customerId` as a key for persistent data. If the domain name for a `customerId` is changed, the Google system automatically updates."]
                    pub customer_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "customerNamePrefix")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "When retrieving all of your subscriptions and filtering for specific customers, you can enter a prefix for a customer name. Using an example customer group that includes `exam.com`, `example20.com` and `example.com`: - `exa` -- Returns all customer names that start with 'exa' which could include `exam.com`, `example20.com`, and `example.com`. A name prefix is similar to using a regular expression's asterisk, exa*. - `example` -- Returns `example20.com` and `example.com`. "]
                    pub customer_name_prefix: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "When retrieving a large list, the `maxResults` is the maximum number of results per page. The `nextPageToken` value takes you to the next page. The default is 20."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Token to specify next page in the list"]
                    pub page_token: ::std::option::Option<::std::string::String>,
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
    #[doc = "JSON template for address of a customer."]
    pub struct Address {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A customer's physical address. An address can be composed of one to three lines. The `addressline2` and `addressLine3` are optional."]
        pub address_line1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line 2 of the address."]
        pub address_line2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "addressLine3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line 3 of the address."]
        pub address_line3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer contact's name. This is required."]
        pub contact_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For `countryCode` information, see the ISO 3166 country code elements. Verify that country is approved for resale of Google products. This property is required when creating a new customer."]
        pub country_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ address_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "address_defaults :: kind")]
        #[doc = "Identifies the resource as a customer address. Value: `customers#address`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An example of a `locality` value is the city of `San Francisco`."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "organizationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The company or company division name. This is required."]
        pub organization_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A `postalCode` example is a postal zip code such as `94043`. This property is required when creating a new customer."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An example of a `region` value is `CA` for the state of California."]
        pub region: ::std::option::Option<::std::string::String>,
    }
    impl Address {
        pub fn builder() -> AddressBuilder {
            AddressBuilder::default()
        }
    }
    mod address_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"customers#address\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for the ChangePlan rpc request."]
    pub struct ChangePlanRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google-issued code (100 char max) for discounted pricing on subscription plans. Deal code must be included in `changePlan` request in order to receive discounted rate. This property is optional. If a deal code has already been added to a subscription, this property may be left empty and the existing discounted rate will still apply (if not empty, only provide the deal code that is already present on the subscription). If a deal code has never been added to a subscription and this property is left blank, regular pricing will apply."]
        pub deal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ change_plan_request_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "change_plan_request_defaults :: kind")]
        #[doc = "Identifies the resource as a subscription change plan request. Value: `subscriptions#changePlanRequest`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "planName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `planName` property is required. This is the name of the subscription's payment plan. For more information about the Google payment plans, see API concepts. Possible values are: - `ANNUAL_MONTHLY_PAY` - The annual commitment plan with monthly payments *Caution: *`ANNUAL_MONTHLY_PAY` is returned as `ANNUAL` in all API responses. - `ANNUAL_YEARLY_PAY` - The annual commitment plan with yearly payments - `FLEXIBLE` - The flexible plan - `TRIAL` - The 30-day free trial plan "]
        pub plan_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is an optional property. This purchase order (PO) information is for resellers to use for their company tracking usage. If a `purchaseOrderId` value is given it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a required property. The seats property is the number of user seat licenses."]
        pub seats: ::std::option::Option<::std::boxed::Box<Seats>>,
    }
    impl ChangePlanRequest {
        pub fn builder() -> ChangePlanRequestBuilder {
            ChangePlanRequestBuilder::default()
        }
    }
    mod change_plan_request_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"subscriptions#changePlanRequest\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "When a Google customer's account is registered with a reseller, the customer's subscriptions for Google services are managed by this reseller. A customer is described by a primary domain name and a physical address."]
    pub struct Customer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternateEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Like the \"Customer email\" in the reseller tools, this email is the secondary contact used if something happens to the customer's service such as service outage or a security issue. This property is required when creating a new customer and should not use the same domain as `customerDomain`."]
        pub alternate_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer's primary domain name string. `customerDomain` is required when creating a new customer. Do not include the `www` prefix in the domain when adding a customer."]
        pub customer_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerDomainVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the customer's primary domain has been verified."]
        pub customer_domain_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This property will always be returned in a response as the unique identifier generated by Google. In a request, this property can be either the primary domain or the unique identifier generated by Google."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ customer_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "customer_defaults :: kind")]
        #[doc = "Identifies the resource as a customer. Value: `reseller#customer`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer contact phone number. Must start with \"+\" followed by the country code. The rest of the number can be contiguous numbers or respect the phone local format conventions, but it must be a real phone number and not, for example, \"123\". This field is silently ignored if invalid."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A customer's address information. Each field has a limit of 255 charcters."]
        pub postal_address: ::std::option::Option<::std::boxed::Box<Address>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUiUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to customer's Admin console dashboard. The read-only URL is generated by the API service. This is used if your client application requires the customer to complete a task in the Admin console."]
        pub resource_ui_url: ::std::option::Option<::std::string::String>,
    }
    impl Customer {
        pub fn builder() -> CustomerBuilder {
            CustomerBuilder::default()
        }
    }
    mod customer_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"reseller#customer\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a subscription renewal settings."]
    pub struct RenewalSettings {
        #[builder(default = "{ renewal_settings_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "renewal_settings_defaults :: kind")]
        #[doc = "Identifies the resource as a subscription renewal setting. Value: `subscriptions#renewalSettings`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renewalType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Renewal settings for the annual commitment plan. For more detailed information, see renewal options in the administrator help center. When renewing a subscription, the `renewalType` is a required property."]
        pub renewal_type: ::std::option::Option<::std::string::String>,
    }
    impl RenewalSettings {
        pub fn builder() -> RenewalSettingsBuilder {
            RenewalSettingsBuilder::default()
        }
    }
    mod renewal_settings_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"subscriptions#renewalSettings\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for resellernotify getwatchdetails response."]
    pub struct ResellernotifyGetwatchdetailsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceAccountEmailAddresses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of registered service accounts."]
        pub service_account_email_addresses:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topic name of the PubSub"]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl ResellernotifyGetwatchdetailsResponse {
        pub fn builder() -> ResellernotifyGetwatchdetailsResponseBuilder {
            ResellernotifyGetwatchdetailsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for resellernotify response."]
    pub struct ResellernotifyResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Topic name of the PubSub"]
        pub topic_name: ::std::option::Option<::std::string::String>,
    }
    impl ResellernotifyResource {
        pub fn builder() -> ResellernotifyResourceBuilder {
            ResellernotifyResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for subscription seats."]
    pub struct Seats {
        #[builder(default = "{ seats_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "seats_defaults :: kind")]
        #[doc = "Identifies the resource as a subscription seat setting. Value: `subscriptions#seats`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "licensedNumberOfSeats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only field containing the current number of users that are assigned a license for the product defined in `skuId`. This field's value is equivalent to the numerical count of users returned by the Enterprise License Manager API method: [`listForProductAndSku`](/admin-sdk/licensing/v1/reference/licenseAssignments/listForProductAndSku)."]
        pub licensed_number_of_seats: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maximumNumberOfSeats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a required property and is exclusive to subscriptions with `FLEXIBLE` or `TRIAL` plans. This property sets the maximum number of licensed users allowed on a subscription. This quantity can be increased up to the maximum limit defined in the reseller's contract. The minimum quantity is the current number of users in the customer account. *Note: *G Suite subscriptions automatically assign a license to every user."]
        pub maximum_number_of_seats: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberOfSeats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a required property and is exclusive to subscriptions with `ANNUAL_MONTHLY_PAY` and `ANNUAL_YEARLY_PAY` plans. This property sets the maximum number of licenses assignable to users on a subscription. The reseller can add more licenses, but once set, the `numberOfSeats` cannot be reduced until renewal. The reseller is invoiced based on the `numberOfSeats` value regardless of how many of these user licenses are assigned. *Note: *G Suite subscriptions automatically assign a license to every user."]
        pub number_of_seats: ::std::option::Option<::std::primitive::i64>,
    }
    impl Seats {
        pub fn builder() -> SeatsBuilder {
            SeatsBuilder::default()
        }
    }
    mod seats_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"subscriptions#seats\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for a subscription."]
    pub struct Subscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only field that returns the current billing method for a subscription."]
        pub billing_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `creationTime` property is the date when subscription was created. It is in milliseconds using the Epoch format. See an example Epoch converter."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerDomain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Primary domain name of the customer"]
        pub customer_domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This property will always be returned in a response as the unique identifier generated by Google. In a request, this property can be either the primary domain or the unique identifier generated by Google."]
        pub customer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google-issued code (100 char max) for discounted pricing on subscription plans. Deal code must be included in `insert` requests in order to receive discounted rate. This property is optional, regular pricing applies if left empty."]
        pub deal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ subscription_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "subscription_defaults :: kind")]
        #[doc = "Identifies the resource as a Subscription. Value: `reseller#subscription`"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "plan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `plan` property is required. In this version of the API, the G Suite plans are the flexible plan, annual commitment plan, and the 30-day free trial plan. For more information about the API\"s payment plans, see the API concepts."]
        pub plan: ::std::option::Option<SubscriptionPlan>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purchaseOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is an optional property. This purchase order (PO) information is for resellers to use for their company tracking usage. If a `purchaseOrderId` value is given it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters."]
        pub purchase_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "renewalSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Renewal settings for the annual commitment plan. For more detailed information, see renewal options in the administrator help center."]
        pub renewal_settings: ::std::option::Option<::std::boxed::Box<RenewalSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceUiUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to customer's Subscriptions page in the Admin console. The read-only URL is generated by the API service. This is used if your client application requires the customer to complete a task using the Subscriptions page in the Admin console."]
        pub resource_ui_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is a required property. The number and limit of user seat licenses in the plan."]
        pub seats: ::std::option::Option<::std::boxed::Box<Seats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skuId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A required property. The `skuId` is a unique system identifier for a product's SKU assigned to a customer in the subscription. For products and SKUs available in this version of the API, see Product and SKU IDs."]
        pub sku_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skuName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only external display name for a product's SKU assigned to a customer in the subscription. SKU names are subject to change at Google's discretion. For products and SKUs available in this version of the API, see Product and SKU IDs."]
        pub sku_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is an optional property."]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `subscriptionId` is the subscription identifier and is unique for each customer. This is a required property. Since a `subscriptionId` changes when a subscription is updated, we recommend not using this ID as a key for persistent data. Use the `subscriptionId` as described in retrieve all reseller subscriptions."]
        pub subscription_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suspensionReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only field containing an enumerable of all the current suspension reasons for a subscription. It is possible for a subscription to have many concurrent, overlapping suspension reasons. A subscription's `STATUS` is `SUSPENDED` until all pending suspensions are removed. Possible options include: - `PENDING_TOS_ACCEPTANCE` - The customer has not logged in and accepted the G Suite Resold Terms of Services. - `RENEWAL_WITH_TYPE_CANCEL` - The customer's commitment ended and their service was cancelled at the end of their term. - `RESELLER_INITIATED` - A manual suspension invoked by a Reseller. - `TRIAL_ENDED` - The customer's trial expired without a plan selected. - `OTHER` - The customer is suspended for an internal Google reason (e.g. abuse or otherwise). "]
        pub suspension_reasons: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only transfer related information for the subscription. For more information, see retrieve transferable subscriptions for a customer."]
        pub transfer_info: ::std::option::Option<SubscriptionTransferInfo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trialSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The G Suite annual commitment and flexible payment plans can be in a 30-day free trial. For more information, see the API concepts."]
        pub trial_settings: ::std::option::Option<SubscriptionTrialSettings>,
    }
    impl Subscription {
        pub fn builder() -> SubscriptionBuilder {
            SubscriptionBuilder::default()
        }
    }
    mod subscription_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"reseller#subscription\"").unwrap()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `plan` property is required. In this version of the API, the G Suite plans are the flexible plan, annual commitment plan, and the 30-day free trial plan. For more information about the API\"s payment plans, see the API concepts."]
    pub struct SubscriptionPlan {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commitmentInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "In this version of the API, annual commitment plan's interval is one year. *Note: *When `billingMethod` value is `OFFLINE`, the subscription property object `plan.commitmentInterval` is omitted in all API responses. "]
        pub commitment_interval: ::std::option::Option<SubscriptionPlanCommitmentInterval>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isCommitmentPlan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `isCommitmentPlan` property's boolean value identifies the plan as an annual commitment plan: - `true` — The subscription's plan is an annual commitment plan. - `false` — The plan is not an annual commitment plan. "]
        pub is_commitment_plan: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "planName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `planName` property is required. This is the name of the subscription's plan. For more information about the Google payment plans, see the API concepts. Possible values are: - `ANNUAL_MONTHLY_PAY` — The annual commitment plan with monthly payments. *Caution: *`ANNUAL_MONTHLY_PAY` is returned as `ANNUAL` in all API responses. - `ANNUAL_YEARLY_PAY` — The annual commitment plan with yearly payments - `FLEXIBLE` — The flexible plan - `TRIAL` — The 30-day free trial plan. A subscription in trial will be suspended after the 30th free day if no payment plan is assigned. Calling `changePlan` will assign a payment plan to a trial but will not activate the plan. A trial will automatically begin its assigned payment plan after its 30th free day or immediately after calling `startPaidService`. - `FREE` — The free plan is exclusive to the Cloud Identity SKU and does not incur any billing. "]
        pub plan_name: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionPlan {
        pub fn builder() -> SubscriptionPlanBuilder {
            SubscriptionPlanBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "In this version of the API, annual commitment plan's interval is one year. *Note: *When `billingMethod` value is `OFFLINE`, the subscription property object `plan.commitmentInterval` is omitted in all API responses. "]
    pub struct SubscriptionPlanCommitmentInterval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An annual commitment plan's interval's `endTime` in milliseconds using the UNIX Epoch format. See an example Epoch converter."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An annual commitment plan's interval's `startTime` in milliseconds using UNIX Epoch format. See an example Epoch converter."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionPlanCommitmentInterval {
        pub fn builder() -> SubscriptionPlanCommitmentIntervalBuilder {
            SubscriptionPlanCommitmentIntervalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Read-only transfer related information for the subscription. For more information, see retrieve transferable subscriptions for a customer."]
    pub struct SubscriptionTransferInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumTransferableSeats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When inserting a subscription, this is the minimum number of seats listed in the transfer order for this product. For example, if the customer has 20 users, the reseller cannot place a transfer order of 15 seats. The minimum is 20 seats."]
        pub minimum_transferable_seats: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transferabilityExpirationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when transfer token or intent to transfer will expire. The time is in milliseconds using UNIX Epoch format."]
        pub transferability_expiration_time: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionTransferInfo {
        pub fn builder() -> SubscriptionTransferInfoBuilder {
            SubscriptionTransferInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The G Suite annual commitment and flexible payment plans can be in a 30-day free trial. For more information, see the API concepts."]
    pub struct SubscriptionTrialSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isInTrial")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Determines if a subscription's plan is in a 30-day free trial or not: - `true` — The plan is in trial. - `false` — The plan is not in trial. "]
        pub is_in_trial: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trialEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date when the trial ends. The value is in milliseconds using the UNIX Epoch format. See an example Epoch converter."]
        pub trial_end_time: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionTrialSettings {
        pub fn builder() -> SubscriptionTrialSettingsBuilder {
            SubscriptionTrialSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A subscription manages the relationship of a Google customer's payment plan with a product's SKU, user licenses, 30-day free trial status, and renewal options. A primary role of a reseller is to manage the Google customer's subscriptions."]
    pub struct Subscriptions {
        #[builder(default = "{ subscriptions_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "subscriptions_defaults :: kind")]
        #[doc = "Identifies the resource as a collection of subscriptions. Value: reseller#subscriptions"]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The continuation token, used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscriptions in this page of results."]
        pub subscriptions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
    }
    impl Subscriptions {
        pub fn builder() -> SubscriptionsBuilder {
            SubscriptionsBuilder::default()
        }
    }
    mod subscriptions_defaults {
        pub fn kind() -> ::std::string::String {
            serde_json::from_str(&"\"reseller#subscriptions\"").unwrap()
        }
    }
}
