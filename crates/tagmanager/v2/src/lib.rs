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
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Continuation token for fetching the next page of results."]
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
                    #[serde(rename = "fingerprint")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "When provided, this fingerprint must match the fingerprint of the account in storage."]
                    pub fingerprint: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
        pub mod resources {
            pub mod containers {
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
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "fingerprint")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "When provided, this fingerprint must match the fingerprint of the container in storage."]
                            pub fingerprint: ::std::option::Option<::std::string::String>,
                        }
                        impl QueryParameters {
                            pub fn builder() -> QueryParametersBuilder {
                                QueryParametersBuilder::default()
                            }
                        }
                    }
                }
                pub mod resources {
                    pub mod environments {
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
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Continuation token for fetching the next page of results."]
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "fingerprint")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When provided, this fingerprint must match the fingerprint of the environment in storage."]
                                    pub fingerprint: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod version_headers {
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
                                    #[serde(rename = "includeDeleted")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Also retrieve deleted (archived) versions when true."]
                                    pub include_deleted:
                                        ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Continuation token for fetching the next page of results."]
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
                    pub mod versions {
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
                                    #[serde(rename = "containerVersionId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The GTM ContainerVersion ID. Specify published to retrieve the currently published version."]
                                    pub container_version_id:
                                        ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "fingerprint")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When provided, this fingerprint must match the fingerprint of the container version in storage."]
                                    pub fingerprint: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "fingerprint")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When provided, this fingerprint must match the fingerprint of the container version in storage."]
                                    pub fingerprint: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                    }
                    pub mod workspaces {
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
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Continuation token for fetching the next page of results."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                            pub mod resolve_conflict {
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
                                    #[serde(rename = "fingerprint")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When provided, this fingerprint must match the fingerprint of the entity_in_workspace in the merge conflict."]
                                    pub fingerprint: ::std::option::Option<::std::string::String>,
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "fingerprint")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "When provided, this fingerprint must match the fingerprint of the workspace in storage."]
                                    pub fingerprint: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod built_in_variables {
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
                                            #[serde(rename = "type")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The types of built-in variables to enable."]
                                            pub _type:
                                                ::std::option::Option<QueryParametersTypeEnum>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "The types of built-in variables to enable."]
                                        pub enum QueryParametersTypeEnum {
                                            #[serde(rename = "builtInVariableTypeUnspecified")]
                                            #[doc = ""]
                                            BuiltInVariableTypeUnspecified,
                                            #[serde(rename = "pageUrl")]
                                            #[doc = ""]
                                            PageUrl,
                                            #[serde(rename = "pageHostname")]
                                            #[doc = ""]
                                            PageHostname,
                                            #[serde(rename = "pagePath")]
                                            #[doc = ""]
                                            PagePath,
                                            #[serde(rename = "referrer")]
                                            #[doc = ""]
                                            Referrer,
                                            #[serde(rename = "event")]
                                            #[doc = "For web or mobile."]
                                            Event,
                                            #[serde(rename = "clickElement")]
                                            #[doc = ""]
                                            ClickElement,
                                            #[serde(rename = "clickClasses")]
                                            #[doc = ""]
                                            ClickClasses,
                                            #[serde(rename = "clickId")]
                                            #[doc = ""]
                                            ClickId,
                                            #[serde(rename = "clickTarget")]
                                            #[doc = ""]
                                            ClickTarget,
                                            #[serde(rename = "clickUrl")]
                                            #[doc = ""]
                                            ClickUrl,
                                            #[serde(rename = "clickText")]
                                            #[doc = ""]
                                            ClickText,
                                            #[serde(rename = "firstPartyServingUrl")]
                                            #[doc = ""]
                                            FirstPartyServingUrl,
                                            #[serde(rename = "formElement")]
                                            #[doc = ""]
                                            FormElement,
                                            #[serde(rename = "formClasses")]
                                            #[doc = ""]
                                            FormClasses,
                                            #[serde(rename = "formId")]
                                            #[doc = ""]
                                            FormId,
                                            #[serde(rename = "formTarget")]
                                            #[doc = ""]
                                            FormTarget,
                                            #[serde(rename = "formUrl")]
                                            #[doc = ""]
                                            FormUrl,
                                            #[serde(rename = "formText")]
                                            #[doc = ""]
                                            FormText,
                                            #[serde(rename = "errorMessage")]
                                            #[doc = ""]
                                            ErrorMessage,
                                            #[serde(rename = "errorUrl")]
                                            #[doc = ""]
                                            ErrorUrl,
                                            #[serde(rename = "errorLine")]
                                            #[doc = ""]
                                            ErrorLine,
                                            #[serde(rename = "newHistoryUrl")]
                                            #[doc = ""]
                                            NewHistoryUrl,
                                            #[serde(rename = "oldHistoryUrl")]
                                            #[doc = ""]
                                            OldHistoryUrl,
                                            #[serde(rename = "newHistoryFragment")]
                                            #[doc = ""]
                                            NewHistoryFragment,
                                            #[serde(rename = "oldHistoryFragment")]
                                            #[doc = ""]
                                            OldHistoryFragment,
                                            #[serde(rename = "newHistoryState")]
                                            #[doc = ""]
                                            NewHistoryState,
                                            #[serde(rename = "oldHistoryState")]
                                            #[doc = ""]
                                            OldHistoryState,
                                            #[serde(rename = "historySource")]
                                            #[doc = ""]
                                            HistorySource,
                                            #[serde(rename = "containerVersion")]
                                            #[doc = "For web or mobile."]
                                            ContainerVersion,
                                            #[serde(rename = "debugMode")]
                                            #[doc = ""]
                                            DebugMode,
                                            #[serde(rename = "randomNumber")]
                                            #[doc = "For web or mobile."]
                                            RandomNumber,
                                            #[serde(rename = "containerId")]
                                            #[doc = "For web or mobile."]
                                            ContainerId,
                                            #[serde(rename = "appId")]
                                            #[doc = ""]
                                            AppId,
                                            #[serde(rename = "appName")]
                                            #[doc = ""]
                                            AppName,
                                            #[serde(rename = "appVersionCode")]
                                            #[doc = ""]
                                            AppVersionCode,
                                            #[serde(rename = "appVersionName")]
                                            #[doc = ""]
                                            AppVersionName,
                                            #[serde(rename = "language")]
                                            #[doc = ""]
                                            Language,
                                            #[serde(rename = "osVersion")]
                                            #[doc = ""]
                                            OsVersion,
                                            #[serde(rename = "platform")]
                                            #[doc = ""]
                                            Platform,
                                            #[serde(rename = "sdkVersion")]
                                            #[doc = ""]
                                            SdkVersion,
                                            #[serde(rename = "deviceName")]
                                            #[doc = ""]
                                            DeviceName,
                                            #[serde(rename = "resolution")]
                                            #[doc = ""]
                                            Resolution,
                                            #[serde(rename = "advertiserId")]
                                            #[doc = ""]
                                            AdvertiserId,
                                            #[serde(rename = "advertisingTrackingEnabled")]
                                            #[doc = ""]
                                            AdvertisingTrackingEnabled,
                                            #[serde(rename = "htmlId")]
                                            #[doc = ""]
                                            HtmlId,
                                            #[serde(rename = "environmentName")]
                                            #[doc = ""]
                                            EnvironmentName,
                                            #[serde(rename = "ampBrowserLanguage")]
                                            #[doc = ""]
                                            AmpBrowserLanguage,
                                            #[serde(rename = "ampCanonicalPath")]
                                            #[doc = ""]
                                            AmpCanonicalPath,
                                            #[serde(rename = "ampCanonicalUrl")]
                                            #[doc = ""]
                                            AmpCanonicalUrl,
                                            #[serde(rename = "ampCanonicalHost")]
                                            #[doc = ""]
                                            AmpCanonicalHost,
                                            #[serde(rename = "ampReferrer")]
                                            #[doc = ""]
                                            AmpReferrer,
                                            #[serde(rename = "ampTitle")]
                                            #[doc = ""]
                                            AmpTitle,
                                            #[serde(rename = "ampClientId")]
                                            #[doc = ""]
                                            AmpClientId,
                                            #[serde(rename = "ampClientTimezone")]
                                            #[doc = ""]
                                            AmpClientTimezone,
                                            #[serde(rename = "ampClientTimestamp")]
                                            #[doc = ""]
                                            AmpClientTimestamp,
                                            #[serde(rename = "ampClientScreenWidth")]
                                            #[doc = ""]
                                            AmpClientScreenWidth,
                                            #[serde(rename = "ampClientScreenHeight")]
                                            #[doc = ""]
                                            AmpClientScreenHeight,
                                            #[serde(rename = "ampClientScrollX")]
                                            #[doc = ""]
                                            AmpClientScrollX,
                                            #[serde(rename = "ampClientScrollY")]
                                            #[doc = ""]
                                            AmpClientScrollY,
                                            #[serde(rename = "ampClientMaxScrollX")]
                                            #[doc = ""]
                                            AmpClientMaxScrollX,
                                            #[serde(rename = "ampClientMaxScrollY")]
                                            #[doc = ""]
                                            AmpClientMaxScrollY,
                                            #[serde(rename = "ampTotalEngagedTime")]
                                            #[doc = ""]
                                            AmpTotalEngagedTime,
                                            #[serde(rename = "ampPageViewId")]
                                            #[doc = ""]
                                            AmpPageViewId,
                                            #[serde(rename = "ampPageLoadTime")]
                                            #[doc = ""]
                                            AmpPageLoadTime,
                                            #[serde(rename = "ampPageDownloadTime")]
                                            #[doc = ""]
                                            AmpPageDownloadTime,
                                            #[serde(rename = "ampGtmEvent")]
                                            #[doc = ""]
                                            AmpGtmEvent,
                                            #[serde(rename = "eventName")]
                                            #[doc = ""]
                                            EventName,
                                            #[serde(rename = "firebaseEventParameterCampaign")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaign,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignAclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAclid,
                                            #[serde(rename = "firebaseEventParameterCampaignAnid")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAnid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignClickTimestamp"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignClickTimestamp,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignContent"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignContent,
                                            #[serde(rename = "firebaseEventParameterCampaignCp1")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignCp1,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignGclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignGclid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignSource"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignSource,
                                            #[serde(rename = "firebaseEventParameterCampaignTerm")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignTerm,
                                            #[serde(rename = "firebaseEventParameterCurrency")]
                                            #[doc = ""]
                                            FirebaseEventParameterCurrency,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkAcceptTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkAcceptTime,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkLinkid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkLinkid,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageDeviceTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageDeviceTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageId"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageId,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageName"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageName,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationTopic"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationTopic,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousAppVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousAppVersion,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousOsVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousOsVersion,
                                            #[serde(rename = "firebaseEventParameterPrice")]
                                            #[doc = ""]
                                            FirebaseEventParameterPrice,
                                            #[serde(rename = "firebaseEventParameterProductId")]
                                            #[doc = ""]
                                            FirebaseEventParameterProductId,
                                            #[serde(rename = "firebaseEventParameterQuantity")]
                                            #[doc = ""]
                                            FirebaseEventParameterQuantity,
                                            #[serde(rename = "firebaseEventParameterValue")]
                                            #[doc = ""]
                                            FirebaseEventParameterValue,
                                            #[serde(rename = "videoProvider")]
                                            #[doc = ""]
                                            VideoProvider,
                                            #[serde(rename = "videoUrl")]
                                            #[doc = ""]
                                            VideoUrl,
                                            #[serde(rename = "videoTitle")]
                                            #[doc = ""]
                                            VideoTitle,
                                            #[serde(rename = "videoDuration")]
                                            #[doc = ""]
                                            VideoDuration,
                                            #[serde(rename = "videoPercent")]
                                            #[doc = ""]
                                            VideoPercent,
                                            #[serde(rename = "videoVisible")]
                                            #[doc = ""]
                                            VideoVisible,
                                            #[serde(rename = "videoStatus")]
                                            #[doc = ""]
                                            VideoStatus,
                                            #[serde(rename = "videoCurrentTime")]
                                            #[doc = ""]
                                            VideoCurrentTime,
                                            #[serde(rename = "scrollDepthThreshold")]
                                            #[doc = ""]
                                            ScrollDepthThreshold,
                                            #[serde(rename = "scrollDepthUnits")]
                                            #[doc = ""]
                                            ScrollDepthUnits,
                                            #[serde(rename = "scrollDepthDirection")]
                                            #[doc = ""]
                                            ScrollDepthDirection,
                                            #[serde(rename = "elementVisibilityRatio")]
                                            #[doc = ""]
                                            ElementVisibilityRatio,
                                            #[serde(rename = "elementVisibilityTime")]
                                            #[doc = ""]
                                            ElementVisibilityTime,
                                            #[serde(rename = "elementVisibilityFirstTime")]
                                            #[doc = ""]
                                            ElementVisibilityFirstTime,
                                            #[serde(rename = "elementVisibilityRecentTime")]
                                            #[doc = ""]
                                            ElementVisibilityRecentTime,
                                            #[serde(rename = "requestPath")]
                                            #[doc = ""]
                                            RequestPath,
                                            #[serde(rename = "requestMethod")]
                                            #[doc = ""]
                                            RequestMethod,
                                            #[serde(rename = "clientName")]
                                            #[doc = ""]
                                            ClientName,
                                            #[serde(rename = "queryString")]
                                            #[doc = ""]
                                            QueryString,
                                        }
                                        impl ::std::default::Default for QueryParametersTypeEnum {
                                            fn default() -> Self {
                                                Self::BuiltInVariableTypeUnspecified
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "type")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The types of built-in variables to delete."]
                                            pub _type:
                                                ::std::option::Option<QueryParametersTypeEnum>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "The types of built-in variables to delete."]
                                        pub enum QueryParametersTypeEnum {
                                            #[serde(rename = "builtInVariableTypeUnspecified")]
                                            #[doc = ""]
                                            BuiltInVariableTypeUnspecified,
                                            #[serde(rename = "pageUrl")]
                                            #[doc = ""]
                                            PageUrl,
                                            #[serde(rename = "pageHostname")]
                                            #[doc = ""]
                                            PageHostname,
                                            #[serde(rename = "pagePath")]
                                            #[doc = ""]
                                            PagePath,
                                            #[serde(rename = "referrer")]
                                            #[doc = ""]
                                            Referrer,
                                            #[serde(rename = "event")]
                                            #[doc = "For web or mobile."]
                                            Event,
                                            #[serde(rename = "clickElement")]
                                            #[doc = ""]
                                            ClickElement,
                                            #[serde(rename = "clickClasses")]
                                            #[doc = ""]
                                            ClickClasses,
                                            #[serde(rename = "clickId")]
                                            #[doc = ""]
                                            ClickId,
                                            #[serde(rename = "clickTarget")]
                                            #[doc = ""]
                                            ClickTarget,
                                            #[serde(rename = "clickUrl")]
                                            #[doc = ""]
                                            ClickUrl,
                                            #[serde(rename = "clickText")]
                                            #[doc = ""]
                                            ClickText,
                                            #[serde(rename = "firstPartyServingUrl")]
                                            #[doc = ""]
                                            FirstPartyServingUrl,
                                            #[serde(rename = "formElement")]
                                            #[doc = ""]
                                            FormElement,
                                            #[serde(rename = "formClasses")]
                                            #[doc = ""]
                                            FormClasses,
                                            #[serde(rename = "formId")]
                                            #[doc = ""]
                                            FormId,
                                            #[serde(rename = "formTarget")]
                                            #[doc = ""]
                                            FormTarget,
                                            #[serde(rename = "formUrl")]
                                            #[doc = ""]
                                            FormUrl,
                                            #[serde(rename = "formText")]
                                            #[doc = ""]
                                            FormText,
                                            #[serde(rename = "errorMessage")]
                                            #[doc = ""]
                                            ErrorMessage,
                                            #[serde(rename = "errorUrl")]
                                            #[doc = ""]
                                            ErrorUrl,
                                            #[serde(rename = "errorLine")]
                                            #[doc = ""]
                                            ErrorLine,
                                            #[serde(rename = "newHistoryUrl")]
                                            #[doc = ""]
                                            NewHistoryUrl,
                                            #[serde(rename = "oldHistoryUrl")]
                                            #[doc = ""]
                                            OldHistoryUrl,
                                            #[serde(rename = "newHistoryFragment")]
                                            #[doc = ""]
                                            NewHistoryFragment,
                                            #[serde(rename = "oldHistoryFragment")]
                                            #[doc = ""]
                                            OldHistoryFragment,
                                            #[serde(rename = "newHistoryState")]
                                            #[doc = ""]
                                            NewHistoryState,
                                            #[serde(rename = "oldHistoryState")]
                                            #[doc = ""]
                                            OldHistoryState,
                                            #[serde(rename = "historySource")]
                                            #[doc = ""]
                                            HistorySource,
                                            #[serde(rename = "containerVersion")]
                                            #[doc = "For web or mobile."]
                                            ContainerVersion,
                                            #[serde(rename = "debugMode")]
                                            #[doc = ""]
                                            DebugMode,
                                            #[serde(rename = "randomNumber")]
                                            #[doc = "For web or mobile."]
                                            RandomNumber,
                                            #[serde(rename = "containerId")]
                                            #[doc = "For web or mobile."]
                                            ContainerId,
                                            #[serde(rename = "appId")]
                                            #[doc = ""]
                                            AppId,
                                            #[serde(rename = "appName")]
                                            #[doc = ""]
                                            AppName,
                                            #[serde(rename = "appVersionCode")]
                                            #[doc = ""]
                                            AppVersionCode,
                                            #[serde(rename = "appVersionName")]
                                            #[doc = ""]
                                            AppVersionName,
                                            #[serde(rename = "language")]
                                            #[doc = ""]
                                            Language,
                                            #[serde(rename = "osVersion")]
                                            #[doc = ""]
                                            OsVersion,
                                            #[serde(rename = "platform")]
                                            #[doc = ""]
                                            Platform,
                                            #[serde(rename = "sdkVersion")]
                                            #[doc = ""]
                                            SdkVersion,
                                            #[serde(rename = "deviceName")]
                                            #[doc = ""]
                                            DeviceName,
                                            #[serde(rename = "resolution")]
                                            #[doc = ""]
                                            Resolution,
                                            #[serde(rename = "advertiserId")]
                                            #[doc = ""]
                                            AdvertiserId,
                                            #[serde(rename = "advertisingTrackingEnabled")]
                                            #[doc = ""]
                                            AdvertisingTrackingEnabled,
                                            #[serde(rename = "htmlId")]
                                            #[doc = ""]
                                            HtmlId,
                                            #[serde(rename = "environmentName")]
                                            #[doc = ""]
                                            EnvironmentName,
                                            #[serde(rename = "ampBrowserLanguage")]
                                            #[doc = ""]
                                            AmpBrowserLanguage,
                                            #[serde(rename = "ampCanonicalPath")]
                                            #[doc = ""]
                                            AmpCanonicalPath,
                                            #[serde(rename = "ampCanonicalUrl")]
                                            #[doc = ""]
                                            AmpCanonicalUrl,
                                            #[serde(rename = "ampCanonicalHost")]
                                            #[doc = ""]
                                            AmpCanonicalHost,
                                            #[serde(rename = "ampReferrer")]
                                            #[doc = ""]
                                            AmpReferrer,
                                            #[serde(rename = "ampTitle")]
                                            #[doc = ""]
                                            AmpTitle,
                                            #[serde(rename = "ampClientId")]
                                            #[doc = ""]
                                            AmpClientId,
                                            #[serde(rename = "ampClientTimezone")]
                                            #[doc = ""]
                                            AmpClientTimezone,
                                            #[serde(rename = "ampClientTimestamp")]
                                            #[doc = ""]
                                            AmpClientTimestamp,
                                            #[serde(rename = "ampClientScreenWidth")]
                                            #[doc = ""]
                                            AmpClientScreenWidth,
                                            #[serde(rename = "ampClientScreenHeight")]
                                            #[doc = ""]
                                            AmpClientScreenHeight,
                                            #[serde(rename = "ampClientScrollX")]
                                            #[doc = ""]
                                            AmpClientScrollX,
                                            #[serde(rename = "ampClientScrollY")]
                                            #[doc = ""]
                                            AmpClientScrollY,
                                            #[serde(rename = "ampClientMaxScrollX")]
                                            #[doc = ""]
                                            AmpClientMaxScrollX,
                                            #[serde(rename = "ampClientMaxScrollY")]
                                            #[doc = ""]
                                            AmpClientMaxScrollY,
                                            #[serde(rename = "ampTotalEngagedTime")]
                                            #[doc = ""]
                                            AmpTotalEngagedTime,
                                            #[serde(rename = "ampPageViewId")]
                                            #[doc = ""]
                                            AmpPageViewId,
                                            #[serde(rename = "ampPageLoadTime")]
                                            #[doc = ""]
                                            AmpPageLoadTime,
                                            #[serde(rename = "ampPageDownloadTime")]
                                            #[doc = ""]
                                            AmpPageDownloadTime,
                                            #[serde(rename = "ampGtmEvent")]
                                            #[doc = ""]
                                            AmpGtmEvent,
                                            #[serde(rename = "eventName")]
                                            #[doc = ""]
                                            EventName,
                                            #[serde(rename = "firebaseEventParameterCampaign")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaign,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignAclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAclid,
                                            #[serde(rename = "firebaseEventParameterCampaignAnid")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAnid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignClickTimestamp"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignClickTimestamp,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignContent"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignContent,
                                            #[serde(rename = "firebaseEventParameterCampaignCp1")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignCp1,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignGclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignGclid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignSource"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignSource,
                                            #[serde(rename = "firebaseEventParameterCampaignTerm")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignTerm,
                                            #[serde(rename = "firebaseEventParameterCurrency")]
                                            #[doc = ""]
                                            FirebaseEventParameterCurrency,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkAcceptTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkAcceptTime,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkLinkid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkLinkid,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageDeviceTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageDeviceTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageId"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageId,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageName"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageName,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationTopic"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationTopic,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousAppVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousAppVersion,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousOsVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousOsVersion,
                                            #[serde(rename = "firebaseEventParameterPrice")]
                                            #[doc = ""]
                                            FirebaseEventParameterPrice,
                                            #[serde(rename = "firebaseEventParameterProductId")]
                                            #[doc = ""]
                                            FirebaseEventParameterProductId,
                                            #[serde(rename = "firebaseEventParameterQuantity")]
                                            #[doc = ""]
                                            FirebaseEventParameterQuantity,
                                            #[serde(rename = "firebaseEventParameterValue")]
                                            #[doc = ""]
                                            FirebaseEventParameterValue,
                                            #[serde(rename = "videoProvider")]
                                            #[doc = ""]
                                            VideoProvider,
                                            #[serde(rename = "videoUrl")]
                                            #[doc = ""]
                                            VideoUrl,
                                            #[serde(rename = "videoTitle")]
                                            #[doc = ""]
                                            VideoTitle,
                                            #[serde(rename = "videoDuration")]
                                            #[doc = ""]
                                            VideoDuration,
                                            #[serde(rename = "videoPercent")]
                                            #[doc = ""]
                                            VideoPercent,
                                            #[serde(rename = "videoVisible")]
                                            #[doc = ""]
                                            VideoVisible,
                                            #[serde(rename = "videoStatus")]
                                            #[doc = ""]
                                            VideoStatus,
                                            #[serde(rename = "videoCurrentTime")]
                                            #[doc = ""]
                                            VideoCurrentTime,
                                            #[serde(rename = "scrollDepthThreshold")]
                                            #[doc = ""]
                                            ScrollDepthThreshold,
                                            #[serde(rename = "scrollDepthUnits")]
                                            #[doc = ""]
                                            ScrollDepthUnits,
                                            #[serde(rename = "scrollDepthDirection")]
                                            #[doc = ""]
                                            ScrollDepthDirection,
                                            #[serde(rename = "elementVisibilityRatio")]
                                            #[doc = ""]
                                            ElementVisibilityRatio,
                                            #[serde(rename = "elementVisibilityTime")]
                                            #[doc = ""]
                                            ElementVisibilityTime,
                                            #[serde(rename = "elementVisibilityFirstTime")]
                                            #[doc = ""]
                                            ElementVisibilityFirstTime,
                                            #[serde(rename = "elementVisibilityRecentTime")]
                                            #[doc = ""]
                                            ElementVisibilityRecentTime,
                                            #[serde(rename = "requestPath")]
                                            #[doc = ""]
                                            RequestPath,
                                            #[serde(rename = "requestMethod")]
                                            #[doc = ""]
                                            RequestMethod,
                                            #[serde(rename = "clientName")]
                                            #[doc = ""]
                                            ClientName,
                                            #[serde(rename = "queryString")]
                                            #[doc = ""]
                                            QueryString,
                                        }
                                        impl ::std::default::Default for QueryParametersTypeEnum {
                                            fn default() -> Self {
                                                Self::BuiltInVariableTypeUnspecified
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "type")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The type of built-in variable to revert."]
                                            pub _type:
                                                ::std::option::Option<QueryParametersTypeEnum>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                        #[derive(
                                            Debug,
                                            PartialEq,
                                            Copy,
                                            Clone,
                                            serde :: Serialize,
                                            serde :: Deserialize,
                                        )]
                                        #[doc = "The type of built-in variable to revert."]
                                        pub enum QueryParametersTypeEnum {
                                            #[serde(rename = "builtInVariableTypeUnspecified")]
                                            #[doc = ""]
                                            BuiltInVariableTypeUnspecified,
                                            #[serde(rename = "pageUrl")]
                                            #[doc = ""]
                                            PageUrl,
                                            #[serde(rename = "pageHostname")]
                                            #[doc = ""]
                                            PageHostname,
                                            #[serde(rename = "pagePath")]
                                            #[doc = ""]
                                            PagePath,
                                            #[serde(rename = "referrer")]
                                            #[doc = ""]
                                            Referrer,
                                            #[serde(rename = "event")]
                                            #[doc = "For web or mobile."]
                                            Event,
                                            #[serde(rename = "clickElement")]
                                            #[doc = ""]
                                            ClickElement,
                                            #[serde(rename = "clickClasses")]
                                            #[doc = ""]
                                            ClickClasses,
                                            #[serde(rename = "clickId")]
                                            #[doc = ""]
                                            ClickId,
                                            #[serde(rename = "clickTarget")]
                                            #[doc = ""]
                                            ClickTarget,
                                            #[serde(rename = "clickUrl")]
                                            #[doc = ""]
                                            ClickUrl,
                                            #[serde(rename = "clickText")]
                                            #[doc = ""]
                                            ClickText,
                                            #[serde(rename = "firstPartyServingUrl")]
                                            #[doc = ""]
                                            FirstPartyServingUrl,
                                            #[serde(rename = "formElement")]
                                            #[doc = ""]
                                            FormElement,
                                            #[serde(rename = "formClasses")]
                                            #[doc = ""]
                                            FormClasses,
                                            #[serde(rename = "formId")]
                                            #[doc = ""]
                                            FormId,
                                            #[serde(rename = "formTarget")]
                                            #[doc = ""]
                                            FormTarget,
                                            #[serde(rename = "formUrl")]
                                            #[doc = ""]
                                            FormUrl,
                                            #[serde(rename = "formText")]
                                            #[doc = ""]
                                            FormText,
                                            #[serde(rename = "errorMessage")]
                                            #[doc = ""]
                                            ErrorMessage,
                                            #[serde(rename = "errorUrl")]
                                            #[doc = ""]
                                            ErrorUrl,
                                            #[serde(rename = "errorLine")]
                                            #[doc = ""]
                                            ErrorLine,
                                            #[serde(rename = "newHistoryUrl")]
                                            #[doc = ""]
                                            NewHistoryUrl,
                                            #[serde(rename = "oldHistoryUrl")]
                                            #[doc = ""]
                                            OldHistoryUrl,
                                            #[serde(rename = "newHistoryFragment")]
                                            #[doc = ""]
                                            NewHistoryFragment,
                                            #[serde(rename = "oldHistoryFragment")]
                                            #[doc = ""]
                                            OldHistoryFragment,
                                            #[serde(rename = "newHistoryState")]
                                            #[doc = ""]
                                            NewHistoryState,
                                            #[serde(rename = "oldHistoryState")]
                                            #[doc = ""]
                                            OldHistoryState,
                                            #[serde(rename = "historySource")]
                                            #[doc = ""]
                                            HistorySource,
                                            #[serde(rename = "containerVersion")]
                                            #[doc = "For web or mobile."]
                                            ContainerVersion,
                                            #[serde(rename = "debugMode")]
                                            #[doc = ""]
                                            DebugMode,
                                            #[serde(rename = "randomNumber")]
                                            #[doc = "For web or mobile."]
                                            RandomNumber,
                                            #[serde(rename = "containerId")]
                                            #[doc = "For web or mobile."]
                                            ContainerId,
                                            #[serde(rename = "appId")]
                                            #[doc = ""]
                                            AppId,
                                            #[serde(rename = "appName")]
                                            #[doc = ""]
                                            AppName,
                                            #[serde(rename = "appVersionCode")]
                                            #[doc = ""]
                                            AppVersionCode,
                                            #[serde(rename = "appVersionName")]
                                            #[doc = ""]
                                            AppVersionName,
                                            #[serde(rename = "language")]
                                            #[doc = ""]
                                            Language,
                                            #[serde(rename = "osVersion")]
                                            #[doc = ""]
                                            OsVersion,
                                            #[serde(rename = "platform")]
                                            #[doc = ""]
                                            Platform,
                                            #[serde(rename = "sdkVersion")]
                                            #[doc = ""]
                                            SdkVersion,
                                            #[serde(rename = "deviceName")]
                                            #[doc = ""]
                                            DeviceName,
                                            #[serde(rename = "resolution")]
                                            #[doc = ""]
                                            Resolution,
                                            #[serde(rename = "advertiserId")]
                                            #[doc = ""]
                                            AdvertiserId,
                                            #[serde(rename = "advertisingTrackingEnabled")]
                                            #[doc = ""]
                                            AdvertisingTrackingEnabled,
                                            #[serde(rename = "htmlId")]
                                            #[doc = ""]
                                            HtmlId,
                                            #[serde(rename = "environmentName")]
                                            #[doc = ""]
                                            EnvironmentName,
                                            #[serde(rename = "ampBrowserLanguage")]
                                            #[doc = ""]
                                            AmpBrowserLanguage,
                                            #[serde(rename = "ampCanonicalPath")]
                                            #[doc = ""]
                                            AmpCanonicalPath,
                                            #[serde(rename = "ampCanonicalUrl")]
                                            #[doc = ""]
                                            AmpCanonicalUrl,
                                            #[serde(rename = "ampCanonicalHost")]
                                            #[doc = ""]
                                            AmpCanonicalHost,
                                            #[serde(rename = "ampReferrer")]
                                            #[doc = ""]
                                            AmpReferrer,
                                            #[serde(rename = "ampTitle")]
                                            #[doc = ""]
                                            AmpTitle,
                                            #[serde(rename = "ampClientId")]
                                            #[doc = ""]
                                            AmpClientId,
                                            #[serde(rename = "ampClientTimezone")]
                                            #[doc = ""]
                                            AmpClientTimezone,
                                            #[serde(rename = "ampClientTimestamp")]
                                            #[doc = ""]
                                            AmpClientTimestamp,
                                            #[serde(rename = "ampClientScreenWidth")]
                                            #[doc = ""]
                                            AmpClientScreenWidth,
                                            #[serde(rename = "ampClientScreenHeight")]
                                            #[doc = ""]
                                            AmpClientScreenHeight,
                                            #[serde(rename = "ampClientScrollX")]
                                            #[doc = ""]
                                            AmpClientScrollX,
                                            #[serde(rename = "ampClientScrollY")]
                                            #[doc = ""]
                                            AmpClientScrollY,
                                            #[serde(rename = "ampClientMaxScrollX")]
                                            #[doc = ""]
                                            AmpClientMaxScrollX,
                                            #[serde(rename = "ampClientMaxScrollY")]
                                            #[doc = ""]
                                            AmpClientMaxScrollY,
                                            #[serde(rename = "ampTotalEngagedTime")]
                                            #[doc = ""]
                                            AmpTotalEngagedTime,
                                            #[serde(rename = "ampPageViewId")]
                                            #[doc = ""]
                                            AmpPageViewId,
                                            #[serde(rename = "ampPageLoadTime")]
                                            #[doc = ""]
                                            AmpPageLoadTime,
                                            #[serde(rename = "ampPageDownloadTime")]
                                            #[doc = ""]
                                            AmpPageDownloadTime,
                                            #[serde(rename = "ampGtmEvent")]
                                            #[doc = ""]
                                            AmpGtmEvent,
                                            #[serde(rename = "eventName")]
                                            #[doc = ""]
                                            EventName,
                                            #[serde(rename = "firebaseEventParameterCampaign")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaign,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignAclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAclid,
                                            #[serde(rename = "firebaseEventParameterCampaignAnid")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignAnid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignClickTimestamp"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignClickTimestamp,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignContent"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignContent,
                                            #[serde(rename = "firebaseEventParameterCampaignCp1")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignCp1,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignGclid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignGclid,
                                            #[serde(
                                                rename = "firebaseEventParameterCampaignSource"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignSource,
                                            #[serde(rename = "firebaseEventParameterCampaignTerm")]
                                            #[doc = ""]
                                            FirebaseEventParameterCampaignTerm,
                                            #[serde(rename = "firebaseEventParameterCurrency")]
                                            #[doc = ""]
                                            FirebaseEventParameterCurrency,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkAcceptTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkAcceptTime,
                                            #[serde(
                                                rename = "firebaseEventParameterDynamicLinkLinkid"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterDynamicLinkLinkid,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageDeviceTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageDeviceTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageId"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageId,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageName"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageName,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationMessageTime"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationMessageTime,
                                            #[serde(
                                                rename = "firebaseEventParameterNotificationTopic"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterNotificationTopic,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousAppVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousAppVersion,
                                            #[serde(
                                                rename = "firebaseEventParameterPreviousOsVersion"
                                            )]
                                            #[doc = ""]
                                            FirebaseEventParameterPreviousOsVersion,
                                            #[serde(rename = "firebaseEventParameterPrice")]
                                            #[doc = ""]
                                            FirebaseEventParameterPrice,
                                            #[serde(rename = "firebaseEventParameterProductId")]
                                            #[doc = ""]
                                            FirebaseEventParameterProductId,
                                            #[serde(rename = "firebaseEventParameterQuantity")]
                                            #[doc = ""]
                                            FirebaseEventParameterQuantity,
                                            #[serde(rename = "firebaseEventParameterValue")]
                                            #[doc = ""]
                                            FirebaseEventParameterValue,
                                            #[serde(rename = "videoProvider")]
                                            #[doc = ""]
                                            VideoProvider,
                                            #[serde(rename = "videoUrl")]
                                            #[doc = ""]
                                            VideoUrl,
                                            #[serde(rename = "videoTitle")]
                                            #[doc = ""]
                                            VideoTitle,
                                            #[serde(rename = "videoDuration")]
                                            #[doc = ""]
                                            VideoDuration,
                                            #[serde(rename = "videoPercent")]
                                            #[doc = ""]
                                            VideoPercent,
                                            #[serde(rename = "videoVisible")]
                                            #[doc = ""]
                                            VideoVisible,
                                            #[serde(rename = "videoStatus")]
                                            #[doc = ""]
                                            VideoStatus,
                                            #[serde(rename = "videoCurrentTime")]
                                            #[doc = ""]
                                            VideoCurrentTime,
                                            #[serde(rename = "scrollDepthThreshold")]
                                            #[doc = ""]
                                            ScrollDepthThreshold,
                                            #[serde(rename = "scrollDepthUnits")]
                                            #[doc = ""]
                                            ScrollDepthUnits,
                                            #[serde(rename = "scrollDepthDirection")]
                                            #[doc = ""]
                                            ScrollDepthDirection,
                                            #[serde(rename = "elementVisibilityRatio")]
                                            #[doc = ""]
                                            ElementVisibilityRatio,
                                            #[serde(rename = "elementVisibilityTime")]
                                            #[doc = ""]
                                            ElementVisibilityTime,
                                            #[serde(rename = "elementVisibilityFirstTime")]
                                            #[doc = ""]
                                            ElementVisibilityFirstTime,
                                            #[serde(rename = "elementVisibilityRecentTime")]
                                            #[doc = ""]
                                            ElementVisibilityRecentTime,
                                            #[serde(rename = "requestPath")]
                                            #[doc = ""]
                                            RequestPath,
                                            #[serde(rename = "requestMethod")]
                                            #[doc = ""]
                                            RequestMethod,
                                            #[serde(rename = "clientName")]
                                            #[doc = ""]
                                            ClientName,
                                            #[serde(rename = "queryString")]
                                            #[doc = ""]
                                            QueryString,
                                        }
                                        impl ::std::default::Default for QueryParametersTypeEnum {
                                            fn default() -> Self {
                                                Self::BuiltInVariableTypeUnspecified
                                            }
                                        }
                                    }
                                }
                            }
                            pub mod folders {
                                pub mod methods {
                                    pub mod entities {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod move_entities_to_folder {
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
                                            #[serde(rename = "tagId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The tags to be moved to the folder."]
                                            pub tag_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "triggerId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The triggers to be moved to the folder."]
                                            pub trigger_id:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "variableId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The variables to be moved to the folder."]
                                            pub variable_id:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the tag in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the folder in storage."]
                                            pub fingerprint:
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
                            pub mod tags {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of thetag in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the tag in storage."]
                                            pub fingerprint:
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
                            pub mod templates {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the template in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the templates in storage."]
                                            pub fingerprint:
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
                            pub mod triggers {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the trigger in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the trigger in storage."]
                                            pub fingerprint:
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
                            pub mod variables {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the variable in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the variable in storage."]
                                            pub fingerprint:
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
                            pub mod zones {
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
                                            #[serde(rename = "pageToken")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Continuation token for fetching the next page of results."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod revert {
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
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the zone in storage."]
                                            pub fingerprint:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "fingerprint")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "When provided, this fingerprint must match the fingerprint of the zone in storage."]
                                            pub fingerprint:
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
            pub mod user_permissions {
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
                            #[serde(rename = "pageToken")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "Continuation token for fetching the next page of results."]
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
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Account."]
    pub struct Account {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Account ID uniquely identifies the GTM Account."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Account as computed at storage time. This value is recomputed whenever the account is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Account display name. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shareData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the account shares data anonymously with Google and others. This flag enables benchmarking by sharing your data in an anonymous form. Google will remove all identifiable information about your website, combine the data with hundreds of other anonymous sites and report aggregate trends in the benchmarking service. @mutable tagmanager.accounts.create @mutable tagmanager.accounts.update"]
        pub share_data: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
    }
    impl Account {
        pub fn builder() -> AccountBuilder {
            AccountBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the Google Tag Manager Account access permissions."]
    pub struct AccountAccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the user has no access, user access, or admin access to an account. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
        pub permission: ::std::option::Option<AccountAccessPermissionEnum>,
    }
    impl AccountAccess {
        pub fn builder() -> AccountAccessBuilder {
            AccountAccessBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the user has no access, user access, or admin access to an account. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub enum AccountAccessPermissionEnum {
        #[serde(rename = "accountPermissionUnspecified")]
        #[doc = ""]
        AccountPermissionUnspecified,
        #[serde(rename = "noAccess")]
        #[doc = ""]
        NoAccess,
        #[serde(rename = "user")]
        #[doc = ""]
        User,
        #[serde(rename = "admin")]
        #[doc = ""]
        Admin,
    }
    impl ::std::default::Default for AccountAccessPermissionEnum {
        fn default() -> Self {
            Self::AccountPermissionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Built-in variables are a special category of variables that are pre-created and non-customizable. They provide common functionality like accessing propeties of the gtm data layer, monitoring clicks, or accessing elements of a page URL."]
    pub struct BuiltInVariable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the built-in variable to be used to refer to the built-in variable."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM BuiltInVariable's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of built-in variable. @required.tagmanager.accounts.containers.workspaces.built_in_variable.update @mutable tagmanager.accounts.containers.workspaces.built_in_variable.update"]
        pub _type: ::std::option::Option<BuiltInVariableTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl BuiltInVariable {
        pub fn builder() -> BuiltInVariableBuilder {
            BuiltInVariableBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of built-in variable. @required.tagmanager.accounts.containers.workspaces.built_in_variable.update @mutable tagmanager.accounts.containers.workspaces.built_in_variable.update"]
    pub enum BuiltInVariableTypeEnum {
        #[serde(rename = "builtInVariableTypeUnspecified")]
        #[doc = ""]
        BuiltInVariableTypeUnspecified,
        #[serde(rename = "pageUrl")]
        #[doc = ""]
        PageUrl,
        #[serde(rename = "pageHostname")]
        #[doc = ""]
        PageHostname,
        #[serde(rename = "pagePath")]
        #[doc = ""]
        PagePath,
        #[serde(rename = "referrer")]
        #[doc = ""]
        Referrer,
        #[serde(rename = "event")]
        #[doc = "For web or mobile."]
        Event,
        #[serde(rename = "clickElement")]
        #[doc = ""]
        ClickElement,
        #[serde(rename = "clickClasses")]
        #[doc = ""]
        ClickClasses,
        #[serde(rename = "clickId")]
        #[doc = ""]
        ClickId,
        #[serde(rename = "clickTarget")]
        #[doc = ""]
        ClickTarget,
        #[serde(rename = "clickUrl")]
        #[doc = ""]
        ClickUrl,
        #[serde(rename = "clickText")]
        #[doc = ""]
        ClickText,
        #[serde(rename = "firstPartyServingUrl")]
        #[doc = ""]
        FirstPartyServingUrl,
        #[serde(rename = "formElement")]
        #[doc = ""]
        FormElement,
        #[serde(rename = "formClasses")]
        #[doc = ""]
        FormClasses,
        #[serde(rename = "formId")]
        #[doc = ""]
        FormId,
        #[serde(rename = "formTarget")]
        #[doc = ""]
        FormTarget,
        #[serde(rename = "formUrl")]
        #[doc = ""]
        FormUrl,
        #[serde(rename = "formText")]
        #[doc = ""]
        FormText,
        #[serde(rename = "errorMessage")]
        #[doc = ""]
        ErrorMessage,
        #[serde(rename = "errorUrl")]
        #[doc = ""]
        ErrorUrl,
        #[serde(rename = "errorLine")]
        #[doc = ""]
        ErrorLine,
        #[serde(rename = "newHistoryUrl")]
        #[doc = ""]
        NewHistoryUrl,
        #[serde(rename = "oldHistoryUrl")]
        #[doc = ""]
        OldHistoryUrl,
        #[serde(rename = "newHistoryFragment")]
        #[doc = ""]
        NewHistoryFragment,
        #[serde(rename = "oldHistoryFragment")]
        #[doc = ""]
        OldHistoryFragment,
        #[serde(rename = "newHistoryState")]
        #[doc = ""]
        NewHistoryState,
        #[serde(rename = "oldHistoryState")]
        #[doc = ""]
        OldHistoryState,
        #[serde(rename = "historySource")]
        #[doc = ""]
        HistorySource,
        #[serde(rename = "containerVersion")]
        #[doc = "For web or mobile."]
        ContainerVersion,
        #[serde(rename = "debugMode")]
        #[doc = ""]
        DebugMode,
        #[serde(rename = "randomNumber")]
        #[doc = "For web or mobile."]
        RandomNumber,
        #[serde(rename = "containerId")]
        #[doc = "For web or mobile."]
        ContainerId,
        #[serde(rename = "appId")]
        #[doc = ""]
        AppId,
        #[serde(rename = "appName")]
        #[doc = ""]
        AppName,
        #[serde(rename = "appVersionCode")]
        #[doc = ""]
        AppVersionCode,
        #[serde(rename = "appVersionName")]
        #[doc = ""]
        AppVersionName,
        #[serde(rename = "language")]
        #[doc = ""]
        Language,
        #[serde(rename = "osVersion")]
        #[doc = ""]
        OsVersion,
        #[serde(rename = "platform")]
        #[doc = ""]
        Platform,
        #[serde(rename = "sdkVersion")]
        #[doc = ""]
        SdkVersion,
        #[serde(rename = "deviceName")]
        #[doc = ""]
        DeviceName,
        #[serde(rename = "resolution")]
        #[doc = ""]
        Resolution,
        #[serde(rename = "advertiserId")]
        #[doc = ""]
        AdvertiserId,
        #[serde(rename = "advertisingTrackingEnabled")]
        #[doc = ""]
        AdvertisingTrackingEnabled,
        #[serde(rename = "htmlId")]
        #[doc = ""]
        HtmlId,
        #[serde(rename = "environmentName")]
        #[doc = ""]
        EnvironmentName,
        #[serde(rename = "ampBrowserLanguage")]
        #[doc = ""]
        AmpBrowserLanguage,
        #[serde(rename = "ampCanonicalPath")]
        #[doc = ""]
        AmpCanonicalPath,
        #[serde(rename = "ampCanonicalUrl")]
        #[doc = ""]
        AmpCanonicalUrl,
        #[serde(rename = "ampCanonicalHost")]
        #[doc = ""]
        AmpCanonicalHost,
        #[serde(rename = "ampReferrer")]
        #[doc = ""]
        AmpReferrer,
        #[serde(rename = "ampTitle")]
        #[doc = ""]
        AmpTitle,
        #[serde(rename = "ampClientId")]
        #[doc = ""]
        AmpClientId,
        #[serde(rename = "ampClientTimezone")]
        #[doc = ""]
        AmpClientTimezone,
        #[serde(rename = "ampClientTimestamp")]
        #[doc = ""]
        AmpClientTimestamp,
        #[serde(rename = "ampClientScreenWidth")]
        #[doc = ""]
        AmpClientScreenWidth,
        #[serde(rename = "ampClientScreenHeight")]
        #[doc = ""]
        AmpClientScreenHeight,
        #[serde(rename = "ampClientScrollX")]
        #[doc = ""]
        AmpClientScrollX,
        #[serde(rename = "ampClientScrollY")]
        #[doc = ""]
        AmpClientScrollY,
        #[serde(rename = "ampClientMaxScrollX")]
        #[doc = ""]
        AmpClientMaxScrollX,
        #[serde(rename = "ampClientMaxScrollY")]
        #[doc = ""]
        AmpClientMaxScrollY,
        #[serde(rename = "ampTotalEngagedTime")]
        #[doc = ""]
        AmpTotalEngagedTime,
        #[serde(rename = "ampPageViewId")]
        #[doc = ""]
        AmpPageViewId,
        #[serde(rename = "ampPageLoadTime")]
        #[doc = ""]
        AmpPageLoadTime,
        #[serde(rename = "ampPageDownloadTime")]
        #[doc = ""]
        AmpPageDownloadTime,
        #[serde(rename = "ampGtmEvent")]
        #[doc = ""]
        AmpGtmEvent,
        #[serde(rename = "eventName")]
        #[doc = ""]
        EventName,
        #[serde(rename = "firebaseEventParameterCampaign")]
        #[doc = ""]
        FirebaseEventParameterCampaign,
        #[serde(rename = "firebaseEventParameterCampaignAclid")]
        #[doc = ""]
        FirebaseEventParameterCampaignAclid,
        #[serde(rename = "firebaseEventParameterCampaignAnid")]
        #[doc = ""]
        FirebaseEventParameterCampaignAnid,
        #[serde(rename = "firebaseEventParameterCampaignClickTimestamp")]
        #[doc = ""]
        FirebaseEventParameterCampaignClickTimestamp,
        #[serde(rename = "firebaseEventParameterCampaignContent")]
        #[doc = ""]
        FirebaseEventParameterCampaignContent,
        #[serde(rename = "firebaseEventParameterCampaignCp1")]
        #[doc = ""]
        FirebaseEventParameterCampaignCp1,
        #[serde(rename = "firebaseEventParameterCampaignGclid")]
        #[doc = ""]
        FirebaseEventParameterCampaignGclid,
        #[serde(rename = "firebaseEventParameterCampaignSource")]
        #[doc = ""]
        FirebaseEventParameterCampaignSource,
        #[serde(rename = "firebaseEventParameterCampaignTerm")]
        #[doc = ""]
        FirebaseEventParameterCampaignTerm,
        #[serde(rename = "firebaseEventParameterCurrency")]
        #[doc = ""]
        FirebaseEventParameterCurrency,
        #[serde(rename = "firebaseEventParameterDynamicLinkAcceptTime")]
        #[doc = ""]
        FirebaseEventParameterDynamicLinkAcceptTime,
        #[serde(rename = "firebaseEventParameterDynamicLinkLinkid")]
        #[doc = ""]
        FirebaseEventParameterDynamicLinkLinkid,
        #[serde(rename = "firebaseEventParameterNotificationMessageDeviceTime")]
        #[doc = ""]
        FirebaseEventParameterNotificationMessageDeviceTime,
        #[serde(rename = "firebaseEventParameterNotificationMessageId")]
        #[doc = ""]
        FirebaseEventParameterNotificationMessageId,
        #[serde(rename = "firebaseEventParameterNotificationMessageName")]
        #[doc = ""]
        FirebaseEventParameterNotificationMessageName,
        #[serde(rename = "firebaseEventParameterNotificationMessageTime")]
        #[doc = ""]
        FirebaseEventParameterNotificationMessageTime,
        #[serde(rename = "firebaseEventParameterNotificationTopic")]
        #[doc = ""]
        FirebaseEventParameterNotificationTopic,
        #[serde(rename = "firebaseEventParameterPreviousAppVersion")]
        #[doc = ""]
        FirebaseEventParameterPreviousAppVersion,
        #[serde(rename = "firebaseEventParameterPreviousOsVersion")]
        #[doc = ""]
        FirebaseEventParameterPreviousOsVersion,
        #[serde(rename = "firebaseEventParameterPrice")]
        #[doc = ""]
        FirebaseEventParameterPrice,
        #[serde(rename = "firebaseEventParameterProductId")]
        #[doc = ""]
        FirebaseEventParameterProductId,
        #[serde(rename = "firebaseEventParameterQuantity")]
        #[doc = ""]
        FirebaseEventParameterQuantity,
        #[serde(rename = "firebaseEventParameterValue")]
        #[doc = ""]
        FirebaseEventParameterValue,
        #[serde(rename = "videoProvider")]
        #[doc = ""]
        VideoProvider,
        #[serde(rename = "videoUrl")]
        #[doc = ""]
        VideoUrl,
        #[serde(rename = "videoTitle")]
        #[doc = ""]
        VideoTitle,
        #[serde(rename = "videoDuration")]
        #[doc = ""]
        VideoDuration,
        #[serde(rename = "videoPercent")]
        #[doc = ""]
        VideoPercent,
        #[serde(rename = "videoVisible")]
        #[doc = ""]
        VideoVisible,
        #[serde(rename = "videoStatus")]
        #[doc = ""]
        VideoStatus,
        #[serde(rename = "videoCurrentTime")]
        #[doc = ""]
        VideoCurrentTime,
        #[serde(rename = "scrollDepthThreshold")]
        #[doc = ""]
        ScrollDepthThreshold,
        #[serde(rename = "scrollDepthUnits")]
        #[doc = ""]
        ScrollDepthUnits,
        #[serde(rename = "scrollDepthDirection")]
        #[doc = ""]
        ScrollDepthDirection,
        #[serde(rename = "elementVisibilityRatio")]
        #[doc = ""]
        ElementVisibilityRatio,
        #[serde(rename = "elementVisibilityTime")]
        #[doc = ""]
        ElementVisibilityTime,
        #[serde(rename = "elementVisibilityFirstTime")]
        #[doc = ""]
        ElementVisibilityFirstTime,
        #[serde(rename = "elementVisibilityRecentTime")]
        #[doc = ""]
        ElementVisibilityRecentTime,
        #[serde(rename = "requestPath")]
        #[doc = ""]
        RequestPath,
        #[serde(rename = "requestMethod")]
        #[doc = ""]
        RequestMethod,
        #[serde(rename = "clientName")]
        #[doc = ""]
        ClientName,
        #[serde(rename = "queryString")]
        #[doc = ""]
        QueryString,
    }
    impl ::std::default::Default for BuiltInVariableTypeEnum {
        fn default() -> Self {
            Self::BuiltInVariableTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Client {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clientId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Client ID uniquely identifies the GTM client."]
        pub client_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Client as computed at storage time. This value is recomputed whenever the client is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client display name. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client's parameters. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFolderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent folder id."]
        pub parent_folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM client's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Priority determines relative firing order. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update"]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client type. @mutable tagmanager.accounts.containers.workspaces.clients.create @mutable tagmanager.accounts.containers.workspaces.clients.update"]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Client {
        pub fn builder() -> ClientBuilder {
            ClientBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a predicate."]
    pub struct Condition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of named parameters (key/value), depending on the condition's type. Notes: - For binary operators, include parameters named arg0 and arg1 for specifying the left and right operands, respectively. - At this time, the left operand (arg0) must be a reference to a variable. - For case-insensitive Regex matching, include a boolean parameter named ignore_case that is set to true. If not specified or set to any other value, the matching will be case sensitive. - To negate an operator, include a boolean parameter named negate boolean parameter that is set to true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of operator for this condition. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub _type: ::std::option::Option<ConditionTypeEnum>,
    }
    impl Condition {
        pub fn builder() -> ConditionBuilder {
            ConditionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of operator for this condition. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
    pub enum ConditionTypeEnum {
        #[serde(rename = "conditionTypeUnspecified")]
        #[doc = ""]
        ConditionTypeUnspecified,
        #[serde(rename = "equals")]
        #[doc = ""]
        Equals,
        #[serde(rename = "contains")]
        #[doc = ""]
        Contains,
        #[serde(rename = "startsWith")]
        #[doc = ""]
        StartsWith,
        #[serde(rename = "endsWith")]
        #[doc = ""]
        EndsWith,
        #[serde(rename = "matchRegex")]
        #[doc = ""]
        MatchRegex,
        #[serde(rename = "greater")]
        #[doc = ""]
        Greater,
        #[serde(rename = "greaterOrEquals")]
        #[doc = ""]
        GreaterOrEquals,
        #[serde(rename = "less")]
        #[doc = ""]
        Less,
        #[serde(rename = "lessOrEquals")]
        #[doc = ""]
        LessOrEquals,
        #[serde(rename = "cssSelector")]
        #[doc = ""]
        CssSelector,
        #[serde(rename = "urlMatches")]
        #[doc = "NOTE(lanzone): When defining a ConditionType here, don't forget to also define a matching PredicateType (in condition.proto)."]
        UrlMatches,
    }
    impl ::std::default::Default for ConditionTypeEnum {
        fn default() -> Self {
            Self::ConditionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Container, which specifies the platform tags will run on, manages workspaces, and retains container versions."]
    pub struct Container {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Container ID uniquely identifies the GTM Container."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "domainName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of domain names associated with the Container. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
        pub domain_name: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Container as computed at storage time. This value is recomputed whenever the account is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container display name. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container Notes. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container Public ID."]
        pub public_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "usageContext")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Usage Contexts for the Container. Valid values include: web, android, or ios. @mutable tagmanager.accounts.containers.create @mutable tagmanager.accounts.containers.update"]
        pub usage_context: ::std::option::Option<::std::vec::Vec<ContainerUsageContextEnum>>,
    }
    impl Container {
        pub fn builder() -> ContainerBuilder {
            ContainerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ContainerUsageContextEnum {
        #[serde(rename = "usageContextUnspecified")]
        #[doc = ""]
        UsageContextUnspecified,
        #[serde(rename = "web")]
        #[doc = ""]
        Web,
        #[serde(rename = "android")]
        #[doc = ""]
        Android,
        #[serde(rename = "ios")]
        #[doc = ""]
        Ios,
        #[serde(rename = "androidSdk5")]
        #[doc = ""]
        AndroidSdk5,
        #[serde(rename = "iosSdk5")]
        #[doc = ""]
        IosSdk5,
        #[serde(rename = "amp")]
        #[doc = ""]
        Amp,
        #[serde(rename = "server")]
        #[doc = ""]
        Server,
    }
    impl ::std::default::Default for ContainerUsageContextEnum {
        fn default() -> Self {
            Self::UsageContextUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Defines the Google Tag Manager Container access permissions."]
    pub struct ContainerAccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of Container permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
        pub permission: ::std::option::Option<ContainerAccessPermissionEnum>,
    }
    impl ContainerAccess {
        pub fn builder() -> ContainerAccessBuilder {
            ContainerAccessBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "List of Container permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
    pub enum ContainerAccessPermissionEnum {
        #[serde(rename = "containerPermissionUnspecified")]
        #[doc = ""]
        ContainerPermissionUnspecified,
        #[serde(rename = "noAccess")]
        #[doc = ""]
        NoAccess,
        #[serde(rename = "read")]
        #[doc = ""]
        Read,
        #[serde(rename = "edit")]
        #[doc = ""]
        Edit,
        #[serde(rename = "approve")]
        #[doc = ""]
        Approve,
        #[serde(rename = "publish")]
        #[doc = ""]
        Publish,
    }
    impl ::std::default::Default for ContainerAccessPermissionEnum {
        fn default() -> Self {
            Self::ContainerPermissionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Container Version."]
    pub struct ContainerVersion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builtInVariable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The built-in variables in the container that this version was taken from."]
        pub built_in_variable:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BuiltInVariable>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "client")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The clients in the container that this version was taken from."]
        pub client: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Client>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container that this version was taken from."]
        pub container: ::std::option::Option<::std::boxed::Box<Container>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
        pub container_version_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom templates in the container that this version was taken from."]
        pub custom_template:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomTemplate>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value of true indicates this container version has been deleted."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container version description. @mutable tagmanager.accounts.containers.versions.update"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Container Version as computed at storage time. This value is recomputed whenever the container version is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The folders in the container that this version was taken from."]
        pub folder: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Folder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container version display name. @mutable tagmanager.accounts.containers.versions.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM ContainerVersions's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tags in the container that this version was taken from."]
        pub tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The triggers in the container that this version was taken from."]
        pub trigger: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variables in the container that this version was taken from."]
        pub variable: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zones in the container that this version was taken from."]
        pub zone: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Zone>>>,
    }
    impl ContainerVersion {
        pub fn builder() -> ContainerVersionBuilder {
            ContainerVersionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Container Version Header."]
    pub struct ContainerVersionHeader {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Container Version ID uniquely identifies the GTM Container Version."]
        pub container_version_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deleted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value of true indicates this container version has been deleted."]
        pub deleted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Container version display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numClients")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of clients in the container version."]
        pub num_clients: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numCustomTemplates")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of custom templates in the container version."]
        pub num_custom_templates: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numMacros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of macros in the container version."]
        pub num_macros: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numRules")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of rules in the container version."]
        pub num_rules: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of tags in the container version."]
        pub num_tags: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numTriggers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of triggers in the container version."]
        pub num_triggers: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numVariables")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of variables in the container version."]
        pub num_variables: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "numZones")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of zones in the container version."]
        pub num_zones: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container Versions's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl ContainerVersionHeader {
        pub fn builder() -> ContainerVersionHeaderBuilder {
            ContainerVersionHeaderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CreateBuiltInVariableResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builtInVariable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of created built-in variables."]
        pub built_in_variable:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BuiltInVariable>>>,
    }
    impl CreateBuiltInVariableResponse {
        pub fn builder() -> CreateBuiltInVariableResponseBuilder {
            CreateBuiltInVariableResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Options for new container versions."]
    pub struct CreateContainerVersionRequestVersionOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the container version to be created."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The notes of the container version to be created."]
        pub notes: ::std::option::Option<::std::string::String>,
    }
    impl CreateContainerVersionRequestVersionOptions {
        pub fn builder() -> CreateContainerVersionRequestVersionOptionsBuilder {
            CreateContainerVersionRequestVersionOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Create container versions response."]
    pub struct CreateContainerVersionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compilerError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compiler errors or not."]
        pub compiler_error: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container version created."]
        pub container_version: ::std::option::Option<::std::boxed::Box<ContainerVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newWorkspacePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated workspace path created as a result of version creation. This field should only be populated if the created version was not a quick preview."]
        pub new_workspace_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syncStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether version creation failed when syncing the workspace to the latest container version."]
        pub sync_status: ::std::option::Option<::std::boxed::Box<SyncStatus>>,
    }
    impl CreateContainerVersionResponse {
        pub fn builder() -> CreateContainerVersionResponseBuilder {
            CreateContainerVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Custom Template's contents."]
    pub struct CustomTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Custom Template as computed at storage time. This value is recomputed whenever the template is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "galleryReference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A reference to the Community Template Gallery entry."]
        pub gallery_reference: ::std::option::Option<::std::boxed::Box<GalleryReference>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom Template display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Custom Template's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom template in text format."]
        pub template_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Custom Template ID uniquely identifies the GTM custom template."]
        pub template_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl CustomTemplate {
        pub fn builder() -> CustomTemplateBuilder {
            CustomTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A workspace entity that may represent a tag, trigger, variable, or folder in addition to its status in the workspace."]
    pub struct Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "changeStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents how the entity has been changed in the workspace."]
        pub change_status: ::std::option::Option<EntityChangeStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "client")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The client being represented by the entity."]
        pub client: ::std::option::Option<::std::boxed::Box<Client>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The folder being represented by the entity."]
        pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tag being represented by the entity."]
        pub tag: ::std::option::Option<::std::boxed::Box<Tag>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The trigger being represented by the entity."]
        pub trigger: ::std::option::Option<::std::boxed::Box<Trigger>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variable being represented by the entity."]
        pub variable: ::std::option::Option<::std::boxed::Box<Variable>>,
    }
    impl Entity {
        pub fn builder() -> EntityBuilder {
            EntityBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents how the entity has been changed in the workspace."]
    pub enum EntityChangeStatusEnum {
        #[serde(rename = "changeStatusUnspecified")]
        #[doc = ""]
        ChangeStatusUnspecified,
        #[serde(rename = "none")]
        #[doc = "The entity has never been changed."]
        None,
        #[serde(rename = "added")]
        #[doc = "The entity is added to the workspace."]
        Added,
        #[serde(rename = "deleted")]
        #[doc = "The entity is deleted from the workspace."]
        Deleted,
        #[serde(rename = "updated")]
        #[doc = "The entity has been updated in the workspace."]
        Updated,
    }
    impl ::std::default::Default for EntityChangeStatusEnum {
        fn default() -> Self {
            Self::ChangeStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Environment. Note that a user can create, delete and update environments of type USER, but can only update the enable_debug and url fields of environments of other types."]
    pub struct Environment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizationCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment authorization code."]
        pub authorization_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorizationTimestamp")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last update time-stamp for the authorization code."]
        pub authorization_timestamp: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersionId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a link to a container version."]
        pub container_version_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment description. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableDebug")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not to enable debug by default for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
        pub enable_debug: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environmentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Environment ID uniquely identifies the GTM Environment."]
        pub environment_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM environment as computed at storage time. This value is recomputed whenever the environment is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The environment display name. Can be set or changed only on USER type environments. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Environment's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of this environment."]
        pub _type: ::std::option::Option<EnvironmentTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Default preview page url for the environment. @mutable tagmanager.accounts.containers.environments.create @mutable tagmanager.accounts.containers.environments.update"]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a link to a quick preview of a workspace."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Environment {
        pub fn builder() -> EnvironmentBuilder {
            EnvironmentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of this environment."]
    pub enum EnvironmentTypeEnum {
        #[serde(rename = "user")]
        #[doc = "Points to a user defined environment."]
        User,
        #[serde(rename = "live")]
        #[doc = "Points to the current live container version."]
        Live,
        #[serde(rename = "latest")]
        #[doc = "Points to the latest container version."]
        Latest,
        #[serde(rename = "workspace")]
        #[doc = "Automatically managed environment that points to a workspace preview or version created by a workspace."]
        Workspace,
    }
    impl ::std::default::Default for EnvironmentTypeEnum {
        fn default() -> Self {
            Self::User
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Folder."]
    pub struct Folder {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Folder as computed at storage time. This value is recomputed whenever the folder is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Folder ID uniquely identifies the GTM Folder."]
        pub folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Folder display name. @mutable tagmanager.accounts.containers.workspaces.folders.create @mutable tagmanager.accounts.containers.workspaces.folders.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this folder in the container. @mutable tagmanager.accounts.containers.workspaces.folders.create @mutable tagmanager.accounts.containers.workspaces.folders.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Folder's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Folder {
        pub fn builder() -> FolderBuilder {
            FolderBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Folder's contents."]
    pub struct FolderEntities {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of tags inside the folder."]
        pub tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of triggers inside the folder."]
        pub trigger: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of variables inside the folder."]
        pub variable: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    }
    impl FolderEntities {
        pub fn builder() -> FolderEntitiesBuilder {
            FolderEntitiesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the link between a custom template and an entry on the Community Template Gallery site."]
    pub struct GalleryReference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "host")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the host for the community gallery template."]
        pub host: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isModified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If a user has manually edited the community gallery template."]
        pub is_modified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "owner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the owner for the community gallery template."]
        pub owner: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repository")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the repository for the community gallery template."]
        pub repository: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "signature")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The signature of the community gallery template as computed at import time. This value is recomputed whenever the template is updated from the gallery."]
        pub signature: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The version of the community gallery template."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GalleryReference {
        pub fn builder() -> GalleryReferenceBuilder {
            GalleryReferenceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The changes that have occurred in the workspace since the base container version."]
    pub struct GetWorkspaceStatusResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeConflict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merge conflict after sync."]
        pub merge_conflict:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MergeConflict>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceChange")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entities that have been changed in the workspace."]
        pub workspace_change: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Entity>>>,
    }
    impl GetWorkspaceStatusResponse {
        pub fn builder() -> GetWorkspaceStatusResponseBuilder {
            GetWorkspaceStatusResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Accounts Response."]
    pub struct ListAccountsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "account")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of GTM Accounts that a user has access to."]
        pub account: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Account>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListAccountsResponse {
        pub fn builder() -> ListAccountsResponseBuilder {
            ListAccountsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List container versions response."]
    pub struct ListContainerVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersionHeader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All container version headers of a GTM Container."]
        pub container_version_header:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerVersionHeader>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListContainerVersionsResponse {
        pub fn builder() -> ListContainerVersionsResponseBuilder {
            ListContainerVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Containers Response."]
    pub struct ListContainersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All Containers of a GTM Account."]
        pub container: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Container>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListContainersResponse {
        pub fn builder() -> ListContainersResponseBuilder {
            ListContainersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of enabled built-in variables."]
    pub struct ListEnabledBuiltInVariablesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "builtInVariable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM BuiltInVariables of a GTM container."]
        pub built_in_variable:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<BuiltInVariable>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListEnabledBuiltInVariablesResponse {
        pub fn builder() -> ListEnabledBuiltInVariablesResponseBuilder {
            ListEnabledBuiltInVariablesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Environments Response."]
    pub struct ListEnvironmentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All Environments of a GTM Container."]
        pub environment: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Environment>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListEnvironmentsResponse {
        pub fn builder() -> ListEnvironmentsResponseBuilder {
            ListEnvironmentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Folders Response."]
    pub struct ListFoldersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Folders of a GTM Container."]
        pub folder: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Folder>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl ListFoldersResponse {
        pub fn builder() -> ListFoldersResponseBuilder {
            ListFoldersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Tags Response."]
    pub struct ListTagsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Tags of a GTM Container."]
        pub tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Tag>>>,
    }
    impl ListTagsResponse {
        pub fn builder() -> ListTagsResponseBuilder {
            ListTagsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListTemplatesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Custom Templates of a GTM Container."]
        pub template: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CustomTemplate>>>,
    }
    impl ListTemplatesResponse {
        pub fn builder() -> ListTemplatesResponseBuilder {
            ListTemplatesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List triggers response."]
    pub struct ListTriggersResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Triggers of a GTM Container."]
        pub trigger: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Trigger>>>,
    }
    impl ListTriggersResponse {
        pub fn builder() -> ListTriggersResponseBuilder {
            ListTriggersResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List user permissions response."]
    pub struct ListUserPermissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userPermission")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM UserPermissions of a GTM Account."]
        pub user_permission:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<UserPermission>>>,
    }
    impl ListUserPermissionsResponse {
        pub fn builder() -> ListUserPermissionsResponseBuilder {
            ListUserPermissionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "List Variables Response."]
    pub struct ListVariablesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Variables of a GTM Container."]
        pub variable: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Variable>>>,
    }
    impl ListVariablesResponse {
        pub fn builder() -> ListVariablesResponseBuilder {
            ListVariablesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of workspaces in a container."]
    pub struct ListWorkspacesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All Workspaces of a GTM Container."]
        pub workspace: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Workspace>>>,
    }
    impl ListWorkspacesResponse {
        pub fn builder() -> ListWorkspacesResponseBuilder {
            ListWorkspacesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ListZonesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Continuation token for fetching the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "All GTM Zones of a GTM Container."]
        pub zone: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Zone>>>,
    }
    impl ListZonesResponse {
        pub fn builder() -> ListZonesResponseBuilder {
            ListZonesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a merge conflict."]
    pub struct MergeConflict {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityInBaseVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The base version entity (since the latest sync operation) that has conflicting changes compared to the workspace. If this field is missing, it means the workspace entity is deleted from the base version."]
        pub entity_in_base_version: ::std::option::Option<::std::boxed::Box<Entity>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityInWorkspace")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The workspace entity that has conflicting changes compared to the base version. If an entity is deleted in a workspace, it will still appear with a deleted change status."]
        pub entity_in_workspace: ::std::option::Option<::std::boxed::Box<Entity>>,
    }
    impl MergeConflict {
        pub fn builder() -> MergeConflictBuilder {
            MergeConflictBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Parameter."]
    pub struct Parameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The named key that uniquely identifies a parameter. Required for top-level parameters, as well as map values. Ignored for list values. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "list")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This list parameter's parameters (keys will be ignored). @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub list: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "map")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This map parameter's parameters (must have keys; keys must be unique). @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub map: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub _type: ::std::option::Option<ParameterTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A parameter's value (may contain variable references such as \"{{myVariable}}\") as appropriate to the specified type. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl Parameter {
        pub fn builder() -> ParameterBuilder {
            ParameterBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The parameter type. Valid values are: - boolean: The value represents a boolean, represented as 'true' or 'false' - integer: The value represents a 64-bit signed integer value, in base 10 - list: A list of parameters should be specified - map: A map of parameters should be specified - template: The value represents any text; this can include variable references (even variable references that might return non-string types) - trigger_reference: The value represents a trigger, represented as the trigger id - tag_reference: The value represents a tag, represented as the tag name @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
    pub enum ParameterTypeEnum {
        #[serde(rename = "typeUnspecified")]
        #[doc = ""]
        TypeUnspecified,
        #[serde(rename = "template")]
        #[doc = "May include variable references (such as \"{{myVariable}}\")."]
        Template,
        #[serde(rename = "integer")]
        #[doc = ""]
        Integer,
        #[serde(rename = "boolean")]
        #[doc = ""]
        Boolean,
        #[serde(rename = "list")]
        #[doc = ""]
        List,
        #[serde(rename = "map")]
        #[doc = ""]
        Map,
        #[serde(rename = "triggerReference")]
        #[doc = ""]
        TriggerReference,
        #[serde(rename = "tagReference")]
        #[doc = ""]
        TagReference,
    }
    impl ::std::default::Default for ParameterTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Publish container version response."]
    pub struct PublishContainerVersionResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compilerError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Compiler errors or not."]
        pub compiler_error: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The container version created."]
        pub container_version: ::std::option::Option<::std::boxed::Box<ContainerVersion>>,
    }
    impl PublishContainerVersionResponse {
        pub fn builder() -> PublishContainerVersionResponseBuilder {
            PublishContainerVersionResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response to quick previewing a workspace."]
    pub struct QuickPreviewResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "compilerError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Were there compiler errors or not."]
        pub compiler_error: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quick previewed container version."]
        pub container_version: ::std::option::Option<::std::boxed::Box<ContainerVersion>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syncStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether quick previewing failed when syncing the workspace to the latest container version."]
        pub sync_status: ::std::option::Option<::std::boxed::Box<SyncStatus>>,
    }
    impl QuickPreviewResponse {
        pub fn builder() -> QuickPreviewResponseBuilder {
            QuickPreviewResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a built-in variable in a workspace."]
    pub struct RevertBuiltInVariableResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the built-in variable is enabled after reversion."]
        pub enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl RevertBuiltInVariableResponse {
        pub fn builder() -> RevertBuiltInVariableResponseBuilder {
            RevertBuiltInVariableResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting folder changes in a workspace."]
    pub struct RevertFolderResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "folder")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Folder as it appears in the latest container version since the last workspace synchronization operation. If no folder is present, that means the folder was deleted in the latest container version."]
        pub folder: ::std::option::Option<::std::boxed::Box<Folder>>,
    }
    impl RevertFolderResponse {
        pub fn builder() -> RevertFolderResponseBuilder {
            RevertFolderResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a tag in a workspace."]
    pub struct RevertTagResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tag as it appears in the latest container version since the last workspace synchronization operation. If no tag is present, that means the tag was deleted in the latest container version."]
        pub tag: ::std::option::Option<::std::boxed::Box<Tag>>,
    }
    impl RevertTagResponse {
        pub fn builder() -> RevertTagResponseBuilder {
            RevertTagResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a template in a workspace."]
    pub struct RevertTemplateResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Template as it appears in the latest container version since the last workspace synchronization operation. If no template is present, that means the template was deleted in the latest container version."]
        pub template: ::std::option::Option<::std::boxed::Box<CustomTemplate>>,
    }
    impl RevertTemplateResponse {
        pub fn builder() -> RevertTemplateResponseBuilder {
            RevertTemplateResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a trigger in a workspace."]
    pub struct RevertTriggerResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trigger")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trigger as it appears in the latest container version since the last workspace synchronization operation. If no trigger is present, that means the trigger was deleted in the latest container version."]
        pub trigger: ::std::option::Option<::std::boxed::Box<Trigger>>,
    }
    impl RevertTriggerResponse {
        pub fn builder() -> RevertTriggerResponseBuilder {
            RevertTriggerResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a variable in a workspace."]
    pub struct RevertVariableResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variable as it appears in the latest container version since the last workspace synchronization operation. If no variable is present, that means the variable was deleted in the latest container version."]
        pub variable: ::std::option::Option<::std::boxed::Box<Variable>>,
    }
    impl RevertVariableResponse {
        pub fn builder() -> RevertVariableResponseBuilder {
            RevertVariableResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of reverting a zone in a workspace."]
    pub struct RevertZoneResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zone as it appears in the latest container version since the last workspace synchronization operation. If no zone is present, that means the zone was deleted in the latest container version."]
        pub zone: ::std::option::Option<::std::boxed::Box<Zone>>,
    }
    impl RevertZoneResponse {
        pub fn builder() -> RevertZoneResponseBuilder {
            RevertZoneResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a reference to atag that fires before another tag in order to set up dependencies."]
    pub struct SetupTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stopOnSetupFailure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, fire the main tag if and only if the setup tag fires successfully. If false, fire the main tag regardless of setup tag firing status."]
        pub stop_on_setup_failure: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the setup tag."]
        pub tag_name: ::std::option::Option<::std::string::String>,
    }
    impl SetupTag {
        pub fn builder() -> SetupTagBuilder {
            SetupTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The status of a workspace after synchronization."]
    pub struct SyncStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeConflict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Synchornization operation detected a merge conflict."]
        pub merge_conflict: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syncError")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An error occurred during the synchronization operation."]
        pub sync_error: ::std::option::Option<::std::primitive::bool>,
    }
    impl SyncStatus {
        pub fn builder() -> SyncStatusBuilder {
            SyncStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A response after synchronizing the workspace to the latest container version."]
    pub struct SyncWorkspaceResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeConflict")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The merge conflict after sync. If this field is not empty, the sync is still treated as successful. But a version cannot be created until all conflicts are resolved."]
        pub merge_conflict:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MergeConflict>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "syncStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether synchronization caused a merge conflict or sync error."]
        pub sync_status: ::std::option::Option<::std::boxed::Box<SyncStatus>>,
    }
    impl SyncWorkspaceResponse {
        pub fn builder() -> SyncWorkspaceResponseBuilder {
            SyncWorkspaceResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Tag."]
    pub struct Tag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingRuleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blocking rule IDs. If any of the listed rules evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub blocking_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blockingTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Blocking trigger IDs. If any of the listed triggers evaluate to true, the tag will not fire. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub blocking_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Tag as computed at storage time. This value is recomputed whenever the tag is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firingRuleId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Firing rule IDs. A tag will fire when any of the listed rules are true and all of its blockingRuleIds (if any specified) are false. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub firing_rule_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "firingTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Firing trigger IDs. A tag will fire when any of the listed triggers are true and all of its blockingTriggerIds (if any specified) are false. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub firing_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveOnly")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If set to true, this tag will only fire in the live environment (e.g. not in preview or debug mode). @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub live_only: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoringMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of key-value pairs of tag metadata to be included in the event data for tag monitoring. Notes: - This parameter must be type MAP. - Each parameter in the map are type TEMPLATE, however cannot contain variable references. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub monitoring_metadata: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitoringMetadataTagNameKey")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If non-empty, then the tag display name will be included in the monitoring metadata map using the key specified. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub monitoring_metadata_tag_name_key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tag display name. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this tag in the container. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tag's parameters. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFolderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent folder id."]
        pub parent_folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Tag's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "paused")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the tag is paused, which prevents the tag from firing. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub paused: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User defined numeric priority of the tag. Tags are fired asynchronously in order of priority. Tags with higher numeric value fire first. A tag's priority can be a positive or negative value. The default value is 0. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub priority: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleEndMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub schedule_end_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleStartMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start timestamp in milliseconds to schedule a tag. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub schedule_start_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setupTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of setup tags. Currently we only allow one."]
        pub setup_tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SetupTag>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagFiringOption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to fire this tag."]
        pub tag_firing_option: ::std::option::Option<TagTagFiringOptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Tag ID uniquely identifies the GTM Tag."]
        pub tag_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "teardownTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of teardown tags. Currently we only allow one."]
        pub teardown_tag: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<TeardownTag>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Tag Type. @mutable tagmanager.accounts.containers.workspaces.tags.create @mutable tagmanager.accounts.containers.workspaces.tags.update"]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Tag {
        pub fn builder() -> TagBuilder {
            TagBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Option to fire this tag."]
    pub enum TagTagFiringOptionEnum {
        #[serde(rename = "tagFiringOptionUnspecified")]
        #[doc = ""]
        TagFiringOptionUnspecified,
        #[serde(rename = "unlimited")]
        #[doc = "Tag can be fired multiple times per event."]
        Unlimited,
        #[serde(rename = "oncePerEvent")]
        #[doc = "Tag can only be fired per event but can be fired multiple times per load (e.g., app load or page load)."]
        OncePerEvent,
        #[serde(rename = "oncePerLoad")]
        #[doc = "Tag can only be fired per load (e.g., app load or page load)."]
        OncePerLoad,
    }
    impl ::std::default::Default for TagTagFiringOptionEnum {
        fn default() -> Self {
            Self::TagFiringOptionUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a tag that fires after another tag in order to tear down dependencies."]
    pub struct TeardownTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stopTeardownOnFailure")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, fire the teardown tag if and only if the main tag fires successfully. If false, fire the teardown tag regardless of main tag firing status."]
        pub stop_teardown_on_failure: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the teardown tag."]
        pub tag_name: ::std::option::Option<::std::string::String>,
    }
    impl TeardownTag {
        pub fn builder() -> TeardownTagBuilder {
            TeardownTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Trigger"]
    pub struct Trigger {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoEventFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used in the case of auto event tracking. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub auto_event_filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "checkValidation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not we should only fire tags if the form submit or link click event is not cancelled by some other event handler (e.g. because of validation). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub check_validation: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "continuousTimeMinMilliseconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A visibility trigger minimum continuous visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub continuous_time_min_milliseconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customEventFilter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Used in the case of custom event, which is fired iff all Conditions are true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub custom_event_filter:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the GTM event that is fired. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub event_name: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The trigger will only fire iff all Conditions are true. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub filter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Trigger as computed at storage time. This value is recomputed whenever the trigger is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalScrollPercentageList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled horizontally. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub horizontal_scroll_percentage_list: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "interval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time between triggering recurring Timer Events (in milliseconds). Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub interval: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intervalSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Time between Timer Events to fire (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub interval_seconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "limit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Limit of the number of GTM events this Timer Trigger will fire. If no limit is set, we will continue to fire GTM events until the user leaves the page. Only valid for Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub limit: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxTimerLengthSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Max time to fire Timer Events (in seconds). Only valid for AMP Timer trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub max_timer_length_seconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Trigger display name. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this trigger in the container. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional parameters. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFolderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent folder id."]
        pub parent_folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Trigger's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A click trigger CSS selector (i.e. \"a\", \"button\" etc.). Only valid for AMP Click trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub selector: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalTimeMinMilliseconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A visibility trigger minimum total visible time (in milliseconds). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub total_time_min_milliseconds: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Trigger ID uniquely identifies the GTM Trigger."]
        pub trigger_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub _type: ::std::option::Option<TriggerTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uniqueTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Globally unique id of the trigger that auto-generates this (a Form Submit, Link Click or Timer listener) if any. Used to make incompatible auto-events work together with trigger filtering based on trigger ids. This value is populated during output generation since the tags implied by triggers don't exist until then. Only valid for Form Submit, Link Click and Timer triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub unique_trigger_id: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "verticalScrollPercentageList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of integer percentage values for scroll triggers. The trigger will fire when each percentage is reached when the view is scrolled vertically. Only valid for AMP scroll triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub vertical_scroll_percentage_list: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visibilitySelector")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A visibility trigger CSS selector (i.e. \"#id\"). Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub visibility_selector: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visiblePercentageMax")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A visibility trigger maximum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub visible_percentage_max: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visiblePercentageMin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A visibility trigger minimum percent visibility. Only valid for AMP Visibility trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub visible_percentage_min: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitForTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not we should delay the form submissions or link opening until all of the tags have fired (by preventing the default action and later simulating the default action). Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub wait_for_tags: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "waitForTagsTimeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How long to wait (in milliseconds) for tags to fire when 'waits_for_tags' above evaluates to true. Only valid for Form Submission and Link Click triggers. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
        pub wait_for_tags_timeout: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Trigger {
        pub fn builder() -> TriggerBuilder {
            TriggerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Defines the data layer event that causes this trigger. @mutable tagmanager.accounts.containers.workspaces.triggers.create @mutable tagmanager.accounts.containers.workspaces.triggers.update"]
    pub enum TriggerTypeEnum {
        #[serde(rename = "eventTypeUnspecified")]
        #[doc = ""]
        EventTypeUnspecified,
        #[serde(rename = "pageview")]
        #[doc = ""]
        Pageview,
        #[serde(rename = "domReady")]
        #[doc = ""]
        DomReady,
        #[serde(rename = "windowLoaded")]
        #[doc = ""]
        WindowLoaded,
        #[serde(rename = "customEvent")]
        #[doc = ""]
        CustomEvent,
        #[serde(rename = "triggerGroup")]
        #[doc = ""]
        TriggerGroup,
        #[serde(rename = "always")]
        #[doc = ""]
        Always,
        #[serde(rename = "firebaseAppException")]
        #[doc = ""]
        FirebaseAppException,
        #[serde(rename = "firebaseAppUpdate")]
        #[doc = ""]
        FirebaseAppUpdate,
        #[serde(rename = "firebaseCampaign")]
        #[doc = ""]
        FirebaseCampaign,
        #[serde(rename = "firebaseFirstOpen")]
        #[doc = ""]
        FirebaseFirstOpen,
        #[serde(rename = "firebaseInAppPurchase")]
        #[doc = ""]
        FirebaseInAppPurchase,
        #[serde(rename = "firebaseNotificationDismiss")]
        #[doc = ""]
        FirebaseNotificationDismiss,
        #[serde(rename = "firebaseNotificationForeground")]
        #[doc = ""]
        FirebaseNotificationForeground,
        #[serde(rename = "firebaseNotificationOpen")]
        #[doc = ""]
        FirebaseNotificationOpen,
        #[serde(rename = "firebaseNotificationReceive")]
        #[doc = ""]
        FirebaseNotificationReceive,
        #[serde(rename = "firebaseOsUpdate")]
        #[doc = ""]
        FirebaseOsUpdate,
        #[serde(rename = "firebaseSessionStart")]
        #[doc = ""]
        FirebaseSessionStart,
        #[serde(rename = "firebaseUserEngagement")]
        #[doc = ""]
        FirebaseUserEngagement,
        #[serde(rename = "formSubmission")]
        #[doc = ""]
        FormSubmission,
        #[serde(rename = "click")]
        #[doc = ""]
        Click,
        #[serde(rename = "linkClick")]
        #[doc = ""]
        LinkClick,
        #[serde(rename = "jsError")]
        #[doc = ""]
        JsError,
        #[serde(rename = "historyChange")]
        #[doc = ""]
        HistoryChange,
        #[serde(rename = "timer")]
        #[doc = ""]
        Timer,
        #[serde(rename = "ampClick")]
        #[doc = ""]
        AmpClick,
        #[serde(rename = "ampTimer")]
        #[doc = ""]
        AmpTimer,
        #[serde(rename = "ampScroll")]
        #[doc = ""]
        AmpScroll,
        #[serde(rename = "ampVisibility")]
        #[doc = ""]
        AmpVisibility,
        #[serde(rename = "youTubeVideo")]
        #[doc = ""]
        YouTubeVideo,
        #[serde(rename = "scrollDepth")]
        #[doc = ""]
        ScrollDepth,
        #[serde(rename = "elementVisibility")]
        #[doc = ""]
        ElementVisibility,
    }
    impl ::std::default::Default for TriggerTypeEnum {
        fn default() -> Self {
            Self::EventTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a user's permissions to an account and its container."]
    pub struct UserPermission {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
        pub account_access: ::std::option::Option<::std::boxed::Box<AccountAccess>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Account ID uniquely identifies the GTM Account."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerAccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container access permissions. @mutable tagmanager.accounts.permissions.create @mutable tagmanager.accounts.permissions.update"]
        pub container_access:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ContainerAccess>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "emailAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User's email address. @mutable tagmanager.accounts.permissions.create"]
        pub email_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM UserPermission's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
    }
    impl UserPermission {
        pub fn builder() -> UserPermissionBuilder {
            UserPermissionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Variable."]
    pub struct Variable {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disablingTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For mobile containers only: A list of trigger IDs for disabling conditional variables; the variable is enabled if one of the enabling trigger is true while all the disabling trigger are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub disabling_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enablingTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "For mobile containers only: A list of trigger IDs for enabling conditional variables; the variable is enabled if one of the enabling triggers is true while all the disabling triggers are false. Treated as an unordered set. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub enabling_trigger_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Variable as computed at storage time. This value is recomputed whenever the variable is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formatValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Option to convert a variable value to other value."]
        pub format_value: ::std::option::Option<::std::boxed::Box<VariableFormatValue>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Variable display name. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this variable in the container. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The variable's parameters. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub parameter: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Parameter>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFolderId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parent folder id."]
        pub parent_folder_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Variable's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleEndMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub schedule_end_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduleStartMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start timestamp in milliseconds to schedule a variable. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub schedule_start_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Variable Type. @mutable tagmanager.accounts.containers.workspaces.variables.create @mutable tagmanager.accounts.containers.workspaces.variables.update"]
        pub _type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variableId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Variable ID uniquely identifies the GTM Variable."]
        pub variable_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Variable {
        pub fn builder() -> VariableBuilder {
            VariableBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VariableFormatValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseConversionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The option to convert a string-type variable value to either lowercase or uppercase."]
        pub case_conversion_type: ::std::option::Option<VariableFormatValueCaseConversionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "convertFalseToValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to convert if a variable value is false."]
        pub convert_false_to_value: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "convertNullToValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to convert if a variable value is null."]
        pub convert_null_to_value: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "convertTrueToValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to convert if a variable value is true."]
        pub convert_true_to_value: ::std::option::Option<::std::boxed::Box<Parameter>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "convertUndefinedToValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value to convert if a variable value is undefined."]
        pub convert_undefined_to_value: ::std::option::Option<::std::boxed::Box<Parameter>>,
    }
    impl VariableFormatValue {
        pub fn builder() -> VariableFormatValueBuilder {
            VariableFormatValueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The option to convert a string-type variable value to either lowercase or uppercase."]
    pub enum VariableFormatValueCaseConversionTypeEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "lowercase")]
        #[doc = "The option to convert a variable value to lowercase."]
        Lowercase,
        #[serde(rename = "uppercase")]
        #[doc = "The option to convert a variable value to uppercase."]
        Uppercase,
    }
    impl ::std::default::Default for VariableFormatValueCaseConversionTypeEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Container Workspace."]
    pub struct Workspace {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Workspace description. @mutable tagmanager.accounts.containers.workspaces.create @mutable tagmanager.accounts.containers.workspaces.update"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Workspace as computed at storage time. This value is recomputed whenever the workspace is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Workspace display name. @mutable tagmanager.accounts.containers.workspaces.create @mutable tagmanager.accounts.containers.workspaces.update"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Workspace ID uniquely identifies the GTM Workspace."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
    }
    impl Workspace {
        pub fn builder() -> WorkspaceBuilder {
            WorkspaceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Google Tag Manager Zone's contents."]
    pub struct Zone {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Account ID."]
        pub account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundary")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This Zone's boundary."]
        pub boundary: ::std::option::Option<::std::boxed::Box<ZoneBoundary>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childContainer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Containers that are children of this Zone."]
        pub child_container:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ZoneChildContainer>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Container ID."]
        pub container_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fingerprint")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fingerprint of the GTM Zone as computed at storage time. This value is recomputed whenever the zone is modified."]
        pub fingerprint: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zone display name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "User notes on how to apply this zone in the container."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "path")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Zone's API relative path."]
        pub path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagManagerUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Auto generated link to the tag manager UI"]
        pub tag_manager_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This Zone's type restrictions."]
        pub type_restriction: ::std::option::Option<::std::boxed::Box<ZoneTypeRestriction>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "workspaceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "GTM Workspace ID."]
        pub workspace_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "zoneId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Zone ID uniquely identifies the GTM Zone."]
        pub zone_id: ::std::option::Option<::std::string::String>,
    }
    impl Zone {
        pub fn builder() -> ZoneBuilder {
            ZoneBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Zone's boundaries."]
    pub struct ZoneBoundary {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conditions that, when conjoined, make up the boundary."]
        pub condition: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Condition>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customEvaluationTriggerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom evaluation trigger IDs. A zone will evaluate its boundary conditions when any of the listed triggers are true."]
        pub custom_evaluation_trigger_id:
            ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ZoneBoundary {
        pub fn builder() -> ZoneBoundaryBuilder {
            ZoneBoundaryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a child container of a Zone."]
    pub struct ZoneChildContainer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nickname")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The zone's nickname for the child container."]
        pub nickname: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The child container's public id."]
        pub public_id: ::std::option::Option<::std::string::String>,
    }
    impl ZoneChildContainer {
        pub fn builder() -> ZoneChildContainerBuilder {
            ZoneChildContainerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a Zone's type restrictions."]
    pub struct ZoneTypeRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if type restrictions have been enabled for this Zone."]
        pub enable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "whitelistedTypeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of type public ids that have been whitelisted for use in this Zone."]
        pub whitelisted_type_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ZoneTypeRestriction {
        pub fn builder() -> ZoneTypeRestrictionBuilder {
            ZoneTypeRestrictionBuilder::default()
        }
    }
}
