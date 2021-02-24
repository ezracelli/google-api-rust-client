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
        serde_json::from_str(&"json").unwrap()
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
    pub mod accounts {
        pub mod resources {
            pub mod clients {
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
                            #[doc = "Requested page size. The server may return fewer clients than requested. If unspecified, the server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListClientsResponse.nextPageToken returned from the previous call to the accounts.clients.list method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "partnerClientId")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Optional unique identifier (from the standpoint of an Ad Exchange sponsor buyer partner) of the client to return. If specified, at most one client will be returned in the response."]
                            pub partner_client_id: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod invitations {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. Server may return fewer clients than requested. If unspecified, server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListClientUserInvitationsResponse.nextPageToken returned from the previous call to the clients.invitations.list method."]
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
                    pub mod users {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer clients than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListClientUsersResponse.nextPageToken returned from the previous call to the accounts.clients.users.list method."]
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
            pub mod creatives {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "duplicateIdMode")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Indicates if multiple creatives can share an ID or not. Default is NO_DUPLICATES (one ID per creative)."]
                            pub duplicate_id_mode:
                                ::std::option::Option<QueryParametersDuplicateIdModeEnum>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Indicates if multiple creatives can share an ID or not. Default is NO_DUPLICATES (one ID per creative)."]
                        pub enum QueryParametersDuplicateIdModeEnum {
                            #[serde(rename = "NO_DUPLICATES")]
                            #[doc = "Recommended. This means that an ID will be unique to a single creative. Multiple creatives will not share an ID."]
                            NoDuplicates,
                            #[serde(rename = "FORCE_ENABLE_DUPLICATE_IDS")]
                            #[doc = "Not recommended. Using this option will allow multiple creatives to share the same ID. Get and Update requests will not be possible for any ID that has more than one creative associated. (List will still function.) This is only intended for backwards compatibility in cases where a single ID is already shared by multiple creatives from previous APIs."]
                            ForceEnableDuplicateIds,
                        }
                        impl ::std::default::Default for QueryParametersDuplicateIdModeEnum {
                            fn default() -> Self {
                                Self::NoDuplicates
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
                            #[doc = "Requested page size. The server may return fewer creatives than requested (due to timeout constraint) even if more are available via another call. If unspecified, server will pick an appropriate default. Acceptable values are 1 to 1000, inclusive."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativesResponse.next_page_token returned from the previous call to 'ListCreatives' method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "query")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An optional query string to filter creatives. If no filter is specified, all active creatives will be returned. Supported queries are: - accountId=*account_id_string* - creativeId=*creative_id_string* - dealsStatus: {approved, conditionally_approved, disapproved, not_checked} - openAuctionStatus: {approved, conditionally_approved, disapproved, not_checked} - attribute: {a numeric attribute from the list of attributes} - disapprovalReason: {a reason from DisapprovalReason} Example: 'accountId=12345 AND (dealsStatus:disapproved AND disapprovalReason:unacceptable_content) OR attribute:47'"]
                            pub query: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod deal_associations {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. Server may return fewer associations than requested. If unspecified, server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListDealAssociationsResponse.next_page_token returned from the previous call to 'ListDealAssociations' method."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "query")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "An optional query string to filter deal associations. If no filter is specified, all associations will be returned. Supported queries are: - accountId=*account_id_string* - creativeId=*creative_id_string* - dealsId=*deals_id_string* - dealsStatus:{approved, conditionally_approved, disapproved, not_checked} - openAuctionStatus:{approved, conditionally_approved, disapproved, not_checked} Example: 'dealsId=12345 AND dealsStatus:disapproved'"]
                                    pub query: ::std::option::Option<::std::string::String>,
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
            pub mod finalized_proposals {
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
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An optional PQL filter query used to query for proposals. Nested repeated fields, such as proposal.deals.targetingCriterion, cannot be filtered."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filterSyntax")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Syntax the filter is written in. Current implementation defaults to PQL but in the future it will be LIST_FILTER."]
                            pub filter_syntax:
                                ::std::option::Option<QueryParametersFilterSyntaxEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token as returned from ListProposalsResponse."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Syntax the filter is written in. Current implementation defaults to PQL but in the future it will be LIST_FILTER."]
                        pub enum QueryParametersFilterSyntaxEnum {
                            #[serde(rename = "FILTER_SYNTAX_UNSPECIFIED")]
                            #[doc = "A placeholder for an undefined filter syntax."]
                            FilterSyntaxUnspecified,
                            #[serde(rename = "PQL")]
                            #[doc = "PQL query syntax. Visit https://developers.google.com/ad-manager/api/pqlreference for PQL documentation and examples."]
                            Pql,
                            #[serde(rename = "LIST_FILTER")]
                            #[doc = "API list filtering syntax. Read about syntax and usage at https://developers.google.com/authorized-buyers/apis/guides/v2/list-filters."]
                            ListFilter,
                        }
                        impl ::std::default::Default for QueryParametersFilterSyntaxEnum {
                            fn default() -> Self {
                                Self::FilterSyntaxUnspecified
                            }
                        }
                    }
                }
            }
            pub mod products {
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
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An optional PQL query used to query for products. See https://developers.google.com/ad-manager/docs/pqlreference for documentation about PQL and examples. Nested repeated fields, such as product.targetingCriterion.inclusions, cannot be filtered."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token as returned from ListProductsResponse."]
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
            pub mod proposals {
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
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "An optional PQL filter query used to query for proposals. Nested repeated fields, such as proposal.deals.targetingCriterion, cannot be filtered."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filterSyntax")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Syntax the filter is written in. Current implementation defaults to PQL but in the future it will be LIST_FILTER."]
                            pub filter_syntax:
                                ::std::option::Option<QueryParametersFilterSyntaxEnum>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token as returned from ListProposalsResponse."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                        #[derive(
                            Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                        )]
                        #[doc = "Syntax the filter is written in. Current implementation defaults to PQL but in the future it will be LIST_FILTER."]
                        pub enum QueryParametersFilterSyntaxEnum {
                            #[serde(rename = "FILTER_SYNTAX_UNSPECIFIED")]
                            #[doc = "A placeholder for an undefined filter syntax."]
                            FilterSyntaxUnspecified,
                            #[serde(rename = "PQL")]
                            #[doc = "PQL query syntax. Visit https://developers.google.com/ad-manager/api/pqlreference for PQL documentation and examples."]
                            Pql,
                            #[serde(rename = "LIST_FILTER")]
                            #[doc = "API list filtering syntax. Read about syntax and usage at https://developers.google.com/authorized-buyers/apis/guides/v2/list-filters."]
                            ListFilter,
                        }
                        impl ::std::default::Default for QueryParametersFilterSyntaxEnum {
                            fn default() -> Self {
                                Self::FilterSyntaxUnspecified
                            }
                        }
                    }
                }
            }
            pub mod publisher_profiles {
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
                            #[doc = "Specify the number of results to include per page."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The page token as return from ListPublisherProfilesResponse."]
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
    pub mod bidders {
        pub mod resources {
            pub mod accounts {
                pub mod resources {
                    pub mod filter_sets {
                        pub mod methods {
                            pub mod create {
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
                                    #[serde(rename = "isTransient")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Whether the filter set is transient, or should be persisted indefinitely. By default, filter sets are not transient. If transient, it will be available for at least 1 hour after creation."]
                                    pub is_transient: ::std::option::Option<::std::primitive::bool>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilterSetsResponse.nextPageToken returned from the previous call to the accounts.filterSets.list method."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod bid_metrics {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidMetricsResponse.nextPageToken returned from the previous call to the bidMetrics.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod bid_response_errors {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidResponseErrorsResponse.nextPageToken returned from the previous call to the bidResponseErrors.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod bid_responses_without_bids {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidResponsesWithoutBidsResponse.nextPageToken returned from the previous call to the bidResponsesWithoutBids.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod filtered_bid_requests {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilteredBidRequestsResponse.nextPageToken returned from the previous call to the filteredBidRequests.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod filtered_bids {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilteredBidsResponse.nextPageToken returned from the previous call to the filteredBids.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                                pub mod resources {
                                    pub mod creatives {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativeStatusBreakdownByCreativeResponse.nextPageToken returned from the previous call to the filteredBids.creatives.list method."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                }
                                                impl QueryParameters {
                                                    pub fn builder() -> QueryParametersBuilder {
                                                        QueryParametersBuilder::default()
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    pub mod details {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                                    pub page_size: ::std::option::Option<
                                                        ::std::primitive::i64,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageToken")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativeStatusBreakdownByDetailResponse.nextPageToken returned from the previous call to the filteredBids.details.list method."]
                                                    pub page_token: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                            pub mod impression_metrics {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListImpressionMetricsResponse.nextPageToken returned from the previous call to the impressionMetrics.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod losing_bids {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListLosingBidsResponse.nextPageToken returned from the previous call to the losingBids.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod non_billable_winning_bids {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListNonBillableWinningBidsResponse.nextPageToken returned from the previous call to the nonBillableWinningBids.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                }
            }
            pub mod filter_sets {
                pub mod methods {
                    pub mod create {
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
                            #[serde(rename = "isTransient")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Whether the filter set is transient, or should be persisted indefinitely. By default, filter sets are not transient. If transient, it will be available for at least 1 hour after creation."]
                            pub is_transient: ::std::option::Option<::std::primitive::bool>,
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilterSetsResponse.nextPageToken returned from the previous call to the accounts.filterSets.list method."]
                            pub page_token: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod bid_metrics {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidMetricsResponse.nextPageToken returned from the previous call to the bidMetrics.list method."]
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
                    pub mod bid_response_errors {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidResponseErrorsResponse.nextPageToken returned from the previous call to the bidResponseErrors.list method."]
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
                    pub mod bid_responses_without_bids {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListBidResponsesWithoutBidsResponse.nextPageToken returned from the previous call to the bidResponsesWithoutBids.list method."]
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
                    pub mod filtered_bid_requests {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilteredBidRequestsResponse.nextPageToken returned from the previous call to the filteredBidRequests.list method."]
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
                    pub mod filtered_bids {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListFilteredBidsResponse.nextPageToken returned from the previous call to the filteredBids.list method."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod creatives {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativeStatusBreakdownByCreativeResponse.nextPageToken returned from the previous call to the filteredBids.creatives.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod details {
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
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                            pub page_size:
                                                ::std::option::Option<::std::primitive::i64>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListCreativeStatusBreakdownByDetailResponse.nextPageToken returned from the previous call to the filteredBids.details.list method."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                    pub mod impression_metrics {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListImpressionMetricsResponse.nextPageToken returned from the previous call to the impressionMetrics.list method."]
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
                    pub mod losing_bids {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListLosingBidsResponse.nextPageToken returned from the previous call to the losingBids.list method."]
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
                    pub mod non_billable_winning_bids {
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
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Requested page size. The server may return fewer results than requested. If unspecified, the server will pick an appropriate default."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "A token identifying a page of results the server should return. Typically, this is the value of ListNonBillableWinningBidsResponse.nextPageToken returned from the previous call to the nonBillableWinningBids.list method."]
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
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An absolute date range, specified by its start date and end date. The supported range of dates begins 30 days before today and ends today. Validity checked upon filter set creation. If a filter set with an absolute date range is run at a later date more than 30 days after start_date, it will fail."]
    pub struct AbsoluteDateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end date of the range (inclusive). Must be within the 30 days leading up to current date, and must be equal to or after start_date."]
        pub end_date: ::std::option::Option<::std::boxed::Box<Date>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start date of the range (inclusive). Must be within the 30 days leading up to current date, and must be equal to or before end_date."]
        pub start_date: ::std::option::Option<::std::boxed::Box<Date>>,
    }
    impl AbsoluteDateRange {
        pub fn builder() -> AbsoluteDateRangeBuilder {
            AbsoluteDateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to accept a proposal."]
    pub struct AcceptProposalRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last known client revision number of the proposal."]
        pub proposal_revision: ::std::option::Option<::std::string::String>,
    }
    impl AcceptProposalRequest {
        pub fn builder() -> AcceptProposalRequestBuilder {
            AcceptProposalRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents size of a single ad slot, or a creative."]
    pub struct AdSize {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the ad slot in pixels. This field will be present only when size type is `PIXEL`."]
        pub height: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size type of the ad slot."]
        pub size_type: ::std::option::Option<AdSizeSizeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the ad slot in pixels. This field will be present only when size type is `PIXEL`."]
        pub width: ::std::option::Option<::std::string::String>,
    }
    impl AdSize {
        pub fn builder() -> AdSizeBuilder {
            AdSizeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The size type of the ad slot."]
    pub enum AdSizeSizeTypeEnum {
        #[serde(rename = "SIZE_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined size type."]
        SizeTypeUnspecified,
        #[serde(rename = "PIXEL")]
        #[doc = "Ad slot with size specified by height and width in pixels."]
        Pixel,
        #[serde(rename = "INTERSTITIAL")]
        #[doc = "Special size to describe an interstitial ad slot."]
        Interstitial,
        #[serde(rename = "NATIVE")]
        #[doc = "Native (mobile) ads rendered by the publisher."]
        Native,
        #[serde(rename = "FLUID")]
        #[doc = "Fluid size (i.e., responsive size) can be resized automatically with the change of outside environment."]
        Fluid,
    }
    impl ::std::default::Default for AdSizeSizeTypeEnum {
        fn default() -> Self {
            Self::SizeTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detected ad technology provider information."]
    pub struct AdTechnologyProviders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedProviderIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The detected ad technology provider IDs for this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv for mapping of provider ID to provided name, a privacy policy URL, and a list of domains which can be attributed to the provider. If the creative contains provider IDs that are outside of those listed in the `BidRequest.adslot.consented_providers_settings.consented_providers` field on the (Google bid protocol)[https://developers.google.com/authorized-buyers/rtb/downloads/realtime-bidding-proto] and the `BidRequest.user.ext.consented_providers_settings.consented_providers` field on the (OpenRTB protocol)[https://developers.google.com/authorized-buyers/rtb/downloads/openrtb-adx-proto], and a bid is submitted with that creative for an impression that will serve to an EEA user, the bid will be filtered before the auction."]
        pub detected_provider_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasUnidentifiedProvider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the creative contains an unidentified ad technology provider. If true for a given creative, any bid submitted with that creative for an impression that will serve to an EEA user will be filtered before the auction."]
        pub has_unidentified_provider: ::std::option::Option<::std::primitive::bool>,
    }
    impl AdTechnologyProviders {
        pub fn builder() -> AdTechnologyProvidersBuilder {
            AdTechnologyProvidersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for associating a deal and a creative."]
    pub struct AddDealAssociationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "association")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The association between a creative and a deal that should be added."]
        pub association: ::std::option::Option<::std::boxed::Box<CreativeDealAssociation>>,
    }
    impl AddDealAssociationRequest {
        pub fn builder() -> AddDealAssociationRequestBuilder {
            AddDealAssociationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for adding a note to a given proposal."]
    pub struct AddNoteRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "note")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the note to add."]
        pub note: ::std::option::Option<::std::boxed::Box<Note>>,
    }
    impl AddNoteRequest {
        pub fn builder() -> AddNoteRequestBuilder {
            AddNoteRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The app type the restriction applies to for mobile device."]
    pub struct AppContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app types this restriction applies to."]
        pub app_types: ::std::option::Option<::std::vec::Vec<AppContextAppTypesEnum>>,
    }
    impl AppContext {
        pub fn builder() -> AppContextBuilder {
            AppContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum AppContextAppTypesEnum {
        #[serde(rename = "NATIVE")]
        #[doc = "Native app context."]
        Native,
        #[serde(rename = "WEB")]
        #[doc = "Mobile web app context."]
        Web,
    }
    impl ::std::default::Default for AppContextAppTypesEnum {
        fn default() -> Self {
            Self::Native
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The auction type the restriction applies to."]
    pub struct AuctionContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auctionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The auction types this restriction applies to."]
        pub auction_types: ::std::option::Option<::std::vec::Vec<AuctionContextAuctionTypesEnum>>,
    }
    impl AuctionContext {
        pub fn builder() -> AuctionContextBuilder {
            AuctionContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum AuctionContextAuctionTypesEnum {
        #[serde(rename = "OPEN_AUCTION")]
        #[doc = "The restriction applies to open auction."]
        OpenAuction,
        #[serde(rename = "DIRECT_DEALS")]
        #[doc = "The restriction applies to direct deals."]
        DirectDeals,
    }
    impl ::std::default::Default for AuctionContextAuctionTypesEnum {
        fn default() -> Self {
            Self::OpenAuction
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The set of metrics that are measured in numbers of bids, representing how many bids with the specified dimension values were considered eligible at each stage of the bidding funnel;"]
    pub struct BidMetricsRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids that Ad Exchange received from the buyer."]
        pub bids: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidsInAuction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids that were permitted to compete in the auction."]
        pub bids_in_auction: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billedImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids for which the buyer was billed."]
        pub billed_impressions: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionsWon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids that won the auction."]
        pub impressions_won: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "measurableImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids for which the corresponding impression was measurable for viewability (as defined by Active View)."]
        pub measurable_impressions: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reachedQueries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids that won the auction and also won the mediation waterfall (if any)."]
        pub reached_queries: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewableImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids for which the corresponding impression was viewable (as defined by Active View)."]
        pub viewable_impressions: ::std::option::Option<::std::boxed::Box<MetricValue>>,
    }
    impl BidMetricsRow {
        pub fn builder() -> BidMetricsRowBuilder {
            BidMetricsRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of impressions with the specified dimension values that were considered to have no applicable bids, as described by the specified status."]
    pub struct BidResponseWithoutBidsStatusRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions for which there was a bid response with the specified status."]
        pub impression_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status specifying why the bid responses were considered to have no applicable bids."]
        pub status: ::std::option::Option<BidResponseWithoutBidsStatusRowStatusEnum>,
    }
    impl BidResponseWithoutBidsStatusRow {
        pub fn builder() -> BidResponseWithoutBidsStatusRowBuilder {
            BidResponseWithoutBidsStatusRowBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status specifying why the bid responses were considered to have no applicable bids."]
    pub enum BidResponseWithoutBidsStatusRowStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined status. This value will never be returned in responses."]
        StatusUnspecified,
        #[serde(rename = "RESPONSES_WITHOUT_BIDS")]
        #[doc = "The response had no bids."]
        ResponsesWithoutBids,
        #[serde(rename = "RESPONSES_WITHOUT_BIDS_FOR_ACCOUNT")]
        #[doc = "The response had no bids for the specified account, though it may have included bids on behalf of other accounts. Applies if: 1. Request is on behalf of a bidder and an account filter is present. 2. Request is on behalf of a child seat."]
        ResponsesWithoutBidsForAccount,
        #[serde(rename = "RESPONSES_WITHOUT_BIDS_FOR_DEAL")]
        #[doc = "The response had no bids for the specified deal, though it may have included bids on other deals on behalf of the account to which the deal belongs. If request is on behalf of a bidder and an account filter is not present, this also includes responses that have bids on behalf of accounts other than the account to which the deal belongs."]
        ResponsesWithoutBidsForDeal,
    }
    impl ::std::default::Default for BidResponseWithoutBidsStatusRowStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a buyer of inventory. Each buyer is identified by a unique Authorized Buyers account ID."]
    pub struct Buyer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Authorized Buyers account ID of the buyer."]
        pub account_id: ::std::option::Option<::std::string::String>,
    }
    impl Buyer {
        pub fn builder() -> BuyerBuilder {
            BuyerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of impressions with the specified dimension values where the corresponding bid request or bid response was not successful, as described by the specified callout status."]
    pub struct CalloutStatusRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calloutStatusId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the callout status. See [callout-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/callout-status-codes)."]
        pub callout_status_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions for which there was a bid request or bid response with the specified callout status."]
        pub impression_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
    }
    impl CalloutStatusRow {
        pub fn builder() -> CalloutStatusRowBuilder {
            CalloutStatusRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request to cancel an ongoing negotiation."]
    pub struct CancelNegotiationRequest {}
    impl CancelNegotiationRequest {
        pub fn builder() -> CancelNegotiationRequestBuilder {
            CancelNegotiationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A client resource represents a client buyer—an agency, a brand, or an advertiser customer of the sponsor buyer. Users associated with the client buyer have restricted access to the Marketplace and certain other sections of the Authorized Buyers UI based on the role granted to the client buyer. All fields are required unless otherwise specified."]
    pub struct Client {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The globally-unique numerical ID of the client. The value of this field is ignored in create and update operations."]
        pub client_account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name used to represent this client to publishers. You may have multiple clients that map to the same entity, but for each client the combination of `clientName` and entity must be unique. You can specify this field as empty."]
        pub client_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numerical identifier of the client entity. The entity can be an advertiser, a brand, or an agency. This identifier is unique among all the entities with the same type. The value of this field is ignored if the entity type is not provided. A list of all known advertisers with their identifiers is available in the [advertisers.txt](https://storage.googleapis.com/adx-rtb-dictionaries/advertisers.txt) file. A list of all known brands with their identifiers is available in the [brands.txt](https://storage.googleapis.com/adx-rtb-dictionaries/brands.txt) file. A list of all known agencies with their identifiers is available in the [agencies.txt](https://storage.googleapis.com/adx-rtb-dictionaries/agencies.txt) file."]
        pub entity_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the entity. This field is automatically fetched based on the type and ID. The value of this field is ignored in create and update operations."]
        pub entity_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`."]
        pub entity_type: ::std::option::Option<ClientEntityTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partnerClientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional arbitrary unique identifier of this client buyer from the standpoint of its Ad Exchange sponsor buyer. This field can be used to associate a client buyer with the identifier in the namespace of its sponsor buyer, lookup client buyers by that identifier and verify whether an Ad Exchange counterpart of a given client buyer already exists. If present, must be unique among all the client buyers for its Ad Exchange sponsor buyer."]
        pub partner_client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`."]
        pub role: ::std::option::Option<ClientRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the client buyer."]
        pub status: ::std::option::Option<ClientStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleToSeller")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the client buyer will be visible to sellers."]
        pub visible_to_seller: ::std::option::Option<::std::primitive::bool>,
    }
    impl Client {
        pub fn builder() -> ClientBuilder {
            ClientBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "An optional field for specifying the type of the client entity: `ADVERTISER`, `BRAND`, or `AGENCY`."]
    pub enum ClientEntityTypeEnum {
        #[serde(rename = "ENTITY_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined client entity type. Should not be used."]
        EntityTypeUnspecified,
        #[serde(rename = "ADVERTISER")]
        #[doc = "An advertiser."]
        Advertiser,
        #[serde(rename = "BRAND")]
        #[doc = "A brand."]
        Brand,
        #[serde(rename = "AGENCY")]
        #[doc = "An advertising agency."]
        Agency,
        #[serde(rename = "ENTITY_TYPE_UNCLASSIFIED")]
        #[doc = "An explicit value for a client that was not yet classified as any particular entity."]
        EntityTypeUnclassified,
    }
    impl ::std::default::Default for ClientEntityTypeEnum {
        fn default() -> Self {
            Self::EntityTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The role which is assigned to the client buyer. Each role implies a set of permissions granted to the client. Must be one of `CLIENT_DEAL_VIEWER`, `CLIENT_DEAL_NEGOTIATOR` or `CLIENT_DEAL_APPROVER`."]
    pub enum ClientRoleEnum {
        #[serde(rename = "CLIENT_ROLE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined client role."]
        ClientRoleUnspecified,
        #[serde(rename = "CLIENT_DEAL_VIEWER")]
        #[doc = "Users associated with this client can see publisher deal offers in the Marketplace. They can neither negotiate proposals nor approve deals. If this client is visible to publishers, they can send deal proposals to this client."]
        ClientDealViewer,
        #[serde(rename = "CLIENT_DEAL_NEGOTIATOR")]
        #[doc = "Users associated with this client can respond to deal proposals sent to them by publishers. They can also initiate deal proposals of their own."]
        ClientDealNegotiator,
        #[serde(rename = "CLIENT_DEAL_APPROVER")]
        #[doc = "Users associated with this client can approve eligible deals on your behalf. Some deals may still explicitly require publisher finalization. If this role is not selected, the sponsor buyer will need to manually approve each of their deals."]
        ClientDealApprover,
    }
    impl ::std::default::Default for ClientRoleEnum {
        fn default() -> Self {
            Self::ClientRoleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the client buyer."]
    pub enum ClientStatusEnum {
        #[serde(rename = "CLIENT_STATUS_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined client status."]
        ClientStatusUnspecified,
        #[serde(rename = "DISABLED")]
        #[doc = "A client that is currently disabled."]
        Disabled,
        #[serde(rename = "ACTIVE")]
        #[doc = "A client that is currently active."]
        Active,
    }
    impl ::std::default::Default for ClientStatusEnum {
        fn default() -> Self {
            Self::ClientStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A client user is created under a client buyer and has restricted access to the Marketplace and certain other sections of the Authorized Buyers UI based on the role granted to the associated client buyer. The only way a new client user can be created is via accepting an email invitation (see the accounts.clients.invitations.create method). All fields are required unless otherwise specified."]
    pub struct ClientUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numerical account ID of the client buyer with which the user is associated; the buyer must be a client of the current sponsor buyer. The value of this field is ignored in an update operation."]
        pub client_account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's email address. The value of this field is ignored in an update operation."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the client user."]
        pub status: ::std::option::Option<ClientUserStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique numerical ID of the client user that has accepted an invitation. The value of this field is ignored in an update operation."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl ClientUser {
        pub fn builder() -> ClientUserBuilder {
            ClientUserBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the client user."]
    pub enum ClientUserStatusEnum {
        #[serde(rename = "USER_STATUS_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined user status."]
        UserStatusUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "A user who was already created but hasn't accepted the invitation yet."]
        Pending,
        #[serde(rename = "ACTIVE")]
        #[doc = "A user that is currently active."]
        Active,
        #[serde(rename = "DISABLED")]
        #[doc = "A user that is currently disabled."]
        Disabled,
    }
    impl ::std::default::Default for ClientUserStatusEnum {
        fn default() -> Self {
            Self::UserStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An invitation for a new client user to get access to the Authorized Buyers UI. All fields are required unless otherwise specified."]
    pub struct ClientUserInvitation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Numerical account ID of the client buyer that the invited user is associated with. The value of this field is ignored in create operations."]
        pub client_account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email address to which the invitation is sent. Email addresses should be unique among all client users under each sponsor buyer."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique numerical ID of the invitation that is sent to the user. The value of this field is ignored in create operations."]
        pub invitation_id: ::std::option::Option<::std::string::String>,
    }
    impl ClientUserInvitation {
        pub fn builder() -> ClientUserInvitationBuilder {
            ClientUserInvitationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for indicating that the proposal's setup step is complete."]
    pub struct CompleteSetupRequest {}
    impl CompleteSetupRequest {
        pub fn builder() -> CompleteSetupRequestBuilder {
            CompleteSetupRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information on how a buyer or seller can be reached."]
    pub struct ContactInformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address for the contact."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the contact."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl ContactInformation {
        pub fn builder() -> ContactInformationBuilder {
            ContactInformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. Shows any corrections that were applied to this creative."]
    pub struct Correction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contexts for the correction."]
        pub contexts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServingContext>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional details about what was corrected."]
        pub details: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of correction that was applied to the creative."]
        pub _type: ::std::option::Option<CorrectionTypeEnum>,
    }
    impl Correction {
        pub fn builder() -> CorrectionBuilder {
            CorrectionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of correction that was applied to the creative."]
    pub enum CorrectionTypeEnum {
        #[serde(rename = "CORRECTION_TYPE_UNSPECIFIED")]
        #[doc = "The correction type is unknown. Refer to the details for more information."]
        CorrectionTypeUnspecified,
        #[serde(rename = "VENDOR_IDS_ADDED")]
        #[doc = "The ad's declared vendors did not match the vendors that were detected. The detected vendors were added."]
        VendorIdsAdded,
        #[serde(rename = "SSL_ATTRIBUTE_REMOVED")]
        #[doc = "The ad had the SSL attribute declared but was not SSL-compliant. The SSL attribute was removed."]
        SslAttributeRemoved,
        #[serde(rename = "FLASH_FREE_ATTRIBUTE_REMOVED")]
        #[doc = "The ad was declared as Flash-free but contained Flash, so the Flash-free attribute was removed."]
        FlashFreeAttributeRemoved,
        #[serde(rename = "FLASH_FREE_ATTRIBUTE_ADDED")]
        #[doc = "The ad was not declared as Flash-free but it did not reference any flash content, so the Flash-free attribute was added."]
        FlashFreeAttributeAdded,
        #[serde(rename = "REQUIRED_ATTRIBUTE_ADDED")]
        #[doc = "The ad did not declare a required creative attribute. The attribute was added."]
        RequiredAttributeAdded,
        #[serde(rename = "REQUIRED_VENDOR_ADDED")]
        #[doc = "The ad did not declare a required technology vendor. The technology vendor was added."]
        RequiredVendorAdded,
        #[serde(rename = "SSL_ATTRIBUTE_ADDED")]
        #[doc = "The ad did not declare the SSL attribute but was SSL-compliant, so the SSL attribute was added."]
        SslAttributeAdded,
        #[serde(rename = "IN_BANNER_VIDEO_ATTRIBUTE_ADDED")]
        #[doc = "Properties consistent with In-banner video were found, so an In-Banner Video attribute was added."]
        InBannerVideoAttributeAdded,
        #[serde(rename = "MRAID_ATTRIBUTE_ADDED")]
        #[doc = "The ad makes calls to the MRAID API so the MRAID attribute was added."]
        MraidAttributeAdded,
        #[serde(rename = "FLASH_ATTRIBUTE_REMOVED")]
        #[doc = "The ad unnecessarily declared the Flash attribute, so the Flash attribute was removed."]
        FlashAttributeRemoved,
        #[serde(rename = "VIDEO_IN_SNIPPET_ATTRIBUTE_ADDED")]
        #[doc = "The ad contains video content."]
        VideoInSnippetAttributeAdded,
    }
    impl ::std::default::Default for CorrectionTypeEnum {
        fn default() -> Self {
            Self::CorrectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A creative and its classification data."]
    pub struct Creative {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account that this creative belongs to. Can be used to filter the response of the creatives.list method."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adChoicesDestinationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to AdChoices destination page."]
        pub ad_choices_destination_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adTechnologyProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The detected ad technology providers."]
        pub ad_technology_providers:
            ::std::option::Option<::std::boxed::Box<AdTechnologyProviders>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the company being advertised in the creative."]
        pub advertiser_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agencyId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The agency ID for this creative."]
        pub agency_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The last update timestamp of the creative via API."]
        pub api_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All attributes for the ads that may be shown from this creative. Can be used to filter the response of the creatives.list method."]
        pub attributes: ::std::option::Option<::std::vec::Vec<CreativeAttributesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickThroughUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of destination URLs for the creative."]
        pub click_through_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "corrections")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Shows any corrections that were applied to this creative."]
        pub corrections: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Correction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The buyer-defined creative ID of this creative. Can be used to filter the response of the creatives.list method."]
        pub creative_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealsStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method."]
        pub deals_status: ::std::option::Option<CreativeDealsStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "declaredClickThroughUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of declared destination URLs for the creative."]
        pub declared_click_through_urls:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedAdvertiserIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Detected advertiser IDs, if any."]
        pub detected_advertiser_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedDomains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The detected domains for this creative."]
        pub detected_domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedLanguages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The detected languages for this creative. The order is arbitrary. The codes are 2 or 5 characters and are documented at https://developers.google.com/adwords/api/docs/appendix/languagecodes."]
        pub detected_languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedProductCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Detected product categories, if any. See the ad-product-categories.txt file in the technical documentation for a list of IDs."]
        pub detected_product_categories:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectedSensitiveCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Detected sensitive categories, if any. See the ad-sensitive-categories.txt file in the technical documentation for a list of IDs. You should use these IDs along with the excluded-sensitive-category field in the bid request to filter your bids."]
        pub detected_sensitive_categories:
            ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "html")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An HTML creative."]
        pub html: ::std::option::Option<::std::boxed::Box<HtmlContent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionTrackingUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of URLs to be called to record an impression."]
        pub impression_tracking_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "native")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A native creative."]
        pub native: ::std::option::Option<::std::boxed::Box<NativeContent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openAuctionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method."]
        pub open_auction_status: ::std::option::Option<CreativeOpenAuctionStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictedCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All restricted categories for the ads that may be shown from this creative."]
        pub restricted_categories:
            ::std::option::Option<::std::vec::Vec<CreativeRestrictedCategoriesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servingRestrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The granular status of this ad in specific contexts. A context here relates to where something ultimately serves (for example, a physical location, a platform, an HTTPS vs HTTP request, or the type of auction)."]
        pub serving_restrictions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServingRestriction>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vendorIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All vendor IDs for the ads that may be shown from this creative. See https://storage.googleapis.com/adx-rtb-dictionaries/vendors.txt for possible values."]
        pub vendor_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The version of this creative."]
        pub version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "video")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A video creative."]
        pub video: ::std::option::Option<::std::boxed::Box<VideoContent>>,
    }
    impl Creative {
        pub fn builder() -> CreativeBuilder {
            CreativeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeAttributesEnum {
        #[serde(rename = "ATTRIBUTE_UNSPECIFIED")]
        #[doc = "Do not use. This is a placeholder value only."]
        AttributeUnspecified,
        #[serde(rename = "IMAGE_RICH_MEDIA")]
        #[doc = "The creative is of type image/rich media. For pretargeting."]
        ImageRichMedia,
        #[serde(rename = "ADOBE_FLASH_FLV")]
        #[doc = "The creative is of video type Adobe Flash FLV. For pretargeting."]
        AdobeFlashFlv,
        #[serde(rename = "IS_TAGGED")]
        #[doc = "The creative is tagged."]
        IsTagged,
        #[serde(rename = "IS_COOKIE_TARGETED")]
        #[doc = "The creative is cookie targeted."]
        IsCookieTargeted,
        #[serde(rename = "IS_USER_INTEREST_TARGETED")]
        #[doc = "The creative is user interest targeted."]
        IsUserInterestTargeted,
        #[serde(rename = "EXPANDING_DIRECTION_NONE")]
        #[doc = "The creative does not expand."]
        ExpandingDirectionNone,
        #[serde(rename = "EXPANDING_DIRECTION_UP")]
        #[doc = "The creative expands up."]
        ExpandingDirectionUp,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN")]
        #[doc = "The creative expands down."]
        ExpandingDirectionDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT")]
        #[doc = "The creative expands left."]
        ExpandingDirectionLeft,
        #[serde(rename = "EXPANDING_DIRECTION_RIGHT")]
        #[doc = "The creative expands right."]
        ExpandingDirectionRight,
        #[serde(rename = "EXPANDING_DIRECTION_UP_LEFT")]
        #[doc = "The creative expands up and left."]
        ExpandingDirectionUpLeft,
        #[serde(rename = "EXPANDING_DIRECTION_UP_RIGHT")]
        #[doc = "The creative expands up and right."]
        ExpandingDirectionUpRight,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_LEFT")]
        #[doc = "The creative expands down and left."]
        ExpandingDirectionDownLeft,
        #[serde(rename = "EXPANDING_DIRECTION_DOWN_RIGHT")]
        #[doc = "The creative expands down and right."]
        ExpandingDirectionDownRight,
        #[serde(rename = "CREATIVE_TYPE_HTML")]
        #[doc = "The creative type is HTML."]
        CreativeTypeHtml,
        #[serde(rename = "CREATIVE_TYPE_VAST_VIDEO")]
        #[doc = "The creative type is VAST video."]
        CreativeTypeVastVideo,
        #[serde(rename = "EXPANDING_DIRECTION_UP_OR_DOWN")]
        #[doc = "The creative expands up or down."]
        ExpandingDirectionUpOrDown,
        #[serde(rename = "EXPANDING_DIRECTION_LEFT_OR_RIGHT")]
        #[doc = "The creative expands left or right."]
        ExpandingDirectionLeftOrRight,
        #[serde(rename = "EXPANDING_DIRECTION_ANY_DIAGONAL")]
        #[doc = "The creative expands on any diagonal."]
        ExpandingDirectionAnyDiagonal,
        #[serde(rename = "EXPANDING_ACTION_ROLLOVER_TO_EXPAND")]
        #[doc = "The creative expands when rolled over."]
        ExpandingActionRolloverToExpand,
        #[serde(rename = "INSTREAM_VAST_VIDEO_TYPE_VPAID_FLASH")]
        #[doc = "The instream vast video type is vpaid flash."]
        InstreamVastVideoTypeVpaidFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_MRAID")]
        #[doc = "The creative is MRAID."]
        RichMediaCapabilityTypeMraid,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_FLASH")]
        #[doc = "The creative is Flash."]
        RichMediaCapabilityTypeFlash,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_HTML5")]
        #[doc = "The creative is HTML5."]
        RichMediaCapabilityTypeHtml5,
        #[serde(rename = "SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of skippable instream video. For pretargeting."]
        SkippableInstreamVideo,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_SSL")]
        #[doc = "The creative is SSL."]
        RichMediaCapabilityTypeSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_NON_SSL")]
        #[doc = "The creative is non-SSL."]
        RichMediaCapabilityTypeNonSsl,
        #[serde(rename = "RICH_MEDIA_CAPABILITY_TYPE_INTERSTITIAL")]
        #[doc = "The creative is an interstitial."]
        RichMediaCapabilityTypeInterstitial,
        #[serde(rename = "NON_SKIPPABLE_INSTREAM_VIDEO")]
        #[doc = "The creative has an instream VAST video type of non-skippable instream video. For pretargeting."]
        NonSkippableInstreamVideo,
        #[serde(rename = "NATIVE_ELIGIBILITY_ELIGIBLE")]
        #[doc = "The creative is eligible for native."]
        NativeEligibilityEligible,
        #[serde(rename = "NON_VPAID")]
        #[doc = "The creative has an instream VAST video type of non-VPAID. For pretargeting."]
        NonVpaid,
        #[serde(rename = "NATIVE_ELIGIBILITY_NOT_ELIGIBLE")]
        #[doc = "The creative is not eligible for native."]
        NativeEligibilityNotEligible,
        #[serde(rename = "ANY_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of any interstitial. For pretargeting."]
        AnyInterstitial,
        #[serde(rename = "NON_INTERSTITIAL")]
        #[doc = "The creative has an interstitial size of non interstitial. For pretargeting."]
        NonInterstitial,
        #[serde(rename = "IN_BANNER_VIDEO")]
        #[doc = "The video type is in-banner video."]
        InBannerVideo,
        #[serde(rename = "RENDERING_SIZELESS_ADX")]
        #[doc = "The creative can dynamically resize to fill a variety of slot sizes."]
        RenderingSizelessAdx,
        #[serde(rename = "OMSDK_1_0")]
        #[doc = "The open measurement SDK is supported."]
        Omsdk10,
    }
    impl ::std::default::Default for CreativeAttributesEnum {
        fn default() -> Self {
            Self::AttributeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The top-level deals status of this creative. If disapproved, an entry for 'auctionType=DIRECT_DEALS' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method."]
    pub enum CreativeDealsStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "The status is unknown."]
        StatusUnspecified,
        #[serde(rename = "NOT_CHECKED")]
        #[doc = "The creative has not been checked."]
        NotChecked,
        #[serde(rename = "CONDITIONALLY_APPROVED")]
        #[doc = "The creative has been conditionally approved. See serving_restrictions for details."]
        ConditionallyApproved,
        #[serde(rename = "APPROVED")]
        #[doc = "The creative has been approved."]
        Approved,
        #[serde(rename = "DISAPPROVED")]
        #[doc = "The creative has been disapproved."]
        Disapproved,
        #[serde(rename = "PENDING_REVIEW")]
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        PendingReview,
        #[serde(rename = "STATUS_TYPE_UNSPECIFIED")]
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        StatusTypeUnspecified,
    }
    impl ::std::default::Default for CreativeDealsStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The top-level open auction status of this creative. If disapproved, an entry for 'auctionType = OPEN_AUCTION' (or 'ALL') in serving_restrictions will also exist. Note that this may be nuanced with other contextual restrictions, in which case, it may be preferable to read from serving_restrictions directly. Can be used to filter the response of the creatives.list method."]
    pub enum CreativeOpenAuctionStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "The status is unknown."]
        StatusUnspecified,
        #[serde(rename = "NOT_CHECKED")]
        #[doc = "The creative has not been checked."]
        NotChecked,
        #[serde(rename = "CONDITIONALLY_APPROVED")]
        #[doc = "The creative has been conditionally approved. See serving_restrictions for details."]
        ConditionallyApproved,
        #[serde(rename = "APPROVED")]
        #[doc = "The creative has been approved."]
        Approved,
        #[serde(rename = "DISAPPROVED")]
        #[doc = "The creative has been disapproved."]
        Disapproved,
        #[serde(rename = "PENDING_REVIEW")]
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        PendingReview,
        #[serde(rename = "STATUS_TYPE_UNSPECIFIED")]
        #[doc = "Placeholder for transition to v1beta1. Currently not used."]
        StatusTypeUnspecified,
    }
    impl ::std::default::Default for CreativeOpenAuctionStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeRestrictedCategoriesEnum {
        #[serde(rename = "NO_RESTRICTED_CATEGORIES")]
        #[doc = "The ad has no restricted categories"]
        NoRestrictedCategories,
        #[serde(rename = "ALCOHOL")]
        #[doc = "The alcohol restricted category."]
        Alcohol,
    }
    impl ::std::default::Default for CreativeRestrictedCategoriesEnum {
        fn default() -> Self {
            Self::NoRestrictedCategories
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The association between a creative and a deal."]
    pub struct CreativeDealAssociation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account the creative belongs to."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the creative associated with the deal."]
        pub creative_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealsId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The externalDealId for the deal associated with the creative."]
        pub deals_id: ::std::option::Option<::std::string::String>,
    }
    impl CreativeDealAssociation {
        pub fn builder() -> CreativeDealAssociationBuilder {
            CreativeDealAssociationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents creative restrictions associated to Programmatic Guaranteed/ Preferred Deal in Ad Manager. This doesn't apply to Private Auction and AdX Preferred Deals."]
    pub struct CreativeRestrictions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the environment that the creatives will be displayed in."]
        pub creative_format: ::std::option::Option<CreativeRestrictionsCreativeFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeSpecifications")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub creative_specifications:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeSpecification>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippableAdType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Skippable video ads allow viewers to skip ads after 5 seconds."]
        pub skippable_ad_type: ::std::option::Option<CreativeRestrictionsSkippableAdTypeEnum>,
    }
    impl CreativeRestrictions {
        pub fn builder() -> CreativeRestrictionsBuilder {
            CreativeRestrictionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The format of the environment that the creatives will be displayed in."]
    pub enum CreativeRestrictionsCreativeFormatEnum {
        #[serde(rename = "CREATIVE_FORMAT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined creative format."]
        CreativeFormatUnspecified,
        #[serde(rename = "DISPLAY")]
        #[doc = "A creative that will be displayed in environments such as a browser."]
        Display,
        #[serde(rename = "VIDEO")]
        #[doc = "A video creative that will be displayed in environments such as a video player."]
        Video,
    }
    impl ::std::default::Default for CreativeRestrictionsCreativeFormatEnum {
        fn default() -> Self {
            Self::CreativeFormatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Skippable video ads allow viewers to skip ads after 5 seconds."]
    pub enum CreativeRestrictionsSkippableAdTypeEnum {
        #[serde(rename = "SKIPPABLE_AD_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined skippable ad type."]
        SkippableAdTypeUnspecified,
        #[serde(rename = "SKIPPABLE")]
        #[doc = "This video ad can be skipped after 5 seconds."]
        Skippable,
        #[serde(rename = "INSTREAM_SELECT")]
        #[doc = "This video ad can be skipped after 5 seconds, and is counted as engaged view after 30 seconds. The creative is hosted on YouTube only, and viewcount of the YouTube video increments after the engaged view."]
        InstreamSelect,
        #[serde(rename = "NOT_SKIPPABLE")]
        #[doc = "This video ad is not skippable."]
        NotSkippable,
    }
    impl ::std::default::Default for CreativeRestrictionsSkippableAdTypeEnum {
        fn default() -> Self {
            Self::SkippableAdTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the size of the creative."]
    pub struct CreativeSize {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedFormats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "What formats are allowed by the publisher. If this repeated field is empty then all formats are allowed. For example, if this field contains AllowedFormatType.AUDIO then the publisher only allows an audio ad (without any video)."]
        pub allowed_formats: ::std::option::Option<::std::vec::Vec<CreativeSizeAllowedFormatsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "companionSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For video creatives specifies the sizes of companion ads (if present). Companion sizes may be filled in only when creative_size_type = VIDEO"]
        pub companion_sizes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Size>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeSizeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creative size type."]
        pub creative_size_type: ::std::option::Option<CreativeSizeCreativeSizeTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nativeTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The native template for this creative. It will have a value only if creative_size_type = CreativeSizeType.NATIVE."]
        pub native_template: ::std::option::Option<CreativeSizeNativeTemplateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For regular or video creative size type, specifies the size of the creative"]
        pub size: ::std::option::Option<::std::boxed::Box<Size>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skippableAdType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of skippable ad for this creative. It will have a value only if creative_size_type = CreativeSizeType.VIDEO."]
        pub skippable_ad_type: ::std::option::Option<CreativeSizeSkippableAdTypeEnum>,
    }
    impl CreativeSize {
        pub fn builder() -> CreativeSizeBuilder {
            CreativeSizeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum CreativeSizeAllowedFormatsEnum {
        #[serde(rename = "UNKNOWN")]
        #[doc = "A placeholder for an undefined allowed format."]
        Unknown,
        #[serde(rename = "AUDIO")]
        #[doc = "An audio-only ad (without any video)."]
        Audio,
    }
    impl ::std::default::Default for CreativeSizeAllowedFormatsEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The creative size type."]
    pub enum CreativeSizeCreativeSizeTypeEnum {
        #[serde(rename = "CREATIVE_SIZE_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined creative size type."]
        CreativeSizeTypeUnspecified,
        #[serde(rename = "REGULAR")]
        #[doc = "The creative is a regular desktop creative."]
        Regular,
        #[serde(rename = "INTERSTITIAL")]
        #[doc = "The creative is an interstitial creative."]
        Interstitial,
        #[serde(rename = "VIDEO")]
        #[doc = "The creative is a video creative."]
        Video,
        #[serde(rename = "NATIVE")]
        #[doc = "The creative is a native (mobile) creative."]
        Native,
    }
    impl ::std::default::Default for CreativeSizeCreativeSizeTypeEnum {
        fn default() -> Self {
            Self::CreativeSizeTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The native template for this creative. It will have a value only if creative_size_type = CreativeSizeType.NATIVE."]
    pub enum CreativeSizeNativeTemplateEnum {
        #[serde(rename = "UNKNOWN_NATIVE_TEMPLATE")]
        #[doc = "A placeholder for an undefined native template."]
        UnknownNativeTemplate,
        #[serde(rename = "NATIVE_CONTENT_AD")]
        #[doc = "The creative is linked to native content ad."]
        NativeContentAd,
        #[serde(rename = "NATIVE_APP_INSTALL_AD")]
        #[doc = "The creative is linked to native app install ad."]
        NativeAppInstallAd,
        #[serde(rename = "NATIVE_VIDEO_CONTENT_AD")]
        #[doc = "The creative is linked to native video content ad."]
        NativeVideoContentAd,
        #[serde(rename = "NATIVE_VIDEO_APP_INSTALL_AD")]
        #[doc = "The creative is linked to native video app install ad."]
        NativeVideoAppInstallAd,
    }
    impl ::std::default::Default for CreativeSizeNativeTemplateEnum {
        fn default() -> Self {
            Self::UnknownNativeTemplate
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of skippable ad for this creative. It will have a value only if creative_size_type = CreativeSizeType.VIDEO."]
    pub enum CreativeSizeSkippableAdTypeEnum {
        #[serde(rename = "SKIPPABLE_AD_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined skippable ad type."]
        SkippableAdTypeUnspecified,
        #[serde(rename = "GENERIC")]
        #[doc = "This video ad can be skipped after 5 seconds."]
        Generic,
        #[serde(rename = "INSTREAM_SELECT")]
        #[doc = "This video ad can be skipped after 5 seconds, and count as engaged view after 30 seconds. The creative is hosted on YouTube only, and viewcount of the YouTube video increments after the engaged view."]
        InstreamSelect,
        #[serde(rename = "NOT_SKIPPABLE")]
        #[doc = "This video ad is not skippable."]
        NotSkippable,
    }
    impl ::std::default::Default for CreativeSizeSkippableAdTypeEnum {
        fn default() -> Self {
            Self::SkippableAdTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents information for a creative that is associated with a Programmatic Guaranteed/Preferred Deal in Ad Manager."]
    pub struct CreativeSpecification {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeCompanionSizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Companion sizes may be filled in only when this is a video creative."]
        pub creative_companion_sizes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdSize>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The size of the creative."]
        pub creative_size: ::std::option::Option<::std::boxed::Box<AdSize>>,
    }
    impl CreativeSpecification {
        pub fn builder() -> CreativeSpecificationBuilder {
            CreativeSpecificationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of bids with the specified dimension values that did not win the auction (either were filtered pre-auction or lost the auction), as described by the specified creative status."]
    pub struct CreativeStatusRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids with the specified status."]
        pub bid_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeStatusId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the creative status. See [creative-status-codes](https://developers.google.com/authorized-buyers/rtb/downloads/creative-status-codes)."]
        pub creative_status_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
    }
    impl CreativeStatusRow {
        pub fn builder() -> CreativeStatusRowBuilder {
            CreativeStatusRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Generic targeting used for targeting dimensions that contains a list of included and excluded numeric IDs."]
    pub struct CriteriaTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedCriteriaIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of numeric IDs to be excluded."]
        pub excluded_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetedCriteriaIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of numeric IDs to be included."]
        pub targeted_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CriteriaTargeting {
        pub fn builder() -> CriteriaTargetingBuilder {
            CriteriaTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a whole or partial calendar date, such as a birthday. The time of day and time zone are either specified elsewhere or are insignificant. The date is relative to the Gregorian Calendar. This can represent one of the following: * A full date, with non-zero year, month, and day values * A month and day value, with a zero year, such as an anniversary * A year on its own, with zero month and day values * A year and month value, with a zero day, such as a credit card expiration date Related types are google.type.TimeOfDay and `google.protobuf.Timestamp`."]
    pub struct Date {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "day")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Day of a month. Must be from 1 to 31 and valid for the year and month, or 0 to specify a year by itself or a year and month where the day isn't significant."]
        pub day: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "month")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Month of a year. Must be from 1 to 12, or 0 to specify a year without a month and day."]
        pub month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "year")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Year of the date. Must be from 1 to 9999, or 0 to specify a date without a year."]
        pub year: ::std::option::Option<::std::primitive::i64>,
    }
    impl Date {
        pub fn builder() -> DateBuilder {
            DateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Daypart targeting message that specifies if the ad can be shown only during certain parts of a day/week."]
    pub struct DayPart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfWeek")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The day of the week to target. If unspecified, applicable to all days."]
        pub day_of_week: ::std::option::Option<DayPartDayOfWeekEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ending time of the day for the ad to show (minute level granularity). The end time is exclusive. This field is not available for filtering in PQL queries."]
        pub end_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The starting time of day for the ad to show (minute level granularity). The start time is inclusive. This field is not available for filtering in PQL queries."]
        pub start_time: ::std::option::Option<::std::boxed::Box<TimeOfDay>>,
    }
    impl DayPart {
        pub fn builder() -> DayPartBuilder {
            DayPartBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The day of the week to target. If unspecified, applicable to all days."]
    pub enum DayPartDayOfWeekEnum {
        #[serde(rename = "DAY_OF_WEEK_UNSPECIFIED")]
        #[doc = "A placeholder for when the day of the week is not specified."]
        DayOfWeekUnspecified,
        #[serde(rename = "MONDAY")]
        #[doc = "Monday"]
        Monday,
        #[serde(rename = "TUESDAY")]
        #[doc = "Tuesday"]
        Tuesday,
        #[serde(rename = "WEDNESDAY")]
        #[doc = "Wednesday"]
        Wednesday,
        #[serde(rename = "THURSDAY")]
        #[doc = "Thursday"]
        Thursday,
        #[serde(rename = "FRIDAY")]
        #[doc = "Friday"]
        Friday,
        #[serde(rename = "SATURDAY")]
        #[doc = "Saturday"]
        Saturday,
        #[serde(rename = "SUNDAY")]
        #[doc = "Sunday"]
        Sunday,
    }
    impl ::std::default::Default for DayPartDayOfWeekEnum {
        fn default() -> Self {
            Self::DayOfWeekUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies the day part targeting criteria."]
    pub struct DayPartTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayParts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of day part targeting criterion."]
        pub day_parts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DayPart>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZoneType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timezone to use for interpreting the day part targeting."]
        pub time_zone_type: ::std::option::Option<DayPartTargetingTimeZoneTypeEnum>,
    }
    impl DayPartTargeting {
        pub fn builder() -> DayPartTargetingBuilder {
            DayPartTargetingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The timezone to use for interpreting the day part targeting."]
    pub enum DayPartTargetingTimeZoneTypeEnum {
        #[serde(rename = "TIME_ZONE_SOURCE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined time zone source."]
        TimeZoneSourceUnspecified,
        #[serde(rename = "PUBLISHER")]
        #[doc = "Use publisher's time zone setting."]
        Publisher,
        #[serde(rename = "USER")]
        #[doc = "Use the user's time zone setting."]
        User,
    }
    impl ::std::default::Default for DayPartTargetingTimeZoneTypeEnum {
        fn default() -> Self {
            Self::TimeZoneSourceUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A deal represents a segment of inventory for displaying ads on. A proposal can contain multiple deals. A deal contains the terms and targeting information that is used for serving."]
    pub struct Deal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Proposed flight end time of the deal. This will generally be stored in a granularity of a second. A value is not required for Private Auction deals or Preferred Deals."]
        pub available_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Proposed flight start time of the deal. This will generally be stored in the granularity of one second since deal serving starts at seconds boundary. Any time specified with more granularity (e.g., in milliseconds) will be truncated towards the start of time in seconds."]
        pub available_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyerPrivateData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Buyer private data (hidden from seller)."]
        pub buyer_private_data: ::std::option::Option<::std::boxed::Box<PrivateData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createProductId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The product ID from which this deal was created. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub create_product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createProductRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Revision number of the product that the deal was created from. If present on create, and the server `product_revision` has advanced since the passed-in `create_product_revision`, an `ABORTED` error will be returned. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub create_product_revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time of the deal creation."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativePreApprovalPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the creative pre-approval policy."]
        pub creative_pre_approval_policy: ::std::option::Option<DealCreativePreApprovalPolicyEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeRestrictions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Restricitions about the creatives associated with the deal (i.e., size) This is available for Programmatic Guaranteed/Preferred Deals in Ad Manager."]
        pub creative_restrictions: ::std::option::Option<::std::boxed::Box<CreativeRestrictions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeSafeFrameCompatibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies whether the creative is safeFrame compatible."]
        pub creative_safe_frame_compatibility:
            ::std::option::Option<DealCreativeSafeFrameCompatibilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A unique deal ID for the deal (server-assigned)."]
        pub deal_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealServingMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Metadata about the serving status of this deal."]
        pub deal_serving_metadata: ::std::option::Option<::std::boxed::Box<DealServingMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The negotiable terms of the deal."]
        pub deal_terms: ::std::option::Option<::std::boxed::Box<DealTerms>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryControl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of fields around delivery control that are interesting for a buyer to see but are non-negotiable. These are set by the publisher."]
        pub delivery_control: ::std::option::Option<::std::boxed::Box<DeliveryControl>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description for the deal terms."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the deal."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalDealId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The external deal ID assigned to this deal once the deal is finalized. This is the deal ID that shows up in serving/reporting etc."]
        pub external_deal_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSetupComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. True, if the buyside inventory setup is complete for this deal."]
        pub is_setup_complete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "programmaticCreativeSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by buyer."]
        pub programmatic_creative_source: ::std::option::Option<DealProgrammaticCreativeSourceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. ID of the proposal that this deal is part of."]
        pub proposal_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerContacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Seller contact information for the deal."]
        pub seller_contacts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syndicationProduct")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The syndication product associated with the deal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub syndication_product: ::std::option::Option<DealSyndicationProductEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies the subset of inventory targeted by the deal."]
        pub targeting: ::std::option::Option<::std::boxed::Box<MarketplaceTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingCriterion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shared targeting visible to buyers and sellers. Each shared targeting entity is AND'd together."]
        pub targeting_criterion:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingCriteria>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the deal was last updated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The web property code for the seller copied over from the product."]
        pub web_property_code: ::std::option::Option<::std::string::String>,
    }
    impl Deal {
        pub fn builder() -> DealBuilder {
            DealBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies the creative pre-approval policy."]
    pub enum DealCreativePreApprovalPolicyEnum {
        #[serde(rename = "CREATIVE_PRE_APPROVAL_POLICY_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined creative pre-approval policy."]
        CreativePreApprovalPolicyUnspecified,
        #[serde(rename = "SELLER_PRE_APPROVAL_REQUIRED")]
        #[doc = "The seller needs to approve each creative before it can serve."]
        SellerPreApprovalRequired,
        #[serde(rename = "SELLER_PRE_APPROVAL_NOT_REQUIRED")]
        #[doc = "The seller does not need to approve each creative before it can serve."]
        SellerPreApprovalNotRequired,
    }
    impl ::std::default::Default for DealCreativePreApprovalPolicyEnum {
        fn default() -> Self {
            Self::CreativePreApprovalPolicyUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies whether the creative is safeFrame compatible."]
    pub enum DealCreativeSafeFrameCompatibilityEnum {
        #[serde(rename = "CREATIVE_SAFE_FRAME_COMPATIBILITY_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined creative safe-frame compatibility."]
        CreativeSafeFrameCompatibilityUnspecified,
        #[serde(rename = "COMPATIBLE")]
        #[doc = "The creatives need to be compatible with the safe frame option."]
        Compatible,
        #[serde(rename = "INCOMPATIBLE")]
        #[doc = "The creatives can be incompatible with the safe frame option."]
        Incompatible,
    }
    impl ::std::default::Default for DealCreativeSafeFrameCompatibilityEnum {
        fn default() -> Self {
            Self::CreativeSafeFrameCompatibilityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies the creative source for programmatic deals. PUBLISHER means creative is provided by seller and ADVERTISER means creative is provided by buyer."]
    pub enum DealProgrammaticCreativeSourceEnum {
        #[serde(rename = "PROGRAMMATIC_CREATIVE_SOURCE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined programmatic creative source."]
        ProgrammaticCreativeSourceUnspecified,
        #[serde(rename = "ADVERTISER")]
        #[doc = "The advertiser provides the creatives."]
        Advertiser,
        #[serde(rename = "PUBLISHER")]
        #[doc = "The publisher provides the creatives to be served."]
        Publisher,
    }
    impl ::std::default::Default for DealProgrammaticCreativeSourceEnum {
        fn default() -> Self {
            Self::ProgrammaticCreativeSourceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The syndication product associated with the deal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
    pub enum DealSyndicationProductEnum {
        #[serde(rename = "SYNDICATION_PRODUCT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined syndication product."]
        SyndicationProductUnspecified,
        #[serde(rename = "CONTENT")]
        #[doc = "This typically represents a web page."]
        Content,
        #[serde(rename = "MOBILE")]
        #[doc = "This represents a mobile property."]
        Mobile,
        #[serde(rename = "VIDEO")]
        #[doc = "This represents video ad formats."]
        Video,
        #[serde(rename = "GAMES")]
        #[doc = "This represents ads shown within games."]
        Games,
    }
    impl ::std::default::Default for DealSyndicationProductEnum {
        fn default() -> Self {
            Self::SyndicationProductUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tracks which parties (if any) have paused a deal. The deal is considered paused if either hasBuyerPaused or hasSellPaused is true."]
    pub struct DealPauseStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyerPauseReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The buyer's reason for pausing, if the buyer paused the deal."]
        pub buyer_pause_reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstPausedBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The role of the person who first paused this deal."]
        pub first_paused_by: ::std::option::Option<DealPauseStatusFirstPausedByEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasBuyerPaused")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True, if the buyer has paused the deal unilaterally."]
        pub has_buyer_paused: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasSellerPaused")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True, if the seller has paused the deal unilaterally."]
        pub has_seller_paused: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerPauseReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The seller's reason for pausing, if the seller paused the deal."]
        pub seller_pause_reason: ::std::option::Option<::std::string::String>,
    }
    impl DealPauseStatus {
        pub fn builder() -> DealPauseStatusBuilder {
            DealPauseStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The role of the person who first paused this deal."]
    pub enum DealPauseStatusFirstPausedByEnum {
        #[serde(rename = "BUYER_SELLER_ROLE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[serde(rename = "BUYER")]
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[serde(rename = "SELLER")]
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ::std::default::Default for DealPauseStatusFirstPausedByEnum {
        fn default() -> Self {
            Self::BuyerSellerRoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message captures metadata about the serving status of a deal."]
    pub struct DealServingMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealPauseStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Tracks which parties (if any) have paused a deal."]
        pub deal_pause_status: ::std::option::Option<::std::boxed::Box<DealPauseStatus>>,
    }
    impl DealServingMetadata {
        pub fn builder() -> DealServingMetadataBuilder {
            DealServingMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The deal terms specify the details of a Product/deal. They specify things like price per buyer, the type of pricing model (e.g., fixed price, auction) and expected impressions from the publisher."]
    pub struct DealTerms {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brandingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Visibility of the URL in bid requests. (default: BRANDED)"]
        pub branding_type: ::std::option::Option<DealTermsBrandingTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publisher provided description for the terms."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedGrossSpend")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-binding estimate of the estimated gross spend for this deal. Can be set by buyer or seller."]
        pub estimated_gross_spend: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "estimatedImpressionsPerDay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Non-binding estimate of the impressions served per day. Can be set by buyer or seller."]
        pub estimated_impressions_per_day: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guaranteedFixedPriceTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The terms for guaranteed fixed price deals."]
        pub guaranteed_fixed_price_terms:
            ::std::option::Option<::std::boxed::Box<GuaranteedFixedPriceTerms>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonGuaranteedAuctionTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The terms for non-guaranteed auction deals."]
        pub non_guaranteed_auction_terms:
            ::std::option::Option<::std::boxed::Box<NonGuaranteedAuctionTerms>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonGuaranteedFixedPriceTerms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The terms for non-guaranteed fixed price deals."]
        pub non_guaranteed_fixed_price_terms:
            ::std::option::Option<::std::boxed::Box<NonGuaranteedFixedPriceTerms>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerTimeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone name. For deals with Cost Per Day billing, defines the time zone used to mark the boundaries of a day. It should be an IANA TZ name, such as \"America/Los_Angeles\". For more information, see https://en.wikipedia.org/wiki/List_of_tz_database_time_zones."]
        pub seller_time_zone: ::std::option::Option<::std::string::String>,
    }
    impl DealTerms {
        pub fn builder() -> DealTermsBuilder {
            DealTermsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Visibility of the URL in bid requests. (default: BRANDED)"]
    pub enum DealTermsBrandingTypeEnum {
        #[serde(rename = "BRANDING_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined branding type."]
        BrandingTypeUnspecified,
        #[serde(rename = "BRANDED")]
        #[doc = "Full URL is included in bid requests."]
        Branded,
        #[serde(rename = "SEMI_TRANSPARENT")]
        #[doc = "A TopLevelDomain or masked URL is sent in bid requests rather than the full one."]
        SemiTransparent,
    }
    impl ::std::default::Default for DealTermsBrandingTypeEnum {
        fn default() -> Self {
            Self::BrandingTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message contains details about how the deals will be paced."]
    pub struct DeliveryControl {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeBlockingLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specified the creative blocking levels to be applied."]
        pub creative_blocking_level:
            ::std::option::Option<DeliveryControlCreativeBlockingLevelEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryRateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies how the impression delivery will be paced."]
        pub delivery_rate_type: ::std::option::Option<DeliveryControlDeliveryRateTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frequencyCaps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Specifies any frequency caps."]
        pub frequency_caps: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FrequencyCap>>>,
    }
    impl DeliveryControl {
        pub fn builder() -> DeliveryControlBuilder {
            DeliveryControlBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specified the creative blocking levels to be applied."]
    pub enum DeliveryControlCreativeBlockingLevelEnum {
        #[serde(rename = "CREATIVE_BLOCKING_LEVEL_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined creative blocking level."]
        CreativeBlockingLevelUnspecified,
        #[serde(rename = "PUBLISHER_BLOCKING_RULES")]
        #[doc = "Publisher blocking rules will be applied."]
        PublisherBlockingRules,
        #[serde(rename = "ADX_POLICY_BLOCKING_ONLY")]
        #[doc = "The Ad Exchange policy blocking rules will be applied."]
        AdxPolicyBlockingOnly,
    }
    impl ::std::default::Default for DeliveryControlCreativeBlockingLevelEnum {
        fn default() -> Self {
            Self::CreativeBlockingLevelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Specifies how the impression delivery will be paced."]
    pub enum DeliveryControlDeliveryRateTypeEnum {
        #[serde(rename = "DELIVERY_RATE_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined delivery rate type."]
        DeliveryRateTypeUnspecified,
        #[serde(rename = "EVENLY")]
        #[doc = "Impressions are served uniformly over the life of the deal."]
        Evenly,
        #[serde(rename = "FRONT_LOADED")]
        #[doc = "Impressions are served front-loaded."]
        FrontLoaded,
        #[serde(rename = "AS_FAST_AS_POSSIBLE")]
        #[doc = "Impressions are served as fast as possible."]
        AsFastAsPossible,
    }
    impl ::std::default::Default for DeliveryControlDeliveryRateTypeEnum {
        fn default() -> Self {
            Self::DeliveryRateTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The reason and details for a disapproval."]
    pub struct Disapproval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "details")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional details about the reason for disapproval."]
        pub details: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The categorized reason for disapproval."]
        pub reason: ::std::option::Option<DisapprovalReasonEnum>,
    }
    impl Disapproval {
        pub fn builder() -> DisapprovalBuilder {
            DisapprovalBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The categorized reason for disapproval."]
    pub enum DisapprovalReasonEnum {
        #[serde(rename = "LENGTH_OF_IMAGE_ANIMATION")]
        #[doc = "The length of the image animation is longer than allowed."]
        LengthOfImageAnimation,
        #[serde(rename = "BROKEN_URL")]
        #[doc = "The click through URL doesn't work properly."]
        BrokenUrl,
        #[serde(rename = "MEDIA_NOT_FUNCTIONAL")]
        #[doc = "Something is wrong with the creative itself."]
        MediaNotFunctional,
        #[serde(rename = "INVALID_FOURTH_PARTY_CALL")]
        #[doc = "The ad makes a fourth party call to an unapproved vendor."]
        InvalidFourthPartyCall,
        #[serde(rename = "INCORRECT_REMARKETING_DECLARATION")]
        #[doc = "The ad targets consumers using remarketing lists and/or collects data for subsequent use in retargeting, but does not correctly declare that use."]
        IncorrectRemarketingDeclaration,
        #[serde(rename = "LANDING_PAGE_ERROR")]
        #[doc = "Clicking on the ad leads to an error page."]
        LandingPageError,
        #[serde(rename = "AD_SIZE_DOES_NOT_MATCH_AD_SLOT")]
        #[doc = "The ad size when rendered does not match the declaration."]
        AdSizeDoesNotMatchAdSlot,
        #[serde(rename = "NO_BORDER")]
        #[doc = "Ads with a white background require a border, which was missing."]
        NoBorder,
        #[serde(rename = "FOURTH_PARTY_BROWSER_COOKIES")]
        #[doc = "The creative attempts to set cookies from a fourth party that is not certified."]
        FourthPartyBrowserCookies,
        #[serde(rename = "LSO_OBJECTS")]
        #[doc = "The creative sets an LSO object."]
        LsoObjects,
        #[serde(rename = "BLANK_CREATIVE")]
        #[doc = "The ad serves a blank."]
        BlankCreative,
        #[serde(rename = "DESTINATION_URLS_UNDECLARED")]
        #[doc = "The ad uses rotation, but not all destination URLs were declared."]
        DestinationUrlsUndeclared,
        #[serde(rename = "PROBLEM_WITH_CLICK_MACRO")]
        #[doc = "There is a problem with the way the click macro is used."]
        ProblemWithClickMacro,
        #[serde(rename = "INCORRECT_AD_TECHNOLOGY_DECLARATION")]
        #[doc = "The ad technology declaration is not accurate."]
        IncorrectAdTechnologyDeclaration,
        #[serde(rename = "INCORRECT_DESTINATION_URL_DECLARATION")]
        #[doc = "The actual destination URL does not match the declared destination URL."]
        IncorrectDestinationUrlDeclaration,
        #[serde(rename = "EXPANDABLE_INCORRECT_DIRECTION")]
        #[doc = "The declared expanding direction does not match the actual direction."]
        ExpandableIncorrectDirection,
        #[serde(rename = "EXPANDABLE_DIRECTION_NOT_SUPPORTED")]
        #[doc = "The ad does not expand in a supported direction."]
        ExpandableDirectionNotSupported,
        #[serde(rename = "EXPANDABLE_INVALID_VENDOR")]
        #[doc = "The ad uses an expandable vendor that is not supported."]
        ExpandableInvalidVendor,
        #[serde(rename = "EXPANDABLE_FUNCTIONALITY")]
        #[doc = "There was an issue with the expandable ad."]
        ExpandableFunctionality,
        #[serde(rename = "VIDEO_INVALID_VENDOR")]
        #[doc = "The ad uses a video vendor that is not supported."]
        VideoInvalidVendor,
        #[serde(rename = "VIDEO_UNSUPPORTED_LENGTH")]
        #[doc = "The length of the video ad is not supported."]
        VideoUnsupportedLength,
        #[serde(rename = "VIDEO_UNSUPPORTED_FORMAT")]
        #[doc = "The format of the video ad is not supported."]
        VideoUnsupportedFormat,
        #[serde(rename = "VIDEO_FUNCTIONALITY")]
        #[doc = "There was an issue with the video ad."]
        VideoFunctionality,
        #[serde(rename = "LANDING_PAGE_DISABLED")]
        #[doc = "The landing page does not conform to Ad Exchange policy."]
        LandingPageDisabled,
        #[serde(rename = "MALWARE_SUSPECTED")]
        #[doc = "The ad or the landing page may contain malware."]
        MalwareSuspected,
        #[serde(rename = "ADULT_IMAGE_OR_VIDEO")]
        #[doc = "The ad contains adult images or video content."]
        AdultImageOrVideo,
        #[serde(rename = "INACCURATE_AD_TEXT")]
        #[doc = "The ad contains text that is unclear or inaccurate."]
        InaccurateAdText,
        #[serde(rename = "COUNTERFEIT_DESIGNER_GOODS")]
        #[doc = "The ad promotes counterfeit designer goods."]
        CounterfeitDesignerGoods,
        #[serde(rename = "POP_UP")]
        #[doc = "The ad causes a popup window to appear."]
        PopUp,
        #[serde(rename = "INVALID_RTB_PROTOCOL_USAGE")]
        #[doc = "The creative does not follow policies set for the RTB protocol."]
        InvalidRtbProtocolUsage,
        #[serde(rename = "RAW_IP_ADDRESS_IN_SNIPPET")]
        #[doc = "The ad contains a URL that uses a numeric IP address for the domain."]
        RawIpAddressInSnippet,
        #[serde(rename = "UNACCEPTABLE_CONTENT_SOFTWARE")]
        #[doc = "The ad or landing page contains unacceptable content because it initiated a software or executable download."]
        UnacceptableContentSoftware,
        #[serde(rename = "UNAUTHORIZED_COOKIE_ON_GOOGLE_DOMAIN")]
        #[doc = "The ad set an unauthorized cookie on a Google domain."]
        UnauthorizedCookieOnGoogleDomain,
        #[serde(rename = "UNDECLARED_FLASH_OBJECTS")]
        #[doc = "Flash content found when no flash was declared."]
        UndeclaredFlashObjects,
        #[serde(rename = "INVALID_SSL_DECLARATION")]
        #[doc = "SSL support declared but not working correctly."]
        InvalidSslDeclaration,
        #[serde(rename = "DIRECT_DOWNLOAD_IN_AD")]
        #[doc = "Rich Media - Direct Download in Ad (ex. PDF download)."]
        DirectDownloadInAd,
        #[serde(rename = "MAXIMUM_DOWNLOAD_SIZE_EXCEEDED")]
        #[doc = "Maximum download size exceeded."]
        MaximumDownloadSizeExceeded,
        #[serde(rename = "DESTINATION_URL_SITE_NOT_CRAWLABLE")]
        #[doc = "Bad Destination URL: Site Not Crawlable."]
        DestinationUrlSiteNotCrawlable,
        #[serde(rename = "BAD_URL_LEGAL_DISAPPROVAL")]
        #[doc = "Bad URL: Legal disapproval."]
        BadUrlLegalDisapproval,
        #[serde(rename = "PHARMA_GAMBLING_ALCOHOL_NOT_ALLOWED")]
        #[doc = "Pharmaceuticals, Gambling, Alcohol not allowed and at least one was detected."]
        PharmaGamblingAlcoholNotAllowed,
        #[serde(rename = "DYNAMIC_DNS_AT_DESTINATION_URL")]
        #[doc = "Dynamic DNS at Destination URL."]
        DynamicDnsAtDestinationUrl,
        #[serde(rename = "POOR_IMAGE_OR_VIDEO_QUALITY")]
        #[doc = "Poor Image / Video Quality."]
        PoorImageOrVideoQuality,
        #[serde(rename = "UNACCEPTABLE_IMAGE_CONTENT")]
        #[doc = "For example, Image Trick to Click."]
        UnacceptableImageContent,
        #[serde(rename = "INCORRECT_IMAGE_LAYOUT")]
        #[doc = "Incorrect Image Layout."]
        IncorrectImageLayout,
        #[serde(rename = "IRRELEVANT_IMAGE_OR_VIDEO")]
        #[doc = "Irrelevant Image / Video."]
        IrrelevantImageOrVideo,
        #[serde(rename = "DESTINATION_SITE_DOES_NOT_ALLOW_GOING_BACK")]
        #[doc = "Broken back button."]
        DestinationSiteDoesNotAllowGoingBack,
        #[serde(rename = "MISLEADING_CLAIMS_IN_AD")]
        #[doc = "Misleading/Inaccurate claims in ads."]
        MisleadingClaimsInAd,
        #[serde(rename = "RESTRICTED_PRODUCTS")]
        #[doc = "Restricted Products."]
        RestrictedProducts,
        #[serde(rename = "UNACCEPTABLE_CONTENT")]
        #[doc = "Unacceptable content. For example, malware."]
        UnacceptableContent,
        #[serde(rename = "AUTOMATED_AD_CLICKING")]
        #[doc = "The ad automatically redirects to the destination site without a click, or reports a click when none were made."]
        AutomatedAdClicking,
        #[serde(rename = "INVALID_URL_PROTOCOL")]
        #[doc = "The ad uses URL protocols that do not exist or are not allowed on AdX."]
        InvalidUrlProtocol,
        #[serde(rename = "UNDECLARED_RESTRICTED_CONTENT")]
        #[doc = "Restricted content (for example, alcohol) was found in the ad but not declared."]
        UndeclaredRestrictedContent,
        #[serde(rename = "INVALID_REMARKETING_LIST_USAGE")]
        #[doc = "Violation of the remarketing list policy."]
        InvalidRemarketingListUsage,
        #[serde(rename = "DESTINATION_SITE_NOT_CRAWLABLE_ROBOTS_TXT")]
        #[doc = "The destination site's robot.txt file prevents it from being crawled."]
        DestinationSiteNotCrawlableRobotsTxt,
        #[serde(rename = "CLICK_TO_DOWNLOAD_NOT_AN_APP")]
        #[doc = "Click to download must link to an app."]
        ClickToDownloadNotAnApp,
        #[serde(rename = "INACCURATE_REVIEW_EXTENSION")]
        #[doc = "A review extension must be an accurate review."]
        InaccurateReviewExtension,
        #[serde(rename = "SEXUALLY_EXPLICIT_CONTENT")]
        #[doc = "Sexually explicit content."]
        SexuallyExplicitContent,
        #[serde(rename = "GAINING_AN_UNFAIR_ADVANTAGE")]
        #[doc = "The ad tries to gain an unfair traffic advantage."]
        GainingAnUnfairAdvantage,
        #[serde(rename = "GAMING_THE_GOOGLE_NETWORK")]
        #[doc = "The ad tries to circumvent Google's advertising systems."]
        GamingTheGoogleNetwork,
        #[serde(rename = "DANGEROUS_PRODUCTS_KNIVES")]
        #[doc = "The ad promotes dangerous knives."]
        DangerousProductsKnives,
        #[serde(rename = "DANGEROUS_PRODUCTS_EXPLOSIVES")]
        #[doc = "The ad promotes explosives."]
        DangerousProductsExplosives,
        #[serde(rename = "DANGEROUS_PRODUCTS_GUNS")]
        #[doc = "The ad promotes guns & parts."]
        DangerousProductsGuns,
        #[serde(rename = "DANGEROUS_PRODUCTS_DRUGS")]
        #[doc = "The ad promotes recreational drugs/services & related equipment."]
        DangerousProductsDrugs,
        #[serde(rename = "DANGEROUS_PRODUCTS_TOBACCO")]
        #[doc = "The ad promotes tobacco products/services & related equipment."]
        DangerousProductsTobacco,
        #[serde(rename = "DANGEROUS_PRODUCTS_WEAPONS")]
        #[doc = "The ad promotes weapons."]
        DangerousProductsWeapons,
        #[serde(rename = "UNCLEAR_OR_IRRELEVANT_AD")]
        #[doc = "The ad is unclear or irrelevant to the destination site."]
        UnclearOrIrrelevantAd,
        #[serde(rename = "PROFESSIONAL_STANDARDS")]
        #[doc = "The ad does not meet professional standards."]
        ProfessionalStandards,
        #[serde(rename = "DYSFUNCTIONAL_PROMOTION")]
        #[doc = "The promotion is unnecessarily difficult to navigate."]
        DysfunctionalPromotion,
        #[serde(rename = "INVALID_INTEREST_BASED_AD")]
        #[doc = "Violation of Google's policy for interest-based ads."]
        InvalidInterestBasedAd,
        #[serde(rename = "MISUSE_OF_PERSONAL_INFORMATION")]
        #[doc = "Misuse of personal information."]
        MisuseOfPersonalInformation,
        #[serde(rename = "OMISSION_OF_RELEVANT_INFORMATION")]
        #[doc = "Omission of relevant information."]
        OmissionOfRelevantInformation,
        #[serde(rename = "UNAVAILABLE_PROMOTIONS")]
        #[doc = "Unavailable promotions."]
        UnavailablePromotions,
        #[serde(rename = "MISLEADING_PROMOTIONS")]
        #[doc = "Misleading or unrealistic promotions."]
        MisleadingPromotions,
        #[serde(rename = "INAPPROPRIATE_CONTENT")]
        #[doc = "Offensive or inappropriate content."]
        InappropriateContent,
        #[serde(rename = "SENSITIVE_EVENTS")]
        #[doc = "Capitalizing on sensitive events."]
        SensitiveEvents,
        #[serde(rename = "SHOCKING_CONTENT")]
        #[doc = "Shocking content."]
        ShockingContent,
        #[serde(rename = "ENABLING_DISHONEST_BEHAVIOR")]
        #[doc = "Products & Services that enable dishonest behavior."]
        EnablingDishonestBehavior,
        #[serde(rename = "TECHNICAL_REQUIREMENTS")]
        #[doc = "The ad does not meet technical requirements."]
        TechnicalRequirements,
        #[serde(rename = "RESTRICTED_POLITICAL_CONTENT")]
        #[doc = "Restricted political content."]
        RestrictedPoliticalContent,
        #[serde(rename = "UNSUPPORTED_CONTENT")]
        #[doc = "Unsupported content."]
        UnsupportedContent,
        #[serde(rename = "INVALID_BIDDING_METHOD")]
        #[doc = "Invalid bidding method."]
        InvalidBiddingMethod,
        #[serde(rename = "VIDEO_TOO_LONG")]
        #[doc = "Video length exceeds limits."]
        VideoTooLong,
        #[serde(rename = "VIOLATES_JAPANESE_PHARMACY_LAW")]
        #[doc = "Unacceptable content: Japanese healthcare."]
        ViolatesJapanesePharmacyLaw,
        #[serde(rename = "UNACCREDITED_PET_PHARMACY")]
        #[doc = "Online pharmacy ID required."]
        UnaccreditedPetPharmacy,
        #[serde(rename = "ABORTION")]
        #[doc = "Unacceptable content: Abortion."]
        Abortion,
        #[serde(rename = "CONTRACEPTIVES")]
        #[doc = "Unacceptable content: Birth control."]
        Contraceptives,
        #[serde(rename = "NEED_CERTIFICATES_TO_ADVERTISE_IN_CHINA")]
        #[doc = "Restricted in China."]
        NeedCertificatesToAdvertiseInChina,
        #[serde(rename = "KCDSP_REGISTRATION")]
        #[doc = "Unacceptable content: Korean healthcare."]
        KcdspRegistration,
        #[serde(rename = "NOT_FAMILY_SAFE")]
        #[doc = "Non-family safe or adult content."]
        NotFamilySafe,
        #[serde(rename = "CLINICAL_TRIAL_RECRUITMENT")]
        #[doc = "Clinical trial recruitment."]
        ClinicalTrialRecruitment,
        #[serde(rename = "MAXIMUM_NUMBER_OF_HTTP_CALLS_EXCEEDED")]
        #[doc = "Maximum number of HTTP calls exceeded."]
        MaximumNumberOfHttpCallsExceeded,
        #[serde(rename = "MAXIMUM_NUMBER_OF_COOKIES_EXCEEDED")]
        #[doc = "Maximum number of cookies exceeded."]
        MaximumNumberOfCookiesExceeded,
        #[serde(rename = "PERSONAL_LOANS")]
        #[doc = "Financial service ad does not adhere to specifications."]
        PersonalLoans,
        #[serde(rename = "UNSUPPORTED_FLASH_CONTENT")]
        #[doc = "Flash content was found in an unsupported context."]
        UnsupportedFlashContent,
        #[serde(rename = "MISUSE_BY_OMID_SCRIPT")]
        #[doc = "Misuse by an Open Measurement SDK script."]
        MisuseByOmidScript,
        #[serde(rename = "NON_WHITELISTED_OMID_VENDOR")]
        #[doc = "Use of an Open Measurement SDK vendor not on approved whitelist."]
        NonWhitelistedOmidVendor,
        #[serde(rename = "DESTINATION_EXPERIENCE")]
        #[doc = "Unacceptable landing page."]
        DestinationExperience,
        #[serde(rename = "UNSUPPORTED_LANGUAGE")]
        #[doc = "Unsupported language."]
        UnsupportedLanguage,
        #[serde(rename = "NON_SSL_COMPLIANT")]
        #[doc = "Non-SSL compliant."]
        NonSslCompliant,
        #[serde(rename = "TEMPORARY_PAUSE")]
        #[doc = "Temporary pausing of creative."]
        TemporaryPause,
        #[serde(rename = "BAIL_BONDS")]
        #[doc = "Promotes services related to bail bonds."]
        BailBonds,
        #[serde(rename = "EXPERIMENTAL_MEDICAL_TREATMENT")]
        #[doc = "Promotes speculative and/or experimental medical treatments."]
        ExperimentalMedicalTreatment,
    }
    impl ::std::default::Default for DisapprovalReasonEnum {
        fn default() -> Self {
            Self::LengthOfImageAnimation
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
    #[doc = "A set of filters that is applied to a request for data. Within a filter set, an AND operation is performed across the filters represented by each field. An OR operation is performed across the filters represented by the multiple values of a repeated field, e.g., \"format=VIDEO AND deal_id=12 AND (seller_network_id=34 OR seller_network_id=56)\"."]
    pub struct FilterSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "absoluteDateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An absolute date range, defined by a start date and an end date. Interpreted relative to Pacific time zone."]
        pub absolute_date_range: ::std::option::Option<::std::boxed::Box<AbsoluteDateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "breakdownDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of dimensions along which to break down the response; may be empty. If multiple dimensions are requested, the breakdown is along the Cartesian product of the requested dimensions."]
        pub breakdown_dimensions:
            ::std::option::Option<::std::vec::Vec<FilterSetBreakdownDimensionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the creative on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, i.e., one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern."]
        pub creative_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dealId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the deal on which to filter; optional. This field may be set only for a filter set that accesses account-level troubleshooting data, i.e., one whose name matches the `bidders/*/accounts/*/filterSets/*` pattern."]
        pub deal_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment on which to filter; optional."]
        pub environment: ::std::option::Option<FilterSetEnvironmentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creative format bidded on or allowed to bid on, can be empty."]
        pub format: ::std::option::Option<FilterSetFormatEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formats")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creative formats bidded on or allowed to bid on, can be empty. Although this field is a list, it can only be populated with a single item. A HTTP 400 bad request error will be returned in the response if you specify multiple items."]
        pub formats: ::std::option::Option<::std::vec::Vec<FilterSetFormatsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-defined name of the filter set. Filter set names must be unique globally and match one of the patterns: - `bidders/*/filterSets/*` (for accessing bidder-level troubleshooting data) - `bidders/*/accounts/*/filterSets/*` (for accessing account-level troubleshooting data) This field is required in create operations."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of platforms on which to filter; may be empty. The filters represented by multiple platforms are ORed together (i.e., if non-empty, results must match any one of the platforms)."]
        pub platforms: ::std::option::Option<::std::vec::Vec<FilterSetPlatformsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherIdentifiers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For Open Bidding partners only. The list of publisher identifiers on which to filter; may be empty. The filters represented by multiple publisher identifiers are ORed together."]
        pub publisher_identifiers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "realtimeTimeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An open-ended realtime time range, defined by the aggregation start timestamp."]
        pub realtime_time_range: ::std::option::Option<::std::boxed::Box<RealtimeTimeRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeDateRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A relative date range, defined by an offset from today and a duration. Interpreted relative to Pacific time zone."]
        pub relative_date_range: ::std::option::Option<::std::boxed::Box<RelativeDateRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerNetworkIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For Authorized Buyers only. The list of IDs of the seller (publisher) networks on which to filter; may be empty. The filters represented by multiple seller network IDs are ORed together (i.e., if non-empty, results must match any one of the publisher networks). See [seller-network-ids](https://developers.google.com/authorized-buyers/rtb/downloads/seller-network-ids) file for the set of existing seller network IDs."]
        pub seller_network_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSeriesGranularity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The granularity of time intervals if a time series breakdown is desired; optional."]
        pub time_series_granularity: ::std::option::Option<FilterSetTimeSeriesGranularityEnum>,
    }
    impl FilterSet {
        pub fn builder() -> FilterSetBuilder {
            FilterSetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum FilterSetBreakdownDimensionsEnum {
        #[serde(rename = "BREAKDOWN_DIMENSION_UNSPECIFIED")]
        #[doc = "A placeholder for an unspecified dimension; should not be used."]
        BreakdownDimensionUnspecified,
        #[serde(rename = "PUBLISHER_IDENTIFIER")]
        #[doc = "The response should be broken down by publisher identifier. This option is available only for Open Bidding buyers."]
        PublisherIdentifier,
    }
    impl ::std::default::Default for FilterSetBreakdownDimensionsEnum {
        fn default() -> Self {
            Self::BreakdownDimensionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The environment on which to filter; optional."]
    pub enum FilterSetEnvironmentEnum {
        #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined environment; indicates that no environment filter will be applied."]
        EnvironmentUnspecified,
        #[serde(rename = "WEB")]
        #[doc = "The ad impression appears on the web."]
        Web,
        #[serde(rename = "APP")]
        #[doc = "The ad impression appears in an app."]
        App,
    }
    impl ::std::default::Default for FilterSetEnvironmentEnum {
        fn default() -> Self {
            Self::EnvironmentUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Creative format bidded on or allowed to bid on, can be empty."]
    pub enum FilterSetFormatEnum {
        #[serde(rename = "FORMAT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined format; indicates that no format filter will be applied."]
        FormatUnspecified,
        #[serde(rename = "NATIVE_DISPLAY")]
        #[doc = "The ad impression is a native ad, and display (i.e., image) format."]
        NativeDisplay,
        #[serde(rename = "NATIVE_VIDEO")]
        #[doc = "The ad impression is a native ad, and video format."]
        NativeVideo,
        #[serde(rename = "NON_NATIVE_DISPLAY")]
        #[doc = "The ad impression is not a native ad, and display (i.e., image) format."]
        NonNativeDisplay,
        #[serde(rename = "NON_NATIVE_VIDEO")]
        #[doc = "The ad impression is not a native ad, and video format."]
        NonNativeVideo,
    }
    impl ::std::default::Default for FilterSetFormatEnum {
        fn default() -> Self {
            Self::FormatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum FilterSetFormatsEnum {
        #[serde(rename = "FORMAT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined format; indicates that no format filter will be applied."]
        FormatUnspecified,
        #[serde(rename = "NATIVE_DISPLAY")]
        #[doc = "The ad impression is a native ad, and display (i.e., image) format."]
        NativeDisplay,
        #[serde(rename = "NATIVE_VIDEO")]
        #[doc = "The ad impression is a native ad, and video format."]
        NativeVideo,
        #[serde(rename = "NON_NATIVE_DISPLAY")]
        #[doc = "The ad impression is not a native ad, and display (i.e., image) format."]
        NonNativeDisplay,
        #[serde(rename = "NON_NATIVE_VIDEO")]
        #[doc = "The ad impression is not a native ad, and video format."]
        NonNativeVideo,
    }
    impl ::std::default::Default for FilterSetFormatsEnum {
        fn default() -> Self {
            Self::FormatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum FilterSetPlatformsEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined platform; indicates that no platform filter will be applied."]
        PlatformUnspecified,
        #[serde(rename = "DESKTOP")]
        #[doc = "The ad impression appears on a desktop."]
        Desktop,
        #[serde(rename = "TABLET")]
        #[doc = "The ad impression appears on a tablet."]
        Tablet,
        #[serde(rename = "MOBILE")]
        #[doc = "The ad impression appears on a mobile device."]
        Mobile,
    }
    impl ::std::default::Default for FilterSetPlatformsEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The granularity of time intervals if a time series breakdown is desired; optional."]
    pub enum FilterSetTimeSeriesGranularityEnum {
        #[serde(rename = "TIME_SERIES_GRANULARITY_UNSPECIFIED")]
        #[doc = "A placeholder for an unspecified interval; no time series is applied. All rows in response will contain data for the entire requested time range."]
        TimeSeriesGranularityUnspecified,
        #[serde(rename = "HOURLY")]
        #[doc = "Indicates that data will be broken down by the hour."]
        Hourly,
        #[serde(rename = "DAILY")]
        #[doc = "Indicates that data will be broken down by the day."]
        Daily,
    }
    impl ::std::default::Default for FilterSetTimeSeriesGranularityEnum {
        fn default() -> Self {
            Self::TimeSeriesGranularityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of filtered bids with the specified dimension values that have the specified creative."]
    pub struct FilteredBidCreativeRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids with the specified creative."]
        pub bid_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the creative."]
        pub creative_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
    }
    impl FilteredBidCreativeRow {
        pub fn builder() -> FilteredBidCreativeRowBuilder {
            FilteredBidCreativeRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of filtered bids with the specified dimension values, among those filtered due to the requested filtering reason (i.e. creative status), that have the specified detail."]
    pub struct FilteredBidDetailRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids with the specified detail."]
        pub bid_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the detail, can be numeric or text. The associated value can be looked up in the dictionary file corresponding to the DetailType in the response message."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detailId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Note: this field will be deprecated, use \"detail\" field instead. When \"detail\" field represents an integer value, this field is populated as the same integer value \"detail\" field represents, otherwise this field will be 0. The ID of the detail. The associated value can be looked up in the dictionary file corresponding to the DetailType in the response message."]
        pub detail_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
    }
    impl FilteredBidDetailRow {
        pub fn builder() -> FilteredBidDetailRowBuilder {
            FilteredBidDetailRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a list of targeted and excluded mobile application IDs that publishers own. Mobile application IDs are from App Store and Google Play Store. Android App ID, for example, com.google.android.apps.maps, can be found in Google Play Store URL. iOS App ID (which is a number) can be found at the end of iTunes store URL. First party mobile applications is either included or excluded."]
    pub struct FirstPartyMobileApplicationTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedAppIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of application IDs to be excluded."]
        pub excluded_app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetedAppIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of application IDs to be included."]
        pub targeted_app_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl FirstPartyMobileApplicationTargeting {
        pub fn builder() -> FirstPartyMobileApplicationTargetingBuilder {
            FirstPartyMobileApplicationTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Frequency cap."]
    pub struct FrequencyCap {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of impressions that can be served to a user within the specified time period."]
        pub max_impressions: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numTimeUnits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time, in the units specified by time_unit_type. Defines the amount of time over which impressions per user are counted and capped."]
        pub num_time_units: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeUnitType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time unit. Along with num_time_units defines the amount of time over which impressions per user are counted and capped."]
        pub time_unit_type: ::std::option::Option<FrequencyCapTimeUnitTypeEnum>,
    }
    impl FrequencyCap {
        pub fn builder() -> FrequencyCapBuilder {
            FrequencyCapBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The time unit. Along with num_time_units defines the amount of time over which impressions per user are counted and capped."]
    pub enum FrequencyCapTimeUnitTypeEnum {
        #[serde(rename = "TIME_UNIT_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined time unit type. This just indicates the variable with this value hasn't been initialized."]
        TimeUnitTypeUnspecified,
        #[serde(rename = "MINUTE")]
        #[doc = "Minute"]
        Minute,
        #[serde(rename = "HOUR")]
        #[doc = "Hour"]
        Hour,
        #[serde(rename = "DAY")]
        #[doc = "Day"]
        Day,
        #[serde(rename = "WEEK")]
        #[doc = "Week"]
        Week,
        #[serde(rename = "MONTH")]
        #[doc = "Month"]
        Month,
        #[serde(rename = "LIFETIME")]
        #[doc = "Lifetime"]
        Lifetime,
    }
    impl ::std::default::Default for FrequencyCapTimeUnitTypeEnum {
        fn default() -> Self {
            Self::TimeUnitTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Terms for Programmatic Guaranteed Deals."]
    pub struct GuaranteedFixedPriceTerms {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedPrices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fixed price for the specified buyer."]
        pub fixed_prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guaranteedImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Guaranteed impressions as a percentage. This is the percentage of guaranteed looks that the buyer is guaranteeing to buy."]
        pub guaranteed_impressions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "guaranteedLooks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of guaranteed looks. Required for deal, optional for product."]
        pub guaranteed_looks: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumDailyLooks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Daily minimum looks for CPD deal types."]
        pub minimum_daily_looks: ::std::option::Option<::std::string::String>,
    }
    impl GuaranteedFixedPriceTerms {
        pub fn builder() -> GuaranteedFixedPriceTermsBuilder {
            GuaranteedFixedPriceTermsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "HTML content for a creative."]
    pub struct HtmlContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the HTML snippet in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTML snippet that displays the ad when inserted in the web page."]
        pub snippet: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the HTML snippet in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl HtmlContent {
        pub fn builder() -> HtmlContentBuilder {
            HtmlContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image resource. You may provide a larger image than was requested, so long as the aspect ratio is preserved."]
    pub struct Image {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image height in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the image."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Image width in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Image {
        pub fn builder() -> ImageBuilder {
            ImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The set of metrics that are measured in numbers of impressions, representing how many impressions with the specified dimension values were considered eligible at each stage of the bidding funnel."]
    pub struct ImpressionMetricsRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableImpressions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions available to the buyer on Ad Exchange. In some cases this value may be unavailable."]
        pub available_impressions: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidRequests")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions for which Ad Exchange sent the buyer a bid request."]
        pub bid_requests: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventoryMatches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions that match the buyer's inventory pretargeting."]
        pub inventory_matches: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responsesWithBids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions for which Ad Exchange received a response from the buyer that contained at least one applicable bid."]
        pub responses_with_bids: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "successfulResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of impressions for which the buyer successfully sent a response to Ad Exchange."]
        pub successful_responses: ::std::option::Option<::std::boxed::Box<MetricValue>>,
    }
    impl ImpressionMetricsRow {
        pub fn builder() -> ImpressionMetricsRowBuilder {
            ImpressionMetricsRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the size of an ad unit that can be targeted on an ad request. It only applies to Private Auction, AdX Preferred Deals and Auction Packages. This targeting does not apply to Programmatic Guaranteed and Preferred Deals in Ad Manager."]
    pub struct InventorySizeTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedInventorySizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of inventory sizes to be excluded."]
        pub excluded_inventory_sizes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdSize>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetedInventorySizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of inventory sizes to be included."]
        pub targeted_inventory_sizes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AdSize>>>,
    }
    impl InventorySizeTargeting {
        pub fn builder() -> InventorySizeTargetingBuilder {
            InventorySizeTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing the metrics that are measured in number of bids."]
    pub struct ListBidMetricsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidMetricsRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, each containing a set of bid metrics."]
        pub bid_metrics_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BidMetricsRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListBidMetricsRequest.pageToken field in the subsequent call to the bidMetrics.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListBidMetricsResponse {
        pub fn builder() -> ListBidMetricsResponseBuilder {
            ListBidMetricsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons that bid responses resulted in an error."]
    pub struct ListBidResponseErrorsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calloutStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of bid responses aggregated by callout status."]
        pub callout_status_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalloutStatusRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListBidResponseErrorsRequest.pageToken field in the subsequent call to the bidResponseErrors.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListBidResponseErrorsResponse {
        pub fn builder() -> ListBidResponseErrorsResponseBuilder {
            ListBidResponseErrorsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons that bid responses were considered to have no applicable bids."]
    pub struct ListBidResponsesWithoutBidsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidResponseWithoutBidsStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of bid responses without bids aggregated by status."]
        pub bid_response_without_bids_status_rows: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<BidResponseWithoutBidsStatusRow>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListBidResponsesWithoutBidsRequest.pageToken field in the subsequent call to the bidResponsesWithoutBids.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListBidResponsesWithoutBidsResponse {
        pub fn builder() -> ListBidResponsesWithoutBidsResponseBuilder {
            ListBidResponsesWithoutBidsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListClientUserInvitationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invitations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned list of client users."]
        pub invitations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientUserInvitation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListClientUserInvitationsRequest.pageToken field in the subsequent call to the clients.invitations.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListClientUserInvitationsResponse {
        pub fn builder() -> ListClientUserInvitationsResponseBuilder {
            ListClientUserInvitationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListClientUsersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListClientUsersRequest.pageToken field in the subsequent call to the clients.invitations.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned list of client users."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ClientUser>>>,
    }
    impl ListClientUsersResponse {
        pub fn builder() -> ListClientUsersResponseBuilder {
            ListClientUsersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListClientsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The returned list of clients."]
        pub clients: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Client>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListClientsRequest.pageToken field in the subsequent call to the accounts.clients.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListClientsResponse {
        pub fn builder() -> ListClientsResponseBuilder {
            ListClientsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all creatives associated with a given filtered bid reason."]
    pub struct ListCreativeStatusBreakdownByCreativeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filteredBidCreativeRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of bids with a given creative status aggregated by creative."]
        pub filtered_bid_creative_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilteredBidCreativeRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListCreativeStatusBreakdownByCreativeRequest.pageToken field in the subsequent call to the filteredBids.creatives.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCreativeStatusBreakdownByCreativeResponse {
        pub fn builder() -> ListCreativeStatusBreakdownByCreativeResponseBuilder {
            ListCreativeStatusBreakdownByCreativeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all details associated with a given filtered bid reason."]
    pub struct ListCreativeStatusBreakdownByDetailResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detailType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of detail that the detail IDs represent."]
        pub detail_type:
            ::std::option::Option<ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filteredBidDetailRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of bids with a given creative status aggregated by detail."]
        pub filtered_bid_detail_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilteredBidDetailRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListCreativeStatusBreakdownByDetailRequest.pageToken field in the subsequent call to the filteredBids.details.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCreativeStatusBreakdownByDetailResponse {
        pub fn builder() -> ListCreativeStatusBreakdownByDetailResponseBuilder {
            ListCreativeStatusBreakdownByDetailResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of detail that the detail IDs represent."]
    pub enum ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
        #[serde(rename = "DETAIL_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined status. This value will never be returned in responses."]
        DetailTypeUnspecified,
        #[serde(rename = "CREATIVE_ATTRIBUTE")]
        #[doc = "Indicates that the detail ID refers to a creative attribute; see [publisher-excludable-creative-attributes](https://developers.google.com/authorized-buyers/rtb/downloads/publisher-excludable-creative-attributes)."]
        CreativeAttribute,
        #[serde(rename = "VENDOR")]
        #[doc = "Indicates that the detail ID refers to a vendor; see [vendors](https://developers.google.com/authorized-buyers/rtb/downloads/vendors). This namespace is different from that of the `ATP_VENDOR` detail type."]
        Vendor,
        #[serde(rename = "SENSITIVE_CATEGORY")]
        #[doc = "Indicates that the detail ID refers to a sensitive category; see [ad-sensitive-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-sensitive-categories)."]
        SensitiveCategory,
        #[serde(rename = "PRODUCT_CATEGORY")]
        #[doc = "Indicates that the detail ID refers to a product category; see [ad-product-categories](https://developers.google.com/authorized-buyers/rtb/downloads/ad-product-categories)."]
        ProductCategory,
        #[serde(rename = "DISAPPROVAL_REASON")]
        #[doc = "Indicates that the detail ID refers to a disapproval reason; see DisapprovalReason enum in [snippet-status-report-proto](https://developers.google.com/authorized-buyers/rtb/downloads/snippet-status-report-proto)."]
        DisapprovalReason,
        #[serde(rename = "POLICY_TOPIC")]
        #[doc = "Indicates that the detail ID refers to a policy topic."]
        PolicyTopic,
        #[serde(rename = "ATP_VENDOR")]
        #[doc = "Indicates that the detail ID refers to an ad technology provider (ATP); see [providers] (https://storage.googleapis.com/adx-rtb-dictionaries/providers.csv). This namespace is different from the `VENDOR` detail type; see [ad technology providers](https://support.google.com/admanager/answer/9012903) for more information."]
        AtpVendor,
        #[serde(rename = "VENDOR_DOMAIN")]
        #[doc = "Indicates that the detail string refers the domain of an unknown vendor."]
        VendorDomain,
        #[serde(rename = "GVL_ID")]
        #[doc = "Indicates that the detail ID refers an IAB GVL ID which Google did not detect in the latest TCF Vendor List. See [Global Vendor List] (https://vendor-list.consensu.org/v2/vendor-list.json)"]
        GvlId,
    }
    impl ::std::default::Default for ListCreativeStatusBreakdownByDetailResponseDetailTypeEnum {
        fn default() -> Self {
            Self::DetailTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response for listing creatives."]
    pub struct ListCreativesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatives")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of creatives."]
        pub creatives: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Creative>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListCreativesRequest.page_token field in the subsequent call to `ListCreatives` method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListCreativesResponse {
        pub fn builder() -> ListCreativesResponseBuilder {
            ListCreativesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response for listing creative and deal associations"]
    pub struct ListDealAssociationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "associations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of associations."]
        pub associations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeDealAssociation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListDealAssociationsRequest.page_token field in the subsequent call to 'ListDealAssociation' method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListDealAssociationsResponse {
        pub fn builder() -> ListDealAssociationsResponseBuilder {
            ListDealAssociationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing filter sets."]
    pub struct ListFilterSetsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filterSets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter sets belonging to the buyer."]
        pub filter_sets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<FilterSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListFilterSetsRequest.pageToken field in the subsequent call to the accounts.filterSets.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListFilterSetsResponse {
        pub fn builder() -> ListFilterSetsResponseBuilder {
            ListFilterSetsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons that bid requests were filtered and not sent to the buyer."]
    pub struct ListFilteredBidRequestsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "calloutStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of filtered bid requests aggregated by callout status."]
        pub callout_status_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CalloutStatusRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListFilteredBidRequestsRequest.pageToken field in the subsequent call to the filteredBidRequests.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListFilteredBidRequestsResponse {
        pub fn builder() -> ListFilteredBidRequestsResponseBuilder {
            ListFilteredBidRequestsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons that bids were filtered from the auction."]
    pub struct ListFilteredBidsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of filtered bids aggregated by filtering reason (i.e. creative status)."]
        pub creative_status_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeStatusRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListFilteredBidsRequest.pageToken field in the subsequent call to the filteredBids.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListFilteredBidsResponse {
        pub fn builder() -> ListFilteredBidsResponseBuilder {
            ListFilteredBidsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing the metrics that are measured in number of impressions."]
    pub struct ListImpressionMetricsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionMetricsRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, each containing a set of impression metrics."]
        pub impression_metrics_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImpressionMetricsRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListImpressionMetricsRequest.pageToken field in the subsequent call to the impressionMetrics.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListImpressionMetricsResponse {
        pub fn builder() -> ListImpressionMetricsResponseBuilder {
            ListImpressionMetricsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons that bids lost in the auction."]
    pub struct ListLosingBidsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of losing bids aggregated by loss reason (i.e. creative status)."]
        pub creative_status_rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CreativeStatusRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListLosingBidsRequest.pageToken field in the subsequent call to the losingBids.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListLosingBidsResponse {
        pub fn builder() -> ListLosingBidsResponseBuilder {
            ListLosingBidsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing all reasons for which a buyer was not billed for a winning bid."]
    pub struct ListNonBillableWinningBidsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A token to retrieve the next page of results. Pass this value in the ListNonBillableWinningBidsRequest.pageToken field in the subsequent call to the nonBillableWinningBids.list method to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nonBillableWinningBidStatusRows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of rows, with counts of bids not billed aggregated by reason."]
        pub non_billable_winning_bid_status_rows: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<NonBillableWinningBidStatusRow>>,
        >,
    }
    impl ListNonBillableWinningBidsResponse {
        pub fn builder() -> ListNonBillableWinningBidsResponseBuilder {
            ListNonBillableWinningBidsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing products visible to the buyer."]
    pub struct ListProductsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List pagination support."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "products")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of matching products at their head revision number."]
        pub products: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
    }
    impl ListProductsResponse {
        pub fn builder() -> ListProductsResponseBuilder {
            ListProductsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for listing proposals."]
    pub struct ListProposalsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of proposals."]
        pub proposals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Proposal>>>,
    }
    impl ListProposalsResponse {
        pub fn builder() -> ListProposalsResponseBuilder {
            ListProposalsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for profiles visible to the buyer."]
    pub struct ListPublisherProfilesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List pagination support"]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherProfiles")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of matching publisher profiles."]
        pub publisher_profiles:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PublisherProfile>>>,
    }
    impl ListPublisherProfilesResponse {
        pub fn builder() -> ListPublisherProfilesResponseBuilder {
            ListPublisherProfilesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The Geo criteria the restriction applies to."]
    pub struct LocationContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geoCriteriaIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs representing the geo location for this context. Please refer to the [geo-table.csv](https://storage.googleapis.com/adx-rtb-dictionaries/geo-table.csv) file for different geo criteria IDs."]
        pub geo_criteria_ids: ::std::option::Option<::std::vec::Vec<::std::primitive::i64>>,
    }
    impl LocationContext {
        pub fn builder() -> LocationContextBuilder {
            LocationContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Targeting represents different criteria that can be used by advertisers to target ad inventory. For example, they can choose to target ad requests only if the user is in the US. Multiple types of targeting are always applied as a logical AND, unless noted otherwise."]
    pub struct MarketplaceTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geoTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Geo criteria IDs to be included/excluded."]
        pub geo_targeting: ::std::option::Option<::std::boxed::Box<CriteriaTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventorySizeTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inventory sizes to be included/excluded."]
        pub inventory_size_targeting:
            ::std::option::Option<::std::boxed::Box<InventorySizeTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placementTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Placement targeting information, e.g., URL, mobile applications."]
        pub placement_targeting: ::std::option::Option<::std::boxed::Box<PlacementTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "technologyTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Technology targeting information, e.g., operating system, device category."]
        pub technology_targeting: ::std::option::Option<::std::boxed::Box<TechnologyTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video targeting information."]
        pub video_targeting: ::std::option::Option<::std::boxed::Box<VideoTargeting>>,
    }
    impl MarketplaceTargeting {
        pub fn builder() -> MarketplaceTargetingBuilder {
            MarketplaceTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A metric value, with an expected value and a variance; represents a count that may be either exact or estimated (i.e. when sampled)."]
    pub struct MetricValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expected value of the metric."]
        pub value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variance (i.e. square of the standard deviation) of the metric value. If value is exact, variance is 0. Can be used to calculate margin of error as a percentage of value, using the following formula, where Z is the standard constant that depends on the desired size of the confidence interval (e.g. for 90% confidence interval, use Z = 1.645): marginOfError = 100 * Z * sqrt(variance) / value"]
        pub variance: ::std::option::Option<::std::string::String>,
    }
    impl MetricValue {
        pub fn builder() -> MetricValueBuilder {
            MetricValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Mobile application targeting settings."]
    pub struct MobileApplicationTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firstPartyTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Publisher owned apps to be targeted or excluded by the publisher to display the ads in."]
        pub first_party_targeting:
            ::std::option::Option<::std::boxed::Box<FirstPartyMobileApplicationTargeting>>,
    }
    impl MobileApplicationTargeting {
        pub fn builder() -> MobileApplicationTargetingBuilder {
            MobileApplicationTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an amount of money with its currency type."]
    pub struct Money {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currencyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The three-letter currency code defined in ISO 4217."]
        pub currency_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of nano (10^-9) units of the amount. The value must be between -999,999,999 and +999,999,999 inclusive. If `units` is positive, `nanos` must be positive or zero. If `units` is zero, `nanos` can be positive, zero, or negative. If `units` is negative, `nanos` must be negative or zero. For example $-1.75 is represented as `units`=-1 and `nanos`=-750,000,000."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "units")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The whole units of the amount. For example if `currencyCode` is `\"USD\"`, then 1 unit is one US dollar."]
        pub units: ::std::option::Option<::std::string::String>,
    }
    impl Money {
        pub fn builder() -> MoneyBuilder {
            MoneyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Native content for a creative."]
    pub struct NativeContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the advertiser or sponsor, to be displayed in the ad creative."]
        pub advertiser_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appIcon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app icon, for app download ads."]
        pub app_icon: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "body")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A long description of the ad."]
        pub body: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "callToAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A label for the button that the user is supposed to click."]
        pub call_to_action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickLinkUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL that the browser/SDK will load when the user clicks the ad."]
        pub click_link_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickTrackingUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to use for click tracking."]
        pub click_tracking_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headline")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short title for the ad."]
        pub headline: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A large image."]
        pub image: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A smaller image, for the advertiser's logo."]
        pub logo: ::std::option::Option<::std::boxed::Box<Image>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceDisplayText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The price of the promoted app including currency info."]
        pub price_display_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "starRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app rating in the app store. Must be in the range [0-5]."]
        pub star_rating: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to the app store to purchase/download the promoted app."]
        pub store_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to fetch a native video ad."]
        pub video_url: ::std::option::Option<::std::string::String>,
    }
    impl NativeContent {
        pub fn builder() -> NativeContentBuilder {
            NativeContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The number of winning bids with the specified dimension values for which the buyer was not billed, as described by the specified status."]
    pub struct NonBillableWinningBidStatusRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bidCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of bids with the specified status."]
        pub bid_count: ::std::option::Option<::std::boxed::Box<MetricValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowDimensions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The values of all dimensions associated with metric values in this row."]
        pub row_dimensions: ::std::option::Option<::std::boxed::Box<RowDimensions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status specifying why the winning bids were not billed."]
        pub status: ::std::option::Option<NonBillableWinningBidStatusRowStatusEnum>,
    }
    impl NonBillableWinningBidStatusRow {
        pub fn builder() -> NonBillableWinningBidStatusRowBuilder {
            NonBillableWinningBidStatusRowBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status specifying why the winning bids were not billed."]
    pub enum NonBillableWinningBidStatusRowStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined status. This value will never be returned in responses."]
        StatusUnspecified,
        #[serde(rename = "AD_NOT_RENDERED")]
        #[doc = "The buyer was not billed because the ad was not rendered by the publisher."]
        AdNotRendered,
        #[serde(rename = "INVALID_IMPRESSION")]
        #[doc = "The buyer was not billed because the impression won by the bid was determined to be invalid."]
        InvalidImpression,
        #[serde(rename = "FATAL_VAST_ERROR")]
        #[doc = "A video impression was served but a fatal error was reported from the client during playback."]
        FatalVastError,
        #[serde(rename = "LOST_IN_MEDIATION")]
        #[doc = "The buyer was not billed because the ad was outplaced in the mediation waterfall."]
        LostInMediation,
    }
    impl ::std::default::Default for NonBillableWinningBidStatusRowStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Terms for Private Auctions. Note that Private Auctions can be created only by the seller, but they can be returned in a get or list request."]
    pub struct NonGuaranteedAuctionTerms {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoOptimizePrivateAuction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if open auction buyers are allowed to compete with invited buyers in this private auction."]
        pub auto_optimize_private_auction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reservePricesPerBuyer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reserve price for the specified buyer."]
        pub reserve_prices_per_buyer:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
    }
    impl NonGuaranteedAuctionTerms {
        pub fn builder() -> NonGuaranteedAuctionTermsBuilder {
            NonGuaranteedAuctionTermsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Terms for Preferred Deals. Note that Preferred Deals cannot be created via the API at this time, but can be returned in a get or list request."]
    pub struct NonGuaranteedFixedPriceTerms {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fixedPrices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fixed price for the specified buyer."]
        pub fixed_prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PricePerBuyer>>>,
    }
    impl NonGuaranteedFixedPriceTerms {
        pub fn builder() -> NonGuaranteedFixedPriceTermsBuilder {
            NonGuaranteedFixedPriceTermsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A proposal may be associated to several notes."]
    pub struct Note {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The timestamp for when this note was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The role of the person (buyer/seller) creating the note."]
        pub creator_role: ::std::option::Option<NoteCreatorRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "note")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual note to attach. (max-length: 1024 unicode code units) Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub note: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noteId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique ID for the note."]
        pub note_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The revision number of the proposal when the note is created."]
        pub proposal_revision: ::std::option::Option<::std::string::String>,
    }
    impl Note {
        pub fn builder() -> NoteBuilder {
            NoteBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The role of the person (buyer/seller) creating the note."]
    pub enum NoteCreatorRoleEnum {
        #[serde(rename = "BUYER_SELLER_ROLE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[serde(rename = "BUYER")]
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[serde(rename = "SELLER")]
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ::std::default::Default for NoteCreatorRoleEnum {
        fn default() -> Self {
            Self::BuyerSellerRoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents targeting information for operating systems."]
    pub struct OperatingSystemTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatingSystemCriteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of operating systems to be included/excluded."]
        pub operating_system_criteria: ::std::option::Option<::std::boxed::Box<CriteriaTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatingSystemVersionCriteria")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of operating system versions to be included/excluded."]
        pub operating_system_version_criteria:
            ::std::option::Option<::std::boxed::Box<CriteriaTargeting>>,
    }
    impl OperatingSystemTargeting {
        pub fn builder() -> OperatingSystemTargetingBuilder {
            OperatingSystemTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message to pause serving for an already-finalized proposal."]
    pub struct PauseProposalRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason why the proposal is being paused. This human readable message will be displayed in the seller's UI. (Max length: 1000 unicode code units.)"]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl PauseProposalRequest {
        pub fn builder() -> PauseProposalRequestBuilder {
            PauseProposalRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents targeting about where the ads can appear, e.g., certain sites or mobile applications. Different placement targeting types will be logically OR'ed."]
    pub struct PlacementTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileApplicationTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Mobile application targeting information in a deal. This doesn't apply to Auction Packages."]
        pub mobile_application_targeting:
            ::std::option::Option<::std::boxed::Box<MobileApplicationTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "urlTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URLs to be included/excluded."]
        pub url_targeting: ::std::option::Option<::std::boxed::Box<UrlTargeting>>,
    }
    impl PlacementTargeting {
        pub fn builder() -> PlacementTargetingBuilder {
            PlacementTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. The type of platform the restriction applies to."]
    pub struct PlatformContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platforms this restriction applies to."]
        pub platforms: ::std::option::Option<::std::vec::Vec<PlatformContextPlatformsEnum>>,
    }
    impl PlatformContext {
        pub fn builder() -> PlatformContextBuilder {
            PlatformContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PlatformContextPlatformsEnum {
        #[serde(rename = "DESKTOP")]
        #[doc = "Desktop platform."]
        Desktop,
        #[serde(rename = "ANDROID")]
        #[doc = "Android platform."]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "iOS platform."]
        Ios,
    }
    impl ::std::default::Default for PlatformContextPlatformsEnum {
        fn default() -> Self {
            Self::Desktop
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a price and a pricing type for a product / deal."]
    pub struct Price {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual price with currency specified."]
        pub amount: ::std::option::Option<::std::boxed::Box<Money>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pricingType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pricing type for the deal/product. (default: CPM)"]
        pub pricing_type: ::std::option::Option<PricePricingTypeEnum>,
    }
    impl Price {
        pub fn builder() -> PriceBuilder {
            PriceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The pricing type for the deal/product. (default: CPM)"]
    pub enum PricePricingTypeEnum {
        #[serde(rename = "PRICING_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined pricing type. If the pricing type is unpsecified, `COST_PER_MILLE` will be used instead."]
        PricingTypeUnspecified,
        #[serde(rename = "COST_PER_MILLE")]
        #[doc = "Cost per thousand impressions."]
        CostPerMille,
        #[serde(rename = "COST_PER_DAY")]
        #[doc = "Cost per day"]
        CostPerDay,
    }
    impl ::std::default::Default for PricePricingTypeEnum {
        fn default() -> Self {
            Self::PricingTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Used to specify pricing rules for buyers/advertisers. Each PricePerBuyer in a product can become 0 or 1 deals. To check if there is a PricePerBuyer for a particular buyer or buyer/advertiser pair, we look for the most specific matching rule - we first look for a rule matching the buyer and advertiser, next a rule with the buyer but an empty advertiser list, and otherwise look for a matching rule where no buyer is set."]
    pub struct PricePerBuyer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "advertiserIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of advertisers for this price when associated with this buyer. If empty, all advertisers with this buyer pay this price."]
        pub advertiser_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The buyer who will pay this price. If unset, all buyers can pay this price (if the advertisers match, and there's no more specific rule matching the buyer)."]
        pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specified price."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl PricePerBuyer {
        pub fn builder() -> PricePerBuyerBuilder {
            PricePerBuyerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Buyers are allowed to store certain types of private data in a proposal/deal."]
    pub struct PrivateData {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A buyer or seller specified reference ID. This can be queried in the list operations (max-length: 1024 unicode code units)."]
        pub reference_id: ::std::option::Option<::std::string::String>,
    }
    impl PrivateData {
        pub fn builder() -> PrivateDataBuilder {
            PrivateDataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Note: this resource requires whitelisting for access. Please contact your account manager for access to Marketplace resources. A product is a segment of inventory that a seller wishes to sell. It is associated with certain terms and targeting information which helps the buyer know more about the inventory."]
    pub struct Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The proposed end time for the deal. The field will be truncated to the order of seconds during serving."]
        pub available_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availableStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inventory availability dates. The start time will be truncated to seconds during serving. Thus, a field specified as 3:23:34.456 (HH:mm:ss.SSS) will be truncated to 3:23:34 when serving."]
        pub available_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorContacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional contact information for the creator of this product."]
        pub creator_contacts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name for this product as set by the seller."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasCreatorSignedOff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the creator has already signed off on the product, then the buyer can finalize the deal by accepting the product as is. When copying to a proposal, if any of the terms are changed, then auto_finalize is automatically set to false."]
        pub has_creator_signed_off: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for the product."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The revision number of the product (auto-assigned by Marketplace)."]
        pub product_revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherProfileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ID which can be used by the Publisher Profile API to get more information about the seller that created this product."]
        pub publisher_profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seller")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the seller that created this product."]
        pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syndicationProduct")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The syndication product associated with the deal."]
        pub syndication_product: ::std::option::Option<ProductSyndicationProductEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetingCriterion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Targeting that is shared between the buyer and the seller. Each targeting criterion has a specified key and for each key there is a list of inclusion value or exclusion values."]
        pub targeting_criterion:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingCriteria>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "terms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The negotiable terms of the deal."]
        pub terms: ::std::option::Option<::std::boxed::Box<DealTerms>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time of last update."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webPropertyCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The web-property code for the seller. This needs to be copied as is when adding a new deal to a proposal."]
        pub web_property_code: ::std::option::Option<::std::string::String>,
    }
    impl Product {
        pub fn builder() -> ProductBuilder {
            ProductBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The syndication product associated with the deal."]
    pub enum ProductSyndicationProductEnum {
        #[serde(rename = "SYNDICATION_PRODUCT_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined syndication product."]
        SyndicationProductUnspecified,
        #[serde(rename = "CONTENT")]
        #[doc = "This typically represents a web page."]
        Content,
        #[serde(rename = "MOBILE")]
        #[doc = "This represents a mobile property."]
        Mobile,
        #[serde(rename = "VIDEO")]
        #[doc = "This represents video ad formats."]
        Video,
        #[serde(rename = "GAMES")]
        #[doc = "This represents ads shown within games."]
        Games,
    }
    impl ::std::default::Default for ProductSyndicationProductEnum {
        fn default() -> Self {
            Self::SyndicationProductUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Note: this resource requires whitelisting for access. Please contact your account manager for access to Marketplace resources. Represents a proposal in the Marketplace. A proposal is the unit of negotiation between a seller and a buyer and contains deals which are served. Note: you can not update, create, or otherwise modify Private Auction or Preferred Deals deals through the API. Fields are updatable unless noted otherwise."]
    pub struct Proposal {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billedBuyer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Reference to the buyer that will get billed for this proposal."]
        pub billed_buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the buyer on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub buyer: ::std::option::Option<::std::boxed::Box<Buyer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyerContacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contact information for the buyer."]
        pub buyer_contacts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyerPrivateData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Private data for buyer. (hidden from seller)."]
        pub buyer_private_data: ::std::option::Option<::std::boxed::Box<PrivateData>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deals")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The deals associated with this proposal. For Private Auction proposals (whose deals have NonGuaranteedAuctionTerms), there will only be one deal."]
        pub deals: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Deal>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name for the proposal."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRenegotiating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. True if the proposal is being renegotiated."]
        pub is_renegotiating: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSetupComplete")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. True, if the buyside inventory setup is complete for this proposal."]
        pub is_setup_complete: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdaterOrCommentorRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The role of the last user that either updated the proposal or left a comment."]
        pub last_updater_or_commentor_role:
            ::std::option::Option<ProposalLastUpdaterOrCommentorRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The notes associated with this proposal."]
        pub notes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Note>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originatorRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Indicates whether the buyer/seller created the proposal."]
        pub originator_role: ::std::option::Option<ProposalOriginatorRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privateAuctionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Private auction ID if this proposal is a private auction proposal."]
        pub private_auction_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique ID of the proposal."]
        pub proposal_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The revision number for the proposal. Each update to the proposal or the deal causes the proposal revision number to auto-increment. The buyer keeps track of the last revision number they know of and pass it in when making an update. If the head revision number on the server has since incremented, then an ABORTED error is returned during the update operation to let the buyer know that a subsequent update was made."]
        pub proposal_revision: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "proposalState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The current state of the proposal."]
        pub proposal_state: ::std::option::Option<ProposalProposalStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seller")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the seller on the proposal. Note: This field may be set only when creating the resource. Modifying this field while updating the resource will result in an error."]
        pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerContacts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Contact information for the seller."]
        pub seller_contacts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContactInformation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "termsAndConditions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The terms and conditions set by the publisher for this proposal."]
        pub terms_and_conditions: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the proposal was last revised."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl Proposal {
        pub fn builder() -> ProposalBuilder {
            ProposalBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The role of the last user that either updated the proposal or left a comment."]
    pub enum ProposalLastUpdaterOrCommentorRoleEnum {
        #[serde(rename = "BUYER_SELLER_ROLE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[serde(rename = "BUYER")]
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[serde(rename = "SELLER")]
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ::std::default::Default for ProposalLastUpdaterOrCommentorRoleEnum {
        fn default() -> Self {
            Self::BuyerSellerRoleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. Indicates whether the buyer/seller created the proposal."]
    pub enum ProposalOriginatorRoleEnum {
        #[serde(rename = "BUYER_SELLER_ROLE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined buyer/seller role."]
        BuyerSellerRoleUnspecified,
        #[serde(rename = "BUYER")]
        #[doc = "Specifies the role as buyer."]
        Buyer,
        #[serde(rename = "SELLER")]
        #[doc = "Specifies the role as seller."]
        Seller,
    }
    impl ::std::default::Default for ProposalOriginatorRoleEnum {
        fn default() -> Self {
            Self::BuyerSellerRoleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The current state of the proposal."]
    pub enum ProposalProposalStateEnum {
        #[serde(rename = "PROPOSAL_STATE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined proposal state."]
        ProposalStateUnspecified,
        #[serde(rename = "PROPOSED")]
        #[doc = "The proposal is under negotiation or renegotiation."]
        Proposed,
        #[serde(rename = "BUYER_ACCEPTED")]
        #[doc = "The proposal has been accepted by the buyer."]
        BuyerAccepted,
        #[serde(rename = "SELLER_ACCEPTED")]
        #[doc = "The proposal has been accepted by the seller."]
        SellerAccepted,
        #[serde(rename = "CANCELED")]
        #[doc = "The negotiations on the proposal were canceled and the proposal was never finalized."]
        Canceled,
        #[serde(rename = "FINALIZED")]
        #[doc = "The proposal is finalized. During renegotiation, the proposal may not be in this state."]
        Finalized,
    }
    impl ::std::default::Default for ProposalProposalStateEnum {
        fn default() -> Self {
            Self::ProposalStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Note: this resource requires whitelisting for access. Please contact your account manager for access to Marketplace resources. Represents a publisher profile (https://support.google.com/admanager/answer/6035806) in Marketplace. All fields are read only. All string fields are free-form text entered by the publisher unless noted otherwise."]
    pub struct PublisherProfile {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audienceDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description on the publisher's audience."]
        pub audience_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buyerPitchStatement")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Statement explaining what's unique about publisher's business, and why buyers should partner with the publisher."]
        pub buyer_pitch_statement: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "directDealsContact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contact information for direct reservation deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses."]
        pub direct_deals_contact: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the publisher profile."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domains")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of domains represented in this publisher profile. Empty if this is a parent profile. These are top private domains, meaning that these will not contain a string like \"photos.google.co.uk/123\", but will instead contain \"google.co.uk\"."]
        pub domains: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googlePlusUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to publisher's Google+ page."]
        pub google_plus_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isParent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if this profile is the parent profile of the seller. A parent profile represents all the inventory from the seller, as opposed to child profile that is created to brand a portion of inventory. One seller should have only one parent publisher profile, and can have multiple child profiles. Publisher profiles for the same seller will have same value of field google.ads.adexchange.buyer.v2beta1.PublisherProfile.seller. See https://support.google.com/admanager/answer/6035806 for details."]
        pub is_parent: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "logoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A Google public URL to the logo for this publisher profile. The logo is stored as a PNG, JPG, or GIF image."]
        pub logo_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaKitUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to additional marketing and sales materials."]
        pub media_kit_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileApps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of apps represented in this publisher profile. Empty if this is a parent profile."]
        pub mobile_apps: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<PublisherProfileMobileApplication>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overview")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Overview of the publisher."]
        pub overview: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "programmaticDealsContact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contact information for programmatic deals. This is free text entered by the publisher and may include information like names, phone numbers and email addresses."]
        pub programmatic_deals_contact: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherProfileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique ID for publisher profile."]
        pub publisher_profile_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rateCardInfoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to a publisher rate card."]
        pub rate_card_info_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "samplePageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to a sample content page."]
        pub sample_page_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seller")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seller of the publisher profile."]
        pub seller: ::std::option::Option<::std::boxed::Box<Seller>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topHeadlines")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Up to three key metrics and rankings. Max 100 characters each. For example \"#1 Mobile News Site for 20 Straight Months\"."]
        pub top_headlines: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl PublisherProfile {
        pub fn builder() -> PublisherProfileBuilder {
            PublisherProfileBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A mobile application that contains a external app ID, name, and app store."]
    pub struct PublisherProfileMobileApplication {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appStore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The app store the app belongs to."]
        pub app_store: ::std::option::Option<PublisherProfileMobileApplicationAppStoreEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "externalAppId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The external ID for the app from its app store."]
        pub external_app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the app."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl PublisherProfileMobileApplication {
        pub fn builder() -> PublisherProfileMobileApplicationBuilder {
            PublisherProfileMobileApplicationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The app store the app belongs to."]
    pub enum PublisherProfileMobileApplicationAppStoreEnum {
        #[serde(rename = "APP_STORE_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an unknown app store."]
        AppStoreTypeUnspecified,
        #[serde(rename = "APPLE_ITUNES")]
        #[doc = "Apple iTunes"]
        AppleItunes,
        #[serde(rename = "GOOGLE_PLAY")]
        #[doc = "Google Play"]
        GooglePlay,
        #[serde(rename = "ROKU")]
        #[doc = "Roku"]
        Roku,
        #[serde(rename = "AMAZON_FIRETV")]
        #[doc = "Amazon Fire TV"]
        AmazonFiretv,
        #[serde(rename = "PLAYSTATION")]
        #[doc = "Playstation"]
        Playstation,
        #[serde(rename = "XBOX")]
        #[doc = "Xbox"]
        Xbox,
        #[serde(rename = "SAMSUNG_TV")]
        #[doc = "Samsung TV"]
        SamsungTv,
        #[serde(rename = "AMAZON")]
        #[doc = "Amazon Appstore"]
        Amazon,
        #[serde(rename = "OPPO")]
        #[doc = "OPPO App Market"]
        Oppo,
        #[serde(rename = "SAMSUNG")]
        #[doc = "Samsung Galaxy Store"]
        Samsung,
        #[serde(rename = "VIVO")]
        #[doc = "VIVO App Store"]
        Vivo,
        #[serde(rename = "XIAOMI")]
        #[doc = "Xiaomi GetApps"]
        Xiaomi,
    }
    impl ::std::default::Default for PublisherProfileMobileApplicationAppStoreEnum {
        fn default() -> Self {
            Self::AppStoreTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An open-ended realtime time range specified by the start timestamp. For filter sets that specify a realtime time range RTB metrics continue to be aggregated throughout the lifetime of the filter set."]
    pub struct RealtimeTimeRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start timestamp of the real-time RTB metrics aggregation."]
        pub start_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl RealtimeTimeRange {
        pub fn builder() -> RealtimeTimeRangeBuilder {
            RealtimeTimeRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A relative date range, specified by an offset and a duration. The supported range of dates begins 30 days before today and ends today, i.e., the limits for these values are: offset_days >= 0 duration_days >= 1 offset_days + duration_days <= 30"]
    pub struct RelativeDateRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of days in the requested date range, e.g., for a range spanning today: 1. For a range spanning the last 7 days: 7."]
        pub duration_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offsetDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end date of the filter set, specified as the number of days before today, e.g., for a range where the last date is today: 0."]
        pub offset_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl RelativeDateRange {
        pub fn builder() -> RelativeDateRangeBuilder {
            RelativeDateRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for removing the association between a deal and a creative."]
    pub struct RemoveDealAssociationRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "association")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The association between a creative and a deal that should be removed."]
        pub association: ::std::option::Option<::std::boxed::Box<CreativeDealAssociation>>,
    }
    impl RemoveDealAssociationRequest {
        pub fn builder() -> RemoveDealAssociationRequestBuilder {
            RemoveDealAssociationRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message to resume (unpause) serving for an already-finalized proposal."]
    pub struct ResumeProposalRequest {}
    impl ResumeProposalRequest {
        pub fn builder() -> ResumeProposalRequestBuilder {
            ResumeProposalRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response may include multiple rows, breaking down along various dimensions. Encapsulates the values of all dimensions for a given row."]
    pub struct RowDimensions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publisherIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The publisher identifier for this row, if a breakdown by [BreakdownDimension.PUBLISHER_IDENTIFIER](https://developers.google.com/authorized-buyers/apis/reference/rest/v2beta1/bidders.accounts.filterSets#FilterSet.BreakdownDimension) was requested."]
        pub publisher_identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time interval that this row represents."]
        pub time_interval: ::std::option::Option<::std::boxed::Box<TimeInterval>>,
    }
    impl RowDimensions {
        pub fn builder() -> RowDimensionsBuilder {
            RowDimensionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. A security context."]
    pub struct SecurityContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The security types in this context."]
        pub securities: ::std::option::Option<::std::vec::Vec<SecurityContextSecuritiesEnum>>,
    }
    impl SecurityContext {
        pub fn builder() -> SecurityContextBuilder {
            SecurityContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum SecurityContextSecuritiesEnum {
        #[serde(rename = "INSECURE")]
        #[doc = "Matches impressions that require insecure compatibility."]
        Insecure,
        #[serde(rename = "SSL")]
        #[doc = "Matches impressions that require SSL compatibility."]
        Ssl,
    }
    impl ::std::default::Default for SecurityContextSecuritiesEnum {
        fn default() -> Self {
            Self::Insecure
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a seller of inventory. Each seller is identified by a unique Ad Manager account ID."]
    pub struct Seller {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID for the seller. The seller fills in this field. The seller account ID is then available to buyer in the product."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional sub-account ID for the seller."]
        pub sub_account_id: ::std::option::Option<::std::string::String>,
    }
    impl Seller {
        pub fn builder() -> SellerBuilder {
            SellerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The serving context for this restriction."]
    pub struct ServingContext {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "all")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches all contexts."]
        pub all: ::std::option::Option<ServingContextAllEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "appType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches impressions for a particular app type."]
        pub app_type: ::std::option::Option<::std::boxed::Box<AppContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auctionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches impressions for a particular auction type."]
        pub auction_type: ::std::option::Option<::std::boxed::Box<AuctionContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches impressions coming from users *or* publishers in a specific location."]
        pub location: ::std::option::Option<::std::boxed::Box<LocationContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches impressions coming from a particular platform."]
        pub platform: ::std::option::Option<::std::boxed::Box<PlatformContext>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Matches impressions for a particular security type."]
        pub security_type: ::std::option::Option<::std::boxed::Box<SecurityContext>>,
    }
    impl ServingContext {
        pub fn builder() -> ServingContextBuilder {
            ServingContextBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Matches all contexts."]
    pub enum ServingContextAllEnum {
        #[serde(rename = "SIMPLE_CONTEXT")]
        #[doc = "A simple context."]
        SimpleContext,
    }
    impl ::std::default::Default for ServingContextAllEnum {
        fn default() -> Self {
            Self::SimpleContext
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Output only. A representation of the status of an ad in a specific context. A context here relates to where something ultimately serves (for example, a user or publisher geo, a platform, an HTTPS vs HTTP request, or the type of auction)."]
    pub struct ServingRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contexts for the restriction."]
        pub contexts: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ServingContext>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disapproval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Disapproval bound to this restriction. Only present if status=DISAPPROVED. Can be used to filter the response of the creatives.list method."]
        pub disapproval: ::std::option::Option<::std::boxed::Box<Disapproval>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disapprovalReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any disapprovals bound to this restriction. Only present if status=DISAPPROVED. Can be used to filter the response of the creatives.list method. Deprecated; please use disapproval field instead."]
        pub disapproval_reasons:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Disapproval>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the creative in this context (for example, it has been explicitly disapproved or is pending review)."]
        pub status: ::std::option::Option<ServingRestrictionStatusEnum>,
    }
    impl ServingRestriction {
        pub fn builder() -> ServingRestrictionBuilder {
            ServingRestrictionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the creative in this context (for example, it has been explicitly disapproved or is pending review)."]
    pub enum ServingRestrictionStatusEnum {
        #[serde(rename = "STATUS_UNSPECIFIED")]
        #[doc = "The status is not known."]
        StatusUnspecified,
        #[serde(rename = "DISAPPROVAL")]
        #[doc = "The ad was disapproved in this context."]
        Disapproval,
        #[serde(rename = "PENDING_REVIEW")]
        #[doc = "The ad is pending review in this context."]
        PendingReview,
    }
    impl ::std::default::Default for ServingRestrictionStatusEnum {
        fn default() -> Self {
            Self::StatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message depicting the size of the creative. The units of width and height depend on the type of the targeting."]
    pub struct Size {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the creative."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the creative"]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Size {
        pub fn builder() -> SizeBuilder {
            SizeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for stopping notifications for changes to creative Status."]
    pub struct StopWatchingCreativeRequest {}
    impl StopWatchingCreativeRequest {
        pub fn builder() -> StopWatchingCreativeRequestBuilder {
            StopWatchingCreativeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Advertisers can target different attributes of an ad slot. For example, they can choose to show ads only if the user is in the U.S. Such targeting criteria can be specified as part of Shared Targeting."]
    pub struct TargetingCriteria {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exclusions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of values to exclude from targeting. Each value is AND'd together."]
        pub exclusions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inclusions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of value to include as part of the targeting. Each value is OR'd together."]
        pub inclusions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TargetingValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The key representing the shared targeting criterion. Targeting criteria defined by Google ad servers will begin with GOOG_. Third parties may define their own keys. A list of permissible keys along with the acceptable values will be provided as part of the external documentation."]
        pub key: ::std::option::Option<::std::string::String>,
    }
    impl TargetingCriteria {
        pub fn builder() -> TargetingCriteriaBuilder {
            TargetingCriteriaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A polymorphic targeting value used as part of Shared Targeting."]
    pub struct TargetingValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeSizeValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creative size value to include/exclude. Filled in when key = GOOG_CREATIVE_SIZE"]
        pub creative_size_value: ::std::option::Option<::std::boxed::Box<CreativeSize>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayPartTargetingValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The daypart targeting to include / exclude. Filled in when the key is GOOG_DAYPART_TARGETING. The definition of this targeting is derived from the structure used by Ad Manager."]
        pub day_part_targeting_value: ::std::option::Option<::std::boxed::Box<DayPartTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The long value to include/exclude."]
        pub long_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The string value to include/exclude."]
        pub string_value: ::std::option::Option<::std::string::String>,
    }
    impl TargetingValue {
        pub fn builder() -> TargetingValueBuilder {
            TargetingValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents targeting about various types of technology."]
    pub struct TechnologyTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceCapabilityTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of device capabilities to be included/excluded."]
        pub device_capability_targeting:
            ::std::option::Option<::std::boxed::Box<CriteriaTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceCategoryTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of device categories to be included/excluded."]
        pub device_category_targeting: ::std::option::Option<::std::boxed::Box<CriteriaTargeting>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operatingSystemTargeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Operating system related targeting information."]
        pub operating_system_targeting:
            ::std::option::Option<::std::boxed::Box<OperatingSystemTargeting>>,
    }
    impl TechnologyTargeting {
        pub fn builder() -> TechnologyTargetingBuilder {
            TechnologyTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An interval of time, with an absolute start and end."]
    pub struct TimeInterval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp marking the end of the range (exclusive) for which data is included."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp marking the start of the range (inclusive) for which data is included."]
        pub start_time: ::std::option::Option<::std::string::String>,
    }
    impl TimeInterval {
        pub fn builder() -> TimeIntervalBuilder {
            TimeIntervalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a time of day. The date and time zone are either not significant or are specified elsewhere. An API may choose to allow leap seconds. Related types are google.type.Date and `google.protobuf.Timestamp`."]
    pub struct TimeOfDay {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hours")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value \"24:00:00\" for scenarios like business closing time."]
        pub hours: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minutes of hour of day. Must be from 0 to 59."]
        pub minutes: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nanos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fractions of seconds in nanoseconds. Must be from 0 to 999,999,999."]
        pub nanos: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Seconds of minutes of the time. Must normally be from 0 to 59. An API may allow the value 60 if it allows leap-seconds."]
        pub seconds: ::std::option::Option<::std::primitive::i64>,
    }
    impl TimeOfDay {
        pub fn builder() -> TimeOfDayBuilder {
            TimeOfDayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a list of targeted and excluded URLs (e.g., google.com). For Private Auction and AdX Preferred Deals, URLs are either included or excluded. For Programmatic Guaranteed and Preferred Deals, this doesn't apply."]
    pub struct UrlTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of URLs to be excluded."]
        pub excluded_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetedUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of URLs to be included."]
        pub targeted_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl UrlTargeting {
        pub fn builder() -> UrlTargetingBuilder {
            UrlTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video content for a creative."]
    pub struct VideoContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL to fetch a video ad."]
        pub video_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoVastXml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contents of a VAST document for a video ad. This document should conform to the VAST 2.0 or 3.0 standard."]
        pub video_vast_xml: ::std::option::Option<::std::string::String>,
    }
    impl VideoContent {
        pub fn builder() -> VideoContentBuilder {
            VideoContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents targeting information about video."]
    pub struct VideoTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedPositionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of video positions to be excluded. Position types can either be included or excluded (XOR)."]
        pub excluded_position_types:
            ::std::option::Option<::std::vec::Vec<VideoTargetingExcludedPositionTypesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetedPositionTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of video positions to be included. When the included list is present, the excluded list must be empty. When the excluded list is present, the included list must be empty."]
        pub targeted_position_types:
            ::std::option::Option<::std::vec::Vec<VideoTargetingTargetedPositionTypesEnum>>,
    }
    impl VideoTargeting {
        pub fn builder() -> VideoTargetingBuilder {
            VideoTargetingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoTargetingExcludedPositionTypesEnum {
        #[serde(rename = "POSITION_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined video position."]
        PositionTypeUnspecified,
        #[serde(rename = "PREROLL")]
        #[doc = "Ad is played before the video."]
        Preroll,
        #[serde(rename = "MIDROLL")]
        #[doc = "Ad is played during the video."]
        Midroll,
        #[serde(rename = "POSTROLL")]
        #[doc = "Ad is played after the video."]
        Postroll,
    }
    impl ::std::default::Default for VideoTargetingExcludedPositionTypesEnum {
        fn default() -> Self {
            Self::PositionTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoTargetingTargetedPositionTypesEnum {
        #[serde(rename = "POSITION_TYPE_UNSPECIFIED")]
        #[doc = "A placeholder for an undefined video position."]
        PositionTypeUnspecified,
        #[serde(rename = "PREROLL")]
        #[doc = "Ad is played before the video."]
        Preroll,
        #[serde(rename = "MIDROLL")]
        #[doc = "Ad is played during the video."]
        Midroll,
        #[serde(rename = "POSTROLL")]
        #[doc = "Ad is played after the video."]
        Postroll,
    }
    impl ::std::default::Default for VideoTargetingTargetedPositionTypesEnum {
        fn default() -> Self {
            Self::PositionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request for watching changes to creative Status."]
    pub struct WatchCreativeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Pub/Sub topic to publish notifications to. This topic must already exist and must give permission to ad-exchange-buyside-reports@google.com to write to the topic. This should be the full resource name in \"projects/{project_id}/topics/{topic_id}\" format."]
        pub topic: ::std::option::Option<::std::string::String>,
    }
    impl WatchCreativeRequest {
        pub fn builder() -> WatchCreativeRequestBuilder {
            WatchCreativeRequestBuilder::default()
        }
    }
}
