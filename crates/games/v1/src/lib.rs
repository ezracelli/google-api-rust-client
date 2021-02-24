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
    pub mod achievement_definitions {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified `maxResults`."]
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
    pub mod achievements {
        pub mod methods {
            pub mod increment {
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
                    #[serde(rename = "requestId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "A randomly generated numeric ID for each request specified by the caller. This number is used at the server to ensure that the request is handled correctly across retries."]
                    pub request_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "stepsToIncrement")]
                    #[doc = "The number of steps to increment."]
                    pub steps_to_increment: ::std::primitive::i64,
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of achievement resources to return in the response, used for paging. For any response, the actual number of achievement resources returned may be less than the specified `maxResults`."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "state")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Tells the server to return only achievements with the specified state. If this parameter isn't specified, all achievements are returned."]
                    pub state: ::std::option::Option<QueryParametersStateEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Tells the server to return only achievements with the specified state. If this parameter isn't specified, all achievements are returned."]
                pub enum QueryParametersStateEnum {
                    #[serde(rename = "ALL")]
                    #[doc = "List all achievements. This is the default."]
                    All,
                    #[serde(rename = "HIDDEN")]
                    #[doc = "List only hidden achievements."]
                    Hidden,
                    #[serde(rename = "REVEALED")]
                    #[doc = "List only revealed achievements."]
                    Revealed,
                    #[serde(rename = "UNLOCKED")]
                    #[doc = "List only unlocked achievements."]
                    Unlocked,
                }
                impl ::std::default::Default for QueryParametersStateEnum {
                    fn default() -> Self {
                        Self::All
                    }
                }
            }
            pub mod set_steps_at_least {
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
                    #[serde(rename = "steps")]
                    #[doc = "The minimum value to set the steps to."]
                    pub steps: ::std::primitive::i64,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod applications {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "platformType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict application details returned to the specific platform."]
                    pub platform_type: ::std::option::Option<QueryParametersPlatformTypeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Restrict application details returned to the specific platform."]
                pub enum QueryParametersPlatformTypeEnum {
                    #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
                    #[doc = "Default value, don't use."]
                    PlatformTypeUnspecified,
                    #[serde(rename = "ANDROID")]
                    #[doc = "Retrieve applications that can be played on Android."]
                    Android,
                    #[serde(rename = "IOS")]
                    #[doc = "Retrieve applications that can be played on iOS."]
                    Ios,
                    #[serde(rename = "WEB_APP")]
                    #[doc = "Retrieve applications that can be played on desktop web."]
                    WebApp,
                }
                impl ::std::default::Default for QueryParametersPlatformTypeEnum {
                    fn default() -> Self {
                        Self::PlatformTypeUnspecified
                    }
                }
            }
            pub mod get_end_point {
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
                    #[serde(rename = "applicationId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The application ID from the Google Play developer console."]
                    pub application_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "endPointType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Type of endpoint being requested."]
                    pub end_point_type: ::std::option::Option<QueryParametersEndPointTypeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Type of endpoint being requested."]
                pub enum QueryParametersEndPointTypeEnum {
                    #[serde(rename = "END_POINT_TYPE_UNSPECIFIED")]
                    #[doc = "Default value. This value is unused."]
                    EndPointTypeUnspecified,
                    #[serde(rename = "PROFILE_CREATION")]
                    #[doc = "Request a URL to create a new profile."]
                    ProfileCreation,
                    #[serde(rename = "PROFILE_SETTINGS")]
                    #[doc = "Request a URL for the Settings view."]
                    ProfileSettings,
                }
                impl ::std::default::Default for QueryParametersEndPointTypeEnum {
                    fn default() -> Self {
                        Self::EndPointTypeUnspecified
                    }
                }
            }
        }
    }
    pub mod events {
        pub mod methods {
            pub mod list_by_player {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of events to return in the response, used for paging. For any response, the actual number of events to return may be less than the specified maxResults."]
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
            pub mod list_definitions {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of event definitions to return in the response, used for paging. For any response, the actual number of event definitions to return may be less than the specified `maxResults`."]
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
            pub mod record {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod leaderboards {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of leaderboards to return in the response. For any response, the actual number of leaderboards returned may be less than the specified `maxResults`."]
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
    pub mod metagame {
        pub mod methods {
            pub mod list_categories_by_player {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of category resources to return in the response, used for paging. For any response, the actual number of category resources returned may be less than the specified `maxResults`."]
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
    pub mod players {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of player resources to return in the response, used for paging. For any response, the actual number of player resources returned may be less than the specified `maxResults`."]
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
    pub mod revisions {
        pub mod methods {
            pub mod check {
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
                    #[serde(rename = "clientRevision")]
                    #[doc = "The revision of the client SDK used by your application. Format: `[PLATFORM_TYPE]:[VERSION_NUMBER]`. Possible values of `PLATFORM_TYPE` are: * `ANDROID` - Client is running the Android SDK. * `IOS` - Client is running the iOS SDK. * `WEB_APP` - Client is running as a Web App."]
                    pub client_revision: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod scores {
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
                    #[serde(rename = "includeRankType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The types of ranks to return. If the parameter is omitted, no ranks will be returned."]
                    pub include_rank_type:
                        ::std::option::Option<QueryParametersIncludeRankTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
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
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The types of ranks to return. If the parameter is omitted, no ranks will be returned."]
                pub enum QueryParametersIncludeRankTypeEnum {
                    #[serde(rename = "INCLUDE_RANK_TYPE_UNSPECIFIED")]
                    #[doc = "Default value. Should be unused."]
                    IncludeRankTypeUnspecified,
                    #[serde(rename = "ALL")]
                    #[doc = "Retrieve all supported ranks. In HTTP, this parameter value can also be specified as `ALL`."]
                    All,
                    #[serde(rename = "PUBLIC")]
                    #[doc = "Retrieve public ranks, if the player is sharing their gameplay activity publicly."]
                    Public,
                    #[serde(rename = "SOCIAL")]
                    #[doc = "(Obsolete) Retrieve the social rank."]
                    Social,
                    #[serde(rename = "FRIENDS")]
                    #[doc = "Retrieve the rank on the friends collection."]
                    Friends,
                }
                impl ::std::default::Default for QueryParametersIncludeRankTypeEnum {
                    fn default() -> Self {
                        Self::IncludeRankTypeUnspecified
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeSpan")]
                    #[doc = "The time span for the scores and ranks you're requesting."]
                    pub time_span: QueryParametersTimeSpanEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The time span for the scores and ranks you're requesting."]
                pub enum QueryParametersTimeSpanEnum {
                    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
                    #[doc = "Default value. This value is unused."]
                    ScoreTimeSpanUnspecified,
                    #[serde(rename = "ALL_TIME")]
                    #[doc = "The score is an all-time score."]
                    AllTime,
                    #[serde(rename = "WEEKLY")]
                    #[doc = "The score is a weekly score."]
                    Weekly,
                    #[serde(rename = "DAILY")]
                    #[doc = "The score is a daily score."]
                    Daily,
                }
                impl ::std::default::Default for QueryParametersTimeSpanEnum {
                    fn default() -> Self {
                        Self::ScoreTimeSpanUnspecified
                    }
                }
            }
            pub mod list_window {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of leaderboard scores to return in the response. For any response, the actual number of leaderboard scores returned may be less than the specified `maxResults`."]
                    pub max_results: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The token returned by the previous request."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "resultsAbove")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred number of scores to return above the player's score. More scores may be returned if the player is at the bottom of the leaderboard; fewer may be returned if the player is at the top. Must be less than or equal to maxResults."]
                    pub results_above: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "returnTopIfAbsent")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "True if the top scores should be returned when the player is not in the leaderboard. Defaults to true."]
                    pub return_top_if_absent: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "timeSpan")]
                    #[doc = "The time span for the scores and ranks you're requesting."]
                    pub time_span: QueryParametersTimeSpanEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The time span for the scores and ranks you're requesting."]
                pub enum QueryParametersTimeSpanEnum {
                    #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
                    #[doc = "Default value. This value is unused."]
                    ScoreTimeSpanUnspecified,
                    #[serde(rename = "ALL_TIME")]
                    #[doc = "The score is an all-time score."]
                    AllTime,
                    #[serde(rename = "WEEKLY")]
                    #[doc = "The score is a weekly score."]
                    Weekly,
                    #[serde(rename = "DAILY")]
                    #[doc = "The score is a daily score."]
                    Daily,
                }
                impl ::std::default::Default for QueryParametersTimeSpanEnum {
                    fn default() -> Self {
                        Self::ScoreTimeSpanUnspecified
                    }
                }
            }
            pub mod submit {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "score")]
                    #[doc = "The score you're submitting. The submitted score is ignored if it is worse than a previously submitted score, where worse depends on the leaderboard sort order. The meaning of the score value depends on the leaderboard format type. For fixed-point, the score represents the raw value. For time, the score represents elapsed time in milliseconds. For currency, the score represents a value in micro units."]
                    pub score: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "scoreTag")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Additional information about the score you're submitting. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
                    pub score_tag: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod submit_multiple {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod snapshots {
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
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
                    #[serde(rename = "language")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The preferred language to use for strings returned by this method."]
                    pub language: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxResults")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The maximum number of snapshot resources to return in the response, used for paging. For any response, the actual number of snapshot resources returned may be less than the specified `maxResults`."]
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
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement definition object."]
    pub struct AchievementDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievementType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the achievement."]
        pub achievement_type: ::std::option::Option<AchievementDefinitionAchievementTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the achievement."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experience points which will be earned when unlocking this achievement."]
        pub experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedTotalSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total steps for an incremental achievement as a string."]
        pub formatted_total_steps: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the achievement."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The initial state of the achievement."]
        pub initial_state: ::std::option::Option<AchievementDefinitionInitialStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRevealedIconUrlDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the revealed icon image being returned is a default image, or is provided by the game."]
        pub is_revealed_icon_url_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isUnlockedIconUrlDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the unlocked icon image being returned is a default image, or is game-provided."]
        pub is_unlocked_icon_url_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinition`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the achievement."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revealedIconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image URL for the revealed achievement icon."]
        pub revealed_icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total steps for an incremental achievement."]
        pub total_steps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unlockedIconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image URL for the unlocked achievement icon."]
        pub unlocked_icon_url: ::std::option::Option<::std::string::String>,
    }
    impl AchievementDefinition {
        pub fn builder() -> AchievementDefinitionBuilder {
            AchievementDefinitionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the achievement."]
    pub enum AchievementDefinitionAchievementTypeEnum {
        #[serde(rename = "ACHIEVEMENT_TYPE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        AchievementTypeUnspecified,
        #[serde(rename = "STANDARD")]
        #[doc = "Achievement is either locked or unlocked."]
        Standard,
        #[serde(rename = "INCREMENTAL")]
        #[doc = "Achievement is incremental."]
        Incremental,
    }
    impl ::std::default::Default for AchievementDefinitionAchievementTypeEnum {
        fn default() -> Self {
            Self::AchievementTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The initial state of the achievement."]
    pub enum AchievementDefinitionInitialStateEnum {
        #[serde(rename = "INITIAL_ACHIEVEMENT_STATE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        InitialAchievementStateUnspecified,
        #[serde(rename = "HIDDEN")]
        #[doc = "Achievement is hidden."]
        Hidden,
        #[serde(rename = "REVEALED")]
        #[doc = "Achievement is revealed."]
        Revealed,
        #[serde(rename = "UNLOCKED")]
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl ::std::default::Default for AchievementDefinitionInitialStateEnum {
        fn default() -> Self {
            Self::InitialAchievementStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of achievement definition objects."]
    pub struct AchievementDefinitionsListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievement definitions."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementDefinition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinitionsListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl AchievementDefinitionsListResponse {
        pub fn builder() -> AchievementDefinitionsListResponseBuilder {
            AchievementDefinitionsListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement increment response"]
    pub struct AchievementIncrementResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current steps recorded for this incremental achievement."]
        pub current_steps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementIncrementResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newlyUnlocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
        pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
    }
    impl AchievementIncrementResponse {
        pub fn builder() -> AchievementIncrementResponseBuilder {
            AchievementIncrementResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement reveal response"]
    pub struct AchievementRevealResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked."]
        pub current_state: ::std::option::Option<AchievementRevealResponseCurrentStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementRevealResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl AchievementRevealResponse {
        pub fn builder() -> AchievementRevealResponseBuilder {
            AchievementRevealResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the achievement for which a reveal was attempted. This might be `UNLOCKED` if the achievement was already unlocked."]
    pub enum AchievementRevealResponseCurrentStateEnum {
        #[serde(rename = "REVEAL_ACHIEVEMENT_STATE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        RevealAchievementStateUnspecified,
        #[serde(rename = "REVEALED")]
        #[doc = "Achievement is revealed."]
        Revealed,
        #[serde(rename = "UNLOCKED")]
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl ::std::default::Default for AchievementRevealResponseCurrentStateEnum {
        fn default() -> Self {
            Self::RevealAchievementStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement set steps at least response."]
    pub struct AchievementSetStepsAtLeastResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current steps recorded for this incremental achievement."]
        pub current_steps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementSetStepsAtLeastResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newlyUnlocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current steps for the achievement has reached the number of steps required to unlock."]
        pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
    }
    impl AchievementSetStepsAtLeastResponse {
        pub fn builder() -> AchievementSetStepsAtLeastResponseBuilder {
            AchievementSetStepsAtLeastResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement unlock response"]
    pub struct AchievementUnlockResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUnlockResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newlyUnlocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
        pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
    }
    impl AchievementUnlockResponse {
        pub fn builder() -> AchievementUnlockResponseBuilder {
            AchievementUnlockResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of achievement update requests."]
    pub struct AchievementUpdateMultipleRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The individual achievement update requests."]
        pub updates:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementUpdateRequest>>>,
    }
    impl AchievementUpdateMultipleRequest {
        pub fn builder() -> AchievementUpdateMultipleRequestBuilder {
            AchievementUpdateMultipleRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for UpdateMultipleAchievements rpc."]
    pub struct AchievementUpdateMultipleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateMultipleResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedAchievements")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updated state of the achievements."]
        pub updated_achievements:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AchievementUpdateResponse>>>,
    }
    impl AchievementUpdateMultipleResponse {
        pub fn builder() -> AchievementUpdateMultipleResponseBuilder {
            AchievementUpdateMultipleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to update an achievement."]
    pub struct AchievementUpdateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievementId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievement this update is being applied to."]
        pub achievement_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incrementPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The payload if an update of type `INCREMENT` was requested for the achievement."]
        pub increment_payload: ::std::option::Option<::std::boxed::Box<GamesAchievementIncrement>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setStepsAtLeastPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The payload if an update of type `SET_STEPS_AT_LEAST` was requested for the achievement."]
        pub set_steps_at_least_payload:
            ::std::option::Option<::std::boxed::Box<GamesAchievementSetStepsAtLeast>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of update being applied."]
        pub update_type: ::std::option::Option<AchievementUpdateRequestUpdateTypeEnum>,
    }
    impl AchievementUpdateRequest {
        pub fn builder() -> AchievementUpdateRequestBuilder {
            AchievementUpdateRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of update being applied."]
    pub enum AchievementUpdateRequestUpdateTypeEnum {
        #[serde(rename = "ACHIEVEMENT_UPDATE_TYPE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        AchievementUpdateTypeUnspecified,
        #[serde(rename = "REVEAL")]
        #[doc = "Achievement is revealed."]
        Reveal,
        #[serde(rename = "UNLOCK")]
        #[doc = "Achievement is unlocked."]
        Unlock,
        #[serde(rename = "INCREMENT")]
        #[doc = "Achievement is incremented."]
        Increment,
        #[serde(rename = "SET_STEPS_AT_LEAST")]
        #[doc = "Achievement progress is set to at least the passed value."]
        SetStepsAtLeast,
    }
    impl ::std::default::Default for AchievementUpdateRequestUpdateTypeEnum {
        fn default() -> Self {
            Self::AchievementUpdateTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An updated achievement."]
    pub struct AchievementUpdateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievementId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievement this update is was applied to."]
        pub achievement_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the achievement."]
        pub current_state: ::std::option::Option<AchievementUpdateResponseCurrentStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current steps recorded for this achievement if it is incremental."]
        pub current_steps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementUpdateResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newlyUnlocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this achievement was newly unlocked (that is, whether the unlock request for the achievement was the first for the player)."]
        pub newly_unlocked: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateOccurred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the requested updates actually affected the achievement."]
        pub update_occurred: ::std::option::Option<::std::primitive::bool>,
    }
    impl AchievementUpdateResponse {
        pub fn builder() -> AchievementUpdateResponseBuilder {
            AchievementUpdateResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the achievement."]
    pub enum AchievementUpdateResponseCurrentStateEnum {
        #[serde(rename = "UPDATED_ACHIEVEMENT_STATE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        UpdatedAchievementStateUnspecified,
        #[serde(rename = "HIDDEN")]
        #[doc = "Achievement is hidden."]
        Hidden,
        #[serde(rename = "REVEALED")]
        #[doc = "Achievement is revealed."]
        Revealed,
        #[serde(rename = "UNLOCKED")]
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl ::std::default::Default for AchievementUpdateResponseCurrentStateEnum {
        fn default() -> Self {
            Self::UpdatedAchievementStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Application resource."]
    pub struct Application {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievement_count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of achievements visible to the currently authenticated player."]
        pub achievement_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The assets of the application."]
        pub assets: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ImageAsset>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of the application."]
        pub author: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category of the application."]
        pub category: ::std::option::Option<::std::boxed::Box<ApplicationCategory>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the application."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabledFeatures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of features that have been enabled for the application."]
        pub enabled_features:
            ::std::option::Option<::std::vec::Vec<ApplicationEnabledFeaturesEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the application."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "instances")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The instances of the application."]
        pub instances: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Instance>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#application`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdatedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last updated timestamp of the application."]
        pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaderboard_count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of leaderboards visible to the currently authenticated player."]
        pub leaderboard_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the application."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "themeColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. \"E0E0E0\")."]
        pub theme_color: ::std::option::Option<::std::string::String>,
    }
    impl Application {
        pub fn builder() -> ApplicationBuilder {
            ApplicationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ApplicationEnabledFeaturesEnum {
        #[serde(rename = "APPLICATION_FEATURE_UNSPECIFIED")]
        #[doc = "Safe default, don't use."]
        ApplicationFeatureUnspecified,
        #[serde(rename = "SNAPSHOTS")]
        #[doc = "Saved Games (snapshots)."]
        Snapshots,
    }
    impl ::std::default::Default for ApplicationEnabledFeaturesEnum {
        fn default() -> Self {
            Self::ApplicationFeatureUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An application category object."]
    pub struct ApplicationCategory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationCategory`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary category."]
        pub primary: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The secondary category."]
        pub secondary: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationCategory {
        pub fn builder() -> ApplicationCategoryBuilder {
            ApplicationCategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party application verification response resource."]
    pub struct ApplicationVerifyResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternate_player_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An alternate ID that was once used for the player that was issued the auth token used in this request. (This field is not normally populated.)"]
        pub alternate_player_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#applicationVerifyResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the player that was issued the auth token used in this request."]
        pub player_id: ::std::option::Option<::std::string::String>,
    }
    impl ApplicationVerifyResponse {
        pub fn builder() -> ApplicationVerifyResponseBuilder {
            ApplicationVerifyResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Data related to individual game categories."]
    pub struct Category {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "category")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The category name."]
        pub category: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experience points earned in this category."]
        pub experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#category`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl Category {
        pub fn builder() -> CategoryBuilder {
            CategoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party list metagame categories response."]
    pub struct CategoryListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of categories with usage data."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Category>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#categoryListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl CategoryListResponse {
        pub fn builder() -> CategoryListResponseBuilder {
            CategoryListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Container for a URL end point of the requested type."]
    pub struct EndPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A URL suitable for loading in a web browser for the requested endpoint."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl EndPoint {
        pub fn builder() -> EndPointBuilder {
            EndPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A batch update failure resource."]
    pub struct EventBatchRecordFailure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCause")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cause for the update failure."]
        pub failure_cause: ::std::option::Option<EventBatchRecordFailureFailureCauseEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventBatchRecordFailure`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "range")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time range which was rejected; empty for a request-wide failure."]
        pub range: ::std::option::Option<::std::boxed::Box<EventPeriodRange>>,
    }
    impl EventBatchRecordFailure {
        pub fn builder() -> EventBatchRecordFailureBuilder {
            EventBatchRecordFailureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The cause for the update failure."]
    pub enum EventBatchRecordFailureFailureCauseEnum {
        #[serde(rename = "EVENT_FAILURE_CAUSE_UNSPECIFIED")]
        #[doc = "Default value. Should not be used."]
        EventFailureCauseUnspecified,
        #[serde(rename = "TOO_LARGE")]
        #[doc = "A batch request was issued with more events than are allowed in a single batch."]
        TooLarge,
        #[serde(rename = "TIME_PERIOD_EXPIRED")]
        #[doc = "A batch was sent with data too far in the past to record."]
        TimePeriodExpired,
        #[serde(rename = "TIME_PERIOD_SHORT")]
        #[doc = "A batch was sent with a time range that was too short."]
        TimePeriodShort,
        #[serde(rename = "TIME_PERIOD_LONG")]
        #[doc = "A batch was sent with a time range that was too long."]
        TimePeriodLong,
        #[serde(rename = "ALREADY_UPDATED")]
        #[doc = "An attempt was made to record a batch of data which was already seen."]
        AlreadyUpdated,
        #[serde(rename = "RECORD_RATE_HIGH")]
        #[doc = "An attempt was made to record data faster than the server will apply updates."]
        RecordRateHigh,
    }
    impl ::std::default::Default for EventBatchRecordFailureFailureCauseEnum {
        fn default() -> Self {
            Self::EventFailureCauseUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event child relationship resource."]
    pub struct EventChild {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the child event."]
        pub child_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventChild`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl EventChild {
        pub fn builder() -> EventChildBuilder {
            EventChildBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event definition resource."]
    pub struct EventDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of events that are a child of this event."]
        pub child_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventChild>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of what this event represents."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name to display for the event."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the event."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base URL for the image that represents the event."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefaultImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
        pub is_default_image_url: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinition`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visibility of event being tracked in this definition."]
        pub visibility: ::std::option::Option<EventDefinitionVisibilityEnum>,
    }
    impl EventDefinition {
        pub fn builder() -> EventDefinitionBuilder {
            EventDefinitionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The visibility of event being tracked in this definition."]
    pub enum EventDefinitionVisibilityEnum {
        #[serde(rename = "EVENT_VISIBILITY_UNSPECIFIED")]
        #[doc = "Default value. Should not be used."]
        EventVisibilityUnspecified,
        #[serde(rename = "REVEALED")]
        #[doc = "This event should be visible to all users."]
        Revealed,
        #[serde(rename = "HIDDEN")]
        #[doc = "This event should only be shown to users that have recorded this event at least once."]
        Hidden,
    }
    impl ::std::default::Default for EventDefinitionVisibilityEnum {
        fn default() -> Self {
            Self::EventVisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ListDefinitions response."]
    pub struct EventDefinitionListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The event definitions."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventDefinition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventDefinitionListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl EventDefinitionListResponse {
        pub fn builder() -> EventDefinitionListResponseBuilder {
            EventDefinitionListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event period time range."]
    pub struct EventPeriodRange {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodRange`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "periodEndMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this update period ends, in millis, since 1970 UTC (Unix Epoch)."]
        pub period_end_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "periodStartMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time when this update period begins, in millis, since 1970 UTC (Unix Epoch)."]
        pub period_start_millis: ::std::option::Option<::std::string::String>,
    }
    impl EventPeriodRange {
        pub fn builder() -> EventPeriodRangeBuilder {
            EventPeriodRangeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event period update resource."]
    pub struct EventPeriodUpdate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventPeriodUpdate`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePeriod")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time period being covered by this update."]
        pub time_period: ::std::option::Option<::std::boxed::Box<EventPeriodRange>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The updates being made for this time period."]
        pub updates: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventUpdateRequest>>>,
    }
    impl EventPeriodUpdate {
        pub fn builder() -> EventPeriodUpdateBuilder {
            EventPeriodUpdateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event update failure resource."]
    pub struct EventRecordFailure {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the event that was not updated."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureCause")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cause for the update failure."]
        pub failure_cause: ::std::option::Option<EventRecordFailureFailureCauseEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordFailure`."]
        pub kind: ::std::option::Option<::std::string::String>,
    }
    impl EventRecordFailure {
        pub fn builder() -> EventRecordFailureBuilder {
            EventRecordFailureBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The cause for the update failure."]
    pub enum EventRecordFailureFailureCauseEnum {
        #[serde(rename = "EVENT_UPDATE_FAILURE_CAUSE_UNSPECIFIED")]
        #[doc = "Default value. Should not use."]
        EventUpdateFailureCauseUnspecified,
        #[serde(rename = "NOT_FOUND")]
        #[doc = "An attempt was made to set an event that was not defined."]
        NotFound,
        #[serde(rename = "INVALID_UPDATE_VALUE")]
        #[doc = "An attempt was made to increment an event by a non-positive value."]
        InvalidUpdateValue,
    }
    impl ::std::default::Default for EventRecordFailureFailureCauseEnum {
        fn default() -> Self {
            Self::EventUpdateFailureCauseUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event period update resource."]
    pub struct EventRecordRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentTimeMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch)."]
        pub current_time_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The request ID used to identify this attempt to record events."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timePeriods")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of the time period updates being made in this request."]
        pub time_periods:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventPeriodUpdate>>>,
    }
    impl EventRecordRequest {
        pub fn builder() -> EventRecordRequestBuilder {
            EventRecordRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event period update resource."]
    pub struct EventUpdateRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the event being modified in this update."]
        pub definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateRequest`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of times this event occurred in this time period."]
        pub update_count: ::std::option::Option<::std::string::String>,
    }
    impl EventUpdateRequest {
        pub fn builder() -> EventUpdateRequestBuilder {
            EventUpdateRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event period update resource."]
    pub struct EventUpdateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "batchFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any batch-wide failures which occurred applying updates."]
        pub batch_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventBatchRecordFailure>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventFailures")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Any failures updating a particular event."]
        pub event_failures:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<EventRecordFailure>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#eventUpdateResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current status of any updated events"]
        pub player_events: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerEvent>>>,
    }
    impl EventUpdateResponse {
        pub fn builder() -> EventUpdateResponseBuilder {
            EventUpdateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The payload to request to increment an achievement."]
    pub struct GamesAchievementIncrement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementIncrement`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The requestId associated with an increment to an achievement."]
        pub request_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of steps to be incremented."]
        pub steps: ::std::option::Option<::std::primitive::i64>,
    }
    impl GamesAchievementIncrement {
        pub fn builder() -> GamesAchievementIncrementBuilder {
            GamesAchievementIncrementBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The payload to request to increment an achievement."]
    pub struct GamesAchievementSetStepsAtLeast {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#GamesAchievementSetStepsAtLeast`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "steps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum number of steps for the achievement to be set to."]
        pub steps: ::std::option::Option<::std::primitive::i64>,
    }
    impl GamesAchievementSetStepsAtLeast {
        pub fn builder() -> GamesAchievementSetStepsAtLeastBuilder {
            GamesAchievementSetStepsAtLeastBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image asset object."]
    pub struct ImageAsset {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the asset."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#imageAsset`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the asset."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the asset."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the asset."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl ImageAsset {
        pub fn builder() -> ImageAssetBuilder {
            ImageAssetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Instance resource."]
    pub struct Instance {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acquisitionUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "URI which shows where a user can acquire this instance."]
        pub acquisition_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "androidInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform dependent details for Android."]
        pub android_instance: ::std::option::Option<::std::boxed::Box<InstanceAndroidDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iosInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform dependent details for iOS."]
        pub ios_instance: ::std::option::Option<::std::boxed::Box<InstanceIosDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instance`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platformType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The platform type."]
        pub platform_type: ::std::option::Option<InstancePlatformTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "realtimePlay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to show if this game instance supports realtime play."]
        pub realtime_play: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "turnBasedPlay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to show if this game instance supports turn based play."]
        pub turn_based_play: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webInstance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Platform dependent details for Web."]
        pub web_instance: ::std::option::Option<::std::boxed::Box<InstanceWebDetails>>,
    }
    impl Instance {
        pub fn builder() -> InstanceBuilder {
            InstanceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The platform type."]
    pub enum InstancePlatformTypeEnum {
        #[serde(rename = "PLATFORM_TYPE_UNSPECIFIED")]
        #[doc = "Default value. Should be unused."]
        PlatformTypeUnspecified,
        #[serde(rename = "ANDROID")]
        #[doc = "Instance is for Android."]
        Android,
        #[serde(rename = "IOS")]
        #[doc = "Instance is for iOS."]
        Ios,
        #[serde(rename = "WEB_APP")]
        #[doc = "Instance is for Web App."]
        WebApp,
    }
    impl ::std::default::Default for InstancePlatformTypeEnum {
        fn default() -> Self {
            Self::PlatformTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Android instance details resource."]
    pub struct InstanceAndroidDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enablePiracyCheck")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag indicating whether the anti-piracy check is enabled."]
        pub enable_piracy_check: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceAndroidDetails`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "packageName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Android package name which maps to Google Play URL."]
        pub package_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this instance is the default for new installations."]
        pub preferred: ::std::option::Option<::std::primitive::bool>,
    }
    impl InstanceAndroidDetails {
        pub fn builder() -> InstanceAndroidDetailsBuilder {
            InstanceAndroidDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The iOS details resource."]
    pub struct InstanceIosDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bundleIdentifier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Bundle identifier."]
        pub bundle_identifier: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itunesAppId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "iTunes App ID."]
        pub itunes_app_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceIosDetails`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferredForIpad")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this instance is the default for new installations on iPad devices."]
        pub preferred_for_ipad: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferredForIphone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this instance is the default for new installations on iPhone devices."]
        pub preferred_for_iphone: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportIpad")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to indicate if this instance supports iPad."]
        pub support_ipad: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supportIphone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flag to indicate if this instance supports iPhone."]
        pub support_iphone: ::std::option::Option<::std::primitive::bool>,
    }
    impl InstanceIosDetails {
        pub fn builder() -> InstanceIosDetailsBuilder {
            InstanceIosDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Web details resource."]
    pub struct InstanceWebDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#instanceWebDetails`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "launchUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Launch URL for the game."]
        pub launch_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "preferred")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that this instance is the default for new installations."]
        pub preferred: ::std::option::Option<::std::primitive::bool>,
    }
    impl InstanceWebDetails {
        pub fn builder() -> InstanceWebDetailsBuilder {
            InstanceWebDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Leaderboard resource."]
    pub struct Leaderboard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "iconUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The icon for the leaderboard."]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard ID."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isIconUrlDefault")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the icon image being returned is a default image, or is game-provided."]
        pub is_icon_url_default: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboard`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the leaderboard."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How scores are ordered."]
        pub order: ::std::option::Option<LeaderboardOrderEnum>,
    }
    impl Leaderboard {
        pub fn builder() -> LeaderboardBuilder {
            LeaderboardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How scores are ordered."]
    pub enum LeaderboardOrderEnum {
        #[serde(rename = "SCORE_ORDER_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreOrderUnspecified,
        #[serde(rename = "LARGER_IS_BETTER")]
        #[doc = "Larger values are better; scores are sorted in descending order"]
        LargerIsBetter,
        #[serde(rename = "SMALLER_IS_BETTER")]
        #[doc = "Smaller values are better; scores are sorted in ascending order"]
        SmallerIsBetter,
    }
    impl ::std::default::Default for LeaderboardOrderEnum {
        fn default() -> Self {
            Self::ScoreOrderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The Leaderboard Entry resource."]
    pub struct LeaderboardEntry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized string for the numerical value of this score."]
        pub formatted_score: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedScoreRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized string for the rank of this score for this leaderboard."]
        pub formatted_score_rank: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardEntry`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player who holds this score."]
        pub player: ::std::option::Option<::std::boxed::Box<Player>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rank of this score for this leaderboard."]
        pub score_rank: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub score_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerical value of this score."]
        pub score_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time span of this high score."]
        pub time_span: ::std::option::Option<LeaderboardEntryTimeSpanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeTimestampMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
        pub write_timestamp_millis: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardEntry {
        pub fn builder() -> LeaderboardEntryBuilder {
            LeaderboardEntryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The time span of this high score."]
    pub enum LeaderboardEntryTimeSpanEnum {
        #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[serde(rename = "ALL_TIME")]
        #[doc = "The score is an all-time score."]
        AllTime,
        #[serde(rename = "WEEKLY")]
        #[doc = "The score is a weekly score."]
        Weekly,
        #[serde(rename = "DAILY")]
        #[doc = "The score is a daily score."]
        Daily,
    }
    impl ::std::default::Default for LeaderboardEntryTimeSpanEnum {
        fn default() -> Self {
            Self::ScoreTimeSpanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of leaderboard objects."]
    pub struct LeaderboardListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboards."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Leaderboard>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardListResponse {
        pub fn builder() -> LeaderboardListResponseBuilder {
            LeaderboardListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A score rank in a leaderboard."]
    pub struct LeaderboardScoreRank {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedNumScores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of scores in the leaderboard as a string."]
        pub formatted_num_scores: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rank in the leaderboard as a string."]
        pub formatted_rank: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScoreRank`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numScores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of scores in the leaderboard."]
        pub num_scores: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rank in the leaderboard."]
        pub rank: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardScoreRank {
        pub fn builder() -> LeaderboardScoreRankBuilder {
            LeaderboardScoreRankBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ListScores response."]
    pub struct LeaderboardScores {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scores in the leaderboard."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LeaderboardEntry>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboardScores`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numScores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of scores in the leaderboard."]
        pub num_scores: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The score of the requesting player on the leaderboard. The player's score may appear both here and in the list of scores above. If you are viewing a public leaderboard and the player is not sharing their gameplay information publicly, the `scoreRank`and `formattedScoreRank` values will not be present."]
        pub player_score: ::std::option::Option<::std::boxed::Box<LeaderboardEntry>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the previous page of results."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
    }
    impl LeaderboardScores {
        pub fn builder() -> LeaderboardScoresBuilder {
            LeaderboardScoresBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The metagame config resource"]
    pub struct MetagameConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Current version of the metagame configuration data. When this data is updated, the version number will be increased by one."]
        pub current_version: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#metagameConfig`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of player levels."]
        pub player_levels: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerLevel>>>,
    }
    impl MetagameConfig {
        pub fn builder() -> MetagameConfigBuilder {
            MetagameConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Player resource."]
    pub struct Player {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "avatarImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base URL for the image that represents the player."]
        pub avatar_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerUrlLandscape")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to the landscape mode player banner image."]
        pub banner_url_landscape: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerUrlPortrait")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url to the portrait mode player banner image."]
        pub banner_url_portrait: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name to display for the player."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experienceInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An object to represent Play Game experience information for the player."]
        pub experience_info: ::std::option::Option<::std::boxed::Box<PlayerExperienceInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game."]
        pub friend_status: ::std::option::Option<PlayerFriendStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#player`"]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A representation of the individual components of the name."]
        pub name: ::std::option::Option<PlayerName>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalPlayerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs."]
        pub original_player_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the player."]
        pub player_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player's profile settings. Controls whether or not the player's profile is visible to other players."]
        pub profile_settings: ::std::option::Option<::std::boxed::Box<ProfileSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player's title rewarded for their game activities."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Player {
        pub fn builder() -> PlayerBuilder {
            PlayerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game."]
    pub enum PlayerFriendStatusEnum {
        #[serde(rename = "FRIEND_STATUS_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        FriendStatusUnspecified,
        #[serde(rename = "NO_RELATIONSHIP")]
        #[doc = "There is no relationship between the players."]
        NoRelationship,
        #[serde(rename = "FRIEND")]
        #[doc = "The player and requester are friends."]
        Friend,
    }
    impl ::std::default::Default for PlayerFriendStatusEnum {
        fn default() -> Self {
            Self::FriendStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A representation of the individual components of the name."]
    pub struct PlayerName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "familyName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The family name of this player. In some places, this is known as the last name."]
        pub family_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "givenName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The given name of this player. In some places, this is known as the first name."]
        pub given_name: ::std::option::Option<::std::string::String>,
    }
    impl PlayerName {
        pub fn builder() -> PlayerNameBuilder {
            PlayerNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An achievement object."]
    pub struct PlayerAchievement {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "achievementState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the achievement."]
        pub achievement_state: ::std::option::Option<PlayerAchievementAchievementStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentSteps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current steps for an incremental achievement."]
        pub current_steps: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Experience points earned for the achievement. This field is absent for achievements that have not yet been unlocked and 0 for achievements that have been unlocked by testers but that are unpublished."]
        pub experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedCurrentStepsString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current steps for an incremental achievement as a string."]
        pub formatted_current_steps_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the achievement."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievement`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdatedTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the last modification to this achievement's state."]
        pub last_updated_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PlayerAchievement {
        pub fn builder() -> PlayerAchievementBuilder {
            PlayerAchievementBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The state of the achievement."]
    pub enum PlayerAchievementAchievementStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        StateUnspecified,
        #[serde(rename = "HIDDEN")]
        #[doc = "Achievement is hidden."]
        Hidden,
        #[serde(rename = "REVEALED")]
        #[doc = "Achievement is revealed."]
        Revealed,
        #[serde(rename = "UNLOCKED")]
        #[doc = "Achievement is unlocked."]
        Unlocked,
    }
    impl ::std::default::Default for PlayerAchievementAchievementStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of achievement objects."]
    pub struct PlayerAchievementListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The achievements."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerAchievement>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievementListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PlayerAchievementListResponse {
        pub fn builder() -> PlayerAchievementListResponseBuilder {
            PlayerAchievementListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event status resource."]
    pub struct PlayerEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definitionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the event definition."]
        pub definition_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedNumEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of times this event has occurred, as a string. The formatting of this string depends on the configuration of your event in the Play Games Developer Console."]
        pub formatted_num_events: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEvent`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numEvents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of times this event has occurred."]
        pub num_events: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the player."]
        pub player_id: ::std::option::Option<::std::string::String>,
    }
    impl PlayerEvent {
        pub fn builder() -> PlayerEventBuilder {
            PlayerEventBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A ListByPlayer response."]
    pub struct PlayerEventListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player events."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerEvent>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEventListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PlayerEventListResponse {
        pub fn builder() -> PlayerEventListResponseBuilder {
            PlayerEventListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "1P/3P metadata about the player's experience."]
    pub struct PlayerExperienceInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current number of experience points for the player."]
        pub current_experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current level of the player."]
        pub current_level: ::std::option::Option<::std::boxed::Box<PlayerLevel>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerExperienceInfo`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastLevelUpTimestampMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the player was leveled up, in millis since Unix epoch UTC."]
        pub last_level_up_timestamp_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The next level of the player. If the current level is the maximum level, this should be same as the current level."]
        pub next_level: ::std::option::Option<::std::boxed::Box<PlayerLevel>>,
    }
    impl PlayerExperienceInfo {
        pub fn builder() -> PlayerExperienceInfoBuilder {
            PlayerExperienceInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A player leaderboard score object."]
    pub struct PlayerLeaderboardScore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendsRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rank of the score in the friends collection for this leaderboard."]
        pub friends_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScore`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaderboard_id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the leaderboard this score is in."]
        pub leaderboard_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The public rank of the score in this leaderboard. This object will not be present if the user is not sharing their scores publicly."]
        pub public_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatted value of this score."]
        pub score_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about the score. Values must contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub score_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerical value of this score."]
        pub score_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "socialRank")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The social rank of the score in this leaderboard."]
        pub social_rank: ::std::option::Option<::std::boxed::Box<LeaderboardScoreRank>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time span of this score."]
        pub time_span: ::std::option::Option<PlayerLeaderboardScoreTimeSpanEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "writeTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp at which this score was recorded, in milliseconds since the epoch in UTC."]
        pub write_timestamp: ::std::option::Option<::std::string::String>,
    }
    impl PlayerLeaderboardScore {
        pub fn builder() -> PlayerLeaderboardScoreBuilder {
            PlayerLeaderboardScoreBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The time span of this score."]
    pub enum PlayerLeaderboardScoreTimeSpanEnum {
        #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[serde(rename = "ALL_TIME")]
        #[doc = "The score is an all-time score."]
        AllTime,
        #[serde(rename = "WEEKLY")]
        #[doc = "The score is a weekly score."]
        Weekly,
        #[serde(rename = "DAILY")]
        #[doc = "The score is a daily score."]
        Daily,
    }
    impl ::std::default::Default for PlayerLeaderboardScoreTimeSpanEnum {
        fn default() -> Self {
            Self::ScoreTimeSpanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of player leaderboard scores."]
    pub struct PlayerLeaderboardScoreListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard scores."]
        pub items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerLeaderboardScore>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScoreListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The pagination token for the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Player resources for the owner of this score."]
        pub player: ::std::option::Option<::std::boxed::Box<Player>>,
    }
    impl PlayerLeaderboardScoreListResponse {
        pub fn builder() -> PlayerLeaderboardScoreListResponseBuilder {
            PlayerLeaderboardScoreListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "1P/3P metadata about a user's level."]
    pub struct PlayerLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLevel`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The level for the user."]
        pub level: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum experience points for this level."]
        pub max_experience_points: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "minExperiencePoints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The minimum experience points for this level."]
        pub min_experience_points: ::std::option::Option<::std::string::String>,
    }
    impl PlayerLevel {
        pub fn builder() -> PlayerLevelBuilder {
            PlayerLevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party player list response."]
    pub struct PlayerListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The players."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Player>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl PlayerListResponse {
        pub fn builder() -> PlayerListResponseBuilder {
            PlayerListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A player score."]
    pub struct PlayerScore {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatted score for this player score."]
        pub formatted_score: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScore`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The numerical value for this player score."]
        pub score: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub score_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeSpan")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time span for this player score."]
        pub time_span: ::std::option::Option<PlayerScoreTimeSpanEnum>,
    }
    impl PlayerScore {
        pub fn builder() -> PlayerScoreBuilder {
            PlayerScoreBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The time span for this player score."]
    pub enum PlayerScoreTimeSpanEnum {
        #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[serde(rename = "ALL_TIME")]
        #[doc = "The score is an all-time score."]
        AllTime,
        #[serde(rename = "WEEKLY")]
        #[doc = "The score is a weekly score."]
        Weekly,
        #[serde(rename = "DAILY")]
        #[doc = "The score is a daily score."]
        Daily,
    }
    impl ::std::default::Default for PlayerScoreTimeSpanEnum {
        fn default() -> Self {
            Self::ScoreTimeSpanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of score submission statuses."]
    pub struct PlayerScoreListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "submittedScores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The score submissions statuses."]
        pub submitted_scores:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScoreResponse>>>,
    }
    impl PlayerScoreListResponse {
        pub fn builder() -> PlayerScoreListResponseBuilder {
            PlayerScoreListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of leaderboard entry resources."]
    pub struct PlayerScoreResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "beatenScoreTimeSpans")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time spans where the submitted score is better than the existing score for that time span."]
        pub beaten_score_time_spans:
            ::std::option::Option<::std::vec::Vec<PlayerScoreResponseBeatenScoreTimeSpansEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The formatted value of the submitted score."]
        pub formatted_score: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaderboardId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard ID that this score was submitted to."]
        pub leaderboard_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub score_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unbeatenScores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The scores in time spans that have not been beaten. As an example, the submitted score may be better than the player's `DAILY` score, but not better than the player's scores for the `WEEKLY` or `ALL_TIME` time spans."]
        pub unbeaten_scores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlayerScore>>>,
    }
    impl PlayerScoreResponse {
        pub fn builder() -> PlayerScoreResponseBuilder {
            PlayerScoreResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum PlayerScoreResponseBeatenScoreTimeSpansEnum {
        #[serde(rename = "SCORE_TIME_SPAN_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        ScoreTimeSpanUnspecified,
        #[serde(rename = "ALL_TIME")]
        #[doc = "The score is an all-time score."]
        AllTime,
        #[serde(rename = "WEEKLY")]
        #[doc = "The score is a weekly score."]
        Weekly,
        #[serde(rename = "DAILY")]
        #[doc = "The score is a daily score."]
        Daily,
    }
    impl ::std::default::Default for PlayerScoreResponseBeatenScoreTimeSpansEnum {
        fn default() -> Self {
            Self::ScoreTimeSpanUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of score submission requests."]
    pub struct PlayerScoreSubmissionList {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreSubmissionList`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scores")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The score submissions."]
        pub scores: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ScoreSubmission>>>,
    }
    impl PlayerScoreSubmissionList {
        pub fn builder() -> PlayerScoreSubmissionListBuilder {
            PlayerScoreSubmissionListBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Profile settings"]
    pub struct ProfileSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "friendsListVisibility")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub friends_list_visibility:
            ::std::option::Option<ProfileSettingsFriendsListVisibilityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#profileSettings`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileVisible")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the player's profile is visible to the currently signed in player."]
        pub profile_visible: ::std::option::Option<::std::primitive::bool>,
    }
    impl ProfileSettings {
        pub fn builder() -> ProfileSettingsBuilder {
            ProfileSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ProfileSettingsFriendsListVisibilityEnum {
        #[serde(rename = "FRIENDS_LIST_VISIBILITY_UNSPECIFIED")]
        #[doc = "Unused."]
        FriendsListVisibilityUnspecified,
        #[serde(rename = "VISIBLE")]
        #[doc = "The friends list is currently visible to the game."]
        Visible,
        #[serde(rename = "REQUEST_REQUIRED")]
        #[doc = "The developer does not have access to the friends list, but can call the Android API to show a consent dialog."]
        RequestRequired,
        #[serde(rename = "UNAVAILABLE")]
        #[doc = "The friends list is currently unavailable for this user, and it is not possible to request access at this time, either because the user has permanently declined or the friends feature is not available to them. In this state, any attempts to request access to the friends list will be unsuccessful."]
        Unavailable,
    }
    impl ::std::default::Default for ProfileSettingsFriendsListVisibilityEnum {
        fn default() -> Self {
            Self::FriendsListVisibilityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request for ResolveSnapshotHead RPC."]
    pub struct ResolveSnapshotHeadRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxConflictsPerSnapshot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum number of SnapshotRevision resources for `conflictingRevisions` to return per SnapshotExtended resource in the response. For any response, the actual number of resources returned may be less than specified by `maxConflictsPerSnapshot`. The value provided should be greater or equal to 0. If no value is provided, the server will use a sensible default."]
        pub max_conflicts_per_snapshot: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolutionPolicy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The automatic resolution policy. All conflicts are resolved in chronological order, starting from the/ least recent. If the comparison metric is equal for the tentative head and the conflict, the head wins."]
        pub resolution_policy:
            ::std::option::Option<ResolveSnapshotHeadRequestResolutionPolicyEnum>,
    }
    impl ResolveSnapshotHeadRequest {
        pub fn builder() -> ResolveSnapshotHeadRequestBuilder {
            ResolveSnapshotHeadRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The automatic resolution policy. All conflicts are resolved in chronological order, starting from the/ least recent. If the comparison metric is equal for the tentative head and the conflict, the head wins."]
    pub enum ResolveSnapshotHeadRequestResolutionPolicyEnum {
        #[serde(rename = "RESOLUTION_POLICY_UNSPECIFIED")]
        #[doc = "Safe default, don't use explicitly."]
        ResolutionPolicyUnspecified,
        #[serde(rename = "USE_HEAD")]
        #[doc = "Drops all conflicts and keeps the current head only."]
        UseHead,
        #[serde(rename = "LONGEST_PLAYTIME")]
        #[doc = "Use the snapshot with the longest played time."]
        LongestPlaytime,
        #[serde(rename = "MOST_RECENTLY_MODIFIED")]
        #[doc = "Use the snapshot that was most recently modified."]
        MostRecentlyModified,
        #[serde(rename = "HIGHEST_PROGRESS")]
        #[doc = "Use the snapshot with the highest progress value."]
        HighestProgress,
        #[serde(rename = "NO_AUTOMATIC_RESOLUTION")]
        #[doc = "Don't resolve conflicts at all. Effectively only returns the current head revision of the snapshot. Corresponds to a game opening the snapshot with manual resolution policy."]
        NoAutomaticResolution,
    }
    impl ::std::default::Default for ResolveSnapshotHeadRequestResolutionPolicyEnum {
        fn default() -> Self {
            Self::ResolutionPolicyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response for ResolveSnapshotHead RPC."]
    pub struct ResolveSnapshotHeadResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshot")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The state of the snapshot."]
        pub snapshot: ::std::option::Option<::std::boxed::Box<SnapshotExtended>>,
    }
    impl ResolveSnapshotHeadResponse {
        pub fn builder() -> ResolveSnapshotHeadResponseBuilder {
            ResolveSnapshotHeadResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party checking a revision response."]
    pub struct RevisionCheckResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "apiVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the API this client revision should use when calling API methods."]
        pub api_version: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#revisionCheckResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "revisionStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the revision check."]
        pub revision_status: ::std::option::Option<RevisionCheckResponseRevisionStatusEnum>,
    }
    impl RevisionCheckResponse {
        pub fn builder() -> RevisionCheckResponseBuilder {
            RevisionCheckResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The result of the revision check."]
    pub enum RevisionCheckResponseRevisionStatusEnum {
        #[serde(rename = "REVISION_STATUS_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        RevisionStatusUnspecified,
        #[serde(rename = "OK")]
        #[doc = "The revision being used is current."]
        Ok,
        #[serde(rename = "DEPRECATED")]
        #[doc = "There is currently a newer version available, but the revision being used still works."]
        Deprecated,
        #[serde(rename = "INVALID")]
        #[doc = "The revision being used is not supported in any released version."]
        Invalid,
    }
    impl ::std::default::Default for RevisionCheckResponseRevisionStatusEnum {
        fn default() -> Self {
            Self::RevisionStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A request to submit a score to leaderboards."]
    pub struct ScoreSubmission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#scoreSubmission`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "leaderboardId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The leaderboard this score is being submitted to."]
        pub leaderboard_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new score being submitted."]
        pub score: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scoreTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional information about this score. Values will contain no more than 64 URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub score_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signature Values will contain URI-safe characters as defined by section 2.3 of RFC 3986."]
        pub signature: ::std::option::Option<::std::string::String>,
    }
    impl ScoreSubmission {
        pub fn builder() -> ScoreSubmissionBuilder {
            ScoreSubmissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An snapshot object."]
    pub struct Snapshot {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cover image of this snapshot. May be absent if there is no image."]
        pub cover_image: ::std::option::Option<::std::boxed::Box<SnapshotImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this snapshot."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "driveId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller."]
        pub drive_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration associated with this snapshot, in millis."]
        pub duration_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the snapshot."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshot`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifiedMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp (in millis since Unix epoch) of the last modification to this snapshot."]
        pub last_modified_millis: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
        pub progress_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of this snapshot."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this snapshot."]
        pub _type: ::std::option::Option<SnapshotTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique name provided when the snapshot was created."]
        pub unique_name: ::std::option::Option<::std::string::String>,
    }
    impl Snapshot {
        pub fn builder() -> SnapshotBuilder {
            SnapshotBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this snapshot."]
    pub enum SnapshotTypeEnum {
        #[serde(rename = "SNAPSHOT_TYPE_UNSPECIFIED")]
        #[doc = "Default value. This value is unused."]
        SnapshotTypeUnspecified,
        #[serde(rename = "SAVE_GAME")]
        #[doc = "A snapshot representing a save game."]
        SaveGame,
    }
    impl ::std::default::Default for SnapshotTypeEnum {
        fn default() -> Self {
            Self::SnapshotTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a snapshot cover image resource. The image is provided by the game."]
    pub struct SnapshotCoverImageResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Hash-like weak identifier of the uploaded image bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
        pub content_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A URL the client can use to download the image. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the image in pixels."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mimeType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The MIME type of the image."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the image resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the image in pixels."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl SnapshotCoverImageResource {
        pub fn builder() -> SnapshotCoverImageResourceBuilder {
            SnapshotCoverImageResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Identifies a snapshot data resource. The data is provided by the game."]
    pub struct SnapshotDataResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentHash")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Hash-like weak identifier of the uploaded blob bytes, consistent per player per application. The content hash for a given resource will not change if the binary data hasn't changed. Except in very rare circumstances, the content_hash for matching binary data will be the same within a given player and application."]
        pub content_hash: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "downloadUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A URL that the client can use to download the blob. May vary across requests, and only guaranteed to be valid for a short time after it is returned."]
        pub download_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the blob resource. It's guaranteed that if two IDs are equal then the contents are equal as well. It's not guaranteed that two identical blobs coming from separate uploads have the same resource ID. The resource ID can only be used within the application, user and resource type it was originally returned for. For example, it's not possible to use SnapshotDataResource's resource ID as the resource_id of a SnapshotCoverImageResource, even if the blob is a valid image file."]
        pub resource_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "size")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Size of the saved game blob in bytes."]
        pub size: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotDataResource {
        pub fn builder() -> SnapshotDataResourceBuilder {
            SnapshotDataResourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A snapshot represents a saved game state referred to using the developer-provided snapshot_name. The set of attributes and binary data for a specific state is called a revision. Each revision is itself immutable, and referred to by a snapshot revision id. At any time, a snapshot has a \"head\" revision, and updates are made against that revision. If a snapshot update is received that isn't against the current head revision, then instead of changing the head revision it will result in a conflicting revision that must be specifically resolved."]
    pub struct SnapshotExtended {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conflictingRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of conflicting revisions. Only set if explicitly requested (e.g. using a field mask or a request flag), or if the RPC guarantees that this field is set. The conflicting revisions are sorted chronologically by their server creation time (oldest first). If there are too many conflicting revisions to return all of them in a single request this will only contain the first batch. In such case, the presented conflicting revisions must be resolved first in order to fetch the next batch."]
        pub conflicting_revisions:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SnapshotRevision>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasConflictingRevisions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An indicator whether the snapshot has any conflicting revisions or not. Always set."]
        pub has_conflicting_revisions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "headRevision")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current head revision (the canonical revision as understood by the server)."]
        pub head_revision: ::std::option::Option<::std::boxed::Box<SnapshotRevision>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snapshotName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An identifier of the snapshot, developer-specified. It must match the pattern [0-9a-zA-Z-._~]{1,100}."]
        pub snapshot_name: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotExtended {
        pub fn builder() -> SnapshotExtendedBuilder {
            SnapshotExtendedBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An image of a snapshot."]
    pub struct SnapshotImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The height of the image."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotImage`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mime_type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The MIME type of the image."]
        pub mime_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the image. This URL may be invalidated at any time and should not be cached."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The width of the image."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl SnapshotImage {
        pub fn builder() -> SnapshotImageBuilder {
            SnapshotImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party list snapshots response."]
    pub struct SnapshotListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snapshots."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Snapshot>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshotListResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token corresponding to the next page of results. If there are no more results, the token is omitted."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotListResponse {
        pub fn builder() -> SnapshotListResponseBuilder {
            SnapshotListResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata about a snapshot revision. Snapshot metadata is immutable - a metadata change corresponds to a new snapshot revision."]
    pub struct SnapshotMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of this snapshot."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deviceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The device that created the current revision."]
        pub device_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gameplayDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration associated with this snapshot. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
        pub gameplay_duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastModifyTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp of the last modification to this snapshot as provided by the client. Values with sub-millisecond precision can be rounded or trimmed to the closest millisecond."]
        pub last_modify_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "progressValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The progress value (64-bit integer set by developer) associated with this snapshot."]
        pub progress_value: ::std::option::Option<::std::string::String>,
    }
    impl SnapshotMetadata {
        pub fn builder() -> SnapshotMetadataBuilder {
            SnapshotMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Snapshot revision resource. Snapshot revisions are immutable."]
    pub struct SnapshotRevision {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blob")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the game provided blob for this revision."]
        pub blob: ::std::option::Option<::std::boxed::Box<SnapshotDataResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reference to the cover image for this revision."]
        pub cover_image: ::std::option::Option<::std::boxed::Box<SnapshotCoverImageResource>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A server generated identifier of the snapshot revision."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Metadata for this snapshot revision."]
        pub metadata: ::std::option::Option<::std::boxed::Box<SnapshotMetadata>>,
    }
    impl SnapshotRevision {
        pub fn builder() -> SnapshotRevisionBuilder {
            SnapshotRevisionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A third party stats resource."]
    pub struct StatsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "avg_session_length_minutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Average session length in minutes of the player. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
        pub avg_session_length_minutes: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "churn_probability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The probability of the player not returning to play the game in the next day. E.g., 0, 0.1, 0.5, ..., 1.0. Not populated if there is not enough information."]
        pub churn_probability: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "days_since_last_played")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of days since the player last played this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        pub days_since_last_played: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "high_spender_probability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The probability of the player going to spend beyond a threshold amount of money. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
        pub high_spender_probability: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uniquely identifies the type of this resource. Value is always the fixed string `games#statsResponse`."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "num_purchases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of in-app purchases made by the player in this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        pub num_purchases: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "num_sessions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The approximate number of sessions of the player within the last 28 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information."]
        pub num_sessions: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "num_sessions_percentile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The approximation of the sessions percentile of the player within the last 30 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
        pub num_sessions_percentile: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spend_percentile")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The approximate spend percentile of the player in this game. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information."]
        pub spend_percentile: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "spend_probability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The probability of the player going to spend the game in the next seven days. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information."]
        pub spend_probability: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "total_spend_next_28_days")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The predicted amount of money that the player going to spend in the next 28 days. E.g., 1, 30, 60, ... . Not populated if there is not enough information."]
        pub total_spend_next_28_days: ::std::option::Option<::std::primitive::f64>,
    }
    impl StatsResponse {
        pub fn builder() -> StatsResponseBuilder {
            StatsResponseBuilder::default()
        }
    }
}
