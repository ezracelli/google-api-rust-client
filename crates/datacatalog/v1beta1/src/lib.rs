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
    pub mod entries {
        pub mod methods {
            pub mod lookup {
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
                    #[serde(rename = "linkedResource")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The full name of the Google Cloud Platform resource the Data Catalog entry represents. See: https://cloud.google.com/apis/design/resource_names#full_resource_name. Full names are case-sensitive. Examples: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId * //pubsub.googleapis.com/projects/projectId/topics/topicId"]
                    pub linked_resource: ::std::option::Option<::std::string::String>,
                    #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
                    #[serde(rename = "sqlResource")]
                    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
                    #[doc = "The SQL name of the entry. SQL names are case-sensitive. Examples: * `pubsub.project_id.topic_id` * ``pubsub.project_id.`topic.id.with.dots` `` * `bigquery.table.project_id.dataset_id.table_id` * `bigquery.dataset.project_id.dataset_id` * `datacatalog.entry.project_id.location_id.entry_group_id.entry_id` `*_id`s shoud satisfy the standard SQL rules for identifiers. https://cloud.google.com/bigquery/docs/reference/standard-sql/lexical."]
                    pub sql_resource: ::std::option::Option<::std::string::String>,
                }
                impl QueryParameters {
                    pub fn builder() -> QueryParametersBuilder {
                        QueryParametersBuilder::default()
                    }
                }
            }
        }
    }
    pub mod projects {
        pub mod resources {
            pub mod locations {
                pub mod resources {
                    pub mod entry_groups {
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
                                    #[serde(rename = "entryGroupId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The id of the entry group to create. The id must begin with a letter or underscore, contain only English letters, numbers and underscores, and be at most 64 characters."]
                                    pub entry_group_id:
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
                                    #[doc = "Optional. If true, deletes all entries in the entry group."]
                                    pub force: ::std::option::Option<::std::primitive::bool>,
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
                                    #[serde(rename = "readMask")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The fields to return. If not set or empty, all fields are returned."]
                                    pub read_mask: ::std::option::Option<::std::string::String>,
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
                                    #[doc = "Optional. The maximum number of items to return. Default is 10. Max limit is 1000. Throws an invalid argument for `page_size > 1000`."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Optional. Token that specifies which page is requested. If empty, the first page is returned."]
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
                                    #[doc = "The fields to update on the entry group. If absent or empty, all modifiable fields are updated."]
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
                            pub mod entries {
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
                                            #[serde(rename = "entryId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The id of the entry to create."]
                                            pub entry_id:
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
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of items to return. Default is 10. Max limit is 1000. Throws an invalid argument for `page_size > 1000`."]
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
                                            #[doc = "Token that specifies which page is requested. If empty, the first page is returned."]
                                            pub page_token:
                                                ::std::option::Option<::std::string::String>,
                                            #[builder(
                                                default = "{ ::std::default::Default::default() }",
                                                setter(into)
                                            )]
                                            #[serde(rename = "readMask")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The fields to return for each Entry. If not set or empty, all fields are returned. For example, setting read_mask to contain only one path \"name\" will cause ListEntries to return a list of Entries with only \"name\" field."]
                                            pub read_mask:
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
                                            #[doc = "The fields to update on the entry. If absent or empty, all modifiable fields are updated. The following fields are modifiable: * For entries with type `DATA_STREAM`: * `schema` * For entries with type `FILESET` * `schema` * `display_name` * `description` * `gcs_fileset_spec` * `gcs_fileset_spec.file_patterns` * For entries with `user_specified_type` * `schema` * `display_name` * `description` * user_specified_type * user_specified_system * linked_resource * source_system_timestamps"]
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
                                                    #[serde(rename = "pageSize")]
                                                    #[serde(
                                                        skip_serializing_if = "::std::option::Option::is_none"
                                                    )]
                                                    #[doc = "The maximum number of tags to return. Default is 10. Max limit is 1000."]
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
                                                    #[doc = "Token that specifies which page is requested. If empty, the first page is returned."]
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
                                                    #[doc = "The fields to update on the Tag. If absent or empty, all modifiable fields are updated. Currently the only modifiable field is the field `fields`."]
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
                                            #[serde(rename = "pageSize")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "The maximum number of tags to return. Default is 10. Max limit is 1000."]
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
                                            #[doc = "Token that specifies which page is requested. If empty, the first page is returned."]
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
                                            #[doc = "The fields to update on the Tag. If absent or empty, all modifiable fields are updated. Currently the only modifiable field is the field `fields`."]
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
                    pub mod tag_templates {
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
                                    #[serde(rename = "tagTemplateId")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. The id of the tag template to create."]
                                    pub tag_template_id:
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
                                    #[doc = "Required. Currently, this field must always be set to `true`. This confirms the deletion of any possible tags using this template. `force = false` will be supported in the future."]
                                    pub force: ::std::option::Option<::std::primitive::bool>,
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
                                    #[doc = "The field mask specifies the parts of the template to overwrite. Allowed fields: * `display_name` If absent or empty, all of the allowed fields above will be updated."]
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
                            pub mod fields {
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
                                            #[serde(rename = "tagTemplateFieldId")]
                                            #[serde(
                                                skip_serializing_if = "::std::option::Option::is_none"
                                            )]
                                            #[doc = "Required. The ID of the tag template field to create. Field ids can contain letters (both uppercase and lowercase), numbers (0-9), underscores (_) and dashes (-). Field IDs must be at least 1 character long and at most 128 characters long. Field IDs must also be unique within their template."]
                                            pub tag_template_field_id:
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
                                            #[doc = "Required. Currently, this field must always be set to `true`. This confirms the deletion of this field from any tags using this field. `force = false` will be supported in the future."]
                                            pub force:
                                                ::std::option::Option<::std::primitive::bool>,
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
                                            #[doc = "Optional. The field mask specifies the parts of the template to be updated. Allowed fields: * `display_name` * `type.enum_type` * `is_required` If `update_mask` is not set or empty, all of the allowed fields above will be updated. When updating an enum type, the provided values will be merged with the existing values. Therefore, enum values can only be added, existing enum values cannot be deleted nor renamed. Updating a template field from optional to required is NOT allowed."]
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
                    pub mod taxonomies {
                        pub mod methods {
                            pub mod export {
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
                                    #[serde(rename = "serializedTaxonomies")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Export taxonomies as serialized taxonomies."]
                                    pub serialized_taxonomies:
                                        ::std::option::Option<::std::primitive::bool>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "taxonomies")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "Required. Resource names of the taxonomies to be exported."]
                                    pub taxonomies: ::std::option::Option<::std::string::String>,
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
                                    #[doc = "The maximum number of items to return. Must be a value between 1 and 1000. If not set, defaults to 50."]
                                    pub page_size: ::std::option::Option<::std::primitive::i64>,
                                    #[builder(
                                        default = "{ ::std::default::Default::default() }",
                                        setter(into)
                                    )]
                                    #[serde(rename = "pageToken")]
                                    #[serde(
                                        skip_serializing_if = "::std::option::Option::is_none"
                                    )]
                                    #[doc = "The next_page_token value returned from a previous list request, if any. If not set, defaults to an empty string."]
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
                                    #[doc = "The update mask applies to the resource. For the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask If not set, defaults to all of the fields that are allowed to update."]
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
                            pub mod policy_tags {
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
                                            #[doc = "The maximum number of items to return. Must be a value between 1 and 1000. If not set, defaults to 50."]
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
                                            #[doc = "The next_page_token value returned from a previous List request, if any. If not set, defaults to an empty string."]
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
                                            #[doc = "The update mask applies to the resource. Only display_name, description and parent_policy_tag can be updated and thus can be listed in the mask. If update_mask is not provided, all allowed fields (i.e. display_name, description and parent) will be updated. For more information including the `FieldMask` definition, see https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask If not set, defaults to all of the fields that are allowed to update."]
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
                }
            }
        }
    }
}
pub mod schemas {
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Associates `members` with a `role`."]
    pub struct Binding {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "condition")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The condition that is associated with this binding. If the condition evaluates to `true`, then this binding applies to the current request. If the condition evaluates to `false`, then this binding does not apply to the current request. However, a different role binding might grant the same role to one or more of the members in this binding. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub condition: ::std::option::Option<::std::boxed::Box<Expr>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "members")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the identities requesting access for a Cloud Platform resource. `members` can have the following values: * `allUsers`: A special identifier that represents anyone who is on the internet; with or without a Google account. * `allAuthenticatedUsers`: A special identifier that represents anyone who is authenticated with a Google account or a service account. * `user:{emailid}`: An email address that represents a specific Google account. For example, `alice@example.com` . * `serviceAccount:{emailid}`: An email address that represents a service account. For example, `my-other-app@appspot.gserviceaccount.com`. * `group:{emailid}`: An email address that represents a Google group. For example, `admins@example.com`. * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a user that has been recently deleted. For example, `alice@example.com?uid=123456789012345678901`. If the user is recovered, this value reverts to `user:{emailid}` and the recovered user retains the role in the binding. * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a service account that has been recently deleted. For example, `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`. If the service account is undeleted, this value reverts to `serviceAccount:{emailid}` and the undeleted service account retains the role in the binding. * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique identifier) representing a Google group that has been recently deleted. For example, `admins@example.com?uid=123456789012345678901`. If the group is recovered, this value reverts to `group:{emailid}` and the recovered group retains the role in the binding. * `domain:{domain}`: The G Suite domain (primary) that represents all the users of that domain. For example, `google.com` or `example.com`. "]
        pub members: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "role")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Role that is assigned to `members`. For example, `roles/viewer`, `roles/editor`, or `roles/owner`."]
        pub role: ::std::option::Option<::std::string::String>,
    }
    impl Binding {
        pub fn builder() -> BindingBuilder {
            BindingBuilder::default()
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
    #[doc = "Represents a textual expression in the Common Expression Language (CEL) syntax. CEL is a C-like expression language. The syntax and semantics of CEL are documented at https://github.com/google/cel-spec. Example (Comparison): title: \"Summary size limit\" description: \"Determines if a summary is less than 100 chars\" expression: \"document.summary.size() < 100\" Example (Equality): title: \"Requestor is owner\" description: \"Determines if requestor is the document owner\" expression: \"document.owner == request.auth.claims.email\" Example (Logic): title: \"Public documents\" description: \"Determine whether the document should be publicly visible\" expression: \"document.type != 'private' && document.type != 'internal'\" Example (Data Manipulation): title: \"Notification string\" description: \"Create a notification string with a timestamp.\" expression: \"'New message received at ' + string(document.create_time)\" The exact variables and functions that may be referenced within an expression are determined by the service that evaluates it. See the service documentation for additional information."]
    pub struct Expr {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expression")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Textual representation of an expression in Common Expression Language syntax."]
        pub expression: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "location")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. String indicating the location of the expression for error reporting, e.g. a file name and a position in the file."]
        pub location: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "title")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Title for the expression, i.e. a short string describing its purpose. This can be used e.g. in UIs which allow to enter the expression."]
        pub title: ::std::option::Option<::std::string::String>,
    }
    impl Expr {
        pub fn builder() -> ExprBuilder {
            ExprBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `GetIamPolicy` method."]
    pub struct GetIamPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "options")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "OPTIONAL: A `GetPolicyOptions` object for specifying options to `GetIamPolicy`."]
        pub options: ::std::option::Option<::std::boxed::Box<GetPolicyOptions>>,
    }
    impl GetIamPolicyRequest {
        pub fn builder() -> GetIamPolicyRequestBuilder {
            GetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Encapsulates settings provided to GetIamPolicy."]
    pub struct GetPolicyOptions {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "requestedPolicyVersion")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The policy format version to be returned. Valid values are 0, 1, and 3. Requests specifying an invalid value will be rejected. Requests for policies with any conditional bindings must specify version 3. Policies without any conditional bindings may specify any valid value or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub requested_policy_version: ::std::option::Option<::std::primitive::i64>,
    }
    impl GetPolicyOptions {
        pub fn builder() -> GetPolicyOptionsBuilder {
            GetPolicyOptionsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Spec for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding"]
    pub struct GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataset")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Data Catalog resource name of the dataset entry the current table belongs to, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`."]
        pub dataset: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "shardCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Total number of shards."]
        pub shard_count: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tablePrefix")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The table name prefix of the shards. The name of any given shard is `[table_prefix]YYYYMMDD`, for example, for shard `MyTable20180101`, the `table_prefix` is `MyTable`."]
        pub table_prefix: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpecBuilder {
            GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a BigQuery table."]
    pub struct GoogleCloudDatacatalogV1beta1BigQueryTableSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableSourceType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The table source type."]
        pub table_source_type: ::std::option::Option<
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tableSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Spec of a BigQuery table. This field should only be populated if `table_source_type` is `BIGQUERY_TABLE`."]
        pub table_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1TableSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Table view specification. This field should only be populated if `table_source_type` is `BIGQUERY_VIEW`."]
        pub view_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ViewSpec>>,
    }
    impl GoogleCloudDatacatalogV1beta1BigQueryTableSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1BigQueryTableSpecBuilder {
            GoogleCloudDatacatalogV1beta1BigQueryTableSpecBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. The table source type."]
    pub enum GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
        #[serde(rename = "TABLE_SOURCE_TYPE_UNSPECIFIED")]
        #[doc = "Default unknown type."]
        TableSourceTypeUnspecified,
        #[serde(rename = "BIGQUERY_VIEW")]
        #[doc = "Table view."]
        BigqueryView,
        #[serde(rename = "BIGQUERY_TABLE")]
        #[doc = "BigQuery native table."]
        BigqueryTable,
        #[serde(rename = "BIGQUERY_MATERIALIZED_VIEW")]
        #[doc = "BigQuery materialized view."]
        BigqueryMaterializedView,
    }
    impl ::std::default::Default for GoogleCloudDatacatalogV1beta1BigQueryTableSpecTableSourceTypeEnum {
        fn default() -> Self {
            Self::TableSourceTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Representation of a column within a schema. Columns could be nested inside other columns."]
    pub struct GoogleCloudDatacatalogV1beta1ColumnSchema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "column")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Name of the column."]
        pub column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of the column. Default value is an empty string."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "mode")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A column's mode indicates whether the values in this column are required, nullable, etc. Only `NULLABLE`, `REQUIRED` and `REPEATED` are supported. Default mode is `NULLABLE`."]
        pub mode: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "subcolumns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Schema of sub-columns. A column can have zero or more sub-columns."]
        pub subcolumns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Type of the column."]
        pub _type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1ColumnSchema {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ColumnSchemaBuilder {
            GoogleCloudDatacatalogV1beta1ColumnSchemaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Entry Metadata. A Data Catalog Entry resource represents another resource in Google Cloud Platform (such as a BigQuery dataset or a Pub/Sub topic), or outside of Google Cloud Platform. Clients can use the `linked_resource` field in the Entry resource to refer to the original resource ID of the source system. An Entry resource contains resource details, such as its schema. An Entry can also be used to attach flexible metadata, such as a Tag."]
    pub struct GoogleCloudDatacatalogV1beta1Entry {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigqueryDateShardedSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification for a group of BigQuery tables with name pattern `[prefix]YYYYMMDD`. Context: https://cloud.google.com/bigquery/docs/partitioned-tables#partitioning_versus_sharding."]
        pub bigquery_date_sharded_spec: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1BigQueryDateShardedSpec>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bigqueryTableSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification that applies to a BigQuery table. This is only valid on entries of type `TABLE`."]
        pub bigquery_table_spec: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1BigQueryTableSpec>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entry description, which can consist of several sentences or paragraphs that describe entry contents. Default value is an empty string."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Display information such as title and description. A short name to identify the entry, for example, \"Analytics Data - Jan 2011\". Default value is an empty string."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsFilesetSpec")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specification that applies to a Cloud Storage fileset. This is only valid on entries of type FILESET."]
        pub gcs_fileset_spec:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1GcsFilesetSpec>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "integratedSystem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
        pub integrated_system:
            ::std::option::Option<GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource this metadata entry refers to. For Google Cloud Platform resources, `linked_resource` is the [full name of the resource](https://cloud.google.com/apis/design/resource_names#full_resource_name). For example, the `linked_resource` for a table resource from BigQuery is: * //bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty string."]
        pub linked_resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The Data Catalog resource name of the entry in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id}/entries/{entry_id} Note that this Entry and its child resources may not actually be stored in the location in this name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "schema")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Schema of the entry. An entry might not have any schema attached to it."]
        pub schema: ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Schema>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sourceSystemTimestamps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamps about the underlying resource, not about this Data Catalog entry. Output only when Entry is of type in the EntryType enum. For entries with user_specified_type, this field is optional and defaults to an empty timestamp."]
        pub source_system_timestamps:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The type of the entry. Only used for Entries with types in the EntryType enum."]
        pub _type: ::std::option::Option<GoogleCloudDatacatalogV1beta1EntryTypeEnum>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userSpecifiedSystem")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "This field indicates the entry's source system that Data Catalog does not integrate with. `user_specified_system` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long."]
        pub user_specified_system: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "userSpecifiedType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entry type if it does not fit any of the input-allowed values listed in `EntryType` enum above. When creating an entry, users should check the enum values first, if nothing matches the entry to be created, then provide a custom value, for example \"my_special_type\". `user_specified_type` strings must begin with a letter or underscore and can only contain letters, numbers, and underscores; are case insensitive; must be at least 1 character and at most 64 characters long. Currently, only FILESET enum value is allowed. All other entries created through Data Catalog must use `user_specified_type`."]
        pub user_specified_type: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1Entry {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1EntryBuilder {
            GoogleCloudDatacatalogV1beta1EntryBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Output only. This field indicates the entry's source system that Data Catalog integrates with, such as BigQuery or Pub/Sub."]
    pub enum GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
        #[serde(rename = "INTEGRATED_SYSTEM_UNSPECIFIED")]
        #[doc = "Default unknown system."]
        IntegratedSystemUnspecified,
        #[serde(rename = "BIGQUERY")]
        #[doc = "BigQuery."]
        Bigquery,
        #[serde(rename = "CLOUD_PUBSUB")]
        #[doc = "Cloud Pub/Sub."]
        CloudPubsub,
    }
    impl ::std::default::Default for GoogleCloudDatacatalogV1beta1EntryIntegratedSystemEnum {
        fn default() -> Self {
            Self::IntegratedSystemUnspecified
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "The type of the entry. Only used for Entries with types in the EntryType enum."]
    pub enum GoogleCloudDatacatalogV1beta1EntryTypeEnum {
        #[serde(rename = "ENTRY_TYPE_UNSPECIFIED")]
        #[doc = "Default unknown type."]
        EntryTypeUnspecified,
        #[serde(rename = "TABLE")]
        #[doc = "Output only. The type of entry that has a GoogleSQL schema, including logical views."]
        Table,
        #[serde(rename = "MODEL")]
        #[doc = "Output only. The type of models. https://cloud.google.com/bigquery-ml/docs/bigqueryml-intro"]
        Model,
        #[serde(rename = "DATA_STREAM")]
        #[doc = "Output only. An entry type which is used for streaming entries. Example: Pub/Sub topic."]
        DataStream,
        #[serde(rename = "FILESET")]
        #[doc = "An entry type which is a set of files or objects. Example: Cloud Storage fileset."]
        Fileset,
    }
    impl ::std::default::Default for GoogleCloudDatacatalogV1beta1EntryTypeEnum {
        fn default() -> Self {
            Self::EntryTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "EntryGroup Metadata. An EntryGroup resource represents a logical grouping of zero or more Data Catalog Entry resources."]
    pub struct GoogleCloudDatacatalogV1beta1EntryGroup {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "dataCatalogTimestamps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamps about this EntryGroup. Default value is empty timestamps."]
        pub data_catalog_timestamps:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entry group description, which can consist of several sentences or paragraphs that describe entry group contents. Default value is an empty string."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A short name to identify the entry group, for example, \"analytics data - jan 2011\". Default value is an empty string."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the entry group in URL format. Example: * projects/{project_id}/locations/{location}/entryGroups/{entry_group_id} Note that this EntryGroup and its child resources may not actually be stored in the location in this name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1EntryGroup {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1EntryGroupBuilder {
            GoogleCloudDatacatalogV1beta1EntryGroupBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ExportTaxonomies."]
    pub struct GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxonomies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "List of taxonomies and policy tags in a tree structure."]
        pub taxonomies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponseBuilder {
            GoogleCloudDatacatalogV1beta1ExportTaxonomiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatacatalogV1beta1FieldType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enumType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents an enum type."]
        pub enum_type: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldTypeEnumType>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "primitiveType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Represents primitive types - string, bool etc."]
        pub primitive_type:
            ::std::option::Option<GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum>,
    }
    impl GoogleCloudDatacatalogV1beta1FieldType {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1FieldTypeBuilder {
            GoogleCloudDatacatalogV1beta1FieldTypeBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Represents primitive types - string, bool etc."]
    pub enum GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
        #[serde(rename = "PRIMITIVE_TYPE_UNSPECIFIED")]
        #[doc = "This is the default invalid value for a type."]
        PrimitiveTypeUnspecified,
        #[serde(rename = "DOUBLE")]
        #[doc = "A double precision number."]
        Double,
        #[serde(rename = "STRING")]
        #[doc = "An UTF-8 string."]
        String,
        #[serde(rename = "BOOL")]
        #[doc = "A boolean value."]
        Bool,
        #[serde(rename = "TIMESTAMP")]
        #[doc = "A timestamp."]
        Timestamp,
    }
    impl ::std::default::Default for GoogleCloudDatacatalogV1beta1FieldTypePrimitiveTypeEnum {
        fn default() -> Self {
            Self::PrimitiveTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumType {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "allowedValues")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        pub allowed_values: ::std::option::Option<
            ::std::vec::Vec<
                ::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue>,
            >,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1FieldTypeEnumType {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeBuilder {
            GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    pub struct GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The display name of the enum value. Must not be an empty string."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValue {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValueBuilder {
            GoogleCloudDatacatalogV1beta1FieldTypeEnumTypeEnumValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Specifications of a single file in Cloud Storage."]
    pub struct GoogleCloudDatacatalogV1beta1GcsFileSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filePath")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The full file path. Example: `gs://bucket_name/a/b.txt`."]
        pub file_path: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "gcsTimestamps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamps about the Cloud Storage file."]
        pub gcs_timestamps:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sizeBytes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The size of the file, in bytes."]
        pub size_bytes: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1GcsFileSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1GcsFileSpecBuilder {
            GoogleCloudDatacatalogV1beta1GcsFileSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Describes a Cloud Storage fileset entry."]
    pub struct GoogleCloudDatacatalogV1beta1GcsFilesetSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "filePatterns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Patterns to identify a set of files in Google Cloud Storage. See [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames) for more information. Note that bucket wildcards are currently not supported. Examples of valid file_patterns: * `gs://bucket_name/dir/*`: matches all files within `bucket_name/dir` directory. * `gs://bucket_name/dir/**`: matches all files in `bucket_name/dir` spanning all subdirectories. * `gs://bucket_name/file*`: matches files prefixed by `file` in `bucket_name` * `gs://bucket_name/??.txt`: matches files with two characters followed by `.txt` in `bucket_name` * `gs://bucket_name/[aeiou].txt`: matches files that contain a single vowel character followed by `.txt` in `bucket_name` * `gs://bucket_name/[a-m].txt`: matches files that contain `a`, `b`, ... or `m` followed by `.txt` in `bucket_name` * `gs://bucket_name/a/*/b`: matches all files in `bucket_name` that match `a/*/b` pattern, such as `a/c/b`, `a/d/b` * `gs://another_bucket/a.txt`: matches `gs://another_bucket/a.txt` You can combine wildcards to provide more powerful matches, for example: * `gs://bucket_name/[a-m]??.j*g`"]
        pub file_patterns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "sampleGcsFileSpecs")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Sample files contained in this fileset, not all files contained in this fileset are represented here."]
        pub sample_gcs_file_specs: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1GcsFileSpec>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1GcsFilesetSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1GcsFilesetSpecBuilder {
            GoogleCloudDatacatalogV1beta1GcsFilesetSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for ImportTaxonomies."]
    pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "inlineSource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Inline source used for taxonomies to be imported."]
        pub inline_source:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1InlineSource>>,
    }
    impl GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequest {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequestBuilder {
            GoogleCloudDatacatalogV1beta1ImportTaxonomiesRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ImportTaxonomies."]
    pub struct GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxonomies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Taxonomies that were imported."]
        pub taxonomies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Taxonomy>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponseBuilder {
            GoogleCloudDatacatalogV1beta1ImportTaxonomiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Inline source used for taxonomies import."]
    pub struct GoogleCloudDatacatalogV1beta1InlineSource {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxonomies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Taxonomies to be imported."]
        pub taxonomies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedTaxonomy>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1InlineSource {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1InlineSourceBuilder {
            GoogleCloudDatacatalogV1beta1InlineSourceBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListEntries."]
    pub struct GoogleCloudDatacatalogV1beta1ListEntriesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entries")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Entry details."]
        pub entries: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Entry>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1ListEntriesResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ListEntriesResponseBuilder {
            GoogleCloudDatacatalogV1beta1ListEntriesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListEntryGroups."]
    pub struct GoogleCloudDatacatalogV1beta1ListEntryGroupsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "entryGroups")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "EntryGroup details."]
        pub entry_groups: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1EntryGroup>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1ListEntryGroupsResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ListEntryGroupsResponseBuilder {
            GoogleCloudDatacatalogV1beta1ListEntryGroupsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListPolicyTags."]
    pub struct GoogleCloudDatacatalogV1beta1ListPolicyTagsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The policy tags that are in the requested taxonomy."]
        pub policy_tags: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1PolicyTag>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1ListPolicyTagsResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ListPolicyTagsResponseBuilder {
            GoogleCloudDatacatalogV1beta1ListPolicyTagsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListTags."]
    pub struct GoogleCloudDatacatalogV1beta1ListTagsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token to retrieve the next page of results. It is set to empty if no items remain in results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "tags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Tag details."]
        pub tags: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Tag>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1ListTagsResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ListTagsResponseBuilder {
            GoogleCloudDatacatalogV1beta1ListTagsResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for ListTaxonomies."]
    pub struct GoogleCloudDatacatalogV1beta1ListTaxonomiesResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Token used to retrieve the next page of results, or empty if there are no more results in the list."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxonomies")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Taxonomies that the project contains."]
        pub taxonomies: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1Taxonomy>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1ListTaxonomiesResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ListTaxonomiesResponseBuilder {
            GoogleCloudDatacatalogV1beta1ListTaxonomiesResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Denotes one policy tag in a taxonomy (e.g. ssn). Policy Tags can be defined in a hierarchy. For example, consider the following hierarchy: Geolocation -> (LatLong, City, ZipCode). PolicyTag \"Geolocation\" contains three child policy tags: \"LatLong\", \"City\", and \"ZipCode\"."]
    pub struct GoogleCloudDatacatalogV1beta1PolicyTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childPolicyTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource names of child policy tags of this policy tag."]
        pub child_policy_tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of this policy tag. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description. If not set, defaults to an empty description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. User defined name of this policy tag. It must: be unique within the parent taxonomy; contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this policy tag, whose format is: \"projects/{project_number}/locations/{location_id}/taxonomies/{taxonomy_id}/policyTags/{id}\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "parentPolicyTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of this policy tag's parent policy tag (e.g. for the \"LatLong\" policy tag in the example above, this field contains the resource name of the \"Geolocation\" policy tag). If empty, it means this policy tag is a top level policy tag (e.g. this field is empty for the \"Geolocation\" policy tag in the example above). If not set, defaults to an empty string."]
        pub parent_policy_tag: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1PolicyTag {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1PolicyTagBuilder {
            GoogleCloudDatacatalogV1beta1PolicyTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for RenameTagTemplateFieldEnumValue."]
    pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newEnumValueDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The new display name of the enum value. For example, `my_new_enum_value`."]
        pub new_enum_value_display_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequest {
        pub fn builder(
        ) -> GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequestBuilder {
            GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldEnumValueRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for RenameTagTemplateField."]
    pub struct GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "newTagTemplateFieldId")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The new ID of this tag template field. For example, `my_new_field`."]
        pub new_tag_template_field_id: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequest {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequestBuilder {
            GoogleCloudDatacatalogV1beta1RenameTagTemplateFieldRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Represents a schema (e.g. BigQuery, GoogleSQL, Avro schema)."]
    pub struct GoogleCloudDatacatalogV1beta1Schema {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "columns")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Schema of columns. A maximum of 10,000 columns and sub-columns can be specified."]
        pub columns: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1ColumnSchema>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1Schema {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SchemaBuilder {
            GoogleCloudDatacatalogV1beta1SchemaBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for SearchCatalog."]
    pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "orderBy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the ordering of results, currently supported case-sensitive choices are: * `relevance`, only supports descending * `last_modified_timestamp [asc|desc]`, defaults to descending if not specified If not specified, defaults to `relevance` descending."]
        pub order_by: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageSize")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Number of results in the search page. If <=0 then defaults to 10. Max limit for page_size is 1000. Throws an invalid argument for page_size > 1000."]
        pub page_size: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "pageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Pagination token returned in an earlier SearchCatalogResponse.next_page_token, which indicates that this is a continuation of a prior SearchCatalogRequest call, and that the system should return the next page of data. If empty, the first page is returned."]
        pub page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "query")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The query string in search query syntax. An empty query string will result in all data assets (in the specified scope) that the user has access to. Query strings can be simple as \"x\" or more qualified as: * name:x * column:x * description:y Note: Query tokens need to have a minimum of 3 characters for substring matching to work correctly. See [Data Catalog Search Syntax](https://cloud.google.com/data-catalog/docs/how-to/search-reference) for more information."]
        pub query: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "scope")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The scope of this search request. A `scope` that has empty `include_org_ids`, `include_project_ids` AND false `include_gcp_public_datasets` is considered invalid. Data Catalog will return an error in such a case."]
        pub scope: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1SearchCatalogRequest {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SearchCatalogRequestBuilder {
            GoogleCloudDatacatalogV1beta1SearchCatalogRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The criteria that select the subspace used for query matching."]
    pub struct GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeGcpPublicDatasets")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "If `true`, include Google Cloud Platform (GCP) public datasets in the search results. Info on GCP public datasets is available at https://cloud.google.com/public-datasets/. By default, GCP public datasets are excluded."]
        pub include_gcp_public_datasets: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeOrgIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of organization IDs to search within. To find your organization ID, follow instructions in https://cloud.google.com/resource-manager/docs/creating-managing-organization."]
        pub include_org_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "includeProjectIds")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The list of project IDs to search within. To learn more about the distinction between project names/IDs/numbers, go to https://cloud.google.com/docs/overview/#projects."]
        pub include_project_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "restrictedLocations")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. The list of locations to search within. 1. If empty, search will be performed in all locations; 2. If any of the locations are NOT in the valid locations list, error will be returned; 3. Otherwise, search only the given locations for matching results. Typical usage is to leave this field empty. When a location is unreachable as returned in the `SearchCatalogResponse.unreachable` field, users can repeat the search request with this parameter set to get additional information on the error. Valid locations: * asia-east1 * asia-east2 * asia-northeast1 * asia-northeast2 * asia-northeast3 * asia-south1 * asia-southeast1 * australia-southeast1 * eu * europe-north1 * europe-west1 * europe-west2 * europe-west3 * europe-west4 * europe-west6 * global * northamerica-northeast1 * southamerica-east1 * us * us-central1 * us-east1 * us-east4 * us-west1 * us-west2"]
        pub restricted_locations: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDatacatalogV1beta1SearchCatalogRequestScope {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SearchCatalogRequestScopeBuilder {
            GoogleCloudDatacatalogV1beta1SearchCatalogRequestScopeBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for SearchCatalog."]
    pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "nextPageToken")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The token that can be used to retrieve the next page of results."]
        pub next_page_token: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "results")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Search results."]
        pub results: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SearchCatalogResult>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "unreachable")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Unreachable locations. Search result does not include data from those locations. Users can get additional information on the error by repeating the search request with a more restrictive parameter -- setting the value for `SearchDataCatalogRequest.scope.restricted_locations`."]
        pub unreachable: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl GoogleCloudDatacatalogV1beta1SearchCatalogResponse {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SearchCatalogResponseBuilder {
            GoogleCloudDatacatalogV1beta1SearchCatalogResponseBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A result that appears in the response of a search request. Each result captures details of one entry that matches the search."]
    pub struct GoogleCloudDatacatalogV1beta1SearchCatalogResult {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "linkedResource")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The full name of the cloud resource the entry belongs to. See: https://cloud.google.com/apis/design/resource_names#full_resource_name. Example: * `//bigquery.googleapis.com/projects/projectId/datasets/datasetId/tables/tableId`"]
        pub linked_resource: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "modifyTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Last-modified timestamp of the entry from the managing system."]
        pub modify_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "relativeResourceName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The relative resource name of the resource in URL format. Examples: * `projects/{project_id}/locations/{location_id}/entryGroups/{entry_group_id}/entries/{entry_id}` * `projects/{project_id}/tagTemplates/{tag_template_id}`"]
        pub relative_resource_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchResultSubtype")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Sub-type of the search result. This is a dot-delimited description of the resource's full type, and is the same as the value callers would provide in the \"type\" search facet. Examples: `entry.table`, `entry.dataStream`, `tagTemplate`."]
        pub search_result_subtype: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "searchResultType")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Type of the search result. This field can be used to determine which Get method to call to fetch the full resource."]
        pub search_result_type: ::std::option::Option<
            GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1SearchCatalogResult {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SearchCatalogResultBuilder {
            GoogleCloudDatacatalogV1beta1SearchCatalogResultBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    #[doc = "Type of the search result. This field can be used to determine which Get method to call to fetch the full resource."]
    pub enum GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum {
        #[serde(rename = "SEARCH_RESULT_TYPE_UNSPECIFIED")]
        #[doc = "Default unknown type."]
        SearchResultTypeUnspecified,
        #[serde(rename = "ENTRY")]
        #[doc = "An Entry."]
        Entry,
        #[serde(rename = "TAG_TEMPLATE")]
        #[doc = "A TagTemplate."]
        TagTemplate,
        #[serde(rename = "ENTRY_GROUP")]
        #[doc = "An EntryGroup."]
        EntryGroup,
    }
    impl ::std::default::Default
        for GoogleCloudDatacatalogV1beta1SearchCatalogResultSearchResultTypeEnum
    {
        fn default() -> Self {
            Self::SearchResultTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message representing one policy tag when exported as a nested proto."]
    pub struct GoogleCloudDatacatalogV1beta1SerializedPolicyTag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "childPolicyTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Children of the policy tag if any."]
        pub child_policy_tags: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the serialized policy tag. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name of the policy tag. Max 200 bytes when encoded in UTF-8."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resource name of the policy tag. This field will be ignored when calling ImportTaxonomies."]
        pub policy_tag: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1SerializedPolicyTag {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SerializedPolicyTagBuilder {
            GoogleCloudDatacatalogV1beta1SerializedPolicyTagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Message capturing a taxonomy and its policy tag hierarchy as a nested proto. Used for taxonomy import/export and mutation."]
    pub struct GoogleCloudDatacatalogV1beta1SerializedTaxonomy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activatedPolicyTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A list of policy types that are activated for a taxonomy."]
        pub activated_policy_types: ::std::option::Option<
            ::std::vec::Vec<
                GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Description of the serialized taxonomy. The length of the description is limited to 2000 bytes when encoded in UTF-8. If not set, defaults to an empty description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Display name of the taxonomy. Max 200 bytes when encoded in UTF-8."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTags")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Top level policy tags associated with the taxonomy if any."]
        pub policy_tags: ::std::option::Option<
            ::std::vec::Vec<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SerializedPolicyTag>>,
        >,
    }
    impl GoogleCloudDatacatalogV1beta1SerializedTaxonomy {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SerializedTaxonomyBuilder {
            GoogleCloudDatacatalogV1beta1SerializedTaxonomyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum {
        #[serde(rename = "POLICY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified policy type."]
        PolicyTypeUnspecified,
        #[serde(rename = "FINE_GRAINED_ACCESS_CONTROL")]
        #[doc = "Fine grained access control policy, which enables access control on tagged resources."]
        FineGrainedAccessControl,
    }
    impl ::std::default::Default
        for GoogleCloudDatacatalogV1beta1SerializedTaxonomyActivatedPolicyTypesEnum
    {
        fn default() -> Self {
            Self::PolicyTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Timestamps about this resource according to a particular system."]
    pub struct GoogleCloudDatacatalogV1beta1SystemTimestamps {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "createTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The creation time of the resource within the given system."]
        pub create_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "expireTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The expiration time of the resource within the given system. Currently only apllicable to BigQuery resources."]
        pub expire_time: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "updateTime")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The last-modified time of the resource within the given system."]
        pub update_time: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1SystemTimestamps {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1SystemTimestampsBuilder {
            GoogleCloudDatacatalogV1beta1SystemTimestampsBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Normal BigQuery table spec."]
    pub struct GoogleCloudDatacatalogV1beta1TableSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "groupedEntry")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. If the table is a dated shard, i.e., with name pattern `[prefix]YYYYMMDD`, `grouped_entry` is the Data Catalog resource name of the date sharded grouped entry, for example, `projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}`. Otherwise, `grouped_entry` is empty."]
        pub grouped_entry: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1TableSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TableSpecBuilder {
            GoogleCloudDatacatalogV1beta1TableSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Tags are used to attach custom metadata to Data Catalog resources. Tags conform to the specifications within their tag template. See [Data Catalog IAM](https://cloud.google.com/data-catalog/docs/concepts/iam) for information on the permissions needed to create or view tags."]
    pub struct GoogleCloudDatacatalogV1beta1Tag {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "column")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Resources like Entry can have schemas associated with them. This scope allows users to attach tags to an individual column based on that schema. For attaching a tag to a nested column, use `.` to separate the column names. Example: * `outer_column.inner_column`"]
        pub column: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. This maps the ID of a tag field to the value of and additional information about that field. Valid field IDs are defined by the tag's template. A tag must have at least 1 field and at most 500 fields."]
        pub fields: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagField>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the tag in URL format. Example: * projects/{project_id}/locations/{location}/entrygroups/{entry_group_id}/entries/{entry_id}/tags/{tag_id} where `tag_id` is a system-generated identifier. Note that this Tag may not actually be stored in the location in this name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "template")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The resource name of the tag template that this tag uses. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} This field cannot be modified after creation."]
        pub template: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "templateDisplayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The display name of the tag template."]
        pub template_display_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1Tag {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TagBuilder {
            GoogleCloudDatacatalogV1beta1TagBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Contains the value and supporting information for a field within a Tag."]
    pub struct GoogleCloudDatacatalogV1beta1TagField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "boolValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the value for a tag field with boolean type."]
        pub bool_value: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The display name of this field."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "doubleValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the value for a tag field with double type."]
        pub double_value: ::std::option::Option<::std::primitive::f64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "enumValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the value for a tag field with enum type. This value must be one of the allowed values in the definition of this enum."]
        pub enum_value: ::std::option::Option<
            ::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagFieldEnumValue>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The order of this field with respect to other fields in this tag. It can be set in Tag. For example, a higher value can indicate a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential."]
        pub order: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "stringValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the value for a tag field with string type."]
        pub string_value: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "timestampValue")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Holds the value for a tag field with timestamp type."]
        pub timestamp_value: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1TagField {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TagFieldBuilder {
            GoogleCloudDatacatalogV1beta1TagFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Holds an enum value."]
    pub struct GoogleCloudDatacatalogV1beta1TagFieldEnumValue {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name of the enum value."]
        pub display_name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1TagFieldEnumValue {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TagFieldEnumValueBuilder {
            GoogleCloudDatacatalogV1beta1TagFieldEnumValueBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A tag template defines a tag, which can have one or more typed fields. The template is used to create and attach the tag to GCP resources. [Tag template roles](https://cloud.google.com/iam/docs/understanding-roles#data-catalog-roles) provide permissions to create, edit, and use the template. See, for example, the [TagTemplate User](https://cloud.google.com/data-catalog/docs/how-to/template-user) role, which includes permission to use the tag template to tag resources."]
    pub struct GoogleCloudDatacatalogV1beta1TagTemplate {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name for this template. Defaults to an empty string."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "fields")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. Map of tag template field IDs to the settings for the field. This map is an exhaustive list of the allowed fields. This map must contain at least one field and at most 500 fields. The keys to this map are tag template field IDs. Field IDs can contain letters (both uppercase and lowercase), numbers (0-9) and underscores (_). Field IDs must be at least 1 character long and at most 64 characters long. Field IDs must start with a letter or underscore."]
        pub fields: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                ::std::boxed::Box<GoogleCloudDatacatalogV1beta1TagTemplateField>,
            >,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The resource name of the tag template in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template_id} Note that this TagTemplate and its child resources may not actually be stored in the location in this name."]
        pub name: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1TagTemplate {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TagTemplateBuilder {
            GoogleCloudDatacatalogV1beta1TagTemplateBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "The template for an individual field within a tag template."]
    pub struct GoogleCloudDatacatalogV1beta1TagTemplateField {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The description for this field. Defaults to an empty string."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The display name for this field. Defaults to an empty string."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "isRequired")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Whether this is a required field. Defaults to false."]
        pub is_required: ::std::option::Option<::std::primitive::bool>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The resource name of the tag template field in URL format. Example: * projects/{project_id}/locations/{location}/tagTemplates/{tag_template}/fields/{field} Note that this TagTemplateField may not actually be stored in the location in this name."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "order")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The order of this field with respect to other fields in this tag template. A higher value indicates a more important field. The value can be negative. Multiple fields can have the same order, and field orders within a tag do not have to be sequential."]
        pub order: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "type")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. The type of value this tag field can contain."]
        pub _type: ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1FieldType>>,
    }
    impl GoogleCloudDatacatalogV1beta1TagTemplateField {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TagTemplateFieldBuilder {
            GoogleCloudDatacatalogV1beta1TagTemplateFieldBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "A taxonomy is a collection of policy tags that classify data along a common axis. For instance a data *sensitivity* taxonomy could contain policy tags denoting PII such as age, zipcode, and SSN. A data *origin* taxonomy could contain policy tags to distinguish user data, employee data, partner data, public data."]
    pub struct GoogleCloudDatacatalogV1beta1Taxonomy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "activatedPolicyTypes")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. A list of policy types that are activated for this taxonomy. If not set, defaults to an empty list."]
        pub activated_policy_types: ::std::option::Option<
            ::std::vec::Vec<GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum>,
        >,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "description")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Optional. Description of this taxonomy. It must: contain only unicode characters, tabs, newlines, carriage returns and page breaks; and be at most 2000 bytes long when encoded in UTF-8. If not set, defaults to an empty description."]
        pub description: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "displayName")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Required. User defined name of this taxonomy. It must: contain only unicode letters, numbers, underscores, dashes and spaces; not start or end with spaces; and be at most 200 bytes long when encoded in UTF-8."]
        pub display_name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "name")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Resource name of this taxonomy, whose format is: \"projects/{project_number}/locations/{location_id}/taxonomies/{id}\"."]
        pub name: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policyTagCount")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Number of policy tags contained in this taxonomy."]
        pub policy_tag_count: ::std::option::Option<::std::primitive::i64>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "taxonomyTimestamps")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. Timestamps about this taxonomy. Only create_time and update_time are used."]
        pub taxonomy_timestamps:
            ::std::option::Option<::std::boxed::Box<GoogleCloudDatacatalogV1beta1SystemTimestamps>>,
    }
    impl GoogleCloudDatacatalogV1beta1Taxonomy {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1TaxonomyBuilder {
            GoogleCloudDatacatalogV1beta1TaxonomyBuilder::default()
        }
    }
    #[derive(Debug, PartialEq, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
    pub enum GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
        #[serde(rename = "POLICY_TYPE_UNSPECIFIED")]
        #[doc = "Unspecified policy type."]
        PolicyTypeUnspecified,
        #[serde(rename = "FINE_GRAINED_ACCESS_CONTROL")]
        #[doc = "Fine grained access control policy, which enables access control on tagged resources."]
        FineGrainedAccessControl,
    }
    impl ::std::default::Default for GoogleCloudDatacatalogV1beta1TaxonomyActivatedPolicyTypesEnum {
        fn default() -> Self {
            Self::PolicyTypeUnspecified
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Table view specification."]
    pub struct GoogleCloudDatacatalogV1beta1ViewSpec {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "viewQuery")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Output only. The query that defines the table view."]
        pub view_query: ::std::option::Option<::std::string::String>,
    }
    impl GoogleCloudDatacatalogV1beta1ViewSpec {
        pub fn builder() -> GoogleCloudDatacatalogV1beta1ViewSpecBuilder {
            GoogleCloudDatacatalogV1beta1ViewSpecBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "An Identity and Access Management (IAM) policy, which specifies access controls for Google Cloud resources. A `Policy` is a collection of `bindings`. A `binding` binds one or more `members` to a single `role`. Members can be user accounts, service accounts, Google groups, and domains (such as G Suite). A `role` is a named list of permissions; each `role` can be an IAM predefined role or a user-created custom role. For some types of Google Cloud resources, a `binding` can also specify a `condition`, which is a logical expression that allows access to a resource only if the expression evaluates to `true`. A condition can add constraints based on attributes of the request, the resource, or both. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). **JSON example:** { \"bindings\": [ { \"role\": \"roles/resourcemanager.organizationAdmin\", \"members\": [ \"user:mike@example.com\", \"group:admins@example.com\", \"domain:google.com\", \"serviceAccount:my-project-id@appspot.gserviceaccount.com\" ] }, { \"role\": \"roles/resourcemanager.organizationViewer\", \"members\": [ \"user:eve@example.com\" ], \"condition\": { \"title\": \"expirable access\", \"description\": \"Does not grant access after Sep 2020\", \"expression\": \"request.time < timestamp('2020-10-01T00:00:00.000Z')\", } } ], \"etag\": \"BwWWja0YfJA=\", \"version\": 3 } **YAML example:** bindings: - members: - user:mike@example.com - group:admins@example.com - domain:google.com - serviceAccount:my-project-id@appspot.gserviceaccount.com role: roles/resourcemanager.organizationAdmin - members: - user:eve@example.com role: roles/resourcemanager.organizationViewer condition: title: expirable access description: Does not grant access after Sep 2020 expression: request.time < timestamp('2020-10-01T00:00:00.000Z') - etag: BwWWja0YfJA= - version: 3 For a description of IAM and its features, see the [IAM documentation](https://cloud.google.com/iam/docs/)."]
    pub struct Policy {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "bindings")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Associates a list of `members` to a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one member."]
        pub bindings: ::std::option::Option<::std::vec::Vec<::std::boxed::Box<Binding>>>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "etag")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "`etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost."]
        pub etag: ::std::option::Option<::std::string::String>,
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "version")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies)."]
        pub version: ::std::option::Option<::std::primitive::i64>,
    }
    impl Policy {
        pub fn builder() -> PolicyBuilder {
            PolicyBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `SetIamPolicy` method."]
    pub struct SetIamPolicyRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "policy")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Cloud Platform services (such as Projects) might reject them."]
        pub policy: ::std::option::Option<::std::boxed::Box<Policy>>,
    }
    impl SetIamPolicyRequest {
        pub fn builder() -> SetIamPolicyRequestBuilder {
            SetIamPolicyRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Request message for `TestIamPermissions` method."]
    pub struct TestIamPermissionsRequest {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "The set of permissions to check for the `resource`. Permissions with wildcards (such as '*' or 'storage.*') are not allowed. For more information see [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions)."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestIamPermissionsRequest {
        pub fn builder() -> TestIamPermissionsRequestBuilder {
            TestIamPermissionsRequestBuilder::default()
        }
    }
    #[derive(
        Clone, Debug, PartialEq, derive_builder :: Builder, serde :: Serialize, serde :: Deserialize,
    )]
    #[doc = "Response message for `TestIamPermissions` method."]
    pub struct TestIamPermissionsResponse {
        #[builder(default = "{ ::std::default::Default::default() }", setter(into))]
        #[serde(rename = "permissions")]
        #[serde(skip_serializing_if = "::std::option::Option::is_none")]
        #[doc = "A subset of `TestPermissionsRequest.permissions` that the caller is allowed."]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl TestIamPermissionsResponse {
        pub fn builder() -> TestIamPermissionsResponseBuilder {
            TestIamPermissionsResponseBuilder::default()
        }
    }
}
