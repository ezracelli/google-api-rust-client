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
    pub mod accounts {
        pub mod methods {
            pub mod claimwebsite {
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
                    #[serde(rename = "overwrite")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Only available to selected merchants. When set to `True`, this flag removes any existing claim on the requested website by another account and replaces it with a claim from this account."]
                    pub overwrite: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ query_parameters_defaults :: force () }", setter(into))]
                    #[serde(rename = "force")]
                    #[serde(default = "query_parameters_defaults :: force")]
                    #[doc = "Flag to delete sub-accounts with products. The default value is false."]
                    pub force: ::std::primitive::bool,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn force() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of accounts to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod accountstatuses {
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
                    #[serde(rename = "destinations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
                    pub destinations: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "destinations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
                    pub destinations: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of account statuses to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
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
    pub mod accounttax {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of tax settings to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod datafeeds {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod fetchnow {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of products to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod datafeedstatuses {
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
                    #[serde(rename = "country")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The country for which to get the datafeed status. If this parameter is provided then language must also be provided. Note that this parameter is required for feeds targeting multiple countries and languages, since a feed may have a different status for each target."]
                    pub country: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The language for which to get the datafeed status. If this parameter is provided then country must also be provided. Note that this parameter is required for feeds targeting multiple countries and languages, since a feed may have a different status for each target."]
                    pub language: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of products to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
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
    pub mod inventory {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod set {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod liasettings {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of LIA settings to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod requestgmbaccess {
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
                    #[serde(rename = "gmbEmail")]
                    #[doc = "The email of the Google My Business account."]
                    pub gmb_email: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod setinventoryverificationcontact {
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
                    #[serde(rename = "contactEmail")]
                    #[doc = "The email of the inventory verification contact."]
                    pub contact_email: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "contactName")]
                    #[doc = "The name of the inventory verification contact."]
                    pub contact_name: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "country")]
                    #[doc = "The country for which inventory verification is requested."]
                    pub country: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "language")]
                    #[doc = "The language for which inventory verification is requested."]
                    pub language: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod setposdataprovider {
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
                    #[serde(rename = "country")]
                    #[doc = "The country for which the POS data provider is selected."]
                    pub country: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "posDataProviderId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The ID of POS data provider."]
                    pub pos_data_provider_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "posExternalAccountId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The account ID by which this merchant is known to the POS data provider."]
                    pub pos_external_account_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod orderreports {
        pub mod methods {
            pub mod listdisbursements {
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
                    #[serde(rename = "disbursementEndDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The last date which disbursements occurred. In ISO 8601 format. Default: current date."]
                    pub disbursement_end_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "disbursementStartDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The first date which disbursements occurred. In ISO 8601 format."]
                    pub disbursement_start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of disbursements to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod listtransactions {
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of disbursements to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "transactionEndDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The last date in which transaction occurred. In ISO 8601 format. Default: current date."]
                    pub transaction_end_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "transactionStartDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The first date in which transaction occurred. In ISO 8601 format."]
                    pub transaction_start_date: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod orderreturns {
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
                    #[serde(rename = "createdEndDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains order returns created before this date (inclusively), in ISO 8601 format."]
                    pub created_end_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "createdStartDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains order returns created after this date (inclusively), in ISO 8601 format."]
                    pub created_start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of order returns to return in the response, used for paging. The default value is 25 returns per page, and the maximum allowed value is 250 returns per page."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the results in the specified order."]
                    pub order_by: ::std::option::Option<QueryParametersOrderByEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
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
                #[doc = "Return the results in the specified order."]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "RETURN_CREATION_TIME_DESC")]
                    #[doc = ""]
                    ReturnCreationTimeDesc,
                    #[serde(rename = "RETURN_CREATION_TIME_ASC")]
                    #[doc = ""]
                    ReturnCreationTimeAsc,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::ReturnCreationTimeDesc
                    }
                }
            }
        }
    }
    pub mod orders {
        pub mod methods {
            pub mod gettestordertemplate {
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
                    #[serde(rename = "country")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The country of the template to retrieve. Defaults to `US`."]
                    pub country: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "acknowledged")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains orders that match the acknowledgement status. When set to true, obtains orders that have been acknowledged. When false, obtains orders that have not been acknowledged. We recommend using this filter set to `false`, in conjunction with the `acknowledge` call, such that only un-acknowledged orders are returned. "]
                    pub acknowledged: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of orders to return in the response, used for paging. The default value is 25 orders per page, and the maximum allowed value is 250 orders per page."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "orderBy")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Order results by placement date in descending or ascending order. Acceptable values are: - placedDateAsc - placedDateDesc "]
                    pub order_by: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "placedDateEnd")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains orders placed before this date (exclusively), in ISO 8601 format."]
                    pub placed_date_end: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "placedDateStart")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains orders placed after this date (inclusively), in ISO 8601 format."]
                    pub placed_date_start: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "statuses")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Obtains orders that match any of the specified statuses. Please note that `active` is a shortcut for `pendingShipment` and `partiallyShipped`, and `completed` is a shortcut for `shipped`, `partiallyDelivered`, `delivered`, `partiallyReturned`, `returned`, and `canceled`."]
                    pub statuses: ::std::option::Option<QueryParametersStatusesEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Obtains orders that match any of the specified statuses. Please note that `active` is a shortcut for `pendingShipment` and `partiallyShipped`, and `completed` is a shortcut for `shipped`, `partiallyDelivered`, `delivered`, `partiallyReturned`, `returned`, and `canceled`."]
                pub enum QueryParametersStatusesEnum {
                    #[serde(rename = "ACTIVE")]
                    #[doc = ""]
                    Active,
                    #[serde(rename = "COMPLETED")]
                    #[doc = ""]
                    Completed,
                    #[serde(rename = "CANCELED")]
                    #[doc = ""]
                    Canceled,
                    #[serde(rename = "IN_PROGRESS")]
                    #[doc = ""]
                    InProgress,
                    #[serde(rename = "PENDING_SHIPMENT")]
                    #[doc = ""]
                    PendingShipment,
                    #[serde(rename = "PARTIALLY_SHIPPED")]
                    #[doc = ""]
                    PartiallyShipped,
                    #[serde(rename = "SHIPPED")]
                    #[doc = ""]
                    Shipped,
                    #[serde(rename = "PARTIALLY_DELIVERED")]
                    #[doc = ""]
                    PartiallyDelivered,
                    #[serde(rename = "DELIVERED")]
                    #[doc = ""]
                    Delivered,
                    #[serde(rename = "PARTIALLY_RETURNED")]
                    #[doc = ""]
                    PartiallyReturned,
                    #[serde(rename = "RETURNED")]
                    #[doc = ""]
                    Returned,
                }
                impl ::std::default::Default for QueryParametersStatusesEnum {
                    fn default() -> Self {
                        Self::Active
                    }
                }
            }
        }
    }
    pub mod pos {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod inventory {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod sale {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod products {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "includeInvalidInsertedItems")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to include the invalid inserted items in the result of the list request. By default the invalid items are not shown (the default value is false)."]
                    pub include_invalid_inserted_items:
                        ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of products to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
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
    pub mod productstatuses {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "includeAttributes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to include full product data in the results of this request. The default value is false."]
                    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
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
                    #[serde(rename = "destinations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
                    pub destinations: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeAttributes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to include full product data in the result of this get request. The default value is false."]
                    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "destinations")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
                    pub destinations: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeAttributes")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to include full product data in the results of the list request. The default value is false."]
                    pub include_attributes: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "includeInvalidInsertedItems")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to include the invalid inserted items in the result of the list request. By default the invalid items are not shown (the default value is false)."]
                    pub include_invalid_inserted_items:
                        ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of product statuses to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
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
    pub mod shippingsettings {
        pub mod methods {
            pub mod custombatch {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of shipping settings to return in the response, used for paging."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod update {
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
                    #[serde(rename = "dryRun")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag to simulate a request like in a live environment. If set to true, dry-run mode checks the validity of the request and returns errors (if any)."]
                    pub dry_run: ::std::option::Option<::std::primitive::bool>,
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
    #[doc = "Account data. After the creation of a new account it may take a few minutes before it is fully operational. The methods delete, insert, and update require the admin role."]
    pub struct Account {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adultContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the merchant sells adult content."]
        pub adult_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adwordsLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of linked AdWords accounts that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected either in the AdWords interface or through the AdWords API. To delete an active link, or to cancel a link request, remove it from the list."]
        pub adwords_links:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountAdwordsLink>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "businessInformation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The business information of the account."]
        pub business_information:
            ::std::option::Option<::std::boxed::Box<AccountBusinessInformation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleMyBusinessLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GMB account which is linked or in the process of being linked with the Merchant Center account."]
        pub google_my_business_link:
            ::std::option::Option<::std::boxed::Box<AccountGoogleMyBusinessLink>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for update. Merchant Center account ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#account`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name for the account."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reviewsUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] This field is never returned and will be ignored if provided."]
        #[deprecated]
        pub reviews_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-specific, locally-unique, internal ID for the child account."]
        pub seller_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "users")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Users with access to the account. Every account (except for subaccounts) must have at least one admin user."]
        pub users: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountUser>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merchant's website."]
        pub website_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "youtubeChannelLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of linked YouTube channels that are active or pending approval. To create a new link request, add a new link with status `active` to the list. It will remain in a `pending` state until approved or rejected in the YT Creator Studio interface. To delete an active link, or to cancel a link request, remove it from the list."]
        pub youtube_channel_links:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountYouTubeChannelLink>>>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CLDR country code (e.g. \"US\"). This value cannot be set for a sub-account of an MCA. All MCA sub-accounts inherit the country of their parent MCA."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal code or ZIP (e.g. \"94043\")."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streetAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Street-level part of the address."]
        pub street_address: ::std::option::Option<::std::string::String>,
    }
    impl AccountAddress {
        pub fn builder() -> AccountAddressBuilder {
            AccountAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountAdwordsLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adwordsId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer ID of the AdWords account."]
        pub adwords_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the link between this Merchant Center account and the AdWords account. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in Google AdWords or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending. Acceptable values are: - \"`active`\" - \"`pending`\" "]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl AccountAdwordsLink {
        pub fn builder() -> AccountAdwordsLinkBuilder {
            AccountAdwordsLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountBusinessInformation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The address of the business."]
        pub address: ::std::option::Option<::std::boxed::Box<AccountAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The customer service information of the business."]
        pub customer_service: ::std::option::Option<::std::boxed::Box<AccountCustomerService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number of the business."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl AccountBusinessInformation {
        pub fn builder() -> AccountBusinessInformationBuilder {
            AccountBusinessInformationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountCustomerService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer service email."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer service phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer service URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl AccountCustomerService {
        pub fn builder() -> AccountCustomerServiceBuilder {
            AccountCustomerServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountGoogleMyBusinessLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gmbEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GMB email address of which a specific account within a GMB account. A sample account within a GMB account could be a business account with set of locations, managed under the GMB account."]
        pub gmb_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the link between this Merchant Center account and the GMB account. Acceptable values are: - \"`active`\" - \"`pending`\" "]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl AccountGoogleMyBusinessLink {
        pub fn builder() -> AccountGoogleMyBusinessLinkBuilder {
            AccountGoogleMyBusinessLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountIdentifier {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aggregatorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The aggregator ID, set for aggregators and subaccounts (in that case, it represents the aggregator of the subaccount)."]
        pub aggregator_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merchant account ID, set for individual accounts and subaccounts."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
    }
    impl AccountIdentifier {
        pub fn builder() -> AccountIdentifierBuilder {
            AccountIdentifierBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of an account, i.e., information about its products, which is computed offline and not returned immediately at insertion time."]
    pub struct AccountStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account for which the status is reported."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountLevelIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of account level issues."]
        pub account_level_issues: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountStatusAccountLevelIssue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataQualityIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED - never populated."]
        pub data_quality_issues: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountStatusDataQualityIssue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accountStatus`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "products")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of product-related data by channel, destination, and country. Data in this field may be delayed by up to 30 minutes."]
        pub products:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusProducts>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "websiteClaimed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the account's website is claimed or not."]
        pub website_claimed: ::std::option::Option<::std::primitive::bool>,
    }
    impl AccountStatus {
        pub fn builder() -> AccountStatusBuilder {
            AccountStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusAccountLevelIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country for which this issue is reported."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination the issue applies to. If this field is empty then the issue applies to all available destinations."]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional details about the issue."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of a web page to help resolving this issue."]
        pub documentation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Issue identifier."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Severity of the issue. Acceptable values are: - \"`critical`\" - \"`error`\" - \"`suggestion`\" "]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Short description of the issue."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl AccountStatusAccountLevelIssue {
        pub fn builder() -> AccountStatusAccountLevelIssueBuilder {
            AccountStatusAccountLevelIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusDataQualityIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub displayed_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exampleItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub example_items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusExampleItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastChecked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub last_checked: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub num_items: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " Acceptable values are: - \"`critical`\" - \"`error`\" - \"`suggestion`\" "]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submittedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub submitted_value: ::std::option::Option<::std::string::String>,
    }
    impl AccountStatusDataQualityIssue {
        pub fn builder() -> AccountStatusDataQualityIssueBuilder {
            AccountStatusDataQualityIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusExampleItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submittedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub submitted_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueOnLandingPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value_on_landing_page: ::std::option::Option<::std::string::String>,
    }
    impl AccountStatusExampleItem {
        pub fn builder() -> AccountStatusExampleItemBuilder {
            AccountStatusExampleItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusItemLevelIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attribute's name, if the issue is caused by a single attribute."]
        pub attribute_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code of the issue."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short issue description in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed issue description in English."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of a web page to help with resolving this issue."]
        pub documentation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items with this issue."]
        pub num_items: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the issue can be resolved by the merchant."]
        pub resolution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How this issue affects serving of the offer."]
        pub servability: ::std::option::Option<::std::string::String>,
    }
    impl AccountStatusItemLevelIssue {
        pub fn builder() -> AccountStatusItemLevelIssueBuilder {
            AccountStatusItemLevelIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusProducts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel the data applies to. Acceptable values are: - \"`local`\" - \"`online`\" "]
        pub channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country the data applies to."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination the data applies to."]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemLevelIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of item-level issues."]
        pub item_level_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatusItemLevelIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Aggregated product statistics."]
        pub statistics: ::std::option::Option<::std::boxed::Box<AccountStatusStatistics>>,
    }
    impl AccountStatusProducts {
        pub fn builder() -> AccountStatusProductsBuilder {
            AccountStatusProductsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountStatusStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "active")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of active offers."]
        pub active: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disapproved")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of disapproved offers."]
        pub disapproved: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expiring")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of expiring offers."]
        pub expiring: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of pending offers."]
        pub pending: ::std::option::Option<::std::string::String>,
    }
    impl AccountStatusStatistics {
        pub fn builder() -> AccountStatusStatisticsBuilder {
            AccountStatusStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The tax settings of a merchant account. All methods require the admin role."]
    pub struct AccountTax {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the account to which these account tax settings belong."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountTax\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax rules. Updating the tax rules will enable US taxes (not reversible). Defining no rules is equivalent to not charging tax at all."]
        pub rules: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountTaxTaxRule>>>,
    }
    impl AccountTax {
        pub fn builder() -> AccountTaxBuilder {
            AccountTaxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tax calculation rule to apply in a state or province (USA only)."]
    pub struct AccountTaxTaxRule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country code in which tax is applicable."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. State (or province) is which the tax is applicable, described by its location ID (also called criteria ID)."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratePercent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Explicit tax rate in percent, represented as a floating point number without the percentage character. Must not be negative."]
        pub rate_percent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingTaxed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, shipping charges are also taxed."]
        pub shipping_taxed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "useGlobalRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the tax rate is taken from a global tax table or specified explicitly."]
        pub use_global_rate: ::std::option::Option<::std::primitive::bool>,
    }
    impl AccountTaxTaxRule {
        pub fn builder() -> AccountTaxTaxRuleBuilder {
            AccountTaxTaxRuleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountUser {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "admin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user is an admin."]
        pub admin: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's email address."]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderManager")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user is an order manager."]
        pub order_manager: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentsAnalyst")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user can access payment statements."]
        pub payments_analyst: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentsManager")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user can manage payment settings."]
        pub payments_manager: ::std::option::Option<::std::primitive::bool>,
    }
    impl AccountUser {
        pub fn builder() -> AccountUserBuilder {
            AccountUserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountYouTubeChannelLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel ID."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the link between this Merchant Center account and the YouTube channel. Upon retrieval, it represents the actual status of the link and can be either `active` if it was approved in YT Creator Studio or `pending` if it's pending approval. Upon insertion, it represents the *intended* status of the link. Re-uploading a link with status `active` when it's still pending or with status `pending` when it's already active will have no effect: the status will remain unchanged. Re-uploading a link with deprecated status `inactive` is equivalent to not submitting the link at all and will delete the link if it was active or cancel the link request if it was pending."]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl AccountYouTubeChannelLink {
        pub fn builder() -> AccountYouTubeChannelLinkBuilder {
            AccountYouTubeChannelLinkBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsAuthInfoResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountIdentifiers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account identifiers corresponding to the authenticated user. - For an individual account: only the merchant ID is defined - For an aggregator: only the aggregator ID is defined - For a subaccount of an MCA: both the merchant ID and the aggregator ID are defined. "]
        pub account_identifiers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountIdentifier>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsAuthInfoResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccountsAuthInfoResponse {
        pub fn builder() -> AccountsAuthInfoResponseBuilder {
            AccountsAuthInfoResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsClaimWebsiteResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsClaimWebsiteResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccountsClaimWebsiteResponse {
        pub fn builder() -> AccountsClaimWebsiteResponseBuilder {
            AccountsClaimWebsiteResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountsCustomBatchRequestEntry>>,
        >,
    }
    impl AccountsCustomBatchRequest {
        pub fn builder() -> AccountsCustomBatchRequestBuilder {
            AccountsCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accounts request."]
    pub struct AccountsCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account to create or update. Only defined if the method is `insert` or `update`."]
        pub account: ::std::option::Option<::std::boxed::Box<Account>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the targeted account. Only defined if the method is not `insert`."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "force")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the account should be deleted if the account has offers. Only applicable if the method is `delete`."]
        pub force: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labelIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Label IDs for the 'updatelabels' request."]
        pub label_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the `link` request."]
        pub link_request:
            ::std::option::Option<::std::boxed::Box<AccountsCustomBatchRequestEntryLinkRequest>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`claimWebsite`\" - \"`delete`\" - \"`get`\" - \"`insert`\" - \"`link`\" - \"`update`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "overwrite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Only applicable if the method is `claimwebsite`. Indicates whether or not to take the claim from another account in case there is a conflict."]
        pub overwrite: ::std::option::Option<::std::primitive::bool>,
    }
    impl AccountsCustomBatchRequestEntry {
        pub fn builder() -> AccountsCustomBatchRequestEntryBuilder {
            AccountsCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsCustomBatchRequestEntryLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action to perform for this link. The `\"request\"` action is only available to select merchants. Acceptable values are: - \"`approve`\" - \"`remove`\" - \"`request`\" "]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the link between the two accounts. Acceptable values are: - \"`channelPartner`\" - \"`eCommercePlatform`\" "]
        pub link_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the linked account."]
        pub linked_account_id: ::std::option::Option<::std::string::String>,
    }
    impl AccountsCustomBatchRequestEntryLinkRequest {
        pub fn builder() -> AccountsCustomBatchRequestEntryLinkRequestBuilder {
            AccountsCustomBatchRequestEntryLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountsCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccountsCustomBatchResponse {
        pub fn builder() -> AccountsCustomBatchResponseBuilder {
            AccountsCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accounts response."]
    pub struct AccountsCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved, created, or updated account. Not defined if the method was `delete`, `claimwebsite` or `link`."]
        pub account: ::std::option::Option<::std::boxed::Box<Account>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accountsCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. This field is never set. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
        pub link_status: ::std::option::Option<::std::string::String>,
    }
    impl AccountsCustomBatchResponseEntry {
        pub fn builder() -> AccountsCustomBatchResponseEntryBuilder {
            AccountsCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsLinkRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Action to perform for this link. The `\"request\"` action is only available to select merchants. Acceptable values are: - \"`approve`\" - \"`remove`\" - \"`request`\" "]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the link between the two accounts. Acceptable values are: - \"`channelPartner`\" - \"`eCommercePlatform`\" "]
        pub link_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the linked account."]
        pub linked_account_id: ::std::option::Option<::std::string::String>,
    }
    impl AccountsLinkRequest {
        pub fn builder() -> AccountsLinkRequestBuilder {
            AccountsLinkRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsLinkResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsLinkResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccountsLinkResponse {
        pub fn builder() -> AccountsLinkResponseBuilder {
            AccountsLinkResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of accounts."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
    }
    impl AccountsListResponse {
        pub fn builder() -> AccountsListResponseBuilder {
            AccountsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountstatusesCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountstatusesCustomBatchRequestEntry>>,
        >,
    }
    impl AccountstatusesCustomBatchRequest {
        pub fn builder() -> AccountstatusesCustomBatchRequestBuilder {
            AccountstatusesCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accountstatuses request."]
    pub struct AccountstatusesCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the (sub-)account whose status to get."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
        pub destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
        pub method: ::std::option::Option<::std::string::String>,
    }
    impl AccountstatusesCustomBatchRequestEntry {
        pub fn builder() -> AccountstatusesCustomBatchRequestEntryBuilder {
            AccountstatusesCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountstatusesCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccountstatusesCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountstatusesCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccountstatusesCustomBatchResponse {
        pub fn builder() -> AccountstatusesCustomBatchResponseBuilder {
            AccountstatusesCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accountstatuses response."]
    pub struct AccountstatusesCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested account status. Defined if and only if the request was successful."]
        pub account_status: ::std::option::Option<::std::boxed::Box<AccountStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    }
    impl AccountstatusesCustomBatchResponseEntry {
        pub fn builder() -> AccountstatusesCustomBatchResponseEntryBuilder {
            AccountstatusesCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccountstatusesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accountstatusesListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of account statuses."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountStatus>>>,
    }
    impl AccountstatusesListResponse {
        pub fn builder() -> AccountstatusesListResponseBuilder {
            AccountstatusesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccounttaxCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccounttaxCustomBatchRequestEntry>>,
        >,
    }
    impl AccounttaxCustomBatchRequest {
        pub fn builder() -> AccounttaxCustomBatchRequestBuilder {
            AccounttaxCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accounttax request."]
    pub struct AccounttaxCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account for which to get/update account tax settings."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account tax settings to update. Only defined if the method is `update`."]
        pub account_tax: ::std::option::Option<::std::boxed::Box<AccountTax>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`update`\" "]
        pub method: ::std::option::Option<::std::string::String>,
    }
    impl AccounttaxCustomBatchRequestEntry {
        pub fn builder() -> AccounttaxCustomBatchRequestEntryBuilder {
            AccounttaxCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccounttaxCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<AccounttaxCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accounttaxCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccounttaxCustomBatchResponse {
        pub fn builder() -> AccounttaxCustomBatchResponseBuilder {
            AccounttaxCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch accounttax response."]
    pub struct AccounttaxCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved or updated account tax settings."]
        pub account_tax: ::std::option::Option<::std::boxed::Box<AccountTax>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#accounttaxCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AccounttaxCustomBatchResponseEntry {
        pub fn builder() -> AccounttaxCustomBatchResponseEntryBuilder {
            AccounttaxCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AccounttaxListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#accounttaxListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of account tax settings."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AccountTax>>>,
    }
    impl AccounttaxListResponse {
        pub fn builder() -> AccounttaxListResponseBuilder {
            AccounttaxListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Amount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Value before taxes."]
        pub pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Tax value."]
        pub tax: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl Amount {
        pub fn builder() -> AmountBuilder {
            AmountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BusinessDayConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "businessDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Regular business days. May not be empty."]
        pub business_days: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl BusinessDayConfig {
        pub fn builder() -> BusinessDayConfigBuilder {
            BusinessDayConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CarrierRate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Carrier service, such as `\"UPS\"` or `\"Fedex\"`. The list of supported carriers can be retrieved via the `getSupportedCarriers` method. Required."]
        pub carrier_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Carrier service, such as `\"ground\"` or `\"2 days\"`. The list of supported services for a carrier can be retrieved via the `getSupportedCarriers` method. Required."]
        pub carrier_service: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flatAdjustment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additive shipping rate modifier. Can be negative. For example `{ \"value\": \"1\", \"currency\" : \"USD\" }` adds $1 to the rate, `{ \"value\": \"-3\", \"currency\" : \"USD\" }` removes $3 from the rate. Optional."]
        pub flat_adjustment: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the carrier rate. Must be unique per rate group. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originPostalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipping origin for this carrier rate. Required."]
        pub origin_postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "percentageAdjustment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Multiplicative shipping rate modifier as a number in decimal notation. Can be negative. For example `\"5.4\"` increases the rate by 5.4%, `\"-3\"` decreases the rate by 3%. Optional."]
        pub percentage_adjustment: ::std::option::Option<::std::string::String>,
    }
    impl CarrierRate {
        pub fn builder() -> CarrierRateBuilder {
            CarrierRateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CarriersCarrier {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR country code of the carrier (e.g., \"US\"). Always present."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the carrier (e.g., `\"UPS\"`). Always present."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of supported services (e.g., `\"ground\"`) for that carrier. Contains at least one service."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl CarriersCarrier {
        pub fn builder() -> CarriersCarrierBuilder {
            CarriersCarrierBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the attribute. Underscores will be replaced by spaces upon insertion."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the attribute. Acceptable values are: - \"`boolean`\" - \"`datetimerange`\" - \"`float`\" - \"`group`\" - \"`int`\" - \"`price`\" - \"`text`\" - \"`time`\" - \"`url`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form unit of the attribute. Unit can only be used for values of type int, float, or price."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the attribute."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CustomAttribute {
        pub fn builder() -> CustomAttributeBuilder {
            CustomAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sub-attributes."]
        pub attributes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the group. Underscores will be replaced by spaces upon insertion."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl CustomGroup {
        pub fn builder() -> CustomGroupBuilder {
            CustomGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CustomerReturnReason {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the reason."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code of the return reason. Acceptable values are: - \"`betterPriceFound`\" - \"`changedMind`\" - \"`damagedOrDefectiveItem`\" - \"`didNotMatchDescription`\" - \"`doesNotFit`\" - \"`expiredItem`\" - \"`incorrectItemReceived`\" - \"`noLongerNeeded`\" - \"`notSpecified`\" - \"`orderedWrongItem`\" - \"`other`\" - \"`qualityNotExpected`\" - \"`receivedTooLate`\" - \"`undeliverable`\" "]
        pub reason_code: ::std::option::Option<::std::string::String>,
    }
    impl CustomerReturnReason {
        pub fn builder() -> CustomerReturnReasonBuilder {
            CustomerReturnReasonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CutoffTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hour of the cutoff time until which an order has to be placed to be processed in the same day. Required."]
        pub hour: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minute of the cutoff time until which an order has to be placed to be processed in the same day. Required."]
        pub minute: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timezone identifier for the cutoff time. A list of identifiers can be found in the AdWords API documentation. E.g. \"Europe/Zurich\". Required."]
        pub timezone: ::std::option::Option<::std::string::String>,
    }
    impl CutoffTime {
        pub fn builder() -> CutoffTimeBuilder {
            CutoffTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Datafeed configuration data."]
    pub struct Datafeed {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO 639-1 language in which the attributes are defined in the data feed."]
        pub attribute_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] Please use targets[].language instead. The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targetCountry`."]
        #[deprecated]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of data feed. For product inventory feeds, only feeds for local stores, not online stores, are supported. Acceptable values are: - \"`local products`\" - \"`product inventory`\" - \"`products`\" "]
        pub content_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fetchSchedule")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fetch schedule for the feed file."]
        pub fetch_schedule: ::std::option::Option<::std::boxed::Box<DatafeedFetchSchedule>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The filename of the feed. All feeds must have a unique file name."]
        pub file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Format of the feed file."]
        pub format: ::std::option::Option<::std::boxed::Box<DatafeedFormat>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for update. The ID of the data feed."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intendedDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] Please use targets[].includedDestinations instead. The list of intended destinations (corresponds to checked check boxes in Merchant Center)."]
        #[deprecated]
        pub intended_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#datafeed`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for insert. A descriptive name of the data feed."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[DEPRECATED] Please use targets[].country instead. The country where the items in the feed will be included in the search index, represented as a CLDR territory code."]
        #[deprecated]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The targets this feed should apply to (country, language, destinations)."]
        pub targets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedTarget>>>,
    }
    impl Datafeed {
        pub fn builder() -> DatafeedBuilder {
            DatafeedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The required fields vary based on the frequency of fetching. For a monthly fetch schedule, day_of_month and hour are required. For a weekly fetch schedule, weekday and hour are required. For a daily fetch schedule, only hour is required."]
    pub struct DatafeedFetchSchedule {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dayOfMonth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The day of the month the feed file should be fetched (1-31)."]
        pub day_of_month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fetchUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL where the feed file can be fetched. Google Merchant Center will support automatic scheduled uploads using the HTTP, HTTPS, FTP, or SFTP protocols, so the value will need to be a valid link using one of those four protocols."]
        pub fetch_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The hour of the day the feed file should be fetched (0-23)."]
        pub hour: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minuteOfHour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minute of the hour the feed file should be fetched (0-59). Read-only."]
        pub minute_of_hour: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional password for fetch_url."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paused")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the scheduled fetch is paused or not."]
        pub paused: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time zone used for schedule. UTC by default. E.g., \"America/Los_Angeles\"."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An optional user name for fetch_url."]
        pub username: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weekday")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The day of the week the feed file should be fetched. Acceptable values are: - \"`monday`\" - \"`tuesday`\" - \"`wednesday`\" - \"`thursday`\" - \"`friday`\" - \"`saturday`\" - \"`sunday`\" "]
        pub weekday: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedFetchSchedule {
        pub fn builder() -> DatafeedFetchScheduleBuilder {
            DatafeedFetchScheduleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedFormat {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnDelimiter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delimiter for the separation of values in a delimiter-separated values feed. If not specified, the delimiter will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - \"`pipe`\" - \"`tab`\" - \"`tilde`\" "]
        pub column_delimiter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Character encoding scheme of the data feed. If not specified, the encoding will be auto-detected. Acceptable values are: - \"`latin-1`\" - \"`utf-16be`\" - \"`utf-16le`\" - \"`utf-8`\" - \"`windows-1252`\" "]
        pub file_encoding: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quotingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies how double quotes are interpreted. If not specified, the mode will be auto-detected. Ignored for non-DSV data feeds. Acceptable values are: - \"`normal character`\" - \"`value quoting`\" "]
        pub quoting_mode: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedFormat {
        pub fn builder() -> DatafeedFormatBuilder {
            DatafeedFormatBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a datafeed, i.e., the result of the last retrieval of the datafeed computed asynchronously when the feed processing is finished."]
    pub struct DatafeedStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country for which the status is reported, represented as a CLDR territory code."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the feed for which the status is reported."]
        pub datafeed_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of errors occurring in the feed."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusError>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of items in the feed that were processed."]
        pub items_total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemsValid")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of items in the feed that were valid."]
        pub items_valid: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#datafeedStatus`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO 639-1 language for which the status is reported."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUploadDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last date at which the feed was uploaded."]
        pub last_upload_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processing status of the feed. Acceptable values are: - \"`\"`failure`\": The feed could not be processed or all items had errors.`\" - \"`in progress`\": The feed is being processed. - \"`none`\": The feed has not yet been processed. For example, a feed that has never been uploaded will have this processing status. - \"`success`\": The feed was processed successfully, though some items might have had errors. "]
        pub processing_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of errors occurring in the feed."]
        pub warnings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusError>>>,
    }
    impl DatafeedStatus {
        pub fn builder() -> DatafeedStatusBuilder {
            DatafeedStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error occurring in the feed, like \"invalid price\"."]
    pub struct DatafeedStatusError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The code of the error, e.g., \"validation/invalid_value\"."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of occurrences of the error in the feed."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "examples")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of example occurrences of the error, grouped by product."]
        pub examples:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatusExample>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error message, e.g., \"Invalid price\"."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedStatusError {
        pub fn builder() -> DatafeedStatusErrorBuilder {
            DatafeedStatusErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An example occurrence for a particular error."]
    pub struct DatafeedStatusExample {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the example item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line number in the data feed where the example is found."]
        pub line_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The problematic value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedStatusExample {
        pub fn builder() -> DatafeedStatusExampleBuilder {
            DatafeedStatusExampleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedTarget {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country where the items in the feed will be included in the search index, represented as a CLDR territory code."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of destinations to exclude for this target (corresponds to unchecked check boxes in Merchant Center)."]
        pub excluded_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includedDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of destinations to include for this target (corresponds to checked check boxes in Merchant Center). Default destinations are always included unless provided in `excludedDestinations`. List of supported destinations (if available to the account): - DisplayAds - Shopping - ShoppingActions - SurfacesAcrossGoogle "]
        pub included_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO 639-1 language of the items in the feed. Must be a valid language for `targets[].country`."]
        pub language: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedTarget {
        pub fn builder() -> DatafeedTargetBuilder {
            DatafeedTargetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedsCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<DatafeedsCustomBatchRequestEntry>>,
        >,
    }
    impl DatafeedsCustomBatchRequest {
        pub fn builder() -> DatafeedsCustomBatchRequestBuilder {
            DatafeedsCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch datafeeds request."]
    pub struct DatafeedsCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data feed to insert."]
        pub datafeed: ::std::option::Option<::std::boxed::Box<Datafeed>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data feed to get, delete or fetch."]
        pub datafeed_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`fetchNow`\" - \"`get`\" - \"`insert`\" - \"`update`\" "]
        pub method: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedsCustomBatchRequestEntry {
        pub fn builder() -> DatafeedsCustomBatchRequestEntryBuilder {
            DatafeedsCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedsCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<DatafeedsCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedsCustomBatchResponse {
        pub fn builder() -> DatafeedsCustomBatchResponseBuilder {
            DatafeedsCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch datafeeds response."]
    pub struct DatafeedsCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested data feed. Defined if and only if the request was successful."]
        pub datafeed: ::std::option::Option<::std::boxed::Box<Datafeed>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    }
    impl DatafeedsCustomBatchResponseEntry {
        pub fn builder() -> DatafeedsCustomBatchResponseEntryBuilder {
            DatafeedsCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedsFetchNowResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsFetchNowResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedsFetchNowResponse {
        pub fn builder() -> DatafeedsFetchNowResponseBuilder {
            DatafeedsFetchNowResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of datafeeds."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Datafeed>>>,
    }
    impl DatafeedsListResponse {
        pub fn builder() -> DatafeedsListResponseBuilder {
            DatafeedsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedstatusesCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<DatafeedstatusesCustomBatchRequestEntry>>,
        >,
    }
    impl DatafeedstatusesCustomBatchRequest {
        pub fn builder() -> DatafeedstatusesCustomBatchRequestBuilder {
            DatafeedstatusesCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch datafeedstatuses request."]
    pub struct DatafeedstatusesCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country for which to get the datafeed status. If this parameter is provided then language must also be provided. Note that for multi-target datafeeds this parameter is required."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeedId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the data feed to get."]
        pub datafeed_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language for which to get the datafeed status. If this parameter is provided then country must also be provided. Note that for multi-target datafeeds this parameter is required."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
        pub method: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedstatusesCustomBatchRequestEntry {
        pub fn builder() -> DatafeedstatusesCustomBatchRequestEntryBuilder {
            DatafeedstatusesCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedstatusesCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<DatafeedstatusesCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedstatusesCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl DatafeedstatusesCustomBatchResponse {
        pub fn builder() -> DatafeedstatusesCustomBatchResponseBuilder {
            DatafeedstatusesCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch datafeedstatuses response."]
    pub struct DatafeedstatusesCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "datafeedStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested data feed status. Defined if and only if the request was successful."]
        pub datafeed_status: ::std::option::Option<::std::boxed::Box<DatafeedStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
    }
    impl DatafeedstatusesCustomBatchResponseEntry {
        pub fn builder() -> DatafeedstatusesCustomBatchResponseEntryBuilder {
            DatafeedstatusesCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DatafeedstatusesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#datafeedstatusesListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of datafeed statuses."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<DatafeedStatus>>>,
    }
    impl DatafeedstatusesListResponse {
        pub fn builder() -> DatafeedstatusesListResponseBuilder {
            DatafeedstatusesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct DeliveryTime {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cutoffTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Business days cutoff time definition. If not configured the cutoff time will be defaulted to 8AM PST."]
        pub cutoff_time: ::std::option::Option<::std::boxed::Box<CutoffTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "handlingBusinessDayConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The business days during which orders can be handled. If not provided, Monday to Friday business days will be assumed."]
        pub handling_business_day_config:
            ::std::option::Option<::std::boxed::Box<BusinessDayConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holidayCutoffs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holiday cutoff definitions. If configured, they specify order cutoff times for holiday-specific shipping."]
        pub holiday_cutoffs:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HolidayCutoff>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxHandlingTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped. Must be greater than or equal to `minHandlingTimeInDays`."]
        pub max_handling_time_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTransitTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Must be greater than or equal to `minTransitTimeInDays`."]
        pub max_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minHandlingTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of business days spent before an order is shipped. 0 means same day shipped, 1 means next day shipped."]
        pub min_handling_time_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minTransitTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum number of business days that is spent in transit. 0 means same day delivery, 1 means next day delivery. Either `{min,max}TransitTimeInDays` or `transitTimeTable` must be set, but not both."]
        pub min_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitBusinessDayConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The business days during which orders can be in-transit. If not provided, Monday to Friday business days will be assumed."]
        pub transit_business_day_config:
            ::std::option::Option<::std::boxed::Box<BusinessDayConfig>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitTimeTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transit time table, number of business days spent in transit based on row and column dimensions. Either `{min,max}TransitTimeInDays` or `transitTimeTable` can be set, but not both."]
        pub transit_time_table: ::std::option::Option<::std::boxed::Box<TransitTable>>,
    }
    impl DeliveryTime {
        pub fn builder() -> DeliveryTimeBuilder {
            DeliveryTimeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An error returned by the API."]
    pub struct Error {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domain")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The domain of the error."]
        pub domain: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the error."]
        pub message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code."]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl Error {
        pub fn builder() -> ErrorBuilder {
            ErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of errors returned by a failed batch entry."]
    pub struct Errors {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP status of the first error in `errors`."]
        pub code: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors."]
        pub errors: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message of the first error in `errors`."]
        pub message: ::std::option::Option<::std::string::String>,
    }
    impl Errors {
        pub fn builder() -> ErrorsBuilder {
            ErrorsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GmbAccounts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Merchant Center account."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gmbAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of GMB accounts which are available to the merchant."]
        pub gmb_accounts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmbAccountsGmbAccount>>>,
    }
    impl GmbAccounts {
        pub fn builder() -> GmbAccountsBuilder {
            GmbAccountsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GmbAccountsGmbAccount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email which identifies the GMB account."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listingCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of listings under this account."]
        pub listing_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the GMB account."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the GMB account (User or Business)."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GmbAccountsGmbAccount {
        pub fn builder() -> GmbAccountsGmbAccountBuilder {
            GmbAccountsGmbAccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A non-empty list of row or column headers for a table. Exactly one of `prices`, `weights`, `numItems`, `postalCodeGroupNames`, or `location` must be set."]
    pub struct Headers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of location ID sets. Must be non-empty. Can only be set if all other fields are not set."]
        pub locations: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocationIdSet>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numberOfItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of inclusive number of items upper bounds. The last value can be `\"infinity\"`. For example `[\"10\", \"50\", \"infinity\"]` represents the headers \"<= 10 items\", \"<= 50 items\", and \"> 50 items\". Must be non-empty. Can only be set if all other fields are not set."]
        pub number_of_items: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeGroupNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of postal group names. The last value can be `\"all other locations\"`. Example: `[\"zone 1\", \"zone 2\", \"all other locations\"]`. The referred postal code groups must match the delivery country of the service. Must be non-empty. Can only be set if all other fields are not set."]
        pub postal_code_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of inclusive order price upper bounds. The last price's value can be `\"infinity\"`. For example `[{\"value\": \"10\", \"currency\": \"USD\"}, {\"value\": \"500\", \"currency\": \"USD\"}, {\"value\": \"infinity\", \"currency\": \"USD\"}]` represents the headers \"<= $10\", \"<= $500\", and \"> $500\". All prices within a service must have the same currency. Must be non-empty. Can only be set if all other fields are not set."]
        pub prices: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Price>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "weights")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of inclusive order weight upper bounds. The last weight's value can be `\"infinity\"`. For example `[{\"value\": \"10\", \"unit\": \"kg\"}, {\"value\": \"50\", \"unit\": \"kg\"}, {\"value\": \"infinity\", \"unit\": \"kg\"}]` represents the headers \"<= 10kg\", \"<= 50kg\", and \"> 50kg\". All weights within a service must have the same unit. Must be non-empty. Can only be set if all other fields are not set."]
        pub weights: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Weight>>>,
    }
    impl Headers {
        pub fn builder() -> HeadersBuilder {
            HeadersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HolidayCutoff {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deadlineDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of the order deadline, in ISO 8601 format. E.g. \"2016-11-29\" for 29th November 2016. Required."]
        pub deadline_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deadlineHour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hour of the day on the deadline date until which the order has to be placed to qualify for the delivery guarantee. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Required."]
        pub deadline_hour: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deadlineTimezone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timezone identifier for the deadline hour. A list of identifiers can be found in the AdWords API documentation. E.g. \"Europe/Zurich\". Required."]
        pub deadline_timezone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holidayId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the holiday. Required."]
        pub holiday_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibleFromDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the deadline will become visible to consumers in ISO 8601 format. E.g. \"2016-10-31\" for 31st October 2016. Required."]
        pub visible_from_date: ::std::option::Option<::std::string::String>,
    }
    impl HolidayCutoff {
        pub fn builder() -> HolidayCutoffBuilder {
            HolidayCutoffBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct HolidaysHoliday {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory code of the country in which the holiday is available. E.g. \"US\", \"DE\", \"GB\". A holiday cutoff can only be configured in a shipping settings service with matching delivery country. Always present."]
        pub country_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "date")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date of the holiday, in ISO 8601 format. E.g. \"2016-12-25\" for Christmas 2016. Always present."]
        pub date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryGuaranteeDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the order has to arrive at the customer's, in ISO 8601 format. E.g. \"2016-12-24\" for 24th December 2016. Always present."]
        pub delivery_guarantee_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryGuaranteeHour")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hour of the day in the delivery location's timezone on the guaranteed delivery date by which the order has to arrive at the customer's. Possible values are: 0 (midnight), 1, ..., 12 (noon), 13, ..., 23. Always present."]
        pub delivery_guarantee_hour: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier for the holiday to be used when configuring holiday cutoffs. Always present."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The holiday type. Always present. Acceptable values are: - \"`Christmas`\" - \"`Easter`\" - \"`Father's Day`\" - \"`Halloween`\" - \"`Independence Day (USA)`\" - \"`Mother's Day`\" - \"`Thanksgiving`\" - \"`Valentine's Day`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl HolidaysHoliday {
        pub fn builder() -> HolidaysHolidayBuilder {
            HolidaysHolidayBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Installment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount the buyer has to pay per month."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "months")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of installments the buyer has to pay."]
        pub months: ::std::option::Option<::std::string::String>,
    }
    impl Installment {
        pub fn builder() -> InstallmentBuilder {
            InstallmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Inventory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The availability of the product. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
        pub availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel0")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label0: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel4")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label4: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number and amount of installments to pay for an item. Brazil only."]
        pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instoreProductLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instore product location. Supported only for local products."]
        pub instore_product_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#inventory`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loyaltyPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
        pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Store pickup information. Only supported for local inventory. Not setting `pickup` means \"don't update\" while setting it to the empty value (`{}` in JSON) means \"delete\". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is \"not supported\"."]
        pub pickup: ::std::option::Option<::std::boxed::Box<InventoryPickup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The price of the product."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity of the product. Must be equal to or greater than zero. Supported only for local products."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
        pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePriceEffectiveDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided."]
        pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellOnGoogleQuantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
        pub sell_on_google_quantity: ::std::option::Option<::std::primitive::i64>,
    }
    impl Inventory {
        pub fn builder() -> InventoryBuilder {
            InventoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InventoryCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<InventoryCustomBatchRequestEntry>>,
        >,
    }
    impl InventoryCustomBatchRequest {
        pub fn builder() -> InventoryCustomBatchRequestBuilder {
            InventoryCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch inventory request."]
    pub struct InventoryCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price and availability of the product."]
        pub inventory: ::std::option::Option<::std::boxed::Box<Inventory>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product for which to update price and availability."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The code of the store for which to update price and availability. Use `online` to update price and availability of an online product."]
        pub store_code: ::std::option::Option<::std::string::String>,
    }
    impl InventoryCustomBatchRequestEntry {
        pub fn builder() -> InventoryCustomBatchRequestEntryBuilder {
            InventoryCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InventoryCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<InventoryCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#inventoryCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl InventoryCustomBatchResponse {
        pub fn builder() -> InventoryCustomBatchResponseBuilder {
            InventoryCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch inventory response."]
    pub struct InventoryCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#inventoryCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl InventoryCustomBatchResponseEntry {
        pub fn builder() -> InventoryCustomBatchResponseEntryBuilder {
            InventoryCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InventoryPickup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickupMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether store pickup is available for this offer and whether the pickup option should be shown as buy, reserve, or not supported. Only supported for local inventory. Unless the value is \"not supported\", must be submitted together with `pickupSla`. Acceptable values are: - \"`buy`\" - \"`not supported`\" - \"`reserve`\" - \"`ship to store`\" "]
        pub pickup_method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickupSla")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The expected date that an order will be ready for pickup, relative to when the order is placed. Only supported for local inventory. Must be submitted together with `pickupMethod`. Acceptable values are: - \"`five day`\" - \"`four day`\" - \"`multi day`\" - \"`multi week`\" - \"`next day`\" - \"`same day`\" - \"`seven day`\" - \"`six day`\" - \"`three day`\" - \"`two day`\" "]
        pub pickup_sla: ::std::option::Option<::std::string::String>,
    }
    impl InventoryPickup {
        pub fn builder() -> InventoryPickupBuilder {
            InventoryPickupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InventorySetRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The availability of the product. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
        pub availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel0")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label0: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel4")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign. Only supported for online products."]
        pub custom_label4: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number and amount of installments to pay for an item. Brazil only."]
        pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instoreProductLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instore product location. Supported only for local products."]
        pub instore_product_location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loyaltyPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
        pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Store pickup information. Only supported for local inventory. Not setting `pickup` means \"don't update\" while setting it to the empty value (`{}` in JSON) means \"delete\". Otherwise, `pickupMethod` and `pickupSla` must be set together, unless `pickupMethod` is \"not supported\"."]
        pub pickup: ::std::option::Option<::std::boxed::Box<InventoryPickup>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The price of the product."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity of the product. Must be equal to or greater than zero. Supported only for local products."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sale price of the product. Mandatory if `sale_price_effective_date` is defined."]
        pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePriceEffectiveDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A date range represented by a pair of ISO 8601 dates separated by a space, comma, or slash. Both dates might be specified as 'null' if undecided."]
        pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellOnGoogleQuantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
        pub sell_on_google_quantity: ::std::option::Option<::std::primitive::i64>,
    }
    impl InventorySetRequest {
        pub fn builder() -> InventorySetRequestBuilder {
            InventorySetRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InventorySetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#inventorySetResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl InventorySetResponse {
        pub fn builder() -> InventorySetResponseBuilder {
            InventorySetResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InvoiceSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalChargeSummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Summary of the total amounts of the additional charges."]
        pub additional_charge_summaries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<InvoiceSummaryAdditionalChargeSummary>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerBalance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub customer_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleBalance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub google_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantBalance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub merchant_balance: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Total price for the product."]
        pub product_total: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionSummaries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub promotion_summaries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
    }
    impl InvoiceSummary {
        pub fn builder() -> InvoiceSummaryBuilder {
            InvoiceSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct InvoiceSummaryAdditionalChargeSummary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Total additional charge for this type."]
        pub total_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Type of the additional charge. Acceptable values are: - \"`shipping`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl InvoiceSummaryAdditionalChargeSummary {
        pub fn builder() -> InvoiceSummaryAdditionalChargeSummaryBuilder {
            InvoiceSummaryAdditionalChargeSummaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiaAboutPageSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the verification process for the About page. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the About page."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl LiaAboutPageSettings {
        pub fn builder() -> LiaAboutPageSettingsBuilder {
            LiaAboutPageSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiaCountrySettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "about")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The settings for the About page."]
        pub about: ::std::option::Option<::std::boxed::Box<LiaAboutPageSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. CLDR country code (e.g. \"US\")."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hostedLocalStorefrontActive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the \"Merchant hosted local storefront\" feature."]
        pub hosted_local_storefront_active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "LIA inventory verification settings."]
        pub inventory: ::std::option::Option<::std::boxed::Box<LiaInventorySettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onDisplayToOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "LIA \"On Display To Order\" settings."]
        pub on_display_to_order:
            ::std::option::Option<::std::boxed::Box<LiaOnDisplayToOrderSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProvider")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The POS data provider linked with this country."]
        pub pos_data_provider: ::std::option::Option<::std::boxed::Box<LiaPosDataProvider>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storePickupActive")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the \"Store pickup\" feature."]
        pub store_pickup_active: ::std::option::Option<::std::primitive::bool>,
    }
    impl LiaCountrySettings {
        pub fn builder() -> LiaCountrySettingsBuilder {
            LiaCountrySettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiaInventorySettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventoryVerificationContactEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The email of the contact for the inventory verification process."]
        pub inventory_verification_contact_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventoryVerificationContactName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the contact for the inventory verification process."]
        pub inventory_verification_contact_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventoryVerificationContactStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the verification contact. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
        pub inventory_verification_contact_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the inventory verification process. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl LiaInventorySettings {
        pub fn builder() -> LiaInventorySettingsBuilder {
            LiaInventorySettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiaOnDisplayToOrderSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingCostPolicyUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipping cost and policy URL."]
        pub shipping_cost_policy_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the ?On display to order? feature. Acceptable values are: - \"`active`\" - \"`inactive`\" - \"`pending`\" "]
        pub status: ::std::option::Option<::std::string::String>,
    }
    impl LiaOnDisplayToOrderSettings {
        pub fn builder() -> LiaOnDisplayToOrderSettingsBuilder {
            LiaOnDisplayToOrderSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiaPosDataProvider {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProviderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the POS data provider."]
        pub pos_data_provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posExternalAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account ID by which this merchant is known to the POS data provider."]
        pub pos_external_account_id: ::std::option::Option<::std::string::String>,
    }
    impl LiaPosDataProvider {
        pub fn builder() -> LiaPosDataProviderBuilder {
            LiaPosDataProviderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Local Inventory ads (LIA) settings. All methods except listposdataproviders require the admin role."]
    pub struct LiaSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account to which these LIA settings belong. Ignored upon update, always present in get request responses."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countrySettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The LIA settings for each country."]
        pub country_settings:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiaCountrySettings>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#liaSettings`\""]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiaSettings {
        pub fn builder() -> LiaSettingsBuilder {
            LiaSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<LiasettingsCustomBatchRequestEntry>>,
        >,
    }
    impl LiasettingsCustomBatchRequest {
        pub fn builder() -> LiasettingsCustomBatchRequestBuilder {
            LiasettingsCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account for which to get/update account LIA settings."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inventory validation contact email. Required only for SetInventoryValidationContact."]
        pub contact_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contactName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inventory validation contact name. Required only for SetInventoryValidationContact."]
        pub contact_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country code. Required only for RequestInventoryVerification."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gmbEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The GMB account. Required only for RequestGmbAccess."]
        pub gmb_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liaSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account Lia settings to update. Only defined if the method is `update`."]
        pub lia_settings: ::std::option::Option<::std::boxed::Box<LiaSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`getAccessibleGmbAccounts`\" - \"`requestGmbAccess`\" - \"`requestInventoryVerification`\" - \"`setInventoryVerificationContact`\" - \"`update`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProviderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of POS data provider. Required only for SetPosProvider."]
        pub pos_data_provider_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posExternalAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account ID by which this merchant is known to the POS provider."]
        pub pos_external_account_id: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsCustomBatchRequestEntry {
        pub fn builder() -> LiasettingsCustomBatchRequestEntryBuilder {
            LiasettingsCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<LiasettingsCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsCustomBatchResponse {
        pub fn builder() -> LiasettingsCustomBatchResponseBuilder {
            LiasettingsCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry to which this entry responds."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if, and only if, the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gmbAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of accessible GMB accounts."]
        pub gmb_accounts: ::std::option::Option<::std::boxed::Box<GmbAccounts>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#liasettingsCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liaSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved or updated Lia settings."]
        pub lia_settings: ::std::option::Option<::std::boxed::Box<LiaSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of POS data providers."]
        pub pos_data_providers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosDataProviders>>>,
    }
    impl LiasettingsCustomBatchResponseEntry {
        pub fn builder() -> LiasettingsCustomBatchResponseEntryBuilder {
            LiasettingsCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsGetAccessibleGmbAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the Merchant Center account."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gmbAccounts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of GMB accounts which are available to the merchant."]
        pub gmb_accounts:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GmbAccountsGmbAccount>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsGetAccessibleGmbAccountsResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsGetAccessibleGmbAccountsResponse {
        pub fn builder() -> LiasettingsGetAccessibleGmbAccountsResponseBuilder {
            LiasettingsGetAccessibleGmbAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsListPosDataProvidersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsListPosDataProvidersResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of POS data providers for each eligible country"]
        pub pos_data_providers:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosDataProviders>>>,
    }
    impl LiasettingsListPosDataProvidersResponse {
        pub fn builder() -> LiasettingsListPosDataProvidersResponseBuilder {
            LiasettingsListPosDataProvidersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of LIA settings."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiaSettings>>>,
    }
    impl LiasettingsListResponse {
        pub fn builder() -> LiasettingsListResponseBuilder {
            LiasettingsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsRequestGmbAccessResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsRequestGmbAccessResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsRequestGmbAccessResponse {
        pub fn builder() -> LiasettingsRequestGmbAccessResponseBuilder {
            LiasettingsRequestGmbAccessResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsRequestInventoryVerificationResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsRequestInventoryVerificationResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsRequestInventoryVerificationResponse {
        pub fn builder() -> LiasettingsRequestInventoryVerificationResponseBuilder {
            LiasettingsRequestInventoryVerificationResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsSetInventoryVerificationContactResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsSetInventoryVerificationContactResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsSetInventoryVerificationContactResponse {
        pub fn builder() -> LiasettingsSetInventoryVerificationContactResponseBuilder {
            LiasettingsSetInventoryVerificationContactResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiasettingsSetPosDataProviderResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#liasettingsSetPosDataProviderResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl LiasettingsSetPosDataProviderResponse {
        pub fn builder() -> LiasettingsSetPosDataProviderResponseBuilder {
            LiasettingsSetPosDataProviderResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LocationIdSet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-empty list of location IDs. They must all be of the same location type (e.g., state)."]
        pub location_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl LocationIdSet {
        pub fn builder() -> LocationIdSetBuilder {
            LocationIdSetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LoyaltyPoints {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of loyalty points program. It is recommended to limit the name to 12 full-width characters or 24 Roman characters."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pointsValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retailer's loyalty points in absolute value."]
        pub points_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ratio of a point when converted to currency. Google assumes currency based on Merchant Center settings. If ratio is left out, it defaults to 1.0."]
        pub ratio: ::std::option::Option<::std::primitive::f64>,
    }
    impl LoyaltyPoints {
        pub fn builder() -> LoyaltyPointsBuilder {
            LoyaltyPointsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Order return. Production access (all methods) requires the order manager role. Sandbox access does not."]
    pub struct MerchantOrderReturn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of creation of the return, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merchant defined order ID."]
        pub merchant_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google order ID."]
        pub order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderReturnId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Order return ID generated by Google."]
        pub order_return_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Items of the return."]
        pub return_items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MerchantOrderReturnItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnShipments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipments of the return."]
        pub return_shipments:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ReturnShipment>>>,
    }
    impl MerchantOrderReturn {
        pub fn builder() -> MerchantOrderReturnBuilder {
            MerchantOrderReturnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MerchantOrderReturnItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customerReturnReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason that the customer chooses to return an item."]
        pub customer_return_reason: ::std::option::Option<::std::boxed::Box<CustomerReturnReason>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product level item ID. If the returned items are of the same product, they will have the same ID."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantReturnReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason that merchant chooses to accept a return item."]
        pub merchant_return_reason: ::std::option::Option<::std::boxed::Box<RefundReason>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product data from the time of the order placement."]
        pub product: ::std::option::Option<::std::boxed::Box<OrderLineItemProduct>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnShipmentIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "IDs of the return shipments that this return item belongs to."]
        pub return_shipment_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the item. Acceptable values are: - \"`canceled`\" - \"`new`\" - \"`received`\" - \"`refunded`\" - \"`rejected`\" "]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl MerchantOrderReturnItem {
        pub fn builder() -> MerchantOrderReturnItemBuilder {
            MerchantOrderReturnItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MinimumOrderValueTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCodeSetWithMovs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub store_code_set_with_movs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<MinimumOrderValueTableStoreCodeSetWithMov>>,
        >,
    }
    impl MinimumOrderValueTable {
        pub fn builder() -> MinimumOrderValueTableBuilder {
            MinimumOrderValueTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of store code sets sharing the same minimum order value. At least two sets are required and the last one must be empty, which signifies 'MOV for all other stores'. Each store code can only appear once across all the sets. All prices within a service must have the same currency."]
    pub struct MinimumOrderValueTableStoreCodeSetWithMov {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCodes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of unique store codes or empty for the catch all."]
        pub store_codes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum order value for the given stores."]
        pub value: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl MinimumOrderValueTableStoreCodeSetWithMov {
        pub fn builder() -> MinimumOrderValueTableStoreCodeSetWithMovBuilder {
            MinimumOrderValueTableStoreCodeSetWithMovBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Order. Production access (all methods) requires the order manager role. Sandbox access does not."]
    pub struct Order {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acknowledged")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the order was acknowledged."]
        pub acknowledged: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Acceptable values are: - \"`googleExpress`\" - \"`purchasesOnGoogle`\" "]
        pub channel_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the customer who placed the order."]
        pub customer: ::std::option::Option<::std::boxed::Box<OrderCustomer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delivery details for shipments of type `delivery`."]
        pub delivery_details: ::std::option::Option<::std::boxed::Box<OrderDeliveryDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The REST ID of the order. Globally unique."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#order`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line items that are ordered."]
        pub line_items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merchant-provided ID of the order."]
        pub merchant_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "netAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The net amount for the order. For example, if an order was originally for a grand total of $100 and a refund was issued for $20, the net amount will be $80."]
        pub net_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the payment method."]
        pub payment_method: ::std::option::Option<::std::boxed::Box<OrderPaymentMethod>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the payment. Acceptable values are: - \"`paymentCaptured`\" - \"`paymentRejected`\" - \"`paymentSecured`\" - \"`pendingAuthorization`\" "]
        pub payment_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickupDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pickup details for shipments of type `pickup`."]
        pub pickup_details: ::std::option::Option<::std::boxed::Box<OrderPickupDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "placedDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date when the order was placed, in ISO 8601 format."]
        pub placed_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the merchant provided promotions applied to the order. To determine which promotions apply to which products, check the `Promotions[].Benefits[].OfferIds` field against the `LineItems[].Product.OfferId` field for each promotion. If a promotion is applied to more than 1 `offerId`, divide the discount value by the number of affected offers to determine how much discount to apply to each `offerId`. Examples: 1. To calculate the line item level discount for a single specific item: For each promotion, subtract the `Promotions[].Benefits[].Discount.value` amount from the `LineItems[].Price.value`. 2. To calculate the line item level discount for multiple quantity of a specific item: For each promotion, divide the `Promotions[].Benefits[].Discount.value` by the quantity of products and substract it from `LineItems[].Product.Price.value` for each quantity item. Only 1 promotion can be applied to an offerId in a given order. To refund an item which had a promotion applied to it, make sure to refund the amount after first subtracting the promotion discount from the item price. More details about the program are here."]
        pub promotions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refunds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Refunds for the order."]
        pub refunds: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderRefund>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipments of the order."]
        pub shipments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderShipment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total cost of shipping for all items."]
        pub shipping_cost: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingCostTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tax for the total shipping cost."]
        pub shipping_cost_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Shipping details are provided with line items instead. Acceptable values are: - \"`economy`\" - \"`expedited`\" - \"`oneDay`\" - \"`sameDay`\" - \"`standard`\" - \"`twoDay`\" "]
        pub shipping_option: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the order. Acceptable values are: - \"`canceled`\" - \"`delivered`\" - \"`inProgress`\" - \"`partiallyDelivered`\" - \"`partiallyReturned`\" - \"`partiallyShipped`\" - \"`pendingShipment`\" - \"`returned`\" - \"`shipped`\" "]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxCollector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The party responsible for collecting and remitting taxes. Acceptable values are: - \"`marketplaceFacilitator`\" - \"`merchant`\" "]
        pub tax_collector: ::std::option::Option<::std::string::String>,
    }
    impl Order {
        pub fn builder() -> OrderBuilder {
            OrderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderAddress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "CLDR country code (e.g. \"US\")."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Strings representing the lines of the printed label for mailing the order, for example: John Smith 1600 Amphitheatre Parkway Mountain View, CA, 94043 United States "]
        pub full_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPostOfficeBox")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the address is a post office box."]
        pub is_post_office_box: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locality")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "City, town or commune. May also include dependent localities or sublocalities (e.g. neighborhoods or suburbs)."]
        pub locality: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Postal Code or ZIP (e.g. \"94043\")."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recipientName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the recipient."]
        pub recipient_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top-level administrative subdivision of the country. For example, a state like California (\"CA\") or a province like Quebec (\"QC\")."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streetAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Street-level part of the address."]
        pub street_address: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl OrderAddress {
        pub fn builder() -> OrderAddressBuilder {
            OrderAddressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderCancellation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actor that created the cancellation. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
        pub actor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the cancellation has been created, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity that was canceled."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Orders that are canceled with a noInventory reason will lead to the removal of the product from Buy on Google until you make an update to that product. This will not affect your Shopping ads. Acceptable values are: - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`customerCanceled`\" - \"`customerInitiatedCancel`\" - \"`customerSupportRequested`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceError`\" - \"`returnRefundAbuse`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrderCancellation {
        pub fn builder() -> OrderCancellationBuilder {
            OrderCancellationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderCustomer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitMarketingPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use marketingRightsInfo instead."]
        pub explicit_marketing_preference: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full name of the customer."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invoiceReceivingEmail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address for the merchant to send value-added tax or invoice documentation of the order. Only the last document sent is made available to the customer. For more information, see About automated VAT invoicing for Buy on Google."]
        pub invoice_receiving_email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingRightsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer's marketing preferences. Contains the marketing opt-in information that is current at the time that the merchant call. User preference selections can change from one order to the next so preferences must be checked with every order."]
        pub marketing_rights_info:
            ::std::option::Option<::std::boxed::Box<OrderCustomerMarketingRightsInfo>>,
    }
    impl OrderCustomer {
        pub fn builder() -> OrderCustomerBuilder {
            OrderCustomerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderCustomerMarketingRightsInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitMarketingPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last known customer selection regarding marketing preferences. In certain cases this selection might not be known, so this field would be empty. If a customer selected `granted` in their most recent order, they can be subscribed to marketing emails. Customers who have chosen `denied` must not be subscribed, or must be unsubscribed if already opted-in. Acceptable values are: - \"`denied`\" - \"`granted`\" "]
        pub explicit_marketing_preference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdatedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet."]
        pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingEmailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Email address that can be used for marketing purposes. The field may be empty even if `explicitMarketingPreference` is 'granted'. This happens when retrieving an old order from the customer who deleted their account."]
        pub marketing_email_address: ::std::option::Option<::std::string::String>,
    }
    impl OrderCustomerMarketingRightsInfo {
        pub fn builder() -> OrderCustomerMarketingRightsInfoBuilder {
            OrderCustomerMarketingRightsInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderDeliveryDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The delivery address"]
        pub address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number of the person receiving the delivery."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl OrderDeliveryDetails {
        pub fn builder() -> OrderDeliveryDetailsBuilder {
            OrderDeliveryDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLegacyPromotion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "benefits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub benefits:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotionBenefit>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectiveDates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time frame when the promotion is active and ready for validation review. Note that the promotion live time may be delayed for a few hours due to the validation review. Start date and end date are separated by a forward slash (/). The start date is specified by the format (YYYY-MM-DD), followed by the letter ?T?, the time of the day when the sale starts (in Greenwich Mean Time, GMT), followed by an expression of the time zone for the sale. The end date is in the same format."]
        pub effective_dates: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericRedemptionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text code that corresponds to the promotion when applied on the retailer?s website."]
        pub generic_redemption_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of the promotion."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full title of the promotion."]
        pub long_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productApplicability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the promotion is applicable to all products or only specific products. Acceptable values are: - \"`allProducts`\" - \"`specificProducts`\" "]
        pub product_applicability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redemptionChannel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that the promotion is valid online. Acceptable values are: - \"`online`\" "]
        pub redemption_channel: ::std::option::Option<::std::string::String>,
    }
    impl OrderLegacyPromotion {
        pub fn builder() -> OrderLegacyPromotionBuilder {
            OrderLegacyPromotionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLegacyPromotionBenefit {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "discount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The discount in the order price when the promotion is applied."]
        pub discount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offerIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The OfferId(s) that were purchased in this order and map to this specific benefit of the promotion."]
        pub offer_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Further describes the benefit of the promotion. Note that we will expand on this enumeration as we support new promotion sub-types. Acceptable values are: - \"`buyMGetMoneyOff`\" - \"`buyMGetNMoneyOff`\" - \"`buyMGetNPercentOff`\" - \"`buyMGetPercentOff`\" - \"`freeGift`\" - \"`freeGiftWithItemId`\" - \"`freeGiftWithValue`\" - \"`freeOvernightShipping`\" - \"`freeShipping`\" - \"`freeTwoDayShipping`\" - \"`moneyOff`\" - \"`percentageOff`\" - \"`rewardPoints`\" - \"`salePrice`\" "]
        pub sub_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxImpact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The impact on tax when the promotion is applied."]
        pub tax_impact: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes whether the promotion applies to products (e.g. 20% off) or to shipping (e.g. Free Shipping). Acceptable values are: - \"`product`\" - \"`shipping`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl OrderLegacyPromotionBenefit {
        pub fn builder() -> OrderLegacyPromotionBenefitBuilder {
            OrderLegacyPromotionBenefitBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Annotations that are attached to the line item."]
        pub annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancellations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cancellations of the line item."]
        pub cancellations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderCancellation>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total price for the line item. For example, if two items for $10 are purchased, the total price will be $20."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product data as seen by customer from the time of the order placement. Note that certain attributes values (e.g. title or gtin) might be reformatted and no longer match values submitted via product feed."]
        pub product: ::std::option::Option<::std::boxed::Box<OrderLineItemProduct>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityCanceled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items canceled."]
        pub quantity_canceled: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityDelivered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items delivered."]
        pub quantity_delivered: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityOrdered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items ordered."]
        pub quantity_ordered: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityPending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items pending."]
        pub quantity_pending: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityReadyForPickup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items ready for pickup."]
        pub quantity_ready_for_pickup: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityReturned")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items returned."]
        pub quantity_returned: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityShipped")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of items shipped."]
        pub quantity_shipped: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the return policy for the line item."]
        pub return_info: ::std::option::Option<::std::boxed::Box<OrderLineItemReturnInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns of the line item."]
        pub returns: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReturn>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details of the requested shipping for the line item."]
        pub shipping_details:
            ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total tax amount for the line item. For example, if two items are purchased, and each have a cost tax of $2, the total tax amount will be $4."]
        pub tax: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl OrderLineItem {
        pub fn builder() -> OrderLineItemBuilder {
            OrderLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemProduct {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Brand of the item."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item's channel (online or local). Acceptable values are: - \"`local`\" - \"`online`\" "]
        pub channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Condition or state of the item. Acceptable values are: - \"`new`\" - \"`refurbished`\" - \"`used`\" "]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fees")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associated fees at order creation time."]
        pub fees:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItemProductFee>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number (GTIN) of the item."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The REST ID of the product."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of an image of the item."]
        pub image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shared identifier for all variants of the same product."]
        pub item_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mpn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manufacturer Part Number (MPN) of the item."]
        pub mpn: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier of the item."]
        pub offer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shownImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL to the cached image shown to the user when order was placed."]
        pub shown_image: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory // code of the target country of the product."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the product."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variantAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variant attributes for the item. These are dimensions of the product, such as color, gender, material, pattern, and size. You can find a comprehensive list of variant attributes here."]
        pub variant_attributes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderLineItemProductVariantAttribute>>,
        >,
    }
    impl OrderLineItemProduct {
        pub fn builder() -> OrderLineItemProductBuilder {
            OrderLineItemProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemProductFee {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount of the fee."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the fee."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl OrderLineItemProductFee {
        pub fn builder() -> OrderLineItemProductFeeBuilder {
            OrderLineItemProductFeeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemProductVariantAttribute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension of the variant."]
        pub dimension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value for the dimension."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl OrderLineItemProductVariantAttribute {
        pub fn builder() -> OrderLineItemProductVariantAttributeBuilder {
            OrderLineItemProductVariantAttributeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemReturnInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "daysToReturn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. How many days later the item can be returned."]
        pub days_to_return: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isReturnable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Whether the item is returnable."]
        pub is_returnable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URL of the item return policy."]
        pub policy_url: ::std::option::Option<::std::string::String>,
    }
    impl OrderLineItemReturnInfo {
        pub fn builder() -> OrderLineItemReturnInfoBuilder {
            OrderLineItemReturnInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemShippingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliverByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The delivery by date, in ISO 8601 format."]
        pub deliver_by_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Details of the shipping method."]
        pub method: ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetailsMethod>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ship by date, in ISO 8601 format."]
        pub ship_by_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of shipment. Indicates whether `deliveryDetails` or `pickupDetails` is applicable for this shipment. Acceptable values are: - \"`delivery`\" - \"`pickup`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl OrderLineItemShippingDetails {
        pub fn builder() -> OrderLineItemShippingDetailsBuilder {
            OrderLineItemShippingDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderLineItemShippingDetailsMethod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier for the shipping. Optional. See `shipments[].carrier` for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxDaysInTransit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Maximum transit time."]
        pub max_days_in_transit: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "methodName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the shipping method."]
        pub method_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minDaysInTransit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Minimum transit time."]
        pub min_days_in_transit: ::std::option::Option<::std::primitive::i64>,
    }
    impl OrderLineItemShippingDetailsMethod {
        pub fn builder() -> OrderLineItemShippingDetailsMethodBuilder {
            OrderLineItemShippingDetailsMethodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderMerchantProvidedAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Key for additional merchant provided (as key-value pairs) annotation about the line item."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value for additional merchant provided (as key-value pairs) annotation about the line item."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl OrderMerchantProvidedAnnotation {
        pub fn builder() -> OrderMerchantProvidedAnnotationBuilder {
            OrderMerchantProvidedAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderPaymentMethod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "billingAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The billing address."]
        pub billing_address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationMonth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The card expiration month (January = 1, February = 2 etc.)."]
        pub expiration_month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationYear")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The card expiration year (4-digit, e.g. 2015)."]
        pub expiration_year: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastFourDigits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last four digits of the card number."]
        pub last_four_digits: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The billing phone number."]
        pub phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of instrument. Acceptable values are: - \"`AMEX`\" - \"`DISCOVER`\" - \"`JCB`\" - \"`MASTERCARD`\" - \"`UNIONPAY`\" - \"`VISA`\" - \"``\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl OrderPaymentMethod {
        pub fn builder() -> OrderPaymentMethodBuilder {
            OrderPaymentMethodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderPickupDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "address")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Address of the pickup location where the shipment should be sent. Note that `recipientName` in the address is the name of the business at the pickup location."]
        pub address: ::std::option::Option<::std::boxed::Box<OrderAddress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "collectors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collectors authorized to pick up shipment from the pickup location."]
        pub collectors:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderPickupDetailsCollector>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the pickup location."]
        pub location_id: ::std::option::Option<::std::string::String>,
    }
    impl OrderPickupDetails {
        pub fn builder() -> OrderPickupDetailsBuilder {
            OrderPickupDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderPickupDetailsCollector {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the person picking up the shipment."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Phone number of the person picking up the shipment."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl OrderPickupDetailsCollector {
        pub fn builder() -> OrderPickupDetailsCollectorBuilder {
            OrderPickupDetailsCollectorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderRefund {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actor that created the refund. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
        pub actor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item has been created, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrderRefund {
        pub fn builder() -> OrderRefundBuilder {
            OrderRefundBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Order disbursement. All methods require the payment analyst role."]
    pub struct OrderReportDisbursement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The disbursement amount."]
        pub disbursement_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementCreationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The disbursement date, in ISO 8601 format."]
        pub disbursement_creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date the disbursement was initiated, in ISO 8601 format."]
        pub disbursement_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the disbursement."]
        pub disbursement_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
    }
    impl OrderReportDisbursement {
        pub fn builder() -> OrderReportDisbursementBuilder {
            OrderReportDisbursementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderReportTransaction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The disbursement amount."]
        pub disbursement_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementCreationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date the disbursement was created, in ISO 8601 format."]
        pub disbursement_creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date the disbursement was initiated, in ISO 8601 format."]
        pub disbursement_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursementId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the disbursement."]
        pub disbursement_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merchant-provided ID of the order."]
        pub merchant_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the order."]
        pub order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total amount for the items."]
        pub product_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productAmountWithRemittedTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Total amount with remitted tax for the items."]
        pub product_amount_with_remitted_tax:
            ::std::option::Option<::std::boxed::Box<ProductAmount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactionDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of the transaction, in ISO 8601 format."]
        pub transaction_date: ::std::option::Option<::std::string::String>,
    }
    impl OrderReportTransaction {
        pub fn builder() -> OrderReportTransactionBuilder {
            OrderReportTransactionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderReturn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actor that created the refund. Acceptable values are: - \"`customer`\" - \"`googleBot`\" - \"`googleCustomerService`\" - \"`googlePayments`\" - \"`googleSabre`\" - \"`merchant`\" "]
        pub actor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item has been created, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quantity that is returned."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrderReturn {
        pub fn builder() -> OrderReturnBuilder {
            OrderReturnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderShipment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier handling the shipment. For supported carriers, Google includes the carrier name and tracking URL in emails to customers. For select supported carriers, Google also automatically updates the shipment status based on the provided shipment ID. *Note:* You can also use unsupported carriers, but emails to customers will not include the carrier name or tracking URL, and there will be no automatic order status updates. Supported carriers for US are: - \"`ups`\" (United Parcel Service) *automatic status updates* - \"`usps`\" (United States Postal Service) *automatic status updates* - \"`fedex`\" (FedEx) *automatic status updates * - \"`dhl`\" (DHL eCommerce) *automatic status updates* (US only) - \"`ontrac`\" (OnTrac) *automatic status updates * - \"`dhl express`\" (DHL Express) - \"`deliv`\" (Deliv) - \"`dynamex`\" (TForce) - \"`lasership`\" (LaserShip) - \"`mpx`\" (Military Parcel Xpress) - \"`uds`\" (United Delivery Service) - \"`efw`\" (Estes Forwarding Worldwide) - \"`jd logistics`\" (JD Logistics) - \"`yunexpress`\" (YunExpress) - \"`china post`\" (China Post) - \"`china ems`\" (China Post Express Mail Service) - \"`singapore post`\" (Singapore Post) - \"`pos malaysia`\" (Pos Malaysia) - \"`postnl`\" (PostNL) - \"`ptt`\" (PTT Turkish Post) - \"`eub`\" (ePacket) - \"`chukou1`\" (Chukou1 Logistics) - \"`bestex`\" (Best Express) - \"`canada post`\" (Canada Post) - \"`purolator`\" (Purolator) - \"`canpar`\" (Canpar) - \"`india post`\" (India Post) - \"`blue dart`\" (Blue Dart) - \"`delhivery`\" (Delhivery) - \"`dtdc`\" (DTDC) - \"`tpc india`\" (TPC India) Supported carriers for FR are: - \"`la poste`\" (La Poste) *automatic status updates * - \"`colissimo`\" (Colissimo by La Poste) *automatic status updates* - \"`ups`\" (United Parcel Service) *automatic status updates * - \"`chronopost`\" (Chronopost by La Poste) - \"`gls`\" (General Logistics Systems France) - \"`dpd`\" (DPD Group by GeoPost) - \"`bpost`\" (Belgian Post Group) - \"`colis prive`\" (Colis Privé) - \"`boxtal`\" (Boxtal) - \"`geodis`\" (GEODIS) - \"`tnt`\" (TNT) - \"`db schenker`\" (DB Schenker) - \"`aramex`\" (Aramex) "]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the shipment has been created, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Present only if `status` is `delivered`"]
        pub delivery_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the shipment."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The line items that are shipped."]
        pub line_items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledDeliveryDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Delivery details of the shipment if scheduling is needed."]
        pub scheduled_delivery_details:
            ::std::option::Option<::std::boxed::Box<OrderShipmentScheduledDeliveryDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the shipment. Acceptable values are: - \"`delivered`\" - \"`readyForPickup`\" - \"`shipped`\" - \"`undeliverable`\" "]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking ID for the shipment."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrderShipment {
        pub fn builder() -> OrderShipmentBuilder {
            OrderShipmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderShipmentLineItemShipment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item that is shipped. This value is assigned by Google when an order is created. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to ship. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity that is shipped."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
    }
    impl OrderShipmentLineItemShipment {
        pub fn builder() -> OrderShipmentLineItemShipmentBuilder {
            OrderShipmentLineItemShipmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderShipmentScheduledDeliveryDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierPhoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The phone number of the carrier fulfilling the delivery. The phone number is formatted as the international notation in ITU-T Recommendation E.123 (e.g., \"+41 44 668 1800\")."]
        pub carrier_phone_number: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date a shipment is scheduled for delivery, in ISO 8601 format."]
        pub scheduled_date: ::std::option::Option<::std::string::String>,
    }
    impl OrderShipmentScheduledDeliveryDetails {
        pub fn builder() -> OrderShipmentScheduledDeliveryDetailsBuilder {
            OrderShipmentScheduledDeliveryDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCreateChargeInvoiceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invoiceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] The ID of the invoice."]
        pub invoice_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invoiceSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Invoice summary."]
        pub invoice_summary: ::std::option::Option<::std::boxed::Box<InvoiceSummary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemInvoices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Invoice details per line item."]
        pub line_item_invoices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ShipmentInvoiceLineItemInvoice>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] The ID of the operation, unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges."]
        pub shipment_group_id: ::std::option::Option<::std::string::String>,
    }
    impl OrderinvoicesCreateChargeInvoiceRequest {
        pub fn builder() -> OrderinvoicesCreateChargeInvoiceRequestBuilder {
            OrderinvoicesCreateChargeInvoiceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCreateChargeInvoiceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderinvoicesCreateChargeInvoiceResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrderinvoicesCreateChargeInvoiceResponse {
        pub fn builder() -> OrderinvoicesCreateChargeInvoiceResponseBuilder {
            OrderinvoicesCreateChargeInvoiceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCreateRefundInvoiceRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invoiceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] The ID of the invoice."]
        pub invoice_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] The ID of the operation, unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refundOnlyOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to create a refund-only invoice. Exactly one of `refundOnlyOption` or `returnOption` must be provided."]
        pub refund_only_option: ::std::option::Option<
            ::std::boxed::Box<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to create an invoice for a refund and mark all items within the invoice as returned. Exactly one of `refundOnlyOption` or `returnOption` must be provided."]
        pub return_option: ::std::option::Option<
            ::std::boxed::Box<OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentInvoices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Invoice details for different shipment groups."]
        pub shipment_invoices:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentInvoice>>>,
    }
    impl OrderinvoicesCreateRefundInvoiceRequest {
        pub fn builder() -> OrderinvoicesCreateRefundInvoiceRequestBuilder {
            OrderinvoicesCreateRefundInvoiceRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCreateRefundInvoiceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderinvoicesCreateRefundInvoiceResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrderinvoicesCreateRefundInvoiceResponse {
        pub fn builder() -> OrderinvoicesCreateRefundInvoiceResponseBuilder {
            OrderinvoicesCreateRefundInvoiceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional description of the refund reason."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOption {
        pub fn builder(
        ) -> OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOptionBuilder {
            OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceRefundOptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional description of the return reason."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOption {
        pub fn builder(
        ) -> OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOptionBuilder {
            OrderinvoicesCustomBatchRequestEntryCreateRefundInvoiceReturnOptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderreportsListDisbursementsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disbursements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of disbursements."]
        pub disbursements:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReportDisbursement>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreportsListDisbursementsResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of disbursements."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl OrderreportsListDisbursementsResponse {
        pub fn builder() -> OrderreportsListDisbursementsResponseBuilder {
            OrderreportsListDisbursementsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderreportsListTransactionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreportsListTransactionsResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of transactions."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transactions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of transactions."]
        pub transactions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderReportTransaction>>>,
    }
    impl OrderreportsListTransactionsResponse {
        pub fn builder() -> OrderreportsListTransactionsResponseBuilder {
            OrderreportsListTransactionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrderreturnsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#orderreturnsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of returns."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MerchantOrderReturn>>>,
    }
    impl OrderreturnsListResponse {
        pub fn builder() -> OrderreturnsListResponseBuilder {
            OrderreturnsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersAcknowledgeRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersAcknowledgeRequest {
        pub fn builder() -> OrdersAcknowledgeRequestBuilder {
            OrdersAcknowledgeRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersAcknowledgeResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersAcknowledgeResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersAcknowledgeResponse {
        pub fn builder() -> OrdersAcknowledgeResponseBuilder {
            OrdersAcknowledgeResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersAdvanceTestOrderResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersAdvanceTestOrderResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersAdvanceTestOrderResponse {
        pub fn builder() -> OrdersAdvanceTestOrderResponseBuilder {
            OrdersAdvanceTestOrderResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelLineItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to cancel. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to cancel."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelLineItemRequest {
        pub fn builder() -> OrdersCancelLineItemRequestBuilder {
            OrdersCancelLineItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelLineItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelLineItemResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelLineItemResponse {
        pub fn builder() -> OrdersCancelLineItemResponseBuilder {
            OrdersCancelLineItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelRequest {
        pub fn builder() -> OrdersCancelRequestBuilder {
            OrdersCancelRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelResponse {
        pub fn builder() -> OrdersCancelResponseBuilder {
            OrdersCancelResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelTestOrderByCustomerRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Acceptable values are: - \"`changedMind`\" - \"`orderedWrongItem`\" - \"`other`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelTestOrderByCustomerRequest {
        pub fn builder() -> OrdersCancelTestOrderByCustomerRequestBuilder {
            OrdersCancelTestOrderByCustomerRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCancelTestOrderByCustomerResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCancelTestOrderByCustomerResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCancelTestOrderByCustomerResponse {
        pub fn builder() -> OrdersCancelTestOrderByCustomerResponseBuilder {
            OrdersCancelTestOrderByCustomerResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCreateTestOrderRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory code of the country of the test order to create. Affects the currency and addresses of orders created via `template_name`, or the addresses of orders created via `test_order`. Acceptable values are: - \"`US`\" - \"`FR`\" Defaults to `US`."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test order template to use. Specify as an alternative to `testOrder` as a shortcut for retrieving a template and then creating an order using that template. Acceptable values are: - \"`template1`\" - \"`template1a`\" - \"`template1b`\" - \"`template2`\" - \"`template3`\" "]
        pub template_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testOrder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test order to create."]
        pub test_order: ::std::option::Option<::std::boxed::Box<TestOrder>>,
    }
    impl OrdersCreateTestOrderRequest {
        pub fn builder() -> OrdersCreateTestOrderRequestBuilder {
            OrdersCreateTestOrderRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCreateTestOrderResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCreateTestOrderResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the newly created test order."]
        pub order_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCreateTestOrderResponse {
        pub fn builder() -> OrdersCreateTestOrderResponseBuilder {
            OrdersCreateTestOrderResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCreateTestReturnRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returned items."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<OrdersCustomBatchRequestEntryCreateTestReturnReturnItem>,
            >,
        >,
    }
    impl OrdersCreateTestReturnRequest {
        pub fn builder() -> OrdersCreateTestReturnRequestBuilder {
            OrdersCreateTestReturnRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCreateTestReturnResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCreateTestReturnResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the newly created test order return."]
        pub return_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCreateTestReturnResponse {
        pub fn builder() -> OrdersCreateTestReturnResponseBuilder {
            OrdersCreateTestReturnResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchRequestEntry>>,
        >,
    }
    impl OrdersCustomBatchRequest {
        pub fn builder() -> OrdersCustomBatchRequestBuilder {
            OrdersCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `cancel` method."]
        pub cancel: ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryCancel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cancelLineItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `cancelLineItem` method."]
        pub cancel_line_item:
            ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryCancelLineItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inStoreRefundLineItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `inStoreReturnLineItem` method."]
        pub in_store_refund_line_item: ::std::option::Option<
            ::std::boxed::Box<OrdersCustomBatchRequestEntryInStoreRefundLineItem>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merchant order ID. Required for `updateMerchantOrderId` and `getByMerchantOrderId` methods."]
        pub merchant_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`acknowledge`\" - \"`cancel`\" - \"`cancelLineItem`\" - \"`get`\" - \"`getByMerchantOrderId`\" - \"`inStoreRefundLineItem`\" - \"`refund`\" - \"`rejectReturnLineItem`\" - \"`returnLineItem`\" - \"`returnRefundLineItem`\" - \"`setLineItemMetadata`\" - \"`shipLineItems`\" - \"`updateLineItemShippingDetails`\" - \"`updateMerchantOrderId`\" - \"`updateShipment`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order. Required for all methods beside `get` and `getByMerchantOrderId`."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the order. Required for all methods beside `getByMerchantOrderId`."]
        pub order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "refund")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `refund` method."]
        pub refund: ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryRefund>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejectReturnLineItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `rejectReturnLineItem` method."]
        pub reject_return_line_item: ::std::option::Option<
            ::std::boxed::Box<OrdersCustomBatchRequestEntryRejectReturnLineItem>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnLineItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `returnLineItem` method."]
        pub return_line_item:
            ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryReturnLineItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnRefundLineItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `returnRefundLineItem` method."]
        pub return_refund_line_item: ::std::option::Option<
            ::std::boxed::Box<OrdersCustomBatchRequestEntryReturnRefundLineItem>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setLineItemMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `setLineItemMetadata` method."]
        pub set_line_item_metadata: ::std::option::Option<
            ::std::boxed::Box<OrdersCustomBatchRequestEntrySetLineItemMetadata>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipLineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `shipLineItems` method."]
        pub ship_line_items:
            ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItems>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateLineItemShippingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `updateLineItemShippingDate` method."]
        pub update_line_item_shipping_details: ::std::option::Option<
            ::std::boxed::Box<OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateShipment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for `updateShipment` method."]
        pub update_shipment:
            ::std::option::Option<::std::boxed::Box<OrdersCustomBatchRequestEntryUpdateShipment>>,
    }
    impl OrdersCustomBatchRequestEntry {
        pub fn builder() -> OrdersCustomBatchRequestEntryBuilder {
            OrdersCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryCancel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryCancel {
        pub fn builder() -> OrdersCustomBatchRequestEntryCancelBuilder {
            OrdersCustomBatchRequestEntryCancelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryCancelLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Amount to refund for the cancelation. Optional. If not set, Google will calculate the default based on the price and tax of the items involved. The amount must not be larger than the net amount left on the order."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to cancellation amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to cancel. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to cancel. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to cancel."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the cancellation. Acceptable values are: - \"`customerInitiatedCancel`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`shippingPriceError`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryCancelLineItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryCancelLineItemBuilder {
            OrdersCustomBatchRequestEntryCancelLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Quantity that is returned."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
    }
    impl OrdersCustomBatchRequestEntryCreateTestReturnReturnItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryCreateTestReturnReturnItemBuilder {
            OrdersCustomBatchRequestEntryCreateTestReturnReturnItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryInStoreRefundLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. Required."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that correspond to refund amount in amountPretax. Required."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryInStoreRefundLineItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryInStoreRefundLineItemBuilder {
            OrdersCustomBatchRequestEntryInStoreRefundLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryRefund {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. Either amount or amountPretax should be filled."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`deliveredLateByCarrier`\" - \"`feeAdjustment`\" - \"`lateShipmentCredit`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`shippingCostAdjustment`\" - \"`taxAdjustment`\" - \"`undeliverableShippingAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryRefund {
        pub fn builder() -> OrdersCustomBatchRequestEntryRefundBuilder {
            OrdersCustomBatchRequestEntryRefundBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryRejectReturnLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`damagedOrUsed`\" - \"`missingComponent`\" - \"`notEligible`\" - \"`other`\" - \"`outOfReturnWindow`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryRejectReturnLineItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryRejectReturnLineItemBuilder {
            OrdersCustomBatchRequestEntryRejectReturnLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryReturnLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryReturnLineItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryReturnLineItemBuilder {
            OrdersCustomBatchRequestEntryReturnLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryReturnRefundLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method)."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryReturnRefundLineItem {
        pub fn builder() -> OrdersCustomBatchRequestEntryReturnRefundLineItemBuilder {
            OrdersCustomBatchRequestEntryReturnRefundLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntrySetLineItemMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntrySetLineItemMetadata {
        pub fn builder() -> OrdersCustomBatchRequestEntrySetLineItemMetadataBuilder {
            OrdersCustomBatchRequestEntrySetLineItemMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryShipLineItems {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line items to ship."]
        pub line_items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the shipment group. Required for orders that use the orderinvoices service."]
        pub shipment_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The ID of the shipment."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs)."]
        pub shipment_infos: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryShipLineItems {
        pub fn builder() -> OrdersCustomBatchRequestEntryShipLineItemsBuilder {
            OrdersCustomBatchRequestEntryShipLineItemsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ID of the shipment. This is assigned by the merchant and is unique to each shipment."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking ID for the shipment."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo {
        pub fn builder() -> OrdersCustomBatchRequestEntryShipLineItemsShipmentInfoBuilder {
            OrdersCustomBatchRequestEntryShipLineItemsShipmentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliverByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
        pub deliver_by_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
        pub ship_by_date: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryUpdateLineItemShippingDetails {
        pub fn builder() -> OrdersCustomBatchRequestEntryUpdateLineItemShippingDetailsBuilder {
            OrdersCustomBatchRequestEntryUpdateLineItemShippingDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchRequestEntryUpdateShipment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`."]
        pub delivery_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the shipment."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New status for the shipment. Not updated if missing. Acceptable values are: - \"`delivered`\" - \"`undeliverable`\" - \"`readyForPickup`\" "]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking ID for the shipment. Not updated if missing."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchRequestEntryUpdateShipment {
        pub fn builder() -> OrdersCustomBatchRequestEntryUpdateShipmentBuilder {
            OrdersCustomBatchRequestEntryUpdateShipmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrdersCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersCustomBatchResponse {
        pub fn builder() -> OrdersCustomBatchResponseBuilder {
            OrdersCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Only defined if 1. the request was successful; and 2. the method is not `get`, `getByMerchantOrderId`, or one of the test methods. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#ordersCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved order. Only defined if the method is `get` and if the request was successful."]
        pub order: ::std::option::Option<::std::boxed::Box<Order>>,
    }
    impl OrdersCustomBatchResponseEntry {
        pub fn builder() -> OrdersCustomBatchResponseEntryBuilder {
            OrdersCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersGetByMerchantOrderIdResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersGetByMerchantOrderIdResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested order."]
        pub order: ::std::option::Option<::std::boxed::Box<Order>>,
    }
    impl OrdersGetByMerchantOrderIdResponse {
        pub fn builder() -> OrdersGetByMerchantOrderIdResponseBuilder {
            OrdersGetByMerchantOrderIdResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersGetTestOrderTemplateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersGetTestOrderTemplateResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested test order template."]
        pub template: ::std::option::Option<::std::boxed::Box<TestOrder>>,
    }
    impl OrdersGetTestOrderTemplateResponse {
        pub fn builder() -> OrdersGetTestOrderTemplateResponseBuilder {
            OrdersGetTestOrderTemplateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersInStoreRefundLineItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. Required."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that correspond to refund amount in amountPretax. Required."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersInStoreRefundLineItemRequest {
        pub fn builder() -> OrdersInStoreRefundLineItemRequestBuilder {
            OrdersInStoreRefundLineItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersInStoreRefundLineItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersInStoreRefundLineItemResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersInStoreRefundLineItemResponse {
        pub fn builder() -> OrdersInStoreRefundLineItemResponseBuilder {
            OrdersInStoreRefundLineItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of orders."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Order>>>,
    }
    impl OrdersListResponse {
        pub fn builder() -> OrdersListResponseBuilder {
            OrdersListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersRefundRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use amountPretax and amountTax instead."]
        pub amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. Either amount or amountPretax should be filled."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the refund. Acceptable values are: - \"`adjustment`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`deliveredLateByCarrier`\" - \"`feeAdjustment`\" - \"`lateShipmentCredit`\" - \"`noInventory`\" - \"`other`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`shippingCostAdjustment`\" - \"`taxAdjustment`\" - \"`undeliverableShippingAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersRefundRequest {
        pub fn builder() -> OrdersRefundRequestBuilder {
            OrdersRefundRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersRefundResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRefundResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersRefundResponse {
        pub fn builder() -> OrdersRefundResponseBuilder {
            OrdersRefundResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersRejectReturnLineItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`damagedOrUsed`\" - \"`missingComponent`\" - \"`notEligible`\" - \"`other`\" - \"`outOfReturnWindow`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersRejectReturnLineItemRequest {
        pub fn builder() -> OrdersRejectReturnLineItemRequestBuilder {
            OrdersRejectReturnLineItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersRejectReturnLineItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersRejectReturnLineItemResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersRejectReturnLineItemResponse {
        pub fn builder() -> OrdersRejectReturnLineItemResponseBuilder {
            OrdersRejectReturnLineItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersReturnLineItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersReturnLineItemRequest {
        pub fn builder() -> OrdersReturnLineItemRequestBuilder {
            OrdersReturnLineItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersReturnLineItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersReturnLineItemResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersReturnLineItemResponse {
        pub fn builder() -> OrdersReturnLineItemResponseBuilder {
            OrdersReturnLineItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersReturnRefundLineItemRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountPretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that is refunded. If omitted, refundless return is assumed (same as calling returnLineItem method)."]
        pub amount_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amount that corresponds to refund amount in amountPretax. Optional, but if filled, then amountPretax must be set. Calculated automatically if not provided."]
        pub amount_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to return. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to return. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity to return and refund. Quantity is required."]
        pub quantity: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason for the return. Acceptable values are: - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`invalidCoupon`\" - \"`malformedShippingAddress`\" - \"`other`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`qualityNotAsExpected`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The explanation of the reason."]
        pub reason_text: ::std::option::Option<::std::string::String>,
    }
    impl OrdersReturnRefundLineItemRequest {
        pub fn builder() -> OrdersReturnRefundLineItemRequestBuilder {
            OrdersReturnRefundLineItemRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersReturnRefundLineItemResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersReturnRefundLineItemResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersReturnRefundLineItemResponse {
        pub fn builder() -> OrdersReturnRefundLineItemResponseBuilder {
            OrdersReturnRefundLineItemResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersSetLineItemMetadataRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "annotations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub annotations: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderMerchantProvidedAnnotation>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersSetLineItemMetadataRequest {
        pub fn builder() -> OrdersSetLineItemMetadataRequestBuilder {
            OrdersSetLineItemMetadataRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersSetLineItemMetadataResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersSetLineItemMetadataResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersSetLineItemMetadataResponse {
        pub fn builder() -> OrdersSetLineItemMetadataResponseBuilder {
            OrdersSetLineItemMetadataResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersShipLineItemsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The carrier handling the shipment. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Line items to ship."]
        pub line_items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderShipmentLineItemShipment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the shipment group. Required for orders that use the orderinvoices service."]
        pub shipment_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The ID of the shipment."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipment information. This field is repeated because a single line item can be shipped in several packages (and have several tracking IDs)."]
        pub shipment_infos: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<OrdersCustomBatchRequestEntryShipLineItemsShipmentInfo>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use shipmentInfo instead. The tracking ID for the shipment."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersShipLineItemsRequest {
        pub fn builder() -> OrdersShipLineItemsRequestBuilder {
            OrdersShipLineItemsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersShipLineItemsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersShipLineItemsResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersShipLineItemsResponse {
        pub fn builder() -> OrdersShipLineItemsResponseBuilder {
            OrdersShipLineItemsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateLineItemShippingDetailsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliverByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updated delivery by date, in ISO 8601 format. If not specified only ship by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
        pub deliver_by_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the line item to set metadata. Either lineItemId or productId is required."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to set metadata. This is the REST ID used in the products service. Either lineItemId or productId is required."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipByDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Updated ship by date, in ISO 8601 format. If not specified only deliver by date is updated. Provided date should be within 1 year timeframe and can not be a date in the past."]
        pub ship_by_date: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateLineItemShippingDetailsRequest {
        pub fn builder() -> OrdersUpdateLineItemShippingDetailsRequestBuilder {
            OrdersUpdateLineItemShippingDetailsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateLineItemShippingDetailsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateLineItemShippingDetailsResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateLineItemShippingDetailsResponse {
        pub fn builder() -> OrdersUpdateLineItemShippingDetailsResponseBuilder {
            OrdersUpdateLineItemShippingDetailsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateMerchantOrderIdRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantOrderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merchant order id to be assigned to the order. Must be unique per merchant."]
        pub merchant_order_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateMerchantOrderIdRequest {
        pub fn builder() -> OrdersUpdateMerchantOrderIdRequestBuilder {
            OrdersUpdateMerchantOrderIdRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateMerchantOrderIdResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateMerchantOrderIdResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateMerchantOrderIdResponse {
        pub fn builder() -> OrdersUpdateMerchantOrderIdResponseBuilder {
            OrdersUpdateMerchantOrderIdResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateShipmentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier handling the shipment. Not updated if missing. See `shipments[].carrier` in the Orders resource representation for a list of acceptable values."]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the shipment has been delivered, in ISO 8601 format. Optional and can be provided only if `status` is `delivered`."]
        pub delivery_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the operation. Unique across all operations for a given order."]
        pub operation_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the shipment."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "New status for the shipment. Not updated if missing. Acceptable values are: - \"`delivered`\" - \"`undeliverable`\" - \"`readyForPickup`\" "]
        pub status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking ID for the shipment. Not updated if missing."]
        pub tracking_id: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateShipmentRequest {
        pub fn builder() -> OrdersUpdateShipmentRequestBuilder {
            OrdersUpdateShipmentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct OrdersUpdateShipmentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "executionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the execution. Acceptable values are: - \"`duplicate`\" - \"`executed`\" "]
        pub execution_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#ordersUpdateShipmentResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl OrdersUpdateShipmentResponse {
        pub fn builder() -> OrdersUpdateShipmentResponseBuilder {
            OrdersUpdateShipmentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PickupCarrierService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the pickup carrier (e.g., `\"UPS\"`). Required."]
        pub carrier_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the pickup service (e.g., `\"Access point\"`). Required."]
        pub service_name: ::std::option::Option<::std::string::String>,
    }
    impl PickupCarrierService {
        pub fn builder() -> PickupCarrierServiceBuilder {
            PickupCarrierServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PickupServicesPickupService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the carrier (e.g., `\"UPS\"`). Always present."]
        pub carrier_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR country code of the carrier (e.g., \"US\"). Always present."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "serviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the pickup service (e.g., `\"Access point\"`). Always present."]
        pub service_name: ::std::option::Option<::std::string::String>,
    }
    impl PickupServicesPickupService {
        pub fn builder() -> PickupServicesPickupServiceBuilder {
            PickupServicesPickupServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosCustomBatchRequestEntry>>>,
    }
    impl PosCustomBatchRequest {
        pub fn builder() -> PosCustomBatchRequestBuilder {
            PosCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inventory to submit. This should be set only if the method is `inventory`."]
        pub inventory: ::std::option::Option<::std::boxed::Box<PosInventory>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the POS data provider."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" - \"`inventory`\" - \"`sale`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sale information to submit. This should be set only if the method is `sale`."]
        pub sale: ::std::option::Option<::std::boxed::Box<PosSale>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "store")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The store information to submit. This should be set only if the method is `insert`."]
        pub store: ::std::option::Option<::std::boxed::Box<PosStore>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The store code. This should be set only if the method is `delete` or `get`."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetMerchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account for which to get/submit data."]
        pub target_merchant_id: ::std::option::Option<::std::string::String>,
    }
    impl PosCustomBatchRequestEntry {
        pub fn builder() -> PosCustomBatchRequestEntryBuilder {
            PosCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosCustomBatchResponseEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl PosCustomBatchResponse {
        pub fn builder() -> PosCustomBatchResponseBuilder {
            PosCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry to which this entry responds."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if, and only if, the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inventory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated inventory information."]
        pub inventory: ::std::option::Option<::std::boxed::Box<PosInventory>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated sale information."]
        pub sale: ::std::option::Option<::std::boxed::Box<PosSale>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "store")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved or updated store information."]
        pub store: ::std::option::Option<::std::boxed::Box<PosStore>>,
    }
    impl PosCustomBatchResponseEntry {
        pub fn builder() -> PosCustomBatchResponseEntryBuilder {
            PosCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosDataProviders {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Country code."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posDataProviders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of POS data providers."]
        pub pos_data_providers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<PosDataProvidersPosDataProvider>>,
        >,
    }
    impl PosDataProviders {
        pub fn builder() -> PosDataProvidersBuilder {
            PosDataProvidersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosDataProvidersPosDataProvider {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of Pos data Provider."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of this POS data Provider."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "providerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account."]
        pub provider_id: ::std::option::Option<::std::string::String>,
    }
    impl PosDataProvidersPosDataProvider {
        pub fn builder() -> PosDataProvidersPosDataProviderBuilder {
            PosDataProvidersPosDataProviderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The absolute quantity of an item available at the given store."]
    pub struct PosInventory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posInventory`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The current price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The available quantity of the item."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosInventory {
        pub fn builder() -> PosInventoryBuilder {
            PosInventoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosInventoryRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The current price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The available quantity of the item."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosInventoryRequest {
        pub fn builder() -> PosInventoryRequestBuilder {
            PosInventoryRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosInventoryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posInventoryResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The current price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The available quantity of the item."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosInventoryResponse {
        pub fn builder() -> PosInventoryResponseBuilder {
            PosInventoryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PosStore>>>,
    }
    impl PosListResponse {
        pub fn builder() -> PosListResponseBuilder {
            PosListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The change of the available quantity of an item at the given store."]
    pub struct PosSale {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posSale`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID to group items from the same sale event."]
        pub sale_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosSale {
        pub fn builder() -> PosSaleBuilder {
            PosSaleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosSaleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID to group items from the same sale event."]
        pub sale_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosSaleRequest {
        pub fn builder() -> PosSaleRequestBuilder {
            PosSaleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PosSaleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item."]
        pub item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#posSaleResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The relative change of the available quantity. Negative for items returned."]
        pub quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "saleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A unique ID to group items from the same sale event."]
        pub sale_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The identifier of the merchant's store. Either a `storeCode` inserted via the API or the code of the store in Google My Business."]
        pub store_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The inventory timestamp, in ISO 8601 format."]
        pub timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PosSaleResponse {
        pub fn builder() -> PosSaleResponseBuilder {
            PosSaleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Store resource."]
    pub struct PosStore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#posStore`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The street address of the store."]
        pub store_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A store identifier that is unique for the given merchant."]
        pub store_code: ::std::option::Option<::std::string::String>,
    }
    impl PosStore {
        pub fn builder() -> PosStoreBuilder {
            PosStoreBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostalCodeGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory code of the country the postal code group applies to. Required."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the postal code group, referred to in headers. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeRanges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A range of postal codes. Required."]
        pub postal_code_ranges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCodeRange>>>,
    }
    impl PostalCodeGroup {
        pub fn builder() -> PostalCodeGroupBuilder {
            PostalCodeGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostalCodeRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeRangeBegin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A postal code or a pattern of the form `prefix*` denoting the inclusive lower bound of the range defining the area. Examples values: `\"94108\"`, `\"9410*\"`, `\"9*\"`. Required."]
        pub postal_code_range_begin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeRangeEnd")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A postal code or a pattern of the form `prefix*` denoting the inclusive upper bound of the range defining the area. It must have the same length as `postalCodeRangeBegin`: if `postalCodeRangeBegin` is a postal code then `postalCodeRangeEnd` must be a postal code too; if `postalCodeRangeBegin` is a pattern then `postalCodeRangeEnd` must be a pattern with the same prefix length. Optional: if not set, then the area is defined as being all the postal codes matching `postalCodeRangeBegin`."]
        pub postal_code_range_end: ::std::option::Option<::std::string::String>,
    }
    impl PostalCodeRange {
        pub fn builder() -> PostalCodeRangeBuilder {
            PostalCodeRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Price {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency of the price."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The price represented as a number."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Price {
        pub fn builder() -> PriceBuilder {
            PriceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = " Required product attributes are primarily defined by the products data specification. See the Products Data Specification Help Center article for information. Some attributes are country-specific, so make sure you select the appropriate country in the drop-down selector at the top of the page. Product data. After inserting, updating, or deleting a product, it may take several minutes before changes take effect."]
    pub struct Product {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalImageLinks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional URLs of images of the item."]
        pub additional_image_links: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalProductTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional categories of the item (formatted as in products data specification)."]
        pub additional_product_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Should be set to true if the item is targeted towards adults."]
        pub adult: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adwordsGrouping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used to group items in an arbitrary way. Only for CPA%, discouraged otherwise."]
        pub adwords_grouping: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adwordsLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Similar to adwords_grouping, but only works on CPC."]
        pub adwords_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adwordsRedirect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Allows advertisers to override the item URL when the product is shown within the context of Product Ads."]
        pub adwords_redirect: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ageGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target age group of the item. Acceptable values are: - \"`adult`\" - \"`infant`\" - \"`kids`\" - \"`newborn`\" - \"`toddler`\" - \"`youngAdult`\" "]
        pub age_group: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aspects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Do not use."]
        pub aspects: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductAspect>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Availability status of the item. Acceptable values are: - \"`in stock`\" - \"`out of stock`\" - \"`preorder`\" "]
        pub availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "availabilityDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The day a pre-ordered product becomes available for delivery, in ISO 8601 format."]
        pub availability_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Brand of the item."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canonicalLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for the canonical version of your item's landing page."]
        pub canonical_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The item's channel (online or local). Acceptable values are: - \"`local`\" - \"`online`\" "]
        pub channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "color")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Color of the item."]
        pub color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Condition or state of the item. Acceptable values are: - \"`new`\" - \"`refurbished`\" - \"`used`\" "]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item."]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "costOfGoodsSold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Cost of goods sold. Used for gross profit reporting."]
        pub cost_of_goods_sold: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of custom (merchant-provided) attributes. It can also be used for submitting any attribute of the feed specification in its generic form (e.g., `{ \"name\": \"size type\", \"value\": \"regular\" }`). This is useful for submitting attributes not explicitly exposed by the API, such as additional attributes used for Buy on Google (formerly known as Shopping Actions)."]
        pub custom_attributes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomAttribute>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of custom (merchant-provided) custom attribute groups."]
        pub custom_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel0")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 0 for custom grouping of items in a Shopping campaign."]
        pub custom_label0: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel1")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 1 for custom grouping of items in a Shopping campaign."]
        pub custom_label1: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel2")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 2 for custom grouping of items in a Shopping campaign."]
        pub custom_label2: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel3")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 3 for custom grouping of items in a Shopping campaign."]
        pub custom_label3: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customLabel4")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom label 4 for custom grouping of items in a Shopping campaign."]
        pub custom_label4: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the item."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the intended destinations for the product."]
        pub destinations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductDestination>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayAdsId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier for an item for dynamic remarketing campaigns."]
        pub display_ads_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayAdsLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL directly to your item's landing page for dynamic remarketing campaigns."]
        pub display_ads_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayAdsSimilarIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Advertiser-specified recommendations."]
        pub display_ads_similar_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayAdsTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of an item for dynamic remarketing campaigns."]
        pub display_ads_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayAdsValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Offer margin for dynamic remarketing campaigns."]
        pub display_ads_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "energyEfficiencyClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
        pub energy_efficiency_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item should expire, as specified upon insertion, in ISO 8601 format. The actual expiration date in Google Shopping is exposed in `productstatuses` as `googleExpirationDate` and might be earlier if `expirationDate` is too far in the future."]
        pub expiration_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Target gender of the item. Acceptable values are: - \"`female`\" - \"`male`\" - \"`unisex`\" "]
        pub gender: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleProductCategory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Google's category of the item (see [Google product taxonomy](https://support.google.com/merchants/answer/1705911)). When querying products, this field will contain the user provided value. There is currently no way to get back the auto assigned google product categories through the API."]
        pub google_product_category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number (GTIN) of the item."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The REST ID of the product. Content API methods that operate on products take this as their `productId` parameter. The REST ID for a product is of the form channel:contentLanguage: targetCountry: offerId."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "identifierExists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "False when the item does not have unique product identifiers appropriate to its category, such as GTIN, MPN, and brand. Required according to the Unique Product Identifier Rules for all target countries except for Canada."]
        pub identifier_exists: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL of an image of the item."]
        pub image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "installment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number and amount of installments to pay for an item."]
        pub installment: ::std::option::Option<::std::boxed::Box<Installment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isBundle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the item is a merchant-defined bundle. A bundle is a custom grouping of different products sold by a merchant for a single price."]
        pub is_bundle: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shared identifier for all variants of the same product."]
        pub item_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#product`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL directly linking to your item's page on your website."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "loyaltyPoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Loyalty points that users receive after purchasing the item. Japan only."]
        pub loyalty_points: ::std::option::Option<::std::boxed::Box<LoyaltyPoints>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "material")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The material of which the item is made."]
        pub material: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxEnergyEfficiencyClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
        pub max_energy_efficiency_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxHandlingTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximal product handling time (in business days)."]
        pub max_handling_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minEnergyEfficiencyClass")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The energy efficiency class as defined in EU directive 2010/30/EU. Acceptable values are: - \"`A`\" - \"`A+`\" - \"`A++`\" - \"`A+++`\" - \"`B`\" - \"`C`\" - \"`D`\" - \"`E`\" - \"`F`\" - \"`G`\" "]
        pub min_energy_efficiency_class: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minHandlingTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimal product handling time (in business days)."]
        pub min_handling_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mobileLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URL for the mobile-optimized version of your item's landing page."]
        pub mobile_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mpn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manufacturer Part Number (MPN) of the item."]
        pub mpn: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "multipack")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of identical products in a merchant-defined multipack."]
        pub multipack: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique identifier for the item. Leading and trailing whitespaces are stripped and multiple whitespaces are replaced by a single whitespace upon submission. Only valid unicode characters are accepted. See the products feed specification for details. *Note:* Content API methods that operate on products take the REST ID of the product, *not* this identifier."]
        pub offer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "onlineOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub online_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pattern")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item's pattern (e.g. polka dots)."]
        pub pattern: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Price of the item."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Your category of the item (formatted as in products data specification)."]
        pub product_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique ID of a promotion."]
        pub promotion_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePrice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Advertised sale price of the item."]
        pub sale_price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "salePriceEffectiveDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date range during which the item is on sale (see products data specification )."]
        pub sale_price_effective_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sellOnGoogleQuantity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quantity of the product that is available for selling on Google. Supported only for online products."]
        pub sell_on_google_quantity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipping rules."]
        pub shipping: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductShipping>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Height of the item for shipping."]
        pub shipping_height: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingLabel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shipping label of the product, used to group product in account-level shipping rules."]
        pub shipping_label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Length of the item for shipping."]
        pub shipping_length: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingWeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Weight of the item for shipping."]
        pub shipping_weight: ::std::option::Option<::std::boxed::Box<ProductShippingWeight>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Width of the item for shipping."]
        pub shipping_width: ::std::option::Option<::std::boxed::Box<ProductShippingDimension>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeSystem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "System in which the size is specified. Recommended for apparel items. Acceptable values are: - \"`AU`\" - \"`BR`\" - \"`CN`\" - \"`DE`\" - \"`EU`\" - \"`FR`\" - \"`IT`\" - \"`JP`\" - \"`MEX`\" - \"`UK`\" - \"`US`\" "]
        pub size_system: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cut of the item. Recommended for apparel items. Acceptable values are: - \"`big and tall`\" - \"`maternity`\" - \"`oversize`\" - \"`petite`\" - \"`plus`\" - \"`regular`\" "]
        pub size_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Size of the item. Only one value is allowed. For variants with different sizes, insert a separate product for each size with the same `itemGroupId` value (see size definition)."]
        pub sizes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of the offer, i.e., how the offer was created. Acceptable values are: - \"`api`\" - \"`crawl`\" - \"`feed`\" "]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory code for the item."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax information."]
        pub taxes: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductTax>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title of the item."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitPricingBaseMeasure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The preference of the denominator of the unit price."]
        pub unit_pricing_base_measure:
            ::std::option::Option<::std::boxed::Box<ProductUnitPricingBaseMeasure>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitPricingMeasure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measure and dimension of an item."]
        pub unit_pricing_measure:
            ::std::option::Option<::std::boxed::Box<ProductUnitPricingMeasure>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validatedDestinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. The read-only list of intended destinations which passed validation."]
        pub validated_destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only warnings."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Error>>>,
    }
    impl Product {
        pub fn builder() -> ProductBuilder {
            ProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductAmount {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priceAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pre-tax or post-tax price depending on the location of the order."]
        pub price_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "remittedTaxAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Remitted tax value."]
        pub remitted_tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax value."]
        pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl ProductAmount {
        pub fn builder() -> ProductAmountBuilder {
            ProductAmountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductAspect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aspectName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub aspect_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub destination_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intention")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub intention: ::std::option::Option<::std::string::String>,
    }
    impl ProductAspect {
        pub fn builder() -> ProductAspectBuilder {
            ProductAspectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductDestination {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the destination."]
        pub destination_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intention")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the destination is required, excluded or should be validated. Acceptable values are: - \"`default`\" - \"`excluded`\" - \"`optional`\" - \"`required`\" "]
        pub intention: ::std::option::Option<::std::string::String>,
    }
    impl ProductDestination {
        pub fn builder() -> ProductDestinationBuilder {
            ProductDestinationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductShipping {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory code of the country to which an item will ship."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationGroupName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location where the shipping is applicable, represented by a location group name."]
        pub location_group_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of a location that the shipping rate applies to as defined in the AdWords API."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The postal code range that the shipping rate applies to, represented by a postal code, a postal code prefix followed by a * wildcard, a range between two postal codes or two postal code prefixes of equal length."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fixed shipping price, represented as a number."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic region to which a shipping rate applies."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "service")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A free-form description of the service class or delivery speed."]
        pub service: ::std::option::Option<::std::string::String>,
    }
    impl ProductShipping {
        pub fn builder() -> ProductShippingBuilder {
            ProductShippingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductShippingDimension {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit of value."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dimension of the product used to calculate the shipping cost of the item."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl ProductShippingDimension {
        pub fn builder() -> ProductShippingDimensionBuilder {
            ProductShippingDimensionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductShippingWeight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit of value."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The weight of the product used to calculate the shipping cost of the item."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl ProductShippingWeight {
        pub fn builder() -> ProductShippingWeightBuilder {
            ProductShippingWeightBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a product, i.e., information about a product computed asynchronously."]
    pub struct ProductStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item has been created, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataQualityIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DEPRECATED - never populated"]
        pub data_quality_issues: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductStatusDataQualityIssue>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intended destinations for the product."]
        pub destination_statuses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductStatusDestinationStatus>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "googleExpirationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item expires in Google Shopping, in ISO 8601 format."]
        pub google_expiration_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemLevelIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of all issues associated with the product."]
        pub item_level_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatusItemLevelIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productStatus`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Date on which the item has been last updated, in ISO 8601 format."]
        pub last_update_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "link")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link to the product."]
        pub link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Product data after applying all the join inputs."]
        pub product: ::std::option::Option<::std::boxed::Box<Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product for which status is reported."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the product."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ProductStatus {
        pub fn builder() -> ProductStatusBuilder {
            ProductStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductStatusDataQualityIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fetchStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub fetch_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub severity: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueOnLandingPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value_on_landing_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "valueProvided")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value_provided: ::std::option::Option<::std::string::String>,
    }
    impl ProductStatusDataQualityIssue {
        pub fn builder() -> ProductStatusDataQualityIssueBuilder {
            ProductStatusDataQualityIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductStatusDestinationStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "approvalPending")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the approval status might change due to further processing."]
        pub approval_pending: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "approvalStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination's approval status. Acceptable values are: - \"`approved`\" - \"`disapproved`\" "]
        pub approval_status: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the destination"]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intention")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Provided for backward compatibility only. Always set to \"required\". Acceptable values are: - \"`default`\" - \"`excluded`\" - \"`optional`\" - \"`required`\" "]
        pub intention: ::std::option::Option<::std::string::String>,
    }
    impl ProductStatusDestinationStatus {
        pub fn builder() -> ProductStatusDestinationStatusBuilder {
            ProductStatusDestinationStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductStatusItemLevelIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "attributeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The attribute's name, if the issue is caused by a single attribute."]
        pub attribute_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "code")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error code of the issue."]
        pub code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short issue description in English."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The destination the issue applies to."]
        pub destination: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A detailed issue description in English."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "documentation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of a web page to help with resolving this issue."]
        pub documentation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the issue can be resolved by the merchant."]
        pub resolution: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "servability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How this issue affects serving of the offer."]
        pub servability: ::std::option::Option<::std::string::String>,
    }
    impl ProductStatusItemLevelIssue {
        pub fn builder() -> ProductStatusItemLevelIssueBuilder {
            ProductStatusItemLevelIssueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductTax {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country within which the item is taxed, specified as a CLDR territory code."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numeric ID of a location that the tax rate applies to as defined in the AdWords API."]
        pub location_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The postal code range that the tax rate applies to, represented by a ZIP code, a ZIP code prefix using * wildcard, a range between two ZIP codes or two ZIP code prefixes of equal length. Examples: 94114, 94*, 94002-95460, 94*-95*."]
        pub postal_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percentage of tax rate that applies to the item price."]
        pub rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "region")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geographic region to which the tax rate applies."]
        pub region: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxShip")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Should be set to true if tax is charged on shipping."]
        pub tax_ship: ::std::option::Option<::std::primitive::bool>,
    }
    impl ProductTax {
        pub fn builder() -> ProductTaxBuilder {
            ProductTaxBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductUnitPricingBaseMeasure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit of the denominator."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The denominator of the unit price."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl ProductUnitPricingBaseMeasure {
        pub fn builder() -> ProductUnitPricingBaseMeasureBuilder {
            ProductUnitPricingBaseMeasureBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductUnitPricingMeasure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unit of the measure."]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The measure of an item."]
        pub value: ::std::option::Option<::std::primitive::f64>,
    }
    impl ProductUnitPricingMeasure {
        pub fn builder() -> ProductUnitPricingMeasureBuilder {
            ProductUnitPricingMeasureBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductsCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductsCustomBatchRequestEntry>>,
        >,
    }
    impl ProductsCustomBatchRequest {
        pub fn builder() -> ProductsCustomBatchRequestBuilder {
            ProductsCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch products request."]
    pub struct ProductsCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`delete`\" - \"`get`\" - \"`insert`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The product to insert. Only required if the method is `insert`."]
        pub product: ::std::option::Option<::std::boxed::Box<Product>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product to get or delete. Only defined if the method is `get` or `delete`."]
        pub product_id: ::std::option::Option<::std::string::String>,
    }
    impl ProductsCustomBatchRequestEntry {
        pub fn builder() -> ProductsCustomBatchRequestEntryBuilder {
            ProductsCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductsCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductsCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productsCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl ProductsCustomBatchResponse {
        pub fn builder() -> ProductsCustomBatchResponseBuilder {
            ProductsCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch products response."]
    pub struct ProductsCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if and only if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productsCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The inserted product. Only defined if the method is `insert` and if the request was successful."]
        pub product: ::std::option::Option<::std::boxed::Box<Product>>,
    }
    impl ProductsCustomBatchResponseEntry {
        pub fn builder() -> ProductsCustomBatchResponseEntryBuilder {
            ProductsCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of products."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Product>>>,
    }
    impl ProductsListResponse {
        pub fn builder() -> ProductsListResponseBuilder {
            ProductsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductstatusesCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductstatusesCustomBatchRequestEntry>>,
        >,
    }
    impl ProductstatusesCustomBatchRequest {
        pub fn builder() -> ProductstatusesCustomBatchRequestBuilder {
            ProductstatusesCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch productstatuses request."]
    pub struct ProductstatusesCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set, only issues for the specified destinations are returned, otherwise only issues for the Shopping destination."]
        pub destinations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub include_attributes: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the product whose status to get."]
        pub product_id: ::std::option::Option<::std::string::String>,
    }
    impl ProductstatusesCustomBatchRequestEntry {
        pub fn builder() -> ProductstatusesCustomBatchRequestEntryBuilder {
            ProductstatusesCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductstatusesCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ProductstatusesCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productstatusesCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl ProductstatusesCustomBatchResponse {
        pub fn builder() -> ProductstatusesCustomBatchResponseBuilder {
            ProductstatusesCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch productstatuses response."]
    pub struct ProductstatusesCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry this entry responds to."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors, if the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#productstatusesCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requested product status. Only defined if the request was successful."]
        pub product_status: ::std::option::Option<::std::boxed::Box<ProductStatus>>,
    }
    impl ProductstatusesCustomBatchResponseEntry {
        pub fn builder() -> ProductstatusesCustomBatchResponseEntryBuilder {
            ProductstatusesCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ProductstatusesListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#productstatusesListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of products statuses."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ProductStatus>>>,
    }
    impl ProductstatusesListResponse {
        pub fn builder() -> ProductstatusesListResponseBuilder {
            ProductstatusesListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Promotion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Amount of the promotion. The values here are the promotion applied to the unit price pretax and to the total of the tax amounts."]
        pub promotion_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] ID of the promotion."]
        pub promotion_id: ::std::option::Option<::std::string::String>,
    }
    impl Promotion {
        pub fn builder() -> PromotionBuilder {
            PromotionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RateGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "applicableShippingLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of shipping labels defining the products to which this rate group applies to. This is a disjunction: only one of the labels has to match for the rate group to apply. May only be empty for the last rate group of a service. Required."]
        pub applicable_shipping_labels:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierRates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of carrier rates that can be referred to by `mainTable` or `singleValue`."]
        pub carrier_rates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CarrierRate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mainTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A table defining the rate group, when `singleValue` is not expressive enough. Can only be set if `singleValue` is not set."]
        pub main_table: ::std::option::Option<::std::boxed::Box<Table>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the rate group. Optional. If set has to be unique within shipping service."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "singleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of the rate group (e.g. flat rate $10). Can only be set if `mainTable` and `subtables` are not set."]
        pub single_value: ::std::option::Option<::std::boxed::Box<Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of subtables referred to by `mainTable`. Can only be set if `mainTable` is set."]
        pub subtables: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Table>>>,
    }
    impl RateGroup {
        pub fn builder() -> RateGroupBuilder {
            RateGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RefundReason {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the reason."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Code of the refund reason. Acceptable values are: - \"`adjustment`\" - \"`autoPostInternal`\" - \"`autoPostInvalidBillingAddress`\" - \"`autoPostNoInventory`\" - \"`autoPostPriceError`\" - \"`autoPostUndeliverableShippingAddress`\" - \"`couponAbuse`\" - \"`courtesyAdjustment`\" - \"`customerCanceled`\" - \"`customerDiscretionaryReturn`\" - \"`customerInitiatedMerchantCancel`\" - \"`customerSupportRequested`\" - \"`deliveredLateByCarrier`\" - \"`deliveredTooLate`\" - \"`expiredItem`\" - \"`failToPushOrderGoogleError`\" - \"`failToPushOrderMerchantError`\" - \"`failToPushOrderMerchantFulfillmentError`\" - \"`failToPushOrderToMerchant`\" - \"`failToPushOrderToMerchantOutOfStock`\" - \"`feeAdjustment`\" - \"`invalidCoupon`\" - \"`lateShipmentCredit`\" - \"`malformedShippingAddress`\" - \"`merchantDidNotShipOnTime`\" - \"`noInventory`\" - \"`orderTimeout`\" - \"`other`\" - \"`paymentAbuse`\" - \"`paymentDeclined`\" - \"`priceAdjustment`\" - \"`priceError`\" - \"`productArrivedDamaged`\" - \"`productNotAsDescribed`\" - \"`promoReallocation`\" - \"`qualityNotAsExpected`\" - \"`returnRefundAbuse`\" - \"`shippingCostAdjustment`\" - \"`shippingPriceError`\" - \"`taxAdjustment`\" - \"`taxError`\" - \"`undeliverableShippingAddress`\" - \"`unsupportedPoBoxAddress`\" - \"`wrongProductShipped`\" "]
        pub reason_code: ::std::option::Option<::std::string::String>,
    }
    impl RefundReason {
        pub fn builder() -> RefundReasonBuilder {
            RefundReasonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ReturnShipment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of creation of the shipment, in ISO 8601 format."]
        pub creation_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of delivery of the shipment, in ISO 8601 format."]
        pub delivery_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnMethodType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the return method. Acceptable values are: - \"`byMail`\" - \"`contactCustomerSupport`\" - \"`returnless`\" "]
        pub return_method_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipment ID generated by Google."]
        pub shipment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentTrackingInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tracking information of the shipment. One return shipment might be handled by several shipping carriers sequentially."]
        pub shipment_tracking_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShipmentTrackingInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date of shipping of the shipment, in ISO 8601 format."]
        pub shipping_date: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "State of the shipment. Acceptable values are: - \"`completed`\" - \"`new`\" - \"`shipped`\" - \"`undeliverable`\" - \"`pending`\" "]
        pub state: ::std::option::Option<::std::string::String>,
    }
    impl ReturnShipment {
        pub fn builder() -> ReturnShipmentBuilder {
            ReturnShipmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Row {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of cells that constitute the row. Must have the same length as `columnHeaders` for two-dimensional tables, a length of 1 for one-dimensional tables. Required."]
        pub cells: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Value>>>,
    }
    impl Row {
        pub fn builder() -> RowBuilder {
            RowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Service {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "active")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A boolean exposing the active status of the shipping service. Required."]
        pub active: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR code of the currency to which this service applies. Must match that of the prices in rate groups."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The CLDR territory code of the country to which the service applies. Required."]
        pub delivery_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deliveryTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time spent in various aspects from order to the delivery of the product. Required."]
        pub delivery_time: ::std::option::Option<::std::boxed::Box<DeliveryTime>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eligibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Eligibility for this service. Acceptable values are: - \"`All scenarios`\" - \"`All scenarios except Shopping Actions`\" - \"`Shopping Actions`\" "]
        pub eligibility: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumOrderValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Minimum order value for this service. If set, indicates that customers will have to spend at least this amount. All prices within a service must have the same currency. Cannot be set together with minimum_order_value_table."]
        pub minimum_order_value: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minimumOrderValueTable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table of per store minimum order values for the pickup fulfillment type. Cannot be set together with minimum_order_value."]
        pub minimum_order_value_table:
            ::std::option::Option<::std::boxed::Box<MinimumOrderValueTable>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form name of the service. Must be unique within target account. Required."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickupService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carrier-service pair delivering items to collection points. The list of supported pickup services can be retrieved via the `getSupportedPickupServices` method. Required if and only if the service delivery type is `pickup`."]
        pub pickup_service: ::std::option::Option<::std::boxed::Box<PickupCarrierService>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rateGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shipping rate group definitions. Only the last one is allowed to have an empty `applicableShippingLabels`, which means \"everything else\". The other `applicableShippingLabels` must not overlap."]
        pub rate_groups: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RateGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of locations this service ships orders to. Acceptable values are: - \"`delivery`\" - \"`pickup`\" "]
        pub shipment_type: ::std::option::Option<::std::string::String>,
    }
    impl Service {
        pub fn builder() -> ServiceBuilder {
            ServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShipmentInvoice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "invoiceSummary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Invoice summary."]
        pub invoice_summary: ::std::option::Option<::std::boxed::Box<InvoiceSummary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemInvoices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Invoice details per line item."]
        pub line_item_invoices: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ShipmentInvoiceLineItemInvoice>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] ID of the shipment group. It is assigned by the merchant in the `shipLineItems` method and is used to group multiple line items that have the same kind of shipping charges."]
        pub shipment_group_id: ::std::option::Option<::std::string::String>,
    }
    impl ShipmentInvoice {
        pub fn builder() -> ShipmentInvoiceBuilder {
            ShipmentInvoiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShipmentInvoiceLineItemInvoice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the line item. Either lineItemId or productId must be set."]
        pub line_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "productId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the product. This is the REST ID used in the products service. Either lineItemId or productId must be set."]
        pub product_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shipmentUnitIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] The shipment unit ID is assigned by the merchant and defines individual quantities within a line item. The same ID can be assigned to units that are the same while units that differ must be assigned a different ID (for example: free or promotional units)."]
        pub shipment_unit_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitInvoice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Invoice details for a single unit."]
        pub unit_invoice: ::std::option::Option<::std::boxed::Box<UnitInvoice>>,
    }
    impl ShipmentInvoiceLineItemInvoice {
        pub fn builder() -> ShipmentInvoiceLineItemInvoiceBuilder {
            ShipmentInvoiceLineItemInvoiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShipmentTrackingInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The shipping carrier that handles the package. Acceptable values are: - \"`boxtal`\" - \"`bpost`\" - \"`chronopost`\" - \"`colisPrive`\" - \"`colissimo`\" - \"`cxt`\" - \"`deliv`\" - \"`dhl`\" - \"`dpd`\" - \"`dynamex`\" - \"`eCourier`\" - \"`easypost`\" - \"`efw`\" - \"`fedex`\" - \"`fedexSmartpost`\" - \"`geodis`\" - \"`gls`\" - \"`googleCourier`\" - \"`gsx`\" - \"`jdLogistics`\" - \"`laPoste`\" - \"`lasership`\" - \"`manual`\" - \"`mpx`\" - \"`onTrac`\" - \"`other`\" - \"`tnt`\" - \"`uds`\" - \"`ups`\" - \"`usps`\" "]
        pub carrier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tracking number for the package."]
        pub tracking_number: ::std::option::Option<::std::string::String>,
    }
    impl ShipmentTrackingInfo {
        pub fn builder() -> ShipmentTrackingInfoBuilder {
            ShipmentTrackingInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The merchant account's shipping settings. All methods except getsupportedcarriers and getsupportedholidays require the admin role."]
    pub struct ShippingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account to which these account shipping settings belong. Ignored upon update, always present in get request responses."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of postal code groups that can be referred to in `services`. Optional."]
        pub postal_code_groups:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostalCodeGroup>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "services")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target account's list of services. Optional."]
        pub services: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Service>>>,
    }
    impl ShippingSettings {
        pub fn builder() -> ShippingSettingsBuilder {
            ShippingSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsCustomBatchRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request entries to be processed in the batch."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ShippingsettingsCustomBatchRequestEntry>>,
        >,
    }
    impl ShippingsettingsCustomBatchRequest {
        pub fn builder() -> ShippingsettingsCustomBatchRequestBuilder {
            ShippingsettingsCustomBatchRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch shippingsettings request."]
    pub struct ShippingsettingsCustomBatchRequestEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the account for which to get/update account shipping settings."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An entry ID, unique within the batch request."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "merchantId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the managing account."]
        pub merchant_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "method")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The method of the batch entry. Acceptable values are: - \"`get`\" - \"`update`\" "]
        pub method: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The account shipping settings to update. Only defined if the method is `update`."]
        pub shipping_settings: ::std::option::Option<::std::boxed::Box<ShippingSettings>>,
    }
    impl ShippingsettingsCustomBatchRequestEntry {
        pub fn builder() -> ShippingsettingsCustomBatchRequestEntryBuilder {
            ShippingsettingsCustomBatchRequestEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsCustomBatchResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the execution of the batch requests."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<ShippingsettingsCustomBatchResponseEntry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsCustomBatchResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl ShippingsettingsCustomBatchResponse {
        pub fn builder() -> ShippingsettingsCustomBatchResponseBuilder {
            ShippingsettingsCustomBatchResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch entry encoding a single non-batch shipping settings response."]
    pub struct ShippingsettingsCustomBatchResponseEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the request entry to which this entry responds."]
        pub batch_id: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors defined if, and only if, the request failed."]
        pub errors: ::std::option::Option<::std::boxed::Box<Errors>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#shippingsettingsCustomBatchResponseEntry`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The retrieved or updated account shipping settings."]
        pub shipping_settings: ::std::option::Option<::std::boxed::Box<ShippingSettings>>,
    }
    impl ShippingsettingsCustomBatchResponseEntry {
        pub fn builder() -> ShippingsettingsCustomBatchResponseEntryBuilder {
            ShippingsettingsCustomBatchResponseEntryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsGetSupportedCarriersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carriers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of supported carriers. May be empty."]
        pub carriers: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CarriersCarrier>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedCarriersResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl ShippingsettingsGetSupportedCarriersResponse {
        pub fn builder() -> ShippingsettingsGetSupportedCarriersResponseBuilder {
            ShippingsettingsGetSupportedCarriersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsGetSupportedHolidaysResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "holidays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of holidays applicable for delivery guarantees. May be empty."]
        pub holidays: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<HolidaysHoliday>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedHolidaysResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl ShippingsettingsGetSupportedHolidaysResponse {
        pub fn builder() -> ShippingsettingsGetSupportedHolidaysResponseBuilder {
            ShippingsettingsGetSupportedHolidaysResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsGetSupportedPickupServicesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsGetSupportedPickupServicesResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pickupServices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of supported pickup services. May be empty."]
        pub pickup_services:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PickupServicesPickupService>>>,
    }
    impl ShippingsettingsGetSupportedPickupServicesResponse {
        pub fn builder() -> ShippingsettingsGetSupportedPickupServicesResponseBuilder {
            ShippingsettingsGetSupportedPickupServicesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ShippingsettingsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"content#shippingsettingsListResponse\"."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token for the retrieval of the next page of shipping settings."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ShippingSettings>>>,
    }
    impl ShippingsettingsListResponse {
        pub fn builder() -> ShippingsettingsListResponseBuilder {
            ShippingsettingsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Table {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headers of the table's columns. Optional: if not set then the table has only one dimension."]
        pub column_headers: ::std::option::Option<::std::boxed::Box<Headers>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the table. Required for subtables, ignored for the main table."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rowHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Headers of the table's rows. Required."]
        pub row_headers: ::std::option::Option<::std::boxed::Box<Headers>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rows that constitute the table. Must have the same length as `rowHeaders`. Required."]
        pub rows: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Row>>>,
    }
    impl Table {
        pub fn builder() -> TableBuilder {
            TableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The details of the customer who placed the order."]
        pub customer: ::std::option::Option<::std::boxed::Box<TestOrderCustomer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableOrderinvoices")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the orderinvoices service should support this order."]
        pub enable_orderinvoices: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"`content#testOrder`\""]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lineItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Line items that are ordered. At least one line item must be provided."]
        pub line_items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TestOrderLineItem>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notificationMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Restricted. Do not use."]
        pub notification_mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paymentMethod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the payment method."]
        pub payment_method: ::std::option::Option<::std::boxed::Box<TestOrderPaymentMethod>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predefinedDeliveryAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Identifier of one of the predefined delivery addresses for the delivery. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
        pub predefined_delivery_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predefinedPickupDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Identifier of one of the predefined pickup details. Required for orders containing line items with shipping type `pickup`. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
        pub predefined_pickup_details: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Ignored if provided."]
        pub promotions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLegacyPromotion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingCost")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The price of shipping for all items. Shipping tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied. Note that shipping is not taxed in certain states."]
        pub shipping_cost: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingCostTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Ignored if provided."]
        pub shipping_cost_tax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The requested shipping option. Acceptable values are: - \"`economy`\" - \"`expedited`\" - \"`oneDay`\" - \"`sameDay`\" - \"`standard`\" - \"`twoDay`\" "]
        pub shipping_option: ::std::option::Option<::std::string::String>,
    }
    impl TestOrder {
        pub fn builder() -> TestOrderBuilder {
            TestOrderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrderCustomer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "email")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Email address of the customer. Acceptable values are: - \"`pog.dwight.schrute@gmail.com`\" - \"`pog.jim.halpert@gmail.com`\" - \"`penpog.pam.beesly@gmail.comding`\" "]
        pub email: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitMarketingPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Please use marketingRightsInfo instead."]
        pub explicit_marketing_preference: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fullName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Full name of the customer."]
        pub full_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "marketingRightsInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Customer's marketing preferences."]
        pub marketing_rights_info:
            ::std::option::Option<::std::boxed::Box<TestOrderCustomerMarketingRightsInfo>>,
    }
    impl TestOrderCustomer {
        pub fn builder() -> TestOrderCustomerBuilder {
            TestOrderCustomerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrderCustomerMarketingRightsInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "explicitMarketingPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last know user use selection regards marketing preferences. In certain cases selection might not be known, so this field would be empty. Acceptable values are: - \"`denied`\" - \"`granted`\" "]
        pub explicit_marketing_preference: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdatedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Timestamp when last time marketing preference was updated. Could be empty, if user wasn't offered a selection yet."]
        pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl TestOrderCustomerMarketingRightsInfo {
        pub fn builder() -> TestOrderCustomerMarketingRightsInfoBuilder {
            TestOrderCustomerMarketingRightsInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrderLineItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "product")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Product data from the time of the order placement."]
        pub product: ::std::option::Option<::std::boxed::Box<TestOrderLineItemProduct>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quantityOrdered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Number of items ordered."]
        pub quantity_ordered: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "returnInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Details of the return policy for the line item."]
        pub return_info: ::std::option::Option<::std::boxed::Box<OrderLineItemReturnInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shippingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Details of the requested shipping for the line item."]
        pub shipping_details:
            ::std::option::Option<::std::boxed::Box<OrderLineItemShippingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitTax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Ignored if provided."]
        pub unit_tax: ::std::option::Option<::std::boxed::Box<Price>>,
    }
    impl TestOrderLineItem {
        pub fn builder() -> TestOrderLineItemBuilder {
            TestOrderLineItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrderLineItemProduct {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brand")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Brand of the item."]
        pub brand: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated. Acceptable values are: - \"`online`\" "]
        pub channel: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Condition or state of the item. Acceptable values are: - \"`new`\" "]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The two-letter ISO 639-1 language code for the item. Acceptable values are: - \"`en`\" - \"`fr`\" "]
        pub content_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fees")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Fees for the item. Optional."]
        pub fees:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<OrderLineItemProductFee>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gtin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Global Trade Item Number (GTIN) of the item. Optional."]
        pub gtin: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URL of an image of the item."]
        pub image_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemGroupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Shared identifier for all variants of the same product. Optional."]
        pub item_group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mpn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Manufacturer Part Number (MPN) of the item. Optional."]
        pub mpn: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. An identifier of the item."]
        pub offer_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "price")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The price for the product. Tax is automatically calculated for orders where marketplace facilitator tax laws are applicable. Otherwise, tax settings from Merchant Center are applied."]
        pub price: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetCountry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The CLDR territory // code of the target country of the product."]
        pub target_country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the product."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variantAttributes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variant attributes for the item. Optional."]
        pub variant_attributes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<OrderLineItemProductVariantAttribute>>,
        >,
    }
    impl TestOrderLineItemProduct {
        pub fn builder() -> TestOrderLineItemProductBuilder {
            TestOrderLineItemProductBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestOrderPaymentMethod {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationMonth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The card expiration month (January = 1, February = 2 etc.)."]
        pub expiration_month: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expirationYear")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The card expiration year (4-digit, e.g. 2015)."]
        pub expiration_year: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastFourDigits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last four digits of the card number."]
        pub last_four_digits: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "predefinedBillingAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The billing address. Acceptable values are: - \"`dwight`\" - \"`jim`\" - \"`pam`\" "]
        pub predefined_billing_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of instrument. Note that real orders might have different values than the four values accepted by `createTestOrder`. Acceptable values are: - \"`AMEX`\" - \"`DISCOVER`\" - \"`MASTERCARD`\" - \"`VISA`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl TestOrderPaymentMethod {
        pub fn builder() -> TestOrderPaymentMethodBuilder {
            TestOrderPaymentMethodBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TransitTable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postalCodeGroupNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of postal group names. The last value can be `\"all other locations\"`. Example: `[\"zone 1\", \"zone 2\", \"all other locations\"]`. The referred postal code groups must match the delivery country of the service."]
        pub postal_code_group_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub rows:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TransitTableTransitTimeRow>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitTimeLabels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of transit time labels. The last value can be `\"all other labels\"`. Example: `[\"food\", \"electronics\", \"all other labels\"]`."]
        pub transit_time_labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TransitTable {
        pub fn builder() -> TransitTableBuilder {
            TransitTableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TransitTableTransitTimeRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "values")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub values: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<TransitTableTransitTimeRowTransitTimeValue>>,
        >,
    }
    impl TransitTableTransitTimeRow {
        pub fn builder() -> TransitTableTransitTimeRowBuilder {
            TransitTableTransitTimeRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TransitTableTransitTimeRowTransitTimeValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTransitTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be greater than or equal to `minTransitTimeInDays`."]
        pub max_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minTransitTimeInDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transit time range (min-max) in business days. 0 means same day delivery, 1 means next day delivery."]
        pub min_transit_time_in_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl TransitTableTransitTimeRowTransitTimeValue {
        pub fn builder() -> TransitTableTransitTimeRowTransitTimeValueBuilder {
            TransitTableTransitTimeRowTransitTimeValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UnitInvoice {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalCharges")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional charges for a unit, e.g. shipping costs."]
        pub additional_charges:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnitInvoiceAdditionalCharge>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub promotions: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitPricePretax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Price of the unit, before applying taxes."]
        pub unit_price_pretax: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unitPriceTaxes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tax amounts to apply to the unit price."]
        pub unit_price_taxes:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UnitInvoiceTaxLine>>>,
    }
    impl UnitInvoice {
        pub fn builder() -> UnitInvoiceBuilder {
            UnitInvoiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UnitInvoiceAdditionalCharge {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalChargeAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Amount of the additional charge."]
        pub additional_charge_amount: ::std::option::Option<::std::boxed::Box<Amount>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalChargePromotions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Deprecated."]
        pub additional_charge_promotions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Promotion>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Type of the additional charge. Acceptable values are: - \"`shipping`\" "]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl UnitInvoiceAdditionalCharge {
        pub fn builder() -> UnitInvoiceAdditionalChargeBuilder {
            UnitInvoiceAdditionalChargeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct UnitInvoiceTaxLine {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxAmount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Tax amount for the tax type."]
        pub tax_amount: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional name of the tax type. This should only be provided if `taxType` is `otherFeeTax`."]
        pub tax_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "[required] Type of the tax. Acceptable values are: - \"`otherFee`\" - \"`otherFeeTax`\" - \"`sales`\" "]
        pub tax_type: ::std::option::Option<::std::string::String>,
    }
    impl UnitInvoiceTaxLine {
        pub fn builder() -> UnitInvoiceTaxLineBuilder {
            UnitInvoiceTaxLineBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The single value of a rate group or the value of a rate group table's cell. Exactly one of `noShipping`, `flatRate`, `pricePercentage`, `carrierRateName`, `subtableName` must be set."]
    pub struct Value {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carrierRateName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of a carrier rate referring to a carrier rate defined in the same rate group. Can only be set if all other fields are not set."]
        pub carrier_rate_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flatRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A flat rate. Can only be set if all other fields are not set."]
        pub flat_rate: ::std::option::Option<::std::boxed::Box<Price>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "noShipping")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, then the product can't ship. Must be true when set, can only be set if all other fields are not set."]
        pub no_shipping: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pricePercentage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A percentage of the price represented as a number in decimal notation (e.g., `\"5.4\"`). Can only be set if all other fields are not set."]
        pub price_percentage: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtableName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of a subtable. Can only be set in table cells (i.e., not for single values), and only if all other fields are not set."]
        pub subtable_name: ::std::option::Option<::std::string::String>,
    }
    impl Value {
        pub fn builder() -> ValueBuilder {
            ValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Weight {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The weight unit. Acceptable values are: - \"`kg`\" - \"`lb`\" "]
        pub unit: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The weight represented as a number."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Weight {
        pub fn builder() -> WeightBuilder {
            WeightBuilder::default()
        }
    }
}
