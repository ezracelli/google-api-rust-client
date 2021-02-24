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
    pub mod blog_user_infos {
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
                    #[serde(rename = "maxPosts")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_posts: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod blogs {
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
                    #[serde(rename = "maxPosts")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_posts: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod get_by_url {
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
                    #[serde(rename = "url")]
                    pub url: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod list_by_user {
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
                    #[serde(rename = "fetchUserInfo")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_user_info: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "role")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub role: ::std::option::Option<QueryParametersRoleEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Default value of status is LIVE."]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersRoleEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersRoleEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Default value of status is LIVE."]
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "DELETED")]
                    #[doc = ""]
                    Deleted,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
        }
    }
    pub mod comments {
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
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
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
                    #[serde(rename = "endDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub end_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchBodies")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_bodies: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "EMPTIED")]
                    #[doc = ""]
                    Emptied,
                    #[serde(rename = "PENDING")]
                    #[doc = ""]
                    Pending,
                    #[serde(rename = "SPAM")]
                    #[doc = ""]
                    Spam,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod list_by_blog {
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
                    #[serde(rename = "endDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub end_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchBodies")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_bodies: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "EMPTIED")]
                    #[doc = ""]
                    Emptied,
                    #[serde(rename = "PENDING")]
                    #[doc = ""]
                    Pending,
                    #[serde(rename = "SPAM")]
                    #[doc = ""]
                    Spam,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
            }
        }
    }
    pub mod page_views {
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
                    #[serde(rename = "range")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub range: ::std::option::Option<QueryParametersRangeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersRangeEnum {
                    #[serde(rename = "all")]
                    #[doc = ""]
                    All,
                    #[serde(rename = "30DAYS")]
                    #[doc = ""]
                    _30Days,
                    #[serde(rename = "7DAYS")]
                    #[doc = ""]
                    _7Days,
                }
                impl ::std::default::Default for QueryParametersRangeEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
        }
    }
    pub mod pages {
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
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
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
                    #[serde(rename = "isDraft")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub is_draft: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "fetchBodies")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_bodies: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "DRAFT")]
                    #[doc = ""]
                    Draft,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod patch {
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
                    #[serde(rename = "publish")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub publish: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revert")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub revert: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "publish")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub publish: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revert")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub revert: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod post_user_infos {
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
                    #[serde(rename = "maxComments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_comments: ::std::option::Option<::std::primitive::i64>,
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
                    #[serde(rename = "endDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub end_date: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: fetch_bodies () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBodies")]
                    #[serde(default = "query_parameters_defaults :: fetch_bodies")]
                    pub fetch_bodies: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "labels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub labels: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(
                        default = "{ query_parameters_defaults :: order_by () }",
                        setter(into)
                    )]
                    #[serde(rename = "orderBy")]
                    #[serde(default = "query_parameters_defaults :: order_by")]
                    pub order_by: QueryParametersOrderByEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_bodies() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                    pub fn order_by() -> super::QueryParametersOrderByEnum {
                        serde_json::from_str(&"\"PUBLISHED\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "ORDER_BY_UNSPECIFIED")]
                    #[doc = ""]
                    OrderByUnspecified,
                    #[serde(rename = "PUBLISHED")]
                    #[doc = ""]
                    Published,
                    #[serde(rename = "UPDATED")]
                    #[doc = ""]
                    Updated,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::Published
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "DRAFT")]
                    #[doc = ""]
                    Draft,
                    #[serde(rename = "SCHEDULED")]
                    #[doc = ""]
                    Scheduled,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
        }
    }
    pub mod posts {
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
                        default = "{ query_parameters_defaults :: fetch_body () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBody")]
                    #[serde(default = "query_parameters_defaults :: fetch_body")]
                    pub fetch_body: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchImages")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_images: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxComments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_comments: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_body() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod get_by_path {
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
                    #[serde(rename = "maxComments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_comments: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "path")]
                    pub path: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
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
                    #[builder(
                        default = "{ query_parameters_defaults :: fetch_body () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBody")]
                    #[serde(default = "query_parameters_defaults :: fetch_body")]
                    pub fetch_body: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchImages")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_images: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "isDraft")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub is_draft: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_body() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
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
                    #[serde(rename = "endDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub end_date: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: fetch_bodies () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBodies")]
                    #[serde(default = "query_parameters_defaults :: fetch_bodies")]
                    pub fetch_bodies: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchImages")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_images: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "labels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub labels: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(
                        default = "{ query_parameters_defaults :: order_by () }",
                        setter(into)
                    )]
                    #[serde(rename = "orderBy")]
                    #[serde(default = "query_parameters_defaults :: order_by")]
                    pub order_by: QueryParametersOrderByEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "startDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub start_date: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "status")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub status: ::std::option::Option<QueryParametersStatusEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "view")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub view: ::std::option::Option<QueryParametersViewEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_bodies() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn order_by() -> super::QueryParametersOrderByEnum {
                        serde_json::from_str(&"\"PUBLISHED\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "ORDER_BY_UNSPECIFIED")]
                    #[doc = ""]
                    OrderByUnspecified,
                    #[serde(rename = "PUBLISHED")]
                    #[doc = ""]
                    Published,
                    #[serde(rename = "UPDATED")]
                    #[doc = ""]
                    Updated,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::Published
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersStatusEnum {
                    #[serde(rename = "LIVE")]
                    #[doc = ""]
                    Live,
                    #[serde(rename = "DRAFT")]
                    #[doc = ""]
                    Draft,
                    #[serde(rename = "SCHEDULED")]
                    #[doc = ""]
                    Scheduled,
                }
                impl ::std::default::Default for QueryParametersStatusEnum {
                    fn default() -> Self {
                        Self::Live
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersViewEnum {
                    #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
                    #[doc = ""]
                    ViewTypeUnspecified,
                    #[serde(rename = "READER")]
                    #[doc = ""]
                    Reader,
                    #[serde(rename = "AUTHOR")]
                    #[doc = ""]
                    Author,
                    #[serde(rename = "ADMIN")]
                    #[doc = ""]
                    Admin,
                }
                impl ::std::default::Default for QueryParametersViewEnum {
                    fn default() -> Self {
                        Self::ViewTypeUnspecified
                    }
                }
            }
            pub mod patch {
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
                        default = "{ query_parameters_defaults :: fetch_body () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBody")]
                    #[serde(default = "query_parameters_defaults :: fetch_body")]
                    pub fetch_body: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchImages")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_images: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxComments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_comments: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publish")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub publish: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revert")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub revert: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_body() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                }
            }
            pub mod publish {
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
                    #[serde(rename = "publishDate")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub publish_date: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod search {
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
                        default = "{ query_parameters_defaults :: fetch_bodies () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBodies")]
                    #[serde(default = "query_parameters_defaults :: fetch_bodies")]
                    pub fetch_bodies: ::std::primitive::bool,
                    #[builder(
                        default = "{ query_parameters_defaults :: order_by () }",
                        setter(into)
                    )]
                    #[serde(rename = "orderBy")]
                    #[serde(default = "query_parameters_defaults :: order_by")]
                    pub order_by: QueryParametersOrderByEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    pub q: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_bodies() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
                    }
                    pub fn order_by() -> super::QueryParametersOrderByEnum {
                        serde_json::from_str(&"\"PUBLISHED\"").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersOrderByEnum {
                    #[serde(rename = "ORDER_BY_UNSPECIFIED")]
                    #[doc = ""]
                    OrderByUnspecified,
                    #[serde(rename = "PUBLISHED")]
                    #[doc = ""]
                    Published,
                    #[serde(rename = "UPDATED")]
                    #[doc = ""]
                    Updated,
                }
                impl ::std::default::Default for QueryParametersOrderByEnum {
                    fn default() -> Self {
                        Self::Published
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
                    #[builder(
                        default = "{ query_parameters_defaults :: fetch_body () }",
                        setter(into)
                    )]
                    #[serde(rename = "fetchBody")]
                    #[serde(default = "query_parameters_defaults :: fetch_body")]
                    pub fetch_body: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "fetchImages")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub fetch_images: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxComments")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_comments: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publish")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub publish: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "revert")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub revert: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn fetch_body() -> ::std::primitive::bool {
                        serde_json::from_str(&"true").unwrap()
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
    pub struct Blog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customMetaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The JSON custom meta-data for the Blog."]
        pub custom_meta_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this blog. This is displayed underneath the title."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this resource."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entry. Always blogger#blog."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The locale this Blog is set to."]
        pub locale: ::std::option::Option<BlogLocale>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of this blog. This is displayed as the title."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container of pages in this blog."]
        pub pages: ::std::option::Option<BlogPages>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "posts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container of posts in this blog."]
        pub posts: ::std::option::Option<BlogPosts>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this blog was published."]
        pub published: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API REST URL to fetch this resource from."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the blog."]
        pub status: ::std::option::Option<BlogStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this blog was last updated."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL where this blog is published."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Blog {
        pub fn builder() -> BlogBuilder {
            BlogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The locale this Blog is set to."]
    pub struct BlogLocale {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country this blog's locale is set to."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language this blog is authored in."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language variant this blog is authored in."]
        pub variant: ::std::option::Option<::std::string::String>,
    }
    impl BlogLocale {
        pub fn builder() -> BlogLocaleBuilder {
            BlogLocaleBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The container of pages in this blog."]
    pub struct BlogPages {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the container for pages in this blog."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of pages in this blog."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl BlogPages {
        pub fn builder() -> BlogPagesBuilder {
            BlogPagesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The container of posts in this blog."]
    pub struct BlogPosts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The List of Posts for this Blog."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Post>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the container for posts in this blog."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of posts in this blog."]
        pub total_items: ::std::option::Option<::std::primitive::i64>,
    }
    impl BlogPosts {
        pub fn builder() -> BlogPostsBuilder {
            BlogPostsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the blog."]
    pub enum BlogStatusEnum {
        #[serde(rename = "LIVE")]
        #[doc = ""]
        Live,
        #[serde(rename = "DELETED")]
        #[doc = ""]
        Deleted,
    }
    impl ::std::default::Default for BlogStatusEnum {
        fn default() -> Self {
            Self::Live
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BlogList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blogUserInfos")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Admin level list of blog per-user information."]
        pub blog_user_infos:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BlogUserInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Blogs this user has Authorship or Admin rights over."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Blog>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#blogList."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl BlogList {
        pub fn builder() -> BlogListBuilder {
            BlogListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BlogPerUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blogId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the Blog resource."]
        pub blog_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasAdminAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the user has Admin level access to the blog."]
        pub has_admin_access: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#blogPerUserInfo."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "photosAlbumKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Photo Album Key for the user when adding photos to the blog."]
        pub photos_album_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER)."]
        pub role: ::std::option::Option<BlogPerUserInfoRoleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the User."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl BlogPerUserInfo {
        pub fn builder() -> BlogPerUserInfoBuilder {
            BlogPerUserInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Access permissions that the user has for the blog (ADMIN, AUTHOR, or READER)."]
    pub enum BlogPerUserInfoRoleEnum {
        #[serde(rename = "VIEW_TYPE_UNSPECIFIED")]
        #[doc = ""]
        ViewTypeUnspecified,
        #[serde(rename = "READER")]
        #[doc = ""]
        Reader,
        #[serde(rename = "AUTHOR")]
        #[doc = ""]
        Author,
        #[serde(rename = "ADMIN")]
        #[doc = ""]
        Admin,
    }
    impl ::std::default::Default for BlogPerUserInfoRoleEnum {
        fn default() -> Self {
            Self::ViewTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct BlogUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Blog resource."]
        pub blog: ::std::option::Option<::std::boxed::Box<Blog>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blog_user_info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a User for the Blog."]
        pub blog_user_info: ::std::option::Option<::std::boxed::Box<BlogPerUserInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#blogUserInfo."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl BlogUserInfo {
        pub fn builder() -> BlogUserInfoBuilder {
            BlogUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Comment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of this Comment."]
        pub author: ::std::option::Option<CommentAuthor>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the blog containing this comment."]
        pub blog: ::std::option::Option<CommentBlog>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The actual content of the comment. May include HTML markup."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this resource."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inReplyTo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the comment this is in reply to."]
        pub in_reply_to: ::std::option::Option<CommentInReplyTo>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entry. Always blogger#comment."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "post")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the post containing this comment."]
        pub post: ::std::option::Option<CommentPost>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this comment was published."]
        pub published: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API REST URL to fetch this resource from."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the comment (only populated for admin users)."]
        pub status: ::std::option::Option<CommentStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this comment was last updated."]
        pub updated: ::std::option::Option<::std::string::String>,
    }
    impl Comment {
        pub fn builder() -> CommentBuilder {
            CommentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The author of this Comment."]
    pub struct CommentAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the creator."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar."]
        pub image: ::std::option::Option<CommentAuthorImage>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the creator's Profile page."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl CommentAuthor {
        pub fn builder() -> CommentAuthorBuilder {
            CommentAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The creator's avatar."]
    pub struct CommentAuthorImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl CommentAuthorImage {
        pub fn builder() -> CommentAuthorImageBuilder {
            CommentAuthorImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about the blog containing this comment."]
    pub struct CommentBlog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the blog containing this comment."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl CommentBlog {
        pub fn builder() -> CommentBlogBuilder {
            CommentBlogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about the comment this is in reply to."]
    pub struct CommentInReplyTo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identified of the parent of this comment."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl CommentInReplyTo {
        pub fn builder() -> CommentInReplyToBuilder {
            CommentInReplyToBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about the post containing this comment."]
    pub struct CommentPost {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the post containing this comment."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl CommentPost {
        pub fn builder() -> CommentPostBuilder {
            CommentPostBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the comment (only populated for admin users)."]
    pub enum CommentStatusEnum {
        #[serde(rename = "LIVE")]
        #[doc = ""]
        Live,
        #[serde(rename = "EMPTIED")]
        #[doc = ""]
        Emptied,
        #[serde(rename = "PENDING")]
        #[doc = ""]
        Pending,
        #[serde(rename = "SPAM")]
        #[doc = ""]
        Spam,
    }
    impl ::std::default::Default for CommentStatusEnum {
        fn default() -> Self {
            Self::Live
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CommentList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the response."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The List of Comments for a Post."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entry. Always blogger#commentList."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the next page, if one exists."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the previous page, if one exists."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CommentList {
        pub fn builder() -> CommentListBuilder {
            CommentListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of this Page."]
        pub author: ::std::option::Option<PageAuthor>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the blog containing this Page."]
        pub blog: ::std::option::Option<PageBlog>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The body content of this Page, in HTML."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this resource."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#page."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this Page was published."]
        pub published: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API REST URL to fetch this resource from."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the page for admin resources (either LIVE or DRAFT)."]
        pub status: ::std::option::Option<PageStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this entity. This is the name displayed in the Admin user interface."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this Page was last updated."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL that this Page is displayed at."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Page {
        pub fn builder() -> PageBuilder {
            PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The author of this Page."]
    pub struct PageAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the creator."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar."]
        pub image: ::std::option::Option<PageAuthorImage>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the creator's Profile page."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PageAuthor {
        pub fn builder() -> PageAuthorBuilder {
            PageAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The creator's avatar."]
    pub struct PageAuthorImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PageAuthorImage {
        pub fn builder() -> PageAuthorImageBuilder {
            PageAuthorImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about the blog containing this Page."]
    pub struct PageBlog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the blog containing this page."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl PageBlog {
        pub fn builder() -> PageBlogBuilder {
            PageBlogBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the page for admin resources (either LIVE or DRAFT)."]
    pub enum PageStatusEnum {
        #[serde(rename = "LIVE")]
        #[doc = ""]
        Live,
        #[serde(rename = "DRAFT")]
        #[doc = ""]
        Draft,
    }
    impl ::std::default::Default for PageStatusEnum {
        fn default() -> Self {
            Self::Live
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PageList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the response."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Pages for a Blog."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Page>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#pageList."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the next page, if one exists."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PageList {
        pub fn builder() -> PageListBuilder {
            PageListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Pageviews {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blogId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blog Id."]
        pub blog_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "counts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container of posts in this blog."]
        pub counts: ::std::option::Option<::std::vec::Vec<PageviewsCounts>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entry. Always blogger#page_views."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Pageviews {
        pub fn builder() -> PageviewsBuilder {
            PageviewsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PageviewsCounts {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count of page views for the given time range."]
        pub count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeRange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time range the given count applies to."]
        pub time_range: ::std::option::Option<PageviewsCountsTimeRangeEnum>,
    }
    impl PageviewsCounts {
        pub fn builder() -> PageviewsCountsBuilder {
            PageviewsCountsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Time range the given count applies to."]
    pub enum PageviewsCountsTimeRangeEnum {
        #[serde(rename = "ALL_TIME")]
        #[doc = ""]
        AllTime,
        #[serde(rename = "THIRTY_DAYS")]
        #[doc = ""]
        ThirtyDays,
        #[serde(rename = "SEVEN_DAYS")]
        #[doc = ""]
        SevenDays,
    }
    impl ::std::default::Default for PageviewsCountsTimeRangeEnum {
        fn default() -> Self {
            Self::AllTime
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Post {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of this Post."]
        pub author: ::std::option::Option<PostAuthor>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blog")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about the blog containing this Post."]
        pub blog: ::std::option::Option<PostBlog>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content of the Post. May contain HTML markup."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customMetaData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The JSON meta-data for the Post."]
        pub custom_meta_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of this Post."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "images")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display image for the Post."]
        pub images: ::std::option::Option<::std::vec::Vec<PostImages>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#post."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of labels this Post was tagged with."]
        pub labels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The location for geotagged posts."]
        pub location: ::std::option::Option<PostLocation>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "published")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this Post was published."]
        pub published: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "readerComments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Comment control and display setting for readers of this post."]
        pub reader_comments: ::std::option::Option<PostReaderCommentsEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container of comments on this Post."]
        pub replies: ::std::option::Option<PostReplies>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API REST URL to fetch this resource from."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Status of the post. Only set for admin-level requests."]
        pub status: ::std::option::Option<PostStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the Post."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "titleLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title link URL, similar to atom's related link."]
        pub title_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "RFC 3339 date-time when this Post was last updated."]
        pub updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL where this Post is displayed."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Post {
        pub fn builder() -> PostBuilder {
            PostBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The author of this Post."]
    pub struct PostAuthor {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the creator."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar."]
        pub image: ::std::option::Option<PostAuthorImage>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the creator's Profile page."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PostAuthor {
        pub fn builder() -> PostAuthorBuilder {
            PostAuthorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The creator's avatar."]
    pub struct PostAuthorImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creator's avatar URL."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PostAuthorImage {
        pub fn builder() -> PostAuthorImageBuilder {
            PostAuthorImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data about the blog containing this Post."]
    pub struct PostBlog {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier of the Blog that contains this Post."]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl PostBlog {
        pub fn builder() -> PostBlogBuilder {
            PostBlogBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostImages {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl PostImages {
        pub fn builder() -> PostImagesBuilder {
            PostImagesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The location for geotagged posts."]
    pub struct PostLocation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location's latitude."]
        pub lat: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lng")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location's longitude."]
        pub lng: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "span")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Location's viewport span. Can be used when rendering a map preview."]
        pub span: ::std::option::Option<::std::string::String>,
    }
    impl PostLocation {
        pub fn builder() -> PostLocationBuilder {
            PostLocationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Comment control and display setting for readers of this post."]
    pub enum PostReaderCommentsEnum {
        #[serde(rename = "ALLOW")]
        #[doc = ""]
        Allow,
        #[serde(rename = "DONT_ALLOW_SHOW_EXISTING")]
        #[doc = ""]
        DontAllowShowExisting,
        #[serde(rename = "DONT_ALLOW_HIDE_EXISTING")]
        #[doc = ""]
        DontAllowHideExisting,
    }
    impl ::std::default::Default for PostReaderCommentsEnum {
        fn default() -> Self {
            Self::Allow
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The container of comments on this Post."]
    pub struct PostReplies {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The List of Comments for this Post."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the comments on this post."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItems")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The count of comments on this post."]
        pub total_items: ::std::option::Option<::std::string::String>,
    }
    impl PostReplies {
        pub fn builder() -> PostRepliesBuilder {
            PostRepliesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Status of the post. Only set for admin-level requests."]
    pub enum PostStatusEnum {
        #[serde(rename = "LIVE")]
        #[doc = ""]
        Live,
        #[serde(rename = "DRAFT")]
        #[doc = ""]
        Draft,
        #[serde(rename = "SCHEDULED")]
        #[doc = ""]
        Scheduled,
    }
    impl ::std::default::Default for PostStatusEnum {
        fn default() -> Self {
            Self::Live
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of the response."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Posts for this Blog."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Post>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#postList."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the next page, if one exists."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the previous page, if one exists."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PostList {
        pub fn builder() -> PostListBuilder {
            PostListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostPerUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blogId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the Blog that the post resource belongs to."]
        pub blog_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasEditAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if the user has Author level access to the post."]
        pub has_edit_access: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#postPerUserInfo."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the Post resource."]
        pub post_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the User."]
        pub user_id: ::std::option::Option<::std::string::String>,
    }
    impl PostPerUserInfo {
        pub fn builder() -> PostPerUserInfoBuilder {
            PostPerUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostUserInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#postUserInfo."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "post")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Post resource."]
        pub post: ::std::option::Option<::std::boxed::Box<Post>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "post_user_info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about a User for the Post."]
        pub post_user_info: ::std::option::Option<::std::boxed::Box<PostPerUserInfo>>,
    }
    impl PostUserInfo {
        pub fn builder() -> PostUserInfoBuilder {
            PostUserInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PostUserInfosList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Posts with User information for the post, for this Blog."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PostUserInfo>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#postList."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination token to fetch the next page, if one exists."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PostUserInfosList {
        pub fn builder() -> PostUserInfosListBuilder {
            PostUserInfosListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct User {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "about")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Profile summary information."]
        pub about: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blogs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container of blogs for this user."]
        pub blogs: ::std::option::Option<UserBlogs>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "created")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of when this profile was created, in seconds since epoch."]
        pub created: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The identifier for this User."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of this entity. Always blogger#user."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locale")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This user's locale"]
        pub locale: ::std::option::Option<UserLocale>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The API REST URL to fetch this resource from."]
        pub self_link: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's profile page."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl User {
        pub fn builder() -> UserBuilder {
            UserBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The container of blogs for this user."]
    pub struct UserBlogs {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the Blogs for this user."]
        pub self_link: ::std::option::Option<::std::string::String>,
    }
    impl UserBlogs {
        pub fn builder() -> UserBlogsBuilder {
            UserBlogsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This user's locale"]
    pub struct UserLocale {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country this blog's locale is set to."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language this blog is authored in."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language variant this blog is authored in."]
        pub variant: ::std::option::Option<::std::string::String>,
    }
    impl UserLocale {
        pub fn builder() -> UserLocaleBuilder {
            UserLocaleBuilder::default()
        }
    }
}
