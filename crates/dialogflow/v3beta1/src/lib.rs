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
    pub mod projects {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod agents {
                        pub mod methods {
                            pub mod get_validation_result {
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
                                    #[serde(rename = "languageCode")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "If not specified, the agent's default language is used."]
                                    pub language_code: ::std::option::Option<::std::string::String>,
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
                                    #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The next_page_token value returned from a previous list request."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "updateMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The mask to control which fields get updated. If the mask is not present, all fields will be updated."]
                                    pub update_mask: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
                                    }
                                }
                            }
                        }
                        pub mod resources {
                            pub mod entity_types {
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `entity_type`: * `EntityType.entities.value` * `EntityType.entities.synonyms` * `EntityType.excluded_phrases.value` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "force")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "This field has no effect for entity type not being used. For entity types that are used by intents or pages: * If `force` is set to false, an error will be returned with message indicating the referencing resources. * If `force` is set to true, Dialogflow will remove the entity type, as well as any references to the entity type (i.e. Page parameter of the entity type will be changed to '@sys.any' and intent parameter of the entity type will be removed)."]
                                            pub force:
                                                ::std::option::Option<::std::primitive::bool>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to retrieve the entity type for. The following fields are language dependent: * `EntityType.entities.value` * `EntityType.entities.synonyms` * `EntityType.excluded_phrases.value` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to list entity types for. The following fields are language dependent: * `EntityType.entities.value` * `EntityType.entities.synonyms` * `EntityType.excluded_phrases.value` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `entity_type`: * `EntityType.entities.value` * `EntityType.entities.synonyms` * `EntityType.excluded_phrases.value` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The mask to control which fields get updated."]
                                            pub update_mask:
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
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of items to return in a single page. By default 20 and at most 100."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod lookup_environment_history {
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
                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The mask to control which fields get updated."]
                                            pub update_mask:
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
                                    pub mod experiments {
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
                                                    #[doc = "The maximum number of items to return in a single page. By default 20 and at most 100."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Required. The mask to control which fields get updated."]
                                                    pub update_mask: ::std::option::Option<
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
                                    pub mod sessions {
                                        pub mod resources {
                                            pub mod entity_types {
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
                                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                                            #[doc = "The next_page_token value returned from a previous list request."]
                                                            pub page_token: ::std::option::Option<
                                                                ::std::string::String,
                                                            >,
                                                        }
                                                        impl QueryParameters {
                                                            pub fn builder(
                                                            ) -> QueryParametersBuilder
                                                            {
                                                                QueryParametersBuilder::default()
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
                                                                default = "{ ::std::default::Default::default() }",
                                                                setter(into)
                                                            )]
                                                            #[serde(rename = "updateMask")]
                                                            #[serde(
                                                                skip_serializing_if = "::std::option::Option::is_none"
                                                            )]
                                                            #[doc = "The mask to control which fields get updated."]
                                                            pub update_mask: ::std::option::Option<
                                                                ::std::string::String,
                                                            >,
                                                        }
                                                        impl QueryParameters {
                                                            pub fn builder(
                                                            ) -> QueryParametersBuilder
                                                            {
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
                            pub mod flows {
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `flow`: * `Flow.event_handlers.trigger_fulfillment.messages` * `Flow.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "force")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "This field has no effect for flows with no incoming transitions. For flows with incoming transitions: * If `force` is set to false, an error will be returned with message indicating the incoming transitions. * If `force` is set to true, Dialogflow will remove the flow, as well as any transitions to the flow (i.e. Target flow in event handlers or Target flow in transition routes that point to this flow will be cleared)."]
                                            pub force:
                                                ::std::option::Option<::std::primitive::bool>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to retrieve the flow for. The following fields are language dependent: * `Flow.event_handlers.trigger_fulfillment.messages` * `Flow.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
                                            }
                                        }
                                    }
                                    pub mod get_validation_result {
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "If not specified, the agent's default language is used."]
                                            pub language_code:
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to list flows for. The following fields are language dependent: * `Flow.event_handlers.trigger_fulfillment.messages` * `Flow.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `flow`: * `Flow.event_handlers.trigger_fulfillment.messages` * `Flow.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The mask to control which fields get updated. If `update_mask` is not specified, an error will be returned."]
                                            pub update_mask:
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
                                    pub mod pages {
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
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language of the following fields in `page`: * `Page.entry_fulfillment.messages` * `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages` * `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages` * `Page.transition_routes.trigger_fulfillment.messages` * `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "force")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "This field has no effect for pages with no incoming transitions. For pages with incoming transitions: * If `force` is set to false, an error will be returned with message indicating the incoming transitions. * If `force` is set to true, Dialogflow will remove the page, as well as any transitions to the page (i.e. Target page in event handlers or Target page in transition routes that point to this page will be cleared)."]
                                                    pub force: ::std::option::Option<
                                                        ::std::primitive::bool,
                                                    >,
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
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to retrieve the page for. The following fields are language dependent: * `Page.entry_fulfillment.messages` * `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages` * `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages` * `Page.transition_routes.trigger_fulfillment.messages` * `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to list pages for. The following fields are language dependent: * `Page.entry_fulfillment.messages` * `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages` * `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages` * `Page.transition_routes.trigger_fulfillment.messages` * `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language of the following fields in `page`: * `Page.entry_fulfillment.messages` * `Page.form.parameters.fill_behavior.initial_prompt_fulfillment.messages` * `Page.form.parameters.fill_behavior.reprompt_event_handlers.messages` * `Page.transition_routes.trigger_fulfillment.messages` * `Page.transition_route_groups.transition_routes.trigger_fulfillment.messages` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The mask to control which fields get updated. If the mask is not present, all fields will be updated."]
                                                    pub update_mask: ::std::option::Option<
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
                                    pub mod transition_route_groups {
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
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to list transition route groups for. The field `messages` in TransitionRoute is language dependent. If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "force")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "This field has no effect for transition route group that no page is using. If the transition route group is referenced by any page: * If `force` is set to false, an error will be returned with message indicating pages that reference the transition route group. * If `force` is set to true, Dialogflow will remove the transition route group, as well as any reference to it."]
                                                    pub force: ::std::option::Option<
                                                        ::std::primitive::bool,
                                                    >,
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
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to list transition route groups for. The field `messages` in TransitionRoute is language dependent. If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
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
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to list transition route groups for. The field `messages` in TransitionRoute is language dependent. If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "languageCode")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The language to list transition route groups for. The field `messages` in TransitionRoute is language dependent. If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                                    pub language_code: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The mask to control which fields get updated."]
                                                    pub update_mask: ::std::option::Option<
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
                                    pub mod versions {
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
                                                    #[doc = "The maximum number of items to return in a single page. By default 20 and at most 100."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "Required. The mask to control which fields get updated. Currently only `description` and `display_name` can be updated."]
                                                    pub update_mask: ::std::option::Option<
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
                            pub mod intents {
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
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `intent`: * `Intent.training_phrases.parts.text` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to retrieve the intent for. The following fields are language dependent: * `Intent.training_phrases.parts.text` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
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
                                            #[serde(rename = "intentView")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The resource view to apply to the returned intent."]
                                            pub intent_view: ::std::option::Option<
                                                QueryParametersIntentViewEnum,
                                            >,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language to list intents for. The following fields are language dependent: * `Intent.training_phrases.parts.text` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
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
                                        #[doc = "The resource view to apply to the returned intent."]
                                        pub enum QueryParametersIntentViewEnum {
                                            #[serde(rename = "INTENT_VIEW_UNSPECIFIED")]
                                            #[doc = "Not specified. Treated as INTENT_VIEW_FULL."]
                                            IntentViewUnspecified,
                                            #[serde(rename = "INTENT_VIEW_PARTIAL")]
                                            #[doc = "Training phrases field is not populated in the response."]
                                            IntentViewPartial,
                                            #[serde(rename = "INTENT_VIEW_FULL")]
                                            #[doc = "All fields are populated."]
                                            IntentViewFull,
                                        }
                                        impl ::std::default::Default for QueryParametersIntentViewEnum {
                                            fn default() -> Self {
                                                Self::IntentViewUnspecified
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "languageCode")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The language of the following fields in `intent`: * `Intent.training_phrases.parts.text` If not specified, the agent's default language is used. [Many languages](https://cloud.google.com/dialogflow/docs/reference/language) are supported. Note: languages must be enabled in the agent before they can be used."]
                                            pub language_code:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The mask to control which fields get updated. If the mask is not present, all fields will be updated."]
                                            pub update_mask:
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
                            pub mod sessions {
                                pub mod resources {
                                    pub mod entity_types {
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
                                                    #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "updateMask")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The mask to control which fields get updated."]
                                                    pub update_mask: ::std::option::Option<
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
                            pub mod test_cases {
                                pub mod methods {
                                    pub mod calculate_coverage {
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
                                            #[doc = "Required. The type of coverage requested."]
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
                                        #[doc = "Required. The type of coverage requested."]
                                        pub enum QueryParametersTypeEnum {
                                            #[serde(rename = "COVERAGE_TYPE_UNSPECIFIED")]
                                            #[doc = "Should never be used."]
                                            CoverageTypeUnspecified,
                                            #[serde(rename = "INTENT")]
                                            #[doc = "Intent coverage."]
                                            Intent,
                                            #[serde(rename = "PAGE_TRANSITION")]
                                            #[doc = "Page transition coverage."]
                                            PageTransition,
                                            #[serde(rename = "TRANSITION_ROUTE_GROUP")]
                                            #[doc = "Transition route group coverage."]
                                            TransitionRouteGroup,
                                        }
                                        impl ::std::default::Default for QueryParametersTypeEnum {
                                            fn default() -> Self {
                                                Self::CoverageTypeUnspecified
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
                                            #[doc = "The maximum number of items to return in a single page. By default 20. Note that when TestCaseView = FULL, the maximum page size allowed is 20. When TestCaseView = BASIC, the maximum page size allowed is 500."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "view")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Specifies whether response should include all fields or just the metadata."]
                                            pub view:
                                                ::std::option::Option<QueryParametersViewEnum>,
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
                                        #[doc = "Specifies whether response should include all fields or just the metadata."]
                                        pub enum QueryParametersViewEnum {
                                            #[serde(rename = "TEST_CASE_VIEW_UNSPECIFIED")]
                                            #[doc = "The default / unset value. The API will default to the BASIC view."]
                                            TestCaseViewUnspecified,
                                            #[serde(rename = "BASIC")]
                                            #[doc = "Include basic metadata about the test case, but not the conversation turns. This is the default value."]
                                            Basic,
                                            #[serde(rename = "FULL")]
                                            #[doc = "Include everything."]
                                            Full,
                                        }
                                        impl ::std::default::Default for QueryParametersViewEnum {
                                            fn default() -> Self {
                                                Self::TestCaseViewUnspecified
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The mask to specify which fields should be updated. The `creationTime` and `lastTestResult` cannot be updated."]
                                            pub update_mask:
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
                                    pub mod results {
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
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The filter expression used to filter test case results. See [API Filtering](https://aip.dev/160). The expression is case insensitive. Only 'AND' is supported for logical operators. The supported syntax is listed below in detail: [AND ] ... [AND latest] The supported fields and operators are: field operator `environment` `=`, `IN` (Use value `draft` for draft environment) `test_time` `>`, `<` `latest` only returns the latest test result in all results for each test case. Examples: * \"environment=draft AND latest\" matches the latest test result for each test case in the draft environment. * \"environment IN (e1,e2)\" matches any test case results with an environment resource name of either \"e1\" or \"e2\". * \"test_time > 1602540713\" matches any test case results with test time later than a unix timestamp in seconds 1602540713."]
                                                    pub filter: ::std::option::Option<
                                                        ::std::string::String,
                                                    >,
                                                    #[builder(
                                                        default = "{ ::std::default::Default::default() }",
                                                        setter(into)
                                                    )]
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                                    #[doc = "The next_page_token value returned from a previous list request."]
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
                            pub mod webhooks {
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
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "force")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "This field has no effect for webhook not being used. For webhooks that are used by pages/flows/transition route groups: * If `force` is set to false, an error will be returned with message indicating the referenced resources. * If `force` is set to true, Dialogflow will remove the webhook, as well as any references to the webhook (i.e. Webhook and tagin fulfillments that point to this webhook will be removed)."]
                                            pub force:
                                                ::std::option::Option<::std::primitive::bool>,
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
                                            #[doc = "The maximum number of items to return in a single page. By default 100 and at most 1000."]
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
                                            #[doc = "The next_page_token value returned from a previous list request."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                        }
                                        impl QueryParameters {
                                            pub fn builder() -> QueryParametersBuilder {
                                                QueryParametersBuilder::default()
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
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "updateMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The mask to control which fields get updated. If the mask is not present, all fields will be updated."]
                                            pub update_mask:
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
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "filter")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list filter."]
                                    pub filter: ::std::option::Option<::std::string::String>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageSize")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The standard list page size."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
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
                    pub mod security_settings {
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
                                    #[doc = "The maximum number of items to return in a single page. By default 20 and at most 100."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The next_page_token value returned from a previous list request."]
                                    pub page_token: ::std::option::Option<::std::string::String>,
                                }
                                impl QueryParameters {
                                    pub fn builder() -> QueryParametersBuilder {
                                        QueryParametersBuilder::default()
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
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "updateMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The mask to control which fields get updated. If the mask is not present, all fields will be updated."]
                                    pub update_mask: ::std::option::Option<::std::string::String>,
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
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "filter")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list filter."]
                            pub filter: ::std::option::Option<::std::string::String>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
                            #[serde(rename = "pageSize")]
                            #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                            #[doc = "The standard list page size."]
                            pub page_size: ::std::option::Option<::std::primitive::i64>,
                            #[builder(
                                default = "{ ::std::default::Default::default() }",
                                setter(into)
                            )]
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
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the natural speech audio to be processed."]
    pub struct GoogleCloudDialogflowCxV3AudioInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes. For non-streaming audio detect intent, both `config` and `audio` must be provided. For streaming audio detect intent, `config` must be provided in the first request and `audio` must be provided in all following requests."]
        pub audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instructs the speech recognizer how to process the speech audio."]
        pub config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3InputAudioConfig>>,
    }
    impl GoogleCloudDialogflowCxV3AudioInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3AudioInputBuilder {
            GoogleCloudDialogflowCxV3AudioInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.BatchRunTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3BatchRunTestCasesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test errors."]
        pub errors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestError>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3BatchRunTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3BatchRunTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3BatchRunTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.BatchRunTestCases."]
    pub struct GoogleCloudDialogflowCxV3BatchRunTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case results. The detailed conversation turns are empty in this response."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3BatchRunTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3BatchRunTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3BatchRunTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "One interaction between a human and virtual agent. The human provides some input and the virtual agent provides a response."]
    pub struct GoogleCloudDialogflowCxV3ConversationTurn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user input."]
        pub user_input: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurnUserInput>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "virtualAgentOutput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The virtual agent output."]
        pub virtual_agent_output: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput>,
        >,
    }
    impl GoogleCloudDialogflowCxV3ConversationTurn {
        pub fn builder() -> GoogleCloudDialogflowCxV3ConversationTurnBuilder {
            GoogleCloudDialogflowCxV3ConversationTurnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The input from the human user."]
    pub struct GoogleCloudDialogflowCxV3ConversationTurnUserInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "injectedParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters that need to be injected into the conversation during intent detection."]
        pub injected_parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supports text input, event input, dtmf input in the test case."]
        pub input: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3QueryInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isWebhookEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
        pub is_webhook_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3ConversationTurnUserInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3ConversationTurnUserInputBuilder {
            GoogleCloudDialogflowCxV3ConversationTurnUserInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The output from the virtual agent."]
    pub struct GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Page on which the utterance was spoken. Only name and displayName will be set."]
        pub current_page: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Page>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnosticInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The diagnostic info output for the turn."]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "differences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If this is part of a result conversation turn, the list of differences between the original run and the replay for this output, if any."]
        pub differences: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestRunDifference>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The session parameters available to the bot at this point."]
        pub session_parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response error from the agent in the test result. If set, other output is empty."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text responses from the agent for the turn."]
        pub text_responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageText>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggeredIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Intent that triggered the response. Only name and displayName will be set."]
        pub triggered_intent:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Intent>>,
    }
    impl GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutput {
        pub fn builder() -> GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutputBuilder {
            GoogleCloudDialogflowCxV3ConversationTurnVirtualAgentOutputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for CreateDocument operation."]
    pub struct GoogleCloudDialogflowCxV3CreateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3CreateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3CreateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3CreateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata associated with the long running operation for Versions.CreateVersion."]
    pub struct GoogleCloudDialogflowCxV3CreateVersionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the created version. Format: `projects//locations//agents//flows//versions/`."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3CreateVersionOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3CreateVersionOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3CreateVersionOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for DeleteDocument operation."]
    pub struct GoogleCloudDialogflowCxV3DeleteDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3DeleteDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3DeleteDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3DeleteDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the input for dtmf event."]
    pub struct GoogleCloudDialogflowCxV3DtmfInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dtmf digits."]
        pub digits: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishDigit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The finish digit (if any)."]
        pub finish_digit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3DtmfInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3DtmfInputBuilder {
            GoogleCloudDialogflowCxV3DtmfInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event handler specifies an event that can be handled during a session. When the specified event happens, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the event, it will be called. * If there is a `target_page` associated with the event, the session will transition into the specified page. * If there is a `target_flow` associated with the event, the session will transition into the specified flow."]
    pub struct GoogleCloudDialogflowCxV3EventHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the event to handle."]
        pub event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this event handler."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks."]
        pub trigger_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
    }
    impl GoogleCloudDialogflowCxV3EventHandler {
        pub fn builder() -> GoogleCloudDialogflowCxV3EventHandlerBuilder {
            GoogleCloudDialogflowCxV3EventHandlerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the event to trigger."]
    pub struct GoogleCloudDialogflowCxV3EventInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the event."]
        pub event: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3EventInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3EventInputBuilder {
            GoogleCloudDialogflowCxV3EventInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.ExportAgent."]
    pub struct GoogleCloudDialogflowCxV3ExportAgentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for agent."]
        pub agent_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in ExportAgentRequest."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3ExportAgentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3ExportAgentResponseBuilder {
            GoogleCloudDialogflowCxV3ExportAgentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.ExportTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3ExportTestCasesMetadata {}
    impl GoogleCloudDialogflowCxV3ExportTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3ExportTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3ExportTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ExportTestCases."]
    pub struct GoogleCloudDialogflowCxV3ExportTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for test cases."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported test cases. This field is populated only if `gcs_uri` is specified in ExportTestCasesRequest."]
        pub gcs_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3ExportTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3ExportTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3ExportTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A form is a data model that groups related parameters that can be collected from the user. The process in which the agent prompts the user and collects parameter values from the user is called form filling. A form can be added to a page. When form filling is done, the filled parameters will be written to the session."]
    pub struct GoogleCloudDialogflowCxV3Form {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters to collect from the user."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3FormParameter>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3Form {
        pub fn builder() -> GoogleCloudDialogflowCxV3FormBuilder {
            GoogleCloudDialogflowCxV3FormBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a form parameter."]
    pub struct GoogleCloudDialogflowCxV3FormParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default value of an optional parameter. If the parameter is required, the default value will be ignored."]
        pub default_value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the parameter, unique within the form."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fillBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Defines fill behavior for the parameter."]
        pub fill_behavior: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3FormParameterFillBehavior>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
        pub redact: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "required")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
        pub required: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3FormParameter {
        pub fn builder() -> GoogleCloudDialogflowCxV3FormParameterBuilder {
            GoogleCloudDialogflowCxV3FormParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how the filling of a parameter should be handled."]
    pub struct GoogleCloudDialogflowCxV3FormParameterFillBehavior {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialPromptFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter."]
        pub initial_prompt_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repromptEventHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are: * `sys.no-match-`, where N can be from 1 to 6 * `sys.no-match-default` * `sys.no-input-`, where N can be from 1 to 6 * `sys.no-input-default` * `sys.invalid-parameter` `initial_prompt_fulfillment` provides the first prompt for the parameter. If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the `sys.no-match-1`/`sys.no-input-1` handler (if defined) will be called to provide a prompt. The `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to the next no-match/no-input event, and so on. A `sys.no-match-default` or `sys.no-input-default` handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed. A `sys.invalid-parameter` handler can be defined to handle the case where the parameter values have been `invalidated` by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the `sys.invalid-parameter` handler (if defined) will be called to provide a prompt. If the event handler for the corresponding event can't be found on the parameter, `initial_prompt_fulfillment` will be re-prompted."]
        pub reprompt_event_handlers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3EventHandler>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3FormParameterFillBehavior {
        pub fn builder() -> GoogleCloudDialogflowCxV3FormParameterFillBehaviorBuilder {
            GoogleCloudDialogflowCxV3FormParameterFillBehaviorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A fulfillment can do one or more of the following actions at the same time: * Generate rich message responses. * Set parameter values. * Call the webhook. Fulfillments can be called at various stages in the Page or Form lifecycle. For example, when a DetectIntentRequest drives a session to enter a new page, the page's entry fulfillment can add a static response to the QueryResult in the returning DetectIntentResponse, call the webhook (for example, to load user data from a database), or both."]
    pub struct GoogleCloudDialogflowCxV3Fulfillment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionalCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conditional cases for this fulfillment."]
        pub conditional_cases: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setParameterActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set parameter values before executing the webhook."]
        pub set_parameter_actions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentSetParameterAction>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tag used by the webhook to identify which fulfillment is being called. This field is required if `webhook` is specified."]
        pub tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhook")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The webhook to call. Format: `projects//locations//agents//webhooks/`."]
        pub webhook: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3Fulfillment {
        pub fn builder() -> GoogleCloudDialogflowCxV3FulfillmentBuilder {
            GoogleCloudDialogflowCxV3FulfillmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored."]
    pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCases {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of cascading if-else conditions."]
        pub cases: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3FulfillmentConditionalCases {
        pub fn builder() -> GoogleCloudDialogflowCxV3FulfillmentConditionalCasesBuilder {
            GoogleCloudDialogflowCxV3FulfillmentConditionalCasesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Each case has a Boolean condition. When it is evaluated to be True, the corresponding messages will be selected and evaluated recursively."]
    pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of case content."]
        pub case_content: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition to activate and select this case. Empty means the condition is always true. The condition is evaluated against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition)."]
        pub condition: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCase {
        pub fn builder() -> GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseBuilder {
            GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The list of messages or conditional cases to activate for this case."]
    pub struct GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional cases to be evaluated."]
        pub additional_cases: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3FulfillmentConditionalCases>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returned message."]
        pub message:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
    }
    impl GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContent {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContentBuilder {
            GoogleCloudDialogflowCxV3FulfillmentConditionalCasesCaseCaseContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Setting a parameter value."]
    pub struct GoogleCloudDialogflowCxV3FulfillmentSetParameterAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the parameter."]
        pub parameter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new value of the parameter. A null value clears the parameter."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3FulfillmentSetParameterAction {
        pub fn builder() -> GoogleCloudDialogflowCxV3FulfillmentSetParameterActionBuilder {
            GoogleCloudDialogflowCxV3FulfillmentSetParameterActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
    pub struct GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Output only. The current state of this operation."]
        pub state: ::std::option::Option<
            GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataStateEnum,
        >,
    }
    impl GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Output only. The current state of this operation."]
    pub enum GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadataStateEnum
    {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ImportDocuments operation."]
    pub struct GoogleCloudDialogflowCxV3ImportDocumentsOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3ImportDocumentsOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3ImportDocumentsOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3ImportDocumentsOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Documents.ImportDocuments."]
    pub struct GoogleCloudDialogflowCxV3ImportDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Includes details about skipped documents or any other warnings."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDialogflowCxV3ImportDocumentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3ImportDocumentsResponseBuilder {
            GoogleCloudDialogflowCxV3ImportDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.ImportTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3ImportTestCasesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Errors for failed test cases."]
        pub errors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseError>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3ImportTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3ImportTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3ImportTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ImportTestCases."]
    pub struct GoogleCloudDialogflowCxV3ImportTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifiers of the new test cases. Format: `projects//locations//agents//testCases/`."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3ImportTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3ImportTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3ImportTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instructs the speech recognizer on how to process the audio content."]
    pub struct GoogleCloudDialogflowCxV3InputAudioConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Audio encoding of the audio content to process."]
        pub audio_encoding:
            ::std::option::Option<GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableWordInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information."]
        pub enable_word_info: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVariant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Which variant of the Speech model to use."]
        pub model_variant:
            ::std::option::Option<GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phraseHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details."]
        pub phrase_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details."]
        pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "singleUtterance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods."]
        pub single_utterance: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3InputAudioConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3InputAudioConfigBuilder {
            GoogleCloudDialogflowCxV3InputAudioConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Audio encoding of the audio content to process."]
    pub enum GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
        #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
        #[doc = "Not specified."]
        AudioEncodingUnspecified,
        #[serde(rename = "AUDIO_ENCODING_LINEAR_16")]
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
        AudioEncodingLinear16,
        #[serde(rename = "AUDIO_ENCODING_FLAC")]
        #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
        AudioEncodingFlac,
        #[serde(rename = "AUDIO_ENCODING_MULAW")]
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
        AudioEncodingMulaw,
        #[serde(rename = "AUDIO_ENCODING_AMR")]
        #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
        AudioEncodingAmr,
        #[serde(rename = "AUDIO_ENCODING_AMR_WB")]
        #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
        AudioEncodingAmrWb,
        #[serde(rename = "AUDIO_ENCODING_OGG_OPUS")]
        #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000."]
        AudioEncodingOggOpus,
        #[serde(rename = "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
        #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
        AudioEncodingSpeexWithHeaderByte,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3InputAudioConfigAudioEncodingEnum {
        fn default() -> Self {
            Self::AudioEncodingUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Which variant of the Speech model to use."]
    pub enum GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
        #[serde(rename = "SPEECH_MODEL_VARIANT_UNSPECIFIED")]
        #[doc = "No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE."]
        SpeechModelVariantUnspecified,
        #[serde(rename = "USE_BEST_AVAILABLE")]
        #[doc = "Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models."]
        UseBestAvailable,
        #[serde(rename = "USE_STANDARD")]
        #[doc = "Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models."]
        UseStandard,
        #[serde(rename = "USE_ENHANCED")]
        #[doc = "Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible."]
        UseEnhanced,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3InputAudioConfigModelVariantEnum {
        fn default() -> Self {
            Self::SpeechModelVariantUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An intent represents a user's intent to interact with a conversational agent. You can provide information for the Dialogflow API to use to match user input to an intent by adding training phrases (i.e., examples of user input) to your intent."]
    pub struct GoogleCloudDialogflowCxV3Intent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the intent, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFallback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event."]
        pub is_fallback: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix \"sys.\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys.head * sys.contextual The above labels do not require value. \"sys.head\" means the intent is a head intent. \"sys.contextual\" means the intent is a contextual intent."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of parameters associated with the intent."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentParameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of training phrases the agent is trained on to identify the intent."]
        pub training_phrases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentTrainingPhrase>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3Intent {
        pub fn builder() -> GoogleCloudDialogflowCxV3IntentBuilder {
            GoogleCloudDialogflowCxV3IntentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the intent to trigger programmatically rather than as a result of natural language processing."]
    pub struct GoogleCloudDialogflowCxV3IntentInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the intent. Format: `projects//locations//agents//intents/`."]
        pub intent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3IntentInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3IntentInputBuilder {
            GoogleCloudDialogflowCxV3IntentInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an intent parameter."]
    pub struct GoogleCloudDialogflowCxV3IntentParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
        pub redact: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3IntentParameter {
        pub fn builder() -> GoogleCloudDialogflowCxV3IntentParameterBuilder {
            GoogleCloudDialogflowCxV3IntentParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an example that the agent is trained on to identify the intent."]
    pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the training phrase."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `parameter_id` field is set."]
        pub parts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentTrainingPhrasePart>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeatCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates how many times this example was added to the intent."]
        pub repeat_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDialogflowCxV3IntentTrainingPhrase {
        pub fn builder() -> GoogleCloudDialogflowCxV3IntentTrainingPhraseBuilder {
            GoogleCloudDialogflowCxV3IntentTrainingPhraseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a part of a training phrase."]
    pub struct GoogleCloudDialogflowCxV3IntentTrainingPhrasePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
        pub parameter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text for this part."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3IntentTrainingPhrasePart {
        pub fn builder() -> GoogleCloudDialogflowCxV3IntentTrainingPhrasePartBuilder {
            GoogleCloudDialogflowCxV3IntentTrainingPhrasePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages. For each flow, you define many pages, where your combined pages can handle a complete conversation on the topics the flow is designed for. At any given moment, exactly one page is the current page, the current page is considered active, and the flow associated with that page is considered active. Every flow has a special start page. When a flow initially becomes active, the start page page becomes the current page. For each conversational turn, the current page will either stay the same or transition to another page. You configure each page to collect information from the end-user that is relevant for the conversational state represented by the page. For more information, see the [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page)."]
    pub struct GoogleCloudDialogflowCxV3Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the page, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the session is entering the page."]
        pub entry_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Handlers associated with the page to handle events such as webhook errors, no match or no input."]
        pub event_handlers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3EventHandler>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The form associated with the page, used for collecting parameters relevant to the page."]
        pub form: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRouteGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ordered list of `TransitionRouteGroups` associated with the page. Transition route groups must be unique within a page. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/`."]
        pub transition_route_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evalauted in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified."]
        pub transition_routes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3TransitionRoute>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3Page {
        pub fn builder() -> GoogleCloudDialogflowCxV3PageBuilder {
            GoogleCloudDialogflowCxV3PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents page information communicated to and from the webhook."]
    pub struct GoogleCloudDialogflowCxV3PageInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the current page. Format: `projects//locations//agents//flows//pages/`."]
        pub current_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. Information about the form."]
        pub form_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfoFormInfo>>,
    }
    impl GoogleCloudDialogflowCxV3PageInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3PageInfoBuilder {
            GoogleCloudDialogflowCxV3PageInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents form information."]
    pub struct GoogleCloudDialogflowCxV3PageInfoFormInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. The parameters contained in the form. Note that the webhook cannot add or remove any form parameter."]
        pub parameter_info: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfo>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3PageInfoFormInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3PageInfoFormInfoBuilder {
            GoogleCloudDialogflowCxV3PageInfoFormInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents parameter information."]
    pub struct GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The human-readable name of the parameter, unique within the form. This field cannot be modified by the webhook."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "justCollected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for WebhookRequest. Ignored for WebhookResponse. Indicates if the parameter value was just collected on the last conversation turn."]
        pub just_collected: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "required")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
        pub required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
        pub state:
            ::std::option::Option<GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. The value of the parameter. This field can be set by the webhook to change the parameter value."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoBuilder {
            GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
    pub enum GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoStateEnum {
        #[serde(rename = "PARAMETER_STATE_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        ParameterStateUnspecified,
        #[serde(rename = "EMPTY")]
        #[doc = "Indicates that the parameter does not have a value."]
        Empty,
        #[serde(rename = "INVALID")]
        #[doc = "Indicates that the parameter value is invalid. This field can be used by the webhook to invalidate the parameter and ask the server to collect it from the user again."]
        Invalid,
        #[serde(rename = "FILLED")]
        #[doc = "Indicates that the parameter has a value."]
        Filled,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3PageInfoFormInfoParameterInfoStateEnum {
        fn default() -> Self {
            Self::ParameterStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the query input. It can contain one of: 1. A conversational query in the form of text. 2. An intent query that specifies which intent to trigger. 3. Natural language speech audio to be processed. 4. An event to be triggered. "]
    pub struct GoogleCloudDialogflowCxV3QueryInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language speech audio to be processed."]
        pub audio: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3AudioInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dtmf")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DTMF event to be handled."]
        pub dtmf: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3DtmfInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The event to be triggered."]
        pub event: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3EventInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent to be triggered."]
        pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3IntentInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language of the input. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language text to be processed."]
        pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TextInput>>,
    }
    impl GoogleCloudDialogflowCxV3QueryInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3QueryInputBuilder {
            GoogleCloudDialogflowCxV3QueryInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ReloadDocument operation."]
    pub struct GoogleCloudDialogflowCxV3ReloadDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3ReloadDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3ReloadDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3ReloadDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a response message that can be returned by a conversational agent. Response messages are also used for output audio synthesis. The approach is as follows: * If at least one OutputAudioText response is present, then all OutputAudioText responses are linearly concatenated, and the result is used for output audio synthesis. * If the OutputAudioText responses are a mixture of text and SSML, then the concatenated result is treated as SSML; otherwise, the result is treated as either text or SSML as appropriate. The agent designer should ideally use either text or SSML consistently throughout the bot design. * Otherwise, all Text responses are linearly concatenated, and the result is used for output audio synthesis. This approach allows for more sophisticated user experience scenarios, where the text displayed to the user may differ from what is heard."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversationSuccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that the conversation succeeded."]
        pub conversation_success: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endInteraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A signal that indicates the interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only when the conversation reaches `END_SESSION` or `END_PAGE` page. It is not supposed to be defined by the user. It's guaranteed that there is at most one such message in each response."]
        pub end_interaction: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageEndInteraction>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveAgentHandoff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hands off conversation to a human agent."]
        pub live_agent_handoff: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mixedAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An audio response message composed of both the synthesized Dialogflow agent responses and responses defined via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
        pub mixed_audio: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageMixedAudio>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
        pub output_audio_text: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a response containing a custom, platform-specific payload."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signal that the client should play an audio clip hosted at a client-specific URI. Dialogflow uses this to construct mixed_audio. However, Dialogflow itself does not try to read or process the URI in any way."]
        pub play_audio: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessagePlayAudio>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a text response."]
        pub text:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageText>>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessage {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about. Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates that the conversation succeeded. * In a webhook response when you determine that you handled the customer issue."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom metadata. Dialogflow doesn't impose any structure on this."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageConversationSuccess {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageConversationSuccessBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageConversationSuccessBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageEndInteraction {}
    impl GoogleCloudDialogflowCxV3ResponseMessageEndInteraction {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageEndInteractionBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageEndInteractionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that the conversation should be handed off to a live agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoff {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoffBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageLiveAgentHandoffBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segments this audio response is composed of."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageMixedAudio {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageMixedAudioBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageMixedAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents one segment of audio."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request."]
        pub audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-specific URI that points to an audio clip accessible to the client. Dialogflow does not impose any validation on it."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegment {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegmentBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageMixedAudioSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSML text to be synthesized. For more information, see [SSML](/speech/text-to-speech/docs/ssml)."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw text to be synthesized."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageOutputAudioText {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageOutputAudioTextBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageOutputAudioTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies an audio clip to be played by the client as part of the response."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessagePlayAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
        pub audio_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessagePlayAudio {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessagePlayAudioBuilder {
            GoogleCloudDialogflowCxV3ResponseMessagePlayAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The text response message."]
    pub struct GoogleCloudDialogflowCxV3ResponseMessageText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A collection of text responses."]
        pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3ResponseMessageText {
        pub fn builder() -> GoogleCloudDialogflowCxV3ResponseMessageTextBuilder {
            GoogleCloudDialogflowCxV3ResponseMessageTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.RunTestCase long running operation."]
    pub struct GoogleCloudDialogflowCxV3RunTestCaseMetadata {}
    impl GoogleCloudDialogflowCxV3RunTestCaseMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3RunTestCaseMetadataBuilder {
            GoogleCloudDialogflowCxV3RunTestCaseMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.RunTestCase."]
    pub struct GoogleCloudDialogflowCxV3RunTestCaseResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result."]
        pub result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
    }
    impl GoogleCloudDialogflowCxV3RunTestCaseResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3RunTestCaseResponseBuilder {
            GoogleCloudDialogflowCxV3RunTestCaseResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents session information communicated to and from the webhook."]
    pub struct GoogleCloudDialogflowCxV3SessionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for WebhookRequest. Optional for WebhookResponse. All parameters collected from forms and intents during the session. Parameters can be created, updated, or removed by the webhook. To remove a parameter from the session, the webhook should explicitly set the parameter value to null in WebhookResponse. The map is keyed by parameters' display names."]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the session. This field can be used by the webhook to identify a session. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/` if environment is specified."]
        pub session: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3SessionInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3SessionInfoBuilder {
            GoogleCloudDialogflowCxV3SessionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a test case."]
    pub struct GoogleCloudDialogflowCxV3TestCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When the test was created."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastTestResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latest test result."]
        pub last_test_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCaseResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional freeform notes about the test case. Limit of 400 characters."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with \"#\" and has a limit of 30 characters."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCaseConversationTurns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly."]
        pub test_case_conversation_turns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurn>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for the test case."]
        pub test_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestConfig>>,
    }
    impl GoogleCloudDialogflowCxV3TestCase {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestCaseBuilder {
            GoogleCloudDialogflowCxV3TestCaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Error info for importing a test."]
    pub struct GoogleCloudDialogflowCxV3TestCaseError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status associated with the test case."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case."]
        pub test_case: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3TestCase>>,
    }
    impl GoogleCloudDialogflowCxV3TestCaseError {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestCaseErrorBuilder {
            GoogleCloudDialogflowCxV3TestCaseErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a result from running a test case in an agent environment."]
    pub struct GoogleCloudDialogflowCxV3TestCaseResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversationTurns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conversation turns uttered during the test case replay in chronological order."]
        pub conversation_turns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ConversationTurn>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment where the test was run. If not set, it indicates the draft environment."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name for the test case result. Format: `projects//locations//agents//testCases/ /results/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the test case passed in the agent environment."]
        pub test_result:
            ::std::option::Option<GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the test was run."]
        pub test_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3TestCaseResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestCaseResultBuilder {
            GoogleCloudDialogflowCxV3TestCaseResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the test case passed in the agent environment."]
    pub enum GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
        #[serde(rename = "TEST_RESULT_UNSPECIFIED")]
        #[doc = "Not specified. Should never be used."]
        TestResultUnspecified,
        #[serde(rename = "PASSED")]
        #[doc = "The test passed."]
        Passed,
        #[serde(rename = "FAILED")]
        #[doc = "The test did not pass."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3TestCaseResultTestResultEnum {
        fn default() -> Self {
            Self::TestResultUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents configurations for a test case."]
    pub struct GoogleCloudDialogflowCxV3TestConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flow name. If not set, default start flow is assumed. Format: `projects//locations//agents//flows/`."]
        pub flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session parameters to be compared when calculating differences."]
        pub tracking_parameters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3TestConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestConfigBuilder {
            GoogleCloudDialogflowCxV3TestConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Error info for running a test."]
    pub struct GoogleCloudDialogflowCxV3TestError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status associated with the test."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case resource name."]
        pub test_case: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the test was completed."]
        pub test_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3TestError {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestErrorBuilder {
            GoogleCloudDialogflowCxV3TestErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The description of differences between original and replayed agent output."]
    pub struct GoogleCloudDialogflowCxV3TestRunDifference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the diff, showing the actual output vs expected output."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of diff."]
        pub _type: ::std::option::Option<GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum>,
    }
    impl GoogleCloudDialogflowCxV3TestRunDifference {
        pub fn builder() -> GoogleCloudDialogflowCxV3TestRunDifferenceBuilder {
            GoogleCloudDialogflowCxV3TestRunDifferenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of diff."]
    pub enum GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
        #[serde(rename = "DIFF_TYPE_UNSPECIFIED")]
        #[doc = "Should never be used."]
        DiffTypeUnspecified,
        #[serde(rename = "INTENT")]
        #[doc = "The intent."]
        Intent,
        #[serde(rename = "PAGE")]
        #[doc = "The page."]
        Page,
        #[serde(rename = "PARAMETERS")]
        #[doc = "The parameters."]
        Parameters,
        #[serde(rename = "UTTERANCE")]
        #[doc = "The message utterance."]
        Utterance,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3TestRunDifferenceTypeEnum {
        fn default() -> Self {
            Self::DiffTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the natural language text to be processed."]
    pub struct GoogleCloudDialogflowCxV3TextInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3TextInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3TextInputBuilder {
            GoogleCloudDialogflowCxV3TextInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transition route specifies a intent that can be matched and/or a data condition that can be evaluated during a session. When a specified transition is matched, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the transition, it will be called. * If there is a `target_page` associated with the transition, the session will transition into the specified page. * If there is a `target_flow` associated with the transition, the session will transition into the specified flow."]
    pub struct GoogleCloudDialogflowCxV3TransitionRoute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition to evaluate against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition). At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of an Intent. Format: `projects//locations//agents//intents/`. Indicates that the transition can only happen when the given intent is matched. At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
        pub intent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this transition route."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the condition is satisfied. At least one of `trigger_fulfillment` and `target` must be specified. When both are defined, `trigger_fulfillment` is executed first."]
        pub trigger_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3Fulfillment>>,
    }
    impl GoogleCloudDialogflowCxV3TransitionRoute {
        pub fn builder() -> GoogleCloudDialogflowCxV3TransitionRouteBuilder {
            GoogleCloudDialogflowCxV3TransitionRouteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for UpdateDocument operation."]
    pub struct GoogleCloudDialogflowCxV3UpdateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3UpdateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3UpdateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3UpdateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for a webhook call."]
    pub struct GoogleCloudDialogflowCxV3WebhookRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectIntentResponseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The unique identifier of the DetectIntentResponse that will be returned to the API caller."]
        pub detect_intent_response_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Information about the fulfillment that triggered this webhook call."]
        pub fulfillment_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the last matched intent."]
        pub intent_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestIntentInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user. Webhook can choose to append or replace this list in WebhookResponse.fulfillment_response;"]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about page status."]
        pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom data set in QueryParameters.payload."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentAnalysisResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result of the current user request. The field is filled when sentiment analysis is configured to be enabled for the request."]
        pub sentiment_analysis_result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about session status."]
        pub session_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3SessionInfo>>,
    }
    impl GoogleCloudDialogflowCxV3WebhookRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookRequestBuilder {
            GoogleCloudDialogflowCxV3WebhookRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents fulfillment information communicated to the webhook."]
    pub struct GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The tag used to identify which fulfillment is being called."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfoBuilder {
            GoogleCloudDialogflowCxV3WebhookRequestFulfillmentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents intent information communicated to the webhook."]
    pub struct GoogleCloudDialogflowCxV3WebhookRequestIntentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence of the matched intent. Values range from 0.0 (completely uncertain) to 1.0 (completely certain)."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The display name of the last matched intent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastMatchedIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The unique identifier of the last matched intent. Format: `projects//locations//agents//intents/`."]
        pub last_matched_intent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters identified as a result of intent matching. This is a map of the name of the identified parameter to the value of the parameter identified from the user's utterance. All parameters defined in the matched intent that are identified will be surfaced here."]
        pub parameters: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValue,
                >,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3WebhookRequestIntentInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookRequestIntentInfoBuilder {
            GoogleCloudDialogflowCxV3WebhookRequestIntentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a value for an intent parameter."]
    pub struct GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Original text value extracted from user utterance."]
        pub original_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Structured value for the parameter extracted from user utterance."]
        pub resolved_value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValue {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValueBuilder {
            GoogleCloudDialogflowCxV3WebhookRequestIntentInfoIntentParameterValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of sentiment analysis."]
    pub struct GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResultBuilder {
            GoogleCloudDialogflowCxV3WebhookRequestSentimentAnalysisResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for a webhook call."]
    pub struct GoogleCloudDialogflowCxV3WebhookResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment response to send to the user. This field can be omitted by the webhook if it does not intend to send any response to the user."]
        pub fulfillment_response: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponse>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about page status. This field can be omitted by the webhook if it does not intend to modify page status."]
        pub page_info: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value to append directly to QueryResult.webhook_payloads."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about session status. This field can be omitted by the webhook if it does not intend to modify session status."]
        pub session_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3SessionInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3WebhookResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookResponseBuilder {
            GoogleCloudDialogflowCxV3WebhookResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a fulfillment response to the user."]
    pub struct GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merge behavior for `messages`."]
        pub merge_behavior: ::std::option::Option<
            GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseMergeBehaviorEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3ResponseMessage>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseBuilder {
            GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Merge behavior for `messages`."]
    pub enum GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseMergeBehaviorEnum {
        #[serde(rename = "MERGE_BEHAVIOR_UNSPECIFIED")]
        #[doc = "Not specified. `APPEND` will be used."]
        MergeBehaviorUnspecified,
        #[serde(rename = "APPEND")]
        #[doc = "`messages` will be appended to the list of messages waiting to be sent to the user."]
        Append,
        #[serde(rename = "REPLACE")]
        #[doc = "`messages` will replace the list of messages waiting to be sent to the user."]
        Replace,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3WebhookResponseFulfillmentResponseMergeBehaviorEnum
    {
        fn default() -> Self {
            Self::MergeBehaviorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Agents are best described as Natural Language Understanding (NLU) modules that transform user requests into actionable data. You can include agents in your app, product, or service to determine user intent and respond to the user in a natural way. After you create an agent, you can add Intents, Entity Types, Flows, Fulfillments, Webhooks, and so on to manage the conversation flows.."]
    pub struct GoogleCloudDialogflowCxV3beta1Agent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "avatarUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI of the agent's avatar. Avatars are used throughout the Dialogflow console and in the self-hosted [Web Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo) integration."]
        pub avatar_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultLanguageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. The default language of the agent as a language tag. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. This field cannot be set by the Agents.UpdateAgent method."]
        pub default_language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the agent. The maximum length is 500 characters. If exceeded, the request is rejected."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the agent, unique within the location."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSpellCorrection")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if automatic spell correction is enabled in detect intent requests."]
        pub enable_spell_correction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableStackdriverLogging")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates if stackdriver logging is enabled for the agent."]
        pub enable_stackdriver_logging: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the agent. Required for the Agents.UpdateAgent method. Agents.CreateAgent populates the name automatically. Format: `projects//locations//agents/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securitySettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the SecuritySettings reference for the agent. Format: `projects//locations//securitySettings/`."]
        pub security_settings: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechToTextSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Speech recognition related settings."]
        pub speech_to_text_settings: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SpeechToTextSettings>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Immutable. Name of the start flow in this agent. A start flow will be automatically created when the agent is created, and can only be deleted by deleting the agent. Format: `projects//locations//agents//flows/`."]
        pub start_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The time zone of the agent from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris."]
        pub time_zone: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1Agent {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1AgentBuilder {
            GoogleCloudDialogflowCxV3beta1AgentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.GetAgentValidationResult."]
    pub struct GoogleCloudDialogflowCxV3beta1AgentValidationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flowValidationResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains all flow validation results."]
        pub flow_validation_results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FlowValidationResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the agent validation result. Format: `projects//locations//agents//validationResult`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1AgentValidationResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1AgentValidationResultBuilder {
            GoogleCloudDialogflowCxV3beta1AgentValidationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the natural speech audio to be processed."]
    pub struct GoogleCloudDialogflowCxV3beta1AudioInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language speech audio to be processed. A single request can contain up to 1 minute of speech audio data. The transcribed text cannot contain more than 256 bytes. For non-streaming audio detect intent, both `config` and `audio` must be provided. For streaming audio detect intent, `config` must be provided in the first request and `audio` must be provided in all following requests."]
        pub audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "config")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Instructs the speech recognizer how to process the speech audio."]
        pub config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1InputAudioConfig>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1AudioInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1AudioInputBuilder {
            GoogleCloudDialogflowCxV3beta1AudioInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for TestCases.BatchDeleteTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1BatchDeleteTestCasesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Format of test case names: `projects//locations/ /agents//testCases/`."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1BatchDeleteTestCasesRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1BatchDeleteTestCasesRequestBuilder {
            GoogleCloudDialogflowCxV3beta1BatchDeleteTestCasesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.BatchRunTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3beta1BatchRunTestCasesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test errors."]
        pub errors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestError>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1BatchRunTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1BatchRunTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1BatchRunTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for TestCases.BatchRunTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1BatchRunTestCasesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If not set, draft environment is assumed. Format: `projects//locations//agents//environments/`."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Format: `projects//locations//agents//testCases/`."]
        pub test_cases: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1BatchRunTestCasesRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1BatchRunTestCasesRequestBuilder {
            GoogleCloudDialogflowCxV3beta1BatchRunTestCasesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.BatchRunTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1BatchRunTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case results. The detailed conversation turns are empty in this response."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1BatchRunTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1BatchRunTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1BatchRunTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.CalculateCoverage."]
    pub struct GoogleCloudDialogflowCxV3beta1CalculateCoverageResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The agent to calculate coverage for. Format: `projects//locations//agents/`."]
        pub agent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentCoverage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intent coverage."]
        pub intent_coverage:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentCoverage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeGroupCoverage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transition route group coverage."]
        pub route_group_coverage: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionCoverage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transition (excluding transition route groups) coverage."]
        pub transition_coverage: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionCoverage>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1CalculateCoverageResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1CalculateCoverageResponseBuilder {
            GoogleCloudDialogflowCxV3beta1CalculateCoverageResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "One interaction between a human and virtual agent. The human provides some input and the virtual agent provides a response."]
    pub struct GoogleCloudDialogflowCxV3beta1ConversationTurn {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user input."]
        pub user_input: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurnUserInput>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "virtualAgentOutput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The virtual agent output."]
        pub virtual_agent_output: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutput>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ConversationTurn {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ConversationTurnBuilder {
            GoogleCloudDialogflowCxV3beta1ConversationTurnBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The input from the human user."]
    pub struct GoogleCloudDialogflowCxV3beta1ConversationTurnUserInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "injectedParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters that need to be injected into the conversation during intent detection."]
        pub injected_parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "input")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Supports text input, event input, dtmf input in the test case."]
        pub input:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isWebhookEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If webhooks should be allowed to trigger in response to the user utterance. Often if parameters are injected, webhooks should not be enabled."]
        pub is_webhook_enabled: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1ConversationTurnUserInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ConversationTurnUserInputBuilder {
            GoogleCloudDialogflowCxV3beta1ConversationTurnUserInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The output from the virtual agent."]
    pub struct GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Page on which the utterance was spoken. Only name and displayName will be set."]
        pub current_page:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnosticInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Input only. The diagnostic info output for the turn."]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "differences")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If this is part of a result conversation turn, the list of differences between the original run and the replay for this output, if any."]
        pub differences: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestRunDifference>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The session parameters available to the bot at this point."]
        pub session_parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Response error from the agent in the test result. If set, other output is empty."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text responses from the agent for the turn."]
        pub text_responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageText>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggeredIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Intent that triggered the response. Only name and displayName will be set."]
        pub triggered_intent:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Intent>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutputBuilder
        {
            GoogleCloudDialogflowCxV3beta1ConversationTurnVirtualAgentOutputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for CreateDocument operation."]
    pub struct GoogleCloudDialogflowCxV3beta1CreateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1CreateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1CreateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1CreateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata associated with the long running operation for Versions.CreateVersion."]
    pub struct GoogleCloudDialogflowCxV3beta1CreateVersionOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the created version. Format: `projects//locations//agents//flows//versions/`."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1CreateVersionOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1CreateVersionOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1CreateVersionOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for DeleteDocument operation."]
    pub struct GoogleCloudDialogflowCxV3beta1DeleteDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1DeleteDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1DeleteDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1DeleteDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request to detect user's intent."]
    pub struct GoogleCloudDialogflowCxV3beta1DetectIntentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instructs the speech synthesizer how to generate the output audio."]
        pub output_audio_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1OutputAudioConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The input specification."]
        pub query_input:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters of this query."]
        pub query_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryParameters>>,
    }
    impl GoogleCloudDialogflowCxV3beta1DetectIntentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1DetectIntentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1DetectIntentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The message returned from the DetectIntent method."]
    pub struct GoogleCloudDialogflowCxV3beta1DetectIntentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.response_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content."]
        pub output_audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The config used by the speech synthesizer to generate the output audio."]
        pub output_audio_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1OutputAudioConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the conversational query."]
        pub query_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues."]
        pub response_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1DetectIntentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1DetectIntentResponseBuilder {
            GoogleCloudDialogflowCxV3beta1DetectIntentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the input for dtmf event."]
    pub struct GoogleCloudDialogflowCxV3beta1DtmfInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "digits")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The dtmf digits."]
        pub digits: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "finishDigit")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The finish digit (if any)."]
        pub finish_digit: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1DtmfInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1DtmfInputBuilder {
            GoogleCloudDialogflowCxV3beta1DtmfInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entities are extracted from user input and represent parameters that are meaningful to your application. For example, a date range, a proper name such as a geographic location or landmark, and so on. Entities represent actionable data for your application. When you define an entity, you can also include synonyms that all map to that entity. For example, \"soft drink\", \"soda\", \"pop\", and so on. There are three types of entities: * **System** - entities that are defined by the Dialogflow API for common data types such as date, time, currency, and so on. A system entity is represented by the `EntityType` type. * **Custom** - entities that are defined by you that represent actionable data that is meaningful to your application. For example, you could define a `pizza.sauce` entity for red or white pizza sauce, a `pizza.cheese` entity for the different types of cheese on a pizza, a `pizza.topping` entity for different toppings, and so on. A custom entity is represented by the `EntityType` type. * **User** - entities that are built for an individual user such as favorites, preferences, playlists, and so on. A user entity is represented by the SessionEntityType type. For more information about entity types, see the [Dialogflow documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct GoogleCloudDialogflowCxV3beta1EntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoExpansionMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the entity type can be automatically expanded."]
        pub auto_expansion_mode:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1EntityTypeAutoExpansionModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the entity type, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableFuzzyExtraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Enables fuzzy entity extraction during classification."]
        pub enable_fuzzy_extraction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of entity entries associated with the entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "excludedPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Collection of exceptional words and phrases that shouldn't be matched. For example, if you have a size entity type with entry `giant`(an adjective), you might consider adding `giants`(a noun) as an exclusion. If the kind of entity type is `KIND_MAP`, then the phrases specified by entities and excluded phrases should be mutually exclusive."]
        pub excluded_phrases: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EntityTypeExcludedPhrase>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates the kind of entity type."]
        pub kind: ::std::option::Option<GoogleCloudDialogflowCxV3beta1EntityTypeKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType. Format: `projects//locations//agents//entityTypes/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether parameters of the entity type should be redacted in log. If redaction is enabled, page parameters and intent parameters referring to the entity type will be replaced by parameter name during logging."]
        pub redact: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1EntityType {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EntityTypeBuilder {
            GoogleCloudDialogflowCxV3beta1EntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates whether the entity type can be automatically expanded."]
    pub enum GoogleCloudDialogflowCxV3beta1EntityTypeAutoExpansionModeEnum {
        #[serde(rename = "AUTO_EXPANSION_MODE_UNSPECIFIED")]
        #[doc = "Auto expansion disabled for the entity."]
        AutoExpansionModeUnspecified,
        #[serde(rename = "AUTO_EXPANSION_MODE_DEFAULT")]
        #[doc = "Allows an agent to recognize values that have not been explicitly listed in the entity."]
        AutoExpansionModeDefault,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1EntityTypeAutoExpansionModeEnum {
        fn default() -> Self {
            Self::AutoExpansionModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates the kind of entity type."]
    pub enum GoogleCloudDialogflowCxV3beta1EntityTypeKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        KindUnspecified,
        #[serde(rename = "KIND_MAP")]
        #[doc = "Map entity types allow mapping of a group of synonyms to a canonical value."]
        KindMap,
        #[serde(rename = "KIND_LIST")]
        #[doc = "List entity types contain a set of entries that do not map to canonical values. However, list entity types can contain references to other entity types (with or without aliases)."]
        KindList,
        #[serde(rename = "KIND_REGEXP")]
        #[doc = "Regexp entity types allow to specify regular expressions in entries values."]
        KindRegexp,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1EntityTypeKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An **entity entry** for an associated entity type."]
    pub struct GoogleCloudDialogflowCxV3beta1EntityTypeEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`."]
        pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A canonical value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases)."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1EntityTypeEntity {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EntityTypeEntityBuilder {
            GoogleCloudDialogflowCxV3beta1EntityTypeEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An excluded entity phrase that should not be matched."]
    pub struct GoogleCloudDialogflowCxV3beta1EntityTypeExcludedPhrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The word or phrase to be excluded."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1EntityTypeExcludedPhrase {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EntityTypeExcludedPhraseBuilder {
            GoogleCloudDialogflowCxV3beta1EntityTypeExcludedPhraseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an environment for an agent. You can create multiple versions of your agent and publish them to separate environments. When you edit an agent, you are editing the draft agent. At any point, you can save the draft agent as an agent version, which is an immutable snapshot of your agent. When you save the draft agent, it is published to the default environment. When you create agent versions, you can publish them to custom environments. You can create a variety of custom environments for testing, development, production, etc."]
    pub struct GoogleCloudDialogflowCxV3beta1Environment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable description of the environment. The maximum length is 500 characters. If exceeded, the request is rejected."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the environment (unique in an agent). Limit of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the environment. Format: `projects//locations//agents//environments/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Update time of this environment."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionConfigs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A list of configurations for flow versions. You should include version configs for all flows that are reachable from `Start Flow` in the agent. Otherwise, an error will be returned."]
        pub version_configs: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EnvironmentVersionConfig>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Environment {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EnvironmentBuilder {
            GoogleCloudDialogflowCxV3beta1EnvironmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for the version."]
    pub struct GoogleCloudDialogflowCxV3beta1EnvironmentVersionConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Format: projects//locations//agents//flows//versions/."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1EnvironmentVersionConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EnvironmentVersionConfigBuilder {
            GoogleCloudDialogflowCxV3beta1EnvironmentVersionConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An event handler specifies an event that can be handled during a session. When the specified event happens, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the event, it will be called. * If there is a `target_page` associated with the event, the session will transition into the specified page. * If there is a `target_flow` associated with the event, the session will transition into the specified flow."]
    pub struct GoogleCloudDialogflowCxV3beta1EventHandler {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the event to handle."]
        pub event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this event handler."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the event occurs. Handling webhook errors with a fulfillment enabled with webhook could cause infinite loop. It is invalid to specify such fulfillment for a handler handling webhooks."]
        pub trigger_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
    }
    impl GoogleCloudDialogflowCxV3beta1EventHandler {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EventHandlerBuilder {
            GoogleCloudDialogflowCxV3beta1EventHandlerBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the event to trigger."]
    pub struct GoogleCloudDialogflowCxV3beta1EventInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name of the event."]
        pub event: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1EventInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1EventInputBuilder {
            GoogleCloudDialogflowCxV3beta1EventInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an experiment in an environment."]
    pub struct GoogleCloudDialogflowCxV3beta1Experiment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Creation time of this experiment."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "definition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The definition of the experiment."]
        pub definition: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ExperimentDefinition>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The human-readable description of the experiment."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the experiment (unique in an environment). Limit of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "End time of this experiment."]
        pub end_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experimentLength")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Maximum number of days to run the experiment."]
        pub experiment_length: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last update time of this experiment."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the experiment. Format: projects//locations//agents//environments//experiments/.."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inference result of the experiment."]
        pub result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ExperimentResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "startTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Start time of this experiment."]
        pub start_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current state of the experiment. Transition triggered by Expriments.StartExperiment: PENDING->RUNNING. Transition triggered by Expriments.CancelExperiment: PENDING->CANCELLED or RUNNING->CANCELLED."]
        pub state: ::std::option::Option<GoogleCloudDialogflowCxV3beta1ExperimentStateEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variantsHistory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The history of updates to the experiment variants."]
        pub variants_history: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1VariantsHistory>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Experiment {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentBuilder {
            GoogleCloudDialogflowCxV3beta1ExperimentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The current state of the experiment. Transition triggered by Expriments.StartExperiment: PENDING->RUNNING. Transition triggered by Expriments.CancelExperiment: PENDING->CANCELLED or RUNNING->CANCELLED."]
    pub enum GoogleCloudDialogflowCxV3beta1ExperimentStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State unspecified."]
        StateUnspecified,
        #[serde(rename = "DRAFT")]
        #[doc = "The experiment is created but not started yet."]
        Draft,
        #[serde(rename = "RUNNING")]
        #[doc = "The experiment is running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The experiment is done."]
        Done,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1ExperimentStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Definition of the experiment."]
    pub struct GoogleCloudDialogflowCxV3beta1ExperimentDefinition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition defines which subset of sessions are selected for this experiment. If not specified, all sessions are eligible. E.g. \"query_input.language_code=en\" See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition)."]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionVariants")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The flow versions as the variants of this experiment."]
        pub version_variants:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1VersionVariants>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExperimentDefinition {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentDefinitionBuilder {
            GoogleCloudDialogflowCxV3beta1ExperimentDefinitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The inference result which includes an objective metric to optimize and the confidence interval."]
    pub struct GoogleCloudDialogflowCxV3beta1ExperimentResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastUpdateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last time the experiment's stats data was updated. Will have default value if stats have never been computed for this experiment."]
        pub last_update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionMetrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Version variants and metrics."]
        pub version_metrics: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ExperimentResultVersionMetrics>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ExperimentResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentResultBuilder {
            GoogleCloudDialogflowCxV3beta1ExperimentResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A confidence interval is a range of possible values for the experiment objective you are trying to measure."]
    pub struct GoogleCloudDialogflowCxV3beta1ExperimentResultConfidenceInterval {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidenceLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence level used to construct the interval, i.e. there is X% chance that the true value is within this interval."]
        pub confidence_level: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lowerBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Lower bound of the interval."]
        pub lower_bound: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percent change between an experiment metric's value and the value for its control."]
        pub ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "upperBound")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Upper bound of the interval."]
        pub upper_bound: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExperimentResultConfidenceInterval {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentResultConfidenceIntervalBuilder
        {
            GoogleCloudDialogflowCxV3beta1ExperimentResultConfidenceIntervalBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metric and corresponding confidence intervals."]
    pub struct GoogleCloudDialogflowCxV3beta1ExperimentResultMetric {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidenceInterval")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The probability that the treatment is better than all other treatments in the experiment"]
        pub confidence_interval: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ExperimentResultConfidenceInterval>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "count")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count value of a metric."]
        pub count: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "countType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Count-based metric type. Only one of type or count_type is specified in each Metric."]
        pub count_type: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1ExperimentResultMetricCountTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ratio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ratio value of a metric."]
        pub ratio: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ratio-based metric type. Only one of type or count_type is specified in each Metric."]
        pub _type:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1ExperimentResultMetricTypeEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExperimentResultMetric {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentResultMetricBuilder {
            GoogleCloudDialogflowCxV3beta1ExperimentResultMetricBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Count-based metric type. Only one of type or count_type is specified in each Metric."]
    pub enum GoogleCloudDialogflowCxV3beta1ExperimentResultMetricCountTypeEnum {
        #[serde(rename = "COUNT_TYPE_UNSPECIFIED")]
        #[doc = "Count type unspecified."]
        CountTypeUnspecified,
        #[serde(rename = "TOTAL_NO_MATCH_COUNT")]
        #[doc = "Total number of occurrences of a 'NO_MATCH'."]
        TotalNoMatchCount,
        #[serde(rename = "TOTAL_TURN_COUNT")]
        #[doc = "Total number of turn counts."]
        TotalTurnCount,
        #[serde(rename = "AVERAGE_TURN_COUNT")]
        #[doc = "Average turn count in a session."]
        AverageTurnCount,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1ExperimentResultMetricCountTypeEnum {
        fn default() -> Self {
            Self::CountTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Ratio-based metric type. Only one of type or count_type is specified in each Metric."]
    pub enum GoogleCloudDialogflowCxV3beta1ExperimentResultMetricTypeEnum {
        #[serde(rename = "METRIC_UNSPECIFIED")]
        #[doc = "Metric unspecified."]
        MetricUnspecified,
        #[serde(rename = "CONTAINED_SESSION_NO_CALLBACK_RATE")]
        #[doc = "Percentage of contained sessions without user calling back in 24 hours."]
        ContainedSessionNoCallbackRate,
        #[serde(rename = "LIVE_AGENT_HANDOFF_RATE")]
        #[doc = "Percentage of sessions that were handed to a human agent."]
        LiveAgentHandoffRate,
        #[serde(rename = "CALLBACK_SESSION_RATE")]
        #[doc = "Percentage of sessions with the same user calling back."]
        CallbackSessionRate,
        #[serde(rename = "ABANDONED_SESSION_RATE")]
        #[doc = "Percentage of sessions where user hung up."]
        AbandonedSessionRate,
        #[serde(rename = "SESSION_END_RATE")]
        #[doc = "Percentage of sessions reached Dialogflow 'END_PAGE' or 'END_SESSION'."]
        SessionEndRate,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1ExperimentResultMetricTypeEnum {
        fn default() -> Self {
            Self::MetricUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Version variant and associated metrics."]
    pub struct GoogleCloudDialogflowCxV3beta1ExperimentResultVersionMetrics {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metrics")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The metrics and corresponding confidence intervals in the inference result."]
        pub metrics: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ExperimentResultMetric>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of sessions that were allocated to this version."]
        pub session_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the flow Version. Format: `projects//locations//agents//flows//versions/`."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExperimentResultVersionMetrics {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExperimentResultVersionMetricsBuilder {
            GoogleCloudDialogflowCxV3beta1ExperimentResultVersionMetricsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Agents.ExportAgent."]
    pub struct GoogleCloudDialogflowCxV3beta1ExportAgentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the agent to. The format of this URI must be `gs:///`. If left unspecified, the serialized agent is returned inline."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExportAgentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExportAgentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1ExportAgentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.ExportAgent."]
    pub struct GoogleCloudDialogflowCxV3beta1ExportAgentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for agent."]
        pub agent_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in ExportAgentRequest."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExportAgentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExportAgentResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ExportAgentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.ExportTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3beta1ExportTestCasesMetadata {}
    impl GoogleCloudDialogflowCxV3beta1ExportTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExportTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1ExportTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for TestCases.ExportTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1ExportTestCasesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataFormat")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The data format of the exported test cases. If not specified, `BLOB` is assumed."]
        pub data_format: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1ExportTestCasesRequestDataFormatEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The filter expression used to filter exported test cases, see [API Filtering](https://aip.dev/160). The expression is case insensitive and supports the following syntax: name = [OR name = ] ... For example: * \"name = t1 OR name = t2\" matches the test case with the exact resource name \"t1\" or \"t2\"."]
        pub filter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to export the test cases to. The format of this URI must be `gs:///`. If unspecified, the serialized test cases is returned inline."]
        pub gcs_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExportTestCasesRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExportTestCasesRequestBuilder {
            GoogleCloudDialogflowCxV3beta1ExportTestCasesRequestBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The data format of the exported test cases. If not specified, `BLOB` is assumed."]
    pub enum GoogleCloudDialogflowCxV3beta1ExportTestCasesRequestDataFormatEnum {
        #[serde(rename = "DATA_FORMAT_UNSPECIFIED")]
        #[doc = "Unspecified format."]
        DataFormatUnspecified,
        #[serde(rename = "BLOB")]
        #[doc = "Raw bytes."]
        Blob,
        #[serde(rename = "JSON")]
        #[doc = "JSON format."]
        Json,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1ExportTestCasesRequestDataFormatEnum
    {
        fn default() -> Self {
            Self::DataFormatUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ExportTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1ExportTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for test cases."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported test cases. This field is populated only if `gcs_uri` is specified in ExportTestCasesRequest."]
        pub gcs_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ExportTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ExportTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ExportTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Flows represents the conversation flows when you build your chatbot agent. A flow consists of many pages connected by the transition routes. Conversations always start with the built-in Start Flow (with an all-0 ID). Transition routes can direct the conversation session from the current flow (parent flow) to another flow (sub flow). When the sub flow is finished, Dialogflow will bring the session back to the parent flow, where the sub flow is started. Usually, when a transition route is followed by a matched intent, the intent will be \"consumed\". This means the intent won't activate more transition routes. However, when the followed transition route moves the conversation session into a different flow, the matched intent can be carried over and to be consumed in the target flow."]
    pub struct GoogleCloudDialogflowCxV3beta1Flow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the flow. The maximum length is 500 characters. If exceeded, the request is rejected."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the flow."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A flow's event handlers serve two purposes: * They are responsible for handling events (e.g. no match, webhook errors) in the flow. * They are inherited by every page's event handlers, which can be used to handle common events regardless of the current page. Event handlers defined in the page have higher priority than those defined in the flow. Unlike transition_routes, these handlers are evaluated on a first-match basis. The first one that matches the event get executed, with the rest being ignored."]
        pub event_handlers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the flow. Format: `projects//locations//agents//flows/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nluSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "NLU related settings of the flow."]
        pub nlu_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1NluSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A flow's transition routes serve two purposes: * They are responsible for matching the user's first utterances in the flow. * They are inherited by every page's transition routes and can support use cases such as the user saying \"help\" or \"can I talk to a human?\", which can be handled in a common way regardless of the current page. Transition routes defined in the page have higher priority than those defined in the flow. TransitionRoutes are evalauted in the following order: * TransitionRoutes with intent specified.. * TransitionRoutes with only condition specified. TransitionRoutes with intent specified are inherited by pages in the flow."]
        pub transition_routes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Flow {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FlowBuilder {
            GoogleCloudDialogflowCxV3beta1FlowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Flows.GetFlowValidationResult."]
    pub struct GoogleCloudDialogflowCxV3beta1FlowValidationResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the flow validation result. Format: `projects//locations//agents//flows//validationResult`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last time the flow was validated."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "validationMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Contains all validation messages."]
        pub validation_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ValidationMessage>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1FlowValidationResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FlowValidationResultBuilder {
            GoogleCloudDialogflowCxV3beta1FlowValidationResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A form is a data model that groups related parameters that can be collected from the user. The process in which the agent prompts the user and collects parameter values from the user is called form filling. A form can be added to a page. When form filling is done, the filled parameters will be written to the session."]
    pub struct GoogleCloudDialogflowCxV3beta1Form {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters to collect from the user."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FormParameter>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Form {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FormBuilder {
            GoogleCloudDialogflowCxV3beta1FormBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a form parameter."]
    pub struct GoogleCloudDialogflowCxV3beta1FormParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The default value of an optional parameter. If the parameter is required, the default value will be ignored."]
        pub default_value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the parameter, unique within the form."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fillBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Defines fill behavior for the parameter."]
        pub fill_behavior: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FormParameterFillBehavior>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
        pub redact: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "required")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
        pub required: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1FormParameter {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FormParameterBuilder {
            GoogleCloudDialogflowCxV3beta1FormParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration for how the filling of a parameter should be handled."]
    pub struct GoogleCloudDialogflowCxV3beta1FormParameterFillBehavior {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "initialPromptFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The fulfillment to provide the initial prompt that the agent can present to the user in order to fill the parameter."]
        pub initial_prompt_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repromptEventHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The handlers for parameter-level events, used to provide reprompt for the parameter or transition to a different page/flow. The supported events are: * `sys.no-match-`, where N can be from 1 to 6 * `sys.no-match-default` * `sys.no-input-`, where N can be from 1 to 6 * `sys.no-input-default` * `sys.invalid-parameter` `initial_prompt_fulfillment` provides the first prompt for the parameter. If the user's response does not fill the parameter, a no-match/no-input event will be triggered, and the fulfillment associated with the `sys.no-match-1`/`sys.no-input-1` handler (if defined) will be called to provide a prompt. The `sys.no-match-2`/`sys.no-input-2` handler (if defined) will respond to the next no-match/no-input event, and so on. A `sys.no-match-default` or `sys.no-input-default` handler will be used to handle all following no-match/no-input events after all numbered no-match/no-input handlers for the parameter are consumed. A `sys.invalid-parameter` handler can be defined to handle the case where the parameter values have been `invalidated` by webhook. For example, if the user's response fill the parameter, however the parameter was invalidated by webhook, the fulfillment associated with the `sys.invalid-parameter` handler (if defined) will be called to provide a prompt. If the event handler for the corresponding event can't be found on the parameter, `initial_prompt_fulfillment` will be re-prompted."]
        pub reprompt_event_handlers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1FormParameterFillBehavior {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FormParameterFillBehaviorBuilder {
            GoogleCloudDialogflowCxV3beta1FormParameterFillBehaviorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request of FulfillIntent"]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillIntentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "match")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The matched intent/event to fulfill."]
        pub _match: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Match>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchIntentRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Must be same as the corresponding MatchIntent request, otherwise the behavior is undefined."]
        pub match_intent_request: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1MatchIntentRequest>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Instructs the speech synthesizer how to generate output audio."]
        pub output_audio_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1OutputAudioConfig>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillIntentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillIntentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillIntentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of FulfillIntent"]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillIntentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The audio data bytes encoded as specified in the request. Note: The output audio is generated based on the values of default platform text responses found in the `query_result.response_messages` field. If multiple default text responses exist, they will be concatenated when generating audio. If no default platform text responses exist, the generated audio content will be empty. In some scenarios, multiple output audio fields may be present in the response structure. In these cases, only the top-most-level audio output has content."]
        pub output_audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The config used by the speech synthesizer to generate the output audio."]
        pub output_audio_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1OutputAudioConfig>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the conversational query."]
        pub query_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the response. It can be used to locate a response in the training example set or for reporting issues."]
        pub response_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillIntentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillIntentResponseBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillIntentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A fulfillment can do one or more of the following actions at the same time: * Generate rich message responses. * Set parameter values. * Call the webhook. Fulfillments can be called at various stages in the Page or Form lifecycle. For example, when a DetectIntentRequest drives a session to enter a new page, the page's entry fulfillment can add a static response to the QueryResult in the returning DetectIntentResponse, call the webhook (for example, to load user data from a database), or both."]
    pub struct GoogleCloudDialogflowCxV3beta1Fulfillment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conditionalCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Conditional cases for this fulfillment."]
        pub conditional_cases: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "setParameterActions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Set parameter values before executing the webhook."]
        pub set_parameter_actions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterAction>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The tag used by the webhook to identify which fulfillment is being called. This field is required if `webhook` is specified."]
        pub tag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhook")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The webhook to call. Format: `projects//locations//agents//webhooks/`."]
        pub webhook: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1Fulfillment {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillmentBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of cascading if-else conditions. Cases are mutually exclusive. The first one with a matching condition is selected, all the rest ignored."]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of cascading if-else conditions."]
        pub cases: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCase>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Each case has a Boolean condition. When it is evaluated to be True, the corresponding messages will be selected and evaluated recursively."]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "caseContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of case content."]
        pub case_content: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContent,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition to activate and select this case. Empty means the condition is always true. The condition is evaluated against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition)."]
        pub condition: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCase {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The list of messages or conditional cases to activate for this case."]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "additionalCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional cases to be evaluated."]
        pub additional_cases: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCases>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "message")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returned message."]
        pub message:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContent {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContentBuilder
        {
            GoogleCloudDialogflowCxV3beta1FulfillmentConditionalCasesCaseCaseContentBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Setting a parameter value."]
    pub struct GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name of the parameter."]
        pub parameter: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The new value of the parameter. A null value clears the parameter."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterAction {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterActionBuilder {
            GoogleCloudDialogflowCxV3beta1FulfillmentSetParameterActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
    pub struct GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Output only. The current state of this operation."]
        pub state: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataStateEnum,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Output only. The current state of this operation."]
    pub enum GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadataStateEnum
    {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ImportDocuments operation."]
    pub struct GoogleCloudDialogflowCxV3beta1ImportDocumentsOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ImportDocumentsOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ImportDocumentsOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1ImportDocumentsOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Documents.ImportDocuments."]
    pub struct GoogleCloudDialogflowCxV3beta1ImportDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Includes details about skipped documents or any other warnings."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ImportDocumentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ImportDocumentsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ImportDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.ImportTestCases long running operation."]
    pub struct GoogleCloudDialogflowCxV3beta1ImportTestCasesMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errors")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Errors for failed test cases."]
        pub errors: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseError>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ImportTestCasesMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ImportTestCasesMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1ImportTestCasesMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for TestCases.ImportTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1ImportTestCasesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for test cases."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to import test cases from. The format of this URI must be `gs:///`."]
        pub gcs_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ImportTestCasesRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ImportTestCasesRequestBuilder {
            GoogleCloudDialogflowCxV3beta1ImportTestCasesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ImportTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1ImportTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "names")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifiers of the new test cases. Format: `projects//locations//agents//testCases/`."]
        pub names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ImportTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ImportTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ImportTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instructs the speech recognizer on how to process the audio content."]
    pub struct GoogleCloudDialogflowCxV3beta1InputAudioConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Audio encoding of the audio content to process."]
        pub audio_encoding:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1InputAudioConfigAudioEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableWordInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `true`, Dialogflow returns SpeechWordInfo in StreamingRecognitionResult with information about the recognized speech words, e.g. start and end time offsets. If false or unspecified, Speech doesn't return any word-level information."]
        pub enable_word_info: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "model")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Which Speech model to select for the given request. Select the model best suited to your domain to get best results. If a model is not explicitly specified, then we auto-select a model based on the parameters in the InputAudioConfig. If enhanced speech model is enabled for the agent and an enhanced version of the specified model for the language does not exist, then the speech is recognized using the standard version of the specified model. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model) for more details."]
        pub model: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelVariant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Which variant of the Speech model to use."]
        pub model_variant:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1InputAudioConfigModelVariantEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phraseHints")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of strings containing words and phrases that the speech recognizer should recognize with higher likelihood. See [the Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints) for more details."]
        pub phrase_hints: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sample rate (in Hertz) of the audio content sent in the query. Refer to [Cloud Speech API documentation](https://cloud.google.com/speech-to-text/docs/basics) for more details."]
        pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "singleUtterance")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. If `false` (default), recognition does not cease until the client closes the stream. If `true`, the recognizer will detect a single spoken utterance in input audio. Recognition ceases when it detects the audio's voice has stopped or paused. In this case, once a detected intent is received, the client should close the stream and start a new request with a new stream as needed. Note: This setting is relevant only for streaming methods."]
        pub single_utterance: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1InputAudioConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1InputAudioConfigBuilder {
            GoogleCloudDialogflowCxV3beta1InputAudioConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Audio encoding of the audio content to process."]
    pub enum GoogleCloudDialogflowCxV3beta1InputAudioConfigAudioEncodingEnum {
        #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
        #[doc = "Not specified."]
        AudioEncodingUnspecified,
        #[serde(rename = "AUDIO_ENCODING_LINEAR_16")]
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM)."]
        AudioEncodingLinear16,
        #[serde(rename = "AUDIO_ENCODING_FLAC")]
        #[doc = "[`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio Codec) is the recommended encoding because it is lossless (therefore recognition is not compromised) and requires only about half the bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are supported."]
        AudioEncodingFlac,
        #[serde(rename = "AUDIO_ENCODING_MULAW")]
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
        AudioEncodingMulaw,
        #[serde(rename = "AUDIO_ENCODING_AMR")]
        #[doc = "Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000."]
        AudioEncodingAmr,
        #[serde(rename = "AUDIO_ENCODING_AMR_WB")]
        #[doc = "Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000."]
        AudioEncodingAmrWb,
        #[serde(rename = "AUDIO_ENCODING_OGG_OPUS")]
        #[doc = "Opus encoded audio frames in Ogg container ([OggOpus](https://wiki.xiph.org/OggOpus)). `sample_rate_hertz` must be 16000."]
        AudioEncodingOggOpus,
        #[serde(rename = "AUDIO_ENCODING_SPEEX_WITH_HEADER_BYTE")]
        #[doc = "Although the use of lossy encodings is not recommended, if a very low bitrate encoding is required, `OGG_OPUS` is highly preferred over Speex encoding. The [Speex](https://speex.org/) encoding supported by Dialogflow API has a header byte in each block, as in MIME type `audio/x-speex-with-header-byte`. It is a variant of the RTP Speex encoding defined in [RFC 5574](https://tools.ietf.org/html/rfc5574). The stream is a sequence of blocks, one block per RTP packet. Each block starts with a byte containing the length of the block, in bytes, followed by one or more frames of Speex data, padded to an integral number of bytes (octets) as specified in RFC 5574. In other words, each RTP header is replaced with a single byte containing the block length. Only Speex wideband is supported. `sample_rate_hertz` must be 16000."]
        AudioEncodingSpeexWithHeaderByte,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1InputAudioConfigAudioEncodingEnum {
        fn default() -> Self {
            Self::AudioEncodingUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Which variant of the Speech model to use."]
    pub enum GoogleCloudDialogflowCxV3beta1InputAudioConfigModelVariantEnum {
        #[serde(rename = "SPEECH_MODEL_VARIANT_UNSPECIFIED")]
        #[doc = "No model variant specified. In this case Dialogflow defaults to USE_BEST_AVAILABLE."]
        SpeechModelVariantUnspecified,
        #[serde(rename = "USE_BEST_AVAILABLE")]
        #[doc = "Use the best available variant of the Speech model that the caller is eligible for. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible for enhanced models."]
        UseBestAvailable,
        #[serde(rename = "USE_STANDARD")]
        #[doc = "Use standard model variant even if an enhanced model is available. See the [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) for details about enhanced models."]
        UseStandard,
        #[serde(rename = "USE_ENHANCED")]
        #[doc = "Use an enhanced model variant: * If an enhanced variant does not exist for the given model and request language, Dialogflow falls back to the standard variant. The [Cloud Speech documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models) describes which models have enhanced variants. * If the API caller isn't eligible for enhanced models, Dialogflow returns an error. Please see the [Dialogflow docs](https://cloud.google.com/dialogflow/docs/data-logging) for how to make your project eligible."]
        UseEnhanced,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1InputAudioConfigModelVariantEnum {
        fn default() -> Self {
            Self::SpeechModelVariantUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An intent represents a user's intent to interact with a conversational agent. You can provide information for the Dialogflow API to use to match user input to an intent by adding training phrases (i.e., examples of user input) to your intent."]
    pub struct GoogleCloudDialogflowCxV3beta1Intent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Human readable description for better understanding an intent like its scope, content, result etc. Maximum character limit: 140 characters."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the intent, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFallback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether this is a fallback intent. Currently only default fallback intent is allowed in the agent, which is added upon agent creation. Adding training phrases to fallback intent is useful in the case of requests that are mistakenly matched, since training phrases assigned to fallback intents act as negative examples that triggers no-match event."]
        pub is_fallback: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "labels")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The key/value metadata to label an intent. Labels can contain lowercase letters, digits and the symbols '-' and '_'. International characters are allowed, including letters from unicase alphabets. Keys must start with a letter. Keys and values can be no longer than 63 characters and no more than 128 bytes. Prefix \"sys-\" is reserved for Dialogflow defined labels. Currently allowed Dialogflow defined labels include: * sys-head * sys-contextual The above labels do not require value. \"sys-head\" means the intent is a head intent. \"sys-contextual\" means the intent is a contextual intent."]
        pub labels:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the intent. Required for the Intents.UpdateIntent method. Intents.CreateIntent populates the name automatically. Format: `projects//locations//agents//intents/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of parameters associated with the intent."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentParameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of training phrases the agent is trained on to identify the intent."]
        pub training_phrases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentTrainingPhrase>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Intent {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentBuilder {
            GoogleCloudDialogflowCxV3beta1IntentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Intent coverage represents the percentage of all possible intents in the agent that are triggered in any of a parent's test cases."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentCoverage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverageScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percent of intents in the agent that are covered."]
        pub coverage_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Intents present in the agent"]
        pub intents: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentCoverageIntent>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentCoverage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentCoverageBuilder {
            GoogleCloudDialogflowCxV3beta1IntentCoverageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The agent's intent."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentCoverageIntent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "covered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the intent is covered by at least one of the agent's test cases."]
        pub covered: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent full resource name"]
        pub intent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentCoverageIntent {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentCoverageIntentBuilder {
            GoogleCloudDialogflowCxV3beta1IntentCoverageIntentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the intent to trigger programmatically rather than as a result of natural language processing."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the intent. Format: `projects//locations//agents//intents/`."]
        pub intent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentInputBuilder {
            GoogleCloudDialogflowCxV3beta1IntentInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an intent parameter."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The entity type of the parameter. Format: `projects/-/locations/-/agents/-/entityTypes/` for system entity types (for example, `projects/-/locations/-/agents/-/entityTypes/sys.date`), or `projects//locations//agents//entityTypes/` for developer entity types."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the parameter. This field is used by training phrases to annotate their parts."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redact")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the parameter content should be redacted in log. If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled."]
        pub redact: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentParameter {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentParameterBuilder {
            GoogleCloudDialogflowCxV3beta1IntentParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an example that the agent is trained on to identify the intent."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentTrainingPhrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "id")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the training phrase."]
        pub id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `parameter_id` field is set."]
        pub parts: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePart>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "repeatCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates how many times this example was added to the intent."]
        pub repeat_count: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentTrainingPhrase {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentTrainingPhraseBuilder {
            GoogleCloudDialogflowCxV3beta1IntentTrainingPhraseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a part of a training phrase."]
    pub struct GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameter used to annotate this part of the training phrase. This field is required for annotated parts of the training phrase."]
        pub parameter_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text for this part."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePart {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePartBuilder {
            GoogleCloudDialogflowCxV3beta1IntentTrainingPhrasePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.ListAgents."]
    pub struct GoogleCloudDialogflowCxV3beta1ListAgentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of agents. There will be a maximum number of items returned based on the page_size field in the request."]
        pub agents: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Agent>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListAgentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListAgentsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListAgentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for EntityTypes.ListEntityTypes."]
    pub struct GoogleCloudDialogflowCxV3beta1ListEntityTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of entity types. There will be a maximum number of items returned based on the page_size field in the request."]
        pub entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EntityType>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListEntityTypesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListEntityTypesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListEntityTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Environments.ListEnvironments."]
    pub struct GoogleCloudDialogflowCxV3beta1ListEnvironmentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of environments. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page."]
        pub environments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Environment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListEnvironmentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListEnvironmentsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListEnvironmentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Experiments.ListExperiments."]
    pub struct GoogleCloudDialogflowCxV3beta1ListExperimentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "experiments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of experiments. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page."]
        pub experiments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Experiment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListExperimentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListExperimentsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListExperimentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Flows.ListFlows."]
    pub struct GoogleCloudDialogflowCxV3beta1ListFlowsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of flows. There will be a maximum number of items returned based on the page_size field in the request."]
        pub flows: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Flow>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListFlowsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListFlowsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListFlowsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Intents.ListIntents."]
    pub struct GoogleCloudDialogflowCxV3beta1ListIntentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of intents. There will be a maximum number of items returned based on the page_size field in the request."]
        pub intents: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Intent>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ListIntentsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListIntentsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListIntentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Pages.ListPages."]
    pub struct GoogleCloudDialogflowCxV3beta1ListPagesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of pages. There will be a maximum number of items returned based on the page_size field in the request."]
        pub pages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListPagesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListPagesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListPagesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for SecuritySettings.ListSecuritySettings."]
    pub struct GoogleCloudDialogflowCxV3beta1ListSecuritySettingsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "securitySettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of security settings."]
        pub security_settings: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SecuritySettings>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListSecuritySettingsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListSecuritySettingsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListSecuritySettingsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for SessionEntityTypes.ListSessionEntityTypes."]
    pub struct GoogleCloudDialogflowCxV3beta1ListSessionEntityTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionEntityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of session entity types. There will be a maximum number of items returned based on the page_size field in the request."]
        pub session_entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionEntityType>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListSessionEntityTypesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListSessionEntityTypesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListSessionEntityTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ListTestCaseResults."]
    pub struct GoogleCloudDialogflowCxV3beta1ListTestCaseResultsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCaseResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of test case results."]
        pub test_case_results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListTestCaseResultsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListTestCaseResultsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListTestCaseResultsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.ListTestCases."]
    pub struct GoogleCloudDialogflowCxV3beta1ListTestCasesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of test cases. There will be a maximum number of items returned based on the page_size field in the request."]
        pub test_cases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCase>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListTestCasesResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListTestCasesResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListTestCasesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TransitionRouteGroups.ListTransitionRouteGroups."]
    pub struct GoogleCloudDialogflowCxV3beta1ListTransitionRouteGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRouteGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of transition route groups. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page."]
        pub transition_route_groups: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRouteGroup>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListTransitionRouteGroupsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListTransitionRouteGroupsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListTransitionRouteGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Versions.ListVersions."]
    pub struct GoogleCloudDialogflowCxV3beta1ListVersionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of versions. There will be a maximum number of items returned based on the page_size field in the request. The list may in some cases be empty or contain fewer entries than page_size even if this isn't the last page."]
        pub versions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Version>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListVersionsResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListVersionsResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListVersionsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Webhooks.ListWebhooks."]
    pub struct GoogleCloudDialogflowCxV3beta1ListWebhooksResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhooks")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of webhooks. There will be a maximum number of items returned based on the page_size field in the request."]
        pub webhooks: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Webhook>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ListWebhooksResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ListWebhooksResponseBuilder {
            GoogleCloudDialogflowCxV3beta1ListWebhooksResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Versions.LoadVersion."]
    pub struct GoogleCloudDialogflowCxV3beta1LoadVersionRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowOverrideAgentResources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is used to prevent accidental overwrite of other agent resources in the draft version, which can potentially impact other flow's behavior. If `allow_override_agent_resources` is false, conflicted agent-level resources will not be overridden (i.e. intents, entities, webhooks)."]
        pub allow_override_agent_resources: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1LoadVersionRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1LoadVersionRequestBuilder {
            GoogleCloudDialogflowCxV3beta1LoadVersionRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Environments.LookupEnvironmentHistory."]
    pub struct GoogleCloudDialogflowCxV3beta1LookupEnvironmentHistoryResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents a list of snapshots for an environment. Time of the snapshots is stored in `update_time`."]
        pub environments: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Environment>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1LookupEnvironmentHistoryResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1LookupEnvironmentHistoryResponseBuilder {
            GoogleCloudDialogflowCxV3beta1LookupEnvironmentHistoryResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents one match result of MatchIntent."]
    pub struct GoogleCloudDialogflowCxV3beta1Match {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence of this match. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The event that matched the query. Only filled for `EVENT` match type."]
        pub event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Intent that matched the query. Some, not all fields are filled in this message, including but not limited to: `name` and `display_name`. Only filled for `INTENT` match type."]
        pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Intent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of this Match."]
        pub match_type: ::std::option::Option<GoogleCloudDialogflowCxV3beta1MatchMatchTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of parameters extracted from the query. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Final text input which was matched during MatchIntent. This value can be different from original input sent in request because of spelling correction or other processing."]
        pub resolved_input: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1Match {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1MatchBuilder {
            GoogleCloudDialogflowCxV3beta1MatchBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of this Match."]
    pub enum GoogleCloudDialogflowCxV3beta1MatchMatchTypeEnum {
        #[serde(rename = "MATCH_TYPE_UNSPECIFIED")]
        #[doc = "Not specified. Should never be used."]
        MatchTypeUnspecified,
        #[serde(rename = "INTENT")]
        #[doc = "The query was matched to an intent."]
        Intent,
        #[serde(rename = "DIRECT_INTENT")]
        #[doc = "The query directly triggered an intent."]
        DirectIntent,
        #[serde(rename = "PARAMETER_FILLING")]
        #[doc = "The query was used for parameter filling."]
        ParameterFilling,
        #[serde(rename = "NO_MATCH")]
        #[doc = "No match was found for the query."]
        NoMatch,
        #[serde(rename = "NO_INPUT")]
        #[doc = "Indicates an empty query."]
        NoInput,
        #[serde(rename = "EVENT")]
        #[doc = "The query directly triggered an event."]
        Event,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1MatchMatchTypeEnum {
        fn default() -> Self {
            Self::MatchTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request of MatchIntent."]
    pub struct GoogleCloudDialogflowCxV3beta1MatchIntentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The input specification."]
        pub query_input:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryParams")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The parameters of this query."]
        pub query_params:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1QueryParameters>>,
    }
    impl GoogleCloudDialogflowCxV3beta1MatchIntentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1MatchIntentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1MatchIntentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response of MatchIntent."]
    pub struct GoogleCloudDialogflowCxV3beta1MatchIntentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current Page. Some, not all fields are filled in this message, including but not limited to `name` and `display_name`."]
        pub current_page:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matches")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Match results, if more than one, ordered descendingly by the confidence we have that the particular intent matches the query."]
        pub matches: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Match>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If natural language text was provided as input, this field will contain a copy of the text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If natural language speech audio was provided as input, this field will contain the trascript for the audio."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an event was provided as input, this field will contain a copy of the event name."]
        pub trigger_event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an intent was provided as input, this field will contain a copy of the intent identifier."]
        pub trigger_intent: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1MatchIntentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1MatchIntentResponseBuilder {
            GoogleCloudDialogflowCxV3beta1MatchIntentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings related to NLU."]
    pub struct GoogleCloudDialogflowCxV3beta1NluSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "classificationThreshold")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "To filter out false positive results and still get variety in matched natural language inputs for your agent, you can tune the machine learning classification threshold. If the returned score value is less than the threshold value, then a no-match event will be triggered. The score values range from 0.0 (completely uncertain) to 1.0 (completely certain). If set to 0.0, the default of 0.3 is used."]
        pub classification_threshold: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelTrainingMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates NLU model training mode."]
        pub model_training_mode:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1NluSettingsModelTrainingModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modelType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the type of NLU model."]
        pub model_type:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1NluSettingsModelTypeEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1NluSettings {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1NluSettingsBuilder {
            GoogleCloudDialogflowCxV3beta1NluSettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates NLU model training mode."]
    pub enum GoogleCloudDialogflowCxV3beta1NluSettingsModelTrainingModeEnum {
        #[serde(rename = "MODEL_TRAINING_MODE_UNSPECIFIED")]
        #[doc = "Not specified. `MODEL_TRAINING_MODE_AUTOMATIC` will be used."]
        ModelTrainingModeUnspecified,
        #[serde(rename = "MODEL_TRAINING_MODE_AUTOMATIC")]
        #[doc = "NLU model training is automatically triggered when a flow gets modified. User can also manually trigger model training in this mode."]
        ModelTrainingModeAutomatic,
        #[serde(rename = "MODEL_TRAINING_MODE_MANUAL")]
        #[doc = "User needs to manually trigger NLU model training. Best for large flows whose models take long time to train."]
        ModelTrainingModeManual,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1NluSettingsModelTrainingModeEnum {
        fn default() -> Self {
            Self::ModelTrainingModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the type of NLU model."]
    pub enum GoogleCloudDialogflowCxV3beta1NluSettingsModelTypeEnum {
        #[serde(rename = "MODEL_TYPE_UNSPECIFIED")]
        #[doc = "Not specified. `MODEL_TYPE_STANDARD` will be used."]
        ModelTypeUnspecified,
        #[serde(rename = "MODEL_TYPE_STANDARD")]
        #[doc = "Use standard NLU model."]
        ModelTypeStandard,
        #[serde(rename = "MODEL_TYPE_ADVANCED")]
        #[doc = "Use advanced NLU model."]
        ModelTypeAdvanced,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1NluSettingsModelTypeEnum {
        fn default() -> Self {
            Self::ModelTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Instructs the speech synthesizer how to generate the output audio content."]
    pub struct GoogleCloudDialogflowCxV3beta1OutputAudioConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioEncoding")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Audio encoding of the synthesized audio content."]
        pub audio_encoding:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1OutputAudioConfigAudioEncodingEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleRateHertz")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The synthesis sample rate (in hertz) for this audio. If not provided, then the synthesizer will use the default sample rate based on the audio encoding. If this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality)."]
        pub sample_rate_hertz: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synthesizeSpeechConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Configuration of how speech should be synthesized."]
        pub synthesize_speech_config: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SynthesizeSpeechConfig>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1OutputAudioConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1OutputAudioConfigBuilder {
            GoogleCloudDialogflowCxV3beta1OutputAudioConfigBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Audio encoding of the synthesized audio content."]
    pub enum GoogleCloudDialogflowCxV3beta1OutputAudioConfigAudioEncodingEnum {
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_UNSPECIFIED")]
        #[doc = "Not specified."]
        OutputAudioEncodingUnspecified,
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_LINEAR_16")]
        #[doc = "Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header."]
        OutputAudioEncodingLinear16,
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_MP3")]
        #[doc = "MP3 audio at 32kbps."]
        OutputAudioEncodingMp3,
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_MP3_64_KBPS")]
        #[doc = "MP3 audio at 64kbps."]
        OutputAudioEncodingMp364Kbps,
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_OGG_OPUS")]
        #[doc = "Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate."]
        OutputAudioEncodingOggOpus,
        #[serde(rename = "OUTPUT_AUDIO_ENCODING_MULAW")]
        #[doc = "8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law."]
        OutputAudioEncodingMulaw,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1OutputAudioConfigAudioEncodingEnum {
        fn default() -> Self {
            Self::OutputAudioEncodingUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A Dialogflow CX conversation (session) can be described and visualized as a state machine. The states of a CX session are represented by pages. For each flow, you define many pages, where your combined pages can handle a complete conversation on the topics the flow is designed for. At any given moment, exactly one page is the current page, the current page is considered active, and the flow associated with that page is considered active. Every flow has a special start page. When a flow initially becomes active, the start page page becomes the current page. For each conversational turn, the current page will either stay the same or transition to another page. You configure each page to collect information from the end-user that is relevant for the conversational state represented by the page. For more information, see the [Page guide](https://cloud.google.com/dialogflow/cx/docs/concept/page)."]
    pub struct GoogleCloudDialogflowCxV3beta1Page {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the page, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the session is entering the page."]
        pub entry_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventHandlers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Handlers associated with the page to handle events such as webhook errors, no match or no input."]
        pub event_handlers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "form")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The form associated with the page, used for collecting parameters relevant to the page."]
        pub form: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Form>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the page. Required for the Pages.UpdatePage method. Pages.CreatePage populates the name automatically. Format: `projects//locations//agents//flows//pages/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRouteGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Ordered list of `TransitionRouteGroups` associated with the page. Transition route groups must be unique within a page. * If multiple transition routes within a page scope refer to the same intent, then the precedence order is: page's transition route -> page's transition route group -> flow's transition routes. * If multiple transition route groups within a page contain the same intent, then the first group in the ordered list takes precedence. Format:`projects//locations//agents//flows//transitionRouteGroups/`."]
        pub transition_route_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of transitions for the transition rules of this page. They route the conversation to another page in the same flow, or another flow. When we are in a certain page, the TransitionRoutes are evalauted in the following order: * TransitionRoutes defined in the page with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in flow with intent specified. * TransitionRoutes defined in the transition route groups with intent specified. * TransitionRoutes defined in the page with only condition specified. * TransitionRoutes defined in the transition route groups with only condition specified."]
        pub transition_routes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1Page {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1PageBuilder {
            GoogleCloudDialogflowCxV3beta1PageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents page information communicated to and from the webhook."]
    pub struct GoogleCloudDialogflowCxV3beta1PageInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the current page. Format: `projects//locations//agents//flows//pages/`."]
        pub current_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. Information about the form."]
        pub form_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfoFormInfo>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1PageInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1PageInfoBuilder {
            GoogleCloudDialogflowCxV3beta1PageInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents form information."]
    pub struct GoogleCloudDialogflowCxV3beta1PageInfoFormInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameterInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. The parameters contained in the form. Note that the webhook cannot add or remove any form parameter."]
        pub parameter_info: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfo>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1PageInfoFormInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1PageInfoFormInfoBuilder {
            GoogleCloudDialogflowCxV3beta1PageInfoFormInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents parameter information."]
    pub struct GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The human-readable name of the parameter, unique within the form. This field cannot be modified by the webhook."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "justCollected")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for WebhookRequest. Ignored for WebhookResponse. Indicates if the parameter value was just collected on the last conversation turn."]
        pub just_collected: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "required")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them. Required parameters must be filled before form filling concludes."]
        pub required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
        pub state: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoStateEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for both WebhookRequest and WebhookResponse. The value of the parameter. This field can be set by the webhook to change the parameter value."]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoBuilder {
            GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Always present for WebhookRequest. Required for WebhookResponse. The state of the parameter. This field can be set to INVALID by the webhook to invalidate the parameter; other values set by the webhook will be ignored."]
    pub enum GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoStateEnum {
        #[serde(rename = "PARAMETER_STATE_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        ParameterStateUnspecified,
        #[serde(rename = "EMPTY")]
        #[doc = "Indicates that the parameter does not have a value."]
        Empty,
        #[serde(rename = "INVALID")]
        #[doc = "Indicates that the parameter value is invalid. This field can be used by the webhook to invalidate the parameter and ask the server to collect it from the user again."]
        Invalid,
        #[serde(rename = "FILLED")]
        #[doc = "Indicates that the parameter has a value."]
        Filled,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1PageInfoFormInfoParameterInfoStateEnum
    {
        fn default() -> Self {
            Self::ParameterStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the query input. It can contain one of: 1. A conversational query in the form of text. 2. An intent query that specifies which intent to trigger. 3. Natural language speech audio to be processed. 4. An event to be triggered. "]
    pub struct GoogleCloudDialogflowCxV3beta1QueryInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language speech audio to be processed."]
        pub audio:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1AudioInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dtmf")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The DTMF event to be handled."]
        pub dtmf: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1DtmfInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "event")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The event to be triggered."]
        pub event:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent to be triggered."]
        pub intent:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1IntentInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language of the input. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The natural language text to be processed."]
        pub text: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TextInput>>,
    }
    impl GoogleCloudDialogflowCxV3beta1QueryInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1QueryInputBuilder {
            GoogleCloudDialogflowCxV3beta1QueryInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the parameters of a conversational query."]
    pub struct GoogleCloudDialogflowCxV3beta1QueryParameters {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "analyzeQueryTextSentiment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configures whether sentiment analysis should be performed. If not provided, sentiment analysis is not performed."]
        pub analyze_query_text_sentiment: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "geoLocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The geo location of this conversational query."]
        pub geo_location: ::std::option::Option<::std::boxed::Box<GoogleTypeLatLng>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional parameters to be put into session parameters. To remove a parameter from the session, clients should explicitly set the parameter value to null. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field can be used to pass custom data into the webhook associated with the agent. Arbitrary JSON objects are supported."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionEntityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session of this query."]
        pub session_entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionEntityType>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeZone")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time zone of this conversational query from the [time zone database](https://www.iana.org/time-zones), e.g., America/New_York, Europe/Paris. If not provided, the time zone specified in the agent is used."]
        pub time_zone: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field can be used to pass HTTP headers for a webhook call. These headers will be sent to webhook along with the headers that have been configured through Dialogflow web console. The headers defined within this field will overwrite the headers configured through Dialogflow console if there is a conflict. Header names are case-insensitive. Google's specified headers are not allowed. Including: \"Host\", \"Content-Length\", \"Connection\", \"From\", \"User-Agent\", \"Accept-Encoding\", \"If-Modified-Since\", \"If-None-Match\", \"X-Forwarded-For\", etc."]
        pub webhook_headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1QueryParameters {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1QueryParametersBuilder {
            GoogleCloudDialogflowCxV3beta1QueryParametersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of a conversational query."]
    pub struct GoogleCloudDialogflowCxV3beta1QueryResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "currentPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The current Page. Some, not all fields are filled in this message, including but not limited to `name` and `display_name`."]
        pub current_page:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnosticInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The free-form diagnostic info. For example, this field could contain webhook call latency. The string keys of the Struct's fields map can change without notice."]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name` and `display_name`. This field is deprecated, please use QueryResult.match instead."]
        pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Intent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentDetectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. This field is deprecated, please use QueryResult.match instead."]
        pub intent_detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "match")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intent match result, could be an intent or an event."]
        pub _match: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Match>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collected session parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich messages returned to the client. Responses vary from simple text messages to more sophisticated, structured payloads used to drive complex logic."]
        pub response_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentAnalysisResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analyss result, which depends on `analyze_query_text_sentiment`, specified in the request."]
        pub sentiment_analysis_result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SentimentAnalysisResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If natural language text was provided as input, this field will contain a copy of the text."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transcript")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If natural language speech audio was provided as input, this field will contain the trascript for the audio."]
        pub transcript: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerEvent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an event was provided as input, this field will contain the name of the event."]
        pub trigger_event: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If an intent was provided as input, this field will contain a copy of the intent identifier."]
        pub trigger_intent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookPayloads")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of webhook payload in WebhookResponse.payload, in the order of call sequence. If some webhook call fails or doesn't return any payload, an empty `Struct` would be used instead."]
        pub webhook_payloads: ::std::option::Option<
            ::std::vec::Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookStatuses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of webhook call status in the order of call sequence."]
        pub webhook_statuses:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDialogflowCxV3beta1QueryResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1QueryResultBuilder {
            GoogleCloudDialogflowCxV3beta1QueryResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ReloadDocument operation."]
    pub struct GoogleCloudDialogflowCxV3beta1ReloadDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ReloadDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ReloadDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1ReloadDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Resource name and display name."]
    pub struct GoogleCloudDialogflowCxV3beta1ResourceName {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display name."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResourceName {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResourceNameBuilder {
            GoogleCloudDialogflowCxV3beta1ResourceNameBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a response message that can be returned by a conversational agent. Response messages are also used for output audio synthesis. The approach is as follows: * If at least one OutputAudioText response is present, then all OutputAudioText responses are linearly concatenated, and the result is used for output audio synthesis. * If the OutputAudioText responses are a mixture of text and SSML, then the concatenated result is treated as SSML; otherwise, the result is treated as either text or SSML as appropriate. The agent designer should ideally use either text or SSML consistently throughout the bot design. * Otherwise, all Text responses are linearly concatenated, and the result is used for output audio synthesis. This approach allows for more sophisticated user experience scenarios, where the text displayed to the user may differ from what is heard."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversationSuccess")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates that the conversation succeeded."]
        pub conversation_success: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccess>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endInteraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. A signal that indicates the interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only when the conversation reaches `END_SESSION` or `END_PAGE` page. It is not supposed to be defined by the user. It's guaranteed that there is at most one such message in each response."]
        pub end_interaction: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteraction>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "liveAgentHandoff")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Hands off conversation to a human agent."]
        pub live_agent_handoff: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoff>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mixedAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. An audio response message composed of both the synthesized Dialogflow agent responses and responses defined via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
        pub mixed_audio: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudio>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputAudioText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
        pub output_audio_text: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioText>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a response containing a custom, platform-specific payload."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "playAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Signal that the client should play an audio clip hosted at a client-specific URI. Dialogflow uses this to construct mixed_audio. However, Dialogflow itself does not try to read or process the URI in any way."]
        pub play_audio: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudio>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a text response."]
        pub text: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageText>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that the conversation succeeded, i.e., the bot handled the issue that the customer talked to it about. Dialogflow only uses this to determine which conversations should be counted as successful and doesn't process the metadata in this message in any way. Note that Dialogflow also considers conversations that get to the conversation end page as successful even if they don't return ConversationSuccess. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates that the conversation succeeded. * In a webhook response when you determine that you handled the customer issue."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccess {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom metadata. Dialogflow doesn't impose any structure on this."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccess {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccessBuilder
        {
            GoogleCloudDialogflowCxV3beta1ResponseMessageConversationSuccessBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that interaction with the Dialogflow agent has ended. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteraction {}
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteraction {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteractionBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageEndInteractionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Indicates that the conversation should be handed off to a live agent. Dialogflow only uses this to determine which conversations were handed off to a human agent for measurement purposes. What else to do with this signal is up to you and your handoff procedures. You may set this, for example: * In the entry_fulfillment of a Page if entering the page indicates something went extremely wrong in the conversation. * In a webhook response when you determine that the customer issue can only be handled by a human."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoff {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "metadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom metadata for your handoff procedure. Dialogflow doesn't impose any structure on this."]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoff {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoffBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageLiveAgentHandoffBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an audio message that is composed of both segments synthesized from the Dialogflow agent prompts and ones hosted externally at the specified URIs. The external URIs are specified via play_audio. This message is generated by Dialogflow only and not supposed to be defined by the user."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "segments")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Segments this audio response is composed of."]
        pub segments: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegment>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudio {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents one segment of audio."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this segment can be interrupted by the end user's speech and the client should then start the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Raw audio synthesized from the Dialogflow agent's response using the output config specified in the request."]
        pub audio: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Client-specific URI that points to an audio clip accessible to the client. Dialogflow does not impose any validation on it."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegment {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegmentBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageMixedAudioSegmentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A text or ssml response that is preferentially used for TTS output audio synthesis, as described in the comment on the ResponseMessage message."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSML text to be synthesized. For more information, see [SSML](/speech/text-to-speech/docs/ssml)."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw text to be synthesized."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioText {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioTextBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageOutputAudioTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifies an audio clip to be played by the client as part of the response."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URI of the audio clip. Dialogflow does not impose any validation on this value. It is specific to the client that reads it."]
        pub audio_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudio {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudioBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessagePlayAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The text response message."]
    pub struct GoogleCloudDialogflowCxV3beta1ResponseMessageText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowPlaybackInterruption")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Whether the playback of this message can be interrupted by the end user's speech and the client can then starts the next Dialogflow request."]
        pub allow_playback_interruption: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A collection of text responses."]
        pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1ResponseMessageText {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ResponseMessageTextBuilder {
            GoogleCloudDialogflowCxV3beta1ResponseMessageTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Agents.RestoreAgent."]
    pub struct GoogleCloudDialogflowCxV3beta1RestoreAgentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Uncompressed raw byte content for agent."]
        pub agent_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [Google Cloud Storage](https://cloud.google.com/storage/docs/) URI to restore agent from. The format of this URI must be `gs:///`."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1RestoreAgentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1RestoreAgentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1RestoreAgentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata returned for the TestCases.RunTestCase long running operation."]
    pub struct GoogleCloudDialogflowCxV3beta1RunTestCaseMetadata {}
    impl GoogleCloudDialogflowCxV3beta1RunTestCaseMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1RunTestCaseMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1RunTestCaseMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for TestCases.RunTestCase."]
    pub struct GoogleCloudDialogflowCxV3beta1RunTestCaseRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Environment name. If not set, draft environment is assumed. Format: `projects//locations//agents//environments/`."]
        pub environment: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1RunTestCaseRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1RunTestCaseRequestBuilder {
            GoogleCloudDialogflowCxV3beta1RunTestCaseRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for TestCases.RunTestCase."]
    pub struct GoogleCloudDialogflowCxV3beta1RunTestCaseResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "result")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result."]
        pub result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
    }
    impl GoogleCloudDialogflowCxV3beta1RunTestCaseResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1RunTestCaseResponseBuilder {
            GoogleCloudDialogflowCxV3beta1RunTestCaseResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the settings related to security issues, such as data redaction and data retention. It may take hours for updates on the settings to propagate to all the related components and take effect."]
    pub struct GoogleCloudDialogflowCxV3beta1SecuritySettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the security settings, unique within the location."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inspectTemplate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "DLP inspect template name. Use this template to define inspect base settings. If empty, we use the default DLP inspect config. The template name will have one of the following formats: `projects/PROJECT_ID/inspectTemplates/TEMPLATE_ID` OR `organizations/ORGANIZATION_ID/inspectTemplates/TEMPLATE_ID`"]
        pub inspect_template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Resource name of the settings. Format: `projects//locations//securitySettings/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "purgeDataTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of types of data to remove when retention settings triggers purge."]
        pub purge_data_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudDialogflowCxV3beta1SecuritySettingsPurgeDataTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactionScope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Defines on what data we apply redaction. Note that we don't redact data to which we don't have access, e.g., Stackdriver logs."]
        pub redaction_scope:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionScopeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "redactionStrategy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Strategy that defines how we do redaction."]
        pub redaction_strategy: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionStrategyEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "retentionWindowDays")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Retains the data for the specified number of days. User must Set a value lower than Dialogflow's default 30d TTL. Setting a value higher than that has no effect. A missing value or setting to 0 also means we use Dialogflow's default TTL."]
        pub retention_window_days: ::std::option::Option<::std::primitive::i64>,
    }
    impl GoogleCloudDialogflowCxV3beta1SecuritySettings {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SecuritySettingsBuilder {
            GoogleCloudDialogflowCxV3beta1SecuritySettingsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDialogflowCxV3beta1SecuritySettingsPurgeDataTypesEnum {
        #[serde(rename = "PURGE_DATA_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified. Do not use."]
        PurgeDataTypeUnspecified,
        #[serde(rename = "DIALOGFLOW_HISTORY")]
        #[doc = "Dialogflow history. This does not include Stackdriver log, which is owned by the user not Dialogflow."]
        DialogflowHistory,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1SecuritySettingsPurgeDataTypesEnum {
        fn default() -> Self {
            Self::PurgeDataTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Defines on what data we apply redaction. Note that we don't redact data to which we don't have access, e.g., Stackdriver logs."]
    pub enum GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionScopeEnum {
        #[serde(rename = "REDACTION_SCOPE_UNSPECIFIED")]
        #[doc = "Don't redact any kind of data."]
        RedactionScopeUnspecified,
        #[serde(rename = "REDACT_DISK_STORAGE")]
        #[doc = "On data to be written to disk or similar devices that are capable of holding data even if power is disconnected. This includes data that are temporarily saved on disk."]
        RedactDiskStorage,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionScopeEnum {
        fn default() -> Self {
            Self::RedactionScopeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Strategy that defines how we do redaction."]
    pub enum GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionStrategyEnum {
        #[serde(rename = "REDACTION_STRATEGY_UNSPECIFIED")]
        #[doc = "Do not redact."]
        RedactionStrategyUnspecified,
        #[serde(rename = "REDACT_WITH_SERVICE")]
        #[doc = "Call redaction service to clean up the data to be persisted."]
        RedactWithService,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1SecuritySettingsRedactionStrategyEnum
    {
        fn default() -> Self {
            Self::RedactionStrategyUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral."]
    pub struct GoogleCloudDialogflowCxV3beta1SentimentAnalysisResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowCxV3beta1SentimentAnalysisResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SentimentAnalysisResultBuilder {
            GoogleCloudDialogflowCxV3beta1SentimentAnalysisResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Session entity types are referred to as **User** entity types and are entities that are built for an individual user such as favorites, preferences, playlists, and so on. You can redefine a session entity type at the session level to extend or replace a custom entity type at the user session level (we refer to the entity types defined at the agent level as \"custom entity types\"). Note: session entity types apply to all queries, regardless of the language. For more information about entity types, see the [Dialogflow documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct GoogleCloudDialogflowCxV3beta1SessionEntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The collection of entities to override or supplement the custom entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityOverrideMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
        pub entity_override_mode: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1SessionEntityTypeEntityOverrideModeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the session entity type. Format: `projects//locations//agents//sessions//entityTypes/` or `projects//locations//agents//environments//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1SessionEntityType {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SessionEntityTypeBuilder {
            GoogleCloudDialogflowCxV3beta1SessionEntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
    pub enum GoogleCloudDialogflowCxV3beta1SessionEntityTypeEntityOverrideModeEnum {
        #[serde(rename = "ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        EntityOverrideModeUnspecified,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_OVERRIDE")]
        #[doc = "The collection of session entities overrides the collection of entities in the corresponding custom entity type."]
        EntityOverrideModeOverride,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
        #[doc = "The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge."]
        EntityOverrideModeSupplement,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1SessionEntityTypeEntityOverrideModeEnum
    {
        fn default() -> Self {
            Self::EntityOverrideModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents session information communicated to and from the webhook."]
    pub struct GoogleCloudDialogflowCxV3beta1SessionInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional for WebhookRequest. Optional for WebhookResponse. All parameters collected from forms and intents during the session. Parameters can be created, updated, or removed by the webhook. To remove a parameter from the session, the webhook should explicitly set the parameter value to null in WebhookResponse. The map is keyed by parameters' display names."]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present for WebhookRequest. Ignored for WebhookResponse. The unique identifier of the session. This field can be used by the webhook to identify a session. Format: `projects//locations//agents//sessions/` or `projects//locations//agents//environments//sessions/` if environment is specified."]
        pub session: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1SessionInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SessionInfoBuilder {
            GoogleCloudDialogflowCxV3beta1SessionInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Settings related to speech recognition."]
    pub struct GoogleCloudDialogflowCxV3beta1SpeechToTextSettings {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableSpeechAdaptation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether to use speech adaptation for speech recognition."]
        pub enable_speech_adaptation: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowCxV3beta1SpeechToTextSettings {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SpeechToTextSettingsBuilder {
            GoogleCloudDialogflowCxV3beta1SpeechToTextSettingsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Experiments.StartExperiment."]
    pub struct GoogleCloudDialogflowCxV3beta1StartExperimentRequest {}
    impl GoogleCloudDialogflowCxV3beta1StartExperimentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1StartExperimentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1StartExperimentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Experiments.StopExperiment."]
    pub struct GoogleCloudDialogflowCxV3beta1StopExperimentRequest {}
    impl GoogleCloudDialogflowCxV3beta1StopExperimentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1StopExperimentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1StopExperimentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Configuration of how speech should be synthesized."]
    pub struct GoogleCloudDialogflowCxV3beta1SynthesizeSpeechConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "effectsProfileId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given."]
        pub effects_profile_id: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pitch")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch."]
        pub pitch: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speakingRate")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error."]
        pub speaking_rate: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "voice")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The desired voice of the synthesized audio."]
        pub voice: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1VoiceSelectionParams>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "volumeGainDb")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. We strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that."]
        pub volume_gain_db: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowCxV3beta1SynthesizeSpeechConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1SynthesizeSpeechConfigBuilder {
            GoogleCloudDialogflowCxV3beta1SynthesizeSpeechConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a test case."]
    pub struct GoogleCloudDialogflowCxV3beta1TestCase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "creationTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. When the test was created."]
        pub creation_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the test case, unique within the agent. Limit of 200 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastTestResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latest test result."]
        pub last_test_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCaseResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the test case. TestCases.CreateTestCase will populate the name automatically. Otherwise use format: `projects//locations//agents/ /testCases/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "notes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Additional freeform notes about the test case. Limit of 400 characters."]
        pub notes: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tags are short descriptions that users may apply to test cases for organizational and filtering purposes. Each tag should start with \"#\" and has a limit of 30 characters."]
        pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCaseConversationTurns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conversation turns uttered when the test case was created, in chronological order. These include the canonical set of agent utterances that should occur when the agent is working properly."]
        pub test_case_conversation_turns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurn>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testConfig")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Config for the test case."]
        pub test_config:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestConfig>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestCase {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestCaseBuilder {
            GoogleCloudDialogflowCxV3beta1TestCaseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Error info for importing a test."]
    pub struct GoogleCloudDialogflowCxV3beta1TestCaseError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status associated with the test case."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case."]
        pub test_case:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TestCase>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestCaseError {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestCaseErrorBuilder {
            GoogleCloudDialogflowCxV3beta1TestCaseErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a result from running a test case in an agent environment."]
    pub struct GoogleCloudDialogflowCxV3beta1TestCaseResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversationTurns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The conversation turns uttered during the test case replay in chronological order."]
        pub conversation_turns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ConversationTurn>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "environment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Environment where the test was run. If not set, it indicates the draft environment."]
        pub environment: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name for the test case result. Format: `projects//locations//agents//testCases/ /results/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the test case passed in the agent environment."]
        pub test_result:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1TestCaseResultTestResultEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The time that the test was run."]
        pub test_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestCaseResult {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestCaseResultBuilder {
            GoogleCloudDialogflowCxV3beta1TestCaseResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Whether the test case passed in the agent environment."]
    pub enum GoogleCloudDialogflowCxV3beta1TestCaseResultTestResultEnum {
        #[serde(rename = "TEST_RESULT_UNSPECIFIED")]
        #[doc = "Not specified. Should never be used."]
        TestResultUnspecified,
        #[serde(rename = "PASSED")]
        #[doc = "The test passed."]
        Passed,
        #[serde(rename = "FAILED")]
        #[doc = "The test did not pass."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1TestCaseResultTestResultEnum {
        fn default() -> Self {
            Self::TestResultUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents configurations for a test case."]
    pub struct GoogleCloudDialogflowCxV3beta1TestConfig {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Flow name. If not set, default start flow is assumed. Format: `projects//locations//agents//flows/`."]
        pub flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trackingParameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Session parameters to be compared when calculating differences."]
        pub tracking_parameters: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestConfig {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestConfigBuilder {
            GoogleCloudDialogflowCxV3beta1TestConfigBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Error info for running a test."]
    pub struct GoogleCloudDialogflowCxV3beta1TestError {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "status")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The status associated with the test."]
        pub status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testCase")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The test case resource name."]
        pub test_case: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "testTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The timestamp when the test was completed."]
        pub test_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestError {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestErrorBuilder {
            GoogleCloudDialogflowCxV3beta1TestErrorBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The description of differences between original and replayed agent output."]
    pub struct GoogleCloudDialogflowCxV3beta1TestRunDifference {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A description of the diff, showing the actual output vs expected output."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of diff."]
        pub _type: ::std::option::Option<GoogleCloudDialogflowCxV3beta1TestRunDifferenceTypeEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1TestRunDifference {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TestRunDifferenceBuilder {
            GoogleCloudDialogflowCxV3beta1TestRunDifferenceBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of diff."]
    pub enum GoogleCloudDialogflowCxV3beta1TestRunDifferenceTypeEnum {
        #[serde(rename = "DIFF_TYPE_UNSPECIFIED")]
        #[doc = "Should never be used."]
        DiffTypeUnspecified,
        #[serde(rename = "INTENT")]
        #[doc = "The intent."]
        Intent,
        #[serde(rename = "PAGE")]
        #[doc = "The page."]
        Page,
        #[serde(rename = "PARAMETERS")]
        #[doc = "The parameters."]
        Parameters,
        #[serde(rename = "UTTERANCE")]
        #[doc = "The message utterance."]
        Utterance,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1TestRunDifferenceTypeEnum {
        fn default() -> Self {
            Self::DiffTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the natural language text to be processed."]
    pub struct GoogleCloudDialogflowCxV3beta1TextInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The UTF-8 encoded natural language text to be processed. Text length must not exceed 256 characters."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1TextInput {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TextInputBuilder {
            GoogleCloudDialogflowCxV3beta1TextInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Flows.TrainFlow."]
    pub struct GoogleCloudDialogflowCxV3beta1TrainFlowRequest {}
    impl GoogleCloudDialogflowCxV3beta1TrainFlowRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TrainFlowRequestBuilder {
            GoogleCloudDialogflowCxV3beta1TrainFlowRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Transition coverage represents the percentage of all possible page transitions (page-level transition routes and event handlers, excluding transition route groups) present within any of a parent's test cases."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionCoverage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverageScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percent of transitions in the agent that are covered."]
        pub coverage_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of Transitions present in the agent."]
        pub transitions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionCoverageTransition>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionCoverage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionCoverageBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionCoverageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transition in a page."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionCoverageTransition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "covered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the transition is covered by at least one of the agent's test cases."]
        pub covered: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "eventHandler")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Event handler."]
        pub event_handler:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1EventHandler>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "index")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The index of a transition in the transition list. Starting from 0."]
        pub index: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The start node of a transition."]
        pub source: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNode>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "target")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The end node of a transition."]
        pub target: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNode>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intent route or condition route."]
        pub transition_route:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionCoverageTransition {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The source or target of a transition."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNode {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "flow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates a transition to a Flow. Only some fields such as name and displayname will be set."]
        pub flow: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Flow>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "page")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates a transition to a Page. Only some fields such as name and displayname will be set."]
        pub page: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Page>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNode {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNodeBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionCoverageTransitionNodeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transition route specifies a intent that can be matched and/or a data condition that can be evaluated during a session. When a specified transition is matched, the following actions are taken in order: * If there is a `trigger_fulfillment` associated with the transition, it will be called. * If there is a `target_page` associated with the transition, the session will transition into the specified page. * If there is a `target_flow` associated with the transition, the session will transition into the specified flow."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionRoute {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition to evaluate against form parameters or session parameters. See the [conditions reference](https://cloud.google.com/dialogflow/cx/docs/reference/condition). At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
        pub condition: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of an Intent. Format: `projects//locations//agents//intents/`. Indicates that the transition can only happen when the given intent is matched. At least one of `intent` or `condition` must be specified. When both `intent` and `condition` are specified, the transition can only happen when both are fulfilled."]
        pub intent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this transition route."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "triggerFulfillment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment to call when the condition is satisfied. At least one of `trigger_fulfillment` and `target` must be specified. When both are defined, `trigger_fulfillment` is executed first."]
        pub trigger_fulfillment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1Fulfillment>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionRoute {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionRouteBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionRouteBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An TransitionRouteGroup represents a group of `TransitionRoutes` to be used by a Page."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionRouteGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the transition route group, unique within the Agent. The display name can be no longer than 30 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the transition route group. TransitionRouteGroups.CreateTransitionRouteGroup populates the name automatically. Format: `projects//locations//agents//flows//transitionRouteGroups/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoutes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transition routes associated with the TransitionRouteGroup."]
        pub transition_routes: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionRouteGroup {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionRouteGroupBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionRouteGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Transition route group coverage represents the percentage of all possible transition routes present within any of a parent's test cases. The results are grouped by the transition route group."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverageScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percent of transition routes in all the transition route groups that are covered."]
        pub coverage_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transition route group coverages."]
        pub coverages: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverage,
                >,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageBuilder {
            GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Coverage result message for one transition route group."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "coverageScore")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The percent of transition routes in the transition route group that are covered."]
        pub coverage_score: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "routeGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transition route group metadata. Only name and displayName will be set."]
        pub route_group: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRouteGroup>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of transition routes and coverage in the transition route group."]
        pub transitions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageTransition,
                >,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageBuilder
        {
            GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A transition coverage in a transition route group."]
    pub struct GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageTransition {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "covered")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether or not the transition route is covered by at least one of the agent's test cases."]
        pub covered: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "transitionRoute")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Intent route or condition route."]
        pub transition_route:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1TransitionRoute>>,
    }
    impl GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageTransition {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageTransitionBuilder
        {
            GoogleCloudDialogflowCxV3beta1TransitionRouteGroupCoverageCoverageTransitionBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for UpdateDocument operation."]
    pub struct GoogleCloudDialogflowCxV3beta1UpdateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1UpdateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1UpdateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowCxV3beta1UpdateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Agents.ValidateAgent."]
    pub struct GoogleCloudDialogflowCxV3beta1ValidateAgentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not specified, the agent's default language is used."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ValidateAgentRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ValidateAgentRequestBuilder {
            GoogleCloudDialogflowCxV3beta1ValidateAgentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for Flows.ValidateFlow."]
    pub struct GoogleCloudDialogflowCxV3beta1ValidateFlowRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If not specified, the agent's default language is used."]
        pub language_code: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1ValidateFlowRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ValidateFlowRequestBuilder {
            GoogleCloudDialogflowCxV3beta1ValidateFlowRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Agent/flow validation message."]
    pub struct GoogleCloudDialogflowCxV3beta1ValidationMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detail")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The message detail."]
        pub detail: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource names of the resources where the message is found."]
        pub resource_names: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResourceName>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the resources where the message is found."]
        pub resource_type:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1ValidationMessageResourceTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resources")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The names of the resources where the message is found."]
        pub resources: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "severity")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates the severity of the message."]
        pub severity:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1ValidationMessageSeverityEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1ValidationMessage {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1ValidationMessageBuilder {
            GoogleCloudDialogflowCxV3beta1ValidationMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the resources where the message is found."]
    pub enum GoogleCloudDialogflowCxV3beta1ValidationMessageResourceTypeEnum {
        #[serde(rename = "RESOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ResourceTypeUnspecified,
        #[serde(rename = "AGENT")]
        #[doc = "Agent."]
        Agent,
        #[serde(rename = "INTENT")]
        #[doc = "Intent."]
        Intent,
        #[serde(rename = "INTENT_TRAINING_PHRASE")]
        #[doc = "Intent training phrase."]
        IntentTrainingPhrase,
        #[serde(rename = "INTENT_PARAMETER")]
        #[doc = "Intent parameter."]
        IntentParameter,
        #[serde(rename = "INTENTS")]
        #[doc = "Multiple intents."]
        Intents,
        #[serde(rename = "INTENT_TRAINING_PHRASES")]
        #[doc = "Multiple training phrases."]
        IntentTrainingPhrases,
        #[serde(rename = "ENTITY_TYPE")]
        #[doc = "Entity type."]
        EntityType,
        #[serde(rename = "ENTITY_TYPES")]
        #[doc = "Multiple entity types."]
        EntityTypes,
        #[serde(rename = "WEBHOOK")]
        #[doc = "Webhook."]
        Webhook,
        #[serde(rename = "FLOW")]
        #[doc = "Flow."]
        Flow,
        #[serde(rename = "PAGE")]
        #[doc = "Page."]
        Page,
        #[serde(rename = "PAGES")]
        #[doc = "Multiple pages."]
        Pages,
        #[serde(rename = "TRANSITION_ROUTE_GROUP")]
        #[doc = "Transition route group."]
        TransitionRouteGroup,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1ValidationMessageResourceTypeEnum {
        fn default() -> Self {
            Self::ResourceTypeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Indicates the severity of the message."]
    pub enum GoogleCloudDialogflowCxV3beta1ValidationMessageSeverityEnum {
        #[serde(rename = "SEVERITY_UNSPECIFIED")]
        #[doc = "Unspecified."]
        SeverityUnspecified,
        #[serde(rename = "INFO")]
        #[doc = "The agent doesn't follow Dialogflow best practices."]
        Info,
        #[serde(rename = "WARNING")]
        #[doc = "The agent may not behave as expected."]
        Warning,
        #[serde(rename = "ERROR")]
        #[doc = "The agent may experience failures."]
        Error,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1ValidationMessageSeverityEnum {
        fn default() -> Self {
            Self::SeverityUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The history of variants update."]
    pub struct GoogleCloudDialogflowCxV3beta1VariantsHistory {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Update time of the variants."]
        pub update_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "versionVariants")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The flow versions as the variants."]
        pub version_variants:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1VersionVariants>>,
    }
    impl GoogleCloudDialogflowCxV3beta1VariantsHistory {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1VariantsHistoryBuilder {
            GoogleCloudDialogflowCxV3beta1VariantsHistoryBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a version of a flow."]
    pub struct GoogleCloudDialogflowCxV3beta1Version {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Create time of the version."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description of the version. The maximum length is 500 characters. If exceeded, the request is rejected."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the version. Limit of 64 characters."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Format: projects//locations//agents//flows//versions/. Version ID is a self-increasing number generated by Dialogflow upon version creation."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nluSettings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The NLU settings of the flow at version creation."]
        pub nlu_settings:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1NluSettings>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The state of this version. This field is read-only and cannot be set by create and update methods."]
        pub state: ::std::option::Option<GoogleCloudDialogflowCxV3beta1VersionStateEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1Version {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1VersionBuilder {
            GoogleCloudDialogflowCxV3beta1VersionBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The state of this version. This field is read-only and cannot be set by create and update methods."]
    pub enum GoogleCloudDialogflowCxV3beta1VersionStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "Not specified. This value is not used."]
        StateUnspecified,
        #[serde(rename = "RUNNING")]
        #[doc = "Version is not ready to serve (e.g. training is running)."]
        Running,
        #[serde(rename = "SUCCEEDED")]
        #[doc = "Training has succeeded and this version is ready to serve."]
        Succeeded,
        #[serde(rename = "FAILED")]
        #[doc = "Version training failed."]
        Failed,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1VersionStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A list of flow version variants."]
    pub struct GoogleCloudDialogflowCxV3beta1VersionVariants {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "variants")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of flow version variants."]
        pub variants: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1VersionVariantsVariant>,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1VersionVariants {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1VersionVariantsBuilder {
            GoogleCloudDialogflowCxV3beta1VersionVariantsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A single flow version with specified traffic allocation."]
    pub struct GoogleCloudDialogflowCxV3beta1VersionVariantsVariant {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isControlGroup")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether the variant is for the control group."]
        pub is_control_group: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trafficAllocation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Percentage of the traffic which should be routed to this version of flow. Traffic allocation for a single flow must sum up to 1.0."]
        pub traffic_allocation: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The name of the flow version. Format: `projects//locations//agents//flows//versions/`."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1VersionVariantsVariant {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1VersionVariantsVariantBuilder {
            GoogleCloudDialogflowCxV3beta1VersionVariantsVariantBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Description of which voice to use for speech synthesis."]
    pub struct GoogleCloudDialogflowCxV3beta1VoiceSelectionParams {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and ssml_gender. For the list of available voices, please refer to [Supported voices and languages](https://cloud.google.com/text-to-speech/docs/voices)."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssmlGender")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
        pub ssml_gender:
            ::std::option::Option<GoogleCloudDialogflowCxV3beta1VoiceSelectionParamsSsmlGenderEnum>,
    }
    impl GoogleCloudDialogflowCxV3beta1VoiceSelectionParams {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1VoiceSelectionParamsBuilder {
            GoogleCloudDialogflowCxV3beta1VoiceSelectionParamsBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as language_code and name. Note that this is only a preference, not requirement. If a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request."]
    pub enum GoogleCloudDialogflowCxV3beta1VoiceSelectionParamsSsmlGenderEnum {
        #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
        #[doc = "An unspecified gender, which means that the client doesn't care which gender the selected voice will have."]
        SsmlVoiceGenderUnspecified,
        #[serde(rename = "SSML_VOICE_GENDER_MALE")]
        #[doc = "A male voice."]
        SsmlVoiceGenderMale,
        #[serde(rename = "SSML_VOICE_GENDER_FEMALE")]
        #[doc = "A female voice."]
        SsmlVoiceGenderFemale,
        #[serde(rename = "SSML_VOICE_GENDER_NEUTRAL")]
        #[doc = "A gender-neutral voice."]
        SsmlVoiceGenderNeutral,
    }
    impl ::std::default::Default for GoogleCloudDialogflowCxV3beta1VoiceSelectionParamsSsmlGenderEnum {
        fn default() -> Self {
            Self::SsmlVoiceGenderUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Webhooks host the developer's business logic. During a session, webhooks allow the developer to use the data extracted by Dialogflow's natural language processing to generate dynamic responses, validate collected data, or trigger actions on the backend."]
    pub struct GoogleCloudDialogflowCxV3beta1Webhook {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "disabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the webhook is disabled."]
        pub disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The human-readable name of the webhook, unique within the agent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericWebService")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Configuration for a generic web service."]
        pub generic_web_service: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookGenericWebService>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the webhook. Required for the Webhooks.UpdateWebhook method. Webhooks.CreateWebhook populates the name automatically. Format: `projects//locations//agents//webhooks/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timeout")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Webhook execution timeout. Execution is considered failed if Dialogflow doesn't receive a response from webhook at the end of the timeout period. Defaults to 5 seconds, maximum allowed timeout is 30 seconds."]
        pub timeout: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1Webhook {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents configuration for a generic web service."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookGenericWebService {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "password")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The password for HTTP Basic authentication."]
        pub password: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestHeaders")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The HTTP request headers to send together with webhook requests."]
        pub request_headers:
            ::std::option::Option<::std::collections::BTreeMap<String, ::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The webhook URI for receiving POST requests. It must use https protocol."]
        pub uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "username")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The user name for HTTP Basic authentication."]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookGenericWebService {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookGenericWebServiceBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookGenericWebServiceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for a webhook call."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "detectIntentResponseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The unique identifier of the DetectIntentResponse that will be returned to the API caller."]
        pub detect_intent_response_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Information about the fulfillment that triggered this webhook call."]
        pub fulfillment_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about the last matched intent."]
        pub intent_info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user. Webhook can choose to append or replace this list in WebhookResponse.fulfillment_response;"]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about page status."]
        pub page_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Custom data set in QueryParameters.payload."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentAnalysisResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result of the current user request. The field is filled when sentiment analysis is configured to be enabled for the request."]
        pub sentiment_analysis_result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about session status."]
        pub session_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionInfo>>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookRequest {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookRequestBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents fulfillment information communicated to the webhook."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The tag used to identify which fulfillment is being called."]
        pub tag: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfoBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookRequestFulfillmentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents intent information communicated to the webhook."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "confidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The confidence of the matched intent. Values range from 0.0 (completely uncertain) to 1.0 (completely certain)."]
        pub confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The display name of the last matched intent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lastMatchedIntent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. The unique identifier of the last matched intent. Format: `projects//locations//agents//intents/`."]
        pub last_matched_intent: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Parameters identified as a result of intent matching. This is a map of the name of the identified parameter to the value of the parameter identified from the user's utterance. All parameters defined in the matched intent that are identified will be surfaced here."]
        pub parameters: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<
                    GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValue,
                >,
            >,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfo {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a value for an intent parameter."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Original text value extracted from user utterance."]
        pub original_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resolvedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Always present. Structured value for the parameter extracted from user utterance."]
        pub resolved_value: ::std::option::Option<::serde_json::Value>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValue {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValueBuilder
        {
            GoogleCloudDialogflowCxV3beta1WebhookRequestIntentInfoIntentParameterValueBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of sentiment analysis."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResult {
        pub fn builder(
        ) -> GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResultBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookRequestSentimentAnalysisResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for a webhook call."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentResponse")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The fulfillment response to send to the user. This field can be omitted by the webhook if it does not intend to send any response to the user."]
        pub fulfillment_response: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponse>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about page status. This field can be omitted by the webhook if it does not intend to modify page status."]
        pub page_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1PageInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Value to append directly to QueryResult.webhook_payloads."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Information about session status. This field can be omitted by the webhook if it does not intend to modify session status."]
        pub session_info:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1SessionInfo>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetFlow")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target flow to transition to. Format: `projects//locations//agents//flows/`."]
        pub target_flow: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "targetPage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The target page to transition to. Format: `projects//locations//agents//flows//pages/`."]
        pub target_page: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookResponseBuilder {
            GoogleCloudDialogflowCxV3beta1WebhookResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a fulfillment response to the user."]
    pub struct GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mergeBehavior")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Merge behavior for `messages`."]
        pub merge_behavior: ::std::option::Option<
            GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseMergeBehaviorEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of rich message responses to present to the user."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowCxV3beta1ResponseMessage>>,
        >,
    }
    impl GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponse {
        pub fn builder() -> GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseBuilder
        {
            GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Merge behavior for `messages`."]
    pub enum GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseMergeBehaviorEnum {
        #[serde(rename = "MERGE_BEHAVIOR_UNSPECIFIED")]
        #[doc = "Not specified. `APPEND` will be used."]
        MergeBehaviorUnspecified,
        #[serde(rename = "APPEND")]
        #[doc = "`messages` will be appended to the list of messages waiting to be sent to the user."]
        Append,
        #[serde(rename = "REPLACE")]
        #[doc = "`messages` will replace the list of messages waiting to be sent to the user."]
        Replace,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowCxV3beta1WebhookResponseFulfillmentResponseMergeBehaviorEnum
    {
        fn default() -> Self {
            Self::MergeBehaviorUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a part of a message possibly annotated with an entity. The part can be an entity or purely a part of the message between two entities or message start/end."]
    pub struct GoogleCloudDialogflowV2AnnotatedMessagePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [Dialogflow system entity type](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. If this is empty, Dialogflow could not annotate the phrase part with a system entity."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The [Dialogflow system entity formatted value ](https://cloud.google.com/dialogflow/docs/reference/system-entities) of this message part. For example for a system entity of type `@sys.unit-currency`, this may contain: { \"amount\": 5, \"currency\": \"USD\" } "]
        pub formatted_value: ::std::option::Option<::serde_json::Value>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A part of a message possibly annotated with an entity."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2AnnotatedMessagePart {
        pub fn builder() -> GoogleCloudDialogflowV2AnnotatedMessagePartBuilder {
            GoogleCloudDialogflowV2AnnotatedMessagePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for EntityTypes.BatchUpdateEntityTypes."]
    pub struct GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of updated or created entity types."]
        pub entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityType>>,
        >,
    }
    impl GoogleCloudDialogflowV2BatchUpdateEntityTypesResponse {
        pub fn builder() -> GoogleCloudDialogflowV2BatchUpdateEntityTypesResponseBuilder {
            GoogleCloudDialogflowV2BatchUpdateEntityTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Intents.BatchUpdateIntents."]
    pub struct GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of updated or created intents."]
        pub intents: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Intent>>,
        >,
    }
    impl GoogleCloudDialogflowV2BatchUpdateIntentsResponse {
        pub fn builder() -> GoogleCloudDialogflowV2BatchUpdateIntentsResponseBuilder {
            GoogleCloudDialogflowV2BatchUpdateIntentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dialogflow contexts are similar to natural language context. If a person says to you \"they are orange\", you need context in order to understand what \"they\" is referring to. Similarly, for Dialogflow to handle an end-user expression like that, it needs to be provided with context in order to correctly match an intent. Using contexts, you can control the flow of a conversation. You can configure contexts for an intent by setting input and output contexts, which are identified by string names. When an intent is matched, any configured output contexts for that intent become active. While any contexts are active, Dialogflow is more likely to match intents that are configured with input contexts that correspond to the currently active contexts. For more information about context, see the [Contexts guide](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
    pub struct GoogleCloudDialogflowV2Context {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lifespanCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries."]
        pub lifespan_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the context. Format: `projects//agent/sessions//contexts/`, or `projects//agent/environments//users//sessions//contexts/`. The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowV2Context {
        pub fn builder() -> GoogleCloudDialogflowV2ContextBuilder {
            GoogleCloudDialogflowV2ContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a notification sent to Pub/Sub subscribers for conversation lifecycle events."]
    pub struct GoogleCloudDialogflowV2ConversationEvent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "conversation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the conversation this notification refers to. Format: `projects//conversations/`."]
        pub conversation: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "errorStatus")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "More detailed information about an error. Only set for type UNRECOVERABLE_ERROR_IN_PHONE_CALL."]
        pub error_status: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newMessagePayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Payload of NEW_MESSAGE event."]
        pub new_message_payload:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Message>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the event that this notification refers to."]
        pub _type: ::std::option::Option<GoogleCloudDialogflowV2ConversationEventTypeEnum>,
    }
    impl GoogleCloudDialogflowV2ConversationEvent {
        pub fn builder() -> GoogleCloudDialogflowV2ConversationEventBuilder {
            GoogleCloudDialogflowV2ConversationEventBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the event that this notification refers to."]
    pub enum GoogleCloudDialogflowV2ConversationEventTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Type not set."]
        TypeUnspecified,
        #[serde(rename = "CONVERSATION_STARTED")]
        #[doc = "A new conversation has been opened. This is fired when a telephone call is answered, or a conversation is created via the API."]
        ConversationStarted,
        #[serde(rename = "CONVERSATION_FINISHED")]
        #[doc = "An existing conversation has closed. This is fired when a telephone call is terminated, or a conversation is closed via the API."]
        ConversationFinished,
        #[serde(rename = "HUMAN_INTERVENTION_NEEDED")]
        #[doc = "An existing conversation has received notification from Dialogflow that human intervention is required."]
        HumanInterventionNeeded,
        #[serde(rename = "NEW_MESSAGE")]
        #[doc = "An existing conversation has received a new message, either from API or telephony. It is configured in ConversationProfile.new_message_event_notification_config"]
        NewMessage,
        #[serde(rename = "UNRECOVERABLE_ERROR")]
        #[doc = "Unrecoverable error during a telephone call. In general non-recoverable errors only occur if something was misconfigured in the ConversationProfile corresponding to the call. After a non-recoverable error, Dialogflow may stop responding. We don't fire this event: * in an API call because we can directly return the error, or, * when we can recover from an error."]
        UnrecoverableError,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2ConversationEventTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Each intent parameter has a type, called the entity type, which dictates exactly how data from an end-user expression is extracted. Dialogflow provides predefined system entities that can match many common types of data. For example, there are system entities for matching dates, times, colors, email addresses, and so on. You can also create your own custom entities for matching custom data. For example, you could define a vegetable entity that can match the types of vegetables available for purchase with a grocery store agent. For more information, see the [Entity guide](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct GoogleCloudDialogflowV2EntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoExpansionMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
        pub auto_expansion_mode:
            ::std::option::Option<GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the entity type."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableFuzzyExtraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Enables fuzzy entity extraction during classification."]
        pub enable_fuzzy_extraction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of entity entries associated with the entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates the kind of entity type."]
        pub kind: ::std::option::Option<GoogleCloudDialogflowV2EntityTypeKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Format: `projects//agent/entityTypes/`."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2EntityType {
        pub fn builder() -> GoogleCloudDialogflowV2EntityTypeBuilder {
            GoogleCloudDialogflowV2EntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
    pub enum GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
        #[serde(rename = "AUTO_EXPANSION_MODE_UNSPECIFIED")]
        #[doc = "Auto expansion disabled for the entity."]
        AutoExpansionModeUnspecified,
        #[serde(rename = "AUTO_EXPANSION_MODE_DEFAULT")]
        #[doc = "Allows an agent to recognize values that have not been explicitly listed in the entity."]
        AutoExpansionModeDefault,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2EntityTypeAutoExpansionModeEnum {
        fn default() -> Self {
            Self::AutoExpansionModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates the kind of entity type."]
    pub enum GoogleCloudDialogflowV2EntityTypeKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        KindUnspecified,
        #[serde(rename = "KIND_MAP")]
        #[doc = "Map entity types allow mapping of a group of synonyms to a reference value."]
        KindMap,
        #[serde(rename = "KIND_LIST")]
        #[doc = "List entity types contain a set of entries that do not map to reference values. However, list entity types can contain references to other entity types (with or without aliases)."]
        KindList,
        #[serde(rename = "KIND_REGEXP")]
        #[doc = "Regexp entity types allow to specify regular expressions in entries values."]
        KindRegexp,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2EntityTypeKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An **entity entry** for an associated entity type."]
    pub struct GoogleCloudDialogflowV2EntityTypeEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`."]
        pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases)."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2EntityTypeEntity {
        pub fn builder() -> GoogleCloudDialogflowV2EntityTypeEntityBuilder {
            GoogleCloudDialogflowV2EntityTypeEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `\"Hello #welcome_event.name! What can I do for you today?\"`."]
    pub struct GoogleCloudDialogflowV2EventInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the event."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowV2EventInput {
        pub fn builder() -> GoogleCloudDialogflowV2EventInputBuilder {
            GoogleCloudDialogflowV2EventInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.ExportAgent."]
    pub struct GoogleCloudDialogflowV2ExportAgentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zip compressed raw byte content for agent."]
        pub agent_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in `ExportAgentRequest`."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2ExportAgentResponse {
        pub fn builder() -> GoogleCloudDialogflowV2ExportAgentResponseBuilder {
            GoogleCloudDialogflowV2ExportAgentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An intent categorizes an end-user's intention for one conversation turn. For each agent, you define many intents, where your combined intents can handle a complete conversation. When an end-user writes or says something, referred to as an end-user expression or end-user input, Dialogflow matches the end-user input to the best intent in your agent. Matching an intent is also known as intent classification. For more information, see the [intent guide](https://cloud.google.com/dialogflow/docs/intents-overview)."]
    pub struct GoogleCloudDialogflowV2Intent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultResponsePlatforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
        pub default_response_platforms: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of this intent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters."]
        pub events: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupIntentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output."]
        pub followup_intent_info: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentFollowupIntentInfo>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputContextNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The list of context names required for this intent to be triggered. Format: `projects//agent/sessions/-/contexts/`."]
        pub input_context_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFallback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether this is a fallback intent."]
        pub is_fallback: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mlDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off."]
        pub ml_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Format: `projects//agent/intents/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of parameters associated with the intent."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentParameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only after creation. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`."]
        pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether to delete all contexts in the current session when this intent is matched."]
        pub reset_contexts: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Read-only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. We populate this field only in the output. Format: `projects//agent/intents/`."]
        pub root_followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of examples that the agent is trained on."]
        pub training_phrases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentTrainingPhrase>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
        pub webhook_state: ::std::option::Option<GoogleCloudDialogflowV2IntentWebhookStateEnum>,
    }
    impl GoogleCloudDialogflowV2Intent {
        pub fn builder() -> GoogleCloudDialogflowV2IntentBuilder {
            GoogleCloudDialogflowV2IntentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Default platform."]
        PlatformUnspecified,
        #[serde(rename = "FACEBOOK")]
        #[doc = "Facebook."]
        Facebook,
        #[serde(rename = "SLACK")]
        #[doc = "Slack."]
        Slack,
        #[serde(rename = "TELEGRAM")]
        #[doc = "Telegram."]
        Telegram,
        #[serde(rename = "KIK")]
        #[doc = "Kik."]
        Kik,
        #[serde(rename = "SKYPE")]
        #[doc = "Skype."]
        Skype,
        #[serde(rename = "LINE")]
        #[doc = "Line."]
        Line,
        #[serde(rename = "VIBER")]
        #[doc = "Viber."]
        Viber,
        #[serde(rename = "ACTIONS_ON_GOOGLE")]
        #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        ActionsOnGoogle,
        #[serde(rename = "GOOGLE_HANGOUTS")]
        #[doc = "Google Hangouts."]
        GoogleHangouts,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2IntentDefaultResponsePlatformsEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
    pub enum GoogleCloudDialogflowV2IntentWebhookStateEnum {
        #[serde(rename = "WEBHOOK_STATE_UNSPECIFIED")]
        #[doc = "Webhook is disabled in the agent and in the intent."]
        WebhookStateUnspecified,
        #[serde(rename = "WEBHOOK_STATE_ENABLED")]
        #[doc = "Webhook is enabled in the agent and in the intent."]
        WebhookStateEnabled,
        #[serde(rename = "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING")]
        #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot filling prompt is forwarded to the webhook."]
        WebhookStateEnabledForSlotFilling,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2IntentWebhookStateEnum {
        fn default() -> Self {
            Self::WebhookStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single followup intent in the chain."]
    pub struct GoogleCloudDialogflowV2IntentFollowupIntentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the followup intent. Format: `projects//agent/intents/`."]
        pub followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`."]
        pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentFollowupIntentInfo {
        pub fn builder() -> GoogleCloudDialogflowV2IntentFollowupIntentInfoBuilder {
            GoogleCloudDialogflowV2IntentFollowupIntentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A rich response message. Corresponds to the intent `Response` field in the Dialogflow console. For more information, see [Rich response messages](https://cloud.google.com/dialogflow/docs/intents-rich-messages)."]
    pub struct GoogleCloudDialogflowV2IntentMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The basic card response for Actions on Google."]
        pub basic_card:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "browseCarouselCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Browse carousel card for Actions on Google."]
        pub browse_carousel_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "card")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The card response."]
        pub card:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carouselSelect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The carousel card response for Actions on Google."]
        pub carousel_select: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCarouselSelect>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The image response."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkOutSuggestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The link out suggestion chip for Actions on Google."]
        pub link_out_suggestion: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listSelect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list card response for Actions on Google."]
        pub list_select: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageListSelect>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The media content card for Actions on Google."]
        pub media_content: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageMediaContent>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A custom platform-specific response."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The platform that this message is intended for."]
        pub platform: ::std::option::Option<GoogleCloudDialogflowV2IntentMessagePlatformEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quickReplies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The quick replies response."]
        pub quick_replies: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageQuickReplies>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The voice and text-only responses for Actions on Google."]
        pub simple_responses: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSimpleResponses>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The suggestion chips for Actions on Google."]
        pub suggestions: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSuggestions>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table card for Actions on Google."]
        pub table_card:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text response."]
        pub text:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageText>>,
    }
    impl GoogleCloudDialogflowV2IntentMessage {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageBuilder {
            GoogleCloudDialogflowV2IntentMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The platform that this message is intended for."]
    pub enum GoogleCloudDialogflowV2IntentMessagePlatformEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Default platform."]
        PlatformUnspecified,
        #[serde(rename = "FACEBOOK")]
        #[doc = "Facebook."]
        Facebook,
        #[serde(rename = "SLACK")]
        #[doc = "Slack."]
        Slack,
        #[serde(rename = "TELEGRAM")]
        #[doc = "Telegram."]
        Telegram,
        #[serde(rename = "KIK")]
        #[doc = "Kik."]
        Kik,
        #[serde(rename = "SKYPE")]
        #[doc = "Skype."]
        Skype,
        #[serde(rename = "LINE")]
        #[doc = "Line."]
        Line,
        #[serde(rename = "VIBER")]
        #[doc = "Viber."]
        Viber,
        #[serde(rename = "ACTIONS_ON_GOOGLE")]
        #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        ActionsOnGoogle,
        #[serde(rename = "GOOGLE_HANGOUTS")]
        #[doc = "Google Hangouts."]
        GoogleHangouts,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2IntentMessagePlatformEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The basic card message. Useful for displaying information."]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of card buttons."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required, unless image is present. The body text of the card."]
        pub formatted_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image for the card."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The subtitle of the card."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageBasicCard {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageBasicCardBuilder {
            GoogleCloudDialogflowV2IntentMessageBasicCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The button object that appears at the bottom of a card."]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openUriAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Action to take when a user taps on the button."]
        pub open_uri_action: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the button."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageBasicCardButton {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageBasicCardButtonBuilder {
            GoogleCloudDialogflowV2IntentMessageBasicCardButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Opens the given URI."]
    pub struct GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The HTTP or HTTPS scheme URI."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriAction {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriActionBuilder
        {
            GoogleCloudDialogflowV2IntentMessageBasicCardButtonOpenUriActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel"]
    pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageDisplayOptions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
        pub image_display_options: ::std::option::Option<
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem,
                >,
            >,
        >,
    }
    impl GoogleCloudDialogflowV2IntentMessageBrowseCarouselCard {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBuilder {
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
    pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
        #[serde(rename = "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED")]
        #[doc = "Fill the gaps between the image and the image container with gray bars."]
        ImageDisplayOptionsUnspecified,
        #[serde(rename = "GRAY")]
        #[doc = "Fill the gaps between the image and the image container with gray bars."]
        Gray,
        #[serde(rename = "WHITE")]
        #[doc = "Fill the gaps between the image and the image container with white bars."]
        White,
        #[serde(rename = "CROPPED")]
        #[doc = "Image is scaled such that the image width and height match or exceed the container dimensions. This may crop the top and bottom of the image if the scaled image height is greater than the container height, or crop the left and right of the image if the scaled image width is greater than the container width. This is similar to \"Zoom Mode\" on a widescreen TV when playing a 4:3 video."]
        Cropped,
        #[serde(rename = "BLURRED_BACKGROUND")]
        #[doc = "Pad the gaps between image and image frame with a blurred copy of the same image."]
        BlurredBackground,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardImageDisplayOptionsEnum
    {
        fn default() -> Self {
            Self::ImageDisplayOptionsUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Browsing carousel tile"]
    pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "description")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Description of the carousel item. Maximum of four lines of text."] pub description : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "footer")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text."] pub footer : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "image")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Hero image for the carousel item."] pub image : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2IntentMessageImage > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "openUriAction")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Action to present to the user."] pub open_uri_action : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "title")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Title of the carousel item. Maximum of two lines of text."] pub title : :: std :: option :: Option < :: std :: string :: String > }
    impl GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItem {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemBuilder
        {
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Actions on Google action to open a given url."]
    pub struct GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "url")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. URL"] pub url : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "urlTypeHint")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."] pub url_type_hint : :: std :: option :: Option < GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum > }
    impl GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction {
        pub fn builder () -> GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionBuilder{
            GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionBuilder :: default ()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."]
    pub enum GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
    {
        #[serde(rename = "URL_TYPE_HINT_UNSPECIFIED")]
        #[doc = "Unspecified"]
        UrlTypeHintUnspecified,
        #[serde(rename = "AMP_ACTION")]
        #[doc = "Url would be an amp action"]
        AmpAction,
        #[serde(rename = "AMP_CONTENT")]
        #[doc = "URL that points directly to AMP content, or to a canonical URL which refers to AMP content via ."]
        AmpContent,
    }
    impl :: std :: default :: Default for GoogleCloudDialogflowV2IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum { fn default () -> Self { Self :: UrlTypeHintUnspecified } }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card response message."]
    pub struct GoogleCloudDialogflowV2IntentMessageCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of card buttons."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCardButton>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The public URI to an image file for the card."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The subtitle of the card."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageCard {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageCardBuilder {
            GoogleCloudDialogflowV2IntentMessageCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains information about a button."]
    pub struct GoogleCloudDialogflowV2IntentMessageCardButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to send back to the Dialogflow API or a URI to open."]
        pub postback: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to show on the button."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageCardButton {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageCardButtonBuilder {
            GoogleCloudDialogflowV2IntentMessageCardButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card for presenting a carousel of options to select from."]
    pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Carousel items."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageCarouselSelectItem>,
            >,
        >,
    }
    impl GoogleCloudDialogflowV2IntentMessageCarouselSelect {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageCarouselSelectBuilder {
            GoogleCloudDialogflowV2IntentMessageCarouselSelectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An item in the carousel."]
    pub struct GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The body text of the card."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image to display."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Additional info about the option item."]
        pub info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Title of the carousel item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageCarouselSelectItem {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageCarouselSelectItemBuilder {
            GoogleCloudDialogflowV2IntentMessageCarouselSelectItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Column properties for TableCard."]
    pub struct GoogleCloudDialogflowV2IntentMessageColumnProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Column heading."]
        pub header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Defines text alignment for all cells in this column."]
        pub horizontal_alignment: ::std::option::Option<
            GoogleCloudDialogflowV2IntentMessageColumnPropertiesHorizontalAlignmentEnum,
        >,
    }
    impl GoogleCloudDialogflowV2IntentMessageColumnProperties {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageColumnPropertiesBuilder {
            GoogleCloudDialogflowV2IntentMessageColumnPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Defines text alignment for all cells in this column."]
    pub enum GoogleCloudDialogflowV2IntentMessageColumnPropertiesHorizontalAlignmentEnum {
        #[serde(rename = "HORIZONTAL_ALIGNMENT_UNSPECIFIED")]
        #[doc = "Text is aligned to the leading edge of the column."]
        HorizontalAlignmentUnspecified,
        #[serde(rename = "LEADING")]
        #[doc = "Text is aligned to the leading edge of the column."]
        Leading,
        #[serde(rename = "CENTER")]
        #[doc = "Text is centered in the column."]
        Center,
        #[serde(rename = "TRAILING")]
        #[doc = "Text is aligned to the trailing edge of the column."]
        Trailing,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2IntentMessageColumnPropertiesHorizontalAlignmentEnum
    {
        fn default() -> Self {
            Self::HorizontalAlignmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The image response message."]
    pub struct GoogleCloudDialogflowV2IntentMessageImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessibilityText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A text description of the image to be used for accessibility, e.g., screen readers."]
        pub accessibility_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The public URI to an image file."]
        pub image_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageImage {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageImageBuilder {
            GoogleCloudDialogflowV2IntentMessageImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The suggestion chip message that allows the user to jump out to the app or website associated with this agent."]
    pub struct GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the app or site this chip is linking to."]
        pub destination_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URI of the app or site to open when the user taps the suggestion chip."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageLinkOutSuggestion {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageLinkOutSuggestionBuilder {
            GoogleCloudDialogflowV2IntentMessageLinkOutSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card for presenting a list of options to select from."]
    pub struct GoogleCloudDialogflowV2IntentMessageListSelect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List items."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageListSelectItem>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Subtitle of the list."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The overall title of the list."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageListSelect {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageListSelectBuilder {
            GoogleCloudDialogflowV2IntentMessageListSelectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An item in the list."]
    pub struct GoogleCloudDialogflowV2IntentMessageListSelectItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The main text describing the item."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image to display."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Additional information about this option."]
        pub info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSelectItemInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the list item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageListSelectItem {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageListSelectItemBuilder {
            GoogleCloudDialogflowV2IntentMessageListSelectItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The media content card for Actions on Google."]
    pub struct GoogleCloudDialogflowV2IntentMessageMediaContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of media objects."]
        pub media_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. What type of media is the content (ie \"audio\")."]
        pub media_type:
            ::std::option::Option<GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum>,
    }
    impl GoogleCloudDialogflowV2IntentMessageMediaContent {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageMediaContentBuilder {
            GoogleCloudDialogflowV2IntentMessageMediaContentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. What type of media is the content (ie \"audio\")."]
    pub enum GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
        #[serde(rename = "RESPONSE_MEDIA_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ResponseMediaTypeUnspecified,
        #[serde(rename = "AUDIO")]
        #[doc = "Response media type is audio."]
        Audio,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2IntentMessageMediaContentMediaTypeEnum {
        fn default() -> Self {
            Self::ResponseMediaTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response media object for media content card."]
    pub struct GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Url where the media is stored."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of media card."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Icon to display above media content."]
        pub icon:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Image to display above media content."]
        pub large_image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of media card."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObject {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObjectBuilder {
            GoogleCloudDialogflowV2IntentMessageMediaContentResponseMediaObjectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The quick replies response message."]
    pub struct GoogleCloudDialogflowV2IntentMessageQuickReplies {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quickReplies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of quick replies."]
        pub quick_replies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the collection of quick replies."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageQuickReplies {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageQuickRepliesBuilder {
            GoogleCloudDialogflowV2IntentMessageQuickRepliesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional info about the select item for when it is triggered in a dialog."]
    pub struct GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique key that will be sent back to the agent if this response is given."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of synonyms that can also be used to trigger this item in dialog."]
        pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowV2IntentMessageSelectItemInfo {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageSelectItemInfoBuilder {
            GoogleCloudDialogflowV2IntentMessageSelectItemInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The simple response message containing speech or text."]
    pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to display."]
        pub display_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textToSpeech")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml."]
        pub text_to_speech: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageSimpleResponse {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageSimpleResponseBuilder {
            GoogleCloudDialogflowV2IntentMessageSimpleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`."]
    pub struct GoogleCloudDialogflowV2IntentMessageSimpleResponses {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of simple responses."]
        pub simple_responses: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSimpleResponse>>,
        >,
    }
    impl GoogleCloudDialogflowV2IntentMessageSimpleResponses {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageSimpleResponsesBuilder {
            GoogleCloudDialogflowV2IntentMessageSimpleResponsesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The suggestion chip message that the user can tap to quickly post a reply to the conversation."]
    pub struct GoogleCloudDialogflowV2IntentMessageSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text shown the in the suggestion chip."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageSuggestion {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageSuggestionBuilder {
            GoogleCloudDialogflowV2IntentMessageSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The collection of suggestions."]
    pub struct GoogleCloudDialogflowV2IntentMessageSuggestions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of suggested replies."]
        pub suggestions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageSuggestion>>,
        >,
    }
    impl GoogleCloudDialogflowV2IntentMessageSuggestions {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageSuggestionsBuilder {
            GoogleCloudDialogflowV2IntentMessageSuggestionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Table card for Actions on Google."]
    pub struct GoogleCloudDialogflowV2IntentMessageTableCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of buttons for the card."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageBasicCardButton>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Display properties for the columns in this table."]
        pub column_properties: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageColumnProperties>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Image which should be displayed on the card."]
        pub image:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageImage>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Rows in this table of data."]
        pub rows: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCardRow>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Subtitle to the title."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageTableCard {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageTableCardBuilder {
            GoogleCloudDialogflowV2IntentMessageTableCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cell of TableCardRow."]
    pub struct GoogleCloudDialogflowV2IntentMessageTableCardCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Text in this cell."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentMessageTableCardCell {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageTableCardCellBuilder {
            GoogleCloudDialogflowV2IntentMessageTableCardCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Row of TableCard."]
    pub struct GoogleCloudDialogflowV2IntentMessageTableCardRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of cells that make up this row."]
        pub cells: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessageTableCardCell>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dividerAfter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Whether to add a visual divider after this row."]
        pub divider_after: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowV2IntentMessageTableCardRow {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageTableCardRowBuilder {
            GoogleCloudDialogflowV2IntentMessageTableCardRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The text response message."]
    pub struct GoogleCloudDialogflowV2IntentMessageText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of the agent's responses."]
        pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowV2IntentMessageText {
        pub fn builder() -> GoogleCloudDialogflowV2IntentMessageTextBuilder {
            GoogleCloudDialogflowV2IntentMessageTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents intent parameters."]
    pub struct GoogleCloudDialogflowV2IntentParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The default value to use when the `value` yields an empty result. Default values can be extracted from contexts by using the following syntax: `#context_name.parameter_name`."]
        pub default_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the parameter."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityTypeDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the entity type, prefixed with `@`, that describes values of the parameter. If the parameter is required, this must be provided."]
        pub entity_type_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mandatory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the parameter is required. That is, whether the intent cannot be completed without collecting the parameter value."]
        pub mandatory: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of this parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prompts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of prompts that the agent can present to the user in order to collect a value for the parameter."]
        pub prompts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The definition of the parameter value. It can be: - a constant string, - a parameter value defined as `$parameter_name`, - an original parameter value defined as `$parameter_name.original`, - a parameter value from some context defined as `#context_name.parameter_name`."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2IntentParameter {
        pub fn builder() -> GoogleCloudDialogflowV2IntentParameterBuilder {
            GoogleCloudDialogflowV2IntentParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an example that the agent is trained on."]
    pub struct GoogleCloudDialogflowV2IntentTrainingPhrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this training phrase."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set."]
        pub parts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentTrainingPhrasePart>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timesAddedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased."]
        pub times_added_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the training phrase."]
        pub _type: ::std::option::Option<GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum>,
    }
    impl GoogleCloudDialogflowV2IntentTrainingPhrase {
        pub fn builder() -> GoogleCloudDialogflowV2IntentTrainingPhraseBuilder {
            GoogleCloudDialogflowV2IntentTrainingPhraseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of the training phrase."]
    pub enum GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Not specified. This value should never be used."]
        TypeUnspecified,
        #[serde(rename = "EXAMPLE")]
        #[doc = "Examples do not contain @-prefixed entity type names, but example parts can be annotated with entity types."]
        Example,
        #[serde(rename = "TEMPLATE")]
        #[doc = "Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases that you've created in template mode, those will continue to work."]
        Template,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2IntentTrainingPhraseTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a part of a training phrase."]
    pub struct GoogleCloudDialogflowV2IntentTrainingPhrasePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The parameter name for the value extracted from the annotated part of the example. This field is required for annotated parts of the training phrase."]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The entity type name prefixed with `@`. This field is required for annotated parts of the training phrase."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text for this part."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userDefined")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the text was manually annotated. This field is set to true when the Dialogflow Console is used to manually annotate the part. When creating an annotated part with the API, you must set this to true."]
        pub user_defined: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowV2IntentTrainingPhrasePart {
        pub fn builder() -> GoogleCloudDialogflowV2IntentTrainingPhrasePartBuilder {
            GoogleCloudDialogflowV2IntentTrainingPhrasePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a message posted into a conversation."]
    pub struct GoogleCloudDialogflowV2Message {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "content")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The message content."]
        pub content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The time when the message was created."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The message language. This should be a [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. Example: \"en-US\"."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messageAnnotation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The annotation for the message."]
        pub message_annotation:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2MessageAnnotation>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the message. Format: `projects//locations//conversations//messages/`."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "participant")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The participant that sends this message."]
        pub participant: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "participantRole")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The role of the participant."]
        pub participant_role:
            ::std::option::Option<GoogleCloudDialogflowV2MessageParticipantRoleEnum>,
    }
    impl GoogleCloudDialogflowV2Message {
        pub fn builder() -> GoogleCloudDialogflowV2MessageBuilder {
            GoogleCloudDialogflowV2MessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The role of the participant."]
    pub enum GoogleCloudDialogflowV2MessageParticipantRoleEnum {
        #[serde(rename = "ROLE_UNSPECIFIED")]
        #[doc = "Participant role not set."]
        RoleUnspecified,
        #[serde(rename = "HUMAN_AGENT")]
        #[doc = "Participant is a human agent."]
        HumanAgent,
        #[serde(rename = "AUTOMATED_AGENT")]
        #[doc = "Participant is an automated agent, such as a Dialogflow agent."]
        AutomatedAgent,
        #[serde(rename = "END_USER")]
        #[doc = "Participant is an end user that has called or chatted with Dialogflow services."]
        EndUser,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2MessageParticipantRoleEnum {
        fn default() -> Self {
            Self::RoleUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of annotation for the message."]
    pub struct GoogleCloudDialogflowV2MessageAnnotation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "containEntities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates whether the text message contains entities."]
        pub contain_entities: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of annotated message parts ordered by their position in the message. You can recover the annotated message by concatenating [AnnotatedMessagePart.text]."]
        pub parts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2AnnotatedMessagePart>>,
        >,
    }
    impl GoogleCloudDialogflowV2MessageAnnotation {
        pub fn builder() -> GoogleCloudDialogflowV2MessageAnnotationBuilder {
            GoogleCloudDialogflowV2MessageAnnotationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the contents of the original request that was passed to the `[Streaming]DetectIntent` call."]
    pub struct GoogleCloudDialogflowV2OriginalDetectIntentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This field is set to the value of the `QueryParameters.payload` field passed in the request. Some integrations that query a Dialogflow agent may provide additional information in the payload. In particular, for the Dialogflow Phone Gateway integration, this field has the form: { \"telephony\": { \"caller_id\": \"+18558363987\" } } Note: The caller ID field (`caller_id`) will be redacted for Trial Edition agents and populated with the caller ID in [E.164 format](https://en.wikipedia.org/wiki/E.164) for Essentials Edition agents."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set by Dialogflow-owned servers."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The version of the protocol used for this request. This field is AoG-specific."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2OriginalDetectIntentRequest {
        pub fn builder() -> GoogleCloudDialogflowV2OriginalDetectIntentRequestBuilder {
            GoogleCloudDialogflowV2OriginalDetectIntentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of conversational query or event processing."]
    pub struct GoogleCloudDialogflowV2QueryResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action name from the matched intent."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allRequiredParamsPresent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is set to: - `false` if the matched intent has required parameters and not all of the required parameter values have been collected. - `true` if all required parameter values have been collected, or if the matched intent doesn't contain any required parameters."]
        pub all_required_params_present: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnosticInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form diagnostic information for the associated detect intent request. The fields of this data can change without notice, so you should not write code that depends on its structure. The data may contain: - webhook call latency - webhook errors"]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of rich messages to present to the user."]
        pub fulfillment_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred."]
        pub fulfillment_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`."]
        pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Intent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentDetectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list."]
        pub intent_detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input."]
        pub query_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentAnalysisResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request."]
        pub sentiment_analysis_result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2SentimentAnalysisResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechRecognitionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. The default of 0.0 is a sentinel value indicating that confidence was not set. This field is not guaranteed to be accurate or set. In particular this field isn't set for StreamingDetectIntent since the streaming endpoint has separate confidence estimates per portion of the audio in StreamingRecognitionResult."]
        pub speech_recognition_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `payload` field returned in the webhook response."]
        pub webhook_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `source` field returned in the webhook response."]
        pub webhook_source: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2QueryResult {
        pub fn builder() -> GoogleCloudDialogflowV2QueryResultBuilder {
            GoogleCloudDialogflowV2QueryResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text."]
    pub struct GoogleCloudDialogflowV2Sentiment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowV2Sentiment {
        pub fn builder() -> GoogleCloudDialogflowV2SentimentBuilder {
            GoogleCloudDialogflowV2SentimentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config"]
    pub struct GoogleCloudDialogflowV2SentimentAnalysisResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryTextSentiment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result for `query_text`."]
        pub query_text_sentiment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2Sentiment>>,
    }
    impl GoogleCloudDialogflowV2SentimentAnalysisResult {
        pub fn builder() -> GoogleCloudDialogflowV2SentimentAnalysisResultBuilder {
            GoogleCloudDialogflowV2SentimentAnalysisResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A session represents a conversation between a Dialogflow agent and an end-user. You can create special entities, called session entities, during a session. Session entities can extend or replace custom entity types and only exist during the session that they were created for. All session data, including session entities, is stored by Dialogflow for 20 minutes. For more information, see the [session entity guide](https://cloud.google.com/dialogflow/docs/entities-session)."]
    pub struct GoogleCloudDialogflowV2SessionEntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The collection of entities associated with this session entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityOverrideMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
        pub entity_override_mode:
            ::std::option::Option<GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of this session entity type. Format: `projects//agent/sessions//entityTypes/`, or `projects//agent/environments//users//sessions//entityTypes/`. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2SessionEntityType {
        pub fn builder() -> GoogleCloudDialogflowV2SessionEntityTypeBuilder {
            GoogleCloudDialogflowV2SessionEntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
    pub enum GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
        #[serde(rename = "ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        EntityOverrideModeUnspecified,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_OVERRIDE")]
        #[doc = "The collection of session entities overrides the collection of entities in the corresponding custom entity type."]
        EntityOverrideModeOverride,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
        #[doc = "The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge."]
        EntityOverrideModeSupplement,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2SessionEntityTypeEntityOverrideModeEnum {
        fn default() -> Self {
            Self::EntityOverrideModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for a webhook call."]
    pub struct GoogleCloudDialogflowV2WebhookRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalDetectIntentRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The contents of the original request that was passed to `[Streaming]DetectIntent` call."]
        pub original_detect_intent_request: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2OriginalDetectIntentRequest>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the conversational query or event processing. Contains the same value as `[Streaming]DetectIntentResponse.query_result`."]
        pub query_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2QueryResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the response. Contains the same value as `[Streaming]DetectIntentResponse.response_id`."]
        pub response_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of detectIntent request session. Can be used to identify end-user inside webhook implementation. Format: `projects//agent/sessions/`, or `projects//agent/environments//users//sessions/`."]
        pub session: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2WebhookRequest {
        pub fn builder() -> GoogleCloudDialogflowV2WebhookRequestBuilder {
            GoogleCloudDialogflowV2WebhookRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for a webhook call. This response is validated by the Dialogflow server. If validation fails, an error will be returned in the QueryResult.diagnostic_info field. Setting JSON fields to an empty value with the wrong type is a common error. To avoid this error: - Use `\"\"` for empty strings - Use `{}` or `null` for empty objects - Use `[]` or `null` for empty arrays For more information, see the [Protocol Buffers Language Guide](https://developers.google.com/protocol-buffers/docs/proto3#json)."]
    pub struct GoogleCloudDialogflowV2WebhookResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupEventInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Invokes the supplied events. When this field is set, Dialogflow ignores the `fulfillment_text`, `fulfillment_messages`, and `payload` fields."]
        pub followup_event_input:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2EventInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The rich response messages intended for the end-user. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_messages sent to the integration or API caller."]
        pub fulfillment_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text response message intended for the end-user. It is recommended to use `fulfillment_messages.text.text[0]` instead. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_text sent to the integration or API caller."]
        pub fulfillment_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of output contexts that will overwrite currently active contexts for the session and reset their lifespans. When provided, Dialogflow uses this field to populate QueryResult.output_contexts sent to the integration or API caller."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This field can be used to pass custom data from your webhook to the integration or API caller. Arbitrary JSON objects are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_payload sent to the integration or API caller. This field is also used by the [Google Assistant integration](https://cloud.google.com/dialogflow/docs/integrations/aog) for rich response messages. See the format definition at [Google Assistant Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionEntityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session. Setting this data from a webhook overwrites the session entity types that have been set using `detectIntent`, `streamingDetectIntent` or SessionEntityType management methods."]
        pub session_entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2SessionEntityType>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A custom field used to identify the webhook source. Arbitrary strings are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_source sent to the integration or API caller."]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2WebhookResponse {
        pub fn builder() -> GoogleCloudDialogflowV2WebhookResponseBuilder {
            GoogleCloudDialogflowV2WebhookResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for EntityTypes.BatchUpdateEntityTypes."]
    pub struct GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of updated or created entity types."]
        pub entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityType>>,
        >,
    }
    impl GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesResponse {
        pub fn builder() -> GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesResponseBuilder {
            GoogleCloudDialogflowV2beta1BatchUpdateEntityTypesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Intents.BatchUpdateIntents."]
    pub struct GoogleCloudDialogflowV2beta1BatchUpdateIntentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of updated or created intents."]
        pub intents: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
        >,
    }
    impl GoogleCloudDialogflowV2beta1BatchUpdateIntentsResponse {
        pub fn builder() -> GoogleCloudDialogflowV2beta1BatchUpdateIntentsResponseBuilder {
            GoogleCloudDialogflowV2beta1BatchUpdateIntentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Dialogflow contexts are similar to natural language context. If a person says to you \"they are orange\", you need context in order to understand what \"they\" is referring to. Similarly, for Dialogflow to handle an end-user expression like that, it needs to be provided with context in order to correctly match an intent. Using contexts, you can control the flow of a conversation. You can configure contexts for an intent by setting input and output contexts, which are identified by string names. When an intent is matched, any configured output contexts for that intent become active. While any contexts are active, Dialogflow is more likely to match intents that are configured with input contexts that correspond to the currently active contexts. For more information about context, see the [Contexts guide](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
    pub struct GoogleCloudDialogflowV2beta1Context {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "lifespanCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The number of conversational query requests after which the context expires. The default is `0`. If set to `0`, the context expires immediately. Contexts expire automatically after 20 minutes if there are no matching queries."]
        pub lifespan_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the context. Supported formats: - `projects//agent/sessions//contexts/`, - `projects//locations//agent/sessions//contexts/`, - `projects//agent/environments//users//sessions//contexts/`, - `projects//locations//agent/environments//users//sessions//contexts/`, The `Context ID` is always converted to lowercase, may only contain characters in a-zA-Z0-9_-% and may be at most 250 bytes long. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. The following context names are reserved for internal use by Dialogflow. You should not use these contexts or create contexts with these names: * `__system_counters__` * `*_id_dialog_context` * `*_dialog_params_size`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of parameters associated with this context. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowV2beta1Context {
        pub fn builder() -> GoogleCloudDialogflowV2beta1ContextBuilder {
            GoogleCloudDialogflowV2beta1ContextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Each intent parameter has a type, called the entity type, which dictates exactly how data from an end-user expression is extracted. Dialogflow provides predefined system entities that can match many common types of data. For example, there are system entities for matching dates, times, colors, email addresses, and so on. You can also create your own custom entities for matching custom data. For example, you could define a vegetable entity that can match the types of vegetables available for purchase with a grocery store agent. For more information, see the [Entity guide](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct GoogleCloudDialogflowV2beta1EntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "autoExpansionMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
        pub auto_expansion_mode:
            ::std::option::Option<GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the entity type."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enableFuzzyExtraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Enables fuzzy entity extraction during classification."]
        pub enable_fuzzy_extraction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of entity entries associated with the entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "kind")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates the kind of entity type."]
        pub kind: ::std::option::Option<GoogleCloudDialogflowV2beta1EntityTypeKindEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the entity type. Required for EntityTypes.UpdateEntityType and EntityTypes.BatchUpdateEntityTypes methods. Supported formats: - `projects//agent/entityTypes/` - `projects//locations//agent/entityTypes/`"]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1EntityType {
        pub fn builder() -> GoogleCloudDialogflowV2beta1EntityTypeBuilder {
            GoogleCloudDialogflowV2beta1EntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Indicates whether the entity type can be automatically expanded."]
    pub enum GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
        #[serde(rename = "AUTO_EXPANSION_MODE_UNSPECIFIED")]
        #[doc = "Auto expansion disabled for the entity."]
        AutoExpansionModeUnspecified,
        #[serde(rename = "AUTO_EXPANSION_MODE_DEFAULT")]
        #[doc = "Allows an agent to recognize values that have not been explicitly listed in the entity."]
        AutoExpansionModeDefault,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1EntityTypeAutoExpansionModeEnum {
        fn default() -> Self {
            Self::AutoExpansionModeUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates the kind of entity type."]
    pub enum GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
        #[serde(rename = "KIND_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        KindUnspecified,
        #[serde(rename = "KIND_MAP")]
        #[doc = "Map entity types allow mapping of a group of synonyms to a reference value."]
        KindMap,
        #[serde(rename = "KIND_LIST")]
        #[doc = "List entity types contain a set of entries that do not map to reference values. However, list entity types can contain references to other entity types (with or without aliases)."]
        KindList,
        #[serde(rename = "KIND_REGEXP")]
        #[doc = "Regexp entity types allow to specify regular expressions in entries values."]
        KindRegexp,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1EntityTypeKindEnum {
        fn default() -> Self {
            Self::KindUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An **entity entry** for an associated entity type."]
    pub struct GoogleCloudDialogflowV2beta1EntityTypeEntity {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A collection of value synonyms. For example, if the entity type is *vegetable*, and `value` is *scallions*, a synonym could be *green onions*. For `KIND_LIST` entity types: * This collection must contain exactly one synonym equal to `value`."]
        pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The primary value associated with this entity entry. For example, if the entity type is *vegetable*, the value could be *scallions*. For `KIND_MAP` entity types: * A reference value to be used in place of synonyms. For `KIND_LIST` entity types: * A string that can contain references to other entity types (with or without aliases)."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1EntityTypeEntity {
        pub fn builder() -> GoogleCloudDialogflowV2beta1EntityTypeEntityBuilder {
            GoogleCloudDialogflowV2beta1EntityTypeEntityBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Events allow for matching intents by event name instead of the natural language input. For instance, input `` can trigger a personalized welcome response. The parameter `name` may be used by the agent in the response: `\"Hello #welcome_event.name! What can I do for you today?\"`."]
    pub struct GoogleCloudDialogflowV2beta1EventInput {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The language of this query. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes. Note that queries in the same session do not necessarily need to specify the same language."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of the event."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of parameters associated with the event. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl GoogleCloudDialogflowV2beta1EventInput {
        pub fn builder() -> GoogleCloudDialogflowV2beta1EventInputBuilder {
            GoogleCloudDialogflowV2beta1EventInputBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Agents.ExportAgent."]
    pub struct GoogleCloudDialogflowV2beta1ExportAgentResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Zip compressed raw byte content for agent."]
        pub agent_content: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "agentUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The URI to a file containing the exported agent. This field is populated only if `agent_uri` is specified in `ExportAgentRequest`."]
        pub agent_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1ExportAgentResponse {
        pub fn builder() -> GoogleCloudDialogflowV2beta1ExportAgentResponseBuilder {
            GoogleCloudDialogflowV2beta1ExportAgentResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An intent categorizes an end-user's intention for one conversation turn. For each agent, you define many intents, where your combined intents can handle a complete conversation. When an end-user writes or says something, referred to as an end-user expression or end-user input, Dialogflow matches the end-user input to the best intent in your agent. Matching an intent is also known as intent classification. For more information, see the [intent guide](https://cloud.google.com/dialogflow/docs/intents-overview)."]
    pub struct GoogleCloudDialogflowV2beta1Intent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the action associated with the intent. Note: The action name must not contain whitespaces."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultResponsePlatforms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The list of platforms for which the first responses will be copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform)."]
        pub default_response_platforms: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of this intent."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endInteraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false."]
        pub end_interaction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "events")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of event names that trigger the intent. If the collection of input contexts is not empty, all of the contexts must be present in the active user session for an event to trigger this intent. Event names are limited to 150 characters."]
        pub events: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupIntentInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Information about all followup intents that have this intent as a direct or indirect parent. We populate this field only in the output."]
        pub followup_intent_info: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inputContextNames")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The list of context names required for this intent to be triggered. Formats: - `projects//agent/sessions/-/contexts/` - `projects//locations//agent/sessions/-/contexts/`"]
        pub input_context_names: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isFallback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether this is a fallback intent."]
        pub is_fallback: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "messages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of rich messages corresponding to the `Response` field in the Dialogflow console."]
        pub messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mlDisabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether Machine Learning is disabled for the intent. Note: If `ml_disabled` setting is set to true, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off."]
        pub ml_disabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mlEnabled")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether Machine Learning is enabled for the intent. Note: If `ml_enabled` setting is set to false, then this intent is not taken into account during inference in `ML ONLY` match mode. Also, auto-markup in the UI is turned off. DEPRECATED! Please use `ml_disabled` field instead. NOTE: If both `ml_enabled` and `ml_disabled` are either not set or false, then the default value is determined as follows: - Before April 15th, 2018 the default is: ml_enabled = false / ml_disabled = true. - After April 15th, 2018 the default is: ml_enabled = true / ml_disabled = false."]
        pub ml_enabled: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of this intent. Required for Intents.UpdateIntent and Intents.BatchUpdateIntents methods. Supported formats: - `projects//agent/intents/` - `projects//locations//agent/intents/`"]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of contexts that are activated when the intent is matched. Context messages in this collection should not set the parameters field. Setting the `lifespan_count` to 0 will reset the context when the intent is matched. Format: `projects//agent/sessions/-/contexts/`."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of parameters associated with the intent."]
        pub parameters: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentParameter>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The unique identifier of the parent intent in the chain of followup intents. You can set this field when creating an intent, for example with CreateIntent or BatchUpdateIntents, in order to make this intent a followup intent. It identifies the parent followup intent. Format: `projects//agent/intents/`."]
        pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "priority")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The priority of this intent. Higher numbers represent higher priorities. - If the supplied value is unspecified or 0, the service translates the value to 500,000, which corresponds to the `Normal` priority in the console. - If the supplied value is negative, the intent is ignored in runtime detect intent requests."]
        pub priority: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "resetContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether to delete all contexts in the current session when this intent is matched."]
        pub reset_contexts: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rootFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of the root intent in the chain of followup intents. It identifies the correct followup intents chain for this intent. Format: `projects//agent/intents/`."]
        pub root_followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "trainingPhrases")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of examples that the agent is trained on."]
        pub training_phrases: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentTrainingPhrase>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookState")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
        pub webhook_state:
            ::std::option::Option<GoogleCloudDialogflowV2beta1IntentWebhookStateEnum>,
    }
    impl GoogleCloudDialogflowV2beta1Intent {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentBuilder {
            GoogleCloudDialogflowV2beta1IntentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Not specified."]
        PlatformUnspecified,
        #[serde(rename = "FACEBOOK")]
        #[doc = "Facebook."]
        Facebook,
        #[serde(rename = "SLACK")]
        #[doc = "Slack."]
        Slack,
        #[serde(rename = "TELEGRAM")]
        #[doc = "Telegram."]
        Telegram,
        #[serde(rename = "KIK")]
        #[doc = "Kik."]
        Kik,
        #[serde(rename = "SKYPE")]
        #[doc = "Skype."]
        Skype,
        #[serde(rename = "LINE")]
        #[doc = "Line."]
        Line,
        #[serde(rename = "VIBER")]
        #[doc = "Viber."]
        Viber,
        #[serde(rename = "ACTIONS_ON_GOOGLE")]
        #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        ActionsOnGoogle,
        #[serde(rename = "TELEPHONY")]
        #[doc = "Telephony Gateway."]
        Telephony,
        #[serde(rename = "GOOGLE_HANGOUTS")]
        #[doc = "Google Hangouts."]
        GoogleHangouts,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1IntentDefaultResponsePlatformsEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Indicates whether webhooks are enabled for the intent."]
    pub enum GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
        #[serde(rename = "WEBHOOK_STATE_UNSPECIFIED")]
        #[doc = "Webhook is disabled in the agent and in the intent."]
        WebhookStateUnspecified,
        #[serde(rename = "WEBHOOK_STATE_ENABLED")]
        #[doc = "Webhook is enabled in the agent and in the intent."]
        WebhookStateEnabled,
        #[serde(rename = "WEBHOOK_STATE_ENABLED_FOR_SLOT_FILLING")]
        #[doc = "Webhook is enabled in the agent and in the intent. Also, each slot filling prompt is forwarded to the webhook."]
        WebhookStateEnabledForSlotFilling,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1IntentWebhookStateEnum {
        fn default() -> Self {
            Self::WebhookStateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a single followup intent in the chain."]
    pub struct GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the followup intent. Format: `projects//agent/intents/`."]
        pub followup_intent_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentFollowupIntentName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the followup intent's parent. Format: `projects//agent/intents/`."]
        pub parent_followup_intent_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentFollowupIntentInfo {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentFollowupIntentInfoBuilder {
            GoogleCloudDialogflowV2beta1IntentFollowupIntentInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Corresponds to the `Response` field in the Dialogflow console."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "basicCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays a basic card for Actions on Google."]
        pub basic_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "browseCarouselCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Browse carousel card for Actions on Google."]
        pub browse_carousel_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "card")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays a card."]
        pub card:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCard>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "carouselSelect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays a carousel card for Actions on Google."]
        pub carousel_select: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays an image."]
        pub image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkOutSuggestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays a link out suggestion chip for Actions on Google."]
        pub link_out_suggestion: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "listSelect")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays a list card for Actions on Google."]
        pub list_select: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageListSelect>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The media content card for Actions on Google."]
        pub media_content: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageMediaContent>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A custom platform-specific response."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "platform")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The platform that this message is intended for."]
        pub platform: ::std::option::Option<GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quickReplies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays quick replies."]
        pub quick_replies: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageQuickReplies>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rbmCarouselRichCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rich Business Messaging (RBM) carousel rich card response."]
        pub rbm_carousel_rich_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rbmStandaloneRichCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Standalone Rich Business Messaging (RBM) rich card response."]
        pub rbm_standalone_rich_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rbmText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Rich Business Messaging (RBM) text response. RBM allows businesses to send enriched and branded versions of SMS. See https://jibe.google.com/business-messaging."]
        pub rbm_text: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmText>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a voice or text-only response for Actions on Google."]
        pub simple_responses: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Displays suggestion chips for Actions on Google."]
        pub suggestions: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSuggestions>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableCard")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table card for Actions on Google."]
        pub table_card: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCard>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telephonyPlayAudio")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Plays audio from a file in Telephony Gateway."]
        pub telephony_play_audio: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telephonySynthesizeSpeech")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Synthesizes speech in Telephony Gateway."]
        pub telephony_synthesize_speech: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "telephonyTransferCall")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Transfers the call in Telephony Gateway."]
        pub telephony_transfer_call: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Returns a text response."]
        pub text:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageText>>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessage {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. The platform that this message is intended for."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
        #[serde(rename = "PLATFORM_UNSPECIFIED")]
        #[doc = "Not specified."]
        PlatformUnspecified,
        #[serde(rename = "FACEBOOK")]
        #[doc = "Facebook."]
        Facebook,
        #[serde(rename = "SLACK")]
        #[doc = "Slack."]
        Slack,
        #[serde(rename = "TELEGRAM")]
        #[doc = "Telegram."]
        Telegram,
        #[serde(rename = "KIK")]
        #[doc = "Kik."]
        Kik,
        #[serde(rename = "SKYPE")]
        #[doc = "Skype."]
        Skype,
        #[serde(rename = "LINE")]
        #[doc = "Line."]
        Line,
        #[serde(rename = "VIBER")]
        #[doc = "Viber."]
        Viber,
        #[serde(rename = "ACTIONS_ON_GOOGLE")]
        #[doc = "Google Assistant See [Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        ActionsOnGoogle,
        #[serde(rename = "TELEPHONY")]
        #[doc = "Telephony Gateway."]
        Telephony,
        #[serde(rename = "GOOGLE_HANGOUTS")]
        #[doc = "Google Hangouts."]
        GoogleHangouts,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1IntentMessagePlatformEnum {
        fn default() -> Self {
            Self::PlatformUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The basic card message. Useful for displaying information."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of card buttons."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "formattedText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required, unless image is present. The body text of the card."]
        pub formatted_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image for the card."]
        pub image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The subtitle of the card."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageBasicCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageBasicCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageBasicCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The button object that appears at the bottom of a card."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "openUriAction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Action to take when a user taps on the button."]
        pub open_uri_action: ::std::option::Option<
            ::std::boxed::Box<
                GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the button."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Opens the given URI."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The HTTP or HTTPS scheme URI."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriAction {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriActionBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageBasicCardButtonOpenUriActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Browse Carousel Card for Actions on Google. https://developers.google.com/actions/assistant/responses#browsing_carousel"]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "imageDisplayOptions")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Settings for displaying the image. Applies to every image in items."] pub image_display_options : :: std :: option :: Option < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "items")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. List of items in the Browse Carousel Card. Minimum of two items, maximum of ten."] pub items : :: std :: option :: Option < :: std :: vec :: Vec < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem > > > }
    impl GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Settings for displaying the image. Applies to every image in items."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum {
        #[serde(rename = "IMAGE_DISPLAY_OPTIONS_UNSPECIFIED")]
        #[doc = "Fill the gaps between the image and the image container with gray bars."]
        ImageDisplayOptionsUnspecified,
        #[serde(rename = "GRAY")]
        #[doc = "Fill the gaps between the image and the image container with gray bars."]
        Gray,
        #[serde(rename = "WHITE")]
        #[doc = "Fill the gaps between the image and the image container with white bars."]
        White,
        #[serde(rename = "CROPPED")]
        #[doc = "Image is scaled such that the image width and height match or exceed the container dimensions. This may crop the top and bottom of the image if the scaled image height is greater than the container height, or crop the left and right of the image if the scaled image width is greater than the container width. This is similar to \"Zoom Mode\" on a widescreen TV when playing a 4:3 video."]
        Cropped,
        #[serde(rename = "BLURRED_BACKGROUND")]
        #[doc = "Pad the gaps between image and image frame with a blurred copy of the same image."]
        BlurredBackground,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardImageDisplayOptionsEnum
    {
        fn default() -> Self {
            Self::ImageDisplayOptionsUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Browsing carousel tile"]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "description")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Description of the carousel item. Maximum of four lines of text."] pub description : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "footer")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Text that appears at the bottom of the Browse Carousel Card. Maximum of one line of text."] pub footer : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "image")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Hero image for the carousel item."] pub image : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageImage > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "openUriAction")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Action to present to the user."] pub open_uri_action : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "title")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. Title of the carousel item. Maximum of two lines of text."] pub title : :: std :: option :: Option < :: std :: string :: String > }
    impl GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItem {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemBuilder
        {
            GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Actions on Google action to open a given url."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "url")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Required. URL"] pub url : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "urlTypeHint")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."] pub url_type_hint : :: std :: option :: Option < GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum > }
    impl GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlAction { pub fn builder () -> GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionBuilder { GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionBuilder :: default () } }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Specifies the type of viewer that is used when opening the URL. Defaults to opening via web browser."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum
    {
        #[serde(rename = "URL_TYPE_HINT_UNSPECIFIED")]
        #[doc = "Unspecified"]
        UrlTypeHintUnspecified,
        #[serde(rename = "AMP_ACTION")]
        #[doc = "Url would be an amp action"]
        AmpAction,
        #[serde(rename = "AMP_CONTENT")]
        #[doc = "URL that points directly to AMP content, or to a canonical URL which refers to AMP content via ."]
        AmpContent,
    }
    impl :: std :: default :: Default for GoogleCloudDialogflowV2beta1IntentMessageBrowseCarouselCardBrowseCarouselCardItemOpenUrlActionUrlTypeHintEnum { fn default () -> Self { Self :: UrlTypeHintUnspecified } }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card response message."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of card buttons."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCardButton>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The public URI to an image file for the card."]
        pub image_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The subtitle of the card."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Optional. Contains information about a button."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageCardButton {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postback")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to send back to the Dialogflow API or a URI to open."]
        pub postback: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to show on the button."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageCardButton {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageCardButtonBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageCardButtonBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card for presenting a carousel of options to select from."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Carousel items."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem>,
            >,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageCarouselSelect {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An item in the carousel."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The body text of the card."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image to display."]
        pub image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Additional info about the option item."]
        pub info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Title of the carousel item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItem {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItemBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageCarouselSelectItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Column properties for TableCard."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageColumnProperties {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "header")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Column heading."]
        pub header: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "horizontalAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Defines text alignment for all cells in this column."]
        pub horizontal_alignment: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesHorizontalAlignmentEnum,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageColumnProperties {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. Defines text alignment for all cells in this column."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesHorizontalAlignmentEnum {
        #[serde(rename = "HORIZONTAL_ALIGNMENT_UNSPECIFIED")]
        #[doc = "Text is aligned to the leading edge of the column."]
        HorizontalAlignmentUnspecified,
        #[serde(rename = "LEADING")]
        #[doc = "Text is aligned to the leading edge of the column."]
        Leading,
        #[serde(rename = "CENTER")]
        #[doc = "Text is centered in the column."]
        Center,
        #[serde(rename = "TRAILING")]
        #[doc = "Text is aligned to the trailing edge of the column."]
        Trailing,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageColumnPropertiesHorizontalAlignmentEnum
    {
        fn default() -> Self {
            Self::HorizontalAlignmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The image response message."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageImage {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "accessibilityText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A text description of the image to be used for accessibility, e.g., screen readers. Required if image_uri is set for CarouselSelect."]
        pub accessibility_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "imageUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The public URI to an image file."]
        pub image_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageImage {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageImageBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageImageBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The suggestion chip message that allows the user to jump out to the app or website associated with this agent."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "destinationName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the app or site this chip is linking to."]
        pub destination_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The URI of the app or site to open when the user taps the suggestion chip."]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestion {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestionBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageLinkOutSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The card for presenting a list of options to select from."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelect {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "items")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List items."]
        pub items: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageListSelectItem>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Subtitle of the list."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The overall title of the list."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageListSelect {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageListSelectBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageListSelectBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An item in the list."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageListSelectItem {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The main text describing the item."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The image to display."]
        pub image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "info")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Additional information about this option."]
        pub info: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The title of the list item."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageListSelectItem {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageListSelectItemBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageListSelectItemBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The media content card for Actions on Google."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaObjects")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. List of media objects."]
        pub media_objects: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<
                    GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject,
                >,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mediaType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. What type of media is the content (ie \"audio\")."]
        pub media_type: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageMediaContent {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageMediaContentBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageMediaContentBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Optional. What type of media is the content (ie \"audio\")."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum {
        #[serde(rename = "RESPONSE_MEDIA_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified."]
        ResponseMediaTypeUnspecified,
        #[serde(rename = "AUDIO")]
        #[doc = "Response media type is audio."]
        Audio,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageMediaContentMediaTypeEnum
    {
        fn default() -> Self {
            Self::ResponseMediaTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response media object for media content card."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "contentUrl")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Url where the media is stored."]
        pub content_url: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of media card."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "icon")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Icon to display above media content."]
        pub icon: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "largeImage")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Image to display above media content."]
        pub large_image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of media card."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObject {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObjectBuilder
        {
            GoogleCloudDialogflowV2beta1IntentMessageMediaContentResponseMediaObjectBuilder::default(
            )
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The quick replies response message."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageQuickReplies {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "quickReplies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of quick replies."]
        pub quick_replies: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The title of the collection of quick replies."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageQuickReplies {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageQuickRepliesBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageQuickRepliesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) Card content"]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the card (at most 2000 bytes). At least one of the title, description or media must be set."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "media")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. However at least one of the title, description or media must be set. Media (image, GIF or a video) to include in the card."]
        pub media: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of suggestions to include in the card."]
        pub suggestions: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title of the card (at most 200 bytes). At least one of the title, description or media must be set."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) Media displayed in Cards The following media-types are currently supported: Image Types * image/jpeg * image/jpg' * image/gif * image/png Video Types * video/h263 * video/m4v * video/mp4 * video/mpeg * video/mpeg4 * video/webm"]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fileUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Publicly reachable URI of the file. The RBM platform determines the MIME type of the file from the content-type field in the HTTP headers when the platform fetches the file. The content-type field must be present and accurate in the HTTP response from the URL."]
        pub file_uri: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "height")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored."]
        pub height: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Publicly reachable URI of the thumbnail.If you don't provide a thumbnail URI, the RBM platform displays a blank placeholder thumbnail until the user's device downloads the file. Depending on the user's setting, the file may not download automatically and may require the user to tap a download button."]
        pub thumbnail_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMedia {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required for cards with vertical orientation. The height of the media within a rich card with a vertical layout. For a standalone card with horizontal layout, height is not customizable, and this field is ignored."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum {
        #[serde(rename = "HEIGHT_UNSPECIFIED")]
        #[doc = "Not specified."]
        HeightUnspecified,
        #[serde(rename = "SHORT")]
        #[doc = "112 DP."]
        Short,
        #[serde(rename = "MEDIUM")]
        #[doc = "168 DP."]
        Medium,
        #[serde(rename = "TALL")]
        #[doc = "264 DP. Not available for rich card carousels when the card width is set to small."]
        Tall,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageRbmCardContentRbmMediaHeightEnum
    {
        fn default() -> Self {
            Self::HeightUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Carousel Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. If you want to show a single card with more control over the layout, please use RbmStandaloneCard instead."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardContents")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The cards in the carousel. A carousel must have at least 2 cards and at most 10."]
        pub card_contents: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardWidth")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The width of the cards in the carousel."]
        pub card_width: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The width of the cards in the carousel."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum {
        #[serde(rename = "CARD_WIDTH_UNSPECIFIED")]
        #[doc = "Not specified."]
        CardWidthUnspecified,
        #[serde(rename = "SMALL")]
        #[doc = "120 DP. Note that tall media cannot be used."]
        Small,
        #[serde(rename = "MEDIUM")]
        #[doc = "232 DP."]
        Medium,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageRbmCarouselCardCardWidthEnum
    {
        fn default() -> Self {
            Self::CardWidthUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Standalone Rich Business Messaging (RBM) rich card. Rich cards allow you to respond to users with more vivid content, e.g. with media and suggestions. You can group multiple rich cards into one using RbmCarouselCard but carousel cards will give you less control over the card layout."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardContent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Card content."]
        pub card_content: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmCardContent>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cardOrientation")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Orientation of the card."]
        pub card_orientation: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "thumbnailImageAlignment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout."]
        pub thumbnail_image_alignment: ::std::option::Option<
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Orientation of the card."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum {
        #[serde(rename = "CARD_ORIENTATION_UNSPECIFIED")]
        #[doc = "Not specified."]
        CardOrientationUnspecified,
        #[serde(rename = "HORIZONTAL")]
        #[doc = "Horizontal layout."]
        Horizontal,
        #[serde(rename = "VERTICAL")]
        #[doc = "Vertical layout."]
        Vertical,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardCardOrientationEnum
    {
        fn default() -> Self {
            Self::CardOrientationUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required if orientation is horizontal. Image preview alignment for standalone cards with horizontal layout."]
    pub enum GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum {
        #[serde(rename = "THUMBNAIL_IMAGE_ALIGNMENT_UNSPECIFIED")]
        #[doc = "Not specified."]
        ThumbnailImageAlignmentUnspecified,
        #[serde(rename = "LEFT")]
        #[doc = "Thumbnail preview is left-aligned."]
        Left,
        #[serde(rename = "RIGHT")]
        #[doc = "Thumbnail preview is right-aligned."]
        Right,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1IntentMessageRbmStandaloneCardThumbnailImageAlignmentEnum
    {
        fn default() -> Self {
            Self::ThumbnailImageAlignmentUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) suggested client-side action that the user can choose from the card."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction { # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "dial")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Dial a phone number"] pub dial : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "openUrl")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Open a URI on device"] pub open_url : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "postbackData")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Opaque payload that the Dialogflow receives in a user event when the user taps the suggested action. This data will be also forwarded to webhook to allow performing custom business logic."] pub postback_data : :: std :: option :: Option < :: std :: string :: String > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "shareLocation")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Suggested client side action: Share user location"] pub share_location : :: std :: option :: Option < :: std :: boxed :: Box < GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation > > , # [builder (default = "{ ::std::default::Default::default() }" , setter (into))] # [serde (rename = "text")] # [serde (skip_serializing_if = "::std::option::Option::is_none")] # [doc = "Text to display alongside the action."] pub text : :: std :: option :: Option < :: std :: string :: String > }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Opens the user's default dialer app with the specified phone number but does not dial automatically."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The phone number to fill in the default dialer app. This field should be in [E.164](https://en.wikipedia.org/wiki/E.164) format. An example of a correctly formatted phone number: +15556767888."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDial {
        pub fn builder(
        ) -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDialBuilder
        {
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionDialBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Opens the user's default web browser app to the specified uri If the user has an app installed that is registered as the default handler for the URL, then this app will be opened instead, and its icon will be used in the suggested action UI."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "uri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The uri to open on the user device"]
        pub uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUri {
        pub fn builder () -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUriBuilder{
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionOpenUriBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Opens the device's location chooser so the user can pick a location to send back to the agent."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation
    {}
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocation {
        pub fn builder () -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocationBuilder{
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedActionRbmSuggestedActionShareLocationBuilder :: default ()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) suggested reply that the user can click instead of typing in their own response."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "postbackData")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Opaque payload that the Dialogflow receives in a user event when the user taps the suggested reply. This data will be also forwarded to webhook to allow performing custom business logic."]
        pub postback_data: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Suggested reply text."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReplyBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReplyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) suggestion. Suggestions allow user to easily select/click a predefined response or perform an action (like opening a web uri)."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Predefined client side actions that user can choose"]
        pub action: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedAction>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "reply")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Predefined replies for user to select instead of typing"]
        pub reply: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestedReply>,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestionBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Rich Business Messaging (RBM) text response with suggestions."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageRbmText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rbmSuggestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. One or more suggestions to show to the user."]
        pub rbm_suggestion: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageRbmSuggestion>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Text sent and displayed to the user."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageRbmText {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageRbmTextBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageRbmTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Additional info about the select item for when it is triggered in a dialog."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "key")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. A unique key that will be sent back to the agent if this response is given."]
        pub key: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "synonyms")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of synonyms that can also be used to trigger this item in dialog."]
        pub synonyms: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfo {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfoBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageSelectItemInfoBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The simple response message containing speech or text."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text to display."]
        pub display_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of text_to_speech or ssml must be provided. Structured spoken response to the user in the SSML format. Mutually exclusive with text_to_speech."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "textToSpeech")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "One of text_to_speech or ssml must be provided. The plain text of the speech output. Mutually exclusive with ssml."]
        pub text_to_speech: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageSimpleResponseBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageSimpleResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The collection of simple response candidates. This message in `QueryResult.fulfillment_messages` and `WebhookResponse.fulfillment_messages` should contain only one `SimpleResponse`."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "simpleResponses")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of simple responses."]
        pub simple_responses: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSimpleResponse>,
            >,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageSimpleResponses {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageSimpleResponsesBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageSimpleResponsesBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The suggestion chip message that the user can tap to quickly post a reply to the conversation."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestion {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text shown the in the suggestion chip."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageSuggestion {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageSuggestionBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageSuggestionBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The collection of suggestions."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageSuggestions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "suggestions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The list of suggested replies."]
        pub suggestions: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageSuggestion>>,
        >,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageSuggestions {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageSuggestionsBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageSuggestionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Table card for Actions on Google."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCard {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "buttons")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of buttons for the card."]
        pub buttons: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageBasicCardButton>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columnProperties")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Display properties for the columns in this table."]
        pub column_properties: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageColumnProperties>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "image")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Image which should be displayed on the card."]
        pub image: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageImage>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "rows")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Rows in this table of data."]
        pub rows: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCardRow>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subtitle")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Subtitle to the title."]
        pub subtitle: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Title of the card."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTableCard {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTableCardBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTableCardBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Cell of TableCardRow."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardCell {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Text in this cell."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTableCardCell {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTableCardCellBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTableCardCellBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Row of TableCard."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTableCardRow {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "cells")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. List of cells that make up this row."]
        pub cells: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessageTableCardCell>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dividerAfter")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Whether to add a visual divider after this row."]
        pub divider_after: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTableCardRow {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTableCardRowBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTableCardRowBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Plays audio from a file in Telephony Gateway."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "audioUri")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. URI to a Google Cloud Storage object containing the audio to play, e.g., \"gs://bucket/object\". The object must contain a single channel (mono) of linear PCM audio (2 bytes / sample) at 8kHz. This object must be readable by the `service-@gcp-sa-dialogflow.iam.gserviceaccount.com` service account where is the number of the Telephony Gateway project (usually the same as the Dialogflow agent project). If the Google Cloud Storage bucket is in the Telephony Gateway project, this permission is added by default when enabling the Dialogflow V2 API. For audio from other sources, consider using the `TelephonySynthesizeSpeech` message with SSML."]
        pub audio_uri: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudio {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudioBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTelephonyPlayAudioBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Synthesizes speech and plays back the synthesized audio to the caller in Telephony Gateway. Telephony Gateway takes the synthesizer settings from `DetectIntentResponse.output_audio_config` which can either be set at request-level or can come from the agent-level synthesizer config."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "ssml")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The SSML to be synthesized. For more information, see [SSML](https://developers.google.com/actions/reference/ssml)."]
        pub ssml: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The raw text to be synthesized."]
        pub text: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeech {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeechBuilder
        {
            GoogleCloudDialogflowV2beta1IntentMessageTelephonySynthesizeSpeechBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Transfers the call in Telephony Gateway."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "phoneNumber")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The phone number to transfer the call to in [E.164 format](https://en.wikipedia.org/wiki/E.164). We currently only allow transferring to US numbers (+1xxxyyyzzzz)."]
        pub phone_number: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCall {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCallBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTelephonyTransferCallBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The text response message."]
    pub struct GoogleCloudDialogflowV2beta1IntentMessageText {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of the agent's responses."]
        pub text: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDialogflowV2beta1IntentMessageText {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentMessageTextBuilder {
            GoogleCloudDialogflowV2beta1IntentMessageTextBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents intent parameters."]
    pub struct GoogleCloudDialogflowV2beta1IntentParameter {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "defaultValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The default value to use when the `value` yields an empty result. Default values can be extracted from contexts by using the following syntax: `#context_name.parameter_name`."]
        pub default_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The name of the parameter."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityTypeDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The name of the entity type, prefixed with `@`, that describes values of the parameter. If the parameter is required, this must be provided."]
        pub entity_type_display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isList")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the parameter represents a list of values."]
        pub is_list: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mandatory")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the parameter is required. That is, whether the intent cannot be completed without collecting the parameter value."]
        pub mandatory: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of this parameter."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "prompts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of prompts that the agent can present to the user in order to collect a value for the parameter."]
        pub prompts: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "value")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The definition of the parameter value. It can be: - a constant string, - a parameter value defined as `$parameter_name`, - an original parameter value defined as `$parameter_name.original`, - a parameter value from some context defined as `#context_name.parameter_name`."]
        pub value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1IntentParameter {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentParameterBuilder {
            GoogleCloudDialogflowV2beta1IntentParameterBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents an example that the agent is trained on."]
    pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrase {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The unique identifier of this training phrase."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The ordered list of training phrase parts. The parts are concatenated in order to form the training phrase. Note: The API does not automatically annotate training phrases like the Dialogflow Console does. Note: Do not forget to include whitespace at part boundaries, so the training phrase is well formatted when the parts are concatenated. If the training phrase does not need to be annotated with parameters, you just need a single part with only the Part.text field set. If you want to annotate the training phrase, you must create multiple parts, where the fields of each part are populated in one of two ways: - `Part.text` is set to a part of the phrase that has no parameters. - `Part.text` is set to a part of the phrase that you want to annotate, and the `entity_type`, `alias`, and `user_defined` fields are all set."]
        pub parts: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timesAddedCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates how many times this example was added to the intent. Each time a developer adds an existing sample by editing an intent or training, this counter is increased."]
        pub times_added_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of the training phrase."]
        pub _type: ::std::option::Option<GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum>,
    }
    impl GoogleCloudDialogflowV2beta1IntentTrainingPhrase {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentTrainingPhraseBuilder {
            GoogleCloudDialogflowV2beta1IntentTrainingPhraseBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. The type of the training phrase."]
    pub enum GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
        #[serde(rename = "TYPE_UNSPECIFIED")]
        #[doc = "Not specified. This value should never be used."]
        TypeUnspecified,
        #[serde(rename = "EXAMPLE")]
        #[doc = "Examples do not contain @-prefixed entity type names, but example parts can be annotated with entity types."]
        Example,
        #[serde(rename = "TEMPLATE")]
        #[doc = "Templates are not annotated with entity types, but they can contain @-prefixed entity type names as substrings. Template mode has been deprecated. Example mode is the only supported way to create new training phrases. If you have existing training phrases that you've created in template mode, those will continue to work."]
        Template,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1IntentTrainingPhraseTypeEnum {
        fn default() -> Self {
            Self::TypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a part of a training phrase."]
    pub struct GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alias")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The parameter name for the value extracted from the annotated part of the example. This field is required for annotated parts of the training phrase."]
        pub alias: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The entity type name prefixed with `@`. This field is required for annotated parts of the training phrase."]
        pub entity_type: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "text")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The text for this part."]
        pub text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userDefined")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates whether the text was manually annotated. This field is set to true when the Dialogflow Console is used to manually annotate the part. When creating an annotated part with the API, you must set this to true."]
        pub user_defined: ::std::option::Option<::std::primitive::bool>,
    }
    impl GoogleCloudDialogflowV2beta1IntentTrainingPhrasePart {
        pub fn builder() -> GoogleCloudDialogflowV2beta1IntentTrainingPhrasePartBuilder {
            GoogleCloudDialogflowV2beta1IntentTrainingPhrasePartBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of querying a Knowledge base."]
    pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswers {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "answers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of answers from Knowledge Connector."]
        pub answers: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer>>,
        >,
    }
    impl GoogleCloudDialogflowV2beta1KnowledgeAnswers {
        pub fn builder() -> GoogleCloudDialogflowV2beta1KnowledgeAnswersBuilder {
            GoogleCloudDialogflowV2beta1KnowledgeAnswersBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An answer from Knowledge Connector."]
    pub struct GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "answer")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The piece of text from the `source` knowledge base document that answers this conversational query."]
        pub answer: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "faqQuestion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The corresponding FAQ question if the answer was extracted from a FAQ Document, empty otherwise."]
        pub faq_question: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system's confidence score that this Knowledge answer is a good match for this conversational query. The range is from 0.0 (completely uncertain) to 1.0 (completely certain). Note: The confidence score is likely to vary somewhat (possibly even for identical requests), as the underlying model is under constant improvement. It may be deprecated in the future. We recommend using `match_confidence_level` which should be generally more stable."]
        pub match_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "matchConfidenceLevel")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer."]
        pub match_confidence_level: ::std::option::Option<
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Indicates which Knowledge Document this answer was extracted from. Format: `projects//knowledgeBases//documents/`."]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswer {
        pub fn builder() -> GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerBuilder {
            GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The system's confidence level that this knowledge answer is a good match for this conversational query. NOTE: The confidence level for a given `` pair may change without notice, as it depends on models that are constantly being improved. However, it will change less frequently than the confidence score below, and should be preferred for referencing the quality of an answer."]
    pub enum GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum {
        #[serde(rename = "MATCH_CONFIDENCE_LEVEL_UNSPECIFIED")]
        #[doc = "Not specified."]
        MatchConfidenceLevelUnspecified,
        #[serde(rename = "LOW")]
        #[doc = "Indicates that the confidence is low."]
        Low,
        #[serde(rename = "MEDIUM")]
        #[doc = "Indicates our confidence is medium."]
        Medium,
        #[serde(rename = "HIGH")]
        #[doc = "Indicates our confidence is high."]
        High,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1KnowledgeAnswersAnswerMatchConfidenceLevelEnum
    {
        fn default() -> Self {
            Self::MatchConfidenceLevelUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
    pub struct GoogleCloudDialogflowV2beta1KnowledgeOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Output only. The current state of this operation."]
        pub state:
            ::std::option::Option<GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataStateEnum>,
    }
    impl GoogleCloudDialogflowV2beta1KnowledgeOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataBuilder {
            GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Output only. The current state of this operation."]
    pub enum GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
    }
    impl ::std::default::Default for GoogleCloudDialogflowV2beta1KnowledgeOperationMetadataStateEnum {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the contents of the original request that was passed to the `[Streaming]DetectIntent` call."]
    pub struct GoogleCloudDialogflowV2beta1OriginalDetectIntentRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This field is set to the value of the `QueryParameters.payload` field passed in the request. Some integrations that query a Dialogflow agent may provide additional information in the payload. In particular, for the Dialogflow Phone Gateway integration, this field has the form: { \"telephony\": { \"caller_id\": \"+18558363987\" } } Note: The caller ID field (`caller_id`) will be redacted for Trial Edition agents and populated with the caller ID in [E.164 format](https://en.wikipedia.org/wiki/E.164) for Essentials Edition agents."]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The source of this request, e.g., `google`, `facebook`, `slack`. It is set by Dialogflow-owned servers."]
        pub source: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The version of the protocol used for this request. This field is AoG-specific."]
        pub version: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1OriginalDetectIntentRequest {
        pub fn builder() -> GoogleCloudDialogflowV2beta1OriginalDetectIntentRequestBuilder {
            GoogleCloudDialogflowV2beta1OriginalDetectIntentRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents the result of conversational query or event processing."]
    pub struct GoogleCloudDialogflowV2beta1QueryResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "action")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The action name from the matched intent."]
        pub action: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allRequiredParamsPresent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field is set to: - `false` if the matched intent has required parameters and not all of the required parameter values have been collected. - `true` if all required parameter values have been collected, or if the matched intent doesn't contain any required parameters."]
        pub all_required_params_present: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "diagnosticInfo")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Free-form diagnostic information for the associated detect intent request. The fields of this data can change without notice, so you should not write code that depends on its structure. The data may contain: - webhook call latency - webhook errors"]
        pub diagnostic_info:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of rich messages to present to the user."]
        pub fulfillment_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The text to be pronounced to the user or shown on the screen. Note: This is a legacy field, `fulfillment_messages` should be preferred."]
        pub fulfillment_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intent")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent that matched the conversational query. Some, not all fields are filled in this message, including but not limited to: `name`, `display_name`, `end_interaction` and `is_fallback`."]
        pub intent: ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1Intent>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "intentDetectionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The intent detection confidence. Values range from 0.0 (completely uncertain) to 1.0 (completely certain). This value is for informational purpose only and is only used to help match the best intent within the classification threshold. This value may change for the same end-user expression at any time due to a model retraining or change in implementation. If there are `multiple knowledge_answers` messages, this value is set to the greatest `knowledgeAnswers.match_confidence` value in the list."]
        pub intent_detection_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "knowledgeAnswers")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result from Knowledge Connector (if any), ordered by decreasing `KnowledgeAnswers.match_confidence`."]
        pub knowledge_answers:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1KnowledgeAnswers>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "languageCode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The language that was triggered during intent detection. See [Language Support](https://cloud.google.com/dialogflow/docs/reference/language) for a list of the currently supported language codes."]
        pub language_code: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of output contexts. If applicable, `output_contexts.parameters` contains entries with name `.original` containing the original parameter values before the query."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parameters")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The collection of extracted parameters. Depending on your protocol or client library language, this is a map, associative array, symbol table, dictionary, or JSON object composed of a collection of (MapKey, MapValue) pairs: - MapKey type: string - MapKey value: parameter name - MapValue type: - If parameter's entity type is a composite entity: map - Else: depending on parameter value type, could be one of string, number, boolean, null, list or map - MapValue value: - If parameter's entity type is a composite entity: map from composite entity property names to property values - Else: parameter value"]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The original conversational query text: - If natural language text was provided as input, `query_text` contains a copy of the input. - If natural language speech audio was provided as input, `query_text` contains the speech recognition result. If speech recognizer produced multiple alternatives, a particular one is picked. - If automatic spell correction is enabled, `query_text` will contain the corrected user input."]
        pub query_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sentimentAnalysisResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result, which depends on the `sentiment_analysis_request_config` specified in the request."]
        pub sentiment_analysis_result: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1SentimentAnalysisResult>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "speechRecognitionConfidence")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The Speech recognition confidence between 0.0 and 1.0. A higher number indicates an estimated greater likelihood that the recognized words are correct. The default of 0.0 is a sentinel value indicating that confidence was not set. This field is not guaranteed to be accurate or set. In particular this field isn't set for StreamingDetectIntent since the streaming endpoint has separate confidence estimates per portion of the audio in StreamingRecognitionResult."]
        pub speech_recognition_confidence: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookPayload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `payload` field returned in the webhook response."]
        pub webhook_payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "webhookSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the query was fulfilled by a webhook call, this field is set to the value of the `source` field returned in the webhook response."]
        pub webhook_source: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1QueryResult {
        pub fn builder() -> GoogleCloudDialogflowV2beta1QueryResultBuilder {
            GoogleCloudDialogflowV2beta1QueryResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The sentiment, such as positive/negative feeling or association, for a unit of analysis, such as the query text."]
    pub struct GoogleCloudDialogflowV2beta1Sentiment {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "magnitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A non-negative number in the [0, +inf) range, which represents the absolute magnitude of sentiment, regardless of score (positive or negative)."]
        pub magnitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "score")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sentiment score between -1.0 (negative sentiment) and 1.0 (positive sentiment)."]
        pub score: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleCloudDialogflowV2beta1Sentiment {
        pub fn builder() -> GoogleCloudDialogflowV2beta1SentimentBuilder {
            GoogleCloudDialogflowV2beta1SentimentBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The result of sentiment analysis. Sentiment analysis inspects user input and identifies the prevailing subjective opinion, especially to determine a user's attitude as positive, negative, or neutral. For Participants.DetectIntent, it needs to be configured in DetectIntentRequest.query_params. For Participants.StreamingDetectIntent, it needs to be configured in StreamingDetectIntentRequest.query_params. And for Participants.AnalyzeContent and Participants.StreamingAnalyzeContent, it needs to be configured in ConversationProfile.human_agent_assistant_config"]
    pub struct GoogleCloudDialogflowV2beta1SentimentAnalysisResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryTextSentiment")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The sentiment analysis result for `query_text`."]
        pub query_text_sentiment:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1Sentiment>>,
    }
    impl GoogleCloudDialogflowV2beta1SentimentAnalysisResult {
        pub fn builder() -> GoogleCloudDialogflowV2beta1SentimentAnalysisResultBuilder {
            GoogleCloudDialogflowV2beta1SentimentAnalysisResultBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A session represents a conversation between a Dialogflow agent and an end-user. You can create special entities, called session entities, during a session. Session entities can extend or replace custom entity types and only exist during the session that they were created for. All session data, including session entities, is stored by Dialogflow for 20 minutes. For more information, see the [session entity guide](https://cloud.google.com/dialogflow/docs/entities-session)."]
    pub struct GoogleCloudDialogflowV2beta1SessionEntityType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entities")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The collection of entities associated with this session entity type."]
        pub entities: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1EntityTypeEntity>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entityOverrideMode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
        pub entity_override_mode: ::std::option::Option<
            GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The unique identifier of this session entity type. Supported formats: - `projects//agent/sessions//entityTypes/` - `projects//locations//agent/sessions//entityTypes/` - `projects//agent/environments//users//sessions//entityTypes/` - `projects//locations//agent/environments/ /users//sessions//entityTypes/` If `Location ID` is not specified we assume default 'us' location. If `Environment ID` is not specified, we assume default 'draft' environment. If `User ID` is not specified, we assume default '-' user. `` must be the display name of an existing entity type in the same agent that will be overridden or supplemented."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1SessionEntityType {
        pub fn builder() -> GoogleCloudDialogflowV2beta1SessionEntityTypeBuilder {
            GoogleCloudDialogflowV2beta1SessionEntityTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Indicates whether the additional data should override or supplement the custom entity type definition."]
    pub enum GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum {
        #[serde(rename = "ENTITY_OVERRIDE_MODE_UNSPECIFIED")]
        #[doc = "Not specified. This value should be never used."]
        EntityOverrideModeUnspecified,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_OVERRIDE")]
        #[doc = "The collection of session entities overrides the collection of entities in the corresponding custom entity type."]
        EntityOverrideModeOverride,
        #[serde(rename = "ENTITY_OVERRIDE_MODE_SUPPLEMENT")]
        #[doc = "The collection of session entities extends the collection of entities in the corresponding custom entity type. Note: Even in this override mode calls to `ListSessionEntityTypes`, `GetSessionEntityType`, `CreateSessionEntityType` and `UpdateSessionEntityType` only return the additional entities added in this session entity type. If you want to get the supplemented list, please call EntityTypes.GetEntityType on the custom entity type and merge."]
        EntityOverrideModeSupplement,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV2beta1SessionEntityTypeEntityOverrideModeEnum
    {
        fn default() -> Self {
            Self::EntityOverrideModeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The request message for a webhook call."]
    pub struct GoogleCloudDialogflowV2beta1WebhookRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "alternativeQueryResults")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Alternative query results from KnowledgeService."]
        pub alternative_query_results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "originalDetectIntentRequest")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The contents of the original request that was passed to `[Streaming]DetectIntent` call."]
        pub original_detect_intent_request: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV2beta1OriginalDetectIntentRequest>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "queryResult")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The result of the conversational query or event processing. Contains the same value as `[Streaming]DetectIntentResponse.query_result`."]
        pub query_result:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1QueryResult>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "responseId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of the response. Contains the same value as `[Streaming]DetectIntentResponse.response_id`."]
        pub response_id: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "session")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The unique identifier of detectIntent request session. Can be used to identify end-user inside webhook implementation. Supported formats: - `projects//agent/sessions/, - `projects//locations//agent/sessions/`, - `projects//agent/environments//users//sessions/`, - `projects//locations//agent/environments//users//sessions/`,"]
        pub session: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1WebhookRequest {
        pub fn builder() -> GoogleCloudDialogflowV2beta1WebhookRequestBuilder {
            GoogleCloudDialogflowV2beta1WebhookRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for a webhook call. This response is validated by the Dialogflow server. If validation fails, an error will be returned in the QueryResult.diagnostic_info field. Setting JSON fields to an empty value with the wrong type is a common error. To avoid this error: - Use `\"\"` for empty strings - Use `{}` or `null` for empty objects - Use `[]` or `null` for empty arrays For more information, see the [Protocol Buffers Language Guide](https://developers.google.com/protocol-buffers/docs/proto3#json)."]
    pub struct GoogleCloudDialogflowV2beta1WebhookResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "endInteraction")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Indicates that this intent ends an interaction. Some integrations (e.g., Actions on Google or Dialogflow phone gateway) use this information to close interaction with an end user. Default is false."]
        pub end_interaction: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "followupEventInput")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Invokes the supplied events. When this field is set, Dialogflow ignores the `fulfillment_text`, `fulfillment_messages`, and `payload` fields."]
        pub followup_event_input:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDialogflowV2beta1EventInput>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentMessages")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The rich response messages intended for the end-user. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_messages sent to the integration or API caller."]
        pub fulfillment_messages: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1IntentMessage>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fulfillmentText")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The text response message intended for the end-user. It is recommended to use `fulfillment_messages.text.text[0]` instead. When provided, Dialogflow uses this field to populate QueryResult.fulfillment_text sent to the integration or API caller."]
        pub fulfillment_text: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "outputContexts")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The collection of output contexts that will overwrite currently active contexts for the session and reset their lifespans. When provided, Dialogflow uses this field to populate QueryResult.output_contexts sent to the integration or API caller."]
        pub output_contexts: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1Context>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "payload")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. This field can be used to pass custom data from your webhook to the integration or API caller. Arbitrary JSON objects are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_payload sent to the integration or API caller. This field is also used by the [Google Assistant integration](https://cloud.google.com/dialogflow/docs/integrations/aog) for rich response messages. See the format definition at [Google Assistant Dialogflow webhook format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)"]
        pub payload:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sessionEntityTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Additional session entity types to replace or extend developer entity types with. The entity synonyms apply to all languages and persist for the session. Setting this data from a webhook overwrites the session entity types that have been set using `detectIntent`, `streamingDetectIntent` or SessionEntityType management methods."]
        pub session_entity_types: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDialogflowV2beta1SessionEntityType>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "source")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A custom field used to identify the webhook source. Arbitrary strings are supported. When provided, Dialogflow uses this field to populate QueryResult.webhook_source sent to the integration or API caller."]
        pub source: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDialogflowV2beta1WebhookResponse {
        pub fn builder() -> GoogleCloudDialogflowV2beta1WebhookResponseBuilder {
            GoogleCloudDialogflowV2beta1WebhookResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for CreateDocument operation."]
    pub struct GoogleCloudDialogflowV3alpha1CreateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1CreateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1CreateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1CreateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for DeleteDocument operation."]
    pub struct GoogleCloudDialogflowV3alpha1DeleteDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1DeleteDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1DeleteDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1DeleteDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata in google::longrunning::Operation for Knowledge operations."]
    pub struct GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "state")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Output only. The current state of this operation."]
        pub state: ::std::option::Option<
            GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataStateEnum,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Required. Output only. The current state of this operation."]
    pub enum GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataStateEnum {
        #[serde(rename = "STATE_UNSPECIFIED")]
        #[doc = "State unspecified."]
        StateUnspecified,
        #[serde(rename = "PENDING")]
        #[doc = "The operation has been created."]
        Pending,
        #[serde(rename = "RUNNING")]
        #[doc = "The operation is currently running."]
        Running,
        #[serde(rename = "DONE")]
        #[doc = "The operation is done, either cancelled or completed."]
        Done,
    }
    impl ::std::default::Default
        for GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadataStateEnum
    {
        fn default() -> Self {
            Self::StateUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ImportDocuments operation."]
    pub struct GoogleCloudDialogflowV3alpha1ImportDocumentsOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1ImportDocumentsOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1ImportDocumentsOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1ImportDocumentsOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for Documents.ImportDocuments."]
    pub struct GoogleCloudDialogflowV3alpha1ImportDocumentsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "warnings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Includes details about skipped documents or any other warnings."]
        pub warnings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleRpcStatus>>>,
    }
    impl GoogleCloudDialogflowV3alpha1ImportDocumentsResponse {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1ImportDocumentsResponseBuilder {
            GoogleCloudDialogflowV3alpha1ImportDocumentsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for ReloadDocument operation."]
    pub struct GoogleCloudDialogflowV3alpha1ReloadDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1ReloadDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1ReloadDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1ReloadDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Metadata for UpdateDocument operation."]
    pub struct GoogleCloudDialogflowV3alpha1UpdateDocumentOperationMetadata {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "genericMetadata")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The generic information of the operation."]
        pub generic_metadata: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDialogflowV3alpha1GenericKnowledgeOperationMetadata>,
        >,
    }
    impl GoogleCloudDialogflowV3alpha1UpdateDocumentOperationMetadata {
        pub fn builder() -> GoogleCloudDialogflowV3alpha1UpdateDocumentOperationMetadataBuilder {
            GoogleCloudDialogflowV3alpha1UpdateDocumentOperationMetadataBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The response message for Operations.ListOperations."]
    pub struct GoogleLongrunningListOperationsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The standard List next-page token."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "operations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of operations that matches the specified filter in the request."]
        pub operations:
            ::std::option::Option<::std::vec::Vec<::std::boxed::Box<GoogleLongrunningOperation>>>,
    }
    impl GoogleLongrunningListOperationsResponse {
        pub fn builder() -> GoogleLongrunningListOperationsResponseBuilder {
            GoogleLongrunningListOperationsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "This resource represents a long-running operation that is the result of a network API call."]
    pub struct GoogleLongrunningOperation {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "done")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available."]
        pub done: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "error")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The error result of the operation in case of failure or cancellation."]
        pub error: ::std::option::Option<::std::boxed::Box<GoogleRpcStatus>>,
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
    impl GoogleLongrunningOperation {
        pub fn builder() -> GoogleLongrunningOperationBuilder {
            GoogleLongrunningOperationBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A generic empty message that you can re-use to avoid defining duplicated empty messages in your APIs. A typical example is to use it as the request or the response type of an API method. For instance: service Foo { rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty); } The JSON representation for `Empty` is empty JSON object `{}`."]
    pub struct GoogleProtobufEmpty {}
    impl GoogleProtobufEmpty {
        pub fn builder() -> GoogleProtobufEmptyBuilder {
            GoogleProtobufEmptyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The `Status` type defines a logical error model that is suitable for different programming environments, including REST APIs and RPC APIs. It is used by [gRPC](https://github.com/grpc). Each `Status` message contains three pieces of data: error code, error message, and error details. You can find out more about this error model and how to work with it in the [API Design Guide](https://cloud.google.com/apis/design/errors)."]
    pub struct GoogleRpcStatus {
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
    impl GoogleRpcStatus {
        pub fn builder() -> GoogleRpcStatusBuilder {
            GoogleRpcStatusBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An object that represents a latitude/longitude pair. This is expressed as a pair of doubles to represent degrees latitude and degrees longitude. Unless specified otherwise, this must conform to the WGS84 standard. Values must be within normalized ranges."]
    pub struct GoogleTypeLatLng {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "latitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The latitude in degrees. It must be in the range [-90.0, +90.0]."]
        pub latitude: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "longitude")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The longitude in degrees. It must be in the range [-180.0, +180.0]."]
        pub longitude: ::std::option::Option<::std::primitive::f64>,
    }
    impl GoogleTypeLatLng {
        pub fn builder() -> GoogleTypeLatLngBuilder {
            GoogleTypeLatLngBuilder::default()
        }
    }
}
