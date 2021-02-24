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
    pub mod matters {
        pub mod methods {
            pub mod get {
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
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which parts of the Matter to return in the response."]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Specifies which parts of the Matter to return in the response."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_UNSPECIFIED")]
                    #[doc = "There is no specified view."]
                    ViewUnspecified,
                    #[serde(rename = "BASIC")]
                    #[doc = "Response includes the matter_id, name, description, and state. Default choice."]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Full representation of matter is returned. Everything above and including MatterPermissions list."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewUnspecified
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
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The number of matters to return in the response. Default and maximum are 100."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The pagination token as returned in the response."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "state")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, list only matters with that specific state. The default is listing matters of all states."]
                    pub state: ::std::option::Option<QueryParametersStateEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies which parts of the matter to return in response."]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "If set, list only matters with that specific state. The default is listing matters of all states."]
                pub enum QueryParametersStateEnum {
                    #[serde(rename = "STATE_UNSPECIFIED")]
                    #[doc = "The matter has no specified state."]
                    StateUnspecified,
                    #[serde(rename = "OPEN")]
                    #[doc = "This matter is open."]
                    Open,
                    #[serde(rename = "CLOSED")]
                    #[doc = "This matter is closed."]
                    Closed,
                    #[serde(rename = "DELETED")]
                    #[doc = "This matter is deleted."]
                    Deleted,
                }
                impl ::std::default::Default for QueryParametersStateEnum {
                    fn default() -> Self {
                        Self::StateUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Specifies which parts of the matter to return in response."]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_UNSPECIFIED")]
                    #[doc = "There is no specified view."]
                    ViewUnspecified,
                    #[serde(rename = "BASIC")]
                    #[doc = "Response includes the matter_id, name, description, and state. Default choice."]
                    Basic,
                    #[serde(rename = "FULL")]
                    #[doc = "Full representation of matter is returned. Everything above and including MatterPermissions list."]
                    Full,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewUnspecified
                    }
                }
            }
        }
        pub mod resources {
            pub mod exports {
                pub mod methods {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The number of exports to return in the response."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The pagination token as returned in the response."]
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
            pub mod holds {
                pub mod methods {
                    pub mod get {
                        #[derive(
                            Clone,
                            Debug,
                            PartialEq,
                            derive_builder :: Builder,
                            serde :: Serialize,
                            serde :: Deserialize,
                        )]
                        pub struct QueryParameters {
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies which parts of the Hold to return."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Specifies which parts of the Hold to return."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "HOLD_VIEW_UNSPECIFIED")]
                            #[doc = "There is no specified view. Defaults to FULL_HOLD."]
                            HoldViewUnspecified,
                            #[serde(rename = "BASIC_HOLD")]
                            #[doc = "Response includes the id, name, update time, corpus, and query."]
                            BasicHold,
                            #[serde(rename = "FULL_HOLD")]
                            #[doc = "Full representation of a Hold. Response includes all fields of 'BASIC' and the entities the Hold applies to, such as accounts, or OU."]
                            FullHold,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::HoldViewUnspecified
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The number of holds to return in the response, between 0 and 100 inclusive. Leaving this empty, or as 0, is the same as page_size = 100."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The pagination token as returned in the response. An empty token means start from the beginning."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "view")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Specifies which parts of the Hold to return."]
                            pub view: ::std::option::Option<QueryParametersViewEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Specifies which parts of the Hold to return."]
                        pub enum QueryParametersViewEnum {
                            #[serde(rename = "HOLD_VIEW_UNSPECIFIED")]
                            #[doc = "There is no specified view. Defaults to FULL_HOLD."]
                            HoldViewUnspecified,
                            #[serde(rename = "BASIC_HOLD")]
                            #[doc = "Response includes the id, name, update time, corpus, and query."]
                            BasicHold,
                            #[serde(rename = "FULL_HOLD")]
                            #[doc = "Full representation of a Hold. Response includes all fields of 'BASIC' and the entities the Hold applies to, such as accounts, or OU."]
                            FullHold,
                        }
                        impl ::std::default::Default for QueryParametersViewEnum {
                            fn default() -> Self {
                                Self::HoldViewUnspecified
                            }
                        }
                    }
                }
            }
            pub mod saved_queries {
                pub mod methods {
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The maximum number of saved queries to return."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The pagination token as returned in the previous response. An empty token means start from the beginning."]
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
    }
    pub mod operations {
        pub mod methods {
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
                    #[serde(rename = "filter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list filter."]
                    pub filter: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page size."]
                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The standard list page token."]
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
    #[doc = "Count number for each account."]
    pub struct AccountCount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account owner."]
        pub account: ::std::option::Option<::std::boxed::Box<UserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of artifacts found for this account."]
        pub count: ::std::option::Option<::std::string::String>,
    }
    impl AccountCount {
        pub fn builder() -> AccountCountBuilder {
            AccountCountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error that occurred when querying a specific account"]
    pub struct AccountCountError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account owner."]
        pub account: ::std::option::Option<::std::boxed::Box<UserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account query error."]
        pub error_type: ::std::option::Option<AccountCountErrorErrorTypeEnum>,
    }
    impl AccountCountError {
        pub fn builder() -> AccountCountErrorBuilder {
            AccountCountErrorBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Account query error."]
    pub enum AccountCountErrorErrorTypeEnum {
        #[serde(rename = "ERROR_TYPE_UNSPECIFIED")]
        #[doc = "Default."]
        ErrorTypeUnspecified,
        #[serde(rename = "WILDCARD_TOO_BROAD")]
        #[doc = "Permanent - prefix terms expanded to too many query terms."]
        WildcardTooBroad,
        #[serde(rename = "TOO_MANY_TERMS")]
        #[doc = "Permanent - query contains too many terms."]
        TooManyTerms,
        #[serde(rename = "LOCATION_UNAVAILABLE")]
        #[doc = "Transient - data in transit between storage replicas, temporarily unavailable."]
        LocationUnavailable,
        #[serde(rename = "UNKNOWN")]
        #[doc = "Unrecognized error."]
        Unknown,
        #[serde(rename = "DEADLINE_EXCEEDED")]
        #[doc = "Deadline exceeded when querying the account."]
        DeadlineExceeded,
    }
    impl ::std::default::Default for AccountCountErrorErrorTypeEnum {
        fn default() -> Self {
            Self::ErrorTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Accounts to search"]
    pub struct AccountInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of accounts to search."]
        pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AccountInfo {
        pub fn builder() -> AccountInfoBuilder {
            AccountInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A status detailing the status of each account creation, and the HeldAccount, if successful."]
    pub struct AddHeldAccountResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If present, this account was successfully created."]
        pub account: ::std::option::Option<::std::boxed::Box<HeldAccount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This represents the success status. If failed, check message."]
        pub status: ::std::option::Option<::std::boxed::Box<Status>>,
    }
    impl AddHeldAccountResult {
        pub fn builder() -> AddHeldAccountResultBuilder {
            AddHeldAccountResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Add a list of accounts to a hold."]
    pub struct AddHeldAccountsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account IDs to identify which accounts to add. Only account_ids or only emails should be specified, but not both."]
        pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Emails to identify which accounts to add. Only emails or only account_ids should be specified, but not both."]
        pub emails: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AddHeldAccountsRequest {
        pub fn builder() -> AddHeldAccountsRequestBuilder {
            AddHeldAccountsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for batch create held accounts."]
    pub struct AddHeldAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of responses, in the same order as the batch request."]
        pub responses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AddHeldAccountResult>>>,
    }
    impl AddHeldAccountsResponse {
        pub fn builder() -> AddHeldAccountsResponseBuilder {
            AddHeldAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Add an account with the permission specified. The role cannot be owner. If an account already has a role in the matter, it will be overwritten."]
    pub struct AddMatterPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ccMe")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only relevant if send_emails is true. True to CC requestor in the email message. False to not CC requestor."]
        pub cc_me: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MatterPermission to add."]
        pub matter_permission: ::std::option::Option<::std::boxed::Box<MatterPermission>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sendEmails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True to send notification email to the added account. False to not send notification email."]
        pub send_emails: ::std::option::Option<::std::primitive::bool>,
    }
    impl AddMatterPermissionsRequest {
        pub fn builder() -> AddMatterPermissionsRequestBuilder {
            AddMatterPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Operations.CancelOperation."]
    pub struct CancelOperationRequest {}
    impl CancelOperationRequest {
        pub fn builder() -> CancelOperationRequestBuilder {
            CancelOperationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Close a matter by ID."]
    pub struct CloseMatterRequest {}
    impl CloseMatterRequest {
        pub fn builder() -> CloseMatterRequestBuilder {
            CloseMatterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a CloseMatterRequest."]
    pub struct CloseMatterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated matter, with state CLOSED."]
        pub matter: ::std::option::Option<::std::boxed::Box<Matter>>,
    }
    impl CloseMatterResponse {
        pub fn builder() -> CloseMatterResponseBuilder {
            CloseMatterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An export file on cloud storage"]
    pub struct CloudStorageFile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bucketName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cloud storage bucket name of this export file. Can be used in cloud storage JSON/XML API, but not to list the bucket contents. Instead, you can get individual export files by object name."]
        pub bucket_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "md5Hash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The md5 hash of the file."]
        pub md5_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "objectName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cloud storage object name of this export file. Can be used in cloud storage JSON/XML API."]
        pub object_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the export file."]
        pub size: ::std::option::Option<::std::string::String>,
    }
    impl CloudStorageFile {
        pub fn builder() -> CloudStorageFileBuilder {
            CloudStorageFileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export sink for cloud storage files."]
    pub struct CloudStorageSink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "files")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The exported files on cloud storage."]
        pub files: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CloudStorageFile>>>,
    }
    impl CloudStorageSink {
        pub fn builder() -> CloudStorageSinkBuilder {
            CloudStorageSinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Corpus specific queries."]
    pub struct CorpusQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details pertaining to Drive holds. If set, corpus must be Drive."]
        pub drive_query: ::std::option::Option<::std::boxed::Box<HeldDriveQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupsQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details pertaining to Groups holds. If set, corpus must be Groups."]
        pub groups_query: ::std::option::Option<::std::boxed::Box<HeldGroupsQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hangoutsChatQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details pertaining to Hangouts Chat holds. If set, corpus must be Hangouts Chat."]
        pub hangouts_chat_query: ::std::option::Option<::std::boxed::Box<HeldHangoutsChatQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details pertaining to mail holds. If set, corpus must be mail."]
        pub mail_query: ::std::option::Option<::std::boxed::Box<HeldMailQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voiceQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details pertaining to Voice holds. If set, corpus must be Voice."]
        pub voice_query: ::std::option::Option<::std::boxed::Box<HeldVoiceQuery>>,
    }
    impl CorpusQuery {
        pub fn builder() -> CorpusQueryBuilder {
            CorpusQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Long running operation metadata for CountArtifacts."]
    pub struct CountArtifactsMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of count operation. Available when operation is done."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The matter ID of the associated matter."]
        pub matter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search query from the request."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of count operation."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl CountArtifactsMetadata {
        pub fn builder() -> CountArtifactsMetadataBuilder {
            CountArtifactsMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Count artifacts request."]
    pub struct CountArtifactsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search query."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "view")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the granularity of the count result returned in response."]
        pub view: ::std::option::Option<CountArtifactsRequestViewEnum>,
    }
    impl CountArtifactsRequest {
        pub fn builder() -> CountArtifactsRequestBuilder {
            CountArtifactsRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the granularity of the count result returned in response."]
    pub enum CountArtifactsRequestViewEnum {
        #[serde(rename = "COUNT_RESULT_VIEW_UNSPECIFIED")]
        #[doc = "Default. It works the same as TOTAL_COUNT."]
        CountResultViewUnspecified,
        #[serde(rename = "TOTAL_COUNT")]
        #[doc = "Response includes: total count, queried accounts count, matching accounts count, non-queryable accounts, queried account errors."]
        TotalCount,
        #[serde(rename = "ALL")]
        #[doc = "Response includes additional breakdown of account count."]
        All,
    }
    impl ::std::default::Default for CountArtifactsRequestViewEnum {
        fn default() -> Self {
            Self::CountResultViewUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition of the response for method CountArtifacts."]
    pub struct CountArtifactsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupsCountResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count metrics of Groups."]
        pub groups_count_result: ::std::option::Option<::std::boxed::Box<GroupsCountResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailCountResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count metrics of Mail."]
        pub mail_count_result: ::std::option::Option<::std::boxed::Box<MailCountResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total count of artifacts. For mail and groups, artifacts refers to messages."]
        pub total_count: ::std::option::Option<::std::string::String>,
    }
    impl CountArtifactsResponse {
        pub fn builder() -> CountArtifactsResponseBuilder {
            CountArtifactsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options for Drive export."]
    pub struct DriveExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeAccessInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to include access level information for users with indirect access to files."]
        pub include_access_info: ::std::option::Option<::std::primitive::bool>,
    }
    impl DriveExportOptions {
        pub fn builder() -> DriveExportOptionsBuilder {
            DriveExportOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Drive search advanced options"]
    pub struct DriveOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeSharedDrives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to include shared drive."]
        pub include_shared_drives: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeTeamDrives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to include Team Drive."]
        pub include_team_drives: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Search the versions of the Drive file as of the reference date. These timestamps are in GMT and rounded down to the given date."]
        pub version_date: ::std::option::Option<::std::string::String>,
    }
    impl DriveOptions {
        pub fn builder() -> DriveOptionsBuilder {
            DriveOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct Empty {}
    impl Empty {
        pub fn builder() -> EmptyBuilder {
            EmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An export"]
    pub struct Export {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cloudStorageSink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Export sink for cloud storage files."]
        pub cloud_storage_sink: ::std::option::Option<::std::boxed::Box<CloudStorageSink>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the export was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Advanced options of the export."]
        pub export_options: ::std::option::Option<::std::boxed::Box<ExportOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The generated export ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The matter ID."]
        pub matter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The export name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search query being exported."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requester")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The requester of the export."]
        pub requester: ::std::option::Option<::std::boxed::Box<UserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Export statistics."]
        pub stats: ::std::option::Option<::std::boxed::Box<ExportStats>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The export status."]
        pub status: ::std::option::Option<ExportStatusEnum>,
    }
    impl Export {
        pub fn builder() -> ExportBuilder {
            ExportBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The export status."]
    pub enum ExportStatusEnum {
        #[serde(rename = "EXPORT_STATUS_UNSPECIFIED")]
        #[doc = "The status is unspecified."]
        ExportStatusUnspecified,
        #[serde(rename = "COMPLETED")]
        #[doc = "The export completed."]
        Completed,
        #[serde(rename = "FAILED")]
        #[doc = "The export failed."]
        Failed,
        #[serde(rename = "IN_PROGRESS")]
        #[doc = "The export is still being executed."]
        InProgress,
    }
    impl ::std::default::Default for ExportStatusEnum {
        fn default() -> Self {
            Self::ExportStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Export advanced options"]
    pub struct ExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option available for Drive export."]
        pub drive_options: ::std::option::Option<::std::boxed::Box<DriveExportOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupsOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option available for groups export."]
        pub groups_options: ::std::option::Option<::std::boxed::Box<GroupsExportOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hangoutsChatOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option available for hangouts chat export."]
        pub hangouts_chat_options:
            ::std::option::Option<::std::boxed::Box<HangoutsChatExportOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option available for mail export."]
        pub mail_options: ::std::option::Option<::std::boxed::Box<MailExportOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested export location."]
        pub region: ::std::option::Option<ExportOptionsRegionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voiceOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option available for voice export."]
        pub voice_options: ::std::option::Option<::std::boxed::Box<VoiceExportOptions>>,
    }
    impl ExportOptions {
        pub fn builder() -> ExportOptionsBuilder {
            ExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The requested export location."]
    pub enum ExportOptionsRegionEnum {
        #[serde(rename = "EXPORT_REGION_UNSPECIFIED")]
        #[doc = "The region is unspecified. Will be treated the same as ANY."]
        ExportRegionUnspecified,
        #[serde(rename = "ANY")]
        #[doc = "Any region."]
        Any,
        #[serde(rename = "US")]
        #[doc = "US region."]
        Us,
        #[serde(rename = "EUROPE")]
        #[doc = "Europe region."]
        Europe,
    }
    impl ::std::default::Default for ExportOptionsRegionEnum {
        fn default() -> Self {
            Self::ExportRegionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stats of an export."]
    pub struct ExportStats {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportedArtifactCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of documents already processed by the export."]
        pub exported_artifact_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeInBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of export in bytes."]
        pub size_in_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalArtifactCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of documents to be exported."]
        pub total_artifact_count: ::std::option::Option<::std::string::String>,
    }
    impl ExportStats {
        pub fn builder() -> ExportStatsBuilder {
            ExportStatsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Groups specific count metrics."]
    pub struct GroupsCountResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountCountErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error occurred when querying these accounts."]
        pub account_count_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCountError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subtotal count per matching account that have more than zero messages."]
        pub account_counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchingAccountsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of accounts that can be queried and have more than zero messages."]
        pub matching_accounts_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonQueryableAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When data scope is HELD_DATA in the request Query, these accounts in the request are not queried because they are not on hold. For other data scope, this field is not set."]
        pub non_queryable_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queriedAccountsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of accounts involved in this count operation."]
        pub queried_accounts_count: ::std::option::Option<::std::string::String>,
    }
    impl GroupsCountResult {
        pub fn builder() -> GroupsCountResultBuilder {
            GroupsCountResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options for groups export."]
    pub struct GroupsExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The export format for groups export."]
        pub export_format: ::std::option::Option<GroupsExportOptionsExportFormatEnum>,
    }
    impl GroupsExportOptions {
        pub fn builder() -> GroupsExportOptionsBuilder {
            GroupsExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The export format for groups export."]
    pub enum GroupsExportOptionsExportFormatEnum {
        #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
        #[doc = "No export format specified."]
        ExportFormatUnspecified,
        #[serde(rename = "MBOX")]
        #[doc = "MBOX as export format."]
        Mbox,
        #[serde(rename = "PST")]
        #[doc = "PST as export format"]
        Pst,
    }
    impl ::std::default::Default for GroupsExportOptionsExportFormatEnum {
        fn default() -> Self {
            Self::ExportFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options for hangouts chat export."]
    pub struct HangoutsChatExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The export format for hangouts chat export."]
        pub export_format: ::std::option::Option<HangoutsChatExportOptionsExportFormatEnum>,
    }
    impl HangoutsChatExportOptions {
        pub fn builder() -> HangoutsChatExportOptionsBuilder {
            HangoutsChatExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The export format for hangouts chat export."]
    pub enum HangoutsChatExportOptionsExportFormatEnum {
        #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
        #[doc = "No export format specified."]
        ExportFormatUnspecified,
        #[serde(rename = "MBOX")]
        #[doc = "MBOX as export format."]
        Mbox,
        #[serde(rename = "PST")]
        #[doc = "PST as export format"]
        Pst,
    }
    impl ::std::default::Default for HangoutsChatExportOptionsExportFormatEnum {
        fn default() -> Self {
            Self::ExportFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Accounts to search"]
    pub struct HangoutsChatInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "roomId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of rooms to search."]
        pub room_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl HangoutsChatInfo {
        pub fn builder() -> HangoutsChatInfoBuilder {
            HangoutsChatInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Hangouts chat search advanced options"]
    pub struct HangoutsChatOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeRooms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to include rooms."]
        pub include_rooms: ::std::option::Option<::std::primitive::bool>,
    }
    impl HangoutsChatOptions {
        pub fn builder() -> HangoutsChatOptionsBuilder {
            HangoutsChatOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An account being held in a particular hold. This structure is immutable. This can be either a single user or a google group, depending on the corpus."]
    pub struct HeldAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account's ID as provided by the Admin SDK."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary email address of the account. If used as an input, this takes precedence over account ID."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The first name of the account holder."]
        pub first_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When the account was put on hold."]
        pub hold_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last name of the account holder."]
        pub last_name: ::std::option::Option<::std::string::String>,
    }
    impl HeldAccount {
        pub fn builder() -> HeldAccountBuilder {
            HeldAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query options for Drive holds."]
    pub struct HeldDriveQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeSharedDriveFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, include files in shared drives in the hold."]
        pub include_shared_drive_files: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeTeamDriveFiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, include files in Team Drives in the hold."]
        pub include_team_drive_files: ::std::option::Option<::std::primitive::bool>,
    }
    impl HeldDriveQuery {
        pub fn builder() -> HeldDriveQueryBuilder {
            HeldDriveQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query options for group holds."]
    pub struct HeldGroupsQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "terms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search terms for the hold."]
        pub terms: ::std::option::Option<::std::string::String>,
    }
    impl HeldGroupsQuery {
        pub fn builder() -> HeldGroupsQueryBuilder {
            HeldGroupsQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query options for hangouts chat holds."]
    pub struct HeldHangoutsChatQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeRooms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, include rooms the user has participated in."]
        pub include_rooms: ::std::option::Option<::std::primitive::bool>,
    }
    impl HeldHangoutsChatQuery {
        pub fn builder() -> HeldHangoutsChatQueryBuilder {
            HeldHangoutsChatQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query options for mail holds."]
    pub struct HeldMailQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "terms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search terms for the hold."]
        pub terms: ::std::option::Option<::std::string::String>,
    }
    impl HeldMailQuery {
        pub fn builder() -> HeldMailQueryBuilder {
            HeldMailQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A organizational unit being held in a particular hold. This structure is immutable."]
    pub struct HeldOrgUnit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holdTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When the org unit was put on hold. This property is immutable."]
        pub hold_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The org unit's immutable ID as provided by the Admin SDK."]
        pub org_unit_id: ::std::option::Option<::std::string::String>,
    }
    impl HeldOrgUnit {
        pub fn builder() -> HeldOrgUnitBuilder {
            HeldOrgUnitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Query options for Voice holds."]
    pub struct HeldVoiceQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coveredData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data covered by this rule. Should be non-empty. Order does not matter and duplicates will be ignored."]
        pub covered_data: ::std::option::Option<::std::vec::Vec<HeldVoiceQueryCoveredDataEnum>>,
    }
    impl HeldVoiceQuery {
        pub fn builder() -> HeldVoiceQueryBuilder {
            HeldVoiceQueryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum HeldVoiceQueryCoveredDataEnum {
        #[serde(rename = "COVERED_DATA_UNSPECIFIED")]
        #[doc = "Covered data unspecified."]
        CoveredDataUnspecified,
        #[serde(rename = "TEXT_MESSAGES")]
        #[doc = "Voice text message will be covered."]
        TextMessages,
        #[serde(rename = "VOICEMAILS")]
        #[doc = "Voicemail will be covered."]
        Voicemails,
        #[serde(rename = "CALL_LOGS")]
        #[doc = "Call logs will be covered."]
        CallLogs,
    }
    impl ::std::default::Default for HeldVoiceQueryCoveredDataEnum {
        fn default() -> Self {
            Self::CoveredDataUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a hold within Vault. A hold restricts purging of artifacts based on the combination of the query and accounts restrictions. A hold can be configured to either apply to an explicitly configured set of accounts, or can be applied to all members of an organizational unit."]
    pub struct Hold {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the hold applies to the enumerated accounts and org_unit must be empty."]
        pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HeldAccount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "corpus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corpus to be searched."]
        pub corpus: ::std::option::Option<HoldCorpusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holdId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique immutable ID of the hold. Assigned during creation."]
        pub hold_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the hold."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, the hold applies to all members of the organizational unit and accounts must be empty. This property is mutable. For groups holds, set the accounts field."]
        pub org_unit: ::std::option::Option<::std::boxed::Box<HeldOrgUnit>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corpus-specific query. If set, the corpusQuery must match corpus type."]
        pub query: ::std::option::Option<::std::boxed::Box<CorpusQuery>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time this hold was modified."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Hold {
        pub fn builder() -> HoldBuilder {
            HoldBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The corpus to be searched."]
    pub enum HoldCorpusEnum {
        #[serde(rename = "CORPUS_TYPE_UNSPECIFIED")]
        #[doc = "No corpus specified."]
        CorpusTypeUnspecified,
        #[serde(rename = "DRIVE")]
        #[doc = "Drive."]
        Drive,
        #[serde(rename = "MAIL")]
        #[doc = "Mail."]
        Mail,
        #[serde(rename = "GROUPS")]
        #[doc = "Groups."]
        Groups,
        #[serde(rename = "HANGOUTS_CHAT")]
        #[doc = "Hangouts Chat."]
        HangoutsChat,
        #[serde(rename = "VOICE")]
        #[doc = "Google Voice."]
        Voice,
    }
    impl ::std::default::Default for HoldCorpusEnum {
        fn default() -> Self {
            Self::CorpusTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The holds for a matter."]
    pub struct ListExportsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exports")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of exports."]
        pub exports: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Export>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListExportsResponse {
        pub fn builder() -> ListExportsResponseBuilder {
            ListExportsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Returns a list of held accounts for a hold."]
    pub struct ListHeldAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The held accounts on a hold."]
        pub accounts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HeldAccount>>>,
    }
    impl ListHeldAccountsResponse {
        pub fn builder() -> ListHeldAccountsResponseBuilder {
            ListHeldAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The holds for a matter."]
    pub struct ListHoldsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of holds."]
        pub holds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Hold>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list. If this is empty, then there are no more holds to list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListHoldsResponse {
        pub fn builder() -> ListHoldsResponseBuilder {
            ListHoldsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Provides the list of matters."]
    pub struct ListMattersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of matters."]
        pub matters: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Matter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListMattersResponse {
        pub fn builder() -> ListMattersResponseBuilder {
            ListMattersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct ListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Operation>>>,
    }
    impl ListOperationsResponse {
        pub fn builder() -> ListOperationsResponseBuilder {
            ListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition of the response for method ListSaveQuery."]
    pub struct ListSavedQueriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Page token to retrieve the next page of results in the list. If this is empty, then there are no more saved queries to list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "savedQueries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of output saved queries."]
        pub saved_queries: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SavedQuery>>>,
    }
    impl ListSavedQueriesResponse {
        pub fn builder() -> ListSavedQueriesResponseBuilder {
            ListSavedQueriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mail specific count metrics."]
    pub struct MailCountResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountCountErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Error occurred when querying these accounts."]
        pub account_count_errors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCountError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountCounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Subtotal count per matching account that have more than zero messages."]
        pub account_counts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountCount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchingAccountsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of accounts that can be queried and have more than zero messages."]
        pub matching_accounts_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonQueryableAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When data scope is HELD_DATA in the request Query, these accounts in the request are not queried because they are not on hold. For other data scope, this field is not set."]
        pub non_queryable_accounts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queriedAccountsCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total number of accounts involved in this count operation."]
        pub queried_accounts_count: ::std::option::Option<::std::string::String>,
    }
    impl MailCountResult {
        pub fn builder() -> MailCountResultBuilder {
            MailCountResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options for mail export."]
    pub struct MailExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The export file format."]
        pub export_format: ::std::option::Option<MailExportOptionsExportFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showConfidentialModeContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to export confidential mode content."]
        pub show_confidential_mode_content: ::std::option::Option<::std::primitive::bool>,
    }
    impl MailExportOptions {
        pub fn builder() -> MailExportOptionsBuilder {
            MailExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The export file format."]
    pub enum MailExportOptionsExportFormatEnum {
        #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
        #[doc = "No export format specified."]
        ExportFormatUnspecified,
        #[serde(rename = "MBOX")]
        #[doc = "MBOX as export format."]
        Mbox,
        #[serde(rename = "PST")]
        #[doc = "PST as export format"]
        Pst,
    }
    impl ::std::default::Default for MailExportOptionsExportFormatEnum {
        fn default() -> Self {
            Self::ExportFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mail search advanced options"]
    pub struct MailOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludeDrafts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set to true to exclude drafts."]
        pub exclude_drafts: ::std::option::Option<::std::primitive::bool>,
    }
    impl MailOptions {
        pub fn builder() -> MailOptionsBuilder {
            MailOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a matter."]
    pub struct Matter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the matter."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The matter ID which is generated by the server. Should be blank when creating a new matter."]
        pub matter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterPermissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of users and access to the matter. Currently there is no programmer defined limit on the number of permissions a matter can have."]
        pub matter_permissions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MatterPermission>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the matter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the matter."]
        pub state: ::std::option::Option<MatterStateEnum>,
    }
    impl Matter {
        pub fn builder() -> MatterBuilder {
            MatterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the matter."]
    pub enum MatterStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "The matter has no specified state."]
        StateUnspecified,
        #[serde(rename = "OPEN")]
        #[doc = "This matter is open."]
        Open,
        #[serde(rename = "CLOSED")]
        #[doc = "This matter is closed."]
        Closed,
        #[serde(rename = "DELETED")]
        #[doc = "This matter is deleted."]
        Deleted,
    }
    impl ::std::default::Default for MatterStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Currently each matter only has one owner, and all others are collaborators. When an account is purged, its corresponding MatterPermission resources cease to exist."]
    pub struct MatterPermission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account ID, as provided by Admin SDK."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's role in this matter."]
        pub role: ::std::option::Option<MatterPermissionRoleEnum>,
    }
    impl MatterPermission {
        pub fn builder() -> MatterPermissionBuilder {
            MatterPermissionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The user's role in this matter."]
    pub enum MatterPermissionRoleEnum {
        #[serde(rename = "ROLE_UNSPECIFIED")]
        #[doc = "No role assigned."]
        RoleUnspecified,
        #[serde(rename = "COLLABORATOR")]
        #[doc = "A collaborator to the matter."]
        Collaborator,
        #[serde(rename = "OWNER")]
        #[doc = "The owner of the matter."]
        Owner,
    }
    impl ::std::default::Default for MatterPermissionRoleEnum {
        fn default() -> Self {
            Self::RoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct Operation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<Status>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "response")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`."]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl Operation {
        pub fn builder() -> OperationBuilder {
            OperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Org Unit to search"]
    pub struct OrgUnitInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Org unit to search, as provided by the Admin SDK Directory API."]
        pub org_unit_id: ::std::option::Option<::std::string::String>,
    }
    impl OrgUnitInfo {
        pub fn builder() -> OrgUnitInfoBuilder {
            OrgUnitInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A query definition relevant for search & export."]
    pub struct Query {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When 'ACCOUNT' is chosen as search method, account_info needs to be specified."]
        pub account_info: ::std::option::Option<::std::boxed::Box<AccountInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "corpus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corpus to search."]
        pub corpus: ::std::option::Option<QueryCorpusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data source to search from."]
        pub data_scope: ::std::option::Option<QueryDataScopeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For Drive search, specify more options in this field."]
        pub drive_options: ::std::option::Option<::std::boxed::Box<DriveOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hangoutsChatInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When 'ROOM' is chosen as search method, hangout_chats_info needs to be specified. (read-only)"]
        pub hangouts_chat_info: ::std::option::Option<::std::boxed::Box<HangoutsChatInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hangoutsChatOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For hangouts chat search, specify more options in this field. (read-only)"]
        pub hangouts_chat_options: ::std::option::Option<::std::boxed::Box<HangoutsChatOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mailOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For mail search, specify more options in this field."]
        pub mail_options: ::std::option::Option<::std::boxed::Box<MailOptions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search method to use. This field is similar to the search_method field but is introduced to support shared drives. It supports all search method types. In case the search_method is TEAM_DRIVE the response of this field will be SHARED_DRIVE only."]
        pub method: ::std::option::Option<QueryMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orgUnitInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When 'ORG_UNIT' is chosen as as search method, org_unit_info needs to be specified."]
        pub org_unit_info: ::std::option::Option<::std::boxed::Box<OrgUnitInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The search method to use."]
        pub search_method: ::std::option::Option<QuerySearchMethodEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedDriveInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When 'SHARED_DRIVE' is chosen as search method, shared_drive_info needs to be specified."]
        pub shared_drive_info: ::std::option::Option<::std::boxed::Box<SharedDriveInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start time range for the search query. These timestamps are in GMT and rounded down to the start of the given date."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "When 'TEAM_DRIVE' is chosen as search method, team_drive_info needs to be specified."]
        pub team_drive_info: ::std::option::Option<::std::boxed::Box<TeamDriveInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "terms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corpus-specific search operators used to generate search results."]
        pub terms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone name. It should be an IANA TZ name, such as \"America/Los_Angeles\". For more information, see Time Zone."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voiceOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For voice search, specify more options in this field."]
        pub voice_options: ::std::option::Option<::std::boxed::Box<VoiceOptions>>,
    }
    impl Query {
        pub fn builder() -> QueryBuilder {
            QueryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The corpus to search."]
    pub enum QueryCorpusEnum {
        #[serde(rename = "CORPUS_TYPE_UNSPECIFIED")]
        #[doc = "No corpus specified."]
        CorpusTypeUnspecified,
        #[serde(rename = "DRIVE")]
        #[doc = "Drive."]
        Drive,
        #[serde(rename = "MAIL")]
        #[doc = "Mail."]
        Mail,
        #[serde(rename = "GROUPS")]
        #[doc = "Groups."]
        Groups,
        #[serde(rename = "HANGOUTS_CHAT")]
        #[doc = "Hangouts Chat."]
        HangoutsChat,
        #[serde(rename = "VOICE")]
        #[doc = "Google Voice."]
        Voice,
    }
    impl ::std::default::Default for QueryCorpusEnum {
        fn default() -> Self {
            Self::CorpusTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The data source to search from."]
    pub enum QueryDataScopeEnum {
        #[serde(rename = "DATA_SCOPE_UNSPECIFIED")]
        #[doc = "No data scope specified."]
        DataScopeUnspecified,
        #[serde(rename = "ALL_DATA")]
        #[doc = "All available data."]
        AllData,
        #[serde(rename = "HELD_DATA")]
        #[doc = "Data on hold."]
        HeldData,
        #[serde(rename = "UNPROCESSED_DATA")]
        #[doc = "Data not processed."]
        UnprocessedData,
    }
    impl ::std::default::Default for QueryDataScopeEnum {
        fn default() -> Self {
            Self::DataScopeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The search method to use. This field is similar to the search_method field but is introduced to support shared drives. It supports all search method types. In case the search_method is TEAM_DRIVE the response of this field will be SHARED_DRIVE only."]
    pub enum QueryMethodEnum {
        #[serde(rename = "SEARCH_METHOD_UNSPECIFIED")]
        #[doc = "A search method must be specified. If a request does not specify a search method, it will be rejected."]
        SearchMethodUnspecified,
        #[serde(rename = "ACCOUNT")]
        #[doc = "Will search all accounts provided in account_info."]
        Account,
        #[serde(rename = "ORG_UNIT")]
        #[doc = "Will search all accounts in the OU specified in org_unit_info."]
        OrgUnit,
        #[serde(rename = "TEAM_DRIVE")]
        #[doc = "Will search for all accounts in the Team Drive specified in team_drive_info."]
        TeamDrive,
        #[serde(rename = "ENTIRE_ORG")]
        #[doc = "Will search for all accounts in the organization. No need to set account_info or org_unit_info. Not all CORPUS_TYPE support this scope. Supported by MAIL."]
        EntireOrg,
        #[serde(rename = "ROOM")]
        #[doc = "Will search in the Room specified in hangout_chats_info. (read-only)"]
        Room,
        #[serde(rename = "SHARED_DRIVE")]
        #[doc = "Will search for all accounts in the shared drive specified in shared_drive_info."]
        SharedDrive,
    }
    impl ::std::default::Default for QueryMethodEnum {
        fn default() -> Self {
            Self::SearchMethodUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The search method to use."]
    pub enum QuerySearchMethodEnum {
        #[serde(rename = "SEARCH_METHOD_UNSPECIFIED")]
        #[doc = "A search method must be specified. If a request does not specify a search method, it will be rejected."]
        SearchMethodUnspecified,
        #[serde(rename = "ACCOUNT")]
        #[doc = "Will search all accounts provided in account_info."]
        Account,
        #[serde(rename = "ORG_UNIT")]
        #[doc = "Will search all accounts in the OU specified in org_unit_info."]
        OrgUnit,
        #[serde(rename = "TEAM_DRIVE")]
        #[doc = "Will search for all accounts in the Team Drive specified in team_drive_info."]
        TeamDrive,
        #[serde(rename = "ENTIRE_ORG")]
        #[doc = "Will search for all accounts in the organization. No need to set account_info or org_unit_info. Not all CORPUS_TYPE support this scope. Supported by MAIL."]
        EntireOrg,
        #[serde(rename = "ROOM")]
        #[doc = "Will search in the Room specified in hangout_chats_info. (read-only)"]
        Room,
        #[serde(rename = "SHARED_DRIVE")]
        #[doc = "Will search for all accounts in the shared drive specified in shared_drive_info."]
        SharedDrive,
    }
    impl ::std::default::Default for QuerySearchMethodEnum {
        fn default() -> Self {
            Self::SearchMethodUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Remove a list of accounts from a hold."]
    pub struct RemoveHeldAccountsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account IDs to identify HeldAccounts to remove."]
        pub account_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl RemoveHeldAccountsRequest {
        pub fn builder() -> RemoveHeldAccountsRequestBuilder {
            RemoveHeldAccountsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for batch delete held accounts."]
    pub struct RemoveHeldAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of statuses for deleted accounts. Results have the same order as the request."]
        pub statuses: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Status>>>,
    }
    impl RemoveHeldAccountsResponse {
        pub fn builder() -> RemoveHeldAccountsResponseBuilder {
            RemoveHeldAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Remove an account as a matter collaborator."]
    pub struct RemoveMatterPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
    }
    impl RemoveMatterPermissionsRequest {
        pub fn builder() -> RemoveMatterPermissionsRequestBuilder {
            RemoveMatterPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Reopen a matter by ID."]
    pub struct ReopenMatterRequest {}
    impl ReopenMatterRequest {
        pub fn builder() -> ReopenMatterRequestBuilder {
            ReopenMatterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to a ReopenMatterRequest."]
    pub struct ReopenMatterResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated matter, with state OPEN."]
        pub matter: ::std::option::Option<::std::boxed::Box<Matter>>,
    }
    impl ReopenMatterResponse {
        pub fn builder() -> ReopenMatterResponseBuilder {
            ReopenMatterResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition of the saved query."]
    pub struct SavedQuery {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The server generated timestamp at which saved query was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the saved query."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The matter ID of the associated matter. The server does not look at this field during create and always uses matter id in the URL."]
        pub matter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The underlying Query object which contains all the information of the saved query."]
        pub query: ::std::option::Option<::std::boxed::Box<Query>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "savedQueryId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique identifier for the saved query."]
        pub saved_query_id: ::std::option::Option<::std::string::String>,
    }
    impl SavedQuery {
        pub fn builder() -> SavedQueryBuilder {
            SavedQueryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Shared drives to search"]
    pub struct SharedDriveInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sharedDriveIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Shared drive IDs, as provided by Drive API."]
        pub shared_drive_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl SharedDriveInfo {
        pub fn builder() -> SharedDriveInfoBuilder {
            SharedDriveInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct Status {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of messages that carry the error details. There is a common set of message types for APIs to use."]
        pub details: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A developer-facing error message, which should be in English. Any user-facing error message should be localized and sent in the google.rpc.Status.details field, or localized by the client."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Status {
        pub fn builder() -> StatusBuilder {
            StatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Team Drives to search"]
    pub struct TeamDriveInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teamDriveIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Team Drive IDs, as provided by Drive API."]
        pub team_drive_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TeamDriveInfo {
        pub fn builder() -> TeamDriveInfoBuilder {
            TeamDriveInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Undelete a matter by ID."]
    pub struct UndeleteMatterRequest {}
    impl UndeleteMatterRequest {
        pub fn builder() -> UndeleteMatterRequestBuilder {
            UndeleteMatterRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "User's information."]
    pub struct UserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The displayed name of the user."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address of the user."]
        pub email: ::std::option::Option<::std::string::String>,
    }
    impl UserInfo {
        pub fn builder() -> UserInfoBuilder {
            UserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The options for voice export."]
    pub struct VoiceExportOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exportFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The export format for voice export."]
        pub export_format: ::std::option::Option<VoiceExportOptionsExportFormatEnum>,
    }
    impl VoiceExportOptions {
        pub fn builder() -> VoiceExportOptionsBuilder {
            VoiceExportOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The export format for voice export."]
    pub enum VoiceExportOptionsExportFormatEnum {
        #[serde(rename = "EXPORT_FORMAT_UNSPECIFIED")]
        #[doc = "No export format specified."]
        ExportFormatUnspecified,
        #[serde(rename = "MBOX")]
        #[doc = "MBOX as export format."]
        Mbox,
        #[serde(rename = "PST")]
        #[doc = "PST as export format"]
        Pst,
    }
    impl ::std::default::Default for VoiceExportOptionsExportFormatEnum {
        fn default() -> Self {
            Self::ExportFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Voice search options"]
    pub struct VoiceOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coveredData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Datatypes to search"]
        pub covered_data: ::std::option::Option<::std::vec::Vec<VoiceOptionsCoveredDataEnum>>,
    }
    impl VoiceOptions {
        pub fn builder() -> VoiceOptionsBuilder {
            VoiceOptionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VoiceOptionsCoveredDataEnum {
        #[serde(rename = "COVERED_DATA_UNSPECIFIED")]
        #[doc = "Covered data unspecified."]
        CoveredDataUnspecified,
        #[serde(rename = "TEXT_MESSAGES")]
        #[doc = "Voice text message will be covered."]
        TextMessages,
        #[serde(rename = "VOICEMAILS")]
        #[doc = "Voicemail will be covered."]
        Voicemails,
        #[serde(rename = "CALL_LOGS")]
        #[doc = "Call logs will be covered."]
        CallLogs,
    }
    impl ::std::default::Default for VoiceOptionsCoveredDataEnum {
        fn default() -> Self {
            Self::CoveredDataUnspecified
        }
    }
}
