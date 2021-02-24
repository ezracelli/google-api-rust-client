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
    pub mod abuse_reports {
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod activities {
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "home")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub home: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more activity resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in an activity resource, the snippet property contains other properties that identify the type of activity, a display title for the activity, and so forth. If you set *part=snippet*, the API response will also contain all of those nested properties."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publishedAfter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub published_after: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publishedBefore")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub published_before: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "regionCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub region_code: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                }
            }
        }
    }
    pub mod captions {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOf")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the Google+ Page for the channel that the request is be on behalf of"]
                    pub on_behalf_of: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod download {
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
                    #[serde(rename = "onBehalfOf")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the Google+ Page for the channel that the request is be on behalf of"]
                    pub on_behalf_of: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "tfmt")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Convert the captions into this format. Supported options are sbv, srt, and vtt."]
                    pub tfmt: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "tlang")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "tlang is the language code; machine translate the captions into this language."]
                    pub tlang: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOf")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the Google+ Page for the channel that the request is be on behalf of"]
                    pub on_behalf_of: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the caption resource parts that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sync")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extra parameter to allow automatically syncing the uploaded caption/transcript with the audio."]
                    pub sync: ::std::option::Option<::std::primitive::bool>,
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
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the captions with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOf")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the Google+ Page for the channel that the request is on behalf of."]
                    pub on_behalf_of: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoId")]
                    #[doc = "Returns the captions for the specified video."]
                    pub video_id: ::std::string::String,
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
                    #[serde(rename = "onBehalfOf")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "ID of the Google+ Page for the channel that the request is on behalf of."]
                    pub on_behalf_of: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sync")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Extra parameter to allow automatically syncing the uploaded caption/transcript with the audio."]
                    pub sync: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod channel_banners {
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Unused, channel_id is currently derived from the security context of the requestor."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
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
    pub mod channel_sections {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part names that you can include in the parameter value are snippet and contentDetails."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the ChannelSections owned by the specified channel ID."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return content in specified language"]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the ChannelSections with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the ChannelSections owned by the authenticated user."]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set *part=snippet*, the API response will also contain all of those nested properties."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part names that you can include in the parameter value are snippet and contentDetails."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod channels {
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
                    #[serde(rename = "categoryId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the channels within the specified guide category ID."]
                    pub category_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "forUsername")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the channel associated with a YouTube username."]
                    pub for_username: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Stands for \"host language\". Specifies the localization language of the metadata to be filled into snippet.localized. The field is filled with the default metadata if there is no localization in the specified language. The parameter value must be a language code included in the list returned by the i18nLanguages.list method (e.g. en_US, es_MX)."]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the channels with the specified IDs."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "managedByMe")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the channels managed by the authenticated user."]
                    pub managed_by_me: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the ids of channels owned by the authenticated user."]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mySubscribers")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the channels subscribed to the authenticated user"]
                    pub my_subscribers: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set *part=contentDetails*, the API response will also contain all of those nested properties."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *onBehalfOfContentOwner* parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The API currently only allows the parameter value to be set to either brandingSettings or invideoPromotion. (You cannot update both of those parts with a single request.) Note that this method overrides the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod comment_threads {
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter identifies the properties that the API response will include. Set the parameter value to snippet. The snippet part has a quota cost of 2 units."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "allThreadsRelatedToChannelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the comment threads of all videos of the channel and the channel comments as well."]
                    pub all_threads_related_to_channel_id:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the comment threads for all the channel comments (ie does not include comments left on videos)."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the comment threads with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(
                        default = "{ query_parameters_defaults :: moderation_status () }",
                        setter(into)
                    )]
                    #[serde(rename = "moderationStatus")]
                    #[serde(default = "query_parameters_defaults :: moderation_status")]
                    #[doc = "Limits the returned comment threads to those with the specified moderation status. Not compatible with the 'id' filter. Valid values: published, heldForReview, likelySpam."]
                    pub moderation_status: QueryParametersModerationStatusEnum,
                    #[builder(default = "{ query_parameters_defaults :: order () }", setter(into))]
                    #[serde(rename = "order")]
                    #[serde(default = "query_parameters_defaults :: order")]
                    pub order: QueryParametersOrderEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "searchTerms")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Limits the returned comment threads to those matching the specified key words. Not compatible with the 'id' filter."]
                    pub search_terms: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: text_format () }",
                        setter(into)
                    )]
                    #[serde(rename = "textFormat")]
                    #[serde(default = "query_parameters_defaults :: text_format")]
                    #[doc = "The requested text format for the returned comments."]
                    pub text_format: QueryParametersTextFormatEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the comment threads of the specified video."]
                    pub video_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                    pub fn moderation_status() -> super::QueryParametersModerationStatusEnum {
                        serde_json::from_str(&"published").unwrap()
                    }
                    pub fn order() -> super::QueryParametersOrderEnum {
                        serde_json::from_str(&"time").unwrap()
                    }
                    pub fn text_format() -> super::QueryParametersTextFormatEnum {
                        serde_json::from_str(&"html").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Limits the returned comment threads to those with the specified moderation status. Not compatible with the 'id' filter. Valid values: published, heldForReview, likelySpam."]
                pub enum QueryParametersModerationStatusEnum {
                    #[serde(rename = "published")]
                    #[doc = "The comment is available for public display."]
                    Published,
                    #[serde(rename = "heldForReview")]
                    #[doc = "The comment is awaiting review by a moderator."]
                    HeldForReview,
                    #[serde(rename = "likelySpam")]
                    #[doc = ""]
                    LikelySpam,
                    #[serde(rename = "rejected")]
                    #[doc = "The comment is unfit for display."]
                    Rejected,
                }
                impl ::std::default::Default for QueryParametersModerationStatusEnum {
                    fn default() -> Self {
                        Self::Published
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersOrderEnum {
                    #[serde(rename = "orderUnspecified")]
                    #[doc = ""]
                    OrderUnspecified,
                    #[serde(rename = "time")]
                    #[doc = "Order by time."]
                    Time,
                    #[serde(rename = "relevance")]
                    #[doc = "Order by relevance."]
                    Relevance,
                }
                impl ::std::default::Default for QueryParametersOrderEnum {
                    fn default() -> Self {
                        Self::Time
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The requested text format for the returned comments."]
                pub enum QueryParametersTextFormatEnum {
                    #[serde(rename = "textFormatUnspecified")]
                    #[doc = ""]
                    TextFormatUnspecified,
                    #[serde(rename = "html")]
                    #[doc = "Returns the comments in HTML format. This is the default value."]
                    Html,
                    #[serde(rename = "plainText")]
                    #[doc = "Returns the comments in plain text format."]
                    PlainText,
                }
                impl ::std::default::Default for QueryParametersTextFormatEnum {
                    fn default() -> Self {
                        Self::Html
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of commentThread resource properties that the API response will include. You must at least include the snippet part in the parameter value since that part contains all of the properties that the API request can update."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod comments {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter identifies the properties that the API response will include. Set the parameter value to snippet. The snippet part has a quota cost of 2 units."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the comments with the given IDs for One Platform."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "parentId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns replies to the specified comment. Note, currently YouTube features only one level of replies (ie replies to top level comments). However replies to replies may be supported in the future."]
                    pub parent_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more comment resource properties that the API response will include."]
                    pub part: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: text_format () }",
                        setter(into)
                    )]
                    #[serde(rename = "textFormat")]
                    #[serde(default = "query_parameters_defaults :: text_format")]
                    #[doc = "The requested text format for the returned comments."]
                    pub text_format: QueryParametersTextFormatEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"20").unwrap()
                    }
                    pub fn text_format() -> super::QueryParametersTextFormatEnum {
                        serde_json::from_str(&"html").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The requested text format for the returned comments."]
                pub enum QueryParametersTextFormatEnum {
                    #[serde(rename = "textFormatUnspecified")]
                    #[doc = ""]
                    TextFormatUnspecified,
                    #[serde(rename = "html")]
                    #[doc = "Returns the comments in HTML format. This is the default value."]
                    Html,
                    #[serde(rename = "plainText")]
                    #[doc = "Returns the comments in plain text format."]
                    PlainText,
                }
                impl ::std::default::Default for QueryParametersTextFormatEnum {
                    fn default() -> Self {
                        Self::Html
                    }
                }
            }
            pub mod mark_as_spam {
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
                    #[serde(rename = "id")]
                    #[doc = "Flags the comments with the given IDs as spam in the caller's opinion."]
                    pub id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod set_moderation_status {
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
                        default = "{ query_parameters_defaults :: ban_author () }",
                        setter(into)
                    )]
                    #[serde(rename = "banAuthor")]
                    #[serde(default = "query_parameters_defaults :: ban_author")]
                    #[doc = "If set to true the author of the comment gets added to the ban list. This means all future comments of the author will autmomatically be rejected. Only valid in combination with STATUS_REJECTED."]
                    pub ban_author: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[doc = "Modifies the moderation status of the comments with the given IDs"]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "moderationStatus")]
                    #[doc = "Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: MODERATION_STATUS_PUBLISHED, MODERATION_STATUS_HELD_FOR_REVIEW, MODERATION_STATUS_REJECTED."]
                    pub moderation_status: QueryParametersModerationStatusEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn ban_author() -> ::std::primitive::bool {
                        serde_json::from_str(&"false").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: MODERATION_STATUS_PUBLISHED, MODERATION_STATUS_HELD_FOR_REVIEW, MODERATION_STATUS_REJECTED."]
                pub enum QueryParametersModerationStatusEnum {
                    #[serde(rename = "published")]
                    #[doc = "The comment is available for public display."]
                    Published,
                    #[serde(rename = "heldForReview")]
                    #[doc = "The comment is awaiting review by a moderator."]
                    HeldForReview,
                    #[serde(rename = "likelySpam")]
                    #[doc = ""]
                    LikelySpam,
                    #[serde(rename = "rejected")]
                    #[doc = "The comment is unfit for display."]
                    Rejected,
                }
                impl ::std::default::Default for QueryParametersModerationStatusEnum {
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
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter identifies the properties that the API response will include. You must at least include the snippet part in the parameter value since that part contains all of the properties that the API request can update."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod i18n_languages {
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
                    #[builder(default = "{ query_parameters_defaults :: hl () }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(default = "query_parameters_defaults :: hl")]
                    pub hl: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the i18nLanguage resource properties that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn hl() -> ::std::string::String {
                        String::from("en_US")
                    }
                }
            }
        }
    }
    pub mod i18n_regions {
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
                    #[builder(default = "{ query_parameters_defaults :: hl () }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(default = "query_parameters_defaults :: hl")]
                    pub hl: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the i18nRegion resource properties that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn hl() -> ::std::string::String {
                        String::from("en_US")
                    }
                }
            }
        }
    }
    pub mod live_broadcasts {
        pub mod methods {
            pub mod bind {
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
                    #[serde(rename = "id")]
                    #[doc = "Broadcast to bind to the stream"]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "streamId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Stream to bind, if not set unbind the current one."]
                    pub stream_id: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "id")]
                    #[doc = "Broadcast to delete."]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part properties that you can include in the parameter value are id, snippet, contentDetails, and status."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "broadcastStatus")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return broadcasts with a certain status, e.g. active broadcasts."]
                    pub broadcast_status: ::std::option::Option<QueryParametersBroadcastStatusEnum>,
                    #[builder(
                        default = "{ query_parameters_defaults :: broadcast_type () }",
                        setter(into)
                    )]
                    #[serde(rename = "broadcastType")]
                    #[serde(default = "query_parameters_defaults :: broadcast_type")]
                    #[doc = "Return only broadcasts with the selected type."]
                    pub broadcast_type: QueryParametersBroadcastTypeEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return broadcasts with the given ids from Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, status and statistics."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn broadcast_type() -> super::QueryParametersBroadcastTypeEnum {
                        serde_json::from_str(&"event").unwrap()
                    }
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Return broadcasts with a certain status, e.g. active broadcasts."]
                pub enum QueryParametersBroadcastStatusEnum {
                    #[serde(rename = "broadcastStatusFilterUnspecified")]
                    #[doc = ""]
                    BroadcastStatusFilterUnspecified,
                    #[serde(rename = "all")]
                    #[doc = "Return all broadcasts."]
                    All,
                    #[serde(rename = "active")]
                    #[doc = "Return current live broadcasts."]
                    Active,
                    #[serde(rename = "upcoming")]
                    #[doc = "Return broadcasts that have not yet started."]
                    Upcoming,
                    #[serde(rename = "completed")]
                    #[doc = "Return broadcasts that have already ended."]
                    Completed,
                }
                impl ::std::default::Default for QueryParametersBroadcastStatusEnum {
                    fn default() -> Self {
                        Self::BroadcastStatusFilterUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Return only broadcasts with the selected type."]
                pub enum QueryParametersBroadcastTypeEnum {
                    #[serde(rename = "broadcastTypeFilterUnspecified")]
                    #[doc = ""]
                    BroadcastTypeFilterUnspecified,
                    #[serde(rename = "all")]
                    #[doc = "Return all broadcasts."]
                    All,
                    #[serde(rename = "event")]
                    #[doc = "Return only scheduled event broadcasts."]
                    Event,
                    #[serde(rename = "persistent")]
                    #[doc = "Return only persistent broadcasts."]
                    Persistent,
                }
                impl ::std::default::Default for QueryParametersBroadcastTypeEnum {
                    fn default() -> Self {
                        Self::Event
                    }
                }
            }
            pub mod transition {
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
                    #[serde(rename = "broadcastStatus")]
                    #[doc = "The status to which the broadcast is going to transition."]
                    pub broadcast_status: QueryParametersBroadcastStatusEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[doc = "Broadcast to transition."]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more liveBroadcast resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, contentDetails, and status."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The status to which the broadcast is going to transition."]
                pub enum QueryParametersBroadcastStatusEnum {
                    #[serde(rename = "statusUnspecified")]
                    #[doc = ""]
                    StatusUnspecified,
                    #[serde(rename = "testing")]
                    #[doc = "Start testing the broadcast. YouTube transmits video to the broadcast's monitor stream. Note that you can only transition a broadcast to the testing state if its contentDetails.monitorStream.enableMonitorStream property is set to true.\","]
                    Testing,
                    #[serde(rename = "live")]
                    #[doc = "Return only persistent broadcasts."]
                    Live,
                    #[serde(rename = "complete")]
                    #[doc = "The broadcast is over. YouTube stops transmitting video."]
                    Complete,
                }
                impl ::std::default::Default for QueryParametersBroadcastStatusEnum {
                    fn default() -> Self {
                        Self::StatusUnspecified
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part properties that you can include in the parameter value are id, snippet, contentDetails, and status. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a broadcast's privacy status is defined in the status part. As such, if your request is updating a private or unlisted broadcast, and the request's part parameter value includes the status part, the broadcast's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the broadcast will revert to the default privacy setting."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod live_chat_bans {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response returns. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod live_chat_messages {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes. It identifies the properties that the write operation will set as well as the properties that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the localization language in which the system messages should be returned."]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "liveChatId")]
                    #[doc = "The id of the live chat for which comments should be returned."]
                    pub live_chat_id: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken property identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the liveChatComment resource parts that the API response will include. Supported values are id and snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "profileImageSize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Specifies the size of the profile image that should be returned for each user."]
                    pub profile_image_size: ::std::option::Option<::std::primitive::i64>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"500").unwrap()
                    }
                }
            }
        }
    }
    pub mod live_chat_moderators {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response returns. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "liveChatId")]
                    #[doc = "The id of the live chat for which moderators should be returned."]
                    pub live_chat_id: ::std::string::String,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the liveChatModerator resource parts that the API response will include. Supported values are id and snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                }
            }
        }
    }
    pub mod live_streams {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part properties that you can include in the parameter value are id, snippet, cdn, content_details, and status."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return LiveStreams with the given ids from Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more liveStream resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, cdn, and status."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part properties that you can include in the parameter value are id, snippet, cdn, and status. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. If the request body does not specify a value for a mutable property, the existing value for that property will be removed."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod members {
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
                    #[serde(rename = "filterByMemberChannelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Comma separated list of channel IDs. Only data about members that are part of this list will be included in the response."]
                    pub filter_by_member_channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hasAccessToLevel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter members in the results set to the ones that have access to a level."]
                    pub has_access_to_level: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ query_parameters_defaults :: mode () }", setter(into))]
                    #[serde(rename = "mode")]
                    #[serde(default = "query_parameters_defaults :: mode")]
                    #[doc = "Parameter that specifies which channel members to return."]
                    pub mode: QueryParametersModeEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the member resource parts that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                    pub fn mode() -> super::QueryParametersModeEnum {
                        serde_json::from_str(&"all_current").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Parameter that specifies which channel members to return."]
                pub enum QueryParametersModeEnum {
                    #[serde(rename = "listMembersModeUnknown")]
                    #[doc = ""]
                    ListMembersModeUnknown,
                    #[serde(rename = "updates")]
                    #[doc = "Return only members that joined after the first call with this mode was made."]
                    Updates,
                    #[serde(rename = "all_current")]
                    #[doc = "Return all current members, from newest to oldest."]
                    AllCurrent,
                }
                impl ::std::default::Default for QueryParametersModeEnum {
                    fn default() -> Self {
                        Self::AllCurrent
                    }
                }
            }
        }
    }
    pub mod memberships_levels {
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the membershipsLevel resource parts that the API response will include. Supported values are id and snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod playlist_items {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set *part=snippet*, the API response will contain all of those properties."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "playlistId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the playlist items within the given playlist."]
                    pub playlist_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the playlist items associated with the given video ID."]
                    pub video_id: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a playlist item can specify a start time and end time, which identify the times portion of the video that should play when users watch the video in the playlist. If your request is updating a playlist item that sets these values, and the request's part parameter value includes the contentDetails part, the playlist item's start and end times will be updated to whatever value the request body specifies. If the request body does not specify values, the existing start and end times will be removed and replaced with the default settings."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod playlists {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the playlists owned by the specified channel ID."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returen content in specified language"]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the playlists with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the playlists owned by the authenticated user."]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set *part=snippet*, the API response will contain all of those properties."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for mutable properties that are contained in any parts that the request body specifies. For example, a playlist's description is contained in the snippet part, which must be included in the request body. If the request does not specify a value for the snippet.description property, the playlist's existing description will be deleted."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod search {
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on resources belonging to this channelId."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "channelType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Add a filter on the channel search."]
                    pub channel_type: ::std::option::Option<QueryParametersChannelTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "eventType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on the livestream status of the videos."]
                    pub event_type: ::std::option::Option<QueryParametersEventTypeEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "forContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search owned by a content owner."]
                    pub for_content_owner: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "forDeveloper")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict the search to only retrieve videos uploaded using the project id of the authenticated user."]
                    pub for_developer: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "forMine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search for the private videos of the authenticated user."]
                    pub for_mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "location")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on location of the video"]
                    pub location: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locationRadius")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on distance from the location (specified above)."]
                    pub location_radius: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ query_parameters_defaults :: order () }", setter(into))]
                    #[serde(rename = "order")]
                    #[serde(default = "query_parameters_defaults :: order")]
                    #[doc = "Sort order of the results."]
                    pub order: QueryParametersOrderEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publishedAfter")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on resources published after this date."]
                    pub published_after: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "publishedBefore")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on resources published before this date."]
                    pub published_before: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "q")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Textual search terms to match."]
                    pub q: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "regionCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Display the content as seen by viewers in this country."]
                    pub region_code: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "relatedToVideoId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Search related to a resource."]
                    pub related_to_video_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "relevanceLanguage")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return results relevant to this language."]
                    pub relevance_language: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: safe_search () }",
                        setter(into)
                    )]
                    #[serde(rename = "safeSearch")]
                    #[serde(default = "query_parameters_defaults :: safe_search")]
                    #[doc = "Indicates whether the search results should include restricted content as well as standard content."]
                    pub safe_search: QueryParametersSafeSearchEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "topicId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Restrict results to a particular topic."]
                    pub topic_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ query_parameters_defaults :: _type () }", setter(into))]
                    #[serde(rename = "type")]
                    #[serde(default = "query_parameters_defaults :: _type")]
                    #[doc = "Restrict results to a particular set of resource types from One Platform."]
                    pub _type: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoCaption")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on the presence of captions on the videos."]
                    pub video_caption: ::std::option::Option<QueryParametersVideoCaptionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoCategoryId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on videos in a specific category."]
                    pub video_category_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoDefinition")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on the definition of the videos."]
                    pub video_definition: ::std::option::Option<QueryParametersVideoDefinitionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoDimension")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on 3d videos."]
                    pub video_dimension: ::std::option::Option<QueryParametersVideoDimensionEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoDuration")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on the duration of the videos."]
                    pub video_duration: ::std::option::Option<QueryParametersVideoDurationEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoEmbeddable")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on embeddable videos."]
                    pub video_embeddable: ::std::option::Option<QueryParametersVideoEmbeddableEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoLicense")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on the license of the videos."]
                    pub video_license: ::std::option::Option<QueryParametersVideoLicenseEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoSyndicated")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on syndicated videos."]
                    pub video_syndicated: ::std::option::Option<QueryParametersVideoSyndicatedEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoType")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Filter on videos of a specific type."]
                    pub video_type: ::std::option::Option<QueryParametersVideoTypeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                    pub fn order() -> super::QueryParametersOrderEnum {
                        serde_json::from_str(&"relevance").unwrap()
                    }
                    pub fn safe_search() -> super::QueryParametersSafeSearchEnum {
                        serde_json::from_str(&"moderate").unwrap()
                    }
                    pub fn _type() -> ::std::string::String {
                        String::from("video,channel,playlist")
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Add a filter on the channel search."]
                pub enum QueryParametersChannelTypeEnum {
                    #[serde(rename = "channelTypeUnspecified")]
                    #[doc = ""]
                    ChannelTypeUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Return all channels."]
                    Any,
                    #[serde(rename = "show")]
                    #[doc = "Only retrieve shows."]
                    Show,
                }
                impl ::std::default::Default for QueryParametersChannelTypeEnum {
                    fn default() -> Self {
                        Self::ChannelTypeUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on the livestream status of the videos."]
                pub enum QueryParametersEventTypeEnum {
                    #[serde(rename = "none")]
                    #[doc = ""]
                    None,
                    #[serde(rename = "upcoming")]
                    #[doc = "The live broadcast is upcoming."]
                    Upcoming,
                    #[serde(rename = "live")]
                    #[doc = "The live broadcast is active."]
                    Live,
                    #[serde(rename = "completed")]
                    #[doc = "The live broadcast has been completed."]
                    Completed,
                }
                impl ::std::default::Default for QueryParametersEventTypeEnum {
                    fn default() -> Self {
                        Self::None
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Sort order of the results."]
                pub enum QueryParametersOrderEnum {
                    #[serde(rename = "searchSortUnspecified")]
                    #[doc = ""]
                    SearchSortUnspecified,
                    #[serde(rename = "date")]
                    #[doc = "Resources are sorted in reverse chronological order based on the date they were created."]
                    Date,
                    #[serde(rename = "rating")]
                    #[doc = "Resources are sorted from highest to lowest rating."]
                    Rating,
                    #[serde(rename = "viewCount")]
                    #[doc = "Resources are sorted from highest to lowest number of views."]
                    ViewCount,
                    #[serde(rename = "relevance")]
                    #[doc = "Resources are sorted based on their relevance to the search query. This is the default value for this parameter."]
                    Relevance,
                    #[serde(rename = "title")]
                    #[doc = "Resources are sorted alphabetically by title."]
                    Title,
                    #[serde(rename = "videoCount")]
                    #[doc = "Channels are sorted in descending order of their number of uploaded videos."]
                    VideoCount,
                }
                impl ::std::default::Default for QueryParametersOrderEnum {
                    fn default() -> Self {
                        Self::Relevance
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Indicates whether the search results should include restricted content as well as standard content."]
                pub enum QueryParametersSafeSearchEnum {
                    #[serde(rename = "safeSearchSettingUnspecified")]
                    #[doc = ""]
                    SafeSearchSettingUnspecified,
                    #[serde(rename = "none")]
                    #[doc = "YouTube will not filter the search result set."]
                    None,
                    #[serde(rename = "moderate")]
                    #[doc = "YouTube will filter some content from search results and, at the least, will filter content that is restricted in your locale. Based on their content, search results could be removed from search results or demoted in search results. This is the default parameter value."]
                    Moderate,
                    #[serde(rename = "strict")]
                    #[doc = "YouTube will try to exclude all restricted content from the search result set. Based on their content, search results could be removed from search results or demoted in search results."]
                    Strict,
                }
                impl ::std::default::Default for QueryParametersSafeSearchEnum {
                    fn default() -> Self {
                        Self::Moderate
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on the presence of captions on the videos."]
                pub enum QueryParametersVideoCaptionEnum {
                    #[serde(rename = "videoCaptionUnspecified")]
                    #[doc = ""]
                    VideoCaptionUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Do not filter results based on caption availability."]
                    Any,
                    #[serde(rename = "closedCaption")]
                    #[doc = "Only include videos that have captions."]
                    ClosedCaption,
                    #[serde(rename = "none")]
                    #[doc = "Only include videos that do not have captions."]
                    None,
                }
                impl ::std::default::Default for QueryParametersVideoCaptionEnum {
                    fn default() -> Self {
                        Self::VideoCaptionUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on the definition of the videos."]
                pub enum QueryParametersVideoDefinitionEnum {
                    #[serde(rename = "any")]
                    #[doc = "Return all videos, regardless of their resolution."]
                    Any,
                    #[serde(rename = "standard")]
                    #[doc = "Only retrieve videos in standard definition."]
                    Standard,
                    #[serde(rename = "high")]
                    #[doc = "Only retrieve HD videos."]
                    High,
                }
                impl ::std::default::Default for QueryParametersVideoDefinitionEnum {
                    fn default() -> Self {
                        Self::Any
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on 3d videos."]
                pub enum QueryParametersVideoDimensionEnum {
                    #[serde(rename = "any")]
                    #[doc = ""]
                    Any,
                    #[serde(rename = "2d")]
                    #[doc = "Restrict search results to exclude 3D videos."]
                    _2d,
                    #[serde(rename = "3d")]
                    #[doc = "Restrict search results to only include 3D videos."]
                    _3d,
                }
                impl ::std::default::Default for QueryParametersVideoDimensionEnum {
                    fn default() -> Self {
                        Self::Any
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on the duration of the videos."]
                pub enum QueryParametersVideoDurationEnum {
                    #[serde(rename = "videoDurationUnspecified")]
                    #[doc = ""]
                    VideoDurationUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Do not filter video search results based on their duration. This is the default value."]
                    Any,
                    #[serde(rename = "short")]
                    #[doc = "Only include videos that are less than four minutes long."]
                    Short,
                    #[serde(rename = "medium")]
                    #[doc = "Only include videos that are between four and 20 minutes long (inclusive)."]
                    Medium,
                    #[serde(rename = "long")]
                    #[doc = "Only include videos longer than 20 minutes."]
                    Long,
                }
                impl ::std::default::Default for QueryParametersVideoDurationEnum {
                    fn default() -> Self {
                        Self::VideoDurationUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on embeddable videos."]
                pub enum QueryParametersVideoEmbeddableEnum {
                    #[serde(rename = "videoEmbeddableUnspecified")]
                    #[doc = ""]
                    VideoEmbeddableUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Return all videos, embeddable or not."]
                    Any,
                    #[serde(rename = "true")]
                    #[doc = "Only retrieve embeddable videos."]
                    True,
                }
                impl ::std::default::Default for QueryParametersVideoEmbeddableEnum {
                    fn default() -> Self {
                        Self::VideoEmbeddableUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on the license of the videos."]
                pub enum QueryParametersVideoLicenseEnum {
                    #[serde(rename = "any")]
                    #[doc = "Return all videos, regardless of which license they have, that match the query parameters."]
                    Any,
                    #[serde(rename = "youtube")]
                    #[doc = "Only return videos that have the standard YouTube license."]
                    Youtube,
                    #[serde(rename = "creativeCommon")]
                    #[doc = "Only return videos that have a Creative Commons license. Users can reuse videos with this license in other videos that they create. Learn more."]
                    CreativeCommon,
                }
                impl ::std::default::Default for QueryParametersVideoLicenseEnum {
                    fn default() -> Self {
                        Self::Any
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on syndicated videos."]
                pub enum QueryParametersVideoSyndicatedEnum {
                    #[serde(rename = "videoSyndicatedUnspecified")]
                    #[doc = ""]
                    VideoSyndicatedUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Return all videos, syndicated or not."]
                    Any,
                    #[serde(rename = "true")]
                    #[doc = "Only retrieve syndicated videos."]
                    True,
                }
                impl ::std::default::Default for QueryParametersVideoSyndicatedEnum {
                    fn default() -> Self {
                        Self::VideoSyndicatedUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Filter on videos of a specific type."]
                pub enum QueryParametersVideoTypeEnum {
                    #[serde(rename = "videoTypeUnspecified")]
                    #[doc = ""]
                    VideoTypeUnspecified,
                    #[serde(rename = "any")]
                    #[doc = "Return all videos."]
                    Any,
                    #[serde(rename = "movie")]
                    #[doc = "Only retrieve movies."]
                    Movie,
                    #[serde(rename = "episode")]
                    #[doc = "Only retrieve episodes of shows."]
                    Episode,
                }
                impl ::std::default::Default for QueryParametersVideoTypeEnum {
                    fn default() -> Self {
                        Self::VideoTypeUnspecified
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "channelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the subscriptions of the given channel owner."]
                    pub channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "forChannelId")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the subscriptions to the subset of these channels that the authenticated user is subscribed to."]
                    pub for_channel_id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the subscriptions with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mine")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Flag for returning the subscriptions of the authenticated user."]
                    pub mine: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "myRecentSubscribers")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub my_recent_subscribers: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "mySubscribers")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the subscribers of the given channel owner."]
                    pub my_subscribers: ::std::option::Option<::std::primitive::bool>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ query_parameters_defaults :: order () }", setter(into))]
                    #[serde(rename = "order")]
                    #[serde(default = "query_parameters_defaults :: order")]
                    #[doc = "The order of the returned subscriptions"]
                    pub order: QueryParametersOrderEnum,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set *part=snippet*, the API response will also contain all of those nested properties."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                    pub fn order() -> super::QueryParametersOrderEnum {
                        serde_json::from_str(&"relevance").unwrap()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "The order of the returned subscriptions"]
                pub enum QueryParametersOrderEnum {
                    #[serde(rename = "subscriptionOrderUnspecified")]
                    #[doc = ""]
                    SubscriptionOrderUnspecified,
                    #[serde(rename = "relevance")]
                    #[doc = "Sort by relevance."]
                    Relevance,
                    #[serde(rename = "unread")]
                    #[doc = "Sort by order of activity."]
                    Unread,
                    #[serde(rename = "alphabetical")]
                    #[doc = "Sort alphabetically."]
                    Alphabetical,
                }
                impl ::std::default::Default for QueryParametersOrderEnum {
                    fn default() -> Self {
                        Self::Relevance
                    }
                }
            }
        }
    }
    pub mod super_chat_events {
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
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return rendered funding amounts in specified language."]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the superChatEvent resource parts that the API response will include. Supported values are id and snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                }
            }
        }
    }
    pub mod tests {
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
                    #[serde(rename = "part")]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod third_party_links {
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
                    #[serde(rename = "linkingToken")]
                    #[doc = "Delete the partner links with the given linking token."]
                    pub linking_token: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Do not use. Required for compatibility."]
                    pub part: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "type")]
                    #[doc = "Type of the link to be deleted."]
                    pub _type: QueryParametersTypeEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Type of the link to be deleted."]
                pub enum QueryParametersTypeEnum {
                    #[serde(rename = "linkUnspecified")]
                    #[doc = ""]
                    LinkUnspecified,
                    #[serde(rename = "channelToStoreLink")]
                    #[doc = "A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube."]
                    ChannelToStoreLink,
                }
                impl ::std::default::Default for QueryParametersTypeEnum {
                    fn default() -> Self {
                        Self::LinkUnspecified
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the thirdPartyLink resource parts that the API request and response will include. Supported values are linkingToken, status, and snippet."]
                    pub part: ::std::string::String,
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
                    #[serde(rename = "linkingToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Get a third party link with the given linking token."]
                    pub linking_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the thirdPartyLink resource parts that the API response will include. Supported values are linkingToken, status, and snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "type")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Get a third party link of the given type."]
                    pub _type: ::std::option::Option<QueryParametersTypeEnum>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Get a third party link of the given type."]
                pub enum QueryParametersTypeEnum {
                    #[serde(rename = "linkUnspecified")]
                    #[doc = ""]
                    LinkUnspecified,
                    #[serde(rename = "channelToStoreLink")]
                    #[doc = "A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube."]
                    ChannelToStoreLink,
                }
                impl ::std::default::Default for QueryParametersTypeEnum {
                    fn default() -> Self {
                        Self::LinkUnspecified
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
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the thirdPartyLink resource parts that the API request and response will include. Supported values are linkingToken, status, and snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod thumbnails {
        pub mod methods {
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "videoId")]
                    #[doc = "Returns the Thumbnail with the given video IDs for Stubby or Apiary."]
                    pub video_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod video_abuse_report_reasons {
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
                    #[builder(default = "{ query_parameters_defaults :: hl () }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(default = "query_parameters_defaults :: hl")]
                    pub hl: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the videoCategory resource parts that the API response will include. Supported values are id and snippet."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn hl() -> ::std::string::String {
                        String::from("en-US")
                    }
                }
            }
        }
    }
    pub mod video_categories {
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
                    #[builder(default = "{ query_parameters_defaults :: hl () }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(default = "query_parameters_defaults :: hl")]
                    pub hl: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Returns the video categories with the given IDs for Stubby or Apiary."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies the videoCategory resource properties that the API response will include. Set the parameter value to snippet."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "regionCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub region_code: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn hl() -> ::std::string::String {
                        String::from("en-US")
                    }
                }
            }
        }
    }
    pub mod videos {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod get_rating {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "autoLevels")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Should auto-levels be applied to the upload."]
                    pub auto_levels: ::std::option::Option<::std::primitive::bool>,
                    #[builder(
                        default = "{ query_parameters_defaults :: notify_subscribers () }",
                        setter(into)
                    )]
                    #[serde(rename = "notifySubscribers")]
                    #[serde(default = "query_parameters_defaults :: notify_subscribers")]
                    #[doc = "Notify the channel subscribers about the new video. As default, the notification is enabled."]
                    pub notify_subscribers: ::std::primitive::bool,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwnerChannel")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "This parameter can only be used in a properly authorized request. *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel."]
                    pub on_behalf_of_content_owner_channel:
                        ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that not all parts contain properties that can be set when inserting or updating a video. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "stabilize")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Should stabilize be applied to the upload."]
                    pub stabilize: ::std::option::Option<::std::primitive::bool>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn notify_subscribers() -> ::std::primitive::bool {
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
                    #[serde(rename = "chart")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the videos that are in the specified chart."]
                    pub chart: ::std::option::Option<QueryParametersChartEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "hl")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Stands for \"host language\". Specifies the localization language of the metadata to be filled into snippet.localized. The field is filled with the default metadata if there is no localization in the specified language. The parameter value must be a language code included in the list returned by the i18nLanguages.list method (e.g. en_US, es_MX)."]
                    pub hl: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "id")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return videos with the given ids."]
                    pub id: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "locale")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub locale: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxHeight")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    pub max_height: ::std::option::Option<::std::primitive::i64>,
                    #[builder(
                        default = "{ query_parameters_defaults :: max_results () }",
                        setter(into)
                    )]
                    #[serde(rename = "maxResults")]
                    #[serde(default = "query_parameters_defaults :: max_results")]
                    #[doc = "The *maxResults* parameter specifies the maximum number of items that should be returned in the result set. *Note:* This parameter is supported for use in conjunction with the myRating and chart parameters, but it is not supported for use in conjunction with the id parameter."]
                    pub max_results: ::std::primitive::i64,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "maxWidth")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return the player with maximum height specified in"]
                    pub max_width: ::std::option::Option<::std::primitive::i64>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "myRating")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Return videos liked/disliked by the authenticated user. Does not support RateType.RATED_TYPE_NONE."]
                    pub my_rating: ::std::option::Option<QueryParametersMyRatingEnum>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "pageToken")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved. *Note:* This parameter is supported for use in conjunction with the myRating and chart parameters, but it is not supported for use in conjunction with the id parameter."]
                    pub page_token: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter specifies a comma-separated list of one or more video resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set *part=snippet*, the API response will contain all of those properties."]
                    pub part: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "regionCode")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "Use a chart that is specific to the specified region"]
                    pub region_code: ::std::option::Option<::std::string::String>,
                    #[builder(
                        default = "{ query_parameters_defaults :: video_category_id () }",
                        setter(into)
                    )]
                    #[serde(rename = "videoCategoryId")]
                    #[serde(default = "query_parameters_defaults :: video_category_id")]
                    #[doc = "Use chart that is specific to the specified video category"]
                    pub video_category_id: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                mod query_parameters_defaults {
                    pub fn max_results() -> ::std::primitive::i64 {
                        serde_json::from_str(&"5").unwrap()
                    }
                    pub fn video_category_id() -> ::std::string::String {
                        String::from("0")
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Return the videos that are in the specified chart."]
                pub enum QueryParametersChartEnum {
                    #[serde(rename = "chartUnspecified")]
                    #[doc = ""]
                    ChartUnspecified,
                    #[serde(rename = "mostPopular")]
                    #[doc = "Return the most popular videos for the specified content region and video category."]
                    MostPopular,
                }
                impl ::std::default::Default for QueryParametersChartEnum {
                    fn default() -> Self {
                        Self::ChartUnspecified
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                #[doc = "Return videos liked/disliked by the authenticated user. Does not support RateType.RATED_TYPE_NONE."]
                pub enum QueryParametersMyRatingEnum {
                    #[serde(rename = "none")]
                    #[doc = ""]
                    None,
                    #[serde(rename = "like")]
                    #[doc = "The entity is liked."]
                    Like,
                    #[serde(rename = "dislike")]
                    #[doc = "The entity is disliked."]
                    Dislike,
                }
                impl ::std::default::Default for QueryParametersMyRatingEnum {
                    fn default() -> Self {
                        Self::None
                    }
                }
            }
            pub mod rate {
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
                    #[serde(rename = "id")]
                    pub id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "rating")]
                    pub rating: QueryParametersRatingEnum,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
                #[derive(
                    Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize,
                )]
                pub enum QueryParametersRatingEnum {
                    #[serde(rename = "none")]
                    #[doc = ""]
                    None,
                    #[serde(rename = "like")]
                    #[doc = "The entity is liked."]
                    Like,
                    #[serde(rename = "dislike")]
                    #[doc = "The entity is disliked."]
                    Dislike,
                }
                impl ::std::default::Default for QueryParametersRatingEnum {
                    fn default() -> Self {
                        Self::None
                    }
                }
            }
            pub mod report_abuse {
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "part")]
                    #[doc = "The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a video's privacy setting is contained in the status part. As such, if your request is updating a private video, and the request's part parameter value includes the status part, the video's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the video will revert to the default privacy setting. In addition, not all parts contain properties that can be set when inserting or updating a video. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response."]
                    pub part: ::std::string::String,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod watermarks {
        pub mod methods {
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
                    #[serde(rename = "channelId")]
                    pub channel_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
            pub mod unset {
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
                    #[serde(rename = "channelId")]
                    pub channel_id: ::std::string::String,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "onBehalfOfContentOwner")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "*Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner."]
                    pub on_behalf_of_content_owner: ::std::option::Option<::std::string::String>,
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
    pub struct AbuseReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "abuseTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub abuse_types: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<AbuseType>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub related_entities:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<RelatedEntity>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subject")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub subject: ::std::option::Option<::std::boxed::Box<Entity>>,
    }
    impl AbuseReport {
        pub fn builder() -> AbuseReportBuilder {
            AbuseReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct AbuseType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
    }
    impl AbuseType {
        pub fn builder() -> AbuseTypeBuilder {
            AbuseTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rights management policy for YouTube resources."]
    pub struct AccessPolicy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of allowed indicates whether the access to the policy is allowed or denied by default."]
        pub allowed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "exception")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of region codes that identify countries where the default policy do not apply."]
        pub exception: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl AccessPolicy {
        pub fn builder() -> AccessPolicyBuilder {
            AccessPolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An *activity* resource contains information about an action that a particular channel, or user, has taken on YouTube.The actions reported in activity feeds include rating a video, sharing a video, marking a video as a favorite, commenting on a video, uploading a video, and so forth. Each activity resource identifies the type of action, the channel associated with the action, and the resource(s) associated with the action, such as the video that was rated or uploaded."]
    pub struct Activity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains information about the content associated with the activity. For example, if the snippet.type value is videoRated, then the contentDetails object's content identifies the rated video."]
        pub content_details: ::std::option::Option<::std::boxed::Box<ActivityContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the activity."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ activity_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "activity_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#activity\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the activity, including the activity's type and group ID."]
        pub snippet: ::std::option::Option<::std::boxed::Box<ActivitySnippet>>,
    }
    impl Activity {
        pub fn builder() -> ActivityBuilder {
            ActivityBuilder::default()
        }
    }
    mod activity_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#activity")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the content of an activity: the video that was shared, the channel that was subscribed to, etc."]
    pub struct ActivityContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bulletin")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bulletin object contains details about a channel bulletin post. This object is only present if the snippet.type is bulletin."]
        pub bulletin: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsBulletin>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channelItem object contains details about a resource which was added to a channel. This property is only present if the snippet.type is channelItem."]
        pub channel_item:
            ::std::option::Option<::std::boxed::Box<ActivityContentDetailsChannelItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment object contains information about a resource that received a comment. This property is only present if the snippet.type is comment."]
        pub comment: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsComment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "favorite")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The favorite object contains information about a video that was marked as a favorite video. This property is only present if the snippet.type is favorite."]
        pub favorite: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsFavorite>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "like")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The like object contains information about a resource that received a positive (like) rating. This property is only present if the snippet.type is like."]
        pub like: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsLike>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlistItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The playlistItem object contains information about a new playlist item. This property is only present if the snippet.type is playlistItem."]
        pub playlist_item:
            ::std::option::Option<::std::boxed::Box<ActivityContentDetailsPlaylistItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "promotedItem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The promotedItem object contains details about a resource which is being promoted. This property is only present if the snippet.type is promotedItem."]
        pub promoted_item:
            ::std::option::Option<::std::boxed::Box<ActivityContentDetailsPromotedItem>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recommendation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recommendation object contains information about a recommended resource. This property is only present if the snippet.type is recommendation."]
        pub recommendation:
            ::std::option::Option<::std::boxed::Box<ActivityContentDetailsRecommendation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "social")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The social object contains details about a social network post. This property is only present if the snippet.type is social."]
        pub social: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsSocial>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription object contains information about a channel that a user subscribed to. This property is only present if the snippet.type is subscription."]
        pub subscription:
            ::std::option::Option<::std::boxed::Box<ActivityContentDetailsSubscription>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The upload object contains information about the uploaded video. This property is only present if the snippet.type is upload."]
        pub upload: ::std::option::Option<::std::boxed::Box<ActivityContentDetailsUpload>>,
    }
    impl ActivityContentDetails {
        pub fn builder() -> ActivityContentDetailsBuilder {
            ActivityContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about a channel bulletin post."]
    pub struct ActivityContentDetailsBulletin {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the resource associated with a bulletin post. @mutable youtube.activities.insert"]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsBulletin {
        pub fn builder() -> ActivityContentDetailsBulletinBuilder {
            ActivityContentDetailsBulletinBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about a resource which was added to a channel."]
    pub struct ActivityContentDetailsChannelItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the resource that was added to the channel."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsChannelItem {
        pub fn builder() -> ActivityContentDetailsChannelItemBuilder {
            ActivityContentDetailsChannelItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a resource that received a comment."]
    pub struct ActivityContentDetailsComment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the resource associated with the comment."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsComment {
        pub fn builder() -> ActivityContentDetailsCommentBuilder {
            ActivityContentDetailsCommentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a video that was marked as a favorite video."]
    pub struct ActivityContentDetailsFavorite {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the resource that was marked as a favorite."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsFavorite {
        pub fn builder() -> ActivityContentDetailsFavoriteBuilder {
            ActivityContentDetailsFavoriteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a resource that received a positive (like) rating."]
    pub struct ActivityContentDetailsLike {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the rated resource."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsLike {
        pub fn builder() -> ActivityContentDetailsLikeBuilder {
            ActivityContentDetailsLikeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a new playlist item."]
    pub struct ActivityContentDetailsPlaylistItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlistId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value that YouTube uses to uniquely identify the playlist."]
        pub playlist_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlistItemId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "ID of the item within the playlist."]
        pub playlist_item_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information about the resource that was added to the playlist."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsPlaylistItem {
        pub fn builder() -> ActivityContentDetailsPlaylistItemBuilder {
            ActivityContentDetailsPlaylistItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about a resource which is being promoted."]
    pub struct ActivityContentDetailsPromotedItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "adTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL the client should fetch to request a promoted item."]
        pub ad_tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "clickTrackingUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL the client should ping to indicate that the user clicked through on this promoted item."]
        pub click_tracking_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creativeViewUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL the client should ping to indicate that the user was shown this promoted item."]
        pub creative_view_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ctaType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of call-to-action, a message to the user indicating action that can be taken."]
        pub cta_type: ::std::option::Option<ActivityContentDetailsPromotedItemCtaTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customCtaButtonText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom call-to-action button text. If specified, it will override the default button text for the cta_type."]
        pub custom_cta_button_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "descriptionText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text description to accompany the promoted item."]
        pub description_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL the client should direct the user to, if the user chooses to visit the advertiser's website."]
        pub destination_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "forecastingUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of forecasting URLs. The client should ping all of these URLs when a promoted item is not available, to indicate that a promoted item could have been shown."]
        pub forecasting_url: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "impressionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of impression URLs. The client should ping all of these URLs to indicate that the user was shown this promoted item."]
        pub impression_url: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the promoted video."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl ActivityContentDetailsPromotedItem {
        pub fn builder() -> ActivityContentDetailsPromotedItemBuilder {
            ActivityContentDetailsPromotedItemBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of call-to-action, a message to the user indicating action that can be taken."]
    pub enum ActivityContentDetailsPromotedItemCtaTypeEnum {
        #[serde(rename = "ctaTypeUnspecified")]
        #[doc = ""]
        CtaTypeUnspecified,
        #[serde(rename = "visitAdvertiserSite")]
        #[doc = ""]
        VisitAdvertiserSite,
    }
    impl ::std::default::Default for ActivityContentDetailsPromotedItemCtaTypeEnum {
        fn default() -> Self {
            Self::CtaTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information that identifies the recommended resource."]
    pub struct ActivityContentDetailsRecommendation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason that the resource is recommended to the user."]
        pub reason: ::std::option::Option<ActivityContentDetailsRecommendationReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the recommended resource."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "seedResourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The seedResourceId object contains information about the resource that caused the recommendation."]
        pub seed_resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsRecommendation {
        pub fn builder() -> ActivityContentDetailsRecommendationBuilder {
            ActivityContentDetailsRecommendationBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reason that the resource is recommended to the user."]
    pub enum ActivityContentDetailsRecommendationReasonEnum {
        #[serde(rename = "reasonUnspecified")]
        #[doc = ""]
        ReasonUnspecified,
        #[serde(rename = "videoFavorited")]
        #[doc = ""]
        VideoFavorited,
        #[serde(rename = "videoLiked")]
        #[doc = ""]
        VideoLiked,
        #[serde(rename = "videoWatched")]
        #[doc = ""]
        VideoWatched,
    }
    impl ::std::default::Default for ActivityContentDetailsRecommendationReasonEnum {
        fn default() -> Self {
            Self::ReasonUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about a social network post."]
    pub struct ActivityContentDetailsSocial {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "author")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The author of the social network post."]
        pub author: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An image of the post's author."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "referenceUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of the social network post."]
        pub reference_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object encapsulates information that identifies the resource associated with a social network post."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the social network."]
        pub _type: ::std::option::Option<ActivityContentDetailsSocialTypeEnum>,
    }
    impl ActivityContentDetailsSocial {
        pub fn builder() -> ActivityContentDetailsSocialBuilder {
            ActivityContentDetailsSocialBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The name of the social network."]
    pub enum ActivityContentDetailsSocialTypeEnum {
        #[serde(rename = "typeUnspecified")]
        #[doc = ""]
        TypeUnspecified,
        #[serde(rename = "googlePlus")]
        #[doc = ""]
        GooglePlus,
        #[serde(rename = "facebook")]
        #[doc = ""]
        Facebook,
        #[serde(rename = "twitter")]
        #[doc = ""]
        Twitter,
    }
    impl ::std::default::Default for ActivityContentDetailsSocialTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a channel that a user subscribed to."]
    pub struct ActivityContentDetailsSubscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resourceId object contains information that identifies the resource that the user subscribed to."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
    }
    impl ActivityContentDetailsSubscription {
        pub fn builder() -> ActivityContentDetailsSubscriptionBuilder {
            ActivityContentDetailsSubscriptionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the uploaded video."]
    pub struct ActivityContentDetailsUpload {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the uploaded video."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl ActivityContentDetailsUpload {
        pub fn builder() -> ActivityContentDetailsUploadBuilder {
            ActivityContentDetailsUploadBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ActivityListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Activity>>>,
        #[builder(
            default = "{ activity_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "activity_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#activityListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl ActivityListResponse {
        pub fn builder() -> ActivityListResponseBuilder {
            ActivityListResponseBuilder::default()
        }
    }
    mod activity_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#activityListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about an activity, including title, description, thumbnails, activity type and group. Next ID: 12"]
    pub struct ActivitySnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel associated with the activity."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel title for the channel responsible for this activity"]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the resource primarily associated with the activity. @mutable youtube.activities.insert"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The group ID associated with the activity. A group ID identifies user events that are associated with the same user and resource. For example, if a user rates a video and marks the same video as a favorite, the entries for those events would have the same group ID in the user's activity feed. In your user interface, you can avoid repetition by grouping events with the same groupId value."]
        pub group_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the video was uploaded."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the resource that is primarily associated with the activity. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the resource primarily associated with the activity."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of activity that the resource describes."]
        pub _type: ::std::option::Option<ActivitySnippetTypeEnum>,
    }
    impl ActivitySnippet {
        pub fn builder() -> ActivitySnippetBuilder {
            ActivitySnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of activity that the resource describes."]
    pub enum ActivitySnippetTypeEnum {
        #[serde(rename = "typeUnspecified")]
        #[doc = ""]
        TypeUnspecified,
        #[serde(rename = "upload")]
        #[doc = ""]
        Upload,
        #[serde(rename = "like")]
        #[doc = ""]
        Like,
        #[serde(rename = "favorite")]
        #[doc = ""]
        Favorite,
        #[serde(rename = "comment")]
        #[doc = ""]
        Comment,
        #[serde(rename = "subscription")]
        #[doc = ""]
        Subscription,
        #[serde(rename = "playlistItem")]
        #[doc = ""]
        PlaylistItem,
        #[serde(rename = "recommendation")]
        #[doc = ""]
        Recommendation,
        #[serde(rename = "bulletin")]
        #[doc = ""]
        Bulletin,
        #[serde(rename = "social")]
        #[doc = ""]
        Social,
        #[serde(rename = "channelItem")]
        #[doc = ""]
        ChannelItem,
        #[serde(rename = "promotedItem")]
        #[doc = ""]
        PromotedItem,
    }
    impl ::std::default::Default for ActivitySnippetTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *caption* resource represents a YouTube caption track. A caption track is associated with exactly one YouTube video."]
    pub struct Caption {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the caption track."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ caption_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "caption_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#caption\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the caption."]
        pub snippet: ::std::option::Option<::std::boxed::Box<CaptionSnippet>>,
    }
    impl Caption {
        pub fn builder() -> CaptionBuilder {
            CaptionBuilder::default()
        }
    }
    mod caption_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#caption")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CaptionListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of captions that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Caption>>>,
        #[builder(
            default = "{ caption_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "caption_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#captionListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl CaptionListResponse {
        pub fn builder() -> CaptionListResponseBuilder {
            CaptionListResponseBuilder::default()
        }
    }
    mod caption_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#captionListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a caption track, such as its language and name."]
    pub struct CaptionSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioTrackType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of audio track associated with the caption track."]
        pub audio_track_type: ::std::option::Option<CaptionSnippetAudioTrackTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed."]
        pub failure_reason: ::std::option::Option<CaptionSnippetFailureReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isAutoSynced")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether YouTube synchronized the caption track to the audio track in the video. The value will be true if a sync was explicitly requested when the caption track was uploaded. For example, when calling the captions.insert or captions.update methods, you can set the sync parameter to true to instruct YouTube to sync the uploaded track to the video. If the value is false, YouTube uses the time codes in the uploaded caption track to determine when to display captions."]
        pub is_auto_synced: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isCC")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the track contains closed captions for the deaf and hard of hearing. The default value is false."]
        pub is_cc: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDraft")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the caption track is a draft. If the value is true, then the track is not publicly visible. The default value is false. @mutable youtube.captions.insert youtube.captions.update"]
        pub is_draft: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isEasyReader")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether caption track is formatted for \"easy reader,\" meaning it is at a third-grade level for language learners. The default value is false."]
        pub is_easy_reader: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isLarge")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the caption track uses large text for the vision-impaired. The default value is false."]
        pub is_large: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the caption track. The property value is a BCP-47 language tag."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdated")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the caption track was last updated."]
        pub last_updated: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the caption track. The name is intended to be visible to the user as an option during playback."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The caption track's status."]
        pub status: ::std::option::Option<CaptionSnippetStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackKind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The caption track's type."]
        pub track_kind: ::std::option::Option<CaptionSnippetTrackKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the video associated with the caption track. @mutable youtube.captions.insert"]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl CaptionSnippet {
        pub fn builder() -> CaptionSnippetBuilder {
            CaptionSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of audio track associated with the caption track."]
    pub enum CaptionSnippetAudioTrackTypeEnum {
        #[serde(rename = "unknown")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "primary")]
        #[doc = ""]
        Primary,
        #[serde(rename = "commentary")]
        #[doc = ""]
        Commentary,
        #[serde(rename = "descriptive")]
        #[doc = ""]
        Descriptive,
    }
    impl ::std::default::Default for CaptionSnippetAudioTrackTypeEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed."]
    pub enum CaptionSnippetFailureReasonEnum {
        #[serde(rename = "unknownFormat")]
        #[doc = ""]
        UnknownFormat,
        #[serde(rename = "unsupportedFormat")]
        #[doc = ""]
        UnsupportedFormat,
        #[serde(rename = "processingFailed")]
        #[doc = ""]
        ProcessingFailed,
    }
    impl ::std::default::Default for CaptionSnippetFailureReasonEnum {
        fn default() -> Self {
            Self::UnknownFormat
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The caption track's status."]
    pub enum CaptionSnippetStatusEnum {
        #[serde(rename = "serving")]
        #[doc = ""]
        Serving,
        #[serde(rename = "syncing")]
        #[doc = ""]
        Syncing,
        #[serde(rename = "failed")]
        #[doc = ""]
        Failed,
    }
    impl ::std::default::Default for CaptionSnippetStatusEnum {
        fn default() -> Self {
            Self::Serving
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The caption track's type."]
    pub enum CaptionSnippetTrackKindEnum {
        #[serde(rename = "standard")]
        #[doc = ""]
        Standard,
        #[serde(rename = "ASR")]
        #[doc = ""]
        Asr,
        #[serde(rename = "forced")]
        #[doc = ""]
        Forced,
    }
    impl ::std::default::Default for CaptionSnippetTrackKindEnum {
        fn default() -> Self {
            Self::Standard
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Brief description of the live stream cdn settings."]
    pub struct CdnSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "format")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The format of the video stream that you are sending to Youtube. "]
        pub format: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The frame rate of the inbound video data."]
        pub frame_rate: ::std::option::Option<CdnSettingsFrameRateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ingestionInfo object contains information that YouTube provides that you need to transmit your RTMP or HTTP stream to YouTube."]
        pub ingestion_info: ::std::option::Option<::std::boxed::Box<IngestionInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestionType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = " The method or protocol used to transmit the video stream."]
        pub ingestion_type: ::std::option::Option<CdnSettingsIngestionTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolution")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resolution of the inbound video data."]
        pub resolution: ::std::option::Option<CdnSettingsResolutionEnum>,
    }
    impl CdnSettings {
        pub fn builder() -> CdnSettingsBuilder {
            CdnSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The frame rate of the inbound video data."]
    pub enum CdnSettingsFrameRateEnum {
        #[serde(rename = "30fps")]
        #[doc = ""]
        _30fps,
        #[serde(rename = "60fps")]
        #[doc = ""]
        _60fps,
        #[serde(rename = "variable")]
        #[doc = ""]
        Variable,
    }
    impl ::std::default::Default for CdnSettingsFrameRateEnum {
        fn default() -> Self {
            Self::_30fps
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = " The method or protocol used to transmit the video stream."]
    pub enum CdnSettingsIngestionTypeEnum {
        #[serde(rename = "rtmp")]
        #[doc = ""]
        Rtmp,
        #[serde(rename = "dash")]
        #[doc = ""]
        Dash,
        #[serde(rename = "webrtc")]
        #[doc = ""]
        Webrtc,
        #[serde(rename = "hls")]
        #[doc = ""]
        Hls,
    }
    impl ::std::default::Default for CdnSettingsIngestionTypeEnum {
        fn default() -> Self {
            Self::Rtmp
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The resolution of the inbound video data."]
    pub enum CdnSettingsResolutionEnum {
        #[serde(rename = "240p")]
        #[doc = ""]
        _240p,
        #[serde(rename = "360p")]
        #[doc = ""]
        _360p,
        #[serde(rename = "480p")]
        #[doc = ""]
        _480p,
        #[serde(rename = "720p")]
        #[doc = ""]
        _720p,
        #[serde(rename = "1080p")]
        #[doc = ""]
        _1080p,
        #[serde(rename = "1440p")]
        #[doc = ""]
        _1440p,
        #[serde(rename = "2160p")]
        #[doc = ""]
        _2160p,
        #[serde(rename = "variable")]
        #[doc = ""]
        Variable,
    }
    impl ::std::default::Default for CdnSettingsResolutionEnum {
        fn default() -> Self {
            Self::_240p
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *channel* resource contains information about a YouTube channel."]
    pub struct Channel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "auditDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process."]
        pub audit_details: ::std::option::Option<::std::boxed::Box<ChannelAuditDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "brandingSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The brandingSettings object encapsulates information about the branding of the channel."]
        pub branding_settings: ::std::option::Option<::std::boxed::Box<ChannelBrandingSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object encapsulates information about the channel's content."]
        pub content_details: ::std::option::Option<::std::boxed::Box<ChannelContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentOwnerDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel."]
        pub content_owner_details:
            ::std::option::Option<::std::boxed::Box<ChannelContentOwnerDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversionPings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conversionPings object encapsulates information about conversion pings that need to be respected by the channel."]
        pub conversion_pings: ::std::option::Option<::std::boxed::Box<ChannelConversionPings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ channel_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "channel_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#channel\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localizations for different languages"]
        pub localizations: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ChannelLocalization>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the channel, such as its title, description, and thumbnail images."]
        pub snippet: ::std::option::Option<::std::boxed::Box<ChannelSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statistics object encapsulates statistics for the channel."]
        pub statistics: ::std::option::Option<::std::boxed::Box<ChannelStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object encapsulates information about the privacy status of the channel."]
        pub status: ::std::option::Option<::std::boxed::Box<ChannelStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The topicDetails object encapsulates information about Freebase topics associated with the channel."]
        pub topic_details: ::std::option::Option<::std::boxed::Box<ChannelTopicDetails>>,
    }
    impl Channel {
        pub fn builder() -> ChannelBuilder {
            ChannelBuilder::default()
        }
    }
    mod channel_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#channel")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The auditDetails object encapsulates channel data that is relevant for YouTube Partners during the audit process."]
    pub struct ChannelAuditDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "communityGuidelinesGoodStanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the channel respects the community guidelines."]
        pub community_guidelines_good_standing: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentIdClaimsGoodStanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the channel has any unresolved claims."]
        pub content_id_claims_good_standing: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "copyrightStrikesGoodStanding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the channel has any copyright strikes."]
        pub copyright_strikes_good_standing: ::std::option::Option<::std::primitive::bool>,
    }
    impl ChannelAuditDetails {
        pub fn builder() -> ChannelAuditDetailsBuilder {
            ChannelAuditDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A channel banner returned as the response to a channel_banner.insert call."]
    pub struct ChannelBannerResource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ channel_banner_resource_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "channel_banner_resource_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#channelBannerResource\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL of this banner image."]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl ChannelBannerResource {
        pub fn builder() -> ChannelBannerResourceBuilder {
            ChannelBannerResourceBuilder::default()
        }
    }
    mod channel_banner_resource_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#channelBannerResource")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Branding properties of a YouTube channel."]
    pub struct ChannelBrandingSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Branding properties for the channel view."]
        pub channel: ::std::option::Option<::std::boxed::Box<ChannelSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional experimental branding properties."]
        pub hints: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PropertyValue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Branding properties for branding images."]
        pub image: ::std::option::Option<::std::boxed::Box<ImageSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "watch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Branding properties for the watch page."]
        pub watch: ::std::option::Option<::std::boxed::Box<WatchSettings>>,
    }
    impl ChannelBrandingSettings {
        pub fn builder() -> ChannelBrandingSettingsBuilder {
            ChannelBrandingSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the content of a channel."]
    pub struct ChannelContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relatedPlaylists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub related_playlists: ::std::option::Option<ChannelContentDetailsRelatedPlaylists>,
    }
    impl ChannelContentDetails {
        pub fn builder() -> ChannelContentDetailsBuilder {
            ChannelContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChannelContentDetailsRelatedPlaylists {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "favorites")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the playlist that contains the channel\"s favorite videos. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list."]
        pub favorites: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the playlist that contains the channel\"s liked videos. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list."]
        pub likes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the playlist that contains the channel\"s uploaded videos. Use the videos.insert method to upload new videos and the videos.delete method to delete previously uploaded videos."]
        pub uploads: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "watchHistory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the playlist that contains the channel\"s watch history. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list."]
        pub watch_history: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "watchLater")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the playlist that contains the channel\"s watch later playlist. Use the playlistItems.insert and playlistItems.delete to add or remove items from that list."]
        pub watch_later: ::std::option::Option<::std::string::String>,
    }
    impl ChannelContentDetailsRelatedPlaylists {
        pub fn builder() -> ChannelContentDetailsRelatedPlaylistsBuilder {
            ChannelContentDetailsRelatedPlaylistsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel."]
    pub struct ChannelContentOwnerDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentOwner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the content owner linked to the channel."]
        pub content_owner: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeLinked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the channel was linked to the content owner."]
        pub time_linked: ::std::option::Option<::std::string::String>,
    }
    impl ChannelContentOwnerDetails {
        pub fn builder() -> ChannelContentOwnerDetailsBuilder {
            ChannelContentOwnerDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping."]
    pub struct ChannelConversionPing {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "context")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the context of the ping."]
        pub context: ::std::option::Option<ChannelConversionPingContextEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url (without the schema) that the player shall send the ping to. It's at caller's descretion to decide which schema to use (http vs https) Example of a returned url: //googleads.g.doubleclick.net/pagead/ viewthroughconversion/962985656/?data=path%3DtHe_path%3Btype%3D cview%3Butuid%3DGISQtTNGYqaYl4sKxoVvKA&labe=default The caller must append biscotti authentication (ms param in case of mobile, for example) to this ping."]
        pub conversion_url: ::std::option::Option<::std::string::String>,
    }
    impl ChannelConversionPing {
        pub fn builder() -> ChannelConversionPingBuilder {
            ChannelConversionPingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Defines the context of the ping."]
    pub enum ChannelConversionPingContextEnum {
        #[serde(rename = "subscribe")]
        #[doc = ""]
        Subscribe,
        #[serde(rename = "unsubscribe")]
        #[doc = ""]
        Unsubscribe,
        #[serde(rename = "cview")]
        #[doc = ""]
        Cview,
    }
    impl ::std::default::Default for ChannelConversionPingContextEnum {
        fn default() -> Self {
            Self::Subscribe
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The conversionPings object encapsulates information about conversion pings that need to be respected by the channel."]
    pub struct ChannelConversionPings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping."]
        pub pings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChannelConversionPing>>>,
    }
    impl ChannelConversionPings {
        pub fn builder() -> ChannelConversionPingsBuilder {
            ChannelConversionPingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChannelListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Channel>>>,
        #[builder(
            default = "{ channel_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "channel_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#channelListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl ChannelListResponse {
        pub fn builder() -> ChannelListResponseBuilder {
            ChannelListResponseBuilder::default()
        }
    }
    mod channel_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#channelListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Channel localization setting"]
    pub struct ChannelLocalization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized strings for channel's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized strings for channel's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ChannelLocalization {
        pub fn builder() -> ChannelLocalizationBuilder {
            ChannelLocalizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChannelProfileDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The YouTube channel ID."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel's URL."]
        pub channel_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel's display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channels's avatar URL."]
        pub profile_image_url: ::std::option::Option<::std::string::String>,
    }
    impl ChannelProfileDetails {
        pub fn builder() -> ChannelProfileDetailsBuilder {
            ChannelProfileDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChannelSection {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains details about the channel section content, such as a list of playlists or channels featured in the section."]
        pub content_details: ::std::option::Option<::std::boxed::Box<ChannelSectionContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel section."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ channel_section_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "channel_section_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#channelSection\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localizations for different languages"]
        pub localizations: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<ChannelSectionLocalization>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the channel section, such as its type, style and title."]
        pub snippet: ::std::option::Option<::std::boxed::Box<ChannelSectionSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targeting")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The targeting object contains basic targeting settings about the channel section."]
        pub targeting: ::std::option::Option<::std::boxed::Box<ChannelSectionTargeting>>,
    }
    impl ChannelSection {
        pub fn builder() -> ChannelSectionBuilder {
            ChannelSectionBuilder::default()
        }
    }
    mod channel_section_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#channelSection")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about a channelsection, including playlists and channels."]
    pub struct ChannelSectionContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel ids for type multiple_channels."]
        pub channels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlists")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed."]
        pub playlists: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ChannelSectionContentDetails {
        pub fn builder() -> ChannelSectionContentDetailsBuilder {
            ChannelSectionContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ChannelSectionListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of ChannelSections that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ChannelSection>>>,
        #[builder(
            default = "{ channel_section_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "channel_section_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#channelSectionListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl ChannelSectionListResponse {
        pub fn builder() -> ChannelSectionListResponseBuilder {
            ChannelSectionListResponseBuilder::default()
        }
    }
    mod channel_section_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#channelSectionListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ChannelSection localization setting"]
    pub struct ChannelSectionLocalization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized strings for channel section's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ChannelSectionLocalization {
        pub fn builder() -> ChannelSectionLocalizationBuilder {
            ChannelSectionLocalizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a channel section, including title, style and position."]
    pub struct ChannelSectionSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel that published the channel section."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the channel section's default title and description."]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized title, read-only."]
        pub localized: ::std::option::Option<::std::boxed::Box<ChannelSectionLocalization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The position of the channel section in the channel."]
        pub position: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "style")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The style of the channel section."]
        pub style: ::std::option::Option<ChannelSectionSnippetStyleEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel section's title for multiple_playlists and multiple_channels."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the channel section."]
        pub _type: ::std::option::Option<ChannelSectionSnippetTypeEnum>,
    }
    impl ChannelSectionSnippet {
        pub fn builder() -> ChannelSectionSnippetBuilder {
            ChannelSectionSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The style of the channel section."]
    pub enum ChannelSectionSnippetStyleEnum {
        #[serde(rename = "channelsectionStyleUnspecified")]
        #[doc = ""]
        ChannelsectionStyleUnspecified,
        #[serde(rename = "horizontalRow")]
        #[doc = ""]
        HorizontalRow,
        #[serde(rename = "verticalList")]
        #[doc = ""]
        VerticalList,
    }
    impl ::std::default::Default for ChannelSectionSnippetStyleEnum {
        fn default() -> Self {
            Self::ChannelsectionStyleUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the channel section."]
    pub enum ChannelSectionSnippetTypeEnum {
        #[serde(rename = "channelsectionTypeUnspecified")]
        #[doc = ""]
        ChannelsectionTypeUnspecified,
        #[serde(rename = "singlePlaylist")]
        #[doc = ""]
        SinglePlaylist,
        #[serde(rename = "multiplePlaylists")]
        #[doc = ""]
        MultiplePlaylists,
        #[serde(rename = "popularUploads")]
        #[doc = ""]
        PopularUploads,
        #[serde(rename = "recentUploads")]
        #[doc = ""]
        RecentUploads,
        #[serde(rename = "likes")]
        #[doc = ""]
        Likes,
        #[serde(rename = "allPlaylists")]
        #[doc = ""]
        AllPlaylists,
        #[serde(rename = "likedPlaylists")]
        #[doc = ""]
        LikedPlaylists,
        #[serde(rename = "recentPosts")]
        #[doc = ""]
        RecentPosts,
        #[serde(rename = "recentActivity")]
        #[doc = ""]
        RecentActivity,
        #[serde(rename = "liveEvents")]
        #[doc = ""]
        LiveEvents,
        #[serde(rename = "upcomingEvents")]
        #[doc = ""]
        UpcomingEvents,
        #[serde(rename = "completedEvents")]
        #[doc = ""]
        CompletedEvents,
        #[serde(rename = "multipleChannels")]
        #[doc = ""]
        MultipleChannels,
        #[serde(rename = "postedVideos")]
        #[doc = ""]
        PostedVideos,
        #[serde(rename = "postedPlaylists")]
        #[doc = ""]
        PostedPlaylists,
        #[serde(rename = "subscriptions")]
        #[doc = ""]
        Subscriptions,
    }
    impl ::std::default::Default for ChannelSectionSnippetTypeEnum {
        fn default() -> Self {
            Self::ChannelsectionTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "ChannelSection targeting setting."]
    pub struct ChannelSectionTargeting {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country the channel section is targeting."]
        pub countries: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language the channel section is targeting."]
        pub languages: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The region the channel section is targeting."]
        pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ChannelSectionTargeting {
        pub fn builder() -> ChannelSectionTargetingBuilder {
            ChannelSectionTargetingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Branding properties for the channel view."]
    pub struct ChannelSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country of the channel."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultTab")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Which content tab users should see when viewing the channel."]
        pub default_tab: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the channel description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featuredChannelsTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Title for the featured channels tab."]
        pub featured_channels_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featuredChannelsUrls")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of featured channels."]
        pub featured_channels_urls: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "keywords")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lists keywords associated with the channel, comma-separated."]
        pub keywords: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moderateComments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible."]
        pub moderate_comments: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A prominent color that can be rendered on this channel page."]
        pub profile_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showBrowseView")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the tab to browse the videos should be displayed."]
        pub show_browse_view: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "showRelatedChannels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether related channels should be proposed."]
        pub show_related_channels: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the channel title."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingAnalyticsAccountId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID for a Google Analytics account to track and measure traffic to the channels."]
        pub tracking_analytics_account_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unsubscribedTrailer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The trailer of the channel, for users that are not subscribers."]
        pub unsubscribed_trailer: ::std::option::Option<::std::string::String>,
    }
    impl ChannelSettings {
        pub fn builder() -> ChannelSettingsBuilder {
            ChannelSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a channel, including title, description and thumbnails."]
    pub struct ChannelSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "country")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The country of the channel."]
        pub country: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "customUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The custom url of the channel."]
        pub custom_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the channel's default title and description."]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the channel."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized title and description, read-only."]
        pub localized: ::std::option::Option<::std::boxed::Box<ChannelLocalization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the channel was created."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the channel. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail. When displaying thumbnails in your application, make sure that your code uses the image URLs exactly as they are returned in API responses. For example, your application should not use the http domain instead of the https domain in a URL returned in an API response. Beginning in July 2018, channel thumbnail URLs will only be available in the https domain, which is how the URLs appear in API responses. After that time, you might see broken images in your application if it tries to load YouTube images from the http domain. Thumbnail images might be empty for newly created channels and might take up to one day to populate."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl ChannelSnippet {
        pub fn builder() -> ChannelSnippetBuilder {
            ChannelSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about a channel: number of subscribers, number of videos in the channel, etc."]
    pub struct ChannelStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commentCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of comments for the channel."]
        pub comment_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hiddenSubscriberCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the number of subscribers is shown for this user."]
        pub hidden_subscriber_count: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriberCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of subscribers that the channel has."]
        pub subscriber_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of videos uploaded to the channel."]
        pub video_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of times the channel has been viewed."]
        pub view_count: ::std::option::Option<::std::string::String>,
    }
    impl ChannelStatistics {
        pub fn builder() -> ChannelStatisticsBuilder {
            ChannelStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "JSON template for the status part of a channel."]
    pub struct ChannelStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isLinked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity."]
        pub is_linked: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longUploadsStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information."]
        pub long_uploads_status: ::std::option::Option<ChannelStatusLongUploadsStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "madeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub made_for_kids: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Privacy status of the channel."]
        pub privacy_status: ::std::option::Option<ChannelStatusPrivacyStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfDeclaredMadeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub self_declared_made_for_kids: ::std::option::Option<::std::primitive::bool>,
    }
    impl ChannelStatus {
        pub fn builder() -> ChannelStatusBuilder {
            ChannelStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information."]
    pub enum ChannelStatusLongUploadsStatusEnum {
        #[serde(rename = "longUploadsUnspecified")]
        #[doc = ""]
        LongUploadsUnspecified,
        #[serde(rename = "allowed")]
        #[doc = ""]
        Allowed,
        #[serde(rename = "eligible")]
        #[doc = ""]
        Eligible,
        #[serde(rename = "disallowed")]
        #[doc = ""]
        Disallowed,
    }
    impl ::std::default::Default for ChannelStatusLongUploadsStatusEnum {
        fn default() -> Self {
            Self::LongUploadsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Privacy status of the channel."]
    pub enum ChannelStatusPrivacyStatusEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "unlisted")]
        #[doc = ""]
        Unlisted,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for ChannelStatusPrivacyStatusEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information specific to a store on a merchandising platform linked to a YouTube channel."]
    pub struct ChannelToStoreLinkDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the store."]
        pub store_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "storeUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Landing page of the store."]
        pub store_url: ::std::option::Option<::std::string::String>,
    }
    impl ChannelToStoreLinkDetails {
        pub fn builder() -> ChannelToStoreLinkDetailsBuilder {
            ChannelToStoreLinkDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Freebase topic information related to the channel."]
    pub struct ChannelTopicDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Wikipedia URLs that describe the channel's content."]
        pub topic_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API."]
        pub topic_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl ChannelTopicDetails {
        pub fn builder() -> ChannelTopicDetailsBuilder {
            ChannelTopicDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *comment* represents a single YouTube comment."]
    pub struct Comment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the comment."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ comment_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#comment\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the comment."]
        pub snippet: ::std::option::Option<::std::boxed::Box<CommentSnippet>>,
    }
    impl Comment {
        pub fn builder() -> CommentBuilder {
            CommentBuilder::default()
        }
    }
    mod comment_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#comment")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CommentListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of comments that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
        #[builder(
            default = "{ comment_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "comment_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#commentListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl CommentListResponse {
        pub fn builder() -> CommentListResponseBuilder {
            CommentListResponseBuilder::default()
        }
    }
    mod comment_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#commentListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a comment, such as its author and text."]
    pub struct CommentSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub author_channel_id:
            ::std::option::Option<::std::boxed::Box<CommentSnippetAuthorChannelId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorChannelUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Link to the author's YouTube channel, if any."]
        pub author_channel_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the user who posted the comment."]
        pub author_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorProfileImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the avatar of the user who posted the comment."]
        pub author_profile_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current viewer can rate this comment."]
        pub can_rate: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video comment it's the video's channel."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of likes this comment has received."]
        pub like_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moderationStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment's moderation status. Will not be set if the comments were requested through the id filter."]
        pub moderation_status: ::std::option::Option<CommentSnippetModerationStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique id of the parent comment, only set for replies."]
        pub parent_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the comment was originally published."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textDisplay")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment's text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc."]
        pub text_display: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textOriginal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment's original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment's author."]
        pub text_original: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updatedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the comment was last updated."]
        pub updated_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the video the comment refers to, if any."]
        pub video_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewerRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future."]
        pub viewer_rating: ::std::option::Option<CommentSnippetViewerRatingEnum>,
    }
    impl CommentSnippet {
        pub fn builder() -> CommentSnippetBuilder {
            CommentSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The comment's moderation status. Will not be set if the comments were requested through the id filter."]
    pub enum CommentSnippetModerationStatusEnum {
        #[serde(rename = "published")]
        #[doc = "The comment is available for public display."]
        Published,
        #[serde(rename = "heldForReview")]
        #[doc = "The comment is awaiting review by a moderator."]
        HeldForReview,
        #[serde(rename = "likelySpam")]
        #[doc = ""]
        LikelySpam,
        #[serde(rename = "rejected")]
        #[doc = "The comment is unfit for display."]
        Rejected,
    }
    impl ::std::default::Default for CommentSnippetModerationStatusEnum {
        fn default() -> Self {
            Self::Published
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future."]
    pub enum CommentSnippetViewerRatingEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "like")]
        #[doc = "The entity is liked."]
        Like,
        #[serde(rename = "dislike")]
        #[doc = "The entity is disliked."]
        Dislike,
    }
    impl ::std::default::Default for CommentSnippetViewerRatingEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The id of the author's YouTube channel, if any."]
    pub struct CommentSnippetAuthorChannelId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl CommentSnippetAuthorChannelId {
        pub fn builder() -> CommentSnippetAuthorChannelIdBuilder {
            CommentSnippetAuthorChannelIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *comment thread* represents information that applies to a top level comment and all its replies. It can also include the top level comment itself and some of the replies."]
    pub struct CommentThread {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the comment thread."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ comment_thread_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "comment_thread_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#commentThread\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "replies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The replies object contains a limited number of replies (if any) to the top level comment found in the snippet."]
        pub replies: ::std::option::Option<::std::boxed::Box<CommentThreadReplies>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the comment thread and also the top level comment."]
        pub snippet: ::std::option::Option<::std::boxed::Box<CommentThreadSnippet>>,
    }
    impl CommentThread {
        pub fn builder() -> CommentThreadBuilder {
            CommentThreadBuilder::default()
        }
    }
    mod comment_thread_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#commentThread")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct CommentThreadListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of comment threads that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<CommentThread>>>,
        #[builder(
            default = "{ comment_thread_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "comment_thread_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#commentThreadListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl CommentThreadListResponse {
        pub fn builder() -> CommentThreadListResponseBuilder {
            CommentThreadListResponseBuilder::default()
        }
    }
    mod comment_thread_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#commentThreadListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Comments written in (direct or indirect) reply to the top level comment."]
    pub struct CommentThreadReplies {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A limited number of replies. Unless the number of replies returned equals total_reply_count in the snippet the returned replies are only a subset of the total number of replies."]
        pub comments: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Comment>>>,
    }
    impl CommentThreadReplies {
        pub fn builder() -> CommentThreadRepliesBuilder {
            CommentThreadRepliesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a comment thread."]
    pub struct CommentThreadSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "canReply")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the current viewer of the thread can reply to it. This is viewer specific - other viewers may see a different value for this field."]
        pub can_reply: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The YouTube channel the comments in the thread refer to or the channel with the video the comments refer to. If video_id isn't set the comments refer to the channel itself."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isPublic")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the thread (and therefore all its comments) is visible to all YouTube users."]
        pub is_public: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topLevelComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The top level comment of this thread."]
        pub top_level_comment: ::std::option::Option<::std::boxed::Box<Comment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalReplyCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of replies (not including the top level comment)."]
        pub total_reply_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the video the comments refer to, if any. No video_id implies a channel discussion comment."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl CommentThreadSnippet {
        pub fn builder() -> CommentThreadSnippetBuilder {
            CommentThreadSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Ratings schemes. The country-specific ratings are mostly for movies and shows. LINT.IfChange"]
    pub struct ContentRating {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "acbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming."]
        pub acb_rating: ::std::option::Option<ContentRatingAcbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agcomRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM)."]
        pub agcom_rating: ::std::option::Option<ContentRatingAgcomRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "anatelRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television."]
        pub anatel_rating: ::std::option::Option<ContentRatingAnatelRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bbfcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's British Board of Film Classification (BBFC) rating."]
        pub bbfc_rating: ::std::option::Option<ContentRatingBbfcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bfvcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Thailand's Board of Film and Video Censors."]
        pub bfvc_rating: ::std::option::Option<ContentRatingBfvcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bmukkRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur)."]
        pub bmukk_rating: ::std::option::Option<ContentRatingBmukkRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "catvRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website."]
        pub catv_rating: ::std::option::Option<ContentRatingCatvRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "catvfrRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website."]
        pub catvfr_rating: ::std::option::Option<ContentRatingCatvfrRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cbfcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Central Board of Film Certification (CBFC - India) rating."]
        pub cbfc_rating: ::std::option::Option<ContentRatingCbfcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cccRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Consejo de Calificación Cinematográfica (Chile) rating."]
        pub ccc_rating: ::std::option::Option<ContentRatingCccRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cceRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Portugal's Comissão de Classificação de Espect´culos."]
        pub cce_rating: ::std::option::Option<ContentRatingCceRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Switzerland."]
        pub chfilm_rating: ::std::option::Option<ContentRatingChfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "chvrsRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Canadian Home Video Rating System (CHVRS) rating."]
        pub chvrs_rating: ::std::option::Option<ContentRatingChvrsRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cicfRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Commission de Contrôle des Films (Belgium)."]
        pub cicf_rating: ::std::option::Option<ContentRatingCicfRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cnaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA)."]
        pub cna_rating: ::std::option::Option<ContentRatingCnaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cncRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rating system in France - Commission de classification cinematographique"]
        pub cnc_rating: ::std::option::Option<ContentRatingCncRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "csaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content."]
        pub csa_rating: ::std::option::Option<ContentRatingCsaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cscfRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF)."]
        pub cscf_rating: ::std::option::Option<ContentRatingCscfRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "czfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in the Czech Republic."]
        pub czfilm_rating: ::std::option::Option<ContentRatingCzfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "djctqRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating."]
        pub djctq_rating: ::std::option::Option<ContentRatingDjctqRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "djctqRatingReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reasons that explain why the video received its DJCQT (Brazil) rating."]
        pub djctq_rating_reasons:
            ::std::option::Option<::std::vec::Vec<ContentRatingDjctqRatingReasonsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ecbmctRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism"]
        pub ecbmct_rating: ::std::option::Option<ContentRatingEcbmctRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eefilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Estonia."]
        pub eefilm_rating: ::std::option::Option<ContentRatingEefilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "egfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Egypt."]
        pub egfilm_rating: ::std::option::Option<ContentRatingEgfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eirinRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Eirin (映倫) rating. Eirin is the Japanese rating system."]
        pub eirin_rating: ::std::option::Option<ContentRatingEirinRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcbmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Malaysia's Film Censorship Board."]
        pub fcbm_rating: ::std::option::Option<ContentRatingFcbmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fcoRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration."]
        pub fco_rating: ::std::option::Option<ContentRatingFcoRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fmocRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This property has been deprecated. Use the contentDetails.contentRating.cncRating instead."]
        pub fmoc_rating: ::std::option::Option<ContentRatingFmocRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fpbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from South Africa's Film and Publication Board."]
        pub fpb_rating: ::std::option::Option<ContentRatingFpbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fpbRatingReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Reasons that explain why the video received its FPB (South Africa) rating."]
        pub fpb_rating_reasons:
            ::std::option::Option<::std::vec::Vec<ContentRatingFpbRatingReasonsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fskRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating."]
        pub fsk_rating: ::std::option::Option<ContentRatingFskRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "grfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Greece."]
        pub grfilm_rating: ::std::option::Option<ContentRatingGrfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icaaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating."]
        pub icaa_rating: ::std::option::Option<ContentRatingIcaaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ifcoRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information."]
        pub ifco_rating: ::std::option::Option<ContentRatingIfcoRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ilfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Israel."]
        pub ilfilm_rating: ::std::option::Option<ContentRatingIlfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "incaaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating."]
        pub incaa_rating: ::std::option::Option<ContentRatingIncaaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kfcbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Kenya Film Classification Board."]
        pub kfcb_rating: ::std::option::Option<ContentRatingKfcbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kijkwijzerRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands)."]
        pub kijkwijzer_rating: ::std::option::Option<ContentRatingKijkwijzerRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kmrbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea."]
        pub kmrb_rating: ::std::option::Option<ContentRatingKmrbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lsfRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Indonesia's Lembaga Sensor Film."]
        pub lsf_rating: ::std::option::Option<ContentRatingLsfRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mccaaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Malta's Film Age-Classification Board."]
        pub mccaa_rating: ::std::option::Option<ContentRatingMccaaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mccypRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People."]
        pub mccyp_rating: ::std::option::Option<ContentRatingMccypRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mcstRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating system for Vietnam - MCST"]
        pub mcst_rating: ::std::option::Option<ContentRatingMcstRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mdaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC)."]
        pub mda_rating: ::std::option::Option<ContentRatingMdaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medietilsynetRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Medietilsynet, the Norwegian Media Authority."]
        pub medietilsynet_rating: ::std::option::Option<ContentRatingMedietilsynetRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mekuRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute)."]
        pub meku_rating: ::std::option::Option<ContentRatingMekuRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "menaMpaaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rating system for MENA countries, a clone of MPAA. It is needed to prevent titles go live w/o additional QC check, since some of them can be inappropriate for the countries at all. See b/33408548 for more details."]
        pub mena_mpaa_rating: ::std::option::Option<ContentRatingMenaMpaaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mibacRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy)."]
        pub mibac_rating: ::std::option::Option<ContentRatingMibacRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mocRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Ministerio de Cultura (Colombia) rating."]
        pub moc_rating: ::std::option::Option<ContentRatingMocRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moctwRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Taiwan's Ministry of Culture (文化部)."]
        pub moctw_rating: ::std::option::Option<ContentRatingMoctwRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mpaaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Motion Picture Association of America (MPAA) rating."]
        pub mpaa_rating: ::std::option::Option<ContentRatingMpaaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mpaatRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html."]
        pub mpaat_rating: ::std::option::Option<ContentRatingMpaatRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mtrcbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Movie and Television Review and Classification Board (Philippines)."]
        pub mtrcb_rating: ::std::option::Option<ContentRatingMtrcbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nbcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Maldives National Bureau of Classification."]
        pub nbc_rating: ::std::option::Option<ContentRatingNbcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nbcplRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Poland."]
        pub nbcpl_rating: ::std::option::Option<ContentRatingNbcplRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nfrcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Bulgarian National Film Center."]
        pub nfrc_rating: ::std::option::Option<ContentRatingNfrcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nfvcbRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Nigeria's National Film and Video Censors Board."]
        pub nfvcb_rating: ::std::option::Option<ContentRatingNfvcbRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nkclvRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia)."]
        pub nkclv_rating: ::std::option::Option<ContentRatingNkclvRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nmcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The National Media Council ratings system for United Arab Emirates."]
        pub nmc_rating: ::std::option::Option<ContentRatingNmcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "oflcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's Office of Film and Literature Classification (OFLC - New Zealand) rating."]
        pub oflc_rating: ::std::option::Option<ContentRatingOflcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pefilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Peru."]
        pub pefilm_rating: ::std::option::Option<ContentRatingPefilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rcnofRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film."]
        pub rcnof_rating: ::std::option::Option<ContentRatingRcnofRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resorteviolenciaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Venezuela."]
        pub resorteviolencia_rating: ::std::option::Option<ContentRatingResorteviolenciaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rtcRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's General Directorate of Radio, Television and Cinematography (Mexico) rating."]
        pub rtc_rating: ::std::option::Option<ContentRatingRtcRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rteRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Ireland's Raidió Teilifís Éireann."]
        pub rte_rating: ::std::option::Option<ContentRatingRteRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "russiaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's National Film Registry of the Russian Federation (MKRF - Russia) rating."]
        pub russia_rating: ::std::option::Option<ContentRatingRussiaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "skfilmRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Slovakia."]
        pub skfilm_rating: ::std::option::Option<ContentRatingSkfilmRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smaisRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating in Iceland."]
        pub smais_rating: ::std::option::Option<ContentRatingSmaisRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smsaRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's rating from Statens medieråd (Sweden's National Media Council)."]
        pub smsa_rating: ::std::option::Option<ContentRatingSmsaRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tvpgRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's TV Parental Guidelines (TVPG) rating."]
        pub tvpg_rating: ::std::option::Option<ContentRatingTvpgRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ytRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rating that YouTube uses to identify age-restricted content."]
        pub yt_rating: ::std::option::Option<ContentRatingYtRatingEnum>,
    }
    impl ContentRating {
        pub fn builder() -> ContentRatingBuilder {
            ContentRatingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming."]
    pub enum ContentRatingAcbRatingEnum {
        #[serde(rename = "acbUnspecified")]
        #[doc = ""]
        AcbUnspecified,
        #[serde(rename = "acbE")]
        #[doc = "E"]
        AcbE,
        #[serde(rename = "acbP")]
        #[doc = "Programs that have been given a P classification by the Australian Communications and Media Authority. These programs are intended for preschool children."]
        AcbP,
        #[serde(rename = "acbC")]
        #[doc = "Programs that have been given a C classification by the Australian Communications and Media Authority. These programs are intended for children (other than preschool children) who are younger than 14 years of age."]
        AcbC,
        #[serde(rename = "acbG")]
        #[doc = "G"]
        AcbG,
        #[serde(rename = "acbPg")]
        #[doc = "PG"]
        AcbPg,
        #[serde(rename = "acbM")]
        #[doc = "M"]
        AcbM,
        #[serde(rename = "acbMa15plus")]
        #[doc = "MA15+"]
        AcbMa15plus,
        #[serde(rename = "acbR18plus")]
        #[doc = "R18+"]
        AcbR18plus,
        #[serde(rename = "acbUnrated")]
        #[doc = ""]
        AcbUnrated,
    }
    impl ::std::default::Default for ContentRatingAcbRatingEnum {
        fn default() -> Self {
            Self::AcbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM)."]
    pub enum ContentRatingAgcomRatingEnum {
        #[serde(rename = "agcomUnspecified")]
        #[doc = ""]
        AgcomUnspecified,
        #[serde(rename = "agcomT")]
        #[doc = "T"]
        AgcomT,
        #[serde(rename = "agcomVm14")]
        #[doc = "VM14"]
        AgcomVm14,
        #[serde(rename = "agcomVm18")]
        #[doc = "VM18"]
        AgcomVm18,
        #[serde(rename = "agcomUnrated")]
        #[doc = ""]
        AgcomUnrated,
    }
    impl ::std::default::Default for ContentRatingAgcomRatingEnum {
        fn default() -> Self {
            Self::AgcomUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television."]
    pub enum ContentRatingAnatelRatingEnum {
        #[serde(rename = "anatelUnspecified")]
        #[doc = ""]
        AnatelUnspecified,
        #[serde(rename = "anatelF")]
        #[doc = "F"]
        AnatelF,
        #[serde(rename = "anatelI")]
        #[doc = "I"]
        AnatelI,
        #[serde(rename = "anatelI7")]
        #[doc = "I-7"]
        AnatelI7,
        #[serde(rename = "anatelI10")]
        #[doc = "I-10"]
        AnatelI10,
        #[serde(rename = "anatelI12")]
        #[doc = "I-12"]
        AnatelI12,
        #[serde(rename = "anatelR")]
        #[doc = "R"]
        AnatelR,
        #[serde(rename = "anatelA")]
        #[doc = "A"]
        AnatelA,
        #[serde(rename = "anatelUnrated")]
        #[doc = ""]
        AnatelUnrated,
    }
    impl ::std::default::Default for ContentRatingAnatelRatingEnum {
        fn default() -> Self {
            Self::AnatelUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's British Board of Film Classification (BBFC) rating."]
    pub enum ContentRatingBbfcRatingEnum {
        #[serde(rename = "bbfcUnspecified")]
        #[doc = ""]
        BbfcUnspecified,
        #[serde(rename = "bbfcU")]
        #[doc = "U"]
        BbfcU,
        #[serde(rename = "bbfcPg")]
        #[doc = "PG"]
        BbfcPg,
        #[serde(rename = "bbfc12a")]
        #[doc = "12A"]
        Bbfc12a,
        #[serde(rename = "bbfc12")]
        #[doc = "12"]
        Bbfc12,
        #[serde(rename = "bbfc15")]
        #[doc = "15"]
        Bbfc15,
        #[serde(rename = "bbfc18")]
        #[doc = "18"]
        Bbfc18,
        #[serde(rename = "bbfcR18")]
        #[doc = "R18"]
        BbfcR18,
        #[serde(rename = "bbfcUnrated")]
        #[doc = ""]
        BbfcUnrated,
    }
    impl ::std::default::Default for ContentRatingBbfcRatingEnum {
        fn default() -> Self {
            Self::BbfcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Thailand's Board of Film and Video Censors."]
    pub enum ContentRatingBfvcRatingEnum {
        #[serde(rename = "bfvcUnspecified")]
        #[doc = ""]
        BfvcUnspecified,
        #[serde(rename = "bfvcG")]
        #[doc = "G"]
        BfvcG,
        #[serde(rename = "bfvcE")]
        #[doc = "E"]
        BfvcE,
        #[serde(rename = "bfvc13")]
        #[doc = "13"]
        Bfvc13,
        #[serde(rename = "bfvc15")]
        #[doc = "15"]
        Bfvc15,
        #[serde(rename = "bfvc18")]
        #[doc = "18"]
        Bfvc18,
        #[serde(rename = "bfvc20")]
        #[doc = "20"]
        Bfvc20,
        #[serde(rename = "bfvcB")]
        #[doc = "B"]
        BfvcB,
        #[serde(rename = "bfvcUnrated")]
        #[doc = ""]
        BfvcUnrated,
    }
    impl ::std::default::Default for ContentRatingBfvcRatingEnum {
        fn default() -> Self {
            Self::BfvcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur)."]
    pub enum ContentRatingBmukkRatingEnum {
        #[serde(rename = "bmukkUnspecified")]
        #[doc = ""]
        BmukkUnspecified,
        #[serde(rename = "bmukkAa")]
        #[doc = "Unrestricted"]
        BmukkAa,
        #[serde(rename = "bmukk6")]
        #[doc = "6+"]
        Bmukk6,
        #[serde(rename = "bmukk8")]
        #[doc = "8+"]
        Bmukk8,
        #[serde(rename = "bmukk10")]
        #[doc = "10+"]
        Bmukk10,
        #[serde(rename = "bmukk12")]
        #[doc = "12+"]
        Bmukk12,
        #[serde(rename = "bmukk14")]
        #[doc = "14+"]
        Bmukk14,
        #[serde(rename = "bmukk16")]
        #[doc = "16+"]
        Bmukk16,
        #[serde(rename = "bmukkUnrated")]
        #[doc = ""]
        BmukkUnrated,
    }
    impl ::std::default::Default for ContentRatingBmukkRatingEnum {
        fn default() -> Self {
            Self::BmukkUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website."]
    pub enum ContentRatingCatvRatingEnum {
        #[serde(rename = "catvUnspecified")]
        #[doc = ""]
        CatvUnspecified,
        #[serde(rename = "catvC")]
        #[doc = "C"]
        CatvC,
        #[serde(rename = "catvC8")]
        #[doc = "C8"]
        CatvC8,
        #[serde(rename = "catvG")]
        #[doc = "G"]
        CatvG,
        #[serde(rename = "catvPg")]
        #[doc = "PG"]
        CatvPg,
        #[serde(rename = "catv14plus")]
        #[doc = "14+"]
        Catv14plus,
        #[serde(rename = "catv18plus")]
        #[doc = "18+"]
        Catv18plus,
        #[serde(rename = "catvUnrated")]
        #[doc = ""]
        CatvUnrated,
        #[serde(rename = "catvE")]
        #[doc = ""]
        CatvE,
    }
    impl ::std::default::Default for ContentRatingCatvRatingEnum {
        fn default() -> Self {
            Self::CatvUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website."]
    pub enum ContentRatingCatvfrRatingEnum {
        #[serde(rename = "catvfrUnspecified")]
        #[doc = ""]
        CatvfrUnspecified,
        #[serde(rename = "catvfrG")]
        #[doc = "G"]
        CatvfrG,
        #[serde(rename = "catvfr8plus")]
        #[doc = "8+"]
        Catvfr8plus,
        #[serde(rename = "catvfr13plus")]
        #[doc = "13+"]
        Catvfr13plus,
        #[serde(rename = "catvfr16plus")]
        #[doc = "16+"]
        Catvfr16plus,
        #[serde(rename = "catvfr18plus")]
        #[doc = "18+"]
        Catvfr18plus,
        #[serde(rename = "catvfrUnrated")]
        #[doc = ""]
        CatvfrUnrated,
        #[serde(rename = "catvfrE")]
        #[doc = ""]
        CatvfrE,
    }
    impl ::std::default::Default for ContentRatingCatvfrRatingEnum {
        fn default() -> Self {
            Self::CatvfrUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Central Board of Film Certification (CBFC - India) rating."]
    pub enum ContentRatingCbfcRatingEnum {
        #[serde(rename = "cbfcUnspecified")]
        #[doc = ""]
        CbfcUnspecified,
        #[serde(rename = "cbfcU")]
        #[doc = "U"]
        CbfcU,
        #[serde(rename = "cbfcUA")]
        #[doc = "U/A"]
        CbfcUa,
        #[serde(rename = "cbfcA")]
        #[doc = "A"]
        CbfcA,
        #[serde(rename = "cbfcS")]
        #[doc = "S"]
        CbfcS,
        #[serde(rename = "cbfcUnrated")]
        #[doc = ""]
        CbfcUnrated,
    }
    impl ::std::default::Default for ContentRatingCbfcRatingEnum {
        fn default() -> Self {
            Self::CbfcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Consejo de Calificación Cinematográfica (Chile) rating."]
    pub enum ContentRatingCccRatingEnum {
        #[serde(rename = "cccUnspecified")]
        #[doc = ""]
        CccUnspecified,
        #[serde(rename = "cccTe")]
        #[doc = "Todo espectador"]
        CccTe,
        #[serde(rename = "ccc6")]
        #[doc = "6+ - Inconveniente para menores de 7 años"]
        Ccc6,
        #[serde(rename = "ccc14")]
        #[doc = "14+"]
        Ccc14,
        #[serde(rename = "ccc18")]
        #[doc = "18+"]
        Ccc18,
        #[serde(rename = "ccc18v")]
        #[doc = "18+ - contenido excesivamente violento"]
        Ccc18v,
        #[serde(rename = "ccc18s")]
        #[doc = "18+ - contenido pornográfico"]
        Ccc18s,
        #[serde(rename = "cccUnrated")]
        #[doc = ""]
        CccUnrated,
    }
    impl ::std::default::Default for ContentRatingCccRatingEnum {
        fn default() -> Self {
            Self::CccUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Portugal's Comissão de Classificação de Espect´culos."]
    pub enum ContentRatingCceRatingEnum {
        #[serde(rename = "cceUnspecified")]
        #[doc = ""]
        CceUnspecified,
        #[serde(rename = "cceM4")]
        #[doc = "4"]
        CceM4,
        #[serde(rename = "cceM6")]
        #[doc = "6"]
        CceM6,
        #[serde(rename = "cceM12")]
        #[doc = "12"]
        CceM12,
        #[serde(rename = "cceM16")]
        #[doc = "16"]
        CceM16,
        #[serde(rename = "cceM18")]
        #[doc = "18"]
        CceM18,
        #[serde(rename = "cceUnrated")]
        #[doc = ""]
        CceUnrated,
        #[serde(rename = "cceM14")]
        #[doc = "14"]
        CceM14,
    }
    impl ::std::default::Default for ContentRatingCceRatingEnum {
        fn default() -> Self {
            Self::CceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Switzerland."]
    pub enum ContentRatingChfilmRatingEnum {
        #[serde(rename = "chfilmUnspecified")]
        #[doc = ""]
        ChfilmUnspecified,
        #[serde(rename = "chfilm0")]
        #[doc = "0"]
        Chfilm0,
        #[serde(rename = "chfilm6")]
        #[doc = "6"]
        Chfilm6,
        #[serde(rename = "chfilm12")]
        #[doc = "12"]
        Chfilm12,
        #[serde(rename = "chfilm16")]
        #[doc = "16"]
        Chfilm16,
        #[serde(rename = "chfilm18")]
        #[doc = "18"]
        Chfilm18,
        #[serde(rename = "chfilmUnrated")]
        #[doc = ""]
        ChfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingChfilmRatingEnum {
        fn default() -> Self {
            Self::ChfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Canadian Home Video Rating System (CHVRS) rating."]
    pub enum ContentRatingChvrsRatingEnum {
        #[serde(rename = "chvrsUnspecified")]
        #[doc = ""]
        ChvrsUnspecified,
        #[serde(rename = "chvrsG")]
        #[doc = "G"]
        ChvrsG,
        #[serde(rename = "chvrsPg")]
        #[doc = "PG"]
        ChvrsPg,
        #[serde(rename = "chvrs14a")]
        #[doc = "14A"]
        Chvrs14a,
        #[serde(rename = "chvrs18a")]
        #[doc = "18A"]
        Chvrs18a,
        #[serde(rename = "chvrsR")]
        #[doc = "R"]
        ChvrsR,
        #[serde(rename = "chvrsE")]
        #[doc = "E"]
        ChvrsE,
        #[serde(rename = "chvrsUnrated")]
        #[doc = ""]
        ChvrsUnrated,
    }
    impl ::std::default::Default for ContentRatingChvrsRatingEnum {
        fn default() -> Self {
            Self::ChvrsUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Commission de Contrôle des Films (Belgium)."]
    pub enum ContentRatingCicfRatingEnum {
        #[serde(rename = "cicfUnspecified")]
        #[doc = ""]
        CicfUnspecified,
        #[serde(rename = "cicfE")]
        #[doc = "E"]
        CicfE,
        #[serde(rename = "cicfKtEa")]
        #[doc = "KT/EA"]
        CicfKtEa,
        #[serde(rename = "cicfKntEna")]
        #[doc = "KNT/ENA"]
        CicfKntEna,
        #[serde(rename = "cicfUnrated")]
        #[doc = ""]
        CicfUnrated,
    }
    impl ::std::default::Default for ContentRatingCicfRatingEnum {
        fn default() -> Self {
            Self::CicfUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA)."]
    pub enum ContentRatingCnaRatingEnum {
        #[serde(rename = "cnaUnspecified")]
        #[doc = ""]
        CnaUnspecified,
        #[serde(rename = "cnaAp")]
        #[doc = "AP"]
        CnaAp,
        #[serde(rename = "cna12")]
        #[doc = "12"]
        Cna12,
        #[serde(rename = "cna15")]
        #[doc = "15"]
        Cna15,
        #[serde(rename = "cna18")]
        #[doc = "18"]
        Cna18,
        #[serde(rename = "cna18plus")]
        #[doc = "18+"]
        Cna18plus,
        #[serde(rename = "cnaUnrated")]
        #[doc = ""]
        CnaUnrated,
    }
    impl ::std::default::Default for ContentRatingCnaRatingEnum {
        fn default() -> Self {
            Self::CnaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rating system in France - Commission de classification cinematographique"]
    pub enum ContentRatingCncRatingEnum {
        #[serde(rename = "cncUnspecified")]
        #[doc = ""]
        CncUnspecified,
        #[serde(rename = "cncT")]
        #[doc = "T"]
        CncT,
        #[serde(rename = "cnc10")]
        #[doc = "10"]
        Cnc10,
        #[serde(rename = "cnc12")]
        #[doc = "12"]
        Cnc12,
        #[serde(rename = "cnc16")]
        #[doc = "16"]
        Cnc16,
        #[serde(rename = "cnc18")]
        #[doc = "18"]
        Cnc18,
        #[serde(rename = "cncE")]
        #[doc = "E"]
        CncE,
        #[serde(rename = "cncInterdiction")]
        #[doc = "interdiction"]
        CncInterdiction,
        #[serde(rename = "cncUnrated")]
        #[doc = ""]
        CncUnrated,
    }
    impl ::std::default::Default for ContentRatingCncRatingEnum {
        fn default() -> Self {
            Self::CncUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content."]
    pub enum ContentRatingCsaRatingEnum {
        #[serde(rename = "csaUnspecified")]
        #[doc = ""]
        CsaUnspecified,
        #[serde(rename = "csaT")]
        #[doc = "T"]
        CsaT,
        #[serde(rename = "csa10")]
        #[doc = "10"]
        Csa10,
        #[serde(rename = "csa12")]
        #[doc = "12"]
        Csa12,
        #[serde(rename = "csa16")]
        #[doc = "16"]
        Csa16,
        #[serde(rename = "csa18")]
        #[doc = "18"]
        Csa18,
        #[serde(rename = "csaInterdiction")]
        #[doc = "Interdiction"]
        CsaInterdiction,
        #[serde(rename = "csaUnrated")]
        #[doc = ""]
        CsaUnrated,
    }
    impl ::std::default::Default for ContentRatingCsaRatingEnum {
        fn default() -> Self {
            Self::CsaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF)."]
    pub enum ContentRatingCscfRatingEnum {
        #[serde(rename = "cscfUnspecified")]
        #[doc = ""]
        CscfUnspecified,
        #[serde(rename = "cscfAl")]
        #[doc = "AL"]
        CscfAl,
        #[serde(rename = "cscfA")]
        #[doc = "A"]
        CscfA,
        #[serde(rename = "cscf6")]
        #[doc = "6"]
        Cscf6,
        #[serde(rename = "cscf9")]
        #[doc = "9"]
        Cscf9,
        #[serde(rename = "cscf12")]
        #[doc = "12"]
        Cscf12,
        #[serde(rename = "cscf16")]
        #[doc = "16"]
        Cscf16,
        #[serde(rename = "cscf18")]
        #[doc = "18"]
        Cscf18,
        #[serde(rename = "cscfUnrated")]
        #[doc = ""]
        CscfUnrated,
    }
    impl ::std::default::Default for ContentRatingCscfRatingEnum {
        fn default() -> Self {
            Self::CscfUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in the Czech Republic."]
    pub enum ContentRatingCzfilmRatingEnum {
        #[serde(rename = "czfilmUnspecified")]
        #[doc = ""]
        CzfilmUnspecified,
        #[serde(rename = "czfilmU")]
        #[doc = "U"]
        CzfilmU,
        #[serde(rename = "czfilm12")]
        #[doc = "12"]
        Czfilm12,
        #[serde(rename = "czfilm14")]
        #[doc = "14"]
        Czfilm14,
        #[serde(rename = "czfilm18")]
        #[doc = "18"]
        Czfilm18,
        #[serde(rename = "czfilmUnrated")]
        #[doc = ""]
        CzfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingCzfilmRatingEnum {
        fn default() -> Self {
            Self::CzfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating."]
    pub enum ContentRatingDjctqRatingEnum {
        #[serde(rename = "djctqUnspecified")]
        #[doc = ""]
        DjctqUnspecified,
        #[serde(rename = "djctqL")]
        #[doc = "L"]
        DjctqL,
        #[serde(rename = "djctq10")]
        #[doc = "10"]
        Djctq10,
        #[serde(rename = "djctq12")]
        #[doc = "12"]
        Djctq12,
        #[serde(rename = "djctq14")]
        #[doc = "14"]
        Djctq14,
        #[serde(rename = "djctq16")]
        #[doc = "16"]
        Djctq16,
        #[serde(rename = "djctq18")]
        #[doc = "18"]
        Djctq18,
        #[serde(rename = "djctqEr")]
        #[doc = ""]
        DjctqEr,
        #[serde(rename = "djctqL10")]
        #[doc = ""]
        DjctqL10,
        #[serde(rename = "djctqL12")]
        #[doc = ""]
        DjctqL12,
        #[serde(rename = "djctqL14")]
        #[doc = ""]
        DjctqL14,
        #[serde(rename = "djctqL16")]
        #[doc = ""]
        DjctqL16,
        #[serde(rename = "djctqL18")]
        #[doc = ""]
        DjctqL18,
        #[serde(rename = "djctq1012")]
        #[doc = ""]
        Djctq1012,
        #[serde(rename = "djctq1014")]
        #[doc = ""]
        Djctq1014,
        #[serde(rename = "djctq1016")]
        #[doc = ""]
        Djctq1016,
        #[serde(rename = "djctq1018")]
        #[doc = ""]
        Djctq1018,
        #[serde(rename = "djctq1214")]
        #[doc = ""]
        Djctq1214,
        #[serde(rename = "djctq1216")]
        #[doc = ""]
        Djctq1216,
        #[serde(rename = "djctq1218")]
        #[doc = ""]
        Djctq1218,
        #[serde(rename = "djctq1416")]
        #[doc = ""]
        Djctq1416,
        #[serde(rename = "djctq1418")]
        #[doc = ""]
        Djctq1418,
        #[serde(rename = "djctq1618")]
        #[doc = ""]
        Djctq1618,
        #[serde(rename = "djctqUnrated")]
        #[doc = ""]
        DjctqUnrated,
    }
    impl ::std::default::Default for ContentRatingDjctqRatingEnum {
        fn default() -> Self {
            Self::DjctqUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ContentRatingDjctqRatingReasonsEnum {
        #[serde(rename = "djctqRatingReasonUnspecified")]
        #[doc = ""]
        DjctqRatingReasonUnspecified,
        #[serde(rename = "djctqViolence")]
        #[doc = "Brazil rating content descriptors. See http://go/brazilratings section F. Violência (Violence)"]
        DjctqViolence,
        #[serde(rename = "djctqExtremeViolence")]
        #[doc = "Violência extrema (Extreme violence)"]
        DjctqExtremeViolence,
        #[serde(rename = "djctqSexualContent")]
        #[doc = "Conteúdo sexual (Sexual content)"]
        DjctqSexualContent,
        #[serde(rename = "djctqNudity")]
        #[doc = "Nudez (Nudity)"]
        DjctqNudity,
        #[serde(rename = "djctqSex")]
        #[doc = "Sexo (Sex)"]
        DjctqSex,
        #[serde(rename = "djctqExplicitSex")]
        #[doc = "Sexo Explícito (Explicit sex)"]
        DjctqExplicitSex,
        #[serde(rename = "djctqDrugs")]
        #[doc = "Drogas (Drugs)"]
        DjctqDrugs,
        #[serde(rename = "djctqLegalDrugs")]
        #[doc = "Drogas Lícitas (Legal drugs)"]
        DjctqLegalDrugs,
        #[serde(rename = "djctqIllegalDrugs")]
        #[doc = "Drogas Ilícitas (Illegal drugs)"]
        DjctqIllegalDrugs,
        #[serde(rename = "djctqInappropriateLanguage")]
        #[doc = "Linguagem Imprópria (Inappropriate language)"]
        DjctqInappropriateLanguage,
        #[serde(rename = "djctqCriminalActs")]
        #[doc = "Atos Criminosos (Criminal Acts)"]
        DjctqCriminalActs,
        #[serde(rename = "djctqImpactingContent")]
        #[doc = "Conteúdo Impactante (Impacting content)"]
        DjctqImpactingContent,
    }
    impl ::std::default::Default for ContentRatingDjctqRatingReasonsEnum {
        fn default() -> Self {
            Self::DjctqRatingReasonUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism"]
    pub enum ContentRatingEcbmctRatingEnum {
        #[serde(rename = "ecbmctUnspecified")]
        #[doc = ""]
        EcbmctUnspecified,
        #[serde(rename = "ecbmctG")]
        #[doc = "G"]
        EcbmctG,
        #[serde(rename = "ecbmct7a")]
        #[doc = "7A"]
        Ecbmct7a,
        #[serde(rename = "ecbmct7plus")]
        #[doc = "7+"]
        Ecbmct7plus,
        #[serde(rename = "ecbmct13a")]
        #[doc = "13A"]
        Ecbmct13a,
        #[serde(rename = "ecbmct13plus")]
        #[doc = "13+"]
        Ecbmct13plus,
        #[serde(rename = "ecbmct15a")]
        #[doc = "15A"]
        Ecbmct15a,
        #[serde(rename = "ecbmct15plus")]
        #[doc = "15+"]
        Ecbmct15plus,
        #[serde(rename = "ecbmct18plus")]
        #[doc = "18+"]
        Ecbmct18plus,
        #[serde(rename = "ecbmctUnrated")]
        #[doc = ""]
        EcbmctUnrated,
    }
    impl ::std::default::Default for ContentRatingEcbmctRatingEnum {
        fn default() -> Self {
            Self::EcbmctUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Estonia."]
    pub enum ContentRatingEefilmRatingEnum {
        #[serde(rename = "eefilmUnspecified")]
        #[doc = ""]
        EefilmUnspecified,
        #[serde(rename = "eefilmPere")]
        #[doc = "Pere"]
        EefilmPere,
        #[serde(rename = "eefilmL")]
        #[doc = "L"]
        EefilmL,
        #[serde(rename = "eefilmMs6")]
        #[doc = "MS-6"]
        EefilmMs6,
        #[serde(rename = "eefilmK6")]
        #[doc = "K-6"]
        EefilmK6,
        #[serde(rename = "eefilmMs12")]
        #[doc = "MS-12"]
        EefilmMs12,
        #[serde(rename = "eefilmK12")]
        #[doc = "K-12"]
        EefilmK12,
        #[serde(rename = "eefilmK14")]
        #[doc = "K-14"]
        EefilmK14,
        #[serde(rename = "eefilmK16")]
        #[doc = "K-16"]
        EefilmK16,
        #[serde(rename = "eefilmUnrated")]
        #[doc = ""]
        EefilmUnrated,
    }
    impl ::std::default::Default for ContentRatingEefilmRatingEnum {
        fn default() -> Self {
            Self::EefilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Egypt."]
    pub enum ContentRatingEgfilmRatingEnum {
        #[serde(rename = "egfilmUnspecified")]
        #[doc = ""]
        EgfilmUnspecified,
        #[serde(rename = "egfilmGn")]
        #[doc = "GN"]
        EgfilmGn,
        #[serde(rename = "egfilm18")]
        #[doc = "18"]
        Egfilm18,
        #[serde(rename = "egfilmBn")]
        #[doc = "BN"]
        EgfilmBn,
        #[serde(rename = "egfilmUnrated")]
        #[doc = ""]
        EgfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingEgfilmRatingEnum {
        fn default() -> Self {
            Self::EgfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Eirin (映倫) rating. Eirin is the Japanese rating system."]
    pub enum ContentRatingEirinRatingEnum {
        #[serde(rename = "eirinUnspecified")]
        #[doc = ""]
        EirinUnspecified,
        #[serde(rename = "eirinG")]
        #[doc = "G"]
        EirinG,
        #[serde(rename = "eirinPg12")]
        #[doc = "PG-12"]
        EirinPg12,
        #[serde(rename = "eirinR15plus")]
        #[doc = "R15+"]
        EirinR15plus,
        #[serde(rename = "eirinR18plus")]
        #[doc = "R18+"]
        EirinR18plus,
        #[serde(rename = "eirinUnrated")]
        #[doc = ""]
        EirinUnrated,
    }
    impl ::std::default::Default for ContentRatingEirinRatingEnum {
        fn default() -> Self {
            Self::EirinUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Malaysia's Film Censorship Board."]
    pub enum ContentRatingFcbmRatingEnum {
        #[serde(rename = "fcbmUnspecified")]
        #[doc = ""]
        FcbmUnspecified,
        #[serde(rename = "fcbmU")]
        #[doc = "U"]
        FcbmU,
        #[serde(rename = "fcbmPg13")]
        #[doc = "PG13"]
        FcbmPg13,
        #[serde(rename = "fcbmP13")]
        #[doc = "P13"]
        FcbmP13,
        #[serde(rename = "fcbm18")]
        #[doc = "18"]
        Fcbm18,
        #[serde(rename = "fcbm18sx")]
        #[doc = "18SX"]
        Fcbm18sx,
        #[serde(rename = "fcbm18pa")]
        #[doc = "18PA"]
        Fcbm18pa,
        #[serde(rename = "fcbm18sg")]
        #[doc = "18SG"]
        Fcbm18sg,
        #[serde(rename = "fcbm18pl")]
        #[doc = "18PL"]
        Fcbm18pl,
        #[serde(rename = "fcbmUnrated")]
        #[doc = ""]
        FcbmUnrated,
    }
    impl ::std::default::Default for ContentRatingFcbmRatingEnum {
        fn default() -> Self {
            Self::FcbmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration."]
    pub enum ContentRatingFcoRatingEnum {
        #[serde(rename = "fcoUnspecified")]
        #[doc = ""]
        FcoUnspecified,
        #[serde(rename = "fcoI")]
        #[doc = "I"]
        FcoI,
        #[serde(rename = "fcoIia")]
        #[doc = "IIA"]
        FcoIia,
        #[serde(rename = "fcoIib")]
        #[doc = "IIB"]
        FcoIib,
        #[serde(rename = "fcoIi")]
        #[doc = "II"]
        FcoIi,
        #[serde(rename = "fcoIii")]
        #[doc = "III"]
        FcoIii,
        #[serde(rename = "fcoUnrated")]
        #[doc = ""]
        FcoUnrated,
    }
    impl ::std::default::Default for ContentRatingFcoRatingEnum {
        fn default() -> Self {
            Self::FcoUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This property has been deprecated. Use the contentDetails.contentRating.cncRating instead."]
    pub enum ContentRatingFmocRatingEnum {
        #[serde(rename = "fmocUnspecified")]
        #[doc = ""]
        FmocUnspecified,
        #[serde(rename = "fmocU")]
        #[doc = "U"]
        FmocU,
        #[serde(rename = "fmoc10")]
        #[doc = "10"]
        Fmoc10,
        #[serde(rename = "fmoc12")]
        #[doc = "12"]
        Fmoc12,
        #[serde(rename = "fmoc16")]
        #[doc = "16"]
        Fmoc16,
        #[serde(rename = "fmoc18")]
        #[doc = "18"]
        Fmoc18,
        #[serde(rename = "fmocE")]
        #[doc = "E"]
        FmocE,
        #[serde(rename = "fmocUnrated")]
        #[doc = ""]
        FmocUnrated,
    }
    impl ::std::default::Default for ContentRatingFmocRatingEnum {
        fn default() -> Self {
            Self::FmocUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from South Africa's Film and Publication Board."]
    pub enum ContentRatingFpbRatingEnum {
        #[serde(rename = "fpbUnspecified")]
        #[doc = ""]
        FpbUnspecified,
        #[serde(rename = "fpbA")]
        #[doc = "A"]
        FpbA,
        #[serde(rename = "fpbPg")]
        #[doc = "PG"]
        FpbPg,
        #[serde(rename = "fpb79Pg")]
        #[doc = "7-9PG"]
        Fpb79Pg,
        #[serde(rename = "fpb1012Pg")]
        #[doc = "10-12PG"]
        Fpb1012Pg,
        #[serde(rename = "fpb13")]
        #[doc = "13"]
        Fpb13,
        #[serde(rename = "fpb16")]
        #[doc = "16"]
        Fpb16,
        #[serde(rename = "fpb18")]
        #[doc = "18"]
        Fpb18,
        #[serde(rename = "fpbX18")]
        #[doc = "X18"]
        FpbX18,
        #[serde(rename = "fpbXx")]
        #[doc = "XX"]
        FpbXx,
        #[serde(rename = "fpbUnrated")]
        #[doc = ""]
        FpbUnrated,
        #[serde(rename = "fpb10")]
        #[doc = "10"]
        Fpb10,
    }
    impl ::std::default::Default for ContentRatingFpbRatingEnum {
        fn default() -> Self {
            Self::FpbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ContentRatingFpbRatingReasonsEnum {
        #[serde(rename = "fpbRatingReasonUnspecified")]
        #[doc = ""]
        FpbRatingReasonUnspecified,
        #[serde(rename = "fpbBlasphemy")]
        #[doc = "South Africa rating content descriptors."]
        FpbBlasphemy,
        #[serde(rename = "fpbLanguage")]
        #[doc = ""]
        FpbLanguage,
        #[serde(rename = "fpbNudity")]
        #[doc = ""]
        FpbNudity,
        #[serde(rename = "fpbPrejudice")]
        #[doc = ""]
        FpbPrejudice,
        #[serde(rename = "fpbSex")]
        #[doc = ""]
        FpbSex,
        #[serde(rename = "fpbViolence")]
        #[doc = ""]
        FpbViolence,
        #[serde(rename = "fpbDrugs")]
        #[doc = ""]
        FpbDrugs,
        #[serde(rename = "fpbSexualViolence")]
        #[doc = ""]
        FpbSexualViolence,
        #[serde(rename = "fpbHorror")]
        #[doc = ""]
        FpbHorror,
        #[serde(rename = "fpbCriminalTechniques")]
        #[doc = ""]
        FpbCriminalTechniques,
        #[serde(rename = "fpbImitativeActsTechniques")]
        #[doc = ""]
        FpbImitativeActsTechniques,
    }
    impl ::std::default::Default for ContentRatingFpbRatingReasonsEnum {
        fn default() -> Self {
            Self::FpbRatingReasonUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating."]
    pub enum ContentRatingFskRatingEnum {
        #[serde(rename = "fskUnspecified")]
        #[doc = ""]
        FskUnspecified,
        #[serde(rename = "fsk0")]
        #[doc = "FSK 0"]
        Fsk0,
        #[serde(rename = "fsk6")]
        #[doc = "FSK 6"]
        Fsk6,
        #[serde(rename = "fsk12")]
        #[doc = "FSK 12"]
        Fsk12,
        #[serde(rename = "fsk16")]
        #[doc = "FSK 16"]
        Fsk16,
        #[serde(rename = "fsk18")]
        #[doc = "FSK 18"]
        Fsk18,
        #[serde(rename = "fskUnrated")]
        #[doc = ""]
        FskUnrated,
    }
    impl ::std::default::Default for ContentRatingFskRatingEnum {
        fn default() -> Self {
            Self::FskUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Greece."]
    pub enum ContentRatingGrfilmRatingEnum {
        #[serde(rename = "grfilmUnspecified")]
        #[doc = ""]
        GrfilmUnspecified,
        #[serde(rename = "grfilmK")]
        #[doc = "K"]
        GrfilmK,
        #[serde(rename = "grfilmE")]
        #[doc = "E"]
        GrfilmE,
        #[serde(rename = "grfilmK12")]
        #[doc = "K-12"]
        GrfilmK12,
        #[serde(rename = "grfilmK13")]
        #[doc = "K-13"]
        GrfilmK13,
        #[serde(rename = "grfilmK15")]
        #[doc = "K-15"]
        GrfilmK15,
        #[serde(rename = "grfilmK17")]
        #[doc = "K-17"]
        GrfilmK17,
        #[serde(rename = "grfilmK18")]
        #[doc = "K-18"]
        GrfilmK18,
        #[serde(rename = "grfilmUnrated")]
        #[doc = ""]
        GrfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingGrfilmRatingEnum {
        fn default() -> Self {
            Self::GrfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating."]
    pub enum ContentRatingIcaaRatingEnum {
        #[serde(rename = "icaaUnspecified")]
        #[doc = ""]
        IcaaUnspecified,
        #[serde(rename = "icaaApta")]
        #[doc = "APTA"]
        IcaaApta,
        #[serde(rename = "icaa7")]
        #[doc = "7"]
        Icaa7,
        #[serde(rename = "icaa12")]
        #[doc = "12"]
        Icaa12,
        #[serde(rename = "icaa13")]
        #[doc = "13"]
        Icaa13,
        #[serde(rename = "icaa16")]
        #[doc = "16"]
        Icaa16,
        #[serde(rename = "icaa18")]
        #[doc = "18"]
        Icaa18,
        #[serde(rename = "icaaX")]
        #[doc = "X"]
        IcaaX,
        #[serde(rename = "icaaUnrated")]
        #[doc = ""]
        IcaaUnrated,
    }
    impl ::std::default::Default for ContentRatingIcaaRatingEnum {
        fn default() -> Self {
            Self::IcaaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information."]
    pub enum ContentRatingIfcoRatingEnum {
        #[serde(rename = "ifcoUnspecified")]
        #[doc = ""]
        IfcoUnspecified,
        #[serde(rename = "ifcoG")]
        #[doc = "G"]
        IfcoG,
        #[serde(rename = "ifcoPg")]
        #[doc = "PG"]
        IfcoPg,
        #[serde(rename = "ifco12")]
        #[doc = "12"]
        Ifco12,
        #[serde(rename = "ifco12a")]
        #[doc = "12A"]
        Ifco12a,
        #[serde(rename = "ifco15")]
        #[doc = "15"]
        Ifco15,
        #[serde(rename = "ifco15a")]
        #[doc = "15A"]
        Ifco15a,
        #[serde(rename = "ifco16")]
        #[doc = "16"]
        Ifco16,
        #[serde(rename = "ifco18")]
        #[doc = "18"]
        Ifco18,
        #[serde(rename = "ifcoUnrated")]
        #[doc = ""]
        IfcoUnrated,
    }
    impl ::std::default::Default for ContentRatingIfcoRatingEnum {
        fn default() -> Self {
            Self::IfcoUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Israel."]
    pub enum ContentRatingIlfilmRatingEnum {
        #[serde(rename = "ilfilmUnspecified")]
        #[doc = ""]
        IlfilmUnspecified,
        #[serde(rename = "ilfilmAa")]
        #[doc = "AA"]
        IlfilmAa,
        #[serde(rename = "ilfilm12")]
        #[doc = "12"]
        Ilfilm12,
        #[serde(rename = "ilfilm14")]
        #[doc = "14"]
        Ilfilm14,
        #[serde(rename = "ilfilm16")]
        #[doc = "16"]
        Ilfilm16,
        #[serde(rename = "ilfilm18")]
        #[doc = "18"]
        Ilfilm18,
        #[serde(rename = "ilfilmUnrated")]
        #[doc = ""]
        IlfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingIlfilmRatingEnum {
        fn default() -> Self {
            Self::IlfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating."]
    pub enum ContentRatingIncaaRatingEnum {
        #[serde(rename = "incaaUnspecified")]
        #[doc = ""]
        IncaaUnspecified,
        #[serde(rename = "incaaAtp")]
        #[doc = "ATP (Apta para todo publico)"]
        IncaaAtp,
        #[serde(rename = "incaaSam13")]
        #[doc = "13 (Solo apta para mayores de 13 años)"]
        IncaaSam13,
        #[serde(rename = "incaaSam16")]
        #[doc = "16 (Solo apta para mayores de 16 años)"]
        IncaaSam16,
        #[serde(rename = "incaaSam18")]
        #[doc = "18 (Solo apta para mayores de 18 años)"]
        IncaaSam18,
        #[serde(rename = "incaaC")]
        #[doc = "X (Solo apta para mayores de 18 años, de exhibición condicionada)"]
        IncaaC,
        #[serde(rename = "incaaUnrated")]
        #[doc = ""]
        IncaaUnrated,
    }
    impl ::std::default::Default for ContentRatingIncaaRatingEnum {
        fn default() -> Self {
            Self::IncaaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Kenya Film Classification Board."]
    pub enum ContentRatingKfcbRatingEnum {
        #[serde(rename = "kfcbUnspecified")]
        #[doc = ""]
        KfcbUnspecified,
        #[serde(rename = "kfcbG")]
        #[doc = "GE"]
        KfcbG,
        #[serde(rename = "kfcbPg")]
        #[doc = "PG"]
        KfcbPg,
        #[serde(rename = "kfcb16plus")]
        #[doc = "16"]
        Kfcb16plus,
        #[serde(rename = "kfcbR")]
        #[doc = "18"]
        KfcbR,
        #[serde(rename = "kfcbUnrated")]
        #[doc = ""]
        KfcbUnrated,
    }
    impl ::std::default::Default for ContentRatingKfcbRatingEnum {
        fn default() -> Self {
            Self::KfcbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands)."]
    pub enum ContentRatingKijkwijzerRatingEnum {
        #[serde(rename = "kijkwijzerUnspecified")]
        #[doc = ""]
        KijkwijzerUnspecified,
        #[serde(rename = "kijkwijzerAl")]
        #[doc = "AL"]
        KijkwijzerAl,
        #[serde(rename = "kijkwijzer6")]
        #[doc = "6"]
        Kijkwijzer6,
        #[serde(rename = "kijkwijzer9")]
        #[doc = "9"]
        Kijkwijzer9,
        #[serde(rename = "kijkwijzer12")]
        #[doc = "12"]
        Kijkwijzer12,
        #[serde(rename = "kijkwijzer16")]
        #[doc = "16"]
        Kijkwijzer16,
        #[serde(rename = "kijkwijzer18")]
        #[doc = ""]
        Kijkwijzer18,
        #[serde(rename = "kijkwijzerUnrated")]
        #[doc = ""]
        KijkwijzerUnrated,
    }
    impl ::std::default::Default for ContentRatingKijkwijzerRatingEnum {
        fn default() -> Self {
            Self::KijkwijzerUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea."]
    pub enum ContentRatingKmrbRatingEnum {
        #[serde(rename = "kmrbUnspecified")]
        #[doc = ""]
        KmrbUnspecified,
        #[serde(rename = "kmrbAll")]
        #[doc = "전체관람가"]
        KmrbAll,
        #[serde(rename = "kmrb12plus")]
        #[doc = "12세 이상 관람가"]
        Kmrb12plus,
        #[serde(rename = "kmrb15plus")]
        #[doc = "15세 이상 관람가"]
        Kmrb15plus,
        #[serde(rename = "kmrbTeenr")]
        #[doc = ""]
        KmrbTeenr,
        #[serde(rename = "kmrbR")]
        #[doc = "청소년 관람불가"]
        KmrbR,
        #[serde(rename = "kmrbUnrated")]
        #[doc = ""]
        KmrbUnrated,
    }
    impl ::std::default::Default for ContentRatingKmrbRatingEnum {
        fn default() -> Self {
            Self::KmrbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Indonesia's Lembaga Sensor Film."]
    pub enum ContentRatingLsfRatingEnum {
        #[serde(rename = "lsfUnspecified")]
        #[doc = ""]
        LsfUnspecified,
        #[serde(rename = "lsfSu")]
        #[doc = "SU"]
        LsfSu,
        #[serde(rename = "lsfA")]
        #[doc = "A"]
        LsfA,
        #[serde(rename = "lsfBo")]
        #[doc = "BO"]
        LsfBo,
        #[serde(rename = "lsf13")]
        #[doc = "13"]
        Lsf13,
        #[serde(rename = "lsfR")]
        #[doc = "R"]
        LsfR,
        #[serde(rename = "lsf17")]
        #[doc = "17"]
        Lsf17,
        #[serde(rename = "lsfD")]
        #[doc = "D"]
        LsfD,
        #[serde(rename = "lsf21")]
        #[doc = "21"]
        Lsf21,
        #[serde(rename = "lsfUnrated")]
        #[doc = ""]
        LsfUnrated,
    }
    impl ::std::default::Default for ContentRatingLsfRatingEnum {
        fn default() -> Self {
            Self::LsfUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Malta's Film Age-Classification Board."]
    pub enum ContentRatingMccaaRatingEnum {
        #[serde(rename = "mccaaUnspecified")]
        #[doc = ""]
        MccaaUnspecified,
        #[serde(rename = "mccaaU")]
        #[doc = "U"]
        MccaaU,
        #[serde(rename = "mccaaPg")]
        #[doc = "PG"]
        MccaaPg,
        #[serde(rename = "mccaa12a")]
        #[doc = "12A"]
        Mccaa12a,
        #[serde(rename = "mccaa12")]
        #[doc = "12"]
        Mccaa12,
        #[serde(rename = "mccaa14")]
        #[doc = "14 - this rating was removed from the new classification structure introduced in 2013."]
        Mccaa14,
        #[serde(rename = "mccaa15")]
        #[doc = "15"]
        Mccaa15,
        #[serde(rename = "mccaa16")]
        #[doc = "16 - this rating was removed from the new classification structure introduced in 2013."]
        Mccaa16,
        #[serde(rename = "mccaa18")]
        #[doc = "18"]
        Mccaa18,
        #[serde(rename = "mccaaUnrated")]
        #[doc = ""]
        MccaaUnrated,
    }
    impl ::std::default::Default for ContentRatingMccaaRatingEnum {
        fn default() -> Self {
            Self::MccaaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People."]
    pub enum ContentRatingMccypRatingEnum {
        #[serde(rename = "mccypUnspecified")]
        #[doc = ""]
        MccypUnspecified,
        #[serde(rename = "mccypA")]
        #[doc = "A"]
        MccypA,
        #[serde(rename = "mccyp7")]
        #[doc = "7"]
        Mccyp7,
        #[serde(rename = "mccyp11")]
        #[doc = "11"]
        Mccyp11,
        #[serde(rename = "mccyp15")]
        #[doc = "15"]
        Mccyp15,
        #[serde(rename = "mccypUnrated")]
        #[doc = ""]
        MccypUnrated,
    }
    impl ::std::default::Default for ContentRatingMccypRatingEnum {
        fn default() -> Self {
            Self::MccypUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating system for Vietnam - MCST"]
    pub enum ContentRatingMcstRatingEnum {
        #[serde(rename = "mcstUnspecified")]
        #[doc = ""]
        McstUnspecified,
        #[serde(rename = "mcstP")]
        #[doc = "P"]
        McstP,
        #[serde(rename = "mcst0")]
        #[doc = "0"]
        Mcst0,
        #[serde(rename = "mcstC13")]
        #[doc = "C13"]
        McstC13,
        #[serde(rename = "mcstC16")]
        #[doc = "C16"]
        McstC16,
        #[serde(rename = "mcst16plus")]
        #[doc = "16+"]
        Mcst16plus,
        #[serde(rename = "mcstC18")]
        #[doc = "C18"]
        McstC18,
        #[serde(rename = "mcstGPg")]
        #[doc = "MCST_G_PG"]
        McstGPg,
        #[serde(rename = "mcstUnrated")]
        #[doc = ""]
        McstUnrated,
    }
    impl ::std::default::Default for ContentRatingMcstRatingEnum {
        fn default() -> Self {
            Self::McstUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC)."]
    pub enum ContentRatingMdaRatingEnum {
        #[serde(rename = "mdaUnspecified")]
        #[doc = ""]
        MdaUnspecified,
        #[serde(rename = "mdaG")]
        #[doc = "G"]
        MdaG,
        #[serde(rename = "mdaPg")]
        #[doc = "PG"]
        MdaPg,
        #[serde(rename = "mdaPg13")]
        #[doc = "PG13"]
        MdaPg13,
        #[serde(rename = "mdaNc16")]
        #[doc = "NC16"]
        MdaNc16,
        #[serde(rename = "mdaM18")]
        #[doc = "M18"]
        MdaM18,
        #[serde(rename = "mdaR21")]
        #[doc = "R21"]
        MdaR21,
        #[serde(rename = "mdaUnrated")]
        #[doc = ""]
        MdaUnrated,
    }
    impl ::std::default::Default for ContentRatingMdaRatingEnum {
        fn default() -> Self {
            Self::MdaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Medietilsynet, the Norwegian Media Authority."]
    pub enum ContentRatingMedietilsynetRatingEnum {
        #[serde(rename = "medietilsynetUnspecified")]
        #[doc = ""]
        MedietilsynetUnspecified,
        #[serde(rename = "medietilsynetA")]
        #[doc = "A"]
        MedietilsynetA,
        #[serde(rename = "medietilsynet6")]
        #[doc = "6"]
        Medietilsynet6,
        #[serde(rename = "medietilsynet7")]
        #[doc = "7"]
        Medietilsynet7,
        #[serde(rename = "medietilsynet9")]
        #[doc = "9"]
        Medietilsynet9,
        #[serde(rename = "medietilsynet11")]
        #[doc = "11"]
        Medietilsynet11,
        #[serde(rename = "medietilsynet12")]
        #[doc = "12"]
        Medietilsynet12,
        #[serde(rename = "medietilsynet15")]
        #[doc = "15"]
        Medietilsynet15,
        #[serde(rename = "medietilsynet18")]
        #[doc = "18"]
        Medietilsynet18,
        #[serde(rename = "medietilsynetUnrated")]
        #[doc = ""]
        MedietilsynetUnrated,
    }
    impl ::std::default::Default for ContentRatingMedietilsynetRatingEnum {
        fn default() -> Self {
            Self::MedietilsynetUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute)."]
    pub enum ContentRatingMekuRatingEnum {
        #[serde(rename = "mekuUnspecified")]
        #[doc = ""]
        MekuUnspecified,
        #[serde(rename = "mekuS")]
        #[doc = "S"]
        MekuS,
        #[serde(rename = "meku7")]
        #[doc = "7"]
        Meku7,
        #[serde(rename = "meku12")]
        #[doc = "12"]
        Meku12,
        #[serde(rename = "meku16")]
        #[doc = "16"]
        Meku16,
        #[serde(rename = "meku18")]
        #[doc = "18"]
        Meku18,
        #[serde(rename = "mekuUnrated")]
        #[doc = ""]
        MekuUnrated,
    }
    impl ::std::default::Default for ContentRatingMekuRatingEnum {
        fn default() -> Self {
            Self::MekuUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The rating system for MENA countries, a clone of MPAA. It is needed to prevent titles go live w/o additional QC check, since some of them can be inappropriate for the countries at all. See b/33408548 for more details."]
    pub enum ContentRatingMenaMpaaRatingEnum {
        #[serde(rename = "menaMpaaUnspecified")]
        #[doc = ""]
        MenaMpaaUnspecified,
        #[serde(rename = "menaMpaaG")]
        #[doc = "G"]
        MenaMpaaG,
        #[serde(rename = "menaMpaaPg")]
        #[doc = "PG"]
        MenaMpaaPg,
        #[serde(rename = "menaMpaaPg13")]
        #[doc = "PG-13"]
        MenaMpaaPg13,
        #[serde(rename = "menaMpaaR")]
        #[doc = "R"]
        MenaMpaaR,
        #[serde(rename = "menaMpaaUnrated")]
        #[doc = "To keep the same enum values as MPAA's items have, skip NC_17."]
        MenaMpaaUnrated,
    }
    impl ::std::default::Default for ContentRatingMenaMpaaRatingEnum {
        fn default() -> Self {
            Self::MenaMpaaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy)."]
    pub enum ContentRatingMibacRatingEnum {
        #[serde(rename = "mibacUnspecified")]
        #[doc = ""]
        MibacUnspecified,
        #[serde(rename = "mibacT")]
        #[doc = ""]
        MibacT,
        #[serde(rename = "mibacVap")]
        #[doc = ""]
        MibacVap,
        #[serde(rename = "mibacVm12")]
        #[doc = ""]
        MibacVm12,
        #[serde(rename = "mibacVm14")]
        #[doc = ""]
        MibacVm14,
        #[serde(rename = "mibacVm18")]
        #[doc = ""]
        MibacVm18,
        #[serde(rename = "mibacUnrated")]
        #[doc = ""]
        MibacUnrated,
    }
    impl ::std::default::Default for ContentRatingMibacRatingEnum {
        fn default() -> Self {
            Self::MibacUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Ministerio de Cultura (Colombia) rating."]
    pub enum ContentRatingMocRatingEnum {
        #[serde(rename = "mocUnspecified")]
        #[doc = ""]
        MocUnspecified,
        #[serde(rename = "mocE")]
        #[doc = "E"]
        MocE,
        #[serde(rename = "mocT")]
        #[doc = "T"]
        MocT,
        #[serde(rename = "moc7")]
        #[doc = "7"]
        Moc7,
        #[serde(rename = "moc12")]
        #[doc = "12"]
        Moc12,
        #[serde(rename = "moc15")]
        #[doc = "15"]
        Moc15,
        #[serde(rename = "moc18")]
        #[doc = "18"]
        Moc18,
        #[serde(rename = "mocX")]
        #[doc = "X"]
        MocX,
        #[serde(rename = "mocBanned")]
        #[doc = "Banned"]
        MocBanned,
        #[serde(rename = "mocUnrated")]
        #[doc = ""]
        MocUnrated,
    }
    impl ::std::default::Default for ContentRatingMocRatingEnum {
        fn default() -> Self {
            Self::MocUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Taiwan's Ministry of Culture (文化部)."]
    pub enum ContentRatingMoctwRatingEnum {
        #[serde(rename = "moctwUnspecified")]
        #[doc = ""]
        MoctwUnspecified,
        #[serde(rename = "moctwG")]
        #[doc = "G"]
        MoctwG,
        #[serde(rename = "moctwP")]
        #[doc = "P"]
        MoctwP,
        #[serde(rename = "moctwPg")]
        #[doc = "PG"]
        MoctwPg,
        #[serde(rename = "moctwR")]
        #[doc = "R"]
        MoctwR,
        #[serde(rename = "moctwUnrated")]
        #[doc = ""]
        MoctwUnrated,
        #[serde(rename = "moctwR12")]
        #[doc = "R-12"]
        MoctwR12,
        #[serde(rename = "moctwR15")]
        #[doc = "R-15"]
        MoctwR15,
    }
    impl ::std::default::Default for ContentRatingMoctwRatingEnum {
        fn default() -> Self {
            Self::MoctwUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Motion Picture Association of America (MPAA) rating."]
    pub enum ContentRatingMpaaRatingEnum {
        #[serde(rename = "mpaaUnspecified")]
        #[doc = ""]
        MpaaUnspecified,
        #[serde(rename = "mpaaG")]
        #[doc = "G"]
        MpaaG,
        #[serde(rename = "mpaaPg")]
        #[doc = "PG"]
        MpaaPg,
        #[serde(rename = "mpaaPg13")]
        #[doc = "PG-13"]
        MpaaPg13,
        #[serde(rename = "mpaaR")]
        #[doc = "R"]
        MpaaR,
        #[serde(rename = "mpaaNc17")]
        #[doc = "NC-17"]
        MpaaNc17,
        #[serde(rename = "mpaaX")]
        #[doc = "! X"]
        MpaaX,
        #[serde(rename = "mpaaUnrated")]
        #[doc = ""]
        MpaaUnrated,
    }
    impl ::std::default::Default for ContentRatingMpaaRatingEnum {
        fn default() -> Self {
            Self::MpaaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html."]
    pub enum ContentRatingMpaatRatingEnum {
        #[serde(rename = "mpaatUnspecified")]
        #[doc = ""]
        MpaatUnspecified,
        #[serde(rename = "mpaatGb")]
        #[doc = "GB"]
        MpaatGb,
        #[serde(rename = "mpaatRb")]
        #[doc = "RB"]
        MpaatRb,
    }
    impl ::std::default::Default for ContentRatingMpaatRatingEnum {
        fn default() -> Self {
            Self::MpaatUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Movie and Television Review and Classification Board (Philippines)."]
    pub enum ContentRatingMtrcbRatingEnum {
        #[serde(rename = "mtrcbUnspecified")]
        #[doc = ""]
        MtrcbUnspecified,
        #[serde(rename = "mtrcbG")]
        #[doc = "G"]
        MtrcbG,
        #[serde(rename = "mtrcbPg")]
        #[doc = "PG"]
        MtrcbPg,
        #[serde(rename = "mtrcbR13")]
        #[doc = "R-13"]
        MtrcbR13,
        #[serde(rename = "mtrcbR16")]
        #[doc = "R-16"]
        MtrcbR16,
        #[serde(rename = "mtrcbR18")]
        #[doc = "R-18"]
        MtrcbR18,
        #[serde(rename = "mtrcbX")]
        #[doc = "X"]
        MtrcbX,
        #[serde(rename = "mtrcbUnrated")]
        #[doc = ""]
        MtrcbUnrated,
    }
    impl ::std::default::Default for ContentRatingMtrcbRatingEnum {
        fn default() -> Self {
            Self::MtrcbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Maldives National Bureau of Classification."]
    pub enum ContentRatingNbcRatingEnum {
        #[serde(rename = "nbcUnspecified")]
        #[doc = ""]
        NbcUnspecified,
        #[serde(rename = "nbcG")]
        #[doc = "G"]
        NbcG,
        #[serde(rename = "nbcPg")]
        #[doc = "PG"]
        NbcPg,
        #[serde(rename = "nbc12plus")]
        #[doc = "12+"]
        Nbc12plus,
        #[serde(rename = "nbc15plus")]
        #[doc = "15+"]
        Nbc15plus,
        #[serde(rename = "nbc18plus")]
        #[doc = "18+"]
        Nbc18plus,
        #[serde(rename = "nbc18plusr")]
        #[doc = "18+R"]
        Nbc18plusr,
        #[serde(rename = "nbcPu")]
        #[doc = "PU"]
        NbcPu,
        #[serde(rename = "nbcUnrated")]
        #[doc = ""]
        NbcUnrated,
    }
    impl ::std::default::Default for ContentRatingNbcRatingEnum {
        fn default() -> Self {
            Self::NbcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Poland."]
    pub enum ContentRatingNbcplRatingEnum {
        #[serde(rename = "nbcplUnspecified")]
        #[doc = ""]
        NbcplUnspecified,
        #[serde(rename = "nbcplI")]
        #[doc = ""]
        NbcplI,
        #[serde(rename = "nbcplIi")]
        #[doc = ""]
        NbcplIi,
        #[serde(rename = "nbcplIii")]
        #[doc = ""]
        NbcplIii,
        #[serde(rename = "nbcplIv")]
        #[doc = ""]
        NbcplIv,
        #[serde(rename = "nbcpl18plus")]
        #[doc = ""]
        Nbcpl18plus,
        #[serde(rename = "nbcplUnrated")]
        #[doc = ""]
        NbcplUnrated,
    }
    impl ::std::default::Default for ContentRatingNbcplRatingEnum {
        fn default() -> Self {
            Self::NbcplUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Bulgarian National Film Center."]
    pub enum ContentRatingNfrcRatingEnum {
        #[serde(rename = "nfrcUnspecified")]
        #[doc = ""]
        NfrcUnspecified,
        #[serde(rename = "nfrcA")]
        #[doc = "A"]
        NfrcA,
        #[serde(rename = "nfrcB")]
        #[doc = "B"]
        NfrcB,
        #[serde(rename = "nfrcC")]
        #[doc = "C"]
        NfrcC,
        #[serde(rename = "nfrcD")]
        #[doc = "D"]
        NfrcD,
        #[serde(rename = "nfrcX")]
        #[doc = "X"]
        NfrcX,
        #[serde(rename = "nfrcUnrated")]
        #[doc = ""]
        NfrcUnrated,
    }
    impl ::std::default::Default for ContentRatingNfrcRatingEnum {
        fn default() -> Self {
            Self::NfrcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Nigeria's National Film and Video Censors Board."]
    pub enum ContentRatingNfvcbRatingEnum {
        #[serde(rename = "nfvcbUnspecified")]
        #[doc = ""]
        NfvcbUnspecified,
        #[serde(rename = "nfvcbG")]
        #[doc = "G"]
        NfvcbG,
        #[serde(rename = "nfvcbPg")]
        #[doc = "PG"]
        NfvcbPg,
        #[serde(rename = "nfvcb12")]
        #[doc = "12"]
        Nfvcb12,
        #[serde(rename = "nfvcb12a")]
        #[doc = "12A"]
        Nfvcb12a,
        #[serde(rename = "nfvcb15")]
        #[doc = "15"]
        Nfvcb15,
        #[serde(rename = "nfvcb18")]
        #[doc = "18"]
        Nfvcb18,
        #[serde(rename = "nfvcbRe")]
        #[doc = "RE"]
        NfvcbRe,
        #[serde(rename = "nfvcbUnrated")]
        #[doc = ""]
        NfvcbUnrated,
    }
    impl ::std::default::Default for ContentRatingNfvcbRatingEnum {
        fn default() -> Self {
            Self::NfvcbUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia)."]
    pub enum ContentRatingNkclvRatingEnum {
        #[serde(rename = "nkclvUnspecified")]
        #[doc = ""]
        NkclvUnspecified,
        #[serde(rename = "nkclvU")]
        #[doc = "U"]
        NkclvU,
        #[serde(rename = "nkclv7plus")]
        #[doc = "7+"]
        Nkclv7plus,
        #[serde(rename = "nkclv12plus")]
        #[doc = "12+"]
        Nkclv12plus,
        #[serde(rename = "nkclv16plus")]
        #[doc = "! 16+"]
        Nkclv16plus,
        #[serde(rename = "nkclv18plus")]
        #[doc = "18+"]
        Nkclv18plus,
        #[serde(rename = "nkclvUnrated")]
        #[doc = ""]
        NkclvUnrated,
    }
    impl ::std::default::Default for ContentRatingNkclvRatingEnum {
        fn default() -> Self {
            Self::NkclvUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The National Media Council ratings system for United Arab Emirates."]
    pub enum ContentRatingNmcRatingEnum {
        #[serde(rename = "nmcUnspecified")]
        #[doc = ""]
        NmcUnspecified,
        #[serde(rename = "nmcG")]
        #[doc = "G"]
        NmcG,
        #[serde(rename = "nmcPg")]
        #[doc = "PG"]
        NmcPg,
        #[serde(rename = "nmcPg13")]
        #[doc = "PG-13"]
        NmcPg13,
        #[serde(rename = "nmcPg15")]
        #[doc = "PG-15"]
        NmcPg15,
        #[serde(rename = "nmc15plus")]
        #[doc = "15+"]
        Nmc15plus,
        #[serde(rename = "nmc18plus")]
        #[doc = "18+"]
        Nmc18plus,
        #[serde(rename = "nmc18tc")]
        #[doc = "18TC"]
        Nmc18tc,
        #[serde(rename = "nmcUnrated")]
        #[doc = ""]
        NmcUnrated,
    }
    impl ::std::default::Default for ContentRatingNmcRatingEnum {
        fn default() -> Self {
            Self::NmcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's Office of Film and Literature Classification (OFLC - New Zealand) rating."]
    pub enum ContentRatingOflcRatingEnum {
        #[serde(rename = "oflcUnspecified")]
        #[doc = ""]
        OflcUnspecified,
        #[serde(rename = "oflcG")]
        #[doc = "G"]
        OflcG,
        #[serde(rename = "oflcPg")]
        #[doc = "PG"]
        OflcPg,
        #[serde(rename = "oflcM")]
        #[doc = "M"]
        OflcM,
        #[serde(rename = "oflcR13")]
        #[doc = "R13"]
        OflcR13,
        #[serde(rename = "oflcR15")]
        #[doc = "R15"]
        OflcR15,
        #[serde(rename = "oflcR16")]
        #[doc = "R16"]
        OflcR16,
        #[serde(rename = "oflcR18")]
        #[doc = "R18"]
        OflcR18,
        #[serde(rename = "oflcUnrated")]
        #[doc = ""]
        OflcUnrated,
        #[serde(rename = "oflcRp13")]
        #[doc = "RP13"]
        OflcRp13,
        #[serde(rename = "oflcRp16")]
        #[doc = "RP16"]
        OflcRp16,
        #[serde(rename = "oflcRp18")]
        #[doc = "RP18"]
        OflcRp18,
    }
    impl ::std::default::Default for ContentRatingOflcRatingEnum {
        fn default() -> Self {
            Self::OflcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Peru."]
    pub enum ContentRatingPefilmRatingEnum {
        #[serde(rename = "pefilmUnspecified")]
        #[doc = ""]
        PefilmUnspecified,
        #[serde(rename = "pefilmPt")]
        #[doc = "PT"]
        PefilmPt,
        #[serde(rename = "pefilmPg")]
        #[doc = "PG"]
        PefilmPg,
        #[serde(rename = "pefilm14")]
        #[doc = "14"]
        Pefilm14,
        #[serde(rename = "pefilm18")]
        #[doc = "18"]
        Pefilm18,
        #[serde(rename = "pefilmUnrated")]
        #[doc = ""]
        PefilmUnrated,
    }
    impl ::std::default::Default for ContentRatingPefilmRatingEnum {
        fn default() -> Self {
            Self::PefilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film."]
    pub enum ContentRatingRcnofRatingEnum {
        #[serde(rename = "rcnofUnspecified")]
        #[doc = ""]
        RcnofUnspecified,
        #[serde(rename = "rcnofI")]
        #[doc = ""]
        RcnofI,
        #[serde(rename = "rcnofIi")]
        #[doc = ""]
        RcnofIi,
        #[serde(rename = "rcnofIii")]
        #[doc = ""]
        RcnofIii,
        #[serde(rename = "rcnofIv")]
        #[doc = ""]
        RcnofIv,
        #[serde(rename = "rcnofV")]
        #[doc = ""]
        RcnofV,
        #[serde(rename = "rcnofVi")]
        #[doc = ""]
        RcnofVi,
        #[serde(rename = "rcnofUnrated")]
        #[doc = ""]
        RcnofUnrated,
    }
    impl ::std::default::Default for ContentRatingRcnofRatingEnum {
        fn default() -> Self {
            Self::RcnofUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Venezuela."]
    pub enum ContentRatingResorteviolenciaRatingEnum {
        #[serde(rename = "resorteviolenciaUnspecified")]
        #[doc = ""]
        ResorteviolenciaUnspecified,
        #[serde(rename = "resorteviolenciaA")]
        #[doc = "A"]
        ResorteviolenciaA,
        #[serde(rename = "resorteviolenciaB")]
        #[doc = "B"]
        ResorteviolenciaB,
        #[serde(rename = "resorteviolenciaC")]
        #[doc = "C"]
        ResorteviolenciaC,
        #[serde(rename = "resorteviolenciaD")]
        #[doc = "D"]
        ResorteviolenciaD,
        #[serde(rename = "resorteviolenciaE")]
        #[doc = "E"]
        ResorteviolenciaE,
        #[serde(rename = "resorteviolenciaUnrated")]
        #[doc = ""]
        ResorteviolenciaUnrated,
    }
    impl ::std::default::Default for ContentRatingResorteviolenciaRatingEnum {
        fn default() -> Self {
            Self::ResorteviolenciaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's General Directorate of Radio, Television and Cinematography (Mexico) rating."]
    pub enum ContentRatingRtcRatingEnum {
        #[serde(rename = "rtcUnspecified")]
        #[doc = ""]
        RtcUnspecified,
        #[serde(rename = "rtcAa")]
        #[doc = "AA"]
        RtcAa,
        #[serde(rename = "rtcA")]
        #[doc = "A"]
        RtcA,
        #[serde(rename = "rtcB")]
        #[doc = "B"]
        RtcB,
        #[serde(rename = "rtcB15")]
        #[doc = "B15"]
        RtcB15,
        #[serde(rename = "rtcC")]
        #[doc = "C"]
        RtcC,
        #[serde(rename = "rtcD")]
        #[doc = "D"]
        RtcD,
        #[serde(rename = "rtcUnrated")]
        #[doc = ""]
        RtcUnrated,
    }
    impl ::std::default::Default for ContentRatingRtcRatingEnum {
        fn default() -> Self {
            Self::RtcUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Ireland's Raidió Teilifís Éireann."]
    pub enum ContentRatingRteRatingEnum {
        #[serde(rename = "rteUnspecified")]
        #[doc = ""]
        RteUnspecified,
        #[serde(rename = "rteGa")]
        #[doc = "GA"]
        RteGa,
        #[serde(rename = "rteCh")]
        #[doc = "CH"]
        RteCh,
        #[serde(rename = "rtePs")]
        #[doc = "PS"]
        RtePs,
        #[serde(rename = "rteMa")]
        #[doc = "MA"]
        RteMa,
        #[serde(rename = "rteUnrated")]
        #[doc = ""]
        RteUnrated,
    }
    impl ::std::default::Default for ContentRatingRteRatingEnum {
        fn default() -> Self {
            Self::RteUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's National Film Registry of the Russian Federation (MKRF - Russia) rating."]
    pub enum ContentRatingRussiaRatingEnum {
        #[serde(rename = "russiaUnspecified")]
        #[doc = ""]
        RussiaUnspecified,
        #[serde(rename = "russia0")]
        #[doc = "0+"]
        Russia0,
        #[serde(rename = "russia6")]
        #[doc = "6+"]
        Russia6,
        #[serde(rename = "russia12")]
        #[doc = "12+"]
        Russia12,
        #[serde(rename = "russia16")]
        #[doc = "16+"]
        Russia16,
        #[serde(rename = "russia18")]
        #[doc = "18+"]
        Russia18,
        #[serde(rename = "russiaUnrated")]
        #[doc = ""]
        RussiaUnrated,
    }
    impl ::std::default::Default for ContentRatingRussiaRatingEnum {
        fn default() -> Self {
            Self::RussiaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Slovakia."]
    pub enum ContentRatingSkfilmRatingEnum {
        #[serde(rename = "skfilmUnspecified")]
        #[doc = ""]
        SkfilmUnspecified,
        #[serde(rename = "skfilmG")]
        #[doc = "G"]
        SkfilmG,
        #[serde(rename = "skfilmP2")]
        #[doc = "P2"]
        SkfilmP2,
        #[serde(rename = "skfilmP5")]
        #[doc = "P5"]
        SkfilmP5,
        #[serde(rename = "skfilmP8")]
        #[doc = "P8"]
        SkfilmP8,
        #[serde(rename = "skfilmUnrated")]
        #[doc = ""]
        SkfilmUnrated,
    }
    impl ::std::default::Default for ContentRatingSkfilmRatingEnum {
        fn default() -> Self {
            Self::SkfilmUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating in Iceland."]
    pub enum ContentRatingSmaisRatingEnum {
        #[serde(rename = "smaisUnspecified")]
        #[doc = ""]
        SmaisUnspecified,
        #[serde(rename = "smaisL")]
        #[doc = "L"]
        SmaisL,
        #[serde(rename = "smais7")]
        #[doc = "7"]
        Smais7,
        #[serde(rename = "smais12")]
        #[doc = "12"]
        Smais12,
        #[serde(rename = "smais14")]
        #[doc = "14"]
        Smais14,
        #[serde(rename = "smais16")]
        #[doc = "16"]
        Smais16,
        #[serde(rename = "smais18")]
        #[doc = "18"]
        Smais18,
        #[serde(rename = "smaisUnrated")]
        #[doc = ""]
        SmaisUnrated,
    }
    impl ::std::default::Default for ContentRatingSmaisRatingEnum {
        fn default() -> Self {
            Self::SmaisUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's rating from Statens medieråd (Sweden's National Media Council)."]
    pub enum ContentRatingSmsaRatingEnum {
        #[serde(rename = "smsaUnspecified")]
        #[doc = ""]
        SmsaUnspecified,
        #[serde(rename = "smsaA")]
        #[doc = "All ages"]
        SmsaA,
        #[serde(rename = "smsa7")]
        #[doc = "7"]
        Smsa7,
        #[serde(rename = "smsa11")]
        #[doc = "11"]
        Smsa11,
        #[serde(rename = "smsa15")]
        #[doc = "15"]
        Smsa15,
        #[serde(rename = "smsaUnrated")]
        #[doc = ""]
        SmsaUnrated,
    }
    impl ::std::default::Default for ContentRatingSmsaRatingEnum {
        fn default() -> Self {
            Self::SmsaUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's TV Parental Guidelines (TVPG) rating."]
    pub enum ContentRatingTvpgRatingEnum {
        #[serde(rename = "tvpgUnspecified")]
        #[doc = ""]
        TvpgUnspecified,
        #[serde(rename = "tvpgY")]
        #[doc = "TV-Y"]
        TvpgY,
        #[serde(rename = "tvpgY7")]
        #[doc = "TV-Y7"]
        TvpgY7,
        #[serde(rename = "tvpgY7Fv")]
        #[doc = "TV-Y7-FV"]
        TvpgY7Fv,
        #[serde(rename = "tvpgG")]
        #[doc = "TV-G"]
        TvpgG,
        #[serde(rename = "tvpgPg")]
        #[doc = "TV-PG"]
        TvpgPg,
        #[serde(rename = "pg14")]
        #[doc = "TV-14"]
        Pg14,
        #[serde(rename = "tvpgMa")]
        #[doc = "TV-MA"]
        TvpgMa,
        #[serde(rename = "tvpgUnrated")]
        #[doc = ""]
        TvpgUnrated,
    }
    impl ::std::default::Default for ContentRatingTvpgRatingEnum {
        fn default() -> Self {
            Self::TvpgUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "A rating that YouTube uses to identify age-restricted content."]
    pub enum ContentRatingYtRatingEnum {
        #[serde(rename = "ytUnspecified")]
        #[doc = ""]
        YtUnspecified,
        #[serde(rename = "ytAgeRestricted")]
        #[doc = ""]
        YtAgeRestricted,
    }
    impl ::std::default::Default for ContentRatingYtRatingEnum {
        fn default() -> Self {
            Self::YtUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct Entity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "typeId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub type_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }
    impl Entity {
        pub fn builder() -> EntityBuilder {
            EntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Geographical coordinates of a point, in WGS84."]
    pub struct GeoPoint {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Altitude above the reference ellipsoid, in meters."]
        pub altitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Latitude in degrees."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Longitude in degrees."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl GeoPoint {
        pub fn builder() -> GeoPointBuilder {
            GeoPointBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An *i18nLanguage* resource identifies a UI language currently supported by YouTube."]
    pub struct I18nLanguage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the i18n language."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ i18n_language_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "i18n_language_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#i18nLanguage\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the i18n language, such as language code and human-readable name."]
        pub snippet: ::std::option::Option<::std::boxed::Box<I18nLanguageSnippet>>,
    }
    impl I18nLanguage {
        pub fn builder() -> I18nLanguageBuilder {
            I18nLanguageBuilder::default()
        }
    }
    mod i18n_language_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#i18nLanguage")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct I18nLanguageListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<I18nLanguage>>>,
        #[builder(
            default = "{ i18n_language_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "i18n_language_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#i18nLanguageListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl I18nLanguageListResponse {
        pub fn builder() -> I18nLanguageListResponseBuilder {
            I18nLanguageListResponseBuilder::default()
        }
    }
    mod i18n_language_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#i18nLanguageListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about an i18n language, such as language code and human-readable name."]
    pub struct I18nLanguageSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short BCP-47 code that uniquely identifies a language."]
        pub hl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the language in the language itself."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl I18nLanguageSnippet {
        pub fn builder() -> I18nLanguageSnippetBuilder {
            I18nLanguageSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *i18nRegion* resource identifies a region where YouTube is available."]
    pub struct I18nRegion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the i18n region."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ i18n_region_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "i18n_region_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#i18nRegion\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the i18n region, such as region code and human-readable name."]
        pub snippet: ::std::option::Option<::std::boxed::Box<I18nRegionSnippet>>,
    }
    impl I18nRegion {
        pub fn builder() -> I18nRegionBuilder {
            I18nRegionBuilder::default()
        }
    }
    mod i18n_region_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#i18nRegion")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct I18nRegionListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<I18nRegion>>>,
        #[builder(
            default = "{ i18n_region_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "i18n_region_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#i18nRegionListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl I18nRegionListResponse {
        pub fn builder() -> I18nRegionListResponseBuilder {
            I18nRegionListResponseBuilder::default()
        }
    }
    mod i18n_region_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#i18nRegionListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about an i18n region, such as region code and human-readable name."]
    pub struct I18nRegionSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The region code as a 2-letter ISO country code."]
        pub gl: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable name of the region."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl I18nRegionSnippet {
        pub fn builder() -> I18nRegionSnippetBuilder {
            I18nRegionSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Branding properties for images associated with the channel."]
    pub struct ImageSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the background image shown on the video watch page. The image should be 1200px by 615px, with a maximum file size of 128k."]
        pub background_image_url: ::std::option::Option<::std::boxed::Box<LocalizedProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerExternalUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This is generated when a ChannelBanner.Insert request has succeeded for the given channel."]
        pub banner_external_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Desktop size (1060x175)."]
        pub banner_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerMobileExtraHdImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Mobile size high resolution (1440x395)."]
        pub banner_mobile_extra_hd_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerMobileHdImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Mobile size high resolution (1280x360)."]
        pub banner_mobile_hd_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerMobileImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Mobile size (640x175)."]
        pub banner_mobile_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerMobileLowImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Mobile size low resolution (320x88)."]
        pub banner_mobile_low_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerMobileMediumHdImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Mobile size medium/high resolution (960x263)."]
        pub banner_mobile_medium_hd_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTabletExtraHdImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Tablet size extra high resolution (2560x424)."]
        pub banner_tablet_extra_hd_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTabletHdImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Tablet size high resolution (2276x377)."]
        pub banner_tablet_hd_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTabletImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Tablet size (1707x283)."]
        pub banner_tablet_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTabletLowImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. Tablet size low resolution (1138x188)."]
        pub banner_tablet_low_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTvHighImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. TV size high resolution (1920x1080)."]
        pub banner_tv_high_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTvImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. TV size extra high resolution (2120x1192)."]
        pub banner_tv_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTvLowImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. TV size low resolution (854x480)."]
        pub banner_tv_low_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannerTvMediumImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Banner image. TV size medium resolution (1280x720)."]
        pub banner_tv_medium_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeBrandedBannerImageImapScript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image map script for the large banner image."]
        pub large_branded_banner_image_imap_script:
            ::std::option::Option<::std::boxed::Box<LocalizedProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeBrandedBannerImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the 854px by 70px image that appears below the video player in the expanded video view of the video watch page."]
        pub large_branded_banner_image_url:
            ::std::option::Option<::std::boxed::Box<LocalizedProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smallBrandedBannerImageImapScript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image map script for the small banner image."]
        pub small_branded_banner_image_imap_script:
            ::std::option::Option<::std::boxed::Box<LocalizedProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "smallBrandedBannerImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for the 640px by 70px banner image that appears below the video player in the default view of the video watch page. The URL for the image that appears above the top-left corner of the video player. This is a 25-pixel-high image with a flexible width that cannot exceed 170 pixels."]
        pub small_branded_banner_image_url:
            ::std::option::Option<::std::boxed::Box<LocalizedProperty>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages."]
        pub tracking_image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "watchIconImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub watch_icon_image_url: ::std::option::Option<::std::string::String>,
    }
    impl ImageSettings {
        pub fn builder() -> ImageSettingsBuilder {
            ImageSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes information necessary for ingesting an RTMP or an HTTP stream."]
    pub struct IngestionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backupIngestionAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The backup ingestion URL that you should use to stream video to YouTube. You have the option of simultaneously streaming the content that you are sending to the ingestionAddress to this URL."]
        pub backup_ingestion_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ingestionAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The primary ingestion URL that you should use to stream video to YouTube. You must stream video to this URL. Depending on which application or tool you use to encode your video stream, you may need to enter the stream URL and stream name separately or you may need to concatenate them in the following format: *STREAM_URL/STREAM_NAME* "]
        pub ingestion_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rtmpsBackupIngestionAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This ingestion url may be used instead of backupIngestionAddress in order to stream via RTMPS. Not applicable to non-RTMP streams."]
        pub rtmps_backup_ingestion_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rtmpsIngestionAddress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This ingestion url may be used instead of ingestionAddress in order to stream via RTMPS. Not applicable to non-RTMP streams."]
        pub rtmps_ingestion_address: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP or RTMP stream name that YouTube assigns to the video stream."]
        pub stream_name: ::std::option::Option<::std::string::String>,
    }
    impl IngestionInfo {
        pub fn builder() -> IngestionInfoBuilder {
            IngestionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "LINT.IfChange Describes an invideo branding."]
    pub struct InvideoBranding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The bytes the uploaded image. Only used in api to youtube communication."]
        pub image_bytes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The url of the uploaded image. Only used in apiary to api communication."]
        pub image_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The spatial position within the video where the branding watermark will be displayed."]
        pub position: ::std::option::Option<::std::boxed::Box<InvideoPosition>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel to which this branding links. If not present it defaults to the current channel."]
        pub target_channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timing")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The temporal position within the video where watermark will be displayed."]
        pub timing: ::std::option::Option<::std::boxed::Box<InvideoTiming>>,
    }
    impl InvideoBranding {
        pub fn builder() -> InvideoBrandingBuilder {
            InvideoBrandingBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one."]
    pub struct InvideoPosition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cornerPosition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes in which corner of the video the visual widget will appear."]
        pub corner_position: ::std::option::Option<InvideoPositionCornerPositionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the position type."]
        pub _type: ::std::option::Option<InvideoPositionTypeEnum>,
    }
    impl InvideoPosition {
        pub fn builder() -> InvideoPositionBuilder {
            InvideoPositionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Describes in which corner of the video the visual widget will appear."]
    pub enum InvideoPositionCornerPositionEnum {
        #[serde(rename = "topLeft")]
        #[doc = ""]
        TopLeft,
        #[serde(rename = "topRight")]
        #[doc = ""]
        TopRight,
        #[serde(rename = "bottomLeft")]
        #[doc = ""]
        BottomLeft,
        #[serde(rename = "bottomRight")]
        #[doc = ""]
        BottomRight,
    }
    impl ::std::default::Default for InvideoPositionCornerPositionEnum {
        fn default() -> Self {
            Self::TopLeft
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Defines the position type."]
    pub enum InvideoPositionTypeEnum {
        #[serde(rename = "corner")]
        #[doc = ""]
        Corner,
    }
    impl ::std::default::Default for InvideoPositionTypeEnum {
        fn default() -> Self {
            Self::Corner
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a temporal position of a visual widget inside a video."]
    pub struct InvideoTiming {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default."]
        pub duration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offsetMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds."]
        pub offset_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video."]
        pub _type: ::std::option::Option<InvideoTimingTypeEnum>,
    }
    impl InvideoTiming {
        pub fn builder() -> InvideoTimingBuilder {
            InvideoTimingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video."]
    pub enum InvideoTimingTypeEnum {
        #[serde(rename = "offsetFromStart")]
        #[doc = ""]
        OffsetFromStart,
        #[serde(rename = "offsetFromEnd")]
        #[doc = ""]
        OffsetFromEnd,
    }
    impl ::std::default::Default for InvideoTimingTypeEnum {
        fn default() -> Self {
            Self::OffsetFromStart
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LanguageTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl LanguageTag {
        pub fn builder() -> LanguageTagBuilder {
            LanguageTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LevelDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name that should be used when referring to this level."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl LevelDetails {
        pub fn builder() -> LevelDetailsBuilder {
            LevelDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *liveBroadcast* resource represents an event that will be streamed, via live video, on YouTube."]
    pub struct LiveBroadcast {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains information about the event's video content, such as whether the content can be shown in an embedded video player or if it will be archived and therefore available for viewing after the event has concluded."]
        pub content_details: ::std::option::Option<::std::boxed::Box<LiveBroadcastContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the broadcast."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ live_broadcast_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "live_broadcast_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveBroadcast\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the event, including its title, description, start time, and end time."]
        pub snippet: ::std::option::Option<::std::boxed::Box<LiveBroadcastSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statistics object contains info about the event's current stats. These include concurrent viewers and total chat count. Statistics can change (in either direction) during the lifetime of an event. Statistics are only returned while the event is live."]
        pub statistics: ::std::option::Option<::std::boxed::Box<LiveBroadcastStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains information about the event's status."]
        pub status: ::std::option::Option<::std::boxed::Box<LiveBroadcastStatus>>,
    }
    impl LiveBroadcast {
        pub fn builder() -> LiveBroadcastBuilder {
            LiveBroadcastBuilder::default()
        }
    }
    mod live_broadcast_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveBroadcast")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed settings of a broadcast."]
    pub struct LiveBroadcastContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundStreamId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value uniquely identifies the live stream bound to the broadcast."]
        pub bound_stream_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boundStreamLastUpdateTimeMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the live stream referenced by boundStreamId was last updated."]
        pub bound_stream_last_update_time_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "closedCaptionsType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub closed_captions_type:
            ::std::option::Option<LiveBroadcastContentDetailsClosedCaptionsTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutoStart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether auto start is enabled for this broadcast. The default value for this property is false. This setting can only be used by Events."]
        pub enable_auto_start: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableAutoStop")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether auto stop is enabled for this broadcast. The default value for this property is false. This setting can only be used by Events."]
        pub enable_auto_stop: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableClosedCaptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether HTTP POST closed captioning is enabled for this broadcast. The ingestion URL of the closed captions is returned through the liveStreams API. This is mutually exclusive with using the closed_captions_type property, and is equivalent to setting closed_captions_type to CLOSED_CAPTIONS_HTTP_POST."]
        pub enable_closed_captions: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableContentEncryption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether YouTube should enable content encryption for the broadcast."]
        pub enable_content_encryption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableDvr")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting determines whether viewers can access DVR controls while watching the video. DVR controls enable the viewer to control the video playback experience by pausing, rewinding, or fast forwarding content. The default value for this property is true. *Important:* You must set the value to true and also set the enableArchive property's value to true if you want to make playback available immediately after the broadcast ends."]
        pub enable_dvr: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableEmbed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether the broadcast video can be played in an embedded player. If you choose to archive the video (using the enableArchive property), this setting will also apply to the archived video."]
        pub enable_embed: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableLowLatency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this broadcast has low latency enabled."]
        pub enable_low_latency: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latencyPreference")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted."]
        pub latency_preference:
            ::std::option::Option<LiveBroadcastContentDetailsLatencyPreferenceEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mesh")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The mesh for projecting the video if projection is mesh. The mesh value must be a UTF-8 string containing the base-64 encoding of 3D mesh data that follows the Spherical Video V2 RFC specification for an mshp box, excluding the box size and type but including the following four reserved zero bytes for the version and flags."]
        pub mesh: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monitorStream")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monitorStream object contains information about the monitor stream, which the broadcaster can use to review the event content before the broadcast stream is shown publicly."]
        pub monitor_stream: ::std::option::Option<::std::boxed::Box<MonitorStreamInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The projection format of this broadcast. This defaults to rectangular."]
        pub projection: ::std::option::Option<LiveBroadcastContentDetailsProjectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordFromStart")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Automatically start recording after the event goes live. The default value for this property is true. *Important:* You must also set the enableDvr property's value to true if you want the playback to be available immediately after the broadcast ends. If you set this property's value to true but do not also set the enableDvr property to true, there may be a delay of around one day before the archived video will be available for playback."]
        pub record_from_start: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startWithSlate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This setting indicates whether the broadcast should automatically begin with an in-stream slate when you update the broadcast's status to live. After updating the status, you then need to send a liveCuepoints.insert request that sets the cuepoint's eventState to end to remove the in-stream slate and make your broadcast stream visible to viewers."]
        pub start_with_slate: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stereoLayout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The 3D stereo layout of this broadcast. This defaults to mono."]
        pub stereo_layout: ::std::option::Option<LiveBroadcastContentDetailsStereoLayoutEnum>,
    }
    impl LiveBroadcastContentDetails {
        pub fn builder() -> LiveBroadcastContentDetailsBuilder {
            LiveBroadcastContentDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum LiveBroadcastContentDetailsClosedCaptionsTypeEnum {
        #[serde(rename = "closedCaptionsTypeUnspecified")]
        #[doc = ""]
        ClosedCaptionsTypeUnspecified,
        #[serde(rename = "closedCaptionsDisabled")]
        #[doc = ""]
        ClosedCaptionsDisabled,
        #[serde(rename = "closedCaptionsHttpPost")]
        #[doc = ""]
        ClosedCaptionsHttpPost,
        #[serde(rename = "closedCaptionsEmbedded")]
        #[doc = ""]
        ClosedCaptionsEmbedded,
    }
    impl ::std::default::Default for LiveBroadcastContentDetailsClosedCaptionsTypeEnum {
        fn default() -> Self {
            Self::ClosedCaptionsTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "If both this and enable_low_latency are set, they must match. LATENCY_NORMAL should match enable_low_latency=false LATENCY_LOW should match enable_low_latency=true LATENCY_ULTRA_LOW should have enable_low_latency omitted."]
    pub enum LiveBroadcastContentDetailsLatencyPreferenceEnum {
        #[serde(rename = "latencyPreferenceUnspecified")]
        #[doc = ""]
        LatencyPreferenceUnspecified,
        #[serde(rename = "normal")]
        #[doc = "Best for: highest quality viewer playbacks and higher resolutions."]
        Normal,
        #[serde(rename = "low")]
        #[doc = "Best for: near real-time interaction, with minimal playback buffering."]
        Low,
        #[serde(rename = "ultraLow")]
        #[doc = "Best for: real-time interaction Does not support: Closed captions, 1440p, and 4k resolutions"]
        UltraLow,
    }
    impl ::std::default::Default for LiveBroadcastContentDetailsLatencyPreferenceEnum {
        fn default() -> Self {
            Self::LatencyPreferenceUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The projection format of this broadcast. This defaults to rectangular."]
    pub enum LiveBroadcastContentDetailsProjectionEnum {
        #[serde(rename = "projectionUnspecified")]
        #[doc = ""]
        ProjectionUnspecified,
        #[serde(rename = "rectangular")]
        #[doc = ""]
        Rectangular,
        #[serde(rename = "360")]
        #[doc = ""]
        _360,
        #[serde(rename = "mesh")]
        #[doc = ""]
        Mesh,
    }
    impl ::std::default::Default for LiveBroadcastContentDetailsProjectionEnum {
        fn default() -> Self {
            Self::ProjectionUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The 3D stereo layout of this broadcast. This defaults to mono."]
    pub enum LiveBroadcastContentDetailsStereoLayoutEnum {
        #[serde(rename = "stereoLayoutUnspecified")]
        #[doc = ""]
        StereoLayoutUnspecified,
        #[serde(rename = "mono")]
        #[doc = ""]
        Mono,
        #[serde(rename = "leftRight")]
        #[doc = ""]
        LeftRight,
        #[serde(rename = "topBottom")]
        #[doc = ""]
        TopBottom,
    }
    impl ::std::default::Default for LiveBroadcastContentDetailsStereoLayoutEnum {
        fn default() -> Self {
            Self::StereoLayoutUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveBroadcastListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of broadcasts that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiveBroadcast>>>,
        #[builder(
            default = "{ live_broadcast_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "live_broadcast_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveBroadcastListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveBroadcastListResponse {
        pub fn builder() -> LiveBroadcastListResponseBuilder {
            LiveBroadcastListResponseBuilder::default()
        }
    }
    mod live_broadcast_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveBroadcastListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic broadcast information."]
    pub struct LiveBroadcastSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actualEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the broadcast actually ended. This information is only available once the broadcast's state is complete."]
        pub actual_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actualStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the broadcast actually started. This information is only available once the broadcast's state is live."]
        pub actual_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel that is publishing the broadcast."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The broadcast's description. As with the title, you can set this field by modifying the broadcast resource or by setting the description field of the corresponding video resource."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefaultBroadcast")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this broadcast is the default broadcast. Internal only."]
        pub is_default_broadcast: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveChatId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the live chat for this broadcast."]
        pub live_chat_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the broadcast was added to YouTube's live broadcast schedule."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the broadcast is scheduled to start."]
        pub scheduled_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the broadcast is scheduled to end."]
        pub scheduled_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the broadcast. For each nested object in this object, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The broadcast's title. Note that the broadcast represents exactly one YouTube video. You can set this field by modifying the broadcast resource or by setting the title field of the corresponding video resource."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl LiveBroadcastSnippet {
        pub fn builder() -> LiveBroadcastSnippetBuilder {
            LiveBroadcastSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about the live broadcast. These represent a snapshot of the values at the time of the request. Statistics are only returned for live broadcasts."]
    pub struct LiveBroadcastStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalChatCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of live chat messages currently on the broadcast. The property and its value will be present if the broadcast is public, has the live chat feature enabled, and has at least one message. Note that this field will not be filled after the broadcast ends. So this property would not identify the number of chat messages for an archived video of a completed live broadcast."]
        pub total_chat_count: ::std::option::Option<::std::string::String>,
    }
    impl LiveBroadcastStatistics {
        pub fn builder() -> LiveBroadcastStatisticsBuilder {
            LiveBroadcastStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Live broadcast state."]
    pub struct LiveBroadcastStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lifeCycleStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method."]
        pub life_cycle_status: ::std::option::Option<LiveBroadcastStatusLifeCycleStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveBroadcastPriority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Priority of the live broadcast event (internal state)."]
        pub live_broadcast_priority:
            ::std::option::Option<LiveBroadcastStatusLiveBroadcastPriorityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "madeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the broadcast is made for kids or not, decided by YouTube instead of the creator. This field is read only."]
        pub made_for_kids: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource."]
        pub privacy_status: ::std::option::Option<LiveBroadcastStatusPrivacyStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The broadcast's recording status."]
        pub recording_status: ::std::option::Option<LiveBroadcastStatusRecordingStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfDeclaredMadeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field will be set to True if the creator declares the broadcast to be kids only: go/live-cw-work."]
        pub self_declared_made_for_kids: ::std::option::Option<::std::primitive::bool>,
    }
    impl LiveBroadcastStatus {
        pub fn builder() -> LiveBroadcastStatusBuilder {
            LiveBroadcastStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The broadcast's status. The status can be updated using the API's liveBroadcasts.transition method."]
    pub enum LiveBroadcastStatusLifeCycleStatusEnum {
        #[serde(rename = "lifeCycleStatusUnspecified")]
        #[doc = "No value or the value is unknown."]
        LifeCycleStatusUnspecified,
        #[serde(rename = "created")]
        #[doc = "Incomplete settings, but otherwise valid"]
        Created,
        #[serde(rename = "ready")]
        #[doc = "Complete settings"]
        Ready,
        #[serde(rename = "testing")]
        #[doc = "Visible only to partner, may need special UI treatment"]
        Testing,
        #[serde(rename = "live")]
        #[doc = "Viper is recording; this means the \"clock\" is running"]
        Live,
        #[serde(rename = "complete")]
        #[doc = "The broadcast is finished."]
        Complete,
        #[serde(rename = "revoked")]
        #[doc = "This broadcast was removed by admin action"]
        Revoked,
        #[serde(rename = "testStarting")]
        #[doc = "Transition into TESTING has been requested"]
        TestStarting,
        #[serde(rename = "liveStarting")]
        #[doc = "Transition into LIVE has been requested"]
        LiveStarting,
    }
    impl ::std::default::Default for LiveBroadcastStatusLifeCycleStatusEnum {
        fn default() -> Self {
            Self::LifeCycleStatusUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Priority of the live broadcast event (internal state)."]
    pub enum LiveBroadcastStatusLiveBroadcastPriorityEnum {
        #[serde(rename = "liveBroadcastPriorityUnspecified")]
        #[doc = ""]
        LiveBroadcastPriorityUnspecified,
        #[serde(rename = "low")]
        #[doc = "Low priority broadcast: for low view count HoAs or other low priority broadcasts."]
        Low,
        #[serde(rename = "normal")]
        #[doc = "Normal priority broadcast: for regular HoAs and broadcasts."]
        Normal,
        #[serde(rename = "high")]
        #[doc = "High priority broadcast: for high profile HoAs, like PixelCorp ones."]
        High,
    }
    impl ::std::default::Default for LiveBroadcastStatusLiveBroadcastPriorityEnum {
        fn default() -> Self {
            Self::LiveBroadcastPriorityUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The broadcast's privacy status. Note that the broadcast represents exactly one YouTube video, so the privacy settings are identical to those supported for videos. In addition, you can set this field by modifying the broadcast resource or by setting the privacyStatus field of the corresponding video resource."]
    pub enum LiveBroadcastStatusPrivacyStatusEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "unlisted")]
        #[doc = ""]
        Unlisted,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for LiveBroadcastStatusPrivacyStatusEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The broadcast's recording status."]
    pub enum LiveBroadcastStatusRecordingStatusEnum {
        #[serde(rename = "liveBroadcastRecordingStatusUnspecified")]
        #[doc = "No value or the value is unknown."]
        LiveBroadcastRecordingStatusUnspecified,
        #[serde(rename = "notRecording")]
        #[doc = "The recording has not yet been started."]
        NotRecording,
        #[serde(rename = "recording")]
        #[doc = "The recording is currently on."]
        Recording,
        #[serde(rename = "recorded")]
        #[doc = "The recording is completed, and cannot be started again."]
        Recorded,
    }
    impl ::std::default::Default for LiveBroadcastStatusRecordingStatusEnum {
        fn default() -> Self {
            Self::LiveBroadcastRecordingStatusUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `__liveChatBan__` resource represents a ban for a YouTube live chat."]
    pub struct LiveChatBan {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the ban."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ live_chat_ban_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "live_chat_ban_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string `\"youtube#liveChatBan\"`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `snippet` object contains basic details about the ban."]
        pub snippet: ::std::option::Option<::std::boxed::Box<LiveChatBanSnippet>>,
    }
    impl LiveChatBan {
        pub fn builder() -> LiveChatBanBuilder {
            LiveChatBanBuilder::default()
        }
    }
    mod live_chat_ban_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveChatBan")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatBanSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "banDurationSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration of a ban, only filled if the ban has type TEMPORARY."]
        pub ban_duration_seconds: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannedUserDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub banned_user_details: ::std::option::Option<::std::boxed::Box<ChannelProfileDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveChatId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The chat this ban is pertinent to."]
        pub live_chat_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of ban."]
        pub _type: ::std::option::Option<LiveChatBanSnippetTypeEnum>,
    }
    impl LiveChatBanSnippet {
        pub fn builder() -> LiveChatBanSnippetBuilder {
            LiveChatBanSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of ban."]
    pub enum LiveChatBanSnippetTypeEnum {
        #[serde(rename = "liveChatBanTypeUnspecified")]
        #[doc = "An invalid ban type."]
        LiveChatBanTypeUnspecified,
        #[serde(rename = "permanent")]
        #[doc = "A permanent ban."]
        Permanent,
        #[serde(rename = "temporary")]
        #[doc = "A temporary ban."]
        Temporary,
    }
    impl ::std::default::Default for LiveChatBanSnippetTypeEnum {
        fn default() -> Self {
            Self::LiveChatBanTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatFanFundingEventDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountDisplayString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rendered string that displays the fund amount and currency to the user."]
        pub amount_display_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of the fund."]
        pub amount_micros: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency in which the fund was made."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment added by the user to this fan funding event."]
        pub user_comment: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatFanFundingEventDetails {
        pub fn builder() -> LiveChatFanFundingEventDetailsBuilder {
            LiveChatFanFundingEventDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *liveChatMessage* resource represents a chat message in a YouTube Live Chat."]
    pub struct LiveChatMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The authorDetails object contains basic details about the user that posted this message."]
        pub author_details: ::std::option::Option<::std::boxed::Box<LiveChatMessageAuthorDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the message."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ live_chat_message_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "live_chat_message_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveChatMessage\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the message."]
        pub snippet: ::std::option::Option<::std::boxed::Box<LiveChatMessageSnippet>>,
    }
    impl LiveChatMessage {
        pub fn builder() -> LiveChatMessageBuilder {
            LiveChatMessageBuilder::default()
        }
    }
    mod live_chat_message_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveChatMessage")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatMessageAuthorDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The YouTube channel ID."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel's URL."]
        pub channel_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel's display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isChatModerator")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the author is a moderator of the live chat."]
        pub is_chat_moderator: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isChatOwner")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the author is the owner of the live chat."]
        pub is_chat_owner: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isChatSponsor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the author is a sponsor of the live chat."]
        pub is_chat_sponsor: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isVerified")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the author's identity has been verified by YouTube."]
        pub is_verified: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "profileImageUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channels's avatar URL."]
        pub profile_image_url: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatMessageAuthorDetails {
        pub fn builder() -> LiveChatMessageAuthorDetailsBuilder {
            LiveChatMessageAuthorDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatMessageDeletedDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "deletedMessageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_message_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatMessageDeletedDetails {
        pub fn builder() -> LiveChatMessageDeletedDetailsBuilder {
            LiveChatMessageDeletedDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatMessageListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiveChatMessage>>>,
        #[builder(
            default = "{ live_chat_message_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "live_chat_message_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveChatMessageListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "offlineAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the underlying stream went offline."]
        pub offline_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pollingIntervalMillis")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount of time the client should wait before polling again."]
        pub polling_interval_millis: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatMessageListResponse {
        pub fn builder() -> LiveChatMessageListResponseBuilder {
            LiveChatMessageListResponseBuilder::default()
        }
    }
    mod live_chat_message_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveChatMessageListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatMessageRetractedDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retractedMessageId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub retracted_message_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatMessageRetractedDetails {
        pub fn builder() -> LiveChatMessageRetractedDetailsBuilder {
            LiveChatMessageRetractedDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatMessageSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "authorChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the user that authored this message, this field is not always filled. textMessageEvent - the user that wrote the message fanFundingEvent - the user that funded the broadcast newSponsorEvent - the user that just became a sponsor messageDeletedEvent - the moderator that took the action messageRetractedEvent - the author that retracted their message userBannedEvent - the moderator that took the action superChatEvent - the user that made the purchase"]
        pub author_channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayMessage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains a string that can be displayed to the user. If this field is not present the message is silent, at the moment only messages of type TOMBSTONE and CHAT_ENDED_EVENT are silent."]
        pub display_message: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fanFundingEventDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the funding event, this is only set if the type is 'fanFundingEvent'."]
        pub fan_funding_event_details:
            ::std::option::Option<::std::boxed::Box<LiveChatFanFundingEventDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasDisplayContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the message has display content that should be displayed to users."]
        pub has_display_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveChatId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub live_chat_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageDeletedDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message_deleted_details:
            ::std::option::Option<::std::boxed::Box<LiveChatMessageDeletedDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageRetractedDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub message_retracted_details:
            ::std::option::Option<::std::boxed::Box<LiveChatMessageRetractedDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the message was orignally published."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "superChatDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the Super Chat event, this is only set if the type is 'superChatEvent'."]
        pub super_chat_details: ::std::option::Option<::std::boxed::Box<LiveChatSuperChatDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "superStickerDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the Super Sticker event, this is only set if the type is 'superStickerEvent'."]
        pub super_sticker_details:
            ::std::option::Option<::std::boxed::Box<LiveChatSuperStickerDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textMessageDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the text message, this is only set if the type is 'textMessageEvent'."]
        pub text_message_details:
            ::std::option::Option<::std::boxed::Box<LiveChatTextMessageDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of message, this will always be present, it determines the contents of the message as well as which fields will be present."]
        pub _type: ::std::option::Option<LiveChatMessageSnippetTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userBannedDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub user_banned_details:
            ::std::option::Option<::std::boxed::Box<LiveChatUserBannedMessageDetails>>,
    }
    impl LiveChatMessageSnippet {
        pub fn builder() -> LiveChatMessageSnippetBuilder {
            LiveChatMessageSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of message, this will always be present, it determines the contents of the message as well as which fields will be present."]
    pub enum LiveChatMessageSnippetTypeEnum {
        #[serde(rename = "invalidType")]
        #[doc = ""]
        InvalidType,
        #[serde(rename = "textMessageEvent")]
        #[doc = ""]
        TextMessageEvent,
        #[serde(rename = "tombstone")]
        #[doc = ""]
        Tombstone,
        #[serde(rename = "fanFundingEvent")]
        #[doc = ""]
        FanFundingEvent,
        #[serde(rename = "chatEndedEvent")]
        #[doc = ""]
        ChatEndedEvent,
        #[serde(rename = "sponsorOnlyModeStartedEvent")]
        #[doc = ""]
        SponsorOnlyModeStartedEvent,
        #[serde(rename = "sponsorOnlyModeEndedEvent")]
        #[doc = ""]
        SponsorOnlyModeEndedEvent,
        #[serde(rename = "newSponsorEvent")]
        #[doc = ""]
        NewSponsorEvent,
        #[serde(rename = "messageDeletedEvent")]
        #[doc = ""]
        MessageDeletedEvent,
        #[serde(rename = "messageRetractedEvent")]
        #[doc = ""]
        MessageRetractedEvent,
        #[serde(rename = "userBannedEvent")]
        #[doc = ""]
        UserBannedEvent,
        #[serde(rename = "superChatEvent")]
        #[doc = ""]
        SuperChatEvent,
        #[serde(rename = "superStickerEvent")]
        #[doc = ""]
        SuperStickerEvent,
    }
    impl ::std::default::Default for LiveChatMessageSnippetTypeEnum {
        fn default() -> Self {
            Self::InvalidType
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *liveChatModerator* resource represents a moderator for a YouTube live chat. A chat moderator has the ability to ban/unban users from a chat, remove message, etc."]
    pub struct LiveChatModerator {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the moderator."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ live_chat_moderator_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "live_chat_moderator_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveChatModerator\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the moderator."]
        pub snippet: ::std::option::Option<::std::boxed::Box<LiveChatModeratorSnippet>>,
    }
    impl LiveChatModerator {
        pub fn builder() -> LiveChatModeratorBuilder {
            LiveChatModeratorBuilder::default()
        }
    }
    mod live_chat_moderator_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveChatModerator")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatModeratorListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of moderators that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiveChatModerator>>>,
        #[builder(
            default = "{ live_chat_moderator_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "live_chat_moderator_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveChatModeratorListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatModeratorListResponse {
        pub fn builder() -> LiveChatModeratorListResponseBuilder {
            LiveChatModeratorListResponseBuilder::default()
        }
    }
    mod live_chat_moderator_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveChatModeratorListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatModeratorSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveChatId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the live chat this moderator can act on."]
        pub live_chat_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "moderatorDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the moderator."]
        pub moderator_details: ::std::option::Option<::std::boxed::Box<ChannelProfileDetails>>,
    }
    impl LiveChatModeratorSnippet {
        pub fn builder() -> LiveChatModeratorSnippetBuilder {
            LiveChatModeratorSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatSuperChatDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountDisplayString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rendered string that displays the fund amount and currency to the user."]
        pub amount_display_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount purchased by the user, in micros (1,750,000 micros = 1.75)."]
        pub amount_micros: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency in which the purchase was made."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tier in which the amount belongs. Lower amounts belong to lower tiers. The lowest tier is 1."]
        pub tier: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userComment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The comment added by the user to this Super Chat event."]
        pub user_comment: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatSuperChatDetails {
        pub fn builder() -> LiveChatSuperChatDetailsBuilder {
            LiveChatSuperChatDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatSuperStickerDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountDisplayString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rendered string that displays the fund amount and currency to the user."]
        pub amount_display_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount purchased by the user, in micros (1,750,000 micros = 1.75)."]
        pub amount_micros: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency in which the purchase was made."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "superStickerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the Super Sticker."]
        pub super_sticker_metadata: ::std::option::Option<::std::boxed::Box<SuperStickerMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tier")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tier in which the amount belongs. Lower amounts belong to lower tiers. The lowest tier is 1."]
        pub tier: ::std::option::Option<::std::primitive::i64>,
    }
    impl LiveChatSuperStickerDetails {
        pub fn builder() -> LiveChatSuperStickerDetailsBuilder {
            LiveChatSuperStickerDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatTextMessageDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user's message."]
        pub message_text: ::std::option::Option<::std::string::String>,
    }
    impl LiveChatTextMessageDetails {
        pub fn builder() -> LiveChatTextMessageDetailsBuilder {
            LiveChatTextMessageDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveChatUserBannedMessageDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "banDurationSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The duration of the ban. This property is only present if the banType is temporary."]
        pub ban_duration_seconds: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "banType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of ban."]
        pub ban_type: ::std::option::Option<LiveChatUserBannedMessageDetailsBanTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bannedUserDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The details of the user that was banned."]
        pub banned_user_details: ::std::option::Option<::std::boxed::Box<ChannelProfileDetails>>,
    }
    impl LiveChatUserBannedMessageDetails {
        pub fn builder() -> LiveChatUserBannedMessageDetailsBuilder {
            LiveChatUserBannedMessageDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of ban."]
    pub enum LiveChatUserBannedMessageDetailsBanTypeEnum {
        #[serde(rename = "permanent")]
        #[doc = ""]
        Permanent,
        #[serde(rename = "temporary")]
        #[doc = ""]
        Temporary,
    }
    impl ::std::default::Default for LiveChatUserBannedMessageDetailsBanTypeEnum {
        fn default() -> Self {
            Self::Permanent
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A live stream describes a live ingestion point."]
    pub struct LiveStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cdn")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube."]
        pub cdn: ::std::option::Option<::std::boxed::Box<CdnSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The content_details object contains information about the stream, including the closed captions ingestion URL."]
        pub content_details: ::std::option::Option<::std::boxed::Box<LiveStreamContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the stream."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ live_stream_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "live_stream_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveStream\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the stream, including its channel, title, and description."]
        pub snippet: ::std::option::Option<::std::boxed::Box<LiveStreamSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains information about live stream's status."]
        pub status: ::std::option::Option<::std::boxed::Box<LiveStreamStatus>>,
    }
    impl LiveStream {
        pub fn builder() -> LiveStreamBuilder {
            LiveStreamBuilder::default()
        }
    }
    mod live_stream_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveStream")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveStreamConfigurationIssue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The long-form description of the issue and how to resolve it."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The short-form reason for this issue."]
        pub reason: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "How severe this issue is to the stream."]
        pub severity: ::std::option::Option<LiveStreamConfigurationIssueSeverityEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The kind of error happening."]
        pub _type: ::std::option::Option<LiveStreamConfigurationIssueTypeEnum>,
    }
    impl LiveStreamConfigurationIssue {
        pub fn builder() -> LiveStreamConfigurationIssueBuilder {
            LiveStreamConfigurationIssueBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "How severe this issue is to the stream."]
    pub enum LiveStreamConfigurationIssueSeverityEnum {
        #[serde(rename = "info")]
        #[doc = ""]
        Info,
        #[serde(rename = "warning")]
        #[doc = ""]
        Warning,
        #[serde(rename = "error")]
        #[doc = ""]
        Error,
    }
    impl ::std::default::Default for LiveStreamConfigurationIssueSeverityEnum {
        fn default() -> Self {
            Self::Info
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The kind of error happening."]
    pub enum LiveStreamConfigurationIssueTypeEnum {
        #[serde(rename = "gopSizeOver")]
        #[doc = ""]
        GopSizeOver,
        #[serde(rename = "gopSizeLong")]
        #[doc = ""]
        GopSizeLong,
        #[serde(rename = "gopSizeShort")]
        #[doc = ""]
        GopSizeShort,
        #[serde(rename = "openGop")]
        #[doc = ""]
        OpenGop,
        #[serde(rename = "badContainer")]
        #[doc = ""]
        BadContainer,
        #[serde(rename = "audioBitrateHigh")]
        #[doc = ""]
        AudioBitrateHigh,
        #[serde(rename = "audioBitrateLow")]
        #[doc = ""]
        AudioBitrateLow,
        #[serde(rename = "audioSampleRate")]
        #[doc = ""]
        AudioSampleRate,
        #[serde(rename = "bitrateHigh")]
        #[doc = ""]
        BitrateHigh,
        #[serde(rename = "bitrateLow")]
        #[doc = ""]
        BitrateLow,
        #[serde(rename = "audioCodec")]
        #[doc = ""]
        AudioCodec,
        #[serde(rename = "videoCodec")]
        #[doc = ""]
        VideoCodec,
        #[serde(rename = "noAudioStream")]
        #[doc = ""]
        NoAudioStream,
        #[serde(rename = "noVideoStream")]
        #[doc = ""]
        NoVideoStream,
        #[serde(rename = "multipleVideoStreams")]
        #[doc = ""]
        MultipleVideoStreams,
        #[serde(rename = "multipleAudioStreams")]
        #[doc = ""]
        MultipleAudioStreams,
        #[serde(rename = "audioTooManyChannels")]
        #[doc = ""]
        AudioTooManyChannels,
        #[serde(rename = "interlacedVideo")]
        #[doc = ""]
        InterlacedVideo,
        #[serde(rename = "frameRateHigh")]
        #[doc = ""]
        FrameRateHigh,
        #[serde(rename = "resolutionMismatch")]
        #[doc = ""]
        ResolutionMismatch,
        #[serde(rename = "videoCodecMismatch")]
        #[doc = ""]
        VideoCodecMismatch,
        #[serde(rename = "videoInterlaceMismatch")]
        #[doc = ""]
        VideoInterlaceMismatch,
        #[serde(rename = "videoProfileMismatch")]
        #[doc = ""]
        VideoProfileMismatch,
        #[serde(rename = "videoBitrateMismatch")]
        #[doc = ""]
        VideoBitrateMismatch,
        #[serde(rename = "framerateMismatch")]
        #[doc = ""]
        FramerateMismatch,
        #[serde(rename = "gopMismatch")]
        #[doc = ""]
        GopMismatch,
        #[serde(rename = "audioSampleRateMismatch")]
        #[doc = ""]
        AudioSampleRateMismatch,
        #[serde(rename = "audioStereoMismatch")]
        #[doc = ""]
        AudioStereoMismatch,
        #[serde(rename = "audioCodecMismatch")]
        #[doc = ""]
        AudioCodecMismatch,
        #[serde(rename = "audioBitrateMismatch")]
        #[doc = ""]
        AudioBitrateMismatch,
        #[serde(rename = "videoResolutionSuboptimal")]
        #[doc = ""]
        VideoResolutionSuboptimal,
        #[serde(rename = "videoResolutionUnsupported")]
        #[doc = ""]
        VideoResolutionUnsupported,
        #[serde(rename = "videoIngestionStarved")]
        #[doc = ""]
        VideoIngestionStarved,
        #[serde(rename = "videoIngestionFasterThanRealtime")]
        #[doc = ""]
        VideoIngestionFasterThanRealtime,
    }
    impl ::std::default::Default for LiveStreamConfigurationIssueTypeEnum {
        fn default() -> Self {
            Self::GopSizeOver
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Detailed settings of a stream."]
    pub struct LiveStreamContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "closedCaptionsIngestionUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ingestion URL where the closed captions of this stream are sent."]
        pub closed_captions_ingestion_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isReusable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the stream is reusable, which means that it can be bound to multiple broadcasts. It is common for broadcasters to reuse the same stream for many different broadcasts if those broadcasts occur at different times. If you set this value to false, then the stream will not be reusable, which means that it can only be bound to one broadcast. Non-reusable streams differ from reusable streams in the following ways: - A non-reusable stream can only be bound to one broadcast. - A non-reusable stream might be deleted by an automated process after the broadcast ends. - The liveStreams.list method does not list non-reusable streams if you call the method and set the mine parameter to true. The only way to use that method to retrieve the resource for a non-reusable stream is to use the id parameter to identify the stream. "]
        pub is_reusable: ::std::option::Option<::std::primitive::bool>,
    }
    impl LiveStreamContentDetails {
        pub fn builder() -> LiveStreamContentDetailsBuilder {
            LiveStreamContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveStreamHealthStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "configurationIssues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The configurations issues on this stream"]
        pub configuration_issues:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiveStreamConfigurationIssue>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTimeSeconds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time this status was updated (in seconds)"]
        pub last_update_time_seconds: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status code of this stream"]
        pub status: ::std::option::Option<LiveStreamHealthStatusStatusEnum>,
    }
    impl LiveStreamHealthStatus {
        pub fn builder() -> LiveStreamHealthStatusBuilder {
            LiveStreamHealthStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status code of this stream"]
    pub enum LiveStreamHealthStatusStatusEnum {
        #[serde(rename = "good")]
        #[doc = ""]
        Good,
        #[serde(rename = "ok")]
        #[doc = ""]
        Ok,
        #[serde(rename = "bad")]
        #[doc = ""]
        Bad,
        #[serde(rename = "noData")]
        #[doc = ""]
        NoData,
        #[serde(rename = "revoked")]
        #[doc = ""]
        Revoked,
    }
    impl ::std::default::Default for LiveStreamHealthStatusStatusEnum {
        fn default() -> Self {
            Self::Good
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveStreamListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of live streams that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LiveStream>>>,
        #[builder(
            default = "{ live_stream_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "live_stream_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#liveStreamListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl LiveStreamListResponse {
        pub fn builder() -> LiveStreamListResponseBuilder {
            LiveStreamListResponseBuilder::default()
        }
    }
    mod live_stream_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#liveStreamListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LiveStreamSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel that is transmitting the stream."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream's description. The value cannot be longer than 10000 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isDefaultStream")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub is_default_stream: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the stream was created."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The stream's title. The value must be between 1 and 128 characters long."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl LiveStreamSnippet {
        pub fn builder() -> LiveStreamSnippetBuilder {
            LiveStreamSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Brief description of the live stream status."]
    pub struct LiveStreamStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "healthStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The health status of the stream."]
        pub health_status: ::std::option::Option<::std::boxed::Box<LiveStreamHealthStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "streamStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub stream_status: ::std::option::Option<LiveStreamStatusStreamStatusEnum>,
    }
    impl LiveStreamStatus {
        pub fn builder() -> LiveStreamStatusBuilder {
            LiveStreamStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum LiveStreamStatusStreamStatusEnum {
        #[serde(rename = "created")]
        #[doc = ""]
        Created,
        #[serde(rename = "ready")]
        #[doc = ""]
        Ready,
        #[serde(rename = "active")]
        #[doc = ""]
        Active,
        #[serde(rename = "inactive")]
        #[doc = ""]
        Inactive,
        #[serde(rename = "error")]
        #[doc = ""]
        Error,
    }
    impl ::std::default::Default for LiveStreamStatusStreamStatusEnum {
        fn default() -> Self {
            Self::Created
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LocalizedProperty {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "default")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub _default: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the default property."]
        pub default_language: ::std::option::Option<::std::boxed::Box<LanguageTag>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub localized: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<LocalizedString>>>,
    }
    impl LocalizedProperty {
        pub fn builder() -> LocalizedPropertyBuilder {
            LocalizedPropertyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct LocalizedString {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl LocalizedString {
        pub fn builder() -> LocalizedStringBuilder {
            LocalizedStringBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *member* resource represents a member for a YouTube channel. A member provides recurring monetary support to a creator and receives special benefits."]
    pub struct Member {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ member_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "member_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#member\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the member."]
        pub snippet: ::std::option::Option<::std::boxed::Box<MemberSnippet>>,
    }
    impl Member {
        pub fn builder() -> MemberBuilder {
            MemberBuilder::default()
        }
    }
    mod member_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#member")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MemberListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of members that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Member>>>,
        #[builder(default = "{ member_list_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "member_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#memberListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl MemberListResponse {
        pub fn builder() -> MemberListResponseBuilder {
            MemberListResponseBuilder::default()
        }
    }
    mod member_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#memberListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MemberSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the channel that's offering memberships."]
        pub creator_channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the member."]
        pub member_details: ::std::option::Option<::std::boxed::Box<ChannelProfileDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipsDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the user's membership."]
        pub memberships_details: ::std::option::Option<::std::boxed::Box<MembershipsDetails>>,
    }
    impl MemberSnippet {
        pub fn builder() -> MemberSnippetBuilder {
            MemberSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MembershipsDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessibleLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ids of all levels that the user has access to. This includes the currently active level and all other levels that are included because of a higher purchase."]
        pub accessible_levels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highestAccessibleLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Id of the highest level that the user has access to at the moment."]
        pub highest_accessible_level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "highestAccessibleLevelDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name for the highest level that the user has access to at the moment."]
        pub highest_accessible_level_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipsDuration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about memberships duration without taking into consideration pricing levels."]
        pub memberships_duration: ::std::option::Option<::std::boxed::Box<MembershipsDuration>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "membershipsDurationAtLevels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Data about memberships duration on particular pricing levels."]
        pub memberships_duration_at_levels:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipsDurationAtLevel>>>,
    }
    impl MembershipsDetails {
        pub fn builder() -> MembershipsDetailsBuilder {
            MembershipsDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MembershipsDuration {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberSince")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the user became a continuous member across all levels."]
        pub member_since: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberTotalDurationMonths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cumulative time the user has been a member across all levels in complete months (the time is rounded down to the nearest integer)."]
        pub member_total_duration_months: ::std::option::Option<::std::primitive::i64>,
    }
    impl MembershipsDuration {
        pub fn builder() -> MembershipsDurationBuilder {
            MembershipsDurationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MembershipsDurationAtLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "level")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pricing level ID."]
        pub level: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberSince")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the user became a continuous member for the given level."]
        pub member_since: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "memberTotalDurationMonths")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The cumulative time the user has been a member for the given level in complete months (the time is rounded down to the nearest integer)."]
        pub member_total_duration_months: ::std::option::Option<::std::primitive::i64>,
    }
    impl MembershipsDurationAtLevel {
        pub fn builder() -> MembershipsDurationAtLevelBuilder {
            MembershipsDurationAtLevelBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *membershipsLevel* resource represents an offer made by YouTube creators for their fans. Users can become members of the channel by joining one of the available levels. They will provide recurring monetary support and receives special benefits."]
    pub struct MembershipsLevel {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the memberships level."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ memberships_level_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "memberships_level_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#membershipsLevelListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the level."]
        pub snippet: ::std::option::Option<::std::boxed::Box<MembershipsLevelSnippet>>,
    }
    impl MembershipsLevel {
        pub fn builder() -> MembershipsLevelBuilder {
            MembershipsLevelBuilder::default()
        }
    }
    mod memberships_level_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#membershipsLevel")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MembershipsLevelListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of pricing levels offered by a creator to the fans."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<MembershipsLevel>>>,
        #[builder(
            default = "{ memberships_level_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "memberships_level_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#membershipsLevelListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl MembershipsLevelListResponse {
        pub fn builder() -> MembershipsLevelListResponseBuilder {
            MembershipsLevelListResponseBuilder::default()
        }
    }
    mod memberships_level_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#membershipsLevelListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct MembershipsLevelSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creatorChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id of the channel that's offering channel memberships."]
        pub creator_channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "levelDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the pricing level."]
        pub level_details: ::std::option::Option<::std::boxed::Box<LevelDetails>>,
    }
    impl MembershipsLevelSnippet {
        pub fn builder() -> MembershipsLevelSnippetBuilder {
            MembershipsLevelSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings and Info of the monitor stream"]
    pub struct MonitorStreamInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "broadcastStreamDelayMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If you have set the enableMonitorStream property to true, then this property determines the length of the live broadcast delay."]
        pub broadcast_stream_delay_ms: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedHtml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "HTML code that embeds a player that plays the monitor stream."]
        pub embed_html: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableMonitorStream")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value determines whether the monitor stream is enabled for the broadcast. If the monitor stream is enabled, then YouTube will broadcast the event content on a special stream intended only for the broadcaster's consumption. The broadcaster can use the stream to review the event content and also to identify the optimal times to insert cuepoints. You need to set this value to true if you intend to have a broadcast delay for your event. *Note:* This property cannot be updated once the broadcast is in the testing or live state."]
        pub enable_monitor_stream: ::std::option::Option<::std::primitive::bool>,
    }
    impl MonitorStreamInfo {
        pub fn builder() -> MonitorStreamInfoBuilder {
            MonitorStreamInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Paging details for lists of resources, including total number of items available and number of resources returned in a single page."]
    pub struct PageInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resultsPerPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of results included in the API response."]
        pub results_per_page: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The total number of results in the result set."]
        pub total_results: ::std::option::Option<::std::primitive::i64>,
    }
    impl PageInfo {
        pub fn builder() -> PageInfoBuilder {
            PageInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *playlist* resource represents a YouTube playlist. A playlist is a collection of videos that can be viewed sequentially and shared with other users. A playlist can contain up to 200 videos, and YouTube does not limit the number of playlists that each user creates. By default, playlists are publicly visible to other users, but playlists can be public or private. YouTube also uses playlists to identify special collections of videos for a channel, such as: - uploaded videos - favorite videos - positively rated (liked) videos - watch history - watch later To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company's videos, playlists, and other YouTube information. You can retrieve the playlist IDs for each of these lists from the channel resource for a given channel. You can then use the playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the playlistItems.insert and playlistItems.delete methods."]
    pub struct Playlist {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains information like video count."]
        pub content_details: ::std::option::Option<::std::boxed::Box<PlaylistContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the playlist."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ playlist_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "playlist_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#playlist\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localizations for different languages"]
        pub localizations: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<PlaylistLocalization>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player object contains information that you would use to play the playlist in an embedded player."]
        pub player: ::std::option::Option<::std::boxed::Box<PlaylistPlayer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the playlist, such as its title and description."]
        pub snippet: ::std::option::Option<::std::boxed::Box<PlaylistSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains status information for the playlist."]
        pub status: ::std::option::Option<::std::boxed::Box<PlaylistStatus>>,
    }
    impl Playlist {
        pub fn builder() -> PlaylistBuilder {
            PlaylistBuilder::default()
        }
    }
    mod playlist_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#playlist")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "itemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of videos in the playlist."]
        pub item_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl PlaylistContentDetails {
        pub fn builder() -> PlaylistContentDetailsBuilder {
            PlaylistContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *playlistItem* resource identifies another resource, such as a video, that is included in a playlist. In addition, the playlistItem resource contains details about the included resource that pertain specifically to how that resource is used in that playlist. YouTube uses playlists to identify special collections of videos for a channel, such as: - uploaded videos - favorite videos - positively rated (liked) videos - watch history - watch later To be more specific, these lists are associated with a channel, which is a collection of a person, group, or company's videos, playlists, and other YouTube information. You can retrieve the playlist IDs for each of these lists from the channel resource for a given channel. You can then use the playlistItems.list method to retrieve any of those lists. You can also add or remove items from those lists by calling the playlistItems.insert and playlistItems.delete methods. For example, if a user gives a positive rating to a video, you would insert that video into the liked videos playlist for that user's channel."]
    pub struct PlaylistItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object is included in the resource if the included item is a YouTube video. The object contains additional information about the video."]
        pub content_details: ::std::option::Option<::std::boxed::Box<PlaylistItemContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the playlist item."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ playlist_item_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "playlist_item_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#playlistItem\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the playlist item, such as its title and position in the playlist."]
        pub snippet: ::std::option::Option<::std::boxed::Box<PlaylistItemSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains information about the playlist item's privacy status."]
        pub status: ::std::option::Option<::std::boxed::Box<PlaylistItemStatus>>,
    }
    impl PlaylistItem {
        pub fn builder() -> PlaylistItemBuilder {
            PlaylistItemBuilder::default()
        }
    }
    mod playlist_item_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#playlistItem")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistItemContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video."]
        pub end_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "note")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A user-generated note for this item."]
        pub note: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0."]
        pub start_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify a video. To retrieve the video resource, set the id query parameter to this value in your API request."]
        pub video_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoPublishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the video was published to YouTube."]
        pub video_published_at: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistItemContentDetails {
        pub fn builder() -> PlaylistItemContentDetailsBuilder {
            PlaylistItemContentDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistItemListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of playlist items that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<PlaylistItem>>>,
        #[builder(
            default = "{ playlist_item_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "playlist_item_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#playlistItemListResponse\". Etag of this resource."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistItemListResponse {
        pub fn builder() -> PlaylistItemListResponseBuilder {
            PlaylistItemListResponseBuilder::default()
        }
    }
    mod playlist_item_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#playlistItemListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a playlist, including title, description and thumbnails. Basic details of a YouTube Playlist item provided by the author. Next ID: 15"]
    pub struct PlaylistItemSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the user that added the item to the playlist."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel title for the channel that the playlist item belongs to."]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlistId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify thGe playlist that the playlist item is in."]
        pub playlist_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "position")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth."]
        pub position: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the item was added to the playlist."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id object contains information that can be used to uniquely identify the resource that is included in the playlist as the playlist item."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the playlist item. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The item's title."]
        pub title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoOwnerChannelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel id for the channel this video belongs to."]
        pub video_owner_channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoOwnerChannelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel title for the channel this video belongs to."]
        pub video_owner_channel_title: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistItemSnippet {
        pub fn builder() -> PlaylistItemSnippetBuilder {
            PlaylistItemSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about the playlist item's privacy status."]
    pub struct PlaylistItemStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This resource's privacy status."]
        pub privacy_status: ::std::option::Option<PlaylistItemStatusPrivacyStatusEnum>,
    }
    impl PlaylistItemStatus {
        pub fn builder() -> PlaylistItemStatusBuilder {
            PlaylistItemStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This resource's privacy status."]
    pub enum PlaylistItemStatusPrivacyStatusEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "unlisted")]
        #[doc = ""]
        Unlisted,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for PlaylistItemStatusPrivacyStatusEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of playlists that match the request criteria"]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Playlist>>>,
        #[builder(
            default = "{ playlist_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "playlist_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#playlistListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistListResponse {
        pub fn builder() -> PlaylistListResponseBuilder {
            PlaylistListResponseBuilder::default()
        }
    }
    mod playlist_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#playlistListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Playlist localization setting"]
    pub struct PlaylistLocalization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized strings for playlist's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized strings for playlist's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistLocalization {
        pub fn builder() -> PlaylistLocalizationBuilder {
            PlaylistLocalizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistPlayer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedHtml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An <iframe> tag that embeds a player that will play the playlist."]
        pub embed_html: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistPlayer {
        pub fn builder() -> PlaylistPlayerBuilder {
            PlaylistPlayerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a playlist, including title, description and thumbnails."]
    pub struct PlaylistSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel that published the playlist."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel title of the channel that the video belongs to."]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the playlist's default title and description."]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The playlist's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized title and description, read-only."]
        pub localized: ::std::option::Option<::std::boxed::Box<PlaylistLocalization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the playlist was created."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Keyword tags associated with the playlist."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailVideoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Note: if the playlist has a custom thumbnail, this field will not be populated. The video id selected by the user that will be used as the thumbnail of this playlist. This field defaults to the first publicly viewable video in the playlist, if: 1. The user has never selected a video to be the thumbnail of the playlist. 2. The user selects a video to be the thumbnail, and then removes that video from the playlist. 3. The user selects a non-owned video to be the thumbnail, but that video becomes private, or gets deleted."]
        pub thumbnail_video_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the playlist. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The playlist's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl PlaylistSnippet {
        pub fn builder() -> PlaylistSnippetBuilder {
            PlaylistSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct PlaylistStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The playlist's privacy status."]
        pub privacy_status: ::std::option::Option<PlaylistStatusPrivacyStatusEnum>,
    }
    impl PlaylistStatus {
        pub fn builder() -> PlaylistStatusBuilder {
            PlaylistStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The playlist's privacy status."]
    pub enum PlaylistStatusPrivacyStatusEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "unlisted")]
        #[doc = ""]
        Unlisted,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for PlaylistStatusPrivacyStatusEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A pair Property / Value."]
    pub struct PropertyValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "property")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A property."]
        pub property: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The property's value."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl PropertyValue {
        pub fn builder() -> PropertyValueBuilder {
            PropertyValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct RelatedEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub entity: ::std::option::Option<::std::boxed::Box<Entity>>,
    }
    impl RelatedEntity {
        pub fn builder() -> RelatedEntityBuilder {
            RelatedEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A resource id is a generic reference that points to another YouTube resource."]
    pub struct ResourceId {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the API resource."]
        pub kind: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playlistId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist."]
        pub playlist_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl ResourceId {
        pub fn builder() -> ResourceIdBuilder {
            ResourceIdBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SearchListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Pagination information for token pagination."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SearchResult>>>,
        #[builder(default = "{ search_list_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "search_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#searchListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub region_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl SearchListResponse {
        pub fn builder() -> SearchListResponseBuilder {
            SearchListResponseBuilder::default()
        }
    }
    mod search_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#searchListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A search result contains information about a YouTube video, channel, or playlist that matches the search parameters specified in an API request. While a search result points to a uniquely identifiable resource, like a video, it does not have its own persistent data."]
    pub struct SearchResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id object contains information that can be used to uniquely identify the resource that matches the search request."]
        pub id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
        #[builder(default = "{ search_result_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "search_result_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#searchResult\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about a search result, such as its title or description. For example, if the search result is a video, then the title will be the video's title and the description will be the video's description."]
        pub snippet: ::std::option::Option<::std::boxed::Box<SearchResultSnippet>>,
    }
    impl SearchResult {
        pub fn builder() -> SearchResultBuilder {
            SearchResultBuilder::default()
        }
    }
    mod search_result_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#searchResult")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a search result, including title, description and thumbnails of the item referenced by the search result."]
    pub struct SearchResultSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value that YouTube uses to uniquely identify the channel that published the resource that the search result identifies."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the channel that published the resource that the search result identifies."]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the search result."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveBroadcastContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's \"none\" if there is not any upcoming/active live broadcasts."]
        pub live_broadcast_content:
            ::std::option::Option<SearchResultSnippetLiveBroadcastContentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation date and time of the resource that the search result identifies."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the search result. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the search result."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SearchResultSnippet {
        pub fn builder() -> SearchResultSnippetBuilder {
            SearchResultSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "It indicates if the resource (video or channel) has upcoming/active live broadcast content. Or it's \"none\" if there is not any upcoming/active live broadcasts."]
    pub enum SearchResultSnippetLiveBroadcastContentEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "upcoming")]
        #[doc = "The live broadcast is upcoming."]
        Upcoming,
        #[serde(rename = "live")]
        #[doc = "The live broadcast is active."]
        Live,
        #[serde(rename = "completed")]
        #[doc = "The live broadcast has been completed."]
        Completed,
    }
    impl ::std::default::Default for SearchResultSnippetLiveBroadcastContentEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *subscription* resource contains information about a YouTube user subscription. A subscription notifies a user when new videos are added to a channel or when another user takes one of several actions on YouTube, such as uploading a video, rating a video, or commenting on a video."]
    pub struct Subscription {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains basic statistics about the subscription."]
        pub content_details: ::std::option::Option<::std::boxed::Box<SubscriptionContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the subscription."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ subscription_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "subscription_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#subscription\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the subscription, including its title and the channel that the user subscribed to."]
        pub snippet: ::std::option::Option<::std::boxed::Box<SubscriptionSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subscriberSnippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscriberSnippet object contains basic details about the subscriber."]
        pub subscriber_snippet:
            ::std::option::Option<::std::boxed::Box<SubscriptionSubscriberSnippet>>,
    }
    impl Subscription {
        pub fn builder() -> SubscriptionBuilder {
            SubscriptionBuilder::default()
        }
    }
    mod subscription_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#subscription")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the content to witch a subscription refers."]
    pub struct SubscriptionContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of activity this subscription is for (only uploads, everything)."]
        pub activity_type: ::std::option::Option<SubscriptionContentDetailsActivityTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newItemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of new items in the subscription since its content was last read."]
        pub new_item_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "totalItemCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The approximate number of items that the subscription points to."]
        pub total_item_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl SubscriptionContentDetails {
        pub fn builder() -> SubscriptionContentDetailsBuilder {
            SubscriptionContentDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of activity this subscription is for (only uploads, everything)."]
    pub enum SubscriptionContentDetailsActivityTypeEnum {
        #[serde(rename = "subscriptionActivityTypeUnspecified")]
        #[doc = ""]
        SubscriptionActivityTypeUnspecified,
        #[serde(rename = "all")]
        #[doc = ""]
        All,
        #[serde(rename = "uploads")]
        #[doc = ""]
        Uploads,
    }
    impl ::std::default::Default for SubscriptionContentDetailsActivityTypeEnum {
        fn default() -> Self {
            Self::SubscriptionActivityTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SubscriptionListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of subscriptions that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Subscription>>>,
        #[builder(
            default = "{ subscription_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "subscription_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#subscriptionListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionListResponse {
        pub fn builder() -> SubscriptionListResponseBuilder {
            SubscriptionListResponseBuilder::default()
        }
    }
    mod subscription_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#subscriptionListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a subscription, including title, description and thumbnails of the subscribed item."]
    pub struct SubscriptionSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the subscriber's channel."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel title for the channel that the subscription belongs to."]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription's details."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time that the subscription was created."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The id object contains information about the channel that the user subscribed to."]
        pub resource_id: ::std::option::Option<::std::boxed::Box<ResourceId>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The subscription's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionSnippet {
        pub fn builder() -> SubscriptionSnippetBuilder {
            SubscriptionSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a subscription's subscriber including title, description, channel ID and thumbnails."]
    pub struct SubscriptionSubscriberSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The channel ID of the subscriber."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the subscriber."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Thumbnails for this subscriber."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The title of the subscriber."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl SubscriptionSubscriberSnippet {
        pub fn builder() -> SubscriptionSubscriberSnippetBuilder {
            SubscriptionSubscriberSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `__superChatEvent__` resource represents a Super Chat purchase on a YouTube channel."]
    pub struct SuperChatEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube assigns to uniquely identify the Super Chat event."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ super_chat_event_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "super_chat_event_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string `\"youtube#superChatEvent\"`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `snippet` object contains basic details about the Super Chat event."]
        pub snippet: ::std::option::Option<::std::boxed::Box<SuperChatEventSnippet>>,
    }
    impl SuperChatEvent {
        pub fn builder() -> SuperChatEventBuilder {
            SuperChatEventBuilder::default()
        }
    }
    mod super_chat_event_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#superChatEvent")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SuperChatEventListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Super Chat purchases that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<SuperChatEvent>>>,
        #[builder(
            default = "{ super_chat_event_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "super_chat_event_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#superChatEventListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl SuperChatEventListResponse {
        pub fn builder() -> SuperChatEventListResponseBuilder {
            SuperChatEventListResponseBuilder::default()
        }
    }
    mod super_chat_event_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#superChatEventListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SuperChatEventSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "amountMicros")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The purchase amount, in micros of the purchase currency. e.g., 1 is represented as 1000000."]
        pub amount_micros: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel id where the event occurred."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text contents of the comment left by the user."]
        pub comment_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createdAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the event occurred."]
        pub created_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currency")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The currency in which the purchase was made. ISO 4217."]
        pub currency: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayString")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A rendered string that displays the purchase amount and currency (e.g., \"$1.00\"). The string is rendered for the given language."]
        pub display_string: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isSuperStickerEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "True if this event is a Super Sticker event."]
        pub is_super_sticker_event: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tier for the paid message, which is based on the amount of money spent to purchase the message."]
        pub message_type: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "superStickerMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If this event is a Super Sticker event, this field will contain metadata about the Super Sticker."]
        pub super_sticker_metadata: ::std::option::Option<::std::boxed::Box<SuperStickerMetadata>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "supporterDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Details about the supporter."]
        pub supporter_details: ::std::option::Option<::std::boxed::Box<ChannelProfileDetails>>,
    }
    impl SuperChatEventSnippet {
        pub fn builder() -> SuperChatEventSnippetBuilder {
            SuperChatEventSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct SuperStickerMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Internationalized alt text that describes the sticker image and any animation associated with it."]
        pub alt_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "altTextLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the localization language in which the alt text is returned."]
        pub alt_text_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stickerId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unique identifier of the Super Sticker. This is a shorter form of the alt_text that includes pack name and a recognizable characteristic of the sticker."]
        pub sticker_id: ::std::option::Option<::std::string::String>,
    }
    impl SuperStickerMetadata {
        pub fn builder() -> SuperStickerMetadataBuilder {
            SuperStickerMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gaia")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub gaia: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub snippet: ::std::option::Option<::std::boxed::Box<TestItemTestItemSnippet>>,
    }
    impl TestItem {
        pub fn builder() -> TestItemBuilder {
            TestItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct TestItemTestItemSnippet {}
    impl TestItemTestItemSnippet {
        pub fn builder() -> TestItemTestItemSnippetBuilder {
            TestItemTestItemSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *third party account link* resource represents a link between a YouTube account or a channel and an account on a third-party service."]
    pub struct ThirdPartyLink {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource"]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ third_party_link_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "third_party_link_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#thirdPartyLink\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkingToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The linking_token identifies a YouTube account and channel with which the third party account is linked."]
        pub linking_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the third- party account link."]
        pub snippet: ::std::option::Option<::std::boxed::Box<ThirdPartyLinkSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains information about the status of the link."]
        pub status: ::std::option::Option<::std::boxed::Box<ThirdPartyLinkStatus>>,
    }
    impl ThirdPartyLink {
        pub fn builder() -> ThirdPartyLinkBuilder {
            ThirdPartyLinkBuilder::default()
        }
    }
    mod third_party_link_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#thirdPartyLink")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic information about a third party account link, including its type and type-specific information."]
    pub struct ThirdPartyLinkSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelToStoreLink")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information specific to a link between a channel and a store on a merchandising platform."]
        pub channel_to_store_link:
            ::std::option::Option<::std::boxed::Box<ChannelToStoreLinkDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the link named after the entities that are being linked."]
        pub _type: ::std::option::Option<ThirdPartyLinkSnippetTypeEnum>,
    }
    impl ThirdPartyLinkSnippet {
        pub fn builder() -> ThirdPartyLinkSnippetBuilder {
            ThirdPartyLinkSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the link named after the entities that are being linked."]
    pub enum ThirdPartyLinkSnippetTypeEnum {
        #[serde(rename = "linkUnspecified")]
        #[doc = ""]
        LinkUnspecified,
        #[serde(rename = "channelToStoreLink")]
        #[doc = "A link that is connecting (or about to connect) a channel with a store on a merchandising platform in order to enable retail commerce capabilities for that channel on YouTube."]
        ChannelToStoreLink,
    }
    impl ::std::default::Default for ThirdPartyLinkSnippetTypeEnum {
        fn default() -> Self {
            Self::LinkUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The third-party link status object contains information about the status of the link."]
    pub struct ThirdPartyLinkStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub link_status: ::std::option::Option<ThirdPartyLinkStatusLinkStatusEnum>,
    }
    impl ThirdPartyLinkStatus {
        pub fn builder() -> ThirdPartyLinkStatusBuilder {
            ThirdPartyLinkStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum ThirdPartyLinkStatusLinkStatusEnum {
        #[serde(rename = "unknown")]
        #[doc = ""]
        Unknown,
        #[serde(rename = "failed")]
        #[doc = ""]
        Failed,
        #[serde(rename = "pending")]
        #[doc = ""]
        Pending,
        #[serde(rename = "linked")]
        #[doc = ""]
        Linked,
    }
    impl ::std::default::Default for ThirdPartyLinkStatusLinkStatusEnum {
        fn default() -> Self {
            Self::Unknown
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A thumbnail is an image representing a YouTube resource."]
    pub struct Thumbnail {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Optional) Height of the thumbnail image."]
        pub height: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "url")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The thumbnail image's URL."]
        pub url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "width")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "(Optional) Width of the thumbnail image."]
        pub width: ::std::option::Option<::std::primitive::i64>,
    }
    impl Thumbnail {
        pub fn builder() -> ThumbnailBuilder {
            ThumbnailBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Internal representation of thumbnails for a YouTube resource."]
    pub struct ThumbnailDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "default")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default image for this resource."]
        pub _default: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "high")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The high quality image for this resource."]
        pub high: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "maxres")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The maximum resolution quality image for this resource."]
        pub maxres: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "medium")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The medium quality image for this resource."]
        pub medium: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "standard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard quality image for this resource."]
        pub standard: ::std::option::Option<::std::boxed::Box<Thumbnail>>,
    }
    impl ThumbnailDetails {
        pub fn builder() -> ThumbnailDetailsBuilder {
            ThumbnailDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct ThumbnailSetResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of thumbnails."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<ThumbnailDetails>>>,
        #[builder(
            default = "{ thumbnail_set_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "thumbnail_set_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#thumbnailSetResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl ThumbnailSetResponse {
        pub fn builder() -> ThumbnailSetResponseBuilder {
            ThumbnailSetResponseBuilder::default()
        }
    }
    mod thumbnail_set_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#thumbnailSetResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Stub token pagination template to suppress results."]
    pub struct TokenPagination {}
    impl TokenPagination {
        pub fn builder() -> TokenPaginationBuilder {
            TokenPaginationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *video* resource represents a YouTube video."]
    pub struct Video {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ageGating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Age restriction details related to a video. This data can only be retrieved by the video owner."]
        pub age_gating: ::std::option::Option<::std::boxed::Box<VideoAgeGating>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The contentDetails object contains information about the video content, including the length of the video and its aspect ratio."]
        pub content_details: ::std::option::Option<::std::boxed::Box<VideoContentDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner."]
        pub file_details: ::std::option::Option<::std::boxed::Box<VideoFileDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the video."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ video_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "video_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#video\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveStreamingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast."]
        pub live_streaming_details:
            ::std::option::Option<::std::boxed::Box<VideoLiveStreamingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localizations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localizations object contains localized versions of the basic details about the video, such as its title and description."]
        pub localizations: ::std::option::Option<
            ::std::collections::BTreeMap<String, ::std::boxed::Box<VideoLocalization>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "monetizationDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The monetizationDetails object encapsulates information about the monetization status of the video."]
        pub monetization_details:
            ::std::option::Option<::std::boxed::Box<VideoMonetizationDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "player")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The player object contains information that you would use to play the video in an embedded player."]
        pub player: ::std::option::Option<::std::boxed::Box<VideoPlayer>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processingDetails object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video. The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner."]
        pub processing_details: ::std::option::Option<::std::boxed::Box<VideoProcessingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projectDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The projectDetails object contains information about the project specific video metadata. b/157517979: This part was never populated after it was added. However, it sees non-zero traffic because there is generated client code in the wild that refers to it [1]. We keep this field and do NOT remove it because otherwise V3 would return an error when this part gets requested [2]. [1] https://developers.google.com/resources/api-libraries/documentation/youtube/v3/csharp/latest/classGoogle_1_1Apis_1_1YouTube_1_1v3_1_1Data_1_1VideoProjectDetails.html [2] http://google3/video/youtube/src/python/servers/data_api/common.py?l=1565-1569&rcl=344141677"]
        pub project_details: ::std::option::Option<::std::boxed::Box<VideoProjectDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordingDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The recordingDetails object encapsulates information about the location, date and address where the video was recorded."]
        pub recording_details: ::std::option::Option<::std::boxed::Box<VideoRecordingDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the video, such as its title, description, and category."]
        pub snippet: ::std::option::Option<::std::boxed::Box<VideoSnippet>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "statistics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The statistics object contains statistics about the video."]
        pub statistics: ::std::option::Option<::std::boxed::Box<VideoStatistics>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status object contains information about the video's uploading, processing, and privacy statuses."]
        pub status: ::std::option::Option<::std::boxed::Box<VideoStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner."]
        pub suggestions: ::std::option::Option<::std::boxed::Box<VideoSuggestions>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicDetails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The topicDetails object encapsulates information about Freebase topics associated with the video."]
        pub topic_details: ::std::option::Option<::std::boxed::Box<VideoTopicDetails>>,
    }
    impl Video {
        pub fn builder() -> VideoBuilder {
            VideoBuilder::default()
        }
    }
    mod video_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#video")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoAbuseReport {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "comments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional comments regarding the abuse report."]
        pub comments: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "language")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language that the content was viewed in."]
        pub language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reasonId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The high-level, or primary, reason that the content is abusive. The value is an abuse report reason ID."]
        pub reason_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryReasonId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The specific, or secondary, reason that this content is abusive (if available). The value is an abuse report reason ID that is a valid secondary reason for the primary reason."]
        pub secondary_reason_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the video."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoAbuseReport {
        pub fn builder() -> VideoAbuseReportBuilder {
            VideoAbuseReportBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A `__videoAbuseReportReason__` resource identifies a reason that a video could be reported as abusive. Video abuse report reasons are used with `video.ReportAbuse`."]
    pub struct VideoAbuseReportReason {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this abuse report reason."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(
            default = "{ video_abuse_report_reason_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "video_abuse_report_reason_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string `\"youtube#videoAbuseReportReason\"`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `snippet` object contains basic details about the abuse report reason."]
        pub snippet: ::std::option::Option<::std::boxed::Box<VideoAbuseReportReasonSnippet>>,
    }
    impl VideoAbuseReportReason {
        pub fn builder() -> VideoAbuseReportReasonBuilder {
            VideoAbuseReportReasonBuilder::default()
        }
    }
    mod video_abuse_report_reason_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoAbuseReportReason")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoAbuseReportReasonListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of valid abuse reasons that are used with `video.ReportAbuse`."]
        pub items:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoAbuseReportReason>>>,
        #[builder(
            default = "{ video_abuse_report_reason_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "video_abuse_report_reason_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string `\"youtube#videoAbuseReportReasonListResponse\"`."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The `visitorId` identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoAbuseReportReasonListResponse {
        pub fn builder() -> VideoAbuseReportReasonListResponseBuilder {
            VideoAbuseReportReasonListResponseBuilder::default()
        }
    }
    mod video_abuse_report_reason_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoAbuseReportReasonListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a video category, such as its localized title."]
    pub struct VideoAbuseReportReasonSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized label belonging to this abuse report reason."]
        pub label: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "secondaryReasons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The secondary reasons associated with this reason, if any are available. (There might be 0 or more.)"]
        pub secondary_reasons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<VideoAbuseReportSecondaryReason>>,
        >,
    }
    impl VideoAbuseReportReasonSnippet {
        pub fn builder() -> VideoAbuseReportReasonSnippetBuilder {
            VideoAbuseReportReasonSnippetBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoAbuseReportSecondaryReason {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of this abuse report secondary reason."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "label")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The localized label for this abuse report secondary reason."]
        pub label: ::std::option::Option<::std::string::String>,
    }
    impl VideoAbuseReportSecondaryReason {
        pub fn builder() -> VideoAbuseReportSecondaryReasonBuilder {
            VideoAbuseReportSecondaryReasonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoAgeGating {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alcoholContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content."]
        pub alcohol_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restricted")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present."]
        pub restricted: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoGameRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Video game rating, if any."]
        pub video_game_rating: ::std::option::Option<VideoAgeGatingVideoGameRatingEnum>,
    }
    impl VideoAgeGating {
        pub fn builder() -> VideoAgeGatingBuilder {
            VideoAgeGatingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Video game rating, if any."]
    pub enum VideoAgeGatingVideoGameRatingEnum {
        #[serde(rename = "anyone")]
        #[doc = ""]
        Anyone,
        #[serde(rename = "m15Plus")]
        #[doc = ""]
        M15Plus,
        #[serde(rename = "m16Plus")]
        #[doc = ""]
        M16Plus,
        #[serde(rename = "m17Plus")]
        #[doc = ""]
        M17Plus,
    }
    impl ::std::default::Default for VideoAgeGatingVideoGameRatingEnum {
        fn default() -> Self {
            Self::Anyone
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A *videoCategory* resource identifies a category that has been or could be associated with uploaded videos."]
    pub struct VideoCategory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the video category."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ video_category_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "video_category_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#videoCategory\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "snippet")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The snippet object contains basic details about the video category, including its title."]
        pub snippet: ::std::option::Option<::std::boxed::Box<VideoCategorySnippet>>,
    }
    impl VideoCategory {
        pub fn builder() -> VideoCategoryBuilder {
            VideoCategoryBuilder::default()
        }
    }
    mod video_category_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoCategory")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoCategoryListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoCategory>>>,
        #[builder(
            default = "{ video_category_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "video_category_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#videoCategoryListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoCategoryListResponse {
        pub fn builder() -> VideoCategoryListResponseBuilder {
            VideoCategoryListResponseBuilder::default()
        }
    }
    mod video_category_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoCategoryListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a video category, such as its localized title."]
    pub struct VideoCategorySnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "assignable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub assignable: ::std::option::Option<::std::primitive::bool>,
        #[builder(
            default = "{ video_category_snippet_defaults :: channel_id () }",
            setter(into)
        )]
        #[serde(rename = "channelId")]
        #[serde(default = "video_category_snippet_defaults :: channel_id")]
        #[doc = "The YouTube channel that created the video category."]
        pub channel_id: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video category's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl VideoCategorySnippet {
        pub fn builder() -> VideoCategorySnippetBuilder {
            VideoCategorySnippetBuilder::default()
        }
    }
    mod video_category_snippet_defaults {
        pub fn channel_id() -> ::std::string::String {
            String::from("UCBR8-60-B28hp2BmDPdntcQ")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the content of a YouTube Video."]
    pub struct VideoContentDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of captions indicates whether the video has captions or not."]
        pub caption: ::std::option::Option<VideoContentDetailsCaptionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentRating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the ratings that the video received under various rating schemes."]
        pub content_rating: ::std::option::Option<::std::boxed::Box<ContentRating>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countryRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The countryRestriction object contains information about the countries where a video is (or is not) viewable."]
        pub country_restriction: ::std::option::Option<::std::boxed::Box<AccessPolicy>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of definition indicates whether the video is available in high definition or only in standard definition."]
        pub definition: ::std::option::Option<VideoContentDetailsDefinitionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dimension")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of dimension indicates whether the video is available in 3D or in 2D."]
        pub dimension: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "duration")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long."]
        pub duration: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "hasCustomThumbnail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the video uploader has provided a custom thumbnail image for the video. This property is only visible to the video uploader."]
        pub has_custom_thumbnail: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "licensedContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of is_license_content indicates whether the video is licensed content."]
        pub licensed_content: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "projection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the projection format of the video."]
        pub projection: ::std::option::Option<VideoContentDetailsProjectionEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "regionRestriction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The regionRestriction object contains information about the countries where a video is (or is not) viewable. The object will contain either the contentDetails.regionRestriction.allowed property or the contentDetails.regionRestriction.blocked property."]
        pub region_restriction:
            ::std::option::Option<::std::boxed::Box<VideoContentDetailsRegionRestriction>>,
    }
    impl VideoContentDetails {
        pub fn builder() -> VideoContentDetailsBuilder {
            VideoContentDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The value of captions indicates whether the video has captions or not."]
    pub enum VideoContentDetailsCaptionEnum {
        #[serde(rename = "true")]
        #[doc = ""]
        True,
        #[serde(rename = "false")]
        #[doc = ""]
        False,
    }
    impl ::std::default::Default for VideoContentDetailsCaptionEnum {
        fn default() -> Self {
            Self::True
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The value of definition indicates whether the video is available in high definition or only in standard definition."]
    pub enum VideoContentDetailsDefinitionEnum {
        #[serde(rename = "sd")]
        #[doc = "sd"]
        Sd,
        #[serde(rename = "hd")]
        #[doc = "hd"]
        Hd,
    }
    impl ::std::default::Default for VideoContentDetailsDefinitionEnum {
        fn default() -> Self {
            Self::Sd
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Specifies the projection format of the video."]
    pub enum VideoContentDetailsProjectionEnum {
        #[serde(rename = "rectangular")]
        #[doc = ""]
        Rectangular,
        #[serde(rename = "360")]
        #[doc = ""]
        _360,
    }
    impl ::std::default::Default for VideoContentDetailsProjectionEnum {
        fn default() -> Self {
            Self::Rectangular
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED Region restriction of the video."]
    pub struct VideoContentDetailsRegionRestriction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries."]
        pub allowed: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "blocked")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries."]
        pub blocked: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl VideoContentDetailsRegionRestriction {
        pub fn builder() -> VideoContentDetailsRegionRestrictionBuilder {
            VideoContentDetailsRegionRestrictionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information."]
    pub struct VideoFileDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioStreams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream."]
        pub audio_streams:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoFileDetailsAudioStream>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bitrateBps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uploaded video file's combined (video and audio) bitrate in bits per second."]
        pub bitrate_bps: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "container")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uploaded video file's container format."]
        pub container: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported: - Date only: YYYY-MM-DD - Naive time: YYYY-MM-DDTHH:MM:SS - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM "]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "durationMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The length of the uploaded video in milliseconds."]
        pub duration_ms: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uploaded file's name. This field is present whether a video file or another type of file was uploaded."]
        pub file_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded."]
        pub file_size: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded."]
        pub file_type: ::std::option::Option<VideoFileDetailsFileTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoStreams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream."]
        pub video_streams:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoFileDetailsVideoStream>>>,
    }
    impl VideoFileDetails {
        pub fn builder() -> VideoFileDetailsBuilder {
            VideoFileDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded."]
    pub enum VideoFileDetailsFileTypeEnum {
        #[serde(rename = "video")]
        #[doc = "Known video file (e.g., an MP4 file)."]
        Video,
        #[serde(rename = "audio")]
        #[doc = "Audio only file (e.g., an MP3 file)."]
        Audio,
        #[serde(rename = "image")]
        #[doc = "Image file (e.g., a JPEG image)."]
        Image,
        #[serde(rename = "archive")]
        #[doc = "Archive file (e.g., a ZIP archive)."]
        Archive,
        #[serde(rename = "document")]
        #[doc = "Document or text file (e.g., MS Word document)."]
        Document,
        #[serde(rename = "project")]
        #[doc = "Movie project file (e.g., Microsoft Windows Movie Maker project)."]
        Project,
        #[serde(rename = "other")]
        #[doc = "Other non-video file type."]
        Other,
    }
    impl ::std::default::Default for VideoFileDetailsFileTypeEnum {
        fn default() -> Self {
            Self::Video
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about an audio stream."]
    pub struct VideoFileDetailsAudioStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bitrateBps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio stream's bitrate, in bits per second."]
        pub bitrate_bps: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of audio channels that the stream contains."]
        pub channel_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio codec that the stream uses."]
        pub codec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vendor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code."]
        pub vendor: ::std::option::Option<::std::string::String>,
    }
    impl VideoFileDetailsAudioStream {
        pub fn builder() -> VideoFileDetailsAudioStreamBuilder {
            VideoFileDetailsAudioStreamBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Information about a video stream."]
    pub struct VideoFileDetailsVideoStream {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "aspectRatio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video content's display aspect ratio, which specifies the aspect ratio in which the video should be displayed."]
        pub aspect_ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bitrateBps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video stream's bitrate, in bits per second."]
        pub bitrate_bps: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "codec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video codec that the stream uses."]
        pub codec: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "frameRateFps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video stream's frame rate, in frames per second."]
        pub frame_rate_fps: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "heightPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded video content's height in pixels."]
        pub height_pixels: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The amount that YouTube needs to rotate the original source content to properly display the video."]
        pub rotation: ::std::option::Option<VideoFileDetailsVideoStreamRotationEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "vendor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A value that uniquely identifies a video vendor. Typically, the value is a four-letter vendor code."]
        pub vendor: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "widthPixels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The encoded video content's width in pixels. You can calculate the video's encoding aspect ratio as width_pixels / height_pixels."]
        pub width_pixels: ::std::option::Option<::std::primitive::i64>,
    }
    impl VideoFileDetailsVideoStream {
        pub fn builder() -> VideoFileDetailsVideoStreamBuilder {
            VideoFileDetailsVideoStreamBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The amount that YouTube needs to rotate the original source content to properly display the video."]
    pub enum VideoFileDetailsVideoStreamRotationEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "clockwise")]
        #[doc = ""]
        Clockwise,
        #[serde(rename = "upsideDown")]
        #[doc = ""]
        UpsideDown,
        #[serde(rename = "counterClockwise")]
        #[doc = ""]
        CounterClockwise,
        #[serde(rename = "other")]
        #[doc = ""]
        Other,
    }
    impl ::std::default::Default for VideoFileDetailsVideoStreamRotationEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Video>>>,
        #[builder(default = "{ video_list_response_defaults :: kind () }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(default = "video_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#videoListResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "General pagination information."]
        pub page_info: ::std::option::Option<::std::boxed::Box<PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prevPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set."]
        pub prev_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tokenPagination")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub token_pagination: ::std::option::Option<::std::boxed::Box<TokenPagination>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoListResponse {
        pub fn builder() -> VideoListResponseBuilder {
            VideoListResponseBuilder::default()
        }
    }
    mod video_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoListResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about the live streaming metadata."]
    pub struct VideoLiveStreamingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activeLiveChatId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video but rather to the new video being displayed at the persistent page."]
        pub active_live_chat_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actualEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the broadcast actually ended. This value will not be available until the broadcast is over."]
        pub actual_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "actualStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the broadcast actually started. This value will not be available until the broadcast begins."]
        pub actual_start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "concurrentViewers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of viewers currently watching the broadcast. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended."]
        pub concurrent_viewers: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledEndTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the broadcast is scheduled to end. If the value is empty or the property is not present, then the broadcast is scheduled to contiue indefinitely."]
        pub scheduled_end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scheduledStartTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the broadcast is scheduled to begin."]
        pub scheduled_start_time: ::std::option::Option<::std::string::String>,
    }
    impl VideoLiveStreamingDetails {
        pub fn builder() -> VideoLiveStreamingDetailsBuilder {
            VideoLiveStreamingDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Localized versions of certain video properties (e.g. title)."]
    pub struct VideoLocalization {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized version of the video's description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized version of the video's title."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl VideoLocalization {
        pub fn builder() -> VideoLocalizationBuilder {
            VideoLocalizationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Details about monetization of a YouTube Video."]
    pub struct VideoMonetizationDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "access")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The value of access indicates whether the video can be monetized or not."]
        pub access: ::std::option::Option<::std::boxed::Box<AccessPolicy>>,
    }
    impl VideoMonetizationDetails {
        pub fn builder() -> VideoMonetizationDetailsBuilder {
            VideoMonetizationDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Player to be used for a video playback."]
    pub struct VideoPlayer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedHeight")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub embed_height: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedHtml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An <iframe> tag that embeds a player that will play the video."]
        pub embed_html: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embedWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The embed width"]
        pub embed_width: ::std::option::Option<::std::string::String>,
    }
    impl VideoPlayer {
        pub fn builder() -> VideoPlayerBuilder {
            VideoPlayerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes processing status and progress and availability of some other Video resource parts."]
    pub struct VideoProcessingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editorSuggestionsAvailability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request."]
        pub editor_suggestions_availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileDetailsAvailability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request."]
        pub file_details_availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingFailureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed."]
        pub processing_failure_reason:
            ::std::option::Option<VideoProcessingDetailsProcessingFailureReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingIssuesAvailability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request."]
        pub processing_issues_availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingProgress")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The processingProgress object contains information about the progress YouTube has made in processing the video. The values are really only relevant if the video's processing status is processing."]
        pub processing_progress:
            ::std::option::Option<::std::boxed::Box<VideoProcessingDetailsProcessingProgress>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed."]
        pub processing_status: ::std::option::Option<VideoProcessingDetailsProcessingStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagSuggestionsAvailability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request."]
        pub tag_suggestions_availability: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailsAvailability")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates whether thumbnail images have been generated for the video."]
        pub thumbnails_availability: ::std::option::Option<::std::string::String>,
    }
    impl VideoProcessingDetails {
        pub fn builder() -> VideoProcessingDetailsBuilder {
            VideoProcessingDetailsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed."]
    pub enum VideoProcessingDetailsProcessingFailureReasonEnum {
        #[serde(rename = "uploadFailed")]
        #[doc = ""]
        UploadFailed,
        #[serde(rename = "transcodeFailed")]
        #[doc = ""]
        TranscodeFailed,
        #[serde(rename = "streamingFailed")]
        #[doc = ""]
        StreamingFailed,
        #[serde(rename = "other")]
        #[doc = ""]
        Other,
    }
    impl ::std::default::Default for VideoProcessingDetailsProcessingFailureReasonEnum {
        fn default() -> Self {
            Self::UploadFailed
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed."]
    pub enum VideoProcessingDetailsProcessingStatusEnum {
        #[serde(rename = "processing")]
        #[doc = ""]
        Processing,
        #[serde(rename = "succeeded")]
        #[doc = ""]
        Succeeded,
        #[serde(rename = "failed")]
        #[doc = ""]
        Failed,
        #[serde(rename = "terminated")]
        #[doc = ""]
        Terminated,
    }
    impl ::std::default::Default for VideoProcessingDetailsProcessingStatusEnum {
        fn default() -> Self {
            Self::Processing
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Video processing progress and completion time estimate."]
    pub struct VideoProcessingDetailsProcessingProgress {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partsProcessed")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating: 100 * parts_processed / parts_total Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video."]
        pub parts_processed: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "partsTotal")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video."]
        pub parts_total: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeLeftMs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video."]
        pub time_left_ms: ::std::option::Option<::std::string::String>,
    }
    impl VideoProcessingDetailsProcessingProgress {
        pub fn builder() -> VideoProcessingDetailsProcessingProgressBuilder {
            VideoProcessingDetailsProcessingProgressBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "DEPRECATED. b/157517979: This part was never populated after it was added. However, it sees non-zero traffic because there is generated client code in the wild that refers to it [1]. We keep this field and do NOT remove it because otherwise V3 would return an error when this part gets requested [2]. [1] https://developers.google.com/resources/api-libraries/documentation/youtube/v3/csharp/latest/classGoogle_1_1Apis_1_1YouTube_1_1v3_1_1Data_1_1VideoProjectDetails.html [2] http://google3/video/youtube/src/python/servers/data_api/common.py?l=1565-1569&rcl=344141677"]
    pub struct VideoProjectDetails {}
    impl VideoProjectDetails {
        pub fn builder() -> VideoProjectDetailsBuilder {
            VideoProjectDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about rating of a video."]
    pub struct VideoRating {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rating")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rating of a video."]
        pub rating: ::std::option::Option<VideoRatingRatingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "videoId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the video."]
        pub video_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoRating {
        pub fn builder() -> VideoRatingBuilder {
            VideoRatingBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Rating of a video."]
    pub enum VideoRatingRatingEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "like")]
        #[doc = "The entity is liked."]
        Like,
        #[serde(rename = "dislike")]
        #[doc = "The entity is disliked."]
        Dislike,
    }
    impl ::std::default::Default for VideoRatingRatingEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct VideoRatingListResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Etag of this resource."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Serialized EventId of the request which produced this response."]
        pub event_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of ratings that match the request criteria."]
        pub items: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<VideoRating>>>,
        #[builder(
            default = "{ video_rating_list_response_defaults :: kind () }",
            setter(into)
        )]
        #[serde(rename = "kind")]
        #[serde(default = "video_rating_list_response_defaults :: kind")]
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"youtube#videoGetRatingResponse\"."]
        pub kind: ::std::string::String,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "visitorId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The visitorId identifies the visitor."]
        pub visitor_id: ::std::option::Option<::std::string::String>,
    }
    impl VideoRatingListResponse {
        pub fn builder() -> VideoRatingListResponseBuilder {
            VideoRatingListResponseBuilder::default()
        }
    }
    mod video_rating_list_response_defaults {
        pub fn kind() -> ::std::string::String {
            String::from("youtube#videoGetRatingResponse")
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Recording information associated with the video."]
    pub struct VideoRecordingDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geolocation information associated with the video."]
        pub location: ::std::option::Option<::std::boxed::Box<GeoPoint>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "locationDescription")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text description of the location where the video was recorded."]
        pub location_description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "recordingDate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the video was recorded."]
        pub recording_date: ::std::option::Option<::std::string::String>,
    }
    impl VideoRecordingDetails {
        pub fn builder() -> VideoRecordingDetailsBuilder {
            VideoRecordingDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a video, including title, description, uploader, thumbnails and category."]
    pub struct VideoSnippet {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The YouTube video category associated with the video."]
        pub category_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The ID that YouTube uses to uniquely identify the channel that the video was uploaded to."]
        pub channel_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "channelTitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Channel title for the channel that the video belongs to."]
        pub channel_title: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultAudioLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default_audio_language property specifies the language spoken in the video's default audio track."]
        pub default_audio_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language of the videos's default snippet."]
        pub default_language: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's description. @mutable youtube.videos.insert youtube.videos.update"]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveBroadcastContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if the video is an upcoming/active live broadcast. Or it's \"none\" if the video is not an upcoming/active live broadcast."]
        pub live_broadcast_content: ::std::option::Option<VideoSnippetLiveBroadcastContentEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "localized")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Localized snippet selected with the hl parameter. If no such localization exists, this field is populated with the default snippet. (Read-only)"]
        pub localized: ::std::option::Option<::std::boxed::Box<VideoLocalization>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishedAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the video was uploaded."]
        pub published_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of keyword tags associated with the video. Tags may contain spaces."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnails")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A map of thumbnail images associated with the video. For each object in the map, the key is the name of the thumbnail image, and the value is an object that contains other information about the thumbnail."]
        pub thumbnails: ::std::option::Option<::std::boxed::Box<ThumbnailDetails>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's title. @mutable youtube.videos.insert youtube.videos.update"]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl VideoSnippet {
        pub fn builder() -> VideoSnippetBuilder {
            VideoSnippetBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates if the video is an upcoming/active live broadcast. Or it's \"none\" if the video is not an upcoming/active live broadcast."]
    pub enum VideoSnippetLiveBroadcastContentEnum {
        #[serde(rename = "none")]
        #[doc = ""]
        None,
        #[serde(rename = "upcoming")]
        #[doc = "The live broadcast is upcoming."]
        Upcoming,
        #[serde(rename = "live")]
        #[doc = "The live broadcast is active."]
        Live,
        #[serde(rename = "completed")]
        #[doc = "The live broadcast has been completed."]
        Completed,
    }
    impl ::std::default::Default for VideoSnippetLiveBroadcastContentEnum {
        fn default() -> Self {
            Self::None
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Statistics about the video, such as the number of times the video was viewed or liked."]
    pub struct VideoStatistics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "commentCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of comments for the video."]
        pub comment_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dislikeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of users who have indicated that they disliked the video by giving it a negative rating."]
        pub dislike_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "favoriteCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of users who currently have the video marked as a favorite video."]
        pub favorite_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "likeCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of users who have indicated that they liked the video by giving it a positive rating."]
        pub like_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The number of times the video has been viewed."]
        pub view_count: ::std::option::Option<::std::string::String>,
    }
    impl VideoStatistics {
        pub fn builder() -> VideoStatisticsBuilder {
            VideoStatisticsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Basic details about a video category, such as its localized title. Next Id: 17"]
    pub struct VideoStatus {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "embeddable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates if the video can be embedded on another website. @mutable youtube.videos.insert youtube.videos.update"]
        pub embeddable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "failureReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed."]
        pub failure_reason: ::std::option::Option<VideoStatusFailureReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "license")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's license. @mutable youtube.videos.insert youtube.videos.update"]
        pub license: ::std::option::Option<VideoStatusLicenseEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "madeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub made_for_kids: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "privacyStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The video's privacy status."]
        pub privacy_status: ::std::option::Option<VideoStatusPrivacyStatusEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publicStatsViewable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled. @mutable youtube.videos.insert youtube.videos.update"]
        pub public_stats_viewable: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "publishAt")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private.."]
        pub publish_at: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rejectionReason")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected."]
        pub rejection_reason: ::std::option::Option<VideoStatusRejectionReasonEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "selfDeclaredMadeForKids")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub self_declared_made_for_kids: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uploadStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status of the uploaded video."]
        pub upload_status: ::std::option::Option<VideoStatusUploadStatusEnum>,
    }
    impl VideoStatus {
        pub fn builder() -> VideoStatusBuilder {
            VideoStatusBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This value explains why a video failed to upload. This property is only present if the uploadStatus property indicates that the upload failed."]
    pub enum VideoStatusFailureReasonEnum {
        #[serde(rename = "conversion")]
        #[doc = "Unable to convert video content."]
        Conversion,
        #[serde(rename = "invalidFile")]
        #[doc = "Invalid file format."]
        InvalidFile,
        #[serde(rename = "emptyFile")]
        #[doc = "Empty file."]
        EmptyFile,
        #[serde(rename = "tooSmall")]
        #[doc = "File was too small."]
        TooSmall,
        #[serde(rename = "codec")]
        #[doc = "Unsupported codec."]
        Codec,
        #[serde(rename = "uploadAborted")]
        #[doc = "Upload wasn't finished."]
        UploadAborted,
    }
    impl ::std::default::Default for VideoStatusFailureReasonEnum {
        fn default() -> Self {
            Self::Conversion
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's license. @mutable youtube.videos.insert youtube.videos.update"]
    pub enum VideoStatusLicenseEnum {
        #[serde(rename = "youtube")]
        #[doc = ""]
        Youtube,
        #[serde(rename = "creativeCommon")]
        #[doc = ""]
        CreativeCommon,
    }
    impl ::std::default::Default for VideoStatusLicenseEnum {
        fn default() -> Self {
            Self::Youtube
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The video's privacy status."]
    pub enum VideoStatusPrivacyStatusEnum {
        #[serde(rename = "public")]
        #[doc = ""]
        Public,
        #[serde(rename = "unlisted")]
        #[doc = ""]
        Unlisted,
        #[serde(rename = "private")]
        #[doc = ""]
        Private,
    }
    impl ::std::default::Default for VideoStatusPrivacyStatusEnum {
        fn default() -> Self {
            Self::Public
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "This value explains why YouTube rejected an uploaded video. This property is only present if the uploadStatus property indicates that the upload was rejected."]
    pub enum VideoStatusRejectionReasonEnum {
        #[serde(rename = "copyright")]
        #[doc = "Copyright infringement."]
        Copyright,
        #[serde(rename = "inappropriate")]
        #[doc = "Inappropriate video content."]
        Inappropriate,
        #[serde(rename = "duplicate")]
        #[doc = "Duplicate upload in the same channel."]
        Duplicate,
        #[serde(rename = "termsOfUse")]
        #[doc = "Terms of use violation."]
        TermsOfUse,
        #[serde(rename = "uploaderAccountSuspended")]
        #[doc = "Uploader account was suspended."]
        UploaderAccountSuspended,
        #[serde(rename = "length")]
        #[doc = "Video duration was too long."]
        Length,
        #[serde(rename = "claim")]
        #[doc = "Blocked by content owner."]
        Claim,
        #[serde(rename = "uploaderAccountClosed")]
        #[doc = "Uploader closed his/her account."]
        UploaderAccountClosed,
        #[serde(rename = "trademark")]
        #[doc = "Trademark infringement."]
        Trademark,
        #[serde(rename = "legal")]
        #[doc = "An unspecified legal reason."]
        Legal,
    }
    impl ::std::default::Default for VideoStatusRejectionReasonEnum {
        fn default() -> Self {
            Self::Copyright
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The status of the uploaded video."]
    pub enum VideoStatusUploadStatusEnum {
        #[serde(rename = "uploaded")]
        #[doc = "Video has been uploaded but not processed yet."]
        Uploaded,
        #[serde(rename = "processed")]
        #[doc = "Video has been successfully processed."]
        Processed,
        #[serde(rename = "failed")]
        #[doc = "Processing has failed. See FailureReason."]
        Failed,
        #[serde(rename = "rejected")]
        #[doc = "Video has been rejected. See RejectionReason."]
        Rejected,
        #[serde(rename = "deleted")]
        #[doc = "Video has been deleted."]
        Deleted,
    }
    impl ::std::default::Default for VideoStatusUploadStatusEnum {
        fn default() -> Self {
            Self::Uploaded
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions."]
    pub struct VideoSuggestions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "editorSuggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of video editing operations that might improve the video quality or playback experience of the uploaded video."]
        pub editor_suggestions:
            ::std::option::Option<::std::vec::Vec<VideoSuggestionsEditorSuggestionsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingErrors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed."]
        pub processing_errors:
            ::std::option::Option<::std::vec::Vec<VideoSuggestionsProcessingErrorsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of suggestions that may improve YouTube's ability to process the video."]
        pub processing_hints:
            ::std::option::Option<::std::vec::Vec<VideoSuggestionsProcessingHintsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "processingWarnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track."]
        pub processing_warnings:
            ::std::option::Option<::std::vec::Vec<VideoSuggestionsProcessingWarningsEnum>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tagSuggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube."]
        pub tag_suggestions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<VideoSuggestionsTagSuggestion>>,
        >,
    }
    impl VideoSuggestions {
        pub fn builder() -> VideoSuggestionsBuilder {
            VideoSuggestionsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoSuggestionsEditorSuggestionsEnum {
        #[serde(rename = "videoAutoLevels")]
        #[doc = "Picture brightness levels seem off and could be corrected."]
        VideoAutoLevels,
        #[serde(rename = "videoStabilize")]
        #[doc = "The video appears shaky and could be stabilized."]
        VideoStabilize,
        #[serde(rename = "videoCrop")]
        #[doc = "Margins (mattes) detected around the picture could be cropped."]
        VideoCrop,
        #[serde(rename = "audioQuietAudioSwap")]
        #[doc = "The audio track appears silent and could be swapped with a better quality one."]
        AudioQuietAudioSwap,
    }
    impl ::std::default::Default for VideoSuggestionsEditorSuggestionsEnum {
        fn default() -> Self {
            Self::VideoAutoLevels
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoSuggestionsProcessingErrorsEnum {
        #[serde(rename = "audioFile")]
        #[doc = "File contains audio only (e.g., an MP3 file)."]
        AudioFile,
        #[serde(rename = "imageFile")]
        #[doc = "Image file (e.g., a JPEG image)."]
        ImageFile,
        #[serde(rename = "projectFile")]
        #[doc = "Movie project file (e.g., Microsoft Windows Movie Maker project)."]
        ProjectFile,
        #[serde(rename = "notAVideoFile")]
        #[doc = "Other non-video file."]
        NotAVideoFile,
        #[serde(rename = "docFile")]
        #[doc = "Document or text file (e.g., MS Word document)."]
        DocFile,
        #[serde(rename = "archiveFile")]
        #[doc = "An archive file (e.g., a ZIP archive)."]
        ArchiveFile,
        #[serde(rename = "unsupportedSpatialAudioLayout")]
        #[doc = "Unsupported spatial audio layout type."]
        UnsupportedSpatialAudioLayout,
    }
    impl ::std::default::Default for VideoSuggestionsProcessingErrorsEnum {
        fn default() -> Self {
            Self::AudioFile
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoSuggestionsProcessingHintsEnum {
        #[serde(rename = "nonStreamableMov")]
        #[doc = "The MP4 file is not streamable, this will slow down the processing. MOOV atom was not found at the beginning of the file."]
        NonStreamableMov,
        #[serde(rename = "sendBestQualityVideo")]
        #[doc = "Probably a better quality version of the video exists. The video has wide screen aspect ratio, but is not an HD video."]
        SendBestQualityVideo,
        #[serde(rename = "sphericalVideo")]
        #[doc = "Uploaded video is spherical video."]
        SphericalVideo,
        #[serde(rename = "spatialAudio")]
        #[doc = "Uploaded video has spatial audio."]
        SpatialAudio,
        #[serde(rename = "vrVideo")]
        #[doc = "Uploaded video is VR video."]
        VrVideo,
        #[serde(rename = "hdrVideo")]
        #[doc = "Uploaded video is HDR video."]
        HdrVideo,
    }
    impl ::std::default::Default for VideoSuggestionsProcessingHintsEnum {
        fn default() -> Self {
            Self::NonStreamableMov
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum VideoSuggestionsProcessingWarningsEnum {
        #[serde(rename = "unknownContainer")]
        #[doc = "Unrecognized file format, transcoding is likely to fail."]
        UnknownContainer,
        #[serde(rename = "unknownVideoCodec")]
        #[doc = "Unrecognized video codec, transcoding is likely to fail."]
        UnknownVideoCodec,
        #[serde(rename = "unknownAudioCodec")]
        #[doc = "Unrecognized audio codec, transcoding is likely to fail."]
        UnknownAudioCodec,
        #[serde(rename = "inconsistentResolution")]
        #[doc = "Conflicting container and stream resolutions."]
        InconsistentResolution,
        #[serde(rename = "hasEditlist")]
        #[doc = "Edit lists are not currently supported."]
        HasEditlist,
        #[serde(rename = "problematicVideoCodec")]
        #[doc = "Video codec that is known to cause problems was used."]
        ProblematicVideoCodec,
        #[serde(rename = "problematicAudioCodec")]
        #[doc = "Audio codec that is known to cause problems was used."]
        ProblematicAudioCodec,
        #[serde(rename = "unsupportedVrStereoMode")]
        #[doc = "Unsupported VR video stereo mode."]
        UnsupportedVrStereoMode,
        #[serde(rename = "unsupportedSphericalProjectionType")]
        #[doc = "Unsupported spherical video projection type."]
        UnsupportedSphericalProjectionType,
        #[serde(rename = "unsupportedHdrPixelFormat")]
        #[doc = "Unsupported HDR pixel format."]
        UnsupportedHdrPixelFormat,
        #[serde(rename = "unsupportedHdrColorMetadata")]
        #[doc = "Unspecified HDR color metadata."]
        UnsupportedHdrColorMetadata,
        #[serde(rename = "problematicHdrLookupTable")]
        #[doc = "Problematic HDR lookup table attached."]
        ProblematicHdrLookupTable,
    }
    impl ::std::default::Default for VideoSuggestionsProcessingWarningsEnum {
        fn default() -> Self {
            Self::UnknownContainer
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single tag suggestion with it's relevance information."]
    pub struct VideoSuggestionsTagSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "categoryRestricts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A set of video categories for which the tag is relevant. You can use this information to display appropriate tag suggestions based on the video category that the video uploader associates with the video. By default, tag suggestions are relevant for all categories if there are no restricts defined for the keyword."]
        pub category_restricts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The keyword tag suggested for the video."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl VideoSuggestionsTagSuggestion {
        pub fn builder() -> VideoSuggestionsTagSuggestionBuilder {
            VideoSuggestionsTagSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Freebase topic information related to the video."]
    pub struct VideoTopicDetails {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relevantTopicIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API."]
        pub relevant_topic_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicCategories")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Wikipedia URLs that provide a high-level description of the video's content."]
        pub topic_categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "topicIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the < a href=\"http://wiki.freebase.com/wiki/Topic_API\">Freebase Topic API."]
        pub topic_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl VideoTopicDetails {
        pub fn builder() -> VideoTopicDetailsBuilder {
            VideoTopicDetailsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Branding properties for the watch. All deprecated."]
    pub struct WatchSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "backgroundColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text color for the video watch page's branded area."]
        pub background_color: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "featuredPlaylistId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "An ID that uniquely identifies a playlist that displays next to the video player."]
        pub featured_playlist_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textColor")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The background color for the video watch page's branded area."]
        pub text_color: ::std::option::Option<::std::string::String>,
    }
    impl WatchSettings {
        pub fn builder() -> WatchSettingsBuilder {
            WatchSettingsBuilder::default()
        }
    }
}
